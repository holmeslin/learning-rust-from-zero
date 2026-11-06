# 在 axum 裡使用

## 本集目標

把第 19 章放在記憶體裡的訂單換成 SQLite：用 `Arc<Mutex<Connection>>` 當 state，在 handler 裡讀寫資料庫，伺服器重開之後訂單還在，並用 `oneshot` 測試。

## 正文

### 把 `Vec` 換成 `Connection`

第 19 章第 5 集的 state 是 `Arc<Mutex<Vec<Order>>>`，伺服器一重開，訂單就清空了。換成資料庫，只要把 `Vec` 換成 `Connection`：`Arc<Mutex<Connection>>`。

為什麼要 `Mutex`？`Connection` 可以搬到另一條執行緒（`Send`），但不能讓好幾條執行緒同時用（不是 `Sync`，第 14 章第 11 集）：`Arc<Connection>` 交給 `thread::spawn` 是 E0277，說 `RefCell<…InnerConnection>` cannot be shared between threads safely。`Mutex<T>` 只要 `T` 是 `Send` 就是 `Sync`，所以 `Arc<Mutex<Connection>>` 能共用：一次只有一個 handler 用那條連線。`Cargo.toml` 比第 19 章第 5 集多一行 rusqlite：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.9"
rusqlite = { version = "0.41.1", features = ["bundled"] }
serde = { version = "1.0.229", features = ["derive"] }
tokio = { version = "1.54.1", features = ["full"] }

[dev-dependencies]
http-body-util = "0.1.5"
tower = { version = "0.5.3", features = ["util"] }
```

`src/lib.rs`：

```rust,ignore
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Serialize)]
struct Order {
    drink: String,
    cups: u8,
    note: Option<String>,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
}

fn internal(e: rusqlite::Error) -> StatusCode {
    eprintln!("資料庫出錯：{e}");
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn create_order(
    State(state): State<AppState>,
    Json(order): Json<Order>,
) -> Result<StatusCode, StatusCode> {
    let db = state.db.lock().expect("鎖中毒");
    db.execute(
        "INSERT INTO orders (drink, cups, note) VALUES (?1, ?2, ?3)",
        params![order.drink, order.cups, order.note],
    )
    .map_err(internal)?;
    Ok(StatusCode::CREATED)
}

async fn list_orders(State(state): State<AppState>) -> Result<Json<Vec<Order>>, StatusCode> {
    let db = state.db.lock().expect("鎖中毒");
    let mut stmt = db
        .prepare("SELECT drink, cups, note FROM orders ORDER BY id")
        .map_err(internal)?;
    let rows = stmt
        .query_map((), |row| {
            Ok(Order {
                drink: row.get(0)?,
                cups: row.get(1)?,
                note: row.get(2)?,
            })
        })
        .map_err(internal)?;
    let orders = rows.collect::<rusqlite::Result<Vec<Order>>>();
    Ok(Json(orders.map_err(internal)?))
}

pub fn app(conn: Connection) -> rusqlite::Result<Router> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL, note TEXT)",
    )?;
    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
    };
    Ok(Router::new()
        .route("/orders", get(list_orders).post(create_order))
        .with_state(state))
}

#[cfg(test)]
mod tests;
```

- `app` 收一條 `Connection`：先用 `CREATE TABLE IF NOT EXISTS` 確保資料表存在（已經有就不動它），再把連線放進 state；建表可能失敗，所以回傳 `rusqlite::Result<Router>`。連線由呼叫端開，`main` 開檔案、測試開別的。
- `create_order` 用第 5 集的參數綁定，`note` 沒填就是 `None`，存成 `NULL`（第 7 集）；`list_orders` 用第 4 集的 `prepare`、`query_map` 與 `collect`，`NULL` 回傳時變成 JSON 的 `null`。
- 鎖只在需要的幾行拿著，中間沒有 `.await`，用標準函式庫的 `Mutex` 就夠（第 19 章第 5 集）。rusqlite 是同步的，查詢進行時會佔住 runtime 的工作執行緒（第 15 章第 12 集）；這裡的查詢很短，直接呼叫沒問題，很慢的查詢才要 `spawn_blocking`。
- 資料庫出錯，詳細訊息只印在伺服器的 stderr，客人只看到 500。這裡用最短的寫法；第 19 章第 6 集的錯誤型別也能搬過來用。

`src/main.rs`：

```rust,ignore
use rusqlite::Connection;

