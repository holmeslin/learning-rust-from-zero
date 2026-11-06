# `tokio::sync::mpsc`

## 本集目標

用 tokio 的 mpsc channel 在任務之間傳訊息：`send` 與 `recv` 都要 `.await`；知道它有容量上限，而且要等所有 `Sender` 都被 drop，channel 才會關閉。

## 正文

### 任務之間傳訊息

第 14 章的 `std::sync::mpsc` 在 async 裡不好用：`recv()` 會把整條執行緒卡住，而不只是目前的任務（原因在第 12 集）。tokio 有自己的版本，`send` 和 `recv` 都是 async，等的時候只讓出任務。下面讓三位店員各自送一張訂單給櫃台：

```rust
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);
    for (clerk, wait) in [("小美", 200), ("阿明", 100), ("小華", 300)] {
        let tx = tx.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(wait)).await;
            tx.send(clerk).await.expect("櫃台不見了");
        });
    }
    drop(tx);
    while let Some(clerk) = rx.recv().await {
        println!("{clerk}送來一張訂單");
    }
    println!("全部收完了");
}
```

執行結果：

```text
阿明送來一張訂單
小美送來一張訂單
小華送來一張訂單
全部收完了
```

- `mpsc::channel(8)` 回傳 `(Sender, Receiver)`，`8` 是容量：channel 裡最多排 8 則訊息。
- `tx.send(clerk).await` 回傳 `Result<(), SendError<T>>`。channel 滿了，`.await` 會等到有空位；接收端已經被 drop，則是 `Err`。
- `rx.recv().await` 回傳 `Option<T>`：有訊息是 `Some`，channel 關閉而且排空了是 `None`，跟 std 的 `recv()` 回傳 `Result` 不同。`recv` 需要 `&mut self`，所以 `rx` 要寫成 `mut rx`。
- `while let Some(clerk) = rx.recv().await` 是收訊息的固定寫法，channel 關了迴圈自然結束。
- 多個生產者跟第 14 章第 6 集一樣：每個任務一份 `tx.clone()`，用 `async move` 搬進去；原本的 `tx` 要 `drop`。

channel 要等所有 `Sender`（原本的加上每個 clone）都被 drop 才會關。把 `drop(tx)` 那一行拿掉，`main` 自己還握著一個 `tx`，三個任務做完之後，`rx.recv().await` 就一直等下去，程式永遠不會結束，而且沒有任何錯誤訊息，跟 std 版完全一樣。

### 容量與背壓

tokio 的 `channel(n)` 一定要給容量，這跟 std 預設無上限的 `mpsc::channel()` 不同，比較像第 14 章第 16 集的 `sync_channel(n)`。channel 滿了，`send(...).await` 就等到有空位，這是背壓（backpressure）。不一樣的是：它等的時候只是這個任務停住，執行緒並沒有被卡住。下面的程式只用一條執行緒，而且櫃台睡著的時候，生產者的任務照樣被排到：

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let sent = Arc::new(AtomicUsize::new(0));
    let (tx, mut rx) = mpsc::channel(2);
    let counter = Arc::clone(&sent);
    tokio::spawn(async move {
        for order in 1..=5 {
            tx.send(order).await.expect("櫃台不見了");
            counter.fetch_add(1, Ordering::SeqCst);
        }
    });
    sleep(Duration::from_millis(200)).await;
    println!("櫃台還沒收，已經送出 {} 筆", sent.load(Ordering::SeqCst));
    let mut received = Vec::new();
    while let Some(order) = rx.recv().await {
        received.push(order);
    }
    println!("櫃台依序收到 {received:?}");
}
```

執行結果：

```text
櫃台還沒收，已經送出 2 筆
櫃台依序收到 [1, 2, 3, 4, 5]
```

生產者要送 5 筆，容量只有 2，第 3 筆的 `send` 就停在那裡，`sent` 停在 2。這個程式沒有第二條執行緒，`send` 要是會卡住執行緒的話，`main` 就永遠醒不過來，也印不出那一行。櫃台開始收之後，生產者才繼續送完。

不想等的時候，用 `try_send`：它不是 async，滿了就回傳 `Err(TrySendError::Full(值))`，把值還給你；接收端已經不在了，是 `Err(TrySendError::Closed(值))`。

另外兩點：容量不能是 0，`mpsc::channel(0)` 會 panic（`mpsc bounded channel requires buffer > 0`）；tokio 也有沒有上限的 `unbounded_channel()`，`send` 不會等，這本書不用。

### 跟 Go 比一比

Go 的帶緩衝 channel 跟它很像：

```go
orders := make(chan int, 8)
go func() {
	defer close(orders)
	for order := 1; order <= 3; order++ {
		orders <- order
	}
}()
for order := range orders {
	fmt.Println("收到第", order, "張訂單")
}
```

對照起來：

- `make(chan int, 8)` 對應 `mpsc::channel(8)`，都有容量，滿了送的人就等。
- Go 要送的一方明寫 `close(orders)`；Rust 沒有 `close`，所有 `Sender` 都被 drop，channel 就關了。
- Go 的 `for order := range orders` 對應 `while let Some(order) = rx.recv().await`。
- 陷阱在容量：Go 最常寫的 `make(chan int)` 是沒有緩衝的，一送一收要碰頭；tokio 的 `channel(0)` 不是這個意思，它會直接 panic。想要最小的 channel，寫 `channel(1)`。

## 重點整理

- `mpsc::channel(n)` 回傳 `(Sender, Receiver)`，容量 `n` 必須大於 0；`tx.send(x).await` 回傳 `Result`，滿了就等；`rx.recv().await` 回傳 `Option<T>`，channel 關閉且排空是 `None`。
- 收訊息的固定寫法：`while let Some(x) = rx.recv().await`；`rx` 要 `mut`。
- 多個生產者：每個任務一份 `tx.clone()`，原本的 `tx` 要 `drop`；忘了 `drop`，`recv` 永遠等下去，沒有錯誤訊息。
- channel 滿了，`send(...).await` 讓出任務而不是卡住執行緒（背壓）；`try_send` 不等，滿了是 `Err(TrySendError::Full(值))`。
- 跟 Go 比：`make(chan T, n)` 對應 `channel(n)`；Rust 靠 drop 關閉，沒有 `close`；`channel(0)` 會 panic，不是 Go 的無緩衝 channel。
