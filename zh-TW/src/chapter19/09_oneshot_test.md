# 用 `oneshot` 測試

## 本集目標

看懂前八集一直照抄的測試：把請求直接交給 `Router`、拿回回應，不開伺服器、不佔連接埠，也能測路由、extractor、JSON 與 state。

## 正文

### 一支完整的小 API

這一集用一支縮小版的 API，內容取自前面幾集：`GET /orders` 列出訂單，`POST /orders` 新增一筆（JSON、state、404 都有）。測試要比對 JSON，所以 `Cargo.toml` 的 `[dev-dependencies]` 多了 `serde_json`（第 17 章第 8 集）：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.9"
serde = { version = "1.0.229", features = ["derive"] }
tokio = { version = "1.54.1", features = ["full"] }

[dev-dependencies]
http-body-util = "0.1.5"
serde_json = "1.0.154"
tower = { version = "0.5.3", features = ["util"] }
```

`src/lib.rs`：

```rust,ignore
use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize)]
struct Order {
    id: usize,
    drink: String,
    cups: u8,
}

#[derive(Deserialize)]
struct NewOrder {
    drink: String,
    cups: u8,
}

#[derive(Clone)]
struct AppState {
    orders: Arc<Mutex<Vec<Order>>>,
}

async fn create_order(
    State(state): State<AppState>,
    Json(new): Json<NewOrder>,
) -> Result<(StatusCode, Json<Order>), StatusCode> {
    if !["tea", "coffee", "milk-tea"].contains(&new.drink.as_str()) {
        return Err(StatusCode::NOT_FOUND);
    }
    let mut orders = state.orders.lock().expect("鎖中毒");
    let order = Order {
        id: orders.len() + 1,
        drink: new.drink,
        cups: new.cups,
    };
    orders.push(order.clone());
    Ok((StatusCode::CREATED, Json(order)))
}

async fn list_orders(State(state): State<AppState>) -> Json<Vec<Order>> {
    Json(state.orders.lock().expect("鎖中毒").clone())
}

pub fn app() -> Router {
    let state = AppState {
        orders: Arc::new(Mutex::new(Vec::new())),
    };
    Router::new()
        .route("/orders", get(list_orders).post(create_order))
        .with_state(state)
}

