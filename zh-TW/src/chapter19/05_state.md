# 共享狀態 `State`

## 本集目標

讓 handler 共用同一份資料：把資料放進 `Router` 的 state，handler 用 `State` extractor 取得；知道為什麼 state 通常是 `Arc<Mutex<T>>`，以及忘了 `with_state` 時編譯器怎麼說。

## 正文

### handler 之間沒有共用的東西

每個 handler 都是獨立的 `async fn`，請求來了就跑、跑完就結束，什麼都不會留下。想記住「今天收了哪些訂單」，資料得放在請求之外。axum 的做法是把共用的資料掛在 `Router` 上，叫 state；handler 用 `State` extractor 拿。`Cargo.toml` 跟上一集一樣，`src/main.rs` 照舊：

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

### 用 `Arc<Mutex<…>>` 放訂單

`src/lib.rs`：

```rust,ignore
use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Deserialize, Serialize)]
struct Order {
    drink: String,
    cups: u8,
}

#[derive(Clone)]
struct AppState {
    orders: Arc<Mutex<Vec<Order>>>,
}

async fn create_order(State(state): State<AppState>, Json(order): Json<Order>) -> StatusCode {
    state.orders.lock().expect("鎖中毒").push(order);
    StatusCode::CREATED
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

- `AppState` 是自己定義的 struct，裡面放要共用的東西。`orders` 是 `Arc<Mutex<Vec<Order>>>`（第 14 章第 9 集）：`Arc` 讓每個 handler 都拿得到同一份，`Mutex` 讓一次只有一個 handler 改它。
- `Router` 要能為每個請求取得一份 state，所以 `AppState` 要 `Clone`；`Arc` 的 clone 只是計數加一，不是複製整個 `Vec`。
- `with_state(state)` 把 state 交給 `Router`；handler 寫 `State(state): State<AppState>`，拆開就拿到 `AppState`。`State` 跟 `Json` 一樣是 extractor，`Json` 照樣放最後。
- 鎖只在需要的那一行拿著，沒有 `.await`，用標準函式庫的 `Mutex` 就夠（第 15 章第 11 集）。state 放在記憶體，重啟就清空，要持久保存，第 20 章換成 SQLite。

### 忘了 `with_state`

`Router` 的型別其實是 `Router<S>`，`S` 是它還缺的 state。加上要 `AppState` 的 handler，就變成 `Router<AppState>`；`fn app() -> Router` 寫的是 `Router<()>`，什麼都不缺。少了 `with_state`，型別就對不上（只貼前幾行，後面的 `note` 指向 axum 內部的位置）：

```rust,compile_fail=E0308,noplayground
use axum::Router;
use axum::extract::State;
use axum::routing::get;

#[derive(Clone)]
struct AppState;

async fn home(State(_): State<AppState>) -> &'static str {
    "歡迎光臨"
}

fn app() -> Router {
    Router::new().route("/", get(home))
}

fn main() {
    app();
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
   --> src/main.rs:13:30
    |
 13 |     Router::new().route("/", get(home))
    |                   -----      ^^^^^^^^^ expected `MethodRouter`, found `MethodRouter<AppState>`
    |                   |
    |                   arguments to this method are incorrect
    |
    = note: expected struct `MethodRouter<()>`
               found struct `MethodRouter<AppState>`
```

### 跑起來

```text
$ curl -s -o /dev/null -w '%{http_code}\n' -X POST localhost:3000/orders -H 'content-type: application/json' -d '{"drink":"tea","cups":1}'
201
$ curl -s -o /dev/null -w '%{http_code}\n' -X POST localhost:3000/orders -H 'content-type: application/json' -d '{"drink":"coffee","cups":2}'
201
$ curl -s localhost:3000/orders
[{"drink":"tea","cups":1},{"drink":"coffee","cups":2}]
```

### 測試

`send` 照舊。`app()` 每呼叫一次就建一份新的 state：同一個測試裡連打好幾次，要用同一個 `Router`；另外呼叫的 `app()` 是空的：

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
async fn orders_are_shared_between_requests() {
    let shop = app();
    let tea = send(&shop, "POST", "/orders", r#"{"drink":"tea","cups":1}"#).await;
    assert_eq!(tea.0, 201);
    send(&shop, "POST", "/orders", r#"{"drink":"coffee","cups":2}"#).await;
    let list = send(&shop, "GET", "/orders", "").await;
    let expected = r#"[{"drink":"tea","cups":1},{"drink":"coffee","cups":2}]"#;
    assert_eq!(list, (200, String::from(expected)));
    let fresh = send(&app(), "GET", "/orders", "").await;
    assert_eq!(fresh.1, "[]");
}
```

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-14e6bdfe9c81aa39)

running 1 test
test tests::orders_are_shared_between_requests ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### 跟 Go 比一比

Go 常把共用的資料（一把 `sync.Mutex` 加上 `orders []Order`）放在 struct 裡，handler 是它的方法：

```go
func (s *server) list(w http.ResponseWriter, r *http.Request) {
	s.mu.Lock()
	defer s.mu.Unlock()
	json.NewEncoder(w).Encode(s.orders)
}
```

- Go 的 `orders` 就在 struct 上，忘了 `mu.Lock()` 照樣編譯，到執行時才可能出現 data race，要靠 `go run -race` 才抓得到。Rust 的 `Vec` 鎖在 `Mutex` 裡，不 `lock()` 就碰不到它，忘不了。

## 重點整理

- 共用的資料掛在 `Router` 上：`Router::new()....with_state(state)`；handler 用 `State(state): State<AppState>` 取得。
- state 要 `Clone`，常見的寫法是 `#[derive(Clone)]` 的 struct，裡面的欄位用 `Arc` 包起來；要改的資料再包 `Mutex`：`Arc<Mutex<T>>`。
- 鎖拿得短、不跨 `.await`，用標準函式庫的 `Mutex`；每次 `app()` 都是一份新的 state；忘了 `with_state`，編譯器會說 `Router<AppState>` 與 `Router` 對不上。
- 跟 Go 比：Go 忘了鎖要到執行時才可能出事；Rust 不拿鎖就碰不到資料。
