# 路由

## 本集目標

用 `Router` 把不同的路徑與方法分給不同的 handler：同一個路徑掛多個方法、用 `nest` 加上共同的開頭路徑、用 `fallback` 自訂找不到時的回應。

## 正文

### 一個路徑，好幾種方法

菜單網站至少要「看菜單」和「下訂單」。`get(...)` 之外還有 `post(...)`、`put(...)`、`delete(...)`，同一個路徑要接好幾種方法，就把它們串在一起。`src/main.rs` 不用動，第 1 集那份照舊：

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

`src/lib.rs`：

```rust,ignore
use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;

async fn menu() -> &'static str {
    "紅茶 30\n咖啡 45\n奶茶 38"
}

async fn list_orders() -> &'static str {
    "目前沒有訂單"
}

async fn create_order() -> &'static str {
    "收到訂單"
}

async fn stats() -> &'static str {
    "今天賣了 0 杯"
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "這裡沒有這個頁面")
}

fn admin() -> Router {
    Router::new().route("/stats", get(stats))
}

pub fn app() -> Router {
    Router::new()
        .route("/menu", get(menu))
        .route("/orders", get(list_orders).post(create_order))
        .nest("/admin", admin())
        .fallback(not_found)
}

#[cfg(test)]
mod tests;
```

- `.route("/orders", get(list_orders).post(create_order))`：`get(...)` 回傳的東西可以再接 `.post(...)`。同一個路徑，GET 去 `list_orders`，POST 去 `create_order`。
- `route` 可以一直串下去，每個路徑一行，整張路由表就是 `app()` 的內容。
- 路徑對了，方法也要對。`/orders` 沒有掛 DELETE，axum 就回 405（Method Not Allowed），並在 `allow` 標頭列出可以用的方法，不用自己寫。

### `nest`：共同的開頭路徑

管理員用的頁面都放在 `/admin` 底下。把它們收成另一個 `Router`，再用 `nest("/admin", ...)` 掛上去，裡面寫的路徑會自動加上這個開頭：`/stats` 變成 `/admin/stats`。

### `fallback`：找不到的時候

沒有路徑認領時，預設是空 body 的 404。`fallback(...)` 換掉這個預設。`not_found` 回傳（狀態碼，文字）這一對，狀態碼交給你決定；為什麼這樣也算回應，第 6 集再說，現在先照寫。

### 跑起來

```text
$ curl -s localhost:3000/admin/stats
今天賣了 0 杯
$ curl -i -s -X DELETE localhost:3000/orders
HTTP/1.1 405 Method Not Allowed
allow: GET,HEAD,POST
content-length: 0
date: Fri, 06 Nov 2026 04:43:00 GMT
$ curl -i -s localhost:3000/menu/
HTTP/1.1 404 Not Found
content-type: text/plain; charset=utf-8
content-length: 24
date: Fri, 06 Nov 2026 04:43:00 GMT

這裡沒有這個頁面
```

`allow` 裡的 `HEAD` 是 axum 替每個 `get` 自動加的。最後一個要留意：`/menu/` 跟 `/menu` 是兩個不同的路徑，axum 不會幫你轉過去，所以掉到 `fallback`。

### 測試

`send` 跟第 1 集一模一樣，後面四個是這一集的測試案例：

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
async fn one_path_two_methods() {
    let app = app();
    assert_eq!(send(&app, "GET", "/orders", "").await.1, "目前沒有訂單");
    assert_eq!(send(&app, "POST", "/orders", "").await.1, "收到訂單");
}

#[tokio::test]
async fn missing_method_is_405() {
    let reply = send(&app(), "DELETE", "/orders", "").await;
    assert_eq!(reply.0, 405);
}

#[tokio::test]
async fn nest_adds_prefix() {
    let reply = send(&app(), "GET", "/admin/stats", "").await;
    assert_eq!(reply, (200, String::from("今天賣了 0 杯")));
}

#[tokio::test]
async fn fallback_answers_404() {
    let reply = send(&app(), "GET", "/menu/", "").await;
    assert_eq!(reply, (404, String::from("這裡沒有這個頁面")));
}
```

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-8ceb485ecabf367b)

running 4 tests
test tests::nest_adds_prefix ... ok
test tests::one_path_two_methods ... ok
test tests::fallback_answers_404 ... ok
test tests::missing_method_is_405 ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

兩次掛上同一個路徑、同一種方法，`app()` 一執行就 panic：`` Overlapping method route. Handler for `GET /menu` already exists ``，位置指向寫錯的那一行。所以路由寫錯，在啟動的時候就會發現，不會等到有人打進來。

### 跟 Go 比一比

Go 1.22 之後的 `ServeMux` 也能把方法寫進樣式，方法不對一樣自動回 405 加 `Allow`：

```go
mux := http.NewServeMux()
mux.HandleFunc("GET /orders", listOrders)
mux.HandleFunc("POST /orders", createOrder)
mux.Handle("/admin/", http.StripPrefix("/admin", admin))
```

- 差在斜線。Go 的樣式以 `/` 結尾（`"/admin/"`）代表整棵子樹，`/admin` 還會被導向 `/admin/`；axum 的 `/admin` 與 `/admin/` 是兩個不相干的路徑，不導向，也沒有子樹的意思。
- 找不到、方法不對時，Go 預設回一行文字（`404 page not found`）；axum 預設是空 body。

## 重點整理

- 同一個路徑接多種方法，把 `get(...)`、`post(...)` 串在一起；方法不在其中，axum 自動回 405 並附上 `allow`。
- `nest("/admin", router)` 把另一個 `Router` 掛在共同的開頭路徑下；`fallback(handler)` 自訂找不到時的回應。
- `/menu` 與 `/menu/` 是不同的路徑，axum 不轉址；同一路徑、同一方法掛兩次，啟動時就 panic。
- 跟 Go 比：Go 以 `/` 結尾的樣式是整棵子樹，axum 沒有這個意思，要靠 `nest`。