#[cfg(test)]
mod tests;
```

`src/main.rs` 還是第 1 集那份。

### `send` 一步一步

`Router` 本身就是 tower 的 `Service`：給它一個請求，它回一個回應，不需要網路。`oneshot` 是 tower 的 `ServiceExt` 提供的方法，意思是「確認準備好、呼叫一次」。`send` 用它走完整的一趟，它就是前八集那一個；後面是這一集的三個測試：

`src/tests.rs`：

```rust,ignore
use super::*;
use axum::body::Body;
use axum::http::Request;
use http_body_util::BodyExt;
use serde_json::Value;
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
async fn orders_are_shared_between_requests() {
    let app = app();
    let created = send(&app, "POST", "/orders", r#"{"drink":"tea","cups":2}"#).await;
    assert_eq!(created.0, 201);
    send(&app, "POST", "/orders", r#"{"drink":"coffee","cups":1}"#).await;

    let (status, body) = send(&app, "GET", "/orders", "").await;
    assert_eq!(status, 200);
    let orders: Value = serde_json::from_str(&body).expect("回應不是 JSON");
    assert_eq!(orders.as_array().map(Vec::len), Some(2));
    assert_eq!(orders[1]["drink"], "coffee");
    assert_eq!(orders[1]["id"], 2);
}

#[tokio::test]
async fn unknown_drink_and_bad_body_are_rejected() {
    let app = app();
    let juice = send(&app, "POST", "/orders", r#"{"drink":"juice","cups":1}"#).await;
    assert_eq!(juice.0, 404);
    let no_cups = send(&app, "POST", "/orders", r#"{"drink":"tea"}"#).await;
    assert_eq!(no_cups.0, 422);
    assert_eq!(send(&app, "GET", "/orders", "").await.1, "[]");
}

#[tokio::test]
async fn response_header_says_json() {
    let request = Request::builder()
        .uri("/orders")
        .body(Body::empty())
        .expect("建立請求失敗");
    let response = app().oneshot(request).await.expect("路由不會出錯");
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "application/json");
}
```

- `Request::builder()` 一步步組出請求：方法、路徑、標頭、body。`Body::from(...)` 是有內容的 body，`Body::empty()` 是空的。`send` 一律帶 `content-type: application/json`，不用 JSON 的請求也不受影響。
- `app.clone().oneshot(request)`：`oneshot` 會用掉那個 `Router`，所以先 `clone`。`Router` 的 clone 很便宜，clone 出來的共用同一份 state，所以同一個測試裡連打好幾次，訂單會累積；每個測試各自呼叫 `app()`，state 互不干擾。
- `.expect("路由不會出錯")`：`oneshot` 的錯誤型別是 `Infallible`（永遠不會發生）。404、422 都是正常的回應，不是 `Err`。
- body 是一串資料塊（chunk），不是字串：`collect().await` 收齊（`http-body-util` 的 `BodyExt`），`to_bytes()` 變成位元組，再 `from_utf8_lossy` 轉成文字。最後回傳（狀態碼，文字），測試就能 `assert_eq!`。

### 比對 JSON、檢查標頭

body 是 JSON 的時候，整段字串比對會被欄位順序、空白卡住。`orders_are_shared_between_requests` 用 `serde_json::from_str` 解析成 `Value`，再用 `["drink"]` 取欄位。要看標頭，就得自己拿 `Response`：`response_header_says_json` 不經過 `send`，直接 `oneshot`，用 `response.headers()["content-type"]` 取 `HeaderMap` 裡的值。

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-7c8e60375a62ba2b)

running 3 tests
test tests::response_header_says_json ... ok
test tests::unknown_drink_and_bad_body_are_rejected ... ok
test tests::orders_are_shared_between_requests ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### `oneshot` 測得到與測不到的

`oneshot` 從 `Router` 開始走：路由、extractor、handler、middleware、錯誤轉換都跑到了，所以前八集的行為都能這樣測。它沒有經過的是 `Router` 之外的東西：真的 TCP 連線、`axum::serve`、關閉訊號。這些用第 8 集那種「綁 `127.0.0.1:0` 自己開伺服器」的測試，數量不必多。

### 跟 Go 比一比

Go 的 `net/http/httptest` 做同一件事：

```go
req := httptest.NewRequest("GET", "/orders", nil)
rec := httptest.NewRecorder()
mux.ServeHTTP(rec, req)
fmt.Println(rec.Code, rec.Body.String())
```

- 兩邊都不開 port，直接呼叫路由。Go 把結果寫進你給的 `ResponseRecorder`，`rec.Body.String()` 就能讀；Rust 回傳 `Response`，body 是串流，要 `collect` 才拿得到全部。要真的開伺服器，Go 用 `httptest.NewServer`（隨機連接埠），axum 就自己綁 `127.0.0.1:0`（第 8 集）。

## 重點整理

- `Router` 是 tower 的 `Service`：`tower::ServiceExt::oneshot(request)` 把請求直接交給它，不開伺服器、不佔連接埠。
- `Request::builder()` 組請求；`oneshot` 會用掉 `Router`，所以 `app.clone().oneshot(...)`，clone 出來的共用同一份 state。
- 回應的 body 是串流：`response.into_body().collect().await` 收齊，`to_bytes()` 取位元組。
- JSON 用 `serde_json::Value` 比對，標頭用 `response.headers()`。
- `oneshot` 測得到路由到 handler 的一切，測不到真的 TCP 與 `serve`，那些另外開伺服器測。
- 跟 Go 比：Go 的 `httptest.NewRecorder` 直接給你寫好的 body；Rust 的 body 要 `collect`。
