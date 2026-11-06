# `execute`

## 本集目標

用 `execute` 建資料表、新增、更新、刪除資料列，看懂它回傳的「影響了幾列」；知道 `execute` 不能拿來跑會回傳資料列的 SQL，以及一次跑好幾條 SQL 要用 `execute_batch`。

## 正文

### 建表、新增、更新、刪除

這一章用一張訂單表 `orders`：`id` 是 `INTEGER PRIMARY KEY`（SQLite 會自動編號），`drink` 是飲料名稱，`cups` 是杯數。每一條「改動資料」的 SQL，都用 `conn.execute(SQL, 參數)`：

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    let created = conn.execute(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL)",
        (),
    )?;
    println!("建表：{created}");
    let inserted = conn.execute("INSERT INTO orders (drink, cups) VALUES ('紅茶', 2)", ())?;
    println!("新增：{inserted}，id 是 {}", conn.last_insert_rowid());
    conn.execute("INSERT INTO orders (drink, cups) VALUES ('咖啡', 1)", ())?;
    println!("第二筆的 id 是 {}", conn.last_insert_rowid());
    let updated = conn.execute("UPDATE orders SET cups = cups + 1", ())?;
    println!("更新：{updated}");
    let deleted = conn.execute("DELETE FROM orders WHERE cups > 2", ())?;
    println!("刪除：{deleted}");
    let none = conn.execute("DELETE FROM orders WHERE cups > 100", ())?;
    println!("沒有符合的：{none}");
    Ok(())
}
```

執行結果：

```text
建表：0
新增：1，id 是 1
第二筆的 id 是 2
更新：2
刪除：1
沒有符合的：0
```

- `execute` 回傳 `rusqlite::Result<usize>`：這條 SQL 改動了幾列。建表沒有改動任何資料列，所以是 0；`DELETE` 沒有符合的資料列，也是 0，不算錯誤。
- 第二個引數是 SQL 的參數。這一集用不到，先照抄 `()`（空的）；第 5 集才解釋它。
- `conn.last_insert_rowid()` 是最近一次 `INSERT` 的 `id`，型別是 `i64`。
- `execute` 只借用 `&self`，所以 `conn` 不必寫 `let mut`。第 6 集的 `transaction` 就不同了。
- 回傳的 `Result` 一定要處理。`conn.execute(...)` 單獨一行沒接 `?`，是 rustc 的 `unused_must_use` 警告（第 9 章第 2 集）。

SQLite 的欄位型別只是「傾向」：`cups INTEGER` 這一欄塞進 `'很多'`，SQLite 照收，存成文字。資料庫不替你擋，所以型別對不對，要等到 Rust 這邊讀出來時才會發現（第 3 集）。

### `execute` 不收 SELECT

`execute` 只給「不回傳資料列」的 SQL 用。拿它跑 `SELECT`，只要查得到資料列，就是 `Err(ExecuteReturnedResults)`：

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL)",
        (),
    )?;
    conn.execute("INSERT INTO orders (drink) VALUES ('紅茶')", ())?;
    match conn.execute("SELECT * FROM orders", ()) {
        Err(e @ rusqlite::Error::ExecuteReturnedResults) => println!("{e}"),
        other => println!("{other:?}"),
    }
    Ok(())
}
```

執行結果：

```text
Execute returned results - did you mean to call query?
```

資料表是空的，`SELECT` 沒有任何資料列可以回傳，這時 `execute` 會回 `Ok(0)`，不會報錯。別靠這個行為，查資料請用下一集的 `query_row`。

### 一次跑好幾條：`execute_batch`

`execute` 只收一條 SQL，兩條以上是 `Err(MultipleStatement)`。一次跑好幾條（最常見是建表）用 `execute_batch`，它回傳 `rusqlite::Result<()>`，沒有「影響幾列」：

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    let two = "CREATE TABLE a (id INTEGER); CREATE TABLE b (id INTEGER)";
    if let Err(e) = conn.execute(two, ()) {
        println!("execute：{e}");
    }
    conn.execute_batch(two)?;
    println!("execute_batch：成功");
    Ok(())
}
```

執行結果：

```text
execute：Multiple statements provided
execute_batch：成功
```

下一集起，每個範例都用 `execute_batch` 建好資料表、放進幾筆資料，再往下做事。

### 跟 Go 比一比

```go
res, err := db.Exec("INSERT INTO orders (drink, cups) VALUES ('紅茶', 2)")
if err != nil {
	return err
}
n, err := res.RowsAffected()
id, err := res.LastInsertId()
```

- Go 的 `Exec` 回傳 `sql.Result`，影響幾列、最後的 id 要再呼叫 `RowsAffected()`、`LastInsertId()`，兩個都帶 `err`。rusqlite 的 `execute` 直接給列數，id 用 `last_insert_rowid()` 取。
- Go 拿 `Exec` 跑 `SELECT` 不會報錯，資料列就這樣被丟掉；rusqlite 會回 `ExecuteReturnedResults`，提醒你該用查詢的函式。

## 重點整理

- `conn.execute(SQL, ())` 跑一條不回傳資料列的 SQL（`CREATE`、`INSERT`、`UPDATE`、`DELETE`），回傳 `Result<usize>`，內容是影響的列數；`()` 是沒有參數，第 5 集解釋。
- `conn.last_insert_rowid()` 取得最近一次 `INSERT` 的 id；`execute` 只借用 `&self`。
- `execute` 跑 `SELECT` 查得到資料列就是 `Err(ExecuteReturnedResults)`，兩條以上的 SQL 是 `Err(MultipleStatement)`。
- 一次跑好幾條 SQL 用 `execute_batch`，回傳 `Result<()>`。
- 跟 Go 比：Go 的 `Exec` 要另外呼叫 `RowsAffected()`、`LastInsertId()`，而且拿它跑 `SELECT` 也不報錯；rusqlite 會報錯。
