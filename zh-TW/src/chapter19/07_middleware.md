# middleware（tower）

## 本集目標

用 `middleware::from_fn` 寫 middleware：在 handler 前後做事（記錄請求）、在 handler 之前就擋下請求（檢查 API key）；知道 `layer` 與 `route_layer` 的差別、多個 middleware 的先後順序，以及它跟 tower 的關係。

## 正文

### middleware 是包在 handler 外面的一層

有些事每個請求都要做：記錄、檢查身分、計時。不想每個 handler 都寫一遍，就寫成 middleware：它包在 handler 外面，請求進來先經過它，回應出去又經過它。axum 的 middleware 建在 tower 上（第 1 集的 `tower` 就是它）：tower 把「接請求、回應」抽象成 `Service`，把「包住一個 `Service`」抽象成 `Layer`，middleware 就是 `Layer`。這一集不碰這兩個 trait，用 axum 的 `from_fn`，把一個 `async fn` 變成 `Layer`。`Cargo.toml` 跟第 1 集一樣，`src/main.rs` 照舊：

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

### 寫兩個 middleware

`from_fn` 要的函式收兩個參數：`Request`（請求）和 `Next`（「接下來的一切」：後面的 middleware 與 handler）。呼叫 `next.run(request).await` 就是放行，拿回後面產生的 `Response`。

`src/lib.rs`：

```rust,ignore
use axum::Router;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::get;

async fn menu() -> &'static str {
    "紅茶 30\n咖啡 45\n奶茶 38"
}

async fn stats() -> &'static str {
    "今天賣了 0 杯"
}

async fn log(request: Request, next: Next) -> Response {
    let line = format!("{} {}", request.method(), request.uri().path());
    let response = next.run(request).await;
    println!("{line} -> {}", response.status());
    response
}

async fn require_key(request: Request, next: Next) -> Result<Response, StatusCode> {
    match request.headers().get("x-api-key") {
        Some(key) if key == "secret" => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

pub fn app() -> Router {
    let admin = Router::new()
        .route("/stats", get(stats))
        .route_layer(middleware::from_fn(require_key));
    Router::new()
        .route("/menu", get(menu))
        .nest("/admin", admin)
        .layer(middleware::from_fn(log))
}

#[cfg(test)]
mod tests;
```

- `log`：放行之前先記下方法與路徑（放行之後 `request` 就被 move 走了），回應回來再印出狀態碼。放行前後都有機會動手腳。
- `require_key`：沒有 `x-api-key: secret` 就不呼叫 `next.run`，直接回 `Err(401)`。請求到這裡就結束，後面的 handler 完全不會被執行。
- `route_layer` 把 `require_key` 只套在 `admin` 這個 `Router` 的路由上。`app()` 最後的 `layer(from_fn(log))` 套在整個 `Router`，也包括 `nest` 進去的 `/admin`。

### 跑起來

```text
$ curl -s localhost:3000/menu
紅茶 30
咖啡 45
奶茶 38
$ curl -s -o /dev/null -w '%{http_code}\n' localhost:3000/admin/stats
401
$ curl -s -H 'x-api-key: secret' localhost:3000/admin/stats
今天賣了 0 杯
```

伺服器那邊的終端機，多了 `log` 印的三行：

```text
GET /menu -> 200 OK
GET /admin/stats -> 401 Unauthorized
GET /admin/stats -> 200 OK
```

被擋下的 `/admin/stats` 也有一行，狀態碼是 401：`log` 在 `require_key` 外面，連被擋下的請求都看得到。

### `layer`、`route_layer` 與順序

- `layer` 套在整個 `Router` 上，找不到路徑時的 `fallback` 也包括在內；`route_layer` 只套在已經存在的路由上，`fallback` 不受影響，所以沒有 key 去敲不存在的 `/admin/nope`，得到的是 404，不是 401（下面的測試確認過）。
- `layer` 只包住「它之前」加進去的路由；`layer` 之後才 `route` 的路徑沒有 middleware。實測：`.route("/x", ...).layer(a).route("/y", ...)`，請求 `/y` 不會經過 `a`。所以 `layer` 一律寫在最後。
- 串兩個 `layer`，後面的在外面：`.layer(a).layer(b)` 的請求先經過 `b`，再經過 `a`，再到 handler。想照由上往下的順序讀，用 tower 的 `ServiceBuilder`：`ServiceBuilder::new().layer(a).layer(b)` 是請求先 `a` 後 `b`（用 `use tower::ServiceBuilder;`，整串再交給一個 `.layer(...)`）。這兩種寫法的順序，實測都跟 `println!` 印出的進出訊息對得上。

### 測試

`send` 照舊；要帶 `x-api-key` 標頭的請求，這一集另寫了 `admin_stats`（第 9 集會統一說明怎麼建請求）：

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

async fn admin_stats(app: &Router, key: &str) -> u16 {
    let request = Request::builder()
        .uri("/admin/stats")
        .header("x-api-key", key)
        .body(Body::empty())
        .expect("建立請求失敗");
    let response = app.clone().oneshot(request).await.expect("路由不會出錯");
    response.status().as_u16()
}

#[tokio::test]
async fn menu_is_public() {
    let reply = send(&app(), "GET", "/menu", "").await;
    assert_eq!(reply, (200, String::from("紅茶 30\n咖啡 45\n奶茶 38")));
}

#[tokio::test]
async fn admin_needs_the_key() {
    let app = app();
    assert_eq!(send(&app, "GET", "/admin/stats", "").await.0, 401);
    assert_eq!(admin_stats(&app, "wrong").await, 401);
    assert_eq!(admin_stats(&app, "secret").await, 200);
}

#[tokio::test]
async fn route_layer_leaves_the_fallback_alone() {
    let reply = send(&app(), "GET", "/admin/nope", "").await;
    assert_eq!(reply.0, 404);
}
```

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-8ceb485ecabf367b)

running 3 tests
test tests::menu_is_public ... ok
test tests::route_layer_leaves_the_fallback_alone ... ok
test tests::admin_needs_the_key ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### 跟 Go 比一比

Go 的 middleware 是「收一個 `http.Handler`、回一個 `http.Handler`」的函式，順序就是你寫的巢狀：

```go
handler := logging(auth(mux))
```

- Go 的順序一目了然，最外面的先寫；axum 的 `.layer(a).layer(b)` 反過來，最後加的在最外面，容易搞反（`ServiceBuilder` 才是由上往下）。
- Go 的 `http.ResponseWriter` 沒有「目前狀態碼」可以讀，`logging` 要看狀態碼，得自己包一層會記錄狀態碼的 `ResponseWriter`；axum 的 `next.run(request).await` 直接回傳 `Response`，`response.status()` 就有了。

## 重點整理

- middleware 是包在 handler 外面的一層，用 `middleware::from_fn(async fn(Request, Next) -> Response)` 寫；`next.run(request).await` 放行，不呼叫就是在 handler 之前擋下請求。
- `Router::layer` 套整個 `Router`（含 `fallback`），而且只包住它之前的路由；`route_layer` 只包路由，不含 `fallback`。
- 多個 `layer` 串在一起，後加的在外面；tower 的 `ServiceBuilder` 由上往下是由外往內。
- axum 的 middleware 就是 tower 的 `Layer`；常用的現成 middleware（日誌、CORS）在 `tower-http`，這本書不展開。
- 跟 Go 比：Go 的順序是你寫的巢狀，axum 的 `.layer` 鏈反過來；axum 的 middleware 直接讀得到狀態碼。