#[tokio::main]
async fn main() {
    let conn = Connection::open("teashop.db").expect("無法開啟 teashop.db");
    let app = teashop::app(conn).expect("無法建立資料表");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("無法綁定 127.0.0.1:3000");
    println!("伺服器啟動：http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("伺服器出錯");
}
```

`cargo build` 的輸出（省略相依套件的編譯進度）：

```text
   Compiling teashop v0.1.0 (/home/you/teashop)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.46s
```

### 跑起來

`cargo run` 啟動之後，`main` 開的 `teashop.db` 會出現在專案資料夾。用第 19 章第 5 集的 `curl` 指令 `POST` 幾張訂單，按 Ctrl-C 停掉伺服器，再 `cargo run` 一次，`GET /orders` 回來的訂單還在：資料存在檔案裡，不在伺服器的記憶體裡。`teashop.db` 是執行後留下的，不想要時自己刪掉。下面的測試把「重開」這件事自動化。

### 測試

一般的測試用 `Connection::open_in_memory()` 就夠，不碰檔案。這裡要測「重開」，所以用暫存的檔案資料庫：先 drop 第一個 `Router`（連線跟著關閉），再用同一個檔案建第二個。`send` 照舊：

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
async fn orders_survive_a_restart() {
    let path = std::env::temp_dir().join("teashop-sqlite-restart.db");
    let _ = std::fs::remove_file(&path);
    let first = app(Connection::open(&path).expect("開不了資料庫")).expect("建立資料表失敗");
    let tea = send(&first, "POST", "/orders", r#"{"drink":"tea","cups":2}"#).await;
    assert_eq!(tea.0, 201);
    let coffee = r#"{"drink":"coffee","cups":1,"note":"少冰"}"#;
    send(&first, "POST", "/orders", coffee).await;
    drop(first);
    let second = app(Connection::open(&path).expect("開不了資料庫")).expect("建立資料表失敗");
    let list = send(&second, "GET", "/orders", "").await;
    let expected =
        r#"[{"drink":"tea","cups":2,"note":null},{"drink":"coffee","cups":1,"note":"少冰"}]"#;
    assert_eq!(list, (200, String::from(expected)));
    drop(second);
    std::fs::remove_file(&path).expect("刪不掉資料庫檔案");
}
```

`cargo test --lib` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-8bb53c1c5d9f3fba)

running 1 test
test tests::orders_survive_a_restart ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### 跟 Go 比一比

Go 的 `*sql.DB` 是連線池，每個 handler 直接共用它，不必自己上鎖。

- rusqlite 的 `Connection` 只是單一條連線，要自己用 `Mutex` 讓 handler 輪流用，同一時間只有一個 handler 在查資料庫。流量大的服務會改用連線池（`r2d2` 之類，這本書不展開）。
- Go 的 `*sql.DB` 能共用，是函式庫的保證，編譯器不檢查；Rust 的 `Connection` 不是 `Sync`，沒用 `Mutex` 就共用，編譯器直接擋下。

## 重點整理

- state 裡放 `Arc<Mutex<Connection>>`：`Connection` 是 `Send` 但不是 `Sync`，`Arc<Connection>` 不能跨執行緒共用（E0277），包上 `Mutex` 才行。
- 連線由呼叫端開、交給 `app(conn)`：`main` 用 `Connection::open("teashop.db")`，測試用 `open_in_memory()`；`CREATE TABLE IF NOT EXISTS` 讓重開時不會重複建表。
- handler 裡鎖住連線、用前幾集的 `execute`、`query_map`；鎖不跨 `.await`，慢查詢才需要 `spawn_blocking`。
- 資料存在檔案裡，伺服器重開之後還在；測試用記憶體資料庫，要測「重開」就用暫存檔。
- 跟 Go 比：`*sql.DB` 是連線池，可直接共用；`Connection` 是單一連線，要 `Mutex`，沒包就共用編譯不過。
