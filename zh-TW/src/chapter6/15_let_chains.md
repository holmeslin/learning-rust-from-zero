# let chains（edition 2024）

## 本集目標

用 `&&` 把 `let` 模式和一般條件接在同一個 `if` 裡，讓「符合模式、而且滿足條件」不必再巢狀兩層。

## 正文

### 巢狀的 `else` 要寫兩次

想要「查得到價格，而且超過 30 元」才標示偏貴，其他情況一律說不算貴。用 `if let` 加一層 `if`，`else` 就得寫兩份：

```rust
fn find_price(code: u32) -> Option<u32> {
    if code == 1 {
        Some(30)
    } else if code == 2 {
        Some(45)
    } else {
        None
    }
}

fn nested(code: u32) {
    if let Some(price) = find_price(code) {
        if price > 30 {
            println!("{code} 號偏貴：{price} 元");
        } else {
            println!("{code} 號不算貴");
        }
    } else {
        println!("{code} 號不算貴");
    }
}

fn chain(code: u32) {
    if let Some(price) = find_price(code)
        && price > 30
    {
        println!("{code} 號偏貴：{price} 元");
    } else {
        println!("{code} 號不算貴");
    }
}

fn main() {
    for code in [1, 2, 9] {
        nested(code);
        chain(code);
    }
}
```

執行結果：

```text
1 號不算貴
1 號不算貴
2 號偏貴：45 元
2 號偏貴：45 元
9 號不算貴
9 號不算貴
```

`nested` 和 `chain` 結果相同。`chain` 用的是 **let chains**：在 `if` 的條件裡，用 `&&` 把 `let 模式 = 值` 跟一般的 `bool` 條件連起來，只要有一項不成立，就走 `else`。`else` 只要寫一次。

### 規則

- 每一項可以是 `let 模式 = 值`，也可以是普通條件，順序隨你排，中間用 `&&` 隔開。
- 由左到右檢查，前面不成立就停下來，不會往後看。
- 前面綁定的變數，後面的條件和大括號裡都能用：`price > 30` 用到了前一項的 `price`。
- 這些變數只在條件全部成立的大括號裡存在，`else` 裡看不到。
- 可以串起好幾個 `let`：

```rust
fn main() {
    let a: Option<u32> = Some(30);
    let b: Option<u32> = Some(45);
    if let Some(x) = a
        && let Some(y) = b
        && x + y > 50
    {
        println!("兩個價格都查到了，合計 {} 元，超過 50", x + y);
    }
}
```

執行結果：

```text
兩個價格都查到了，合計 75 元，超過 50
```

### 不能用 `||`

`let` 跟 `||` 不能放在同一個條件裡，因為「或」的時候，不知道哪邊的變數有值：

```rust,compile_fail
fn main() {
    let a: Option<u32> = Some(30);
    let flag = false;
    if let Some(x) = a || flag {
        println!("{x}");
    }
}
```

編譯錯誤：

```text
error: `||` operators are not supported in let chain conditions
 --> src/main.rs:4:24
  |
4 |     if let Some(x) = a || flag {
  |                        ^^
```

### 版本需求

let chains 在 Rust 1.88 才穩定，而且只有 edition 2024 能用。第 1 章 `cargo new` 產生的專案，`Cargo.toml` 已經是 `edition = "2024"`，直接就能寫。在舊的 edition，同樣的程式會編譯失敗，改用巢狀的 `if let`，或是第 10 集的 match guard。

## 重點整理

- let chains：`if let Some(x) = a && x > 3 { ... } else { ... }`，用 `&&` 把 `let 模式 = 值` 與一般條件串起來。
- 由左到右檢查，前面綁定的變數後面可以用；變數只在條件全部成立的區塊裡存在。
- 不用在 `if let` 裡再包一層 `if`，也不用把 `else` 寫兩次。
- `let` 不能搭配 `||`。
- 需要 Rust 1.88 以上，而且只有 edition 2024 可用。
