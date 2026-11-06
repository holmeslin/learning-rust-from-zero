# 不要阻塞 runtime：`spawn_blocking`

## 本集目標

知道在任務裡「卡住執行緒」會拖累同一條執行緒上的所有任務；用 `spawn_blocking` 把會卡住、很耗時的工作交給另一批執行緒。

## 正文

### 卡住執行緒，任務全都得等

async 任務是協作式的：任務只在 `.await` 的地方把執行緒讓出來。如果任務裡呼叫了會卡住執行緒、又沒有 `.await` 的函式，這條執行緒就整個被它霸佔，上面的其他任務一個都動不了。

下面用單執行緒的 runtime 看得最清楚。計時任務每 100 毫秒滴答一次，攪拌機任務卻用 `std::thread::sleep` 睡 400 毫秒（假裝在做一件很耗時的事）：

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let ticker = tokio::spawn(async {
        for n in 1..=3 {
            sleep(Duration::from_millis(100)).await;
            println!("滴答 {n}");
        }
    });
    let mixer = tokio::spawn(async {
        println!("攪拌機開始");
        std::thread::sleep(Duration::from_millis(400));
        println!("攪拌機結束");
    });
    ticker.await.expect("計時出事了");
    mixer.await.expect("攪拌機出事了");
}
```

執行結果：

```text
攪拌機開始
攪拌機結束
滴答 1
滴答 2
滴答 3
```

滴答應該在 100 毫秒就響，結果等到攪拌機結束才開始：`std::thread::sleep` 沒有 `.await`，攪拌機任務從頭到尾沒有把執行緒讓出去，計時任務只好排隊。把 `std::thread::sleep` 換成 `tokio::time::sleep(...).await`，滴答就會夾在中間（第 9 集的 `timeout` 失效也是同一個原因）。

多執行緒 runtime 有好幾條工作執行緒，影響沒這麼明顯，但每個卡住的任務就佔掉一條。卡住的任務一多，把所有工作執行緒都佔滿，整個 runtime 一樣停擺。

會卡住執行緒的常見東西：

- `std::thread::sleep`，標準函式庫的同步 I/O（第 17 章的讀寫檔案，以及 `std::sync::mpsc` 的 `recv`）。
- 拿不到的 `std::sync::Mutex`。
- 純計算但要算很久的程式（壓縮、加密、大量運算）。

準則是：兩個 `.await` 之間，不要做太久的事。

### `spawn_blocking`：交給另外一批執行緒

`tokio::task::spawn_blocking` 收的是普通的閉包（不是 `Future`），tokio 會把它放到另外準備的執行緒上跑，不會佔用跑任務的工作執行緒。回傳 `JoinHandle`，對它 `.await` 就拿到閉包的回傳值：

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let ticker = tokio::spawn(async {
        for n in 1..=3 {
            sleep(Duration::from_millis(100)).await;
            println!("滴答 {n}");
        }
    });
    let mixer = tokio::task::spawn_blocking(|| {
        println!("攪拌機開始");
        std::thread::sleep(Duration::from_millis(400));
        println!("攪拌機結束");
        "一杯奶昔"
    });
    ticker.await.expect("計時出事了");
    let cup = mixer.await.expect("攪拌機出事了");
    println!("送出{cup}");
}
```

執行結果：

```text
攪拌機開始
滴答 1
滴答 2
滴答 3
攪拌機結束
送出一杯奶昔
```

攪拌機一樣睡了 400 毫秒，但這次睡在別的執行緒上，滴答照常每 100 毫秒響一次。

- 閉包的要求跟 `thread::spawn` 一樣：`Send + 'static`，要用外面的資料就 `move` 進去（第 14 章第 3 集）。
- `.await` 的結果是 `Result<T, JoinError>`，閉包 panic 的話是 `Err`，跟 `tokio::spawn` 一樣。
- 純計算也一樣處理，例如自己寫的 `total_sales` 要算很久：`tokio::task::spawn_blocking(move || total_sales(days))`。
- 已經開始跑的阻塞任務，取消不了。對它的 `JoinHandle` 呼叫 `abort()` 沒有用，閉包還是會跑完、`.await` 還是拿到結果。所以不要在 `spawn_blocking` 裡寫會永遠不結束的迴圈。

### 跟 Go 比一比

Go 沒有這一集的問題。goroutine 在系統呼叫（讀檔、睡覺）卡住的時候，runtime 會把這條執行緒上的其他 goroutine 轉去別條執行緒；一直在算的 goroutine，runtime 也能搶佔（第 1 集說過）。所以在 goroutine 裡直接呼叫會卡住的函式，不會拖累別人。

Rust 的 runtime 不會搶佔，也分不出哪個函式會卡住。哪些工作該放進 `spawn_blocking`，要由你判斷。

## 重點整理

- async 任務只在 `.await` 讓出執行緒；任務裡呼叫沒有 `.await` 又會卡住的函式（`std::thread::sleep`、同步 I/O、拿不到的 `std::sync::Mutex`、很久的計算），會讓同一條執行緒上的其他任務全部等著。
- 準則：兩個 `.await` 之間不要做太久的事。
- `tokio::task::spawn_blocking(閉包)`：把會卡住的閉包交給另一批執行緒，回傳 `JoinHandle`，`.await` 得到 `Result<T, JoinError>`；閉包要 `Send + 'static`。
- 已經開始跑的阻塞任務不能取消，`abort()` 沒有用。
- 跟 Go 比：Go 的 runtime 會自動處理卡住的 goroutine 並能搶佔，Rust 要自己判斷、自己放進 `spawn_blocking`。
