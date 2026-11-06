# `String` 深入：`push`、`+`、`format!`

## 本集目標

學會把文字接成新字串的四種方法：`push`（加一個字元）、`+=`、`+`（會 move 左邊）和 `format!`（不 move 任何東西）。

## 正文

### `push`、`push_str` 與 `+=`

第 2 章第 15 集學過 `push_str`，加的是一段文字。只加一個字元，用 `push`，參數是單引號的 `char`（第 1 章第 18 集）。`+=` 也能把文字接到後面，效果跟 `push_str` 一樣：

```rust
fn main() {
    let mut order = String::from("紅茶");
    order.push('，');
    order.push_str("少冰");
    order += "、去糖";
    println!("{order}");
}
```

執行結果：

```text
紅茶，少冰、去糖
```

三種寫法都直接改動 `order`，所以要 `let mut`。`push` 的參數是 `char`（單引號），`push_str` 和 `+=` 要 `&str`（雙引號）：把 `'，'` 寫成 `"，"` 給 `push`，型別就不對了。

### `+`：左邊會被 move

兩個字串相接，也可以用 `+`。左邊必須是 `String`，右邊必須是 `&str`：

```rust
fn main() {
    let first = String::from("紅茶");
    let second = String::from("拿鐵");
    let both = first + "和" + &second;
    println!("{both}");
    println!("{second}");
}
```

執行結果：

```text
紅茶和拿鐵
拿鐵
```

- `first + "和"`：`first` 的所有權交給 `+`，`+` 在它後面接上「和」，把結果當成新的 `String` 還給你，再接 `&second`。接完之後 `first` 已經被 move，不能再用了（第 3 章第 5 集）。
- `&second`：右邊要的是 `&str`，`&String` 會自動轉換，跟第 4 章第 10 集一樣，現在照寫。`second` 只是被借用，後面還能用。

右邊少了 `&` 的話：

```rust,compile_fail=E0308
fn main() {
    let first = String::from("紅茶");
    let second = String::from("拿鐵");
    let both = first + second;
    println!("{both}");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:4:24
  |
4 |     let both = first + second;
  |                        ^^^^^^ expected `&str`, found `String`
  |
help: consider borrowing here
  |
4 |     let both = first + &second;
  |                        +
```

`` expected `&str`, found `String` ``，`help` 建議在 `second` 前面加 `&`，照做就好。

### `format!`：組合多段，不拿走任何東西

要接的片段很多，或者夾雜數字，`+` 就很難讀。`format!` 的格式字串跟 `println!` 一樣，不同的是它不印出來，而是回傳一個 `String`。`format!` 的原理是巨集，第 16 章第 1 集才會解釋，現在照 `println!` 的寫法用就好：

```rust
fn main() {
    let name = String::from("小明");
    let drink = String::from("紅茶");
    let price = 30;
    let line = format!("{name} 點了 {drink}，{price} 元");
    println!("{line}");
    println!("{name} {drink}");
}
```

執行結果：

```text
小明 點了 紅茶，30 元
小明 紅茶
```

`format!` 只是讀取 `name`、`drink`，沒有拿走它們，所以最後一行還能印。

### 跟 Go 比一比

Go 的 `+` 兩邊都是 `string`，兩邊用完還能繼續用：

```go
first := "紅茶"
second := "拿鐵"
both := first + "和" + second
fmt.Println(first, second, both) // 都還在
```

Rust 的 `+` 會拿走左邊的 `String`（直接在它後面接文字，省掉一次複製），所以左邊用完就沒了；右邊要寫成 `&str`。Go 的 `fmt.Sprintf` 對應 Rust 的 `format!`。

## 重點整理

- `s.push(c)` 加一個字元（`char`，單引號）；`s.push_str("...")` 和 `s += "..."` 加一段文字；`s` 要 `let mut`。
- `a + &b`：左邊必須是 `String`，而且會被 move；右邊要 `&str`（`&String` 會自動轉換）；`a + b` 是 E0308。
- `format!("...")` 格式字串跟 `println!` 一樣，回傳 `String`，不拿走任何變數；原理第 16 章第 1 集解釋。
- 跟 Go 比：Go 的 `+` 兩邊都還能用；Rust 的 `+` 會 move 左邊。
