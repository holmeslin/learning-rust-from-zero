# `prepare` 與 `query_map`

## 本集目標

用 `prepare` 準備好一條 SQL，再用 `query_map` 走訪查到的每一列，收成一份 `Vec`；知道為什麼走訪的迭代器不能活得比 `Statement` 久。

## 正文

### 查好幾列

`query_row` 只取一列。要拿全部的資料列，分兩步：`conn.prepare(SQL)` 把 SQL 編譯成預備陳述式（prepared statement，型別是 `Statement`），再呼叫它的 `query_map(參數, 閉包)`。閉包跟 `query_row` 的一樣，把一列變成一個值；`query_map` 回傳一個迭代器（第 11 章第 8 集），每個元素是 `rusqlite::Result<T>`：

```rust,noplayground
use rusqlite::Connection;

struct Order {
    id: i64,
    drink: String,
    cups: u32,
}

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL);
         INSERT INTO orders (drink, cups) VALUES ('紅茶', 2);
         INSERT INTO orders (drink, cups) VALUES ('咖啡', 1);
         INSERT INTO orders (drink, cups) VALUES ('奶茶', 3);",
    )?;
    let mut stmt = conn.prepare("SELECT id, drink, cups FROM orders ORDER BY id")?;
    let rows = stmt.query_map((), |row| {
        Ok(Order {
            id: row.get(0)?,
            drink: row.get(1)?,
            cups: row.get(2)?,
        })
    })?;
    for row in rows {
        let order = row?;
        println!("{}: {} x {}", order.id, order.drink, order.cups);
    }
    Ok(())
}
```

執行結果：

```text
1: 紅茶 x 2
2: 咖啡 x 1
3: 奶茶 x 3
```

- `prepare` 要 `let mut stmt`：走訪會改動 `Statement` 內部的狀態，所以 `query_map` 需要 `&mut self`。
- `query_map(...)?` 的 `?` 處理「SQL 本身有問題」；`for row in rows` 裡每一列還有各自的 `Result`，所以 `row?` 又處理一次，因為讀某一列可能失敗（型別對不上，像上一集）。
- 閉包把一列變成 `Order`：`row.get(0)?` 讀的型別由 `Order` 欄位的型別決定。每個欄位都要在程式裡讀到，不然 rustc 會警告 `dead_code`（第 5 章第 6 集），所以 `println!` 把三個欄位都印了。
- `Statement` 可以重複使用。同一條 SQL 要跑很多次，`prepare` 放在迴圈外面，只編譯一次。每次呼叫 `query_map` 都從頭查，看得到新寫入的資料。

### 收成 `Vec`

多半不會在 `for` 裡直接用，而是收成 `Vec`。迭代器的元素是 `Result<Order>`，`collect` 可以收成 `Result<Vec<Order>>`（第 11 章第 11 集）：只要有一列失敗，整個結果就是那個 `Err`。把查詢包成函式，回傳型別決定 `collect` 收成什麼：

```rust,noplayground
use rusqlite::Connection;

struct Order {
    id: i64,
    drink: String,
    cups: u32,
}

fn list(conn: &Connection) -> rusqlite::Result<Vec<Order>> {
    let mut stmt = conn.prepare("SELECT id, drink, cups FROM orders ORDER BY id")?;
    let rows = stmt.query_map((), |row| {
        Ok(Order {
            id: row.get(0)?,
            drink: row.get(1)?,
            cups: row.get(2)?,
        })
    })?;
    rows.collect()
}

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(
        "CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL, cups INTEGER NOT NULL);
         INSERT INTO orders (drink, cups) VALUES ('紅茶', 2);",
    )?;
    for order in list(&conn)? {
        println!("{}: {} x {}", order.id, order.drink, order.cups);
    }
    conn.execute("INSERT INTO orders (drink, cups) VALUES ('咖啡', 1)", ())?;
    println!("目前共 {} 筆", list(&conn)?.len());
    Ok(())
}
```

執行結果：

```text
1: 紅茶 x 2
目前共 2 筆
```

### 迭代器不能活得比 `stmt` 久

`prepare` 借用了 `conn`，`query_map` 回傳的迭代器又借用了 `stmt`（生命週期，第 10 章）。如果想把迭代器從放著 `stmt` 的區塊帶出去，`stmt` 先被 drop，迭代器就成了懸置參照：

```rust,compile_fail=E0597,noplayground
use rusqlite::Connection;

fn main() -> rusqlite::Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("CREATE TABLE orders (id INTEGER PRIMARY KEY, drink TEXT NOT NULL)")?;
    let rows = {
        let mut stmt = conn.prepare("SELECT drink FROM orders")?;
        stmt.query_map((), |row| row.get::<_, String>(0))?
    };
    for row in rows {
        println!("{}", row?);
    }
    Ok(())
}
```

編譯錯誤：

```text
error[E0597]: `stmt` does not live long enough
 --> src/main.rs:8:9
  |
6 |     let rows = {
  |         ---- borrow later stored here
7 |         let mut stmt = conn.prepare("SELECT drink FROM orders")?;
  |             -------- binding `stmt` declared here
8 |         stmt.query_map((), |row| row.get::<_, String>(0))?
  |         ^^^^ borrowed value does not live long enough
9 |     };
  |     - `stmt` dropped here while still borrowed
```

所以要在 `stmt` 還活著的時候把結果收完，像上面的 `list`：`rows.collect()` 在函式結束之前就把迭代器消耗掉，回傳的 `Vec<Order>` 已經是自己擁有的資料，不再借用任何東西。

### 跟 Go 比一比

```go
rows, err := db.Query("SELECT id, drink, cups FROM orders")
if err != nil {
	return err
}
defer rows.Close()
for rows.Next() {
	var o Order
	if err := rows.Scan(&o.ID, &o.Drink, &o.Cups); err != nil {
		return err
	}
}
return rows.Err()
```

- Go 有兩個要自己記的收尾動作：`rows.Close()`（忘了，連線就一直被佔住）、迴圈結束後的 `rows.Err()`（忘了，走到一半的錯誤就被吞掉）。
- Rust 的迭代器每個元素都是 `Result`，錯誤在走訪的當下就擺在你面前，用 `?` 或 `collect` 處理；`Statement` 離開作用域就自己釋放，沒有 `Close` 可以忘。

## 重點整理

- `conn.prepare(SQL)` 得到 `Statement`（要 `let mut`），`stmt.query_map((), |row| ...)` 回傳迭代器，每個元素是 `Result<T>`。
- 閉包把一列變成一個值（tuple 或 struct）；`query_map(...)?` 與每一列的 `row?` 各處理一次錯誤。
- 要 `Vec` 就 `collect`：元素是 `Result<T>` 時，可以直接收成 `Result<Vec<T>>`。
- `Statement` 借用 `conn`，迭代器借用 `Statement`；迭代器不能比 `stmt` 活得久（E0597），要在 `stmt` 還活著時收完。
- 跟 Go 比：Go 要自己 `rows.Close()` 與檢查 `rows.Err()`；Rust 的每個元素都是 `Result`，`stmt` 自動釋放。
