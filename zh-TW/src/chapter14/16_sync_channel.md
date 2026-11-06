# `sync_channel` 與背壓

## 本集目標

用 `mpsc::sync_channel` 建立有容量上限的 channel，看懂「滿了就讓生產者等」的背壓（backpressure），也會用 `try_send` 在滿的時候不等待。

## 正文

### `channel()` 沒有上限

前面幾集的 `mpsc::channel()` 是沒有上限的：`send` 永遠不會等，訊息先全部排進佇列。如果生產者做得比消費者收得快，佇列就一直長，記憶體跟著一直用，直到程式被系統砍掉。

想讓生產者「慢下來配合消費者」，要用 `mpsc::sync_channel(n)`。`n` 是 channel 最多能排幾則訊息：

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let sent = AtomicUsize::new(0);
    let (tx, rx) = mpsc::sync_channel(2);
    thread::scope(|s| {
        s.spawn(|| {
            for order in 1..=6 {
                tx.send(order).expect("櫃台不見了");
                sent.fetch_add(1, Ordering::SeqCst);
            }
        });
        thread::sleep(Duration::from_millis(300));
        println!("櫃台還沒收，已經送出 {} 筆", sent.load(Ordering::SeqCst));
        let mut received = Vec::new();
        for _ in 1..=6 {
            received.push(rx.recv().expect("channel 關閉了"));
        }
        println!("櫃台依序收到 {received:?}");
    });
    println!("全部送完：{} 筆", sent.load(Ordering::SeqCst));
}
```

執行結果：

```text
櫃台還沒收，已經送出 2 筆
櫃台依序收到 [1, 2, 3, 4, 5, 6]
全部送完：6 筆
```

- `sync_channel(2)` 回傳 `(SyncSender, Receiver)`，`SyncSender` 的 `send` 用法跟 `Sender` 一樣，也能 `clone`。
- 生產者要送 6 筆，櫃台睡了 300 毫秒都沒收。channel 只排得下 2 筆，所以第 3 筆的 `send` 卡在那裡，`sent` 停在 2。
- 櫃台開始 `recv`，channel 有空位，生產者才逐筆送完，最後 `sent` 是 6。
- 這就是背壓：消費者慢，生產者被迫跟著慢，佇列的長度有上限，記憶體不會暴增。
- 換成 `mpsc::channel()`，同一支程式會在 300 毫秒後印出「已經送出 6 筆」：生產者一口氣送完，完全不等。

`sync_channel(0)` 是特例：沒有任何排隊的空間，`send` 要等到有人真的 `recv` 才算送出，生產者跟消費者一對一交接（把上面的 `sync_channel(2)` 改成 0，`sent` 在 300 毫秒後是 0）。

### `try_send`：滿了就別等

有時候不想卡住等，滿了就放棄、或先做別的事。`try_send` 不等，channel 滿了就回傳 `Err(TrySendError::Full(值))`，把沒送出去的值還給你：

```rust
use std::sync::mpsc::{self, TrySendError};

fn main() {
    let (tx, rx) = mpsc::sync_channel(2);
    for order in ["紅茶", "咖啡", "奶茶"] {
        match tx.try_send(order) {
            Ok(()) => println!("{order} 排進去了"),
            Err(TrySendError::Full(order)) => println!("{order} 排不進去，請客人等一下"),
            Err(TrySendError::Disconnected(_)) => println!("櫃台打烊了"),
        }
    }
    println!("櫃台收了 {}", rx.recv().expect("channel 關閉了"));
    println!("騰出一格，再試一次：{:?}", tx.try_send("奶茶"));
}
```

執行結果：

```text
紅茶 排進去了
咖啡 排進去了
奶茶 排不進去，請客人等一下
櫃台收了 紅茶
騰出一格，再試一次：Ok(())
```

- `use std::sync::mpsc::{self, TrySendError};` 裡的 `self` 代表 `mpsc` 本身（第 12 章第 5 集），所以可以寫 `mpsc::sync_channel`，也能直接寫 `TrySendError`。
- 前兩筆排進去，第三筆發現滿了，回傳 `Full`，裡面的 `"奶茶"` 原封不動還給我們，可以稍後重送或丟掉。
- 另一個失敗原因是 `Disconnected`：`Receiver` 已經被 drop，沒人會收了。
- 櫃台收走一筆之後，空出一格，同樣的 `try_send("奶茶")` 就成功了，回傳 `Ok(())`。
- 這支程式只有一條執行緒，所以每一行的結果是固定的，不用擔心誰先誰後。

### 什麼時候用哪一種

- 訊息量小、不怕堆積：`channel()` 最省事，生產者永遠不會被擋。
- 生產者可能比消費者快、或資料很大：用 `sync_channel(n)`，容量上限就是記憶體的上限，生產者自然被拖慢。
- 不想被擋住：`try_send`，滿了自己決定怎麼辦（丟掉、重試、回報忙碌）。

### 跟 Go 比一比

Go 的 `make(chan T)` 沒有緩衝，送的人要等收的人；`make(chan T, n)` 有 `n` 格緩衝。這兩種正好對應 `sync_channel(0)` 與 `sync_channel(n)`：

```go
orders := make(chan int, 2) // 對應 mpsc::sync_channel(2)
```

陷阱在 Rust 預設的 `mpsc::channel()`：它不是 Go 的任何一種 channel，而是沒有上限。把 Go 的 `make(chan int, 2)` 照直覺翻成 `mpsc::channel()`，程式照樣能跑、結果也一樣，但背壓不見了，生產者再也不會被擋。從 Go 轉過來，要有背壓就要明寫 `sync_channel`。

## 重點整理

- `mpsc::channel()` 沒有容量上限，`send` 不會等；生產者比消費者快，佇列會一直長。
- `mpsc::sync_channel(n)` 最多排 `n` 筆，滿了 `send` 就等到有空位（背壓）；`n` 為 0 時要等有人 `recv` 才算送出。
- `try_send` 不等：滿了回傳 `Err(TrySendError::Full(值))`，把值還給你；收端不在是 `Disconnected`。
- 要限制記憶體、讓生產者配合消費者的速度，用 `sync_channel`。
- 跟 Go 比：`make(chan T, n)` 對應 `sync_channel(n)`；Rust 的預設 `channel()` 是無限的，照翻會弄丟背壓。
