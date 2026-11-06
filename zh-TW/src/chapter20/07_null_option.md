# `NULL` 與 `Option`

## 本集目標

用 `Option<T>` 對應 SQL 的 `NULL`：寫入時用 `None` 存成 `NULL`，讀出時用 `Option<T>` 接；也用 `.optional()` 把「查不到資料列」變成 `Option`，並避開 `= NULL` 的陷阱。

## 正文

### 資料欄可以是 `NULL`

資料欄沒有寫 `NOT NULL`，就可以是 `NULL`，意思是「沒有值」。給訂單加一欄備註 `note TEXT`，不是每張訂單都有。Rust 沒有 `NULL`，這種「有或沒有」就是 `Option<T>`（第 6 章第 6 集）：寫入時 `None` 存成 `NULL`，`Some(值)` 存成那個值；讀出時，`NULL` 讀成 `None`。

```rust,noplayground
use rusqlite::{Connection, params};

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, note TEXT)",
    )?;
    let note: Option<&str> = Some("少冰");
    conn.execute(
        "INSERT INTO orders (drink, note) VALUES (?1, ?2)",
        params!["咖啡", note],
    )?;
    conn.execute(
        "INSERT INTO orders (drink, note) VALUES (?1, ?2)",
        params!["奶茶", None::<String>],
    )?;
    let mut stmt = conn.prepare("SELECT drink, note FROM orders ORDER BY id")?;
    let rows = stmt.query_map((), |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?))
    })?;
    for row in rows {
        let (drink, note) = row?;
        match note {
            Some(text) => println!("{drink}：{text}"),
            None => println!("{drink}：沒有備註"),
        }
    }
    Ok(())
}
```

執行結果：

```text
咖啡：少冰
奶茶：沒有備註
```

- `params!` 裡的 `Option<&str>`、`Option<String>` 都能綁：`None` 是 `NULL`。直接寫 `None` 編譯器不知道是什麼型別的 `Option`，要標成 `None::<String>`（不標是 E0282）。
- 讀的時候，把型別寫成 `Option<String>`，`NULL` 就是 `None`，讀出來照第 6 章用 `match`、`if let` 處理。
- 空字串 `''` 不是 `NULL`：讀出來是 `Some("")`。

### 該用 `Option` 的地方沒用

`NULL` 讀成 `String`、`i64` 這種不是 `Option` 的型別，不會自己變成空字串或 0，而是 `Err`：

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, note TEXT);
         INSERT INTO orders (drink) VALUES ('紅茶');",
    )?;
    let note: rusqlite::Result<String> =
        conn.query_row("SELECT note FROM orders", (), |row| row.get(0));
    if let Err(e) = note {
        println!("{e}");
    }
    let max: rusqlite::Result<i64> =
        conn.query_row("SELECT MAX(id) FROM orders WHERE id > 100", (), |row| {
            row.get(0)
        });
    if let Err(e) = max {
        println!("{e}");
    }
    Ok(())
}
```

執行結果：

```text
Invalid column type Null at index: 0, name: note
Invalid column type Null at index: 0, name: MAX(id)
```

第二個才是容易踩到的：`MAX`、`SUM` 這類聚合函式，遇到沒有任何資料列可算，結果是 `NULL`，不是 0。資料表有資料時一切正常，一旦是空的，讀成 `i64` 就變成 `Err`。有可能沒有值的資料欄、聚合結果，一律讀成 `Option<T>`。（`count(*)` 例外，沒有資料列時是 0。）

### 查不到資料列：`.optional()`

上一集看到，查不到資料列是 `Err(QueryReturnedNoRows)`。但「沒有這一筆」常常不是錯誤，只是一種結果。先 `use rusqlite::OptionalExtension;`，再在 `query_row` 後面接 `.optional()`，`QueryReturnedNoRows` 就變成 `Ok(None)`，其他錯誤照舊是 `Err`：

```rust,noplayground
use rusqlite::{Connection, OptionalExtension};

