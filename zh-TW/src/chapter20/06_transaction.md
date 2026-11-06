# 交易

## 本集目標

用 `conn.transaction()` 把好幾條 SQL 包成一個交易：全部成功才 `commit`，中途出錯就全部復原；知道沒呼叫 `commit` 就離開是什麼結果，以及為什麼 `transaction` 要 `&mut Connection`。

## 正文

### 沒有交易：寫到一半

一份訂單有好幾項飲料，要一起寫進資料庫。如果寫到第二項出錯，前面那一項已經寫進去了：

```rust,noplayground
use rusqlite::{Connection, params};

fn add_all(conn: &Connection, items: &[(&str, i64)]) -> rusqlite::Result<()> {
    for (drink, cups) in items {
        conn.execute(
            "INSERT INTO orders (drink, cups) VALUES (?1, ?2)",
            params![drink, cups],
        )?;
    }
    Ok(())
}

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL CHECK (cups > 0))",
    )?;
    let result = add_all(&conn, &[("紅茶", 2), ("咖啡", 0), ("奶茶", 1)]);
    if let Err(e) = result {
        println!("出錯了：{e}");
    }
    let n: i64 = conn.query_row("SELECT count(*) FROM orders", (), |row| row.get(0))?;
    println!("資料表裡有 {n} 筆");
    Ok(())
}
```

執行結果：

```text
出錯了：CHECK constraint failed: cups > 0
資料表裡有 1 筆
```

資料表的 `CHECK (cups > 0)` 不接受 0 杯，第二項被擋下來，`?` 讓函式提早回傳；但第一項的紅茶已經寫進去了，一份訂單只存了一半。

### 用交易包起來

`conn.transaction()` 開始一個交易，回傳 `Transaction`。在它上面呼叫 `execute`、`query_row`（它能當 `Connection` 用），最後呼叫 `tx.commit()` 提交（commit），這些改動才算數：

```rust,noplayground
use rusqlite::{Connection, params};

fn add_all(conn: &mut Connection, items: &[(&str, i64)]) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;
    for (drink, cups) in items {
        tx.execute(
            "INSERT INTO orders (drink, cups) VALUES (?1, ?2)",
            params![drink, cups],
        )?;
    }
    tx.commit()
}

fn count(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row("SELECT count(*) FROM orders", (), |row| row.get(0))
}

fn main() -> rusqlite::Result<()> {
    let mut conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL CHECK (cups > 0))",
    )?;
    let bad = add_all(&mut conn, &[("紅茶", 2), ("咖啡", 0), ("奶茶", 1)]);
    println!(
        "出錯了嗎：{}，資料表裡有 {} 筆",
        bad.is_err(),
        count(&conn)?
    );
    add_all(&mut conn, &[("紅茶", 2), ("奶茶", 1)])?;
    println!("資料表裡有 {} 筆", count(&conn)?);
    Ok(())
}
```

執行結果：

```text
出錯了嗎：true，資料表裡有 0 筆
資料表裡有 2 筆
```

- 同樣的壞資料，這次一筆都沒留下：出錯時 `?` 提早回傳，`tx` 在函式結束時被 drop，沒有 `commit` 的交易就被復原（rollback）。第二次兩項都成功，`tx.commit()` 之後才都算數。
- `tx.commit()` 回傳 `rusqlite::Result<()>`，所以函式最後一行直接寫 `tx.commit()`，提交失敗也會傳給呼叫端。
- `Transaction` 能直接呼叫 `execute`、`query_row`、`prepare`，因為它實作了 `Deref`（第 13 章第 3 集）到 `Connection`。
- 要主動放棄就呼叫 `tx.rollback()`；不呼叫也一樣，離開作用域就復原。

### 忘了 `commit`

