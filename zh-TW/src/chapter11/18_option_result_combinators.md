# `Option` / `Result` 的組合子

## 本集目標

用 `Option` 與 `Result` 的方法（`unwrap_or`、`map`、`and_then`、`ok_or`、`ok`、`map_err`、`is_some`、`is_ok`……）取代一部分 `match`，把「有值、沒值」「成功、失敗」串成一條。

## 正文

### 把它們看成最多一個元素的集合

`Option` 像是「最多只有一個元素」的集合，`Result` 像是「成功時有一個元素」的集合。迭代器的 `map`、`filter` 能做的事，它們也有對應的方法。這些方法常被叫做**組合子**（combinator）：不用 `match`，就能把值串起來。

下面前兩組方法，都用同一個回傳 `Option<u32>` 的 `find_price` 來示範。

### 取值，或給預設值

```rust
fn find_price(code: u32) -> Option<u32> {
    match code {
        1 => Some(30),
        2 => Some(45),
        _ => None,
    }
}

fn default_price() -> u32 {
    println!("（查預設價格）");
    35
}

fn main() {
    let tea = find_price(1);
    let unknown = find_price(9);
    println!("{} {}", tea.is_some(), unknown.is_none());
    println!("{}", unknown.unwrap_or(0));
    println!("{}", tea.unwrap_or(default_price()));
    println!("{}", tea.unwrap_or_else(default_price));
    println!("{}", unknown.unwrap_or_else(default_price));
}
```

執行結果：

```text
true true
0
（查預設價格）
30
30
（查預設價格）
35
```

- `is_some()`、`is_none()` 回答有沒有值，回傳 `bool`（`Result` 對應 `is_ok()`、`is_err()`）。
- `unwrap_or(預設值)`：`Some` 取值，`None` 用預設值，不會 panic。
- `unwrap_or_else(閉包)`：預設值要算的時候，改傳閉包（或函式名稱），只有 `None` 才會呼叫。看輸出：`tea.unwrap_or(default_price())` 那行，`tea` 明明是 `Some`，還是先印出「查預設價格」，因為引數一定會先算；`tea.unwrap_or_else(default_price)` 那行沒有印，只有 `unknown` 那行才呼叫了它。

### 轉換裡面的值：`map` 與 `and_then`

`map(閉包)` 只改 `Some` 裡面的值，`None` 還是 `None`。如果閉包本身也回傳 `Option`，用 `map` 會多包一層，改用 `and_then`，它會把兩層攤平：

```rust
fn find_price(code: u32) -> Option<u32> {
    match code {
        1 => Some(30),
        2 => Some(45),
        _ => None,
    }
}

fn member_price(price: u32) -> Option<u32> {
    if price >= 40 { Some(price - 5) } else { None }
}

fn main() {
    println!("{:?}", find_price(1).map(|p| p + 5));
    println!("{:?}", find_price(9).map(|p| p + 5));
    println!("{:?}", find_price(2).map(member_price));
    println!("{:?}", find_price(2).and_then(member_price));
    println!("{:?}", find_price(1).and_then(member_price));
}
```

執行結果：

```text
Some(35)
None
Some(Some(40))
Some(40)
None
```

會員價只有 40 元以上的飲料才有，所以 `member_price` 也可能回傳 `None`。`map(member_price)` 得到 `Some(Some(40))`，`and_then(member_price)` 才是我們要的 `Some(40)`；任何一步是 `None`，結果就是 `None`。

### `Option` 與 `Result` 互轉，換錯誤

- `ok_or(錯誤)`、`ok_or_else(閉包)`：`Option` 變 `Result`，`None` 換成指定的 `Err`。第 9 章第 6 集 `?` 的 `help` 建議的就是它。
- `ok()`：`Result` 變 `Option`，丟掉錯誤的原因，`Err` 變 `None`。
- `map_err(閉包)`：只改 `Err` 裡的錯誤，`Ok` 不動；常搭配 `?` 使用。
- `map`、`unwrap_or`、`is_ok` 在 `Result` 上也有，意思相同。

第 9 章第 6 集用 `match` 寫的 `price_of`，現在可以一行完成；再加一個轉換數量的步驟：

```rust
use std::collections::HashMap;

fn total(menu: &HashMap<String, u32>, name: &str, qty: &str) -> Result<u32, String> {
    let price = menu.get(name).ok_or_else(|| format!("菜單沒有 {name}"))?;
    let qty: u32 = qty.parse().map_err(|e| format!("數量不對：{e}"))?;
    Ok(price * qty)
}

fn main() {
    let mut menu: HashMap<String, u32> = HashMap::new();
    menu.insert(String::from("紅茶"), 30);
    println!("{:?}", total(&menu, "紅茶", "3"));
    println!("{:?}", total(&menu, "奶茶", "3"));
    println!("{:?}", total(&menu, "紅茶", "abc"));
    println!("{:?}", "abc".parse::<u32>().ok());
}
```

執行結果：

```text
Ok(90)
Err("菜單沒有 奶茶")
Err("數量不對：invalid digit found in string")
None
```

- `ok_or_else(|| format!(...))` 讓沒找到時才組錯誤訊息；錯誤是 `&str` 這種便宜的值，直接 `ok_or("沒有")` 就好。
- `qty.parse()` 的錯誤是 `ParseIntError`，函式回傳的是 `Result<u32, String>`，`?` 轉不過去（第 9 章第 9 集），所以用 `map_err` 先換成 `String`。
- 最後一行 `.ok()` 把解析失敗變成 `None`。

## 重點整理

- `is_some()`、`is_none()`、`is_ok()`、`is_err()` 回傳 `bool`；`unwrap_or(值)` 給預設值，`unwrap_or_else(閉包)` 只在需要時才計算。
- `map(閉包)` 轉換 `Some`/`Ok` 裡的值；閉包本身回傳 `Option` 時用 `and_then`，避免多一層。
- `ok_or(錯誤)`／`ok_or_else(閉包)` 把 `Option` 變成 `Result`；`ok()` 把 `Result` 變成 `Option`；`map_err(閉包)` 換掉 `Err` 的內容，常搭配 `?`。
- 預設值或錯誤要現場計算時用 `_else` 版本，便宜的值直接用 `unwrap_or`、`ok_or`。
