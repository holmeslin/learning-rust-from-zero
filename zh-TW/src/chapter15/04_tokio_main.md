# tokio 與 `#[tokio::main]`

## 本集目標

知道 runtime 負責什麼、怎麼把 tokio 加進專案，看懂 `#[tokio::main]` 替你做了哪些事，也會挑 `current_thread` 或預設的多執行緒 runtime。

## 正文

### runtime 負責什麼

前幾集的 `Future` 自己不會動，要有人呼叫 `poll`。這個人就是 runtime，它做三件事：

- 排程：決定哪個任務現在可以跑，反覆 `poll` 它們，遇到 `Pending` 就換下一個。
- 計時：`sleep` 這類等待，時間到了要叫醒對應的任務。
- 非同步 I/O：等網路、等檔案，資料好了叫醒任務（第 19 章的 axum 伺服器就建立在這上面）。

標準函式庫沒有附 runtime，社群有好幾個，tokio 是最常用的，這本書的範例都用它。

### 把 tokio 加進專案

跟第 12 章第 14、15 集教的一樣，用 `cargo add` 加相依套件，並打開 feature：

```bash
cargo add tokio --features full
```

`Cargo.toml` 會多一行（版本以這本書驗證過的為準）：

```toml
[dependencies]
tokio = { version = "1.54.1", features = ["full"] }
```

tokio 把功能切成很多 feature，`full` 是全部打開，學習時最省事；正式的專案可以只開用得到的，編譯會快一點。

### `#[tokio::main]` 替你做了什麼

前幾集一直照抄的 `#[tokio::main]`，是替 `main` 加工的屬性（它是巨集，第 16 章第 4 集再說明）。它做的事，自己寫出來是這樣：

```rust
fn main() {
    let runtime = tokio::runtime::Runtime::new().expect("無法啟動 runtime");
    runtime.block_on(async {
        println!("開店");
    });
}
```

執行結果：

```text
開店
```

- `Runtime::new()` 啟動一個 runtime，`expect` 處理啟動失敗（第 9 章第 4 集）。
- `block_on` 把一個 `Future` 在目前這條執行緒上跑到結束，才回傳。它是同步世界進入 async 世界的入口。
- `#[tokio::main]` 就是把 `async fn main()` 的內容包進 `block_on(async { ... })`，所以寫出來的 `main` 才能是 async。

`main` 直接寫成 `async fn`、卻沒有 `#[tokio::main]`，編譯器會擋下：

```rust,compile_fail=E0752
async fn main() {
    println!("開店");
}
```

編譯錯誤：

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:1:1
  |
1 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

要注意：已經在 async 裡面了，不能再對另一個 runtime 呼叫 `block_on`，否則會 panic，訊息是 `Cannot start a runtime from within a runtime`。在 async 裡要等 `Future`，一律用 `.await`。

### 兩種 runtime

`#[tokio::main]` 預設啟動多執行緒的 runtime：開一批工作執行緒（預設跟 CPU 核心數一樣多），任務可以在它們之間移動，同時有好幾個任務在不同的核心上跑。

不需要平行、或想讓行為單純一點時，可以改成只用一條執行緒：

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let workers = tokio::runtime::Handle::current().metrics().num_workers();
    println!("worker 數量：{workers}");
}
```

執行結果：

```text
worker 數量：1
```

- `flavor = "current_thread"`：所有任務都在目前這一條執行緒上輪流跑，同一時間只有一個任務在動。
- `worker_threads = 2` 這類參數可以指定多執行緒版本的工作執行緒數量，寫成 `#[tokio::main(worker_threads = 2)]`。
- `Handle::current().metrics().num_workers()` 查目前 runtime 有幾條工作執行緒，這裡只是拿來觀察，平常用不到。

單執行緒版本的好處是任務只會在 `.await` 的地方切換，行為好預測；第 12 集會用它示範「卡住 runtime」是怎麼回事。多執行緒版本能同時用上多顆核心，所以任務裡用到的資料要能安全跨執行緒搬動，這是第 14 集的主題。

## 重點整理

- runtime 負責排程任務（反覆 `poll`）、計時與非同步 I/O；標準函式庫沒有附，這本書用 tokio。
- 加入 tokio：`cargo add tokio --features full`，`Cargo.toml` 寫 `tokio = { version = "1.54.1", features = ["full"] }`。
- `#[tokio::main]` 把 `async fn main()` 包成 `Runtime::new()` 加 `block_on(async { ... })`；`main` 不能直接寫成 `async fn`（E0752）；在 async 裡不能再 `block_on`。
- 預設是多執行緒 runtime（工作執行緒數量預設是 CPU 核心數，`worker_threads` 可指定）；`flavor = "current_thread"` 只用一條執行緒，任務只在 `.await` 處切換。
