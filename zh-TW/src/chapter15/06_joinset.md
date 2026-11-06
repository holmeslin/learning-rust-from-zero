# `JoinSet`

## 本集目標

用 `JoinSet` 管理一批任務，依「誰先做完」的順序拿結果，也知道它被丟掉的時候，還沒做完的任務會被取消。

## 正文

### 一批任務，先好的先處理

第 5 集把 `JoinHandle` 放進 `Vec`，只能照開任務的順序等：紅茶最慢，後面的人就算早就好了，也得排在它後面。想要「先好的先處理」，改用 `JoinSet`：

```rust
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::sleep;

async fn brew(drink: &str, wait: u64) -> String {
    sleep(Duration::from_millis(wait)).await;
    format!("一杯{drink}")
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    for (drink, wait) in [("紅茶", 300), ("咖啡", 100), ("奶茶", 200)] {
        set.spawn(brew(drink, wait));
    }
    println!("開了 {} 個任務", set.len());
    while let Some(result) = set.join_next().await {
        let cup = result.expect("店員出事了");
        println!("送出{cup}");
    }
}
```

執行結果：

```text
開了 3 個任務
送出一杯咖啡
送出一杯奶茶
送出一杯紅茶
```

- `JoinSet::new()` 建立一組任務，`set.spawn(future)` 開任務，用法跟 `tokio::spawn` 一樣，對 `Future` 的要求也一樣（`Send`、`'static`）。
- `set.join_next().await` 等「下一個做完的任務」，回傳 `Option<Result<T, JoinError>>`：有任務做完是 `Some`，裡面的 `Result` 跟第 5 集的 `handle.await` 一樣；全部都收完了就是 `None`。
- `while let Some(result) = set.join_next().await` 是收完整組任務的固定寫法。
- 結果依完成的順序出來：咖啡、奶茶、紅茶，跟開任務的順序無關。要知道結果是哪一個任務的，就讓任務自己把名字連同結果一起回傳。
- `set.len()` 是還沒收走的任務數量。

想一次等完全部，可以用 `set.join_all().await`，回傳 `Vec<T>`，同樣依完成順序。只是任何一個任務 panic，`join_all` 自己也會跟著 panic，所以要個別處理失敗的時候，還是用 `join_next`。

### 結果型別要一樣

一個 `JoinSet<T>` 裡的任務，結果都是同一種型別 `T`。第一個任務回傳整數，第二個就不能回傳字串：

```rust,compile_fail=E0271
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    set.spawn(async { 38 });
    set.spawn(async { "咖啡" });
    while let Some(result) = set.join_next().await {
        println!("{result:?}");
    }
}
```

編譯錯誤：

```text
error[E0271]: expected `{async block@main.rs:7:15}` to be a future that resolves to `{integer}`, but it resolves to `&str`
   --> src/main.rs:7:15
    |
  7 |     set.spawn(async { "咖啡" });
    |         ----- ^^^^^^^^^^^^^^^^ expected integer, found `&str`
    |         |
    |         required by a bound introduced by this call
    |
```

只貼了前半段，後面的 `note` 指向 tokio 的 `JoinSet::spawn` 定義，說明它要求 `Future<Output = T>`。真的需要不同種類的結果，可以讓每個任務回傳同一個 `enum`。

### 丟掉 `JoinSet`，任務就被取消

第 5 集說丟掉 `JoinHandle` 不會取消任務。`JoinSet` 相反：它被丟掉（離開作用域，或是呼叫 `drop`）的時候，裡面還沒做完的任務會全部被取消：

```rust
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    set.spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("咖啡好了");
        "咖啡"
    });
    set.spawn(async {
        sleep(Duration::from_millis(300)).await;
        println!("紅茶好了");
        "紅茶"
    });
    let first = set.join_next().await.expect("沒有任務");
    let drink = first.expect("店員出事了");
    println!("先到的是{drink}");
    drop(set);
    sleep(Duration::from_millis(500)).await;
    println!("打烊");
}
```

執行結果：

```text
咖啡好了
先到的是咖啡
打烊
```

咖啡先好、被收走之後，我們丟掉整組任務。紅茶還在等，就被取消了，所以後面再睡 500 毫秒，也看不到「紅茶好了」。想主動取消，可以呼叫 `set.abort_all()`；被取消的任務，`join_next` 會回傳 `Err(JoinError)`，`is_cancelled()` 是 `true`。

任務裡面 panic 的話，`join_next` 也只是回傳那一個任務的 `Err`，其他任務照常進行，跟第 5 集一樣。

### 跟 Go 比一比

Go 的 goroutine 開出去之後，沒有人能「丟掉」它：沒有 `WaitGroup` 等它、啟動它的函式早就回傳了，它還是會一直跑到結束，忘了管的話就是 goroutine 洩漏。

`JoinSet` 把「這批任務歸誰管」寫進型別裡：擁有 `JoinSet` 的人離開，這批任務就跟著結束。Go 的 `errgroup` 配合 `context` 能做到類似的事，但得自己把 `context` 傳進每個 goroutine，並且在裡面逐一檢查；`JoinSet` 不需要任務配合，只是取消只會發生在任務停在 `.await` 的地方（第 9 集會再看到這件事）。

## 重點整理

- `JoinSet::new()`、`set.spawn(future)` 開一組任務；`while let Some(result) = set.join_next().await` 依完成順序收結果，全部收完回傳 `None`；`join_all()` 一次收完，任何一個 panic 它也 panic。
- `JoinSet<T>` 裡所有任務的結果型別都是 `T`，不同會是 E0271。
- 丟掉 `JoinSet`，還沒做完的任務全部被取消；`abort_all()` 可主動取消，取消的任務是 `Err`，`is_cancelled()` 為 `true`。
- 丟掉單一 `JoinHandle` 不會取消任務，丟掉 `JoinSet` 會。
- 跟 Go 比：goroutine 沒有人能丟掉，`JoinSet` 讓一批任務的壽命跟著擁有它的人走。
