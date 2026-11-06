# extractor：`Path` 與 `Query`

## 本集目標

用 extractor 從請求的路徑與查詢字串取出資料：`Path` 取路徑裡的片段，`Query` 取 `?` 後面的參數，並知道格式不對時 axum 會怎麼回應。

## 正文

### handler 的參數就是 extractor

handler 要用到請求的內容，不用自己去翻請求物件：把需要的東西寫成參數，axum 呼叫 handler 之前會先從請求裡取出來，取不出來就直接回錯誤，handler 根本不會被呼叫。這種參數叫 extractor。這一集用兩個：`Path` 和 `Query`，先加上 `serde`（第 17 章第 9 集），查詢字串要靠它解析成 struct。`src/main.rs` 照舊：

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

### `Path`：路徑裡的片段

路徑裡用 `{名字}` 標出會變的片段，handler 用 `Path<型別>` 接。一個片段就是單一型別，好幾個片段就用 tuple，順序跟路徑裡出現的順序一樣。再加上 `Query<Options>`：`Options` 是 `derive(Deserialize)` 的 struct，欄位名稱就是參數名稱，`Option` 欄位代表可有可無：

`src/lib.rs`：

```rust,ignore
use axum::Router;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::get;
use serde::Deserialize;

fn price_of(id: &str) -> Option<(&'static str, u32)> {
    match id {
        "tea" => Some(("紅茶", 30)),
        "coffee" => Some(("咖啡", 45)),
        "milk-tea" => Some(("奶茶", 38)),
        _ => None,
    }
}

#[derive(Deserialize)]
struct Options {
    sugar: Option<u32>,
}

async fn drink(Path(id): Path<String>, Query(options): Query<Options>) -> (StatusCode, String) {
    let Some((name, price)) = price_of(&id) else {
        return (StatusCode::NOT_FOUND, format!("沒有這個飲料：{id}"));
    };
    let sugar = options.sugar.unwrap_or(100);
    (StatusCode::OK, format!("{name} {price} 元（糖 {sugar}%）"))
}

async fn total(Path((id, cups)): Path<(String, u8)>) -> (StatusCode, String) {
    match price_of(&id) {
        Some((name, price)) => {
            let sum = price * u32::from(cups);
            (StatusCode::OK, format!("{name} × {cups} = {sum} 元"))
        }
        None => (StatusCode::NOT_FOUND, format!("沒有這個飲料：{id}")),
    }
}

pub fn app() -> Router {
    Router::new()
        .route("/drinks/{id}", get(drink))
        .route("/total/{id}/{cups}", get(total))
}

#[cfg(test)]
mod tests;
```

- `/drinks/{id}` 的 `{id}` 由 `Path(id): Path<String>` 接：`Path(id)` 直接在參數裡把 `Path` 這個 tuple struct 拆開（第 5 章第 5 集、第 6 章第 12 集），`id` 就是 `String`。
- `/total/{id}/{cups}` 有兩個片段，所以是 `Path<(String, u8)>`：`id` 是 `String`，`cups` 是 `u8`。`cups` 寫成 `u8`，路徑裡不是 0 到 255 的整數就進不了 handler。
- `Query(options)` 把 `?sugar=50` 解析成 `Options`：`sugar` 是 `Option<u32>`，沒給就是 `None`，給了就要能轉成 `u32`。
- 路徑的片段名稱用大括號 `{id}`。舊文章常寫 `:id`，axum 0.8 遇到會直接 panic，要求改成 `{id}`。

### 跑起來

```text
$ curl -s localhost:3000/drinks/tea
紅茶 30 元（糖 100%）
$ curl -s 'localhost:3000/drinks/coffee?sugar=50'
咖啡 45 元（糖 50%）
$ curl -s localhost:3000/total/milk-tea/3
奶茶 × 3 = 114 元
$ curl -s localhost:3000/drinks/juice
沒有這個飲料：juice
```

### 格式不對的時候

型別就是規則。參數轉不過去，axum 不呼叫 handler，直接回 400（Bad Request）加一行說明：

```text
$ curl -i -s localhost:3000/total/tea/abc
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 69
date: Fri, 06 Nov 2026 04:46:10 GMT

Invalid URL: Cannot parse value at index 1 with value `abc` to a `u8`
$ curl -s 'localhost:3000/drinks/tea?sugar=abc'
Failed to deserialize query string: sugar: invalid digit found in string
```

說明是英文的，由 axum 產生；查詢字串不對，狀態碼也是 400。第 6 集會教怎麼自己決定錯誤長什麼樣。

### 測試

`send` 照舊，新的是後面三個測試案例：

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
async fn drink_reads_path_and_query() {
    let app = app();
    let plain = send(&app, "GET", "/drinks/tea", "").await;
    assert_eq!(plain, (200, String::from("紅茶 30 元（糖 100%）")));
    let custom = send(&app, "GET", "/drinks/coffee?sugar=50", "").await;
    assert_eq!(custom, (200, String::from("咖啡 45 元（糖 50%）")));
    let unknown = send(&app, "GET", "/drinks/juice", "").await;
    assert_eq!(unknown, (404, String::from("沒有這個飲料：juice")));
}

#[tokio::test]
async fn total_reads_two_segments() {
    let reply = send(&app(), "GET", "/total/milk-tea/3", "").await;
    assert_eq!(reply, (200, String::from("奶茶 × 3 = 114 元")));
}

#[tokio::test]
async fn bad_input_never_reaches_the_handler() {
    let app = app();
    assert_eq!(send(&app, "GET", "/total/tea/abc", "").await.0, 400);
    assert_eq!(send(&app, "GET", "/total/tea/256", "").await.0, 400);
    assert_eq!(send(&app, "GET", "/drinks/tea?sugar=abc", "").await.0, 400);
}
```

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-14e6bdfe9c81aa39)

running 3 tests
test tests::total_reads_two_segments ... ok
test tests::drink_reads_path_and_query ... ok
test tests::bad_input_never_reaches_the_handler ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### 跟 Go 比一比

Go 的 `net/http` 取路徑片段與查詢參數，拿到的都是字串：

```go
id := r.PathValue("id")
sugar, err := strconv.Atoi(r.URL.Query().Get("sugar"))
```

- `Query().Get` 沒有這個參數時回傳空字串，跟「給了一個空字串」分不出來；轉成數字要自己 `strconv.Atoi` 並處理錯誤，忘了處理，錯誤的輸入就悄悄變成 0。
- axum 把型別寫在參數上：能不能轉、少了參數怎麼辦（`Option`），編譯的時候就寫清楚，格式不對的請求在進入 handler 之前就被擋下。

## 重點整理

- extractor 是 handler 的參數，axum 在呼叫 handler 之前從請求取出；取不出來就直接回 400，handler 不會被呼叫。
- `Path<型別>` 取路徑片段：路徑寫 `{名字}`，一個片段用單一型別，多個片段用 tuple，順序跟路徑相同。
- `Query<結構>` 取查詢字串：struct 要 `derive(Deserialize)`，`Option` 欄位可有可無。
- 跟 Go 比：Go 拿到的都是字串，轉型與錯誤要自己處理；axum 用型別決定怎麼解析，不合格的請求進不了 handler。
