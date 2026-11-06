# `async fn` 與 `.await`

## 本集目標

會寫 `async fn`，用 `.await` 拿到它的結果；知道 `.await` 只能寫在 async 的環境裡，所以 `main` 也得是 async。

## 正文

### 寫一個 async fn

在 `fn` 前面加 `async`，其他寫法不變，回傳型別照樣寫成「結果」的型別。呼叫它之後再加 `.await`，就拿到結果：

```rust
async fn tea_price() -> u32 {
    38
}

#[tokio::main]
async fn main() {
    let price = tea_price().await;
    println!("紅茶 {price} 元");
}
```

執行結果：

```text
紅茶 38 元
```

- `async fn tea_price() -> u32` 的 `-> u32` 寫的是結果的型別，不用管 async 在底下做了什麼。
- `tea_price().await` 的結果是 `u32`，所以 `price` 可以直接印。
- `#[tokio::main]` 與 `async fn main()` 先照抄：async 程式需要一個「負責執行的引擎」才跑得起來，這一行替你準備好。這一章每個範例的 `main` 都長這樣，第 4 集再解釋。
- 這支程式沒有東西可等，只是看語法。

### `.await`：在這裡等結果

加上真的會等的東西。`tokio::time::sleep` 是 async 版的睡眠（它來自 crate tokio，第 4 集會多認識一點），`.await` 它的時候，這個任務停下來，把執行緒讓出去，時間到了再回來接著做：

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn brew(drink: &str) -> String {
    println!("開始泡{drink}");
    sleep(Duration::from_millis(100)).await;
    println!("{drink}泡好了");
    format!("一杯{drink}")
}

async fn serve_set() -> String {
    let tea = brew("紅茶").await;
    let coffee = brew("咖啡").await;
    format!("{tea}加{coffee}")
}

#[tokio::main]
async fn main() {
    let order = serve_set().await;
    println!("送出：{order}");
}
```

執行結果：

```text
開始泡紅茶
紅茶泡好了
開始泡咖啡
咖啡泡好了
送出：一杯紅茶加一杯咖啡
```

- `brew("紅茶").await` 這一行會停在 `sleep` 的地方，等 100 毫秒；等的這段時間，執行緒是空的，可以去做別的事。
- 兩個 `.await` 是一個接一個：咖啡要等紅茶整杯泡好才開始。同一個任務裡的 `.await` 跟一般的循序程式一樣，順序不會亂，讓出去的是執行緒，不是你的執行順序。想讓兩杯同時泡，要把它們各自開成任務，第 5 集才教。
- `serve_set` 裡面用了 `.await`，所以它自己也是 `async fn`，呼叫它的 `main` 一樣要 `.await`。

### async fn 回傳的不是結果

忘了 `.await` 的話，拿到的不是 `u32`：

```rust,compile_fail=E0308
async fn tea_price() -> u32 {
    38
}

fn main() {
    let price: u32 = tea_price();
    println!("紅茶 {price} 元");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:6:22
  |
6 |     let price: u32 = tea_price();
  |                ---   ^^^^^^^^^^^ expected `u32`, found future
  |                |
  |                expected due to this
```

`async fn` 呼叫之後得到的是一個 `Future`（第 3 集），代表「還沒做完的工作」，要 `.await` 才變成 `u32`。編譯器把它叫做 future，所以訊息是 `expected u32, found future`。

### `.await` 只能用在 async 裡

反過來，在普通函式裡寫 `.await` 也不行：

```rust,compile_fail=E0728
async fn tea_price() -> u32 {
    38
}

fn main() {
    let price = tea_price().await;
    println!("紅茶 {price} 元");
}
```

編譯錯誤：

```text
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src/main.rs:6:29
  |
5 | fn main() {
  | --------- this is not `async`
6 |     let price = tea_price().await;
  |                             ^^^^^ only allowed inside `async` functions and blocks
```

`.await` 的意思是「暫停這個任務」，函式本身得是 async，才有東西可以暫停。所以呼叫 async 函式的函式也要是 async，一路往上到 `main`，這就是 `main` 得加 `#[tokio::main]` 的原因。

### 跟 Go 比一比

Go 的函式沒有顏色：任何函式裡都可以做會等的事（睡覺、讀 channel、發網路請求），呼叫的人不用知道。

Rust 的 async 函式有「顏色」：只有 async 的環境才能 `.await` 它。某個很底層的函式改成 async，呼叫它的函式也得跟著改成 async，一路改到 `main`，讀程式的人常說 async 會傳染。好處是每個「可能停下來」的地方都有 `.await` 標記，讀程式一眼就看得出來；Go 裡看到 `x := f()`，看不出 `f` 裡面等了多久。

## 重點整理

- `async fn` 的回傳型別照寫（`-> u32`）；呼叫它得到 `Future`，要加 `.await` 才拿到 `u32`。
- `.await` 暫停目前的任務、等結果好了再繼續；讓出去的是執行緒，同一個任務裡的 `.await` 仍然一個接一個。
- `.await` 只能寫在 async 函式或 async 區塊裡，否則 E0728；忘了 `.await`、把 Future 當結果用是 E0308。
- `main` 也要是 async：先照抄 `#[tokio::main]` 加 `async fn main()`，第 4 集解釋；`tokio::time::sleep(...).await` 是 async 版的睡眠。
- 跟 Go 比：Go 的函式沒有顏色，Rust 的 async 會傳染，換來每個會停下來的地方都看得見。
