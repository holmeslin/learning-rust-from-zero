# 參數綁定

## 本集目標

把變數帶進 SQL：用 `?1` 這種佔位符加 `params!`，取代字串拼接；認得 `named_params!` 的具名參數，並知道哪些型別、哪些位置不能綁。

## 正文

### 佔位符與 `params!`

前幾集的 SQL，值都寫死在字串裡。條件或資料來自變數時，在 SQL 裡寫佔位符，再把值放進 `execute`、`query_row`、`query_map` 的第二個引數，也就是前幾集照抄的 `()` 的位置：

```rust,noplayground
use rusqlite::{Connection, params};

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL)",
    )?;
    let drink = "紅茶";
    let cups = 2;
    conn.execute(
        "INSERT INTO orders (drink, cups) VALUES (?1, ?2)",
        params![drink, cups],
    )?;
    conn.execute(
        "INSERT INTO orders (drink, cups) VALUES (?1, ?2)",
        params!["咖啡", 1],
    )?;
    let n = conn.execute(
        "UPDATE orders SET cups = ?1 WHERE drink = ?2",
        params![5, "咖啡"],
    )?;
    println!("更新 {n} 筆");
    let cups: u32 = conn.query_row("SELECT cups FROM orders WHERE drink = ?1", [drink], |row| {
        row.get(0)
    })?;
    println!("{drink}：{cups} 杯");
    Ok(())
}
```

執行結果：

```text
更新 1 筆
紅茶：2 杯
```

- `?1`、`?2` 是佔位符，數字是參數的位置，從 1 開始；`params![a, b]` 依序提供。只有一個參數時，不必編號，寫 `?` 也行。
- `params!` 是巨集（第 16 章），參數的型別可以各不相同：字串、整數混著放。`params![]` 裡放的是參照，原本的變數照樣能用。
- 全部是同一種型別的話，也可以直接寫陣列：`[drink]`、`[1, 2]`。混了不同型別的陣列，是一般的陣列型別錯誤（E0308）：

```rust,compile_fail=E0308,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE orders (drink TEXT NOT NULL, cups INTEGER NOT NULL)")?;
    conn.execute("INSERT INTO orders (drink, cups) VALUES (?1, ?2)", ["紅茶", 2])?;
    Ok(())
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:6:77
  |
6 |     conn.execute("INSERT INTO orders (drink, cups) VALUES (?1, ?2)", ["紅茶", 2])?;
  |                                                                               ^ expected `&str`, found integer
  |
help: replace the comma with a semicolon to create an array
  |
6 -     conn.execute("INSERT INTO orders (drink, cups) VALUES (?1, ?2)", ["紅茶", 2])?;
6 +     conn.execute("INSERT INTO orders (drink, cups) VALUES (?1, ?2)", ["紅茶"; 2])?;
  |
```

編譯器建議「把逗號換成分號」，那是在教你寫「重複 2 次」的陣列，不是我們要的意思，不用理它；混型別就改用 `params!`。小括號的 tuple（`("紅茶", 2)`）也行，這本書統一用 `params!`。

### 為什麼不用 `format!` 拼字串

用 `format!` 把值塞進 SQL，看起來也能動，但是值裡的字元會被當成 SQL 本身。把兩種寫法放在一起比：

```rust,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL);
         INSERT INTO orders (drink, cups) VALUES ('紅茶', 2);
         INSERT INTO orders (drink, cups) VALUES ('咖啡', 1);",
    )?;
    let input = "x' OR '1'='1";
    let sql = format!("SELECT count(*) FROM orders WHERE drink = '{input}'");
    println!("{sql}");
    let n: i64 = conn.query_row(&sql, (), |row| row.get(0))?;
    println!("拼字串：找到 {n} 筆");
    let n: i64 = conn.query_row(
        "SELECT count(*) FROM orders WHERE drink = ?1",
        [input],
        |row| row.get(0),
    )?;
    println!("綁定：找到 {n} 筆");
    let apostrophe = "it's";
    let sql = format!("INSERT INTO orders (drink, cups) VALUES ('{apostrophe}', 1)");
    if let Err(e) = conn.execute(&sql, ()) {
        println!("{e}");
    }
    Ok(())
}
```

執行結果：

```text
SELECT count(*) FROM orders WHERE drink = 'x' OR '1'='1'
拼字串：找到 2 筆
綁定：找到 0 筆
near "s": syntax error in INSERT INTO orders (drink, cups) VALUES ('it's', 1) at offset 45
```

- 輸入 `x' OR '1'='1` 改變了 SQL 的意思，條件變成「永遠成立」，每一筆都被找出來：這就是 SQL 注入（SQL injection）。輸入含一個單引號（`it's`），連 SQL 都寫壞了。
- 用 `?1` 綁定，值永遠只是「值」，不會被當成 SQL 解讀。凡是來自變數的值，一律用佔位符。

### 具名參數

參數很多的時候，數字容易對錯。SQL 裡寫 `:名字`，參數用 `named_params!` 提供：

```rust,noplayground
use rusqlite::{Connection, named_params};

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL)",
    )?;
    conn.execute(
        "INSERT INTO orders (drink, cups) VALUES (:drink, :cups)",
        named_params! { ":drink": "紅茶", ":cups": 2 },
    )?;
    let cups: u32 = conn.query_row(
        "SELECT cups FROM orders WHERE drink = :drink",
        named_params! { ":drink": "紅茶" },
        |row| row.get(0),
    )?;
    println!("{cups}");
    Ok(())
}
```

執行結果：

```text
2
```

### 能綁的與不能綁的

- 佔位符只能代替「值」，不能代替資料表或欄位名稱。`INSERT INTO ?1 ...` 是語法錯誤（`near "?1": syntax error`）；名稱要靠程式自己從已知的清單挑，不能直接用使用者的輸入組字串。
- 參數個數對不上，執行時才會發現：`params!["紅茶"]` 配兩個佔位符，是 `Err(InvalidParameterCount(1, 2))`。
- `u64` 沒有實作 `ToSql`（`params![u64::MAX]` 是 E0277）：SQLite 的整數是 64 位元有號整數，放不下 `u64` 的上半段。用 `i64`，或 `u8`、`u32` 這類放得下的型別。

## 重點整理

- 值來自變數時，SQL 裡寫佔位符（`?1`、`?2`，從 1 起算），第二個引數用 `params![a, b]`（型別可以不同）或同型別的陣列 `[a]`；混型別的陣列是 E0308。
- 不要用 `format!` 拼 SQL：值會被當成 SQL 解讀（SQL 注入），含單引號還會把 SQL 寫壞；綁定的值永遠只是值。
- 具名參數：SQL 寫 `:名字`，參數用 `named_params! { ":名字": 值 }`。
- 佔位符只能代替值，不能代替資料表或欄位名稱；個數不對是執行時期的 `InvalidParameterCount`。
- `u64` 不能當參數（沒有 `ToSql`）；SQLite 的整數是 64 位元有號整數。
