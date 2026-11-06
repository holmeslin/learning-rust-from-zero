# `query_row`

## 本集目標

用 `query_row` 查出一列資料，把資料欄讀成 Rust 的型別；知道沒有那一列、型別對不上時，各會得到什麼錯誤。

## 正文

### 查一列

`conn.query_row(SQL, 參數, 閉包)` 跑一條 `SELECT`，把查到的第一列交給閉包。閉包拿到 `&Row`，用 `row.get(索引)` 讀出資料欄（索引從 0 開始），回傳值就是 `query_row` 的結果。

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL);
         INSERT INTO orders (drink, cups) VALUES ('紅茶', 2);
         INSERT INTO orders (drink, cups) VALUES ('咖啡', 1);",
    )?;
    let total: i64 = conn.query_row("SELECT SUM(cups) FROM orders", (), |row| row.get(0))?;
    println!("共 {total} 杯");
    let (drink, cups): (String, u32) =
        conn.query_row("SELECT drink, cups FROM orders WHERE id = 1", (), |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
    println!("第 1 筆：{drink} {cups} 杯");
    Ok(())
}
```

執行結果：

```text
共 3 杯
第 1 筆：紅茶 2 杯
```

- 閉包的回傳型別是 `rusqlite::Result<T>`。只要讀一欄，直接回傳 `row.get(0)`；讀好幾欄，就用 `?` 逐欄讀，再包進 `Ok(...)`（第 9 章第 5 集）。
- `row.get` 讀成什麼型別，由接收的地方決定：`let total: i64` 讓它讀成 `i64`，`(String, u32)` 讓兩欄各讀成 `String` 與 `u32`。型別對不上時的下場，見下一節。
- 欄位也可以用名稱讀，`row.get("drink")`，不必記住索引；不確定型別時寫成 `row.get::<_, String>("drink")`。
- `SELECT` 查到好幾列，`query_row` 只取第一列，其他的丟掉，不會報錯。所以要取特定的一列，通常搭配 `WHERE` 或 `ORDER BY` 加 `LIMIT 1`。
- 這一集的 SQL 都把條件直接寫死在字串裡（`WHERE id = 1`）；條件來自變數時怎麼寫，第 5 集才說明，現在不要用字串拼起來。

### 沒有那一列、型別對不上

查不到資料列，`query_row` 回傳 `Err(rusqlite::Error::QueryReturnedNoRows)`。它是一般的 `Err`，用 `match` 就能分辨（第 9 章第 3 集）。型別對不上也是 `Err`：

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL);
         INSERT INTO orders (drink, cups) VALUES ('紅茶', 2);",
    )?;
    let missing: rusqlite::Result<String> =
        conn.query_row("SELECT drink FROM orders WHERE id = 99", (), |row| {
            row.get(0)
        });
    match missing {
        Err(rusqlite::Error::QueryReturnedNoRows) => println!("沒有第 99 筆"),
        Err(e) => return Err(e),
        Ok(drink) => println!("{drink}"),
    }
    let wrong: rusqlite::Result<i64> =
        conn.query_row("SELECT drink FROM orders", (), |row| row.get(0));
    if let Err(e) = wrong {
        println!("讀不出來：{e}");
    }
    Ok(())
}
```

執行結果：

```text
沒有第 99 筆
讀不出來：Invalid column type Text at index: 0, name: drink
```

- `QueryReturnedNoRows` 是「沒有資料」，不一定是錯誤，常常只是正常的一種結果。第 7 集會看到把它變成 `Option` 的寫法。
- 讀取是嚴格的：`drink` 是文字，讀成 `i64` 就是 `InvalidColumnType`，rusqlite 不會替你轉換。數字放不下目標型別（把 300 讀成 `u8`）是 `IntegralValueOutOfRange`；索引或名稱不存在是 `InvalidColumnIndex`、`InvalidColumnName`。
- 這些都是執行時期才發現的錯誤：SQL 是字串，編譯器看不到它。所以每一次 `query_row` 都要用 `?` 或 `match` 處理 `Result`。

### 跟 Go 比一比

Go 的 `Scan` 對型別很寬鬆，能轉就轉：

```go
var n int
err := db.QueryRow("SELECT '42'").Scan(&n)
fmt.Println(n, err) // 42 <nil>
```

- 文字 `'42'` 讀進 `int` 變數，Go 會自動轉成 42；整數也能讀進 `string`。只有轉不動（`'abc'` 讀成 `int`）才有錯。rusqlite 不轉：文字就是文字，整數就是整數，型別不符一律是 `Err`。
- Go 寫成 `QueryRow(...).Scan(&x)`，查不到資料列的 `sql.ErrNoRows` 要等到 `Scan` 才冒出來；rusqlite 的 `query_row` 一次做完，結果就是一個 `Result`。

## 重點整理

- `conn.query_row(SQL, (), |row| ...)` 跑 `SELECT`，把第一列交給閉包，閉包回傳 `rusqlite::Result<T>`；`row.get(索引)` 或 `row.get("欄名")` 讀資料欄，型別由接收的地方決定。
- 讀多欄：`|row| Ok((row.get(0)?, row.get(1)?))`；`SELECT` 查到好幾列時只取第一列。
- 查不到資料列是 `Err(QueryReturnedNoRows)`；型別對不上是 `Err(InvalidColumnType)`、數字放不下是 `IntegralValueOutOfRange`，都是執行時期才發現的 `Err`。
- 這一集的條件寫死在 SQL 裡，變數怎麼帶進去是第 5 集。
- 跟 Go 比：Go 的 `Scan` 會自動轉型（文字 `'42'` 能讀成 `int`）；rusqlite 的 `get` 嚴格，型別不符就是 `Err`。
