# 錯誤回應 `IntoResponse`

## 本集目標

知道 handler 的回傳值為什麼幾乎什麼型別都行（只要實作 `IntoResponse`），並自己定義一個錯誤型別，讓 handler 可以用 `?`、錯誤自動變成帶狀態碼的 JSON 回應。

## 正文

### 能變成回應的型別

handler 的回傳型別只有一個要求：實作 `IntoResponse` trait（第 8 章），也就是知道怎麼把自己變成 HTTP 回應。前面幾集已經用過好幾種：`&'static str` 與 `String`（狀態碼 200，body 是文字）、`StatusCode`（只有狀態碼，沒有 body，像第 4 集的 404）、`Json<T>`（JSON body）、`(StatusCode, 內容)`（在內容前面加上狀態碼，像第 2 集的 `fallback` 與第 4 集的 201）。還有 `Result<T, E>`：`T` 和 `E` 都實作 `IntoResponse`，`Ok` 與 `Err` 各自變成回應。

所以 handler 只要 `return Err(...)` 就能回錯誤，不必自己動手寫回應，也不會像 Go 那樣忘了 `return`。

### 自己的錯誤型別

前幾集的錯誤都只有一個狀態碼，沒有說明。實際的 API 通常會回一段 JSON 說明哪裡錯。把所有可能的錯誤收進一個 enum（第 9 章），再替它實作 `IntoResponse`，狀態碼與格式就集中在同一個地方。`Display` 用第 17 章第 13 集的 `thiserror`，所以 `Cargo.toml` 多一行：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.9"
serde = { version = "1.0.229", features = ["derive"] }
thiserror = "2.0.21"
tokio = { version = "1.54.1", features = ["full"] }

[dev-dependencies]
http-body-util = "0.1.5"
tower = { version = "0.5.3", features = ["util"] }
```

`src/lib.rs`：

```rust,ignore
use axum::Json;
use axum::Router;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const DRINKS: [&str; 3] = ["tea", "coffee", "milk-tea"];

#[derive(Deserialize, Serialize)]
struct Order {
    drink: String,
    cups: u8,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("沒有這個飲料：{0}")]
    UnknownDrink(String),
    #[error("杯數要大於 0")]
    NoCups,
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::UnknownDrink(_) => StatusCode::NOT_FOUND,
            AppError::NoCups => StatusCode::UNPROCESSABLE_ENTITY,
        };
        let body = ErrorBody {
            error: self.to_string(),
        };
        (status, Json(body)).into_response()
    }
}

fn check(order: &Order) -> Result<(), AppError> {
    if order.cups == 0 {
        return Err(AppError::NoCups);
    }
    if !DRINKS.contains(&order.drink.as_str()) {
        return Err(AppError::UnknownDrink(order.drink.clone()));
    }
    Ok(())
}

async fn create_order(Json(order): Json<Order>) -> Result<(StatusCode, Json<Order>), AppError> {
    check(&order)?;
    Ok((StatusCode::CREATED, Json(order)))
}

pub fn app() -> Router {
    Router::new().route("/orders", post(create_order))
}

#[cfg(test)]
mod tests;
```

- `AppError` 每個 variant 是一種錯誤，`#[error("…")]` 決定 `to_string()` 的文字。
- `into_response` 先決定狀態碼：`match` 每個 variant 對應一個，新增 variant 時，漏掉的分支編譯器會指出來（第 6 章第 4 集）。再把說明包成 `{"error": "…"}`，用 `(status, Json(body))` 這一對變成回應。
- `check` 檢查訂單，有問題就回 `Err(AppError::…)`；`create_order` 回傳 `Result<(StatusCode, Json<Order>), AppError>`，用 `check(&order)?` 把錯誤一路往外丟（第 9 章第 5 集），到了 `into_response` 才變成回應。
- `src/main.rs` 照舊。

### 跑起來

```text
$ curl -s -w '\n[%{http_code}]\n' -X POST localhost:3000/orders -H 'content-type: application/json' -d '{"drink":"juice","cups":1}'
{"error":"沒有這個飲料：juice"}
[404]
$ curl -s -w '\n[%{http_code}]\n' -X POST localhost:3000/orders -H 'content-type: application/json' -d '{"drink":"tea","cups":0}'
{"error":"杯數要大於 0"}
[422]
```

### 測試

`send` 照舊：

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
async fn unknown_drink_is_404_with_json() {
    let order = r#"{"drink":"juice","cups":1}"#;
    let reply = send(&app(), "POST", "/orders", order).await;
    let error = r#"{"error":"沒有這個飲料：juice"}"#;
    assert_eq!(reply, (404, String::from(error)));
}

#[tokio::test]
async fn zero_cups_is_422_with_json() {
    let order = r#"{"drink":"tea","cups":0}"#;
    let reply = send(&app(), "POST", "/orders", order).await;
    assert_eq!(reply, (422, String::from(r#"{"error":"杯數要大於 0"}"#)));
}

#[tokio::test]
async fn valid_order_is_201() {
    let order = r#"{"drink":"coffee","cups":2}"#;
    assert_eq!(send(&app(), "POST", "/orders", order).await.0, 201);
}
```

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-70f388c1c66f54aa)

running 3 tests
test tests::zero_cups_is_422_with_json ... ok
test tests::unknown_drink_is_404_with_json ... ok
test tests::valid_order_is_201 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

handler 裡 panic 不會變成 500：處理那個請求的任務結束，連線被直接切斷（`curl` 會說 `Empty reply from server`，結束碼 52），伺服器照常服務其他請求。該回的錯誤，要自己回傳 `Err`。

### 跟 Go 比一比

Go 的 `http.Error` 寫完回應，函式並不會結束：

```go
if drink != "tea" {
	http.Error(w, "沒有這個飲料："+drink, http.StatusNotFound)
	// 忘了 return
}
fmt.Fprintln(w, "紅茶 30 元")
```

- 忘了 `return`，程式繼續往下跑：狀態碼是 404，body 卻是「沒有這個飲料：juice」加上「紅茶 30 元」兩段（實測）。
- axum 的 handler 是回傳一個值，每條路徑都得交出回應，`Err(...)` 一回傳函式就結束；狀態碼對應也只寫在 `into_response` 一處，不會每個 handler 各寫各的。

## 重點整理

- handler 可以回傳任何實作 `IntoResponse` 的型別：字串、`StatusCode`、`Json<T>`、`(StatusCode, 內容)`、`Result<T, E>`（兩邊都要實作）。
- 自己的錯誤型別（enum）實作 `IntoResponse`：決定每個 variant 的狀態碼，並把說明包成 JSON。
- handler 回傳 `Result<T, AppError>`，就能用 `?` 與提早 `return Err(...)`，錯誤自動變成回應；handler 裡 panic 不會變成 500，連線會被切斷。
- 跟 Go 比：Go 寫了 `http.Error` 還要記得 `return`；axum 回傳 `Err` 函式就結束。
