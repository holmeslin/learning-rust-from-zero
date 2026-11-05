# 常用字串方法

## 本集目標

認得幾個最常用的字串方法：`trim`、`to_uppercase`、`replace`、`contains`、`starts_with`、`split`、`find`，知道哪些回傳新字串、哪些回傳 `bool` 或 `Option`。

## 正文

`String` 和 `&str` 都能呼叫這些方法（原理第 13 章），下面的範例不特別分。

### 回傳新字串：原字串不變

`trim` 去掉頭尾的空白，第 4 章第 10 集看過，它回傳 `&str`。`to_uppercase`、`to_lowercase`、`replace` 回傳新的 `String`，原本的字串不會被改：

```rust
fn main() {
    let order = String::from("  Red Tea, no ice  ");
    let clean = order.trim();
    println!("[{clean}]");
    println!("{}", clean.to_uppercase());
    println!("{}", clean.to_lowercase());
    println!("{}", clean.replace("no", "less"));
    println!("[{order}]");
}
```

執行結果：

```text
[Red Tea, no ice]
RED TEA, NO ICE
red tea, no ice
Red Tea, less ice
[  Red Tea, no ice  ]
```

最後一行的 `order` 還是原樣，頭尾的空白都在。

### 回傳 `bool`：判斷

`contains` 問有沒有包含某段文字，`starts_with`、`ends_with` 問開頭和結尾，`is_empty` 問是不是空的：

```rust
fn main() {
    let order = String::from("Red Tea, no ice");
    println!("{}", order.contains("Tea"));
    println!("{}", order.starts_with("Red"));
    println!("{}", order.ends_with("sugar"));
    println!("{}", order.is_empty());
}
```

執行結果：

```text
true
true
false
false
```

要找的東西可以是 `&str`，也可以是單一個 `char`，例如 `order.contains('T')`。

### `split`：切開

`split` 用指定的字元或文字把字串切成幾段，`split_whitespace` 用空白切（連續的空白當成一個）。結果跟 `chars()` 一樣，直接放在 `for` 後面走訪（原理第 11 章）：

```rust
fn main() {
    let line = "30,45,38";
    let mut total = 0;
    for part in line.split(',') {
        let price: i32 = part.parse().expect("請輸入整數");
        total += price;
    }
    println!("合計 {total}");
    for word in "紅茶  拿鐵 咖啡".split_whitespace() {
        println!("{word}");
    }
}
```

執行結果：

```text
合計 113
紅茶
拿鐵
咖啡
```

每一段 `part` 都是 `&str`，所以第 1 章第 10 集的 `parse` 固定句型可以直接接上去，把逗號分隔的數字加起來。

### `find`：找位置

`find` 回傳 `Option<usize>`：找到是 `Some(位置)`，找不到是 `None`。位置是位元組位置，剛好可以搭配第 4 章的 `&s[起..迄]`：

```rust
fn main() {
    let email = String::from("xiaoming@example.com");
    match email.find('@') {
        Some(i) => println!("帳號 {}，網域 {}", &email[..i], &email[i + 1..]),
        None => println!("不是 email"),
    }
}
```

執行結果：

```text
帳號 xiaoming，網域 example.com
```

`@` 是 ASCII，只佔一個位元組，所以網域從 `i + 1` 開始。要找的字元如果是中文，下一個位置要加上它佔的位元組數，不是加 1。

## 重點整理

- `to_uppercase`、`to_lowercase`、`replace(舊, 新)` 回傳新的 `String`，原字串不變；`trim` 回傳 `&str`。
- `contains`、`starts_with`、`ends_with`、`is_empty` 回傳 `bool`；要找的可以是 `&str` 或 `char`。
- `split(分隔)` 和 `split_whitespace()` 切開字串，可以直接用 `for` 走訪，每一段是 `&str`。
- `find` 回傳 `Option<usize>`，位置是位元組位置，常搭配 `&s[起..迄]`。
- 這些方法 `String` 與 `&str` 都能用，原理第 13 章解釋。
