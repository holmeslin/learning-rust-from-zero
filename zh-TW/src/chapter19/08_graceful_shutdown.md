# graceful shutdown

## 本集目標

讓伺服器收到 Ctrl-C 時好好收工：不再接新的連線，把進行中的請求處理完，才結束程式。

## 正文

### 直接關掉會怎樣

前幾集要停伺服器，只能按 Ctrl-C，行程當場結束。這時候正在處理的請求，客人拿到的是被切斷的連線。graceful shutdown（優雅關閉）的順序是：收到關閉訊號、不再接新連線、等進行中的請求做完、再結束。axum 的 `serve` 提供這個功能，只要告訴它「什麼時候該關」。

### `with_graceful_shutdown`

這一集多一個 `/slow`，泡一杯要 1 秒，好讓我們有機會在請求進行中按 Ctrl-C。為了能測試，啟動伺服器的動作放進 library 的 `serve`，關閉的訊號當參數傳進去：任何 `Future` 都行，它一完成，關閉就開始。`Cargo.toml` 多了 `tokio-util`，測試要用（第 15 章第 10 集）：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.9"
tokio = { version = "1.54.1", features = ["full"] }

[dev-dependencies]
http-body-util = "0.1.5"
tokio-util = "0.7.20"
tower = { version = "0.5.3", features = ["util"] }
```

`src/lib.rs`：

```rust,ignore
use axum::Router;
use axum::routing::get;
use std::time::Duration;
use tokio::net::TcpListener;

async fn slow() -> &'static str {
    tokio::time::sleep(Duration::from_secs(1)).await;
    "泡好了"
}

pub fn app() -> Router {
    Router::new().route("/slow", get(slow))
}

pub async fn serve(listener: TcpListener, shutdown: impl Future<Output = ()> + Send + 'static) {
    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown)
        .await
        .expect("伺服器出錯");
}

#[cfg(test)]
mod tests;
```

`src/main.rs`：

```rust,ignore
use tokio::net::TcpListener;
use tokio::signal;

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("無法監聽 Ctrl-C");
    println!("收到 Ctrl-C，等進行中的請求做完");
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("無法綁定 127.0.0.1:3000");
    println!("伺服器啟動：http://127.0.0.1:3000");
    teashop::serve(listener, shutdown_signal()).await;
    println!("伺服器已關閉");
}
```

- `serve` 把 listener 與「關閉的訊號」交給 axum：`.with_graceful_shutdown(shutdown)` 這個 future 完成、而且進行中的請求都做完了，`.await` 才會回來。
- `main` 傳進去的訊號是 `shutdown_signal()`：等 `ctrl_c()`（第 18 章第 7 集），收到就印一行並結束，`serve` 隨之開始收工。`serve` 回來之後，才印「伺服器已關閉」。
- Linux 的容器（Docker、Kubernetes）通常送 SIGTERM 要程式結束，不是 Ctrl-C。Unix 上要另外等 `tokio::signal::unix::signal(SignalKind::terminate())`，再用 `select!`（第 15 章第 8 集）跟 `ctrl_c()` 合起來等，這本書只示範 Ctrl-C。

`cargo build` 的輸出（省略相依套件的編譯進度）：

```text
   Compiling teashop v0.1.0 (/home/you/teashop)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.38s
```

### 實際看看

終端機甲跑伺服器，終端機乙去要一杯要等 1 秒的飲料，等的時候，到終端機甲按 Ctrl-C：

```text
$ cargo run
     Running `target/debug/teashop`
伺服器啟動：http://127.0.0.1:3000
```

```text
$ curl -s localhost:3000/slow
泡好了
```

伺服器那邊的終端機，按下 Ctrl-C 之後：

```text
^C
收到 Ctrl-C，等進行中的請求做完
伺服器已關閉
```

`curl` 照樣拿到「泡好了」，程式在那杯做完之後才結束。

### 測試

`oneshot` 不經過真正的 listener，測不到關閉，所以這個測試自己開伺服器：綁在 `127.0.0.1:0`（port 0 是請作業系統挑一個沒人用的，不會跟別的程式搶），用 `tokio::spawn` 讓伺服器與「客人」並行。客人是手寫的 TCP 連線，把 HTTP 請求當文字直接送出去；讀寫用 tokio 的版本，就是第 17 章第 1 集 `Read`／`Write` 的 async 版。客人送出之後，等 200 毫秒（請求一定進來了、還沒泡完），再取消 token，訊號就發出去了：

`src/tests.rs`：

```rust,ignore
use super::*;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::sync::CancellationToken;

async fn raw_get(addr: SocketAddr, path: &str) -> String {
    let mut stream = TcpStream::connect(addr).await.expect("連不上伺服器");
    let request = format!("GET {path} HTTP/1.1\r\nhost: test\r\nconnection: close\r\n\r\n");
    let bytes = request.as_bytes();
    stream.write_all(bytes).await.expect("寫入失敗");
    let mut reply = String::new();
    stream.read_to_string(&mut reply).await.expect("讀取失敗");
    reply
}

#[tokio::test]
async fn shutdown_waits_for_the_request_in_flight() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("無法綁定");
    let addr = listener.local_addr().expect("沒有位址");
    let token = CancellationToken::new();
    let server = tokio::spawn(serve(listener, token.clone().cancelled_owned()));

    let client = tokio::spawn(raw_get(addr, "/slow"));
    tokio::time::sleep(Duration::from_millis(200)).await;
    token.cancel();

    let reply = client.await.expect("客人的任務出錯");
    assert!(reply.starts_with("HTTP/1.1 200 OK"));
    assert!(reply.ends_with("泡好了"));
    server.await.expect("伺服器的任務出錯");
    assert!(TcpStream::connect(addr).await.is_err());
}
```

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-25f06a4bb206a44f)

running 1 test
test tests::shutdown_waits_for_the_request_in_flight ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.01s
```

測試確認三件事：關閉訊號發出之後，進行中的請求照樣收到完整的回應；`serve` 隨後結束；結束之後再連就連不上。

### 跟 Go 比一比

Go 的 `http.Server` 也有 `Shutdown`，常見的寫法：

```go
go func() {
	<-ctx.Done() // signal.NotifyContext 的 Ctrl-C
	srv.Shutdown(context.Background())
}()
err := srv.ListenAndServe()
```

- 陷阱：`Shutdown` 一被呼叫，`ListenAndServe` 就立刻回傳 `http.ErrServerClosed`，不會等請求做完；真正等的是 `Shutdown` 自己。`main` 在 `ListenAndServe` 回來後就結束，進行中的請求照樣被切斷（實測：`curl` 得到 `Empty reply from server`），得另外等 `Shutdown` 回來。
- axum 的 `serve(...).with_graceful_shutdown(...).await` 回來的時候，請求就已經做完了，不會有這個落差。

## 重點整理

- graceful shutdown：收到訊號，不再接新連線，等進行中的請求做完，再結束。
- `axum::serve(listener, app).with_graceful_shutdown(future)`：`future` 完成就開始收工，`.await` 等到請求都做完才回來。
- 訊號可以是任何 `Future`：`tokio::signal::ctrl_c()`（Ctrl-C）、`CancellationToken::cancelled_owned()`；容器常用的 SIGTERM 要另外等 `signal::unix`。
- 要測關閉，得自己開伺服器：綁 `127.0.0.1:0`，用 `tokio::spawn` 並行，`oneshot` 測不到。
- 跟 Go 比：Go 的 `ListenAndServe` 在 `Shutdown` 之後立刻回傳，要等的是 `Shutdown`；axum 的 `serve` 等到收工才回來。