沒有 `commit` 就離開，不是錯誤，也沒有警告，改動悄悄消失：

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let mut conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL)")?;
    {
        let tx = conn.transaction()?;
        tx.execute("INSERT INTO orders (drink) VALUES ('紅茶')", ())?;
        let inside: i64 = tx.query_row("SELECT count(*) FROM orders", (), |row| row.get(0))?;
        println!("交易裡有 {inside} 筆");
    }
    let after: i64 = conn.query_row("SELECT count(*) FROM orders", (), |row| row.get(0))?;
    println!("離開之後有 {after} 筆");
    Ok(())
}
```

執行結果：

```text
交易裡有 1 筆
離開之後有 0 筆
```

交易裡看得到自己寫的資料，沒有 `commit` 就離開，資料就沒了。所以寫交易的函式，最後一定要有 `commit`，也一定要有測試確認資料真的留下來。

### 為什麼要 `&mut Connection`

`conn.transaction()` 需要 `&mut self`，所以 `conn` 要寫成 `let mut conn`，不然是 E0596（`` cannot borrow `conn` as mutable `` ）。而且交易還開著的時候，`conn` 被它借走了，不能再用：

```rust,compile_fail=E0502,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let mut conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL)")?;
    let tx = conn.transaction()?;
    tx.execute("INSERT INTO orders (drink) VALUES ('紅茶')", ())?;
    conn.execute("INSERT INTO orders (drink) VALUES ('咖啡')", ())?;
    tx.commit()?;
    Ok(())
}
```

編譯錯誤：

```text
error[E0502]: cannot borrow `conn` as immutable because it is also borrowed as mutable
 --> src/main.rs:8:5
  |
6 |     let tx = conn.transaction()?;
  |              ---- mutable borrow occurs here
7 |     tx.execute("INSERT INTO orders (drink) VALUES ('紅茶')", ())?;
8 |     conn.execute("INSERT INTO orders (drink) VALUES ('咖啡')", ())?;
  |     ^^^^ immutable borrow occurs here
9 |     tx.commit()?;
  |     -- mutable borrow later used here
```

手滑把 `tx.execute` 寫成 `conn.execute`，那條 SQL 就跑在交易之外：編譯器直接擋下來。

### 跟 Go 比一比

```go
tx, err := db.Begin()
if err != nil {
	return err
}
defer tx.Rollback()
if _, err := tx.Exec("INSERT INTO orders (drink, cups) VALUES (?, ?)", "紅茶", 2); err != nil {
	return err
}
if _, err := db.Exec("INSERT INTO orders (drink, cups) VALUES (?, ?)", "咖啡", 1); err != nil {
	return err
}
return tx.Commit()
```

- 第二個 `Exec` 寫成 `db` 而不是 `tx`，Go 編得過：它從連線池拿另一條連線，在交易之外執行。用檔案資料庫試，會得到 `database is locked`。Rust 的 `tx` 借走了 `conn`，這種寫法編譯不過。
- Go 要自己寫 `defer tx.Rollback()`，不然出錯提早 `return`，交易沒結束，連線就一直被佔住。Rust 的 `Transaction` 被 drop 時自動復原，不必寫；相對的，忘了 `commit` 是悄悄復原，不是佔住連線。

## 重點整理

- `let tx = conn.transaction()?;`（`conn` 要 `let mut`，函式參數寫 `&mut Connection`），在 `tx` 上跑 SQL，最後 `tx.commit()?` 才算數；`commit` 回傳 `Result`。
- 沒有 `commit` 就離開（含 `?` 提早回傳），`Transaction` 被 drop 時自動復原；也可以主動 `tx.rollback()`。
- 忘了 `commit` 沒有錯誤也沒有警告，改動悄悄消失，要靠測試確認。
- `transaction` 借走 `&mut Connection`，交易開著時不能再用 `conn`（E0502），避免把 SQL 誤跑在交易之外。
- 跟 Go 比：Go 的 `db.Exec` 與 `tx.Exec` 混用編得過，要自己 `defer tx.Rollback()`；Rust 由借用規則擋下，drop 時自動復原。
