# `tokio::spawn`

## 本集目標

用 `tokio::spawn` 把一個 `Future` 開成獨立的任務，讓它在背後同時進行；用 `JoinHandle` 等結果，並知道任務裡的資料要 `async move`。

## 正文

### 開成任務，同時進行

第 2 集的兩個 `.await` 是一個接一個。想讓三杯飲料同時泡，要把每一杯各自開成任務。`tokio::spawn` 收一個 `Future`，交給 runtime 排程，馬上回傳 `JoinHandle`；對 handle `.await`，才會拿到任務的結果：

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn brew(drink: &str, wait: u64) -> String {
    sleep(Duration::from_millis(wait)).await;
    println!("{drink}好了");
    format!("一杯{drink}")
}

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    for (drink, wait) in [("紅茶", 300), ("咖啡", 100), ("奶茶", 200)] {
        handles.push(tokio::spawn(brew(drink, wait)));
    }
    for handle in handles {
        let cup = handle.await.expect("店員出事了");
        println!("送出{cup}");
    }
}
```

執行結果：

```text
咖啡好了
奶茶好了
紅茶好了
送出一杯紅茶
送出一杯咖啡
送出一杯奶茶
```

- 三個任務同時在等，先好的先印：咖啡、奶茶、紅茶。第 1 集的執行緒版是三條執行緒，這裡是三個任務，不管在幾條執行緒上跑，寫法上都不用管。
- `handle.await` 的結果是 `Result<String, JoinError>`，跟第 14 章第 2 集的 `join()` 一樣要處理，這裡用 `.expect`。
- 按 handle 的順序等：先等紅茶，所以「送出」依序是紅茶、咖啡、奶茶；咖啡和奶茶早就好了，`.await` 立刻拿到結果。
- 任務很輕，開上萬個也沒問題：一萬個任務各自等 100 毫秒，一樣很快就跑完。第 1 集的執行緒寫法，同樣的事得開一萬條執行緒。

### spawn 之後就開跑了

第 3 集說 `Future` 是惰性的，但 `tokio::spawn` 一呼叫，任務就排進 runtime，不用等有人 `.await` handle：

- 兩個 handle 都不接、也不 `.await`，任務照樣會做完。丟掉 `JoinHandle` 不會取消任務，只是不再等它的結果。
- `main` 結束，整個 runtime 跟著關掉，還沒做完的任務直接被丟掉，跟第 14 章第 1 集的執行緒一樣。所以要等任務做完，就得 `.await` 它的 handle。

### `async move`：任務不能借區域變數

`spawn` 的任務可能活得比目前的函式久，所以它不能借區域變數，規則跟第 14 章第 3 集的 `thread::spawn` 一樣（`'static`）：

```rust,compile_fail=E0373
#[tokio::main]
async fn main() {
    let drink = String::from("紅茶");
    let handle = tokio::spawn(async {
        println!("{drink} 好了");
    });
    handle.await.expect("店員出事了");
}
```

編譯錯誤：

```text
error[E0373]: async block may outlive the current function, but it borrows `drink`, which is owned by the current function
 --> src/main.rs:4:31
  |
4 |     let handle = tokio::spawn(async {
  |                               ^^^^^ may outlive borrowed value `drink`
5 |         println!("{drink} 好了");
  |                    ----- `drink` is borrowed here
  |
  = note: async blocks are not executed immediately and must either take a reference or ownership of outside variables they use
help: to force the async block to take ownership of `drink` (and any other referenced variables), use the `move` keyword
  |
4 |     let handle = tokio::spawn(async move {
  |                                     ++++
```

`help` 說得很清楚：在 `async` 後面加 `move`，讓區塊拿走 `drink` 的所有權。這跟閉包的 `move` 是同一回事。`async move { ... }` 是開任務最常見的寫法。要在好幾個任務裡用同一份資料，照第 14 章的做法：`Arc`（第 14 章第 8 集），或先 `.clone()`。

### 任務 panic 了

任務裡的 panic 只會結束那個任務，不會連累別人。handle 的 `.await` 會得到 `Err(JoinError)`，`is_panic()` 可以分辨是 panic：

```rust
#[tokio::main]
async fn main() {
    let good = tokio::spawn(async { 38 });
    let bad = tokio::spawn(async {
        panic!("機器爆炸了");
    });
    println!("{:?}", good.await.ok());
    match bad.await {
        Err(e) if e.is_panic() => println!("任務 panic 了"),
        _ => println!("沒事"),
    }
    println!("店照常營業");
}
```

執行結果：

```text
Some(38)
任務 panic 了
店照常營業
```

panic 的訊息照樣印在 stderr：`thread 'tokio-rt-worker' (數字) panicked at src/main.rs:5:9:`，下一行是 `機器爆炸了`。括號裡的數字每次都不同，執行緒名稱也會隨 tokio 的版本改變，所以上面只貼 stdout。

### 跟 Go 比一比

Go 的 `go f()` 沒有回傳值，要拿結果得自己接 `WaitGroup` 或 channel；`tokio::spawn` 直接給你 `JoinHandle`，`.await` 它就拿到回傳值。

另一個差別是 panic。Go 的 goroutine 沒有 `recover` 就 panic，整個程式立刻結束（結束碼 2）。tokio 的任務 panic 只結束那個任務，`handle.await` 回傳 `Err(JoinError)`，其他任務和 `main` 都繼續跑。

## 重點整理

- `tokio::spawn(future)` 把 `Future` 開成獨立任務，馬上回傳 `JoinHandle<T>`；呼叫之後任務就開始跑，不需要有人 `.await`。
- `handle.await` 得到 `Result<T, JoinError>`；丟掉 handle 不會取消任務；`main` 結束，沒做完的任務就被丟掉。
- 任務要求 `'static`：不能借區域變數，要寫 `async move { ... }`（否則 E0373），共用資料用 `Arc` 或 `.clone()`。
- 任務 panic 只結束自己，`handle.await` 是 `Err`，用 `is_panic()` 分辨。
- 任務很輕，開上萬個也沒問題。跟 Go 比：`spawn` 回傳 `JoinHandle`，goroutine panic 會讓整個程式結束，任務 panic 不會。
