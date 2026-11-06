# 第一個 axum 伺服器

## 本集目標

用 axum 寫一個回應「歡迎光臨」的 HTTP 伺服器，跑起來用 `curl` 打它，並認得 handler、`Router`、`axum::serve` 這三個零件。

## 正文

### 專案與相依套件

這一章的專案是飲料店的線上菜單，叫 `teashop`，每一集都是一個能編譯、能測試的小專案。axum 是 Web 框架，建在 tokio 上（第 15 章），所以兩個都要：

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
tower = { version = "0.5.3", features = ["util"] }
```

`[dev-dependencies]` 是只有測試才用的套件（第 12 章第 14 集），後面的「不開伺服器也能測」會用到。

### handler 與 `Router`

axum 的 handler 就是 `async fn`：收請求的資料，回傳一個能變成 HTTP 回應的值。`Router` 負責「哪個路徑、哪種方法交給哪個 handler」。照第 12 章第 2 集的分工，路由放在 library，`src/main.rs` 只負責開伺服器：

`src/lib.rs`：

```rust,ignore
use axum::Router;
use axum::routing::get;

async fn home() -> &'static str {
    "歡迎光臨"
}

pub fn app() -> Router {
    Router::new().route("/", get(home))
}

#[cfg(test)]
mod tests;
```

- `home` 回傳 `&'static str`（第 10 章第 7 集）。字串會變成狀態碼 200、`content-type: text/plain; charset=utf-8` 的回應，不必自己組。
- `route("/", get(home))`：路徑 `/` 收到 GET 請求，就呼叫 `home`。`get(...)` 把 handler 包成「只收 GET」。
- 路由獨立成 `app()`，測試不必開伺服器就能用它（本集最後一節）。
- 最後的 `mod tests;` 是測試用的檔案，先放著。

### 開伺服器

`src/main.rs`：

```rust,ignore
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("無法綁定 127.0.0.1:3000");
    println!("伺服器啟動：http://127.0.0.1:3000");
    axum::serve(listener, teashop::app())
        .await
        .expect("伺服器出錯");
}
```

- `TcpListener::bind` 佔住一個位址和連接埠（port）。`127.0.0.1` 只有本機連得進來；對外開放要用 `0.0.0.0`。
- `axum::serve` 接手這個 listener，一直收連線、交給 `Router`，所以它不會自己結束。
- `#[tokio::main]` 是第 15 章第 4 集看過的寫法：`serve` 是 async，`main` 也得是。

`cargo build` 的輸出（省略相依套件的編譯進度）：

```text
   Compiling teashop v0.1.0 (/home/you/teashop)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.64s
```

### 跑起來

`cargo run` 第一次要下載、編譯 axum 與 tokio 等一串套件，要等一陣子。編完之後，程式印出一行就停在那裡等連線。另開一個終端機用 `curl` 打（`-s` 不印進度）：

```text
$ cargo run
     Running `target/debug/teashop`
伺服器啟動：http://127.0.0.1:3000
```

```text
$ curl -s localhost:3000/
歡迎光臨
```

加上 `-i` 連回應標頭一起印，順便看看沒有人認領的路徑會怎樣：

```text
$ curl -i -s localhost:3000/
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 12
date: Fri, 06 Nov 2026 04:42:10 GMT

歡迎光臨
$ curl -i -s localhost:3000/menu
HTTP/1.1 404 Not Found
content-length: 0
date: Fri, 06 Nov 2026 04:42:10 GMT
```

`content-length: 12` 是 UTF-8 的位元組數：四個中文字，每個三個位元組；`date` 每次都不同。沒有人認領的路徑，axum 自動回 404。要停掉伺服器，在跑 `cargo run` 的終端機按 Ctrl-C（第 8 集再讓它收得更體面）。

### 不開伺服器也能測

不必真的佔一個連接埠，也能確認 `app()` 回應什麼：把請求直接交給 `Router`，拿回回應。這段先照抄，用法在第 9 集正式說明。本章每一集的測試檔都是這個樣子，只換測試案例。`src/lib.rs` 最後那行 `mod tests;` 對應的就是這個檔案（第 12 章第 6 集）：

`src/tests.rs`：

```rust,ignore
use super::*;
use axum::body::Body;
use axum::http::Request;
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn send(app: &Router, method: &str, path: &str, body: &str) -> (u16, String) {
    let request = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("建立請求失敗");
    let response = app.clone().oneshot(request).await.expect("路由不會出錯");
    let status = response.status().as_u16();
    let collected = response.into_body().collect().await.expect("讀取失敗");
    let text = String::from_utf8_lossy(&collected.to_bytes()).into_owned();
    (status, text)
}

#[tokio::test]
async fn home_says_welcome() {
    let reply = send(&app(), "GET", "/", "").await;
    assert_eq!(reply, (200, String::from("歡迎光臨")));
}

#[tokio::test]
async fn other_paths_are_404() {
    let reply = send(&app(), "GET", "/menu", "").await;
    assert_eq!(reply, (404, String::new()));
}
```

這一章都用 `cargo test --lib`，只跑 library 裡的測試；輸出裡 `teashop-` 後面的雜湊與秒數，每個人都不同。

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-8ceb485ecabf367b)

running 2 tests
test tests::other_paths_are_404 ... ok
test tests::home_says_welcome ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`send` 送出一個請求，回傳（狀態碼，body 文字）。`home_says_welcome` 確認 `/` 回 200 加「歡迎光臨」，`other_paths_are_404` 確認別的路徑是 404、沒有 body。以後的測試都用 `send`，只比對狀態碼與文字。

### 跟 Go 比一比

Go 的 `net/http` 長得很像：

```go
func home(w http.ResponseWriter, r *http.Request) {
	fmt.Fprint(w, "歡迎光臨")
}

func main() {
	http.HandleFunc("/", home)
	http.ListenAndServe("127.0.0.1:3000", nil)
}
```

- Go 的 handler 把內容寫進 `w`，什麼都沒寫就是空的 200；axum 的 handler 回傳一個值，型別決定回應長什麼樣。
- Go 的 `"/"` 是兜底：任何沒人認領的路徑都會落到它，`curl localhost:3000/menu` 照樣得到「歡迎光臨」。axum 的 `"/"` 只有 `/` 自己，其他路徑是 404。
- Go 的 goroutine 排程是內建的；axum 要靠 `#[tokio::main]` 啟動 runtime。

## 重點整理

- axum 的 handler 是 `async fn`，回傳值（像 `&'static str`）會變成 HTTP 回應；`Router::new().route(路徑, get(handler))` 把路徑與 handler 接起來。
- `TcpListener::bind` 加 `axum::serve` 開伺服器，`#[tokio::main]` 提供 runtime；`serve` 不會自己結束，停掉要按 Ctrl-C。
- 路由放進 library 的 `pub fn app() -> Router`，`main` 只負責開伺服器，測試才能直接用 `app()`。
- 沒有人認領的路徑，axum 回 404；跟 Go 比：Go 的 `"/"` 兜底所有路徑，axum 的 `"/"` 只有 `/`。
