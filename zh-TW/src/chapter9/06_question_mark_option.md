# `?` 也能用在 `Option`

## 本集目標

在回傳 `Option` 的函式裡用 `?`：有值就取出，`None` 就提早回傳 `None`；並知道不能把 `Option` 的 `?` 用在回傳 `Result` 的函式裡。

## 正文

### `None` 就提早回傳

`?` 不只認 `Result`。放在回傳 `Option` 的函式裡，規則幾乎一樣：

```rust
use std::collections::HashMap;

fn combo_price(menu: &HashMap<String, u32>, first: &str, second: &str) -> Option<u32> {
    let a = menu.get(first)?;
    let b = menu.get(second)?;
    Some(a + b)
}

fn main() {
    let mut menu: HashMap<String, u32> = HashMap::new();
    menu.insert(String::from("紅茶"), 30);
    menu.insert(String::from("咖啡"), 45);
    println!("{:?}", combo_price(&menu, "紅茶", "咖啡"));
    println!("{:?}", combo_price(&menu, "紅茶", "奶茶"));
}
```

執行結果：

```text
Some(75)
None
```

- 運算式是 `Some(值)`：`menu.get(first)?` 的值就是 `值`，這裡是 `&u32`（第 7 章第 12 集），程式繼續往下。
- 運算式是 `None`：函式立刻 `return None`。第二次呼叫的 `奶茶` 不在菜單上，所以 `let b` 那行就結束了，沒有走到 `Some(a + b)`。
- 最後要自己包成 `Some(a + b)`，因為函式回傳的是 `Option<u32>`。跟 `Result` 一樣，不用 `match` 就能把「任何一步沒有就整個沒有」寫成直線。

### 不能混用

`?` 要把 `None` 或 `Err` 原樣交出去，所以函式的回傳型別必須是同一類。在回傳 `Result` 的函式裡對 `Option` 用 `?`，編譯器不會幫你把 `None` 變成 `Err`：

```rust,compile_fail=E0277
use std::collections::HashMap;

fn price_of(menu: &HashMap<String, u32>, name: &str) -> Result<u32, String> {
    let price = menu.get(name)?;
    Ok(*price)
}

fn main() {
    let mut menu: HashMap<String, u32> = HashMap::new();
    menu.insert(String::from("紅茶"), 30);
    println!("{:?}", price_of(&menu, "奶茶"));
}
```

編譯錯誤：

```text
error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
 --> src/main.rs:4:31
  |
3 | fn price_of(menu: &HashMap<String, u32>, name: &str) -> Result<u32, String> {
  | --------------------------------------------------------------------------- this function returns a `Result`
4 |     let price = menu.get(name)?;
  |                               ^ use `.ok_or(...)?` to provide an error compatible with `Result<u32, String>`
```

`` the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result` ``：`None` 裡沒有失敗的原因，編譯器不知道該填什麼。這個原因要你自己給。箭頭旁建議的 `.ok_or(...)` 是第 11 章的方法，現在先用 `match` 把 `Option` 轉成 `Result`：

```rust
use std::collections::HashMap;

fn price_of(menu: &HashMap<String, u32>, name: &str) -> Result<u32, String> {
    match menu.get(name) {
        Some(price) => Ok(*price),
        None => Err(format!("菜單沒有 {name}")),
    }
}

fn main() {
    let mut menu: HashMap<String, u32> = HashMap::new();
    menu.insert(String::from("紅茶"), 30);
    println!("{:?}", price_of(&menu, "紅茶"));
    println!("{:?}", price_of(&menu, "奶茶"));
}
```

執行結果：

```text
Ok(30)
Err("菜單沒有 奶茶")
```

反過來也一樣：在回傳 `Option` 的函式裡，對 `Result` 用 `?` 不行。現在的規則是「函式回傳什麼，`?` 就只能接同一類」。

## 重點整理

- 在回傳 `Option` 的函式裡，`運算式?`：`Some(值)` 取出值，`None` 就立刻 `return None`。
- 最後要用 `Some(...)` 包起結果；多個步驟都可能沒有時，用 `?` 比巢狀 `match` 簡單。
- `?` 不能混用：在回傳 `Result` 的函式裡對 `Option` 用 `?` 是 E0277（反過來也是）。
- 要把 `Option` 變成 `Result`，用 `match` 自己給失敗的原因：`Some(x) => Ok(x)`、`None => Err(...)`。
