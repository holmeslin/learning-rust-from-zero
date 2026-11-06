# `select!`

## 本集目標

用 `tokio::select!` 同時等好幾件事，最先好的那件勝出；知道沒勝出的 `Future` 會被丟掉（取消）。

## 正文

### 誰先好就用誰

客人點了咖啡也點了紅茶，只要先好的那杯。`tokio::select!` 同時等好幾個 `Future`，其中一個做完，就執行它的處理區塊，其他的整個丟掉：

```rust
use std::time::Duration;
use tokio::time::sleep;

struct Cup(&'static str);

impl Drop for Cup {
    fn drop(&mut self) {
        println!("{}的杯子收走了", self.0);
    }
}

async fn brew(drink: &'static str, wait: u64) -> &'static str {
    let _cup = Cup(drink);
    sleep(Duration::from_millis(wait)).await;
    println!("{drink}泡好了");
    drink
}

#[tokio::main]
async fn main() {
    tokio::select! {
        tea = brew("紅茶", 300) => println!("先好的是{tea}"),
        coffee = brew("咖啡", 100) => println!("先好的是{coffee}"),
    }
    println!("打烊");
}
```

執行結果：

```text
咖啡泡好了
咖啡的杯子收走了
紅茶的杯子收走了
先好的是咖啡
打烊
```

- 每個分支寫成 `模式 = Future => 處理`。這個巨集（第 16 章第 1 集會揭曉巨集）會同時 `poll` 所有分支的 `Future`。
- 咖啡 100 毫秒就好，所以先勝出：先印「咖啡泡好了」，它的杯子在 `brew` 結束時收走，然後執行咖啡那一支的處理，印「先好的是咖啡」。
- 紅茶沒有被等完，它的 `Future` 被丟掉（第 3 集說過，丟掉就是取消）。`brew` 裡的區域變數 `_cup` 跟著 drop，所以看得到「紅茶的杯子收走了」，卻從來沒有「紅茶泡好了」。`select!` 先丟掉落選的 `Future`，才執行勝出分支的處理，所以這一行排在「先好的是咖啡」前面。
- `select!` 整個結束之後，才輪到後面的「打烊」。

沒勝出的那一邊，做到一半的事就停在原地，不會有人接著做完。

### 在迴圈裡等訊息，也等計時

`select!` 最常放在迴圈裡，一邊收訊息、一邊盯著別的事情。下面的櫃台一直收單，超過 350 毫秒沒有新單就打烊：

```rust
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);
    tokio::spawn(async move {
        for order in 1..=3 {
            sleep(Duration::from_millis(100)).await;
            tx.send(order).await.expect("櫃台不見了");
        }
    });
    loop {
        tokio::select! {
            Some(order) = rx.recv() => println!("收到第 {order} 張訂單"),
            _ = sleep(Duration::from_millis(350)) => {
                println!("太久沒人點，打烊");
                break;
            }
        }
    }
}
```

執行結果：

```text
收到第 1 張訂單
收到第 2 張訂單
收到第 3 張訂單
太久沒人點，打烊
```

- `Some(order) = rx.recv()` 的左邊是模式：`recv()` 的結果是 `Some(order)` 才算這一支勝出；結果是 `None`（channel 關閉）時模式對不上，這一支就停用，只剩其他分支。
- 每一圈都建立一個新的 `sleep(350 毫秒)`，所以計時是從「最後一次收到單」開始算，這就是閒置逾時。
- 三張單收完，生產者任務結束、channel 關閉，`recv()` 回傳 `None`，這一支停用；只剩計時那一支，350 毫秒到了就 `break`。
- 所有分支都停用的話，可以多寫一支 `else => ...` 處理，不寫的話會 panic。

### 同時就緒的時候

兩個以上的分支同時就緒時，`select!` 預設隨機挑一個，避免永遠偏心第一支。所以範例裡各分支要隔開足夠的時間，結果才會固定。想要固定從上往下檢查，可以在最前面加 `biased;`，這本書不展開。

### 跟 Go 比一比

Go 的 `select` 長得很像，同樣是「好幾件事誰先好」，兩個以上同時就緒也是隨機挑一個。不一樣的地方是比什麼：

- Go 的 `case` 只能是 channel 的收或送，沒被選到的 `case` 本來就什麼都還沒做。
- Rust 的每個分支是一整個 `Future`，可以是任何 async 工作（泡茶、計時、收訊息）。沒勝出的 `Future` 做到一半就被丟掉，工作真的停下來。
- 在 Go 裡，要讓一個 goroutine 做到一半停下來，得靠它自己去看 `ctx.Done()`；`select!` 丟掉 `Future` 不需要對方配合。

## 重點整理

- `tokio::select! { 模式 = Future => 處理, ... }` 同時等好幾個 `Future`，最先做完的那個執行處理區塊。
- 沒勝出的 `Future` 會被丟掉，也就是取消：做到一半的事停住，裡面的區域變數照常 drop。
- 分支的模式對不上（例如 `Some(x) = rx.recv()` 遇到 `None`）時，這個分支停用；全部停用要寫 `else`，否則 panic。
- 在迴圈裡每圈建立新的 `sleep`，可以做閒置逾時。
- 同時就緒時預設隨機挑一個；跟 Go 比：Go 的 `case` 是 channel 操作，Rust 的分支是整個 `Future`，落選的工作會被取消。
