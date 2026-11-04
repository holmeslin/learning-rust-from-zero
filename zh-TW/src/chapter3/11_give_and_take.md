# 「交出去再拿回來」很麻煩

## 本集目標

看出「傳進去、再傳回來」這種寫法有多麻煩，知道為什麼 Rust 需要第 4 章的借用。

## 正文

### 只是想看一眼

假設要寫一個函式，算 `String` 的長度。函式只需要讀它，卻因為參數是 `String`，把所有權整個拿走了：

```rust,compile_fail=E0382
fn length(s: String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("哈囉");
    let n = length(s);
    println!("{s} 的長度是 {n}");
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:8:16
  |
6 |     let s = String::from("哈囉");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
7 |     let n = length(s);
  |                    - value moved here
8 |     println!("{s} 的長度是 {n}");
  |                ^ value borrowed here after move
  |
note: consider changing this parameter type in function `length` to borrow instead if owning the value isn't necessary
 --> src/main.rs:1:14
  |
1 | fn length(s: String) -> usize {
  |    ------    ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
help: consider cloning the value if the performance cost is acceptable
  |
7 |     let n = length(s.clone());
  |                     ++++++++
```

`length(s)` 把 `s` move 進去，函式一結束就 drop 了，`main` 裡不能再印 `s`。

### 把它還回來

照上一集的做法，函式把 `String` 連同結果一起回傳。要回傳兩個東西，就用 tuple（第 2 章第 9 集），呼叫端再解構（第 10 集）：

```rust
fn length(s: String) -> (String, usize) {
    let n = s.len();
    (s, n)
}

fn main() {
    let s = String::from("哈囉");
    let (s, n) = length(s);
    println!("{s} 的長度是 {n}");
}
```

執行結果：

```text
哈囉 的長度是 6
```

能用了。注意函式裡要先算好 `let n = s.len();`，再回傳 `(s, n)`。如果寫成 `(s, s.len())`，`s` 在 tuple 的第一個位置就已經 move 走，後面的 `s.len()` 又用到它，同樣是 E0382。

### 東西一多就更痛

如果要算兩個字串的總長度，兩個都得還回來：

```rust
fn total_length(a: String, b: String) -> (String, String, usize) {
    let n = a.len() + b.len();
    (a, b, n)
}

fn main() {
    let a = String::from("哈囉");
    let b = String::from("世界");
    let (a, b, n) = total_length(a, b);
    println!("{a} {b} 共 {n} 個位元組");
}
```

執行結果：

```text
哈囉 世界 共 12 個位元組
```

函式只是想讀一讀，卻得把每個參數都原封不動回傳，呼叫端還得照順序一個個接回來。參數愈多，愈容易寫錯。用 `clone` 能繞過去，但每次都要複製一份，也不划算。

### 需要的是另一種做法

這些麻煩的根源，是「用一下」和「擁有」被綁在一起：函式只想用，卻只能拿走。如果有辦法讓函式「借用」一下，用完就還，所有權根本不必搬家，上面的程式就能簡單很多。這正是下一章的主題。

## 重點整理

- 參數是 `String` 的函式會拿走所有權，即使它只是想讀；呼叫端之後就不能再用（E0382）。
- 要拿回來，只能把它放進回傳值（常用 tuple）再解構：`let (s, n) = length(s);`。
- 要還的東西愈多，函式簽名和呼叫端都愈長、愈容易出錯；用 `clone` 則每次都要複製。
- 問題在於「只想用」也得「擁有」；第 4 章的借用就是解法。
