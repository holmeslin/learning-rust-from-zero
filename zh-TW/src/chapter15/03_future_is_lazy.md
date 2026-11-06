# `Future` 是惰性的

## 本集目標

知道呼叫 async 函式不會執行它，只是得到一個 `Future`，要 `.await` 才會開跑；也看一眼 `Future` trait 的樣子，知道 `.await` 在背後做什麼。

## 正文

### 呼叫 async 函式，什麼都不會發生

```rust
async fn brew(drink: &str) -> String {
    println!("開始泡{drink}");
    format!("一杯{drink}")
}

#[tokio::main]
async fn main() {
    let order = brew("紅茶");
    println!("訂單收到了");
    let cup = order.await;
    println!("{cup}");
}
```

執行結果：

```text
訂單收到了
開始泡紅茶
一杯紅茶
```

「開始泡紅茶」排在「訂單收到了」後面：`brew("紅茶")` 那一行只是做出一個 `Future`，函式本體一行都還沒跑；到 `order.await` 才開始執行。這就是 `Future` 的惰性（lazy）：它是「這件事該怎麼做」的描述，沒有人推它，它就不動。

### 只呼叫不 `.await`

忘了 `.await` 是最常見的錯。編譯器看得出來，會警告：

```rust,warn
async fn brew(drink: &str) -> String {
    println!("開始泡{drink}");
    format!("一杯{drink}")
}

#[tokio::main]
async fn main() {
    brew("紅茶");
    println!("收工");
}
```

編譯警告：

```text
warning: unused implementer of `Future` that must be used
 --> src/main.rs:8:5
  |
8 |     brew("紅茶");
  |     ^^^^^^^^^^^^
  |
  = note: futures do nothing unless you `.await` or poll them
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
```

執行結果：

```text
收工
```

程式編譯得過，但「開始泡紅茶」從頭到尾沒有印：`Future` 做出來就被丟掉，函式本體根本沒跑。`futures do nothing unless you .await or poll them` 說的就是這件事。看到這個警告，幾乎都是漏了 `.await`，要馬上補上。

### `Future` 長什麼樣子

標準函式庫裡的 `Future` 是 trait，只有一個要實作的方法（`Pin` 與 `Context` 先不用懂，附錄二 d 才介紹 `Pin`）：

```text
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

負責執行任務的 runtime 會呼叫 `poll`（輪詢，polling），問一句「做完了嗎？」：

- 回傳 `Poll::Ready(值)`：做完了，`值` 就是 `Output`，也就是 `.await` 的結果。
- 回傳 `Poll::Pending`：還沒好。這個任務讓出執行緒，並留下「好了再叫我」的記號，例如計時器到了、網路資料來了，runtime 才會再 `poll` 一次。

所以 `.await` 可以讀成：「一直 `poll`，直到 `Ready`；遇到 `Pending` 就把執行緒讓出去」。`async fn` 與 `async` 區塊都是編譯器替你寫好 `poll` 的 `Future`，平常不需要自己實作。

`.await` 會把 `Future` 用掉（move），同一個 `Future` 不能 `.await` 第二次，否則是 E0382（`use of moved value`）。要再做一次，就再呼叫一次函式。

### `async` 區塊

不想另外寫函式的時候，可以用 `async { ... }` 直接做出一個 `Future`。它也是惰性的，`.await` 的結果是區塊最後的那個值：

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let order = async {
        println!("開始泡紅茶");
        sleep(Duration::from_millis(100)).await;
        38
    };
    println!("訂單收到了");
    let price = order.await;
    println!("{price} 元");
}
```

執行結果：

```text
訂單收到了
開始泡紅茶
38 元
```

第 5 集的 `tokio::spawn` 會很常用到 `async` 區塊。

### 跟 Go 比一比

Go 的 `go brew("紅茶")` 一寫下去，goroutine 立刻排進 runtime，馬上就可能開跑。Rust 的 `brew("紅茶")` 什麼都不會做，要有人 `.await`（或第 5 集的 `tokio::spawn`）它，才會真的執行。

從 Go 轉過來，常見的錯是以為「呼叫了就會跑」。忘了 `.await` 時沒有任何執行時期的錯誤，工作只是默默沒做；前面看到的 `unused_must_use` 警告，就是編譯器替你抓這件事。

另一個後果：丟掉一個還沒做完的 `Future`，就等於取消它，做到一半的事會直接停住。Go 的 goroutine 沒有這種事，開出去就一直跑到結束。第 8、9 集的 `select!` 與 `timeout` 就靠這一點來取消工作。

## 重點整理

- 呼叫 `async fn` 只會得到 `Future`，函式本體要等到 `.await`（或 `tokio::spawn`）才開始跑；只呼叫不 `.await` 會有 `unused_must_use` 警告。
- `Future` trait：`poll` 回傳 `Poll::Ready(值)` 或 `Poll::Pending`；`.await` 是一直 `poll` 直到 `Ready`，遇到 `Pending` 就讓出執行緒。
- `.await` 會 move 掉 `Future`，同一個 `Future` 不能 `.await` 第二次（E0382）。
- `async { ... }` 區塊也是 `Future`，一樣惰性，`.await` 的結果是區塊最後的值。
- 跟 Go 比：`go f()` 立刻開跑，Rust 的 `f()` 不會跑；丟掉沒做完的 `Future` 就是取消它。