fn find(conn: &Connection, id: i64) -> rusqlite::Result<Option<String>> {
    conn.query_row("SELECT drink FROM orders WHERE id = ?1", [id], |row| {
        row.get(0)
    })
    .optional()
}

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL);
         INSERT INTO orders (drink) VALUES ('紅茶');",
    )?;
    println!("{:?}", find(&conn, 1)?);
    println!("{:?}", find(&conn, 99)?);
    Ok(())
}
```

執行結果：

```text
Some("紅茶")
None
```

`find` 回傳 `rusqlite::Result<Option<String>>`：外層的 `Result` 是「查詢有沒有出錯」，內層的 `Option` 是「有沒有這一筆」。`OptionalExtension` 是 trait，要先 `use` 進來，方法才找得到（第 12 章第 5 集）。

### `= NULL` 永遠找不到

SQL 的 `NULL` 跟任何值比較（包含另一個 `NULL`）都不成立，所以 `WHERE note = NULL` 一筆也找不到。判斷有沒有值要用 `IS NULL`；參數可能是 `None` 時，用 `IS ?1`：

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, note TEXT);
         INSERT INTO orders (drink) VALUES ('紅茶');
         INSERT INTO orders (drink, note) VALUES ('咖啡', '少冰');",
    )?;
    let none: Option<&str> = None;
    let eq: i64 = conn.query_row(
        "SELECT count(*) FROM orders WHERE note = ?1",
        [none],
        |row| row.get(0),
    )?;
    println!("= ?1 找到 {eq} 筆");
    let is: i64 = conn.query_row(
        "SELECT count(*) FROM orders WHERE note IS ?1",
        [none],
        |row| row.get(0),
    )?;
    println!("IS ?1 找到 {is} 筆");
    Ok(())
}
```

執行結果：

```text
= ?1 找到 0 筆
IS ?1 找到 1 筆
```

`IS` 對有值的情況跟 `=` 一樣（`IS ?1` 配 `Some("少冰")` 找到咖啡那一筆），遇到 `NULL` 也能比。這是 SQL 的規則，跟 Rust 無關，不過 `Option` 讓人很容易把 `None` 直接綁進 `=`，所以特別提醒。

### 跟 Go 比一比

Go 用 `sql.NullString` 這類結構接可能是 `NULL` 的欄位：

```go
var note sql.NullString
err := db.QueryRow("SELECT note FROM orders").Scan(&note)
if note.Valid {
	fmt.Println(note.String)
}
```

- 讀成一般的 `string`，遇到 `NULL` 會得到 `converting NULL to string is unsupported`；`MAX` 讀成 `int` 也一樣。這點跟 rusqlite 的 `Err` 一樣，都是執行時期才發現。
- 差別在接的型別：Go 的 `NullString` 是 `String` 加 `Valid` 的結構，忘了看 `Valid` 照樣能讀到空字串；Rust 的 `Option<String>` 沒有 `match` 或 `unwrap` 就拿不到裡面的值，沒辦法跳過「有沒有值」這一步。

## 重點整理

- `NULL` 對應 `Option<T>`：寫入時 `None` 是 `NULL`（要標型別，例如 `None::<String>`），讀出時用 `Option<T>` 接，`NULL` 是 `None`；空字串不是 `NULL`。
- 把 `NULL` 讀成 `String`、`i64` 是 `Err(InvalidColumnType)`；空資料表上的 `MAX`、`SUM` 結果是 `NULL`，要讀成 `Option<T>`。
- `use rusqlite::OptionalExtension;` 之後，`query_row(...).optional()` 把 `QueryReturnedNoRows` 變成 `Ok(None)`，回傳 `Result<Option<T>>`。
- SQL 裡 `= NULL` 永遠不成立，判斷用 `IS NULL`，參數可能是 `None` 用 `IS ?1`。
- 跟 Go 比：Go 用 `sql.NullString`（`Valid` 加值）；Rust 的 `Option<T>` 沒有 `match` 就拿不到值，不會忘了檢查。
