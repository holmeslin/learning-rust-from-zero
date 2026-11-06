# async 閉包

## 本集目標

會寫 async 閉包 `async |x| { ... }`，用 `AsyncFn`、`AsyncFnMut`、`AsyncFnOnce` 收下它，也知道它跟「回傳 async 區塊的閉包」差在哪裡。

## 正文

### 寫一個 async 閉包

閉包（第 11 章）前面加 `async`，呼叫它得到一個 `Future`，要 `.await`：

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let brew = async |drink: &str| {
        sleep(Duration::from_millis(100)).await;
        format!("一杯{drink}")
    };
    let cup = brew("紅茶").await;
    println!("{cup}");
}
```

執行結果：

```text
一杯紅茶
```

`async |drink: &str| { ... }` 跟 `async fn` 的關係，就是閉包跟 `fn` 的關係：一樣是「呼叫時才執行、得到 `Future`」，但可以擷取外面的變數，也不用取名字。

### 傳進函式：`AsyncFn`、`AsyncFnMut`、`AsyncFnOnce`

要把 async 閉包交給函式，參數型別用 `AsyncFn` 系列，它們對應第 11 章第 4、5 集的 `Fn`、`FnMut`、`FnOnce`：

| 閉包 | async 閉包 | 閉包本體對擷取的變數 |
| --- | --- | --- |
| `Fn` | `AsyncFn` | 只是讀 |
| `FnMut` | `AsyncFnMut` | 會修改 |
| `FnOnce` | `AsyncFnOnce` | 會搬走，只能呼叫一次 |

寫法是 `impl AsyncFn(參數型別) -> 結果型別`，`->` 後面是 `Future` 的 `Output`，也就是 `.await` 之後的結果，例如 `job: impl AsyncFn(&str) -> String`。閉包只是讀外面的變數，就符合 `AsyncFn`，函式可以呼叫它很多次；要在閉包裡修改外面的變數，參數就寫 `AsyncFnMut`，而且要寫 `mut job`（跟第 11 章的 `FnMut` 一樣）；要把變數搬走（`async move || ...`），就是 `AsyncFnOnce`，只能呼叫一次。

### 為什麼不用「回傳 async 區塊的閉包」

在 async 閉包出現之前（Rust 1.85 之前），要得到同樣的效果，得寫一個回傳 async 區塊的普通閉包，函式則要多一個泛型 `Fut` 來表示「閉包回傳的那個 `Future`」。碰到要修改外面變數的時候，這種寫法會出問題：

```rust,compile_fail=E0507
use std::time::Duration;
use tokio::time::sleep;

async fn with_retry<F, Fut>(attempts: u32, mut job: F) -> bool
where
    F: FnMut(u32) -> Fut,
    Fut: Future<Output = bool>,
{
    for n in 1..=attempts {
        if job(n).await {
            return true;
        }
    }
    false
}

#[tokio::main]
async fn main() {
    let mut log = Vec::new();
    let ok = with_retry(3, |n| async move {
        sleep(Duration::from_millis(10)).await;
        log.push(n);
        n == 2
    })
    .await;
    println!("{ok} {log:?}");
}
```

編譯錯誤：

```text
error[E0507]: cannot move out of `log`, a captured variable in an `FnMut` closure
  --> src/main.rs:20:32
   |
19 |     let mut log = Vec::new();
   |         -------   ---------- move occurs because `log` has type `Vec<u32>`, which does not implement the `Copy` trait
   |         |
   |         captured outer variable
20 |     let ok = with_retry(3, |n| async move {
   |                            --- ^^^^^^^^^^ `log` is moved here
   |                            |
   |                            captured by this `FnMut` closure
21 |         sleep(Duration::from_millis(10)).await;
22 |         log.push(n);
   |         --- variable moved due to use in coroutine
   |
```

只貼前半段，後面還有第二個錯誤（`log` 已經被搬走）和 `help`。問題在於：閉包每次被呼叫，都要做出一個新的 `Future`，這個 `Future` 要用 `log`，只能把 `log` 搬進去，可是 `FnMut` 的閉包不能把擷取到的變數搬走（它還要再被呼叫）。換成 `async move` 以外的寫法（單純 `async { ... }`），會變成另一種錯誤：`Future` 借著閉包的變數，卻要活得比閉包這一次的呼叫還久。

async 閉包就沒有這個問題：它做出來的 `Future` 可以直接借閉包擷取的變數。

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn with_retry(attempts: u32, mut job: impl AsyncFnMut(u32) -> bool) -> bool {
    for n in 1..=attempts {
        if job(n).await {
            return true;
        }
    }
    false
}

#[tokio::main]
async fn main() {
    let mut log = Vec::new();
    let ok = with_retry(3, async |n| {
        sleep(Duration::from_millis(10)).await;
        log.push(n);
        n == 2
    })
    .await;
    println!("{ok} {log:?}");
}
```

執行結果：

```text
true [1, 2]
```

同樣的邏輯，不用多一個 `Fut` 泛型，閉包裡也能放心 `log.push(n)`：`with_retry` 試到第 2 次成功就停，`log` 記下了 `[1, 2]`。

## 重點整理

- `async |參數| { ... }` 是 async 閉包：呼叫它得到 `Future`，要 `.await`；`async move |參數| { ... }` 把擷取的變數搬進去。
- 傳進函式用 `impl AsyncFn(參數) -> 結果`、`AsyncFnMut`、`AsyncFnOnce`，對應 `Fn`、`FnMut`、`FnOnce`；`AsyncFnMut` 的參數要寫 `mut job`。
- 舊寫法是「回傳 async 區塊的閉包」加 `F: FnMut(..) -> Fut, Fut: Future` 泛型；要在 async 部分修改擷取的變數會是 E0507（或 E0373）。
- async 閉包做出來的 `Future` 可以借閉包擷取的變數，所以不用泛型 `Fut`，也沒有這個問題。
