# `Connection::open` 與 bundled

## 本集目標

在專案裡加上 rusqlite，用 `Connection::open` 開出 SQLite 資料庫；知道 `bundled` 是什麼、開不起來時會怎樣，以及這一章的範例怎麼做到不留下檔案。

## 正文

### rusqlite 與 `bundled`

SQLite 是把整個資料庫放進單一檔案的函式庫，不必另外開資料庫伺服器，程式直接呼叫函式讀寫它。它是用 C 寫的，rusqlite 把那些 C 函式包成 Rust 的型別。`Cargo.toml` 這樣寫：

```toml
[dependencies]
rusqlite = { version = "0.41.1", features = ["bundled"] }
```

沒有 `bundled` 的話，rusqlite 會去連結系統上已經裝好的 SQLite，你的機器沒裝就編不過，版本不同行為也可能不同。`bundled` 是 Cargo 的 feature（第 12 章第 15 集）：它把 SQLite 的 C 程式碼一起帶進來，編譯的時候用 C 編譯器（第 16 章第 11 集的 `build.rs` 做的就是這類事）編進你的程式。什麼都不用先裝、版本固定，代價是第一次編譯比較久，執行檔也大一些。

印出 SQLite 的版本，確認它真的在裡面：

```rust,noplayground
fn main() {
    println!("SQLite 版本：{}", rusqlite::version());
}
```

執行結果：

```text
SQLite 版本：3.54.1
```

版本跟著 rusqlite 的版本走。rusqlite 不在 Playground 的 crate 清單裡，所以這一章的範例都沒有執行按鈕。

### 開一個資料庫檔案

`Connection::open(路徑)` 開啟指定的資料庫，檔案不存在就建立。它回傳 `rusqlite::Result<Connection>`：`rusqlite::Result<T>` 是 `Result<T, rusqlite::Error>` 的型別別名（附錄一 f），所以 `main` 寫成 `fn main() -> rusqlite::Result<()>`，就能用 `?`（第 9 章第 11 集）。

範例把檔案放在系統暫存目錄（第 17 章第 5 集），最後刪掉，執行完不會留下東西：

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let path = std::env::temp_dir().join("teashop-sqlite-open.db");
    println!("開檔前：{}", path.exists());
    let conn = Connection::open(&path)?;
    println!("開檔後：{}", path.exists());
    let size = std::fs::metadata(&path).expect("讀不到檔案資訊").len();
    println!("檔案大小：{size}");
    drop(conn);
    std::fs::remove_file(&path).expect("刪不掉資料庫檔案");
    println!("刪除之後：{}", path.exists());
    Ok(())
}
```

執行結果：

```text
開檔前：false
開檔後：true
檔案大小：0
刪除之後：false
```

- 檔案在 `open` 的當下就建出來了，但是空的：SQLite 要等到第一次寫入才會放東西進去。
- `Connection` 離開作用域就自動關閉（`Drop`，第 13 章第 4 集），不必像 Go 寫 `defer db.Close()`。這裡先 `drop(conn)` 關掉連線再刪檔案（Windows 不讓你刪還開著的檔案）。想知道關閉有沒有出錯，才呼叫 `conn.close()`，它會回傳 `Result`。

### 記憶體資料庫

`Connection::open_in_memory()` 開出放在記憶體裡的資料庫：不碰檔案，連線一關，資料就沒了。每呼叫一次都是全新、空的資料庫，兩條連線互相看不到對方的資料。這一章之後的範例都用它，所以不用清檔案。

### 開不起來的時候

路徑所在的資料夾不存在，`open` 當場回傳 `Err`：

```rust,noplayground
use rusqlite::Connection;

fn main() {
    match Connection::open("no-such-dir/teashop.db") {
        Ok(_) => println!("開起來了"),
        Err(e) => {
            println!("{e}");
            println!("{:?}", e.sqlite_error_code());
        }
    }
}
```

執行結果：

```text
unable to open database file: no-such-dir/teashop.db
Some(CannotOpen)
```

`sqlite_error_code()` 回傳 SQLite 自己的錯誤分類（`Option<ErrorCode>`），這裡是 `CannotOpen`。

### 跟 Go 比一比

Go 的 `sql.Open` 不會真的連線，只檢查參數：

```go
db, err := sql.Open("sqlite3", "no-such-dir/teashop.db")
fmt.Println(err)       // <nil>：只檢查參數
fmt.Println(db.Ping()) // 到這裡才知道打不開
```

- Go 要等到第一次查詢或 `Ping` 才發現路徑有問題；`Connection::open` 當場開檔，錯誤當場回來。
- `*sql.DB` 是連線池，很多個 goroutine 可以共用；`Connection` 只是單一條連線，不能這樣用，第 8 集說明怎麼共用。

## 重點整理

- 加上 `rusqlite = { version = "0.41.1", features = ["bundled"] }`：`bundled` 把 SQLite 的 C 程式碼一起編進來，不必先裝，版本固定，第一次編譯比較久。
- `Connection::open(路徑)` 開檔案資料庫，檔案不存在就建立（當下是空檔案）；`Connection::open_in_memory()` 開記憶體資料庫，每次都是全新的。
- 回傳 `rusqlite::Result<T>`，開不起來是 `Err`；`Connection` 離開作用域自動關閉。
- 這一章的範例沒有 Playground 按鈕；碰檔案的範例放暫存目錄、最後刪掉，其餘用記憶體資料庫。
- 跟 Go 比：`sql.Open` 不連線，要 `Ping` 才知道；`Connection::open` 當場開檔；`Connection` 是單一連線，不是連線池。
