# JSON API

## 本集目標

用 `Json<T>` 收發 JSON：回應時把 struct 變成 JSON，請求時把 JSON 解析成 struct，並知道格式不對時 axum 回什麼。

## 正文

### `Json` 同時是 extractor 與回應

`Json<T>` 做兩件事。放在參數位置，它是 extractor：讀請求的 body，用 serde 解析成 `T`（`T` 要 `Deserialize`）。放在回傳值，它是回應：把 `T` 序列化成 JSON，並加上 `content-type: application/json`（`T` 要 `Serialize`）。`Cargo.toml` 跟上一集一樣，`src/main.rs` 照舊：

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
tower = { version = "0.5.3", features = ["util"] }
```

兩個端點：`GET /menu` 回菜單；`POST /orders` 收一筆訂單，確認飲料在菜單上，就原樣回給你（狀態碼 201）：

`src/lib.rs`：

```rust,ignore
use axum::Json;
use axum::Router;
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Drink {
    id: &'static str,
    name: &'static str,
    price: u32,
}

fn drink(id: &'static str, name: &'static str, price: u32) -> Drink {
    Drink { id, name, price }
}

fn drinks() -> Vec<Drink> {
    vec![
        drink("tea", "紅茶", 30),
        drink("coffee", "咖啡", 45),
        drink("milk-tea", "奶茶", 38),
    ]
}

#[derive(Deserialize, Serialize)]
struct Order {
    drink: String,
    cups: u8,
}

async fn menu() -> Json<Vec<Drink>> {
    Json(drinks())
}

async fn create_order(Json(order): Json<Order>) -> Result<(StatusCode, Json<Order>), StatusCode> {
    if drinks().iter().any(|d| d.id == order.drink) {
        Ok((StatusCode::CREATED, Json(order)))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub fn app() -> Router {
    Router::new()
        .route("/menu", get(menu))
        .route("/orders", post(create_order))
}

#[cfg(test)]
mod tests;
```

- 只會被送出去的型別（`Drink`）`derive(Serialize)`；`Order` 既要收也要送，兩個都 `derive`（第 17 章第 9 集）。`Json<Vec<Drink>>` 把整個 `Vec` 變成 JSON 陣列。
- `Json(order): Json<Order>` 拆開之後，`order` 就是解析好的 `Order`。回傳 `Result<(StatusCode, Json<Order>), StatusCode>`：成功是（201，`Json(order)`），飲料不在菜單上是單獨一個 404。
- `Json` 會吃掉整個 body，只能有一個，而且要放在所有 extractor 的最後。放錯位置，編譯器只會說某個 `Handler<_, _>` 沒有實作，不會點名是哪個參數；看到這個訊息，先檢查參數的型別與順序。

### 跑起來

```text
$ curl -s localhost:3000/menu
[{"id":"tea","name":"紅茶","price":30},{"id":"coffee","name":"咖啡","price":45},{"id":"milk-tea","name":"奶茶","price":38}]
$ curl -i -s -X POST localhost:3000/orders -H 'content-type: application/json' -d '{"drink":"tea","cups":2}'
HTTP/1.1 201 Created
content-type: application/json
content-length: 24
date: Fri, 06 Nov 2026 05:00:00 GMT

{"drink":"tea","cups":2}
```

### 格式不對的時候

跟 `Path`、`Query` 一樣，body 不合格就不會進 handler，axum 依原因回不同的狀態碼。`-w` 在回應後面多印一行狀態碼（中括號裡）：

```text
$ curl -s -w '\n[%{http_code}]\n' -X POST localhost:3000/orders -H 'content-type: application/json' -d '{"drink":"tea"}'
Failed to deserialize the JSON body into the target type: missing field `cups` at line 1 column 15
[422]
$ curl -s -w '\n[%{http_code}]\n' -X POST localhost:3000/orders -d '{"drink":"tea","cups":2}'
Expected request with `Content-Type: application/json`
[415]
```

- 422：JSON 沒問題，但欄位對不上 `Order`（少了 `cups`，或型別不對）；400：連 JSON 都不是（測試裡有）。
- 415：沒有 `content-type: application/json`。`curl -d` 預設送的是 `application/x-www-form-urlencoded`，所以被擋下。

### 測試

`send` 一律帶 `content-type: application/json`，所以 415 留給上面的 `curl`，測試只管其他三種：

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
async fn menu_is_a_json_array() {
    let (status, body) = send(&app(), "GET", "/menu", "").await;
    assert_eq!(status, 200);
    assert!(body.starts_with(r#"[{"id":"tea","name":"紅茶","price":30},"#));
}

#[tokio::test]
async fn order_is_echoed_back() {
    let app = app();
    let order = r#"{"drink":"tea","cups":2}"#;
    let reply = send(&app, "POST", "/orders", order).await;
    assert_eq!(reply, (201, String::from(order)));
    let juice = r#"{"drink":"juice","cups":1}"#;
    assert_eq!(send(&app, "POST", "/orders", juice).await.0, 404);
}

#[tokio::test]
async fn bad_body_never_reaches_the_handler() {
    let app = app();
    let missing = send(&app, "POST", "/orders", r#"{"drink":"tea"}"#).await;
    assert_eq!(missing.0, 422);
    assert_eq!(send(&app, "POST", "/orders", "tea").await.0, 400);
}
```

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-14e6bdfe9c81aa39)

running 3 tests
test tests::menu_is_a_json_array ... ok
test tests::order_is_echoed_back ... ok
test tests::bad_body_never_reaches_the_handler ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### 跟 Go 比一比

Go 的 `encoding/json` 要自己 `Decode`，也要自己決定錯了怎麼辦：

```go
var order Order
err := json.NewDecoder(r.Body).Decode(&order)
```

- 少了欄位，Go 的 `err` 是 `nil`，欄位悄悄是零值（`Cups` 是 0）；axum 的 `Order` 少了 `cups` 就是 422，進不了 handler。想要「可以不給」，在 Rust 明確寫成 `Option<u8>`。

## 重點整理

- `Json<T>` 在參數位置是 extractor（`T: Deserialize`），在回傳位置是回應（`T: Serialize`，自動加 `content-type: application/json`）。
- `Json` 要放在所有 extractor 的最後一個；順序錯了，編譯器只說 `Handler<_, _>` 沒有實作。
- body 不合格，axum 不呼叫 handler：不是 JSON 是 400、欄位對不上是 422、缺 `content-type: application/json` 是 415。
- 跟 Go 比：Go 缺欄位悄悄是零值；axum 缺欄位就是 422，除非型別寫成 `Option`。
