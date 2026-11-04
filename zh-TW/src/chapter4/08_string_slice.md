# 字串 slice `&str`

## 本集目標

用 `&s[起..迄]` 取得字串的一部分，知道它的型別是 `&str`、位置是以位元組計算，而且它是對原字串的借用。

## 正文

### 取一段

字串 slice 是指向 `String` 其中一段的參照。寫法是 `&s[起..迄]`，範圍包含「起」、不含「迄」：

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello}|{world}");
}
```

執行結果：

```text
hello|world
```

`hello` 的型別是 `&str`，讀成「字串 slice」。它沒有複製文字，只記著「從 `s` 的第 0 個位元組開始、長 5」：

```text
位置:             0 1 2 3 4 5 6 7 8 9 10
s 的 heap:        h e l l o   w o r l d
                  ^---------^
hello: [ ptr | len 5 ] --'
```

跟前面學的一樣，`&` 開頭就是借用：`hello` 借的是 `s`。

範圍可以省略：起點是 0 時可以不寫起（`&s[..5]`），到結尾時可以不寫迄（`&s[6..]`），兩個都不寫（`&s[..]`）就是整個字串。

### slice 是借用，所以受借用規則管

`hello` 借著 `s`，這時候去改 `s`，就是 `&` 和 `&mut` 撞在一起：

```rust,compile_fail=E0502
fn main() {
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    s.push_str("!!");
    println!("{hello}");
}
```

編譯錯誤：

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let hello = &s[0..5];
  |                  - immutable borrow occurs here
4 |     s.push_str("!!");
  |     ^^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("{hello}");
  |                ----- immutable borrow later used here
```

這跟第 4 集的 E0502 一樣。如果 `push_str` 讓 `s` 搬到別的記憶體，`hello` 指著的位置就失效了，所以編譯器不讓你改。把 `println!` 排到 `push_str` 前面就行（第 5 集）。

### 回傳 slice

函式可以回傳 slice。因為它借自傳進來的參數，第 6 集說的規則就成立：

```rust
fn prefix(s: &String, n: usize) -> &str {
    &s[0..n]
}

fn main() {
    let s = String::from("hello world");
    let p = prefix(&s, 5);
    println!("{p}");
}
```

執行結果：

```text
hello
```

回傳型別 `&str` 前面沒有寫任何生命週期標註，因為這個函式只有一個參照參數 `s`，編譯器知道回傳的是借自它。原理第 10 章再解釋。

### 位置是位元組，切到字元中間會 panic

`s[起..迄]` 的數字是**位元組**位置，不是第幾個字。英文字母一個字佔 1 個位元組，中文字佔 3 個（第 2 章第 15 集）。如果切在一個字的中間，執行時會 panic：

```rust,exit=101
fn main() {
    let s = String::from("哈囉");
    println!("{}", &s[0..3]);
    println!("{}", &s[0..2]);
}
```

執行結果（某一次）：

```text
哈

thread 'main' (156083154) panicked at src/main.rs:4:22:
end byte index 2 is not a char boundary; it is inside '哈' (bytes 0..3 of string)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

第一行 `&s[0..3]` 剛好是「哈」。第二行切到位元組 2，落在「哈」的中間，所以 panic，訊息說 `end byte index 2 is not a char boundary`。範圍超出字串長度也會 panic。`thread 'main'` 後面的數字每次執行都不一樣。

所以字串 slice 的起訖位置要自己算準。想逐字處理，第 7 章會學到更好的辦法。

### 跟 Go 比一比

Go 的 `s[0:5]` 看起來一樣，也是位元組位置，也不會複製。不一樣的有兩點：

```go
s := "hello world"
hello := s[0:5]
t := "哈囉"
fmt.Println(t[0:2]) // 不會 panic，印出壞掉的 UTF-8
```

- Go 切在字元中間，不會出錯，會得到一個壞掉的字串；Rust 直接 panic。
- Go 的字串不可變，原字串不會被改。Rust 的 `String` 可以改，所以 slice 存在的期間，借用規則不讓你改原本的 `String`。

## 重點整理

- `&s[起..迄]` 取得字串 slice，型別是 `&str`；範圍含起不含迄，可以省略起或迄（`..5`、`6..`、`..`）。
- slice 不複製資料，是對原 `String` 的借用，所以 slice 還要用時，不能改原本的 `String`（E0502）。
- 函式可以回傳 `&str`；只有一個參照參數時，回傳的 slice 借自它。
- 位置以位元組計算；切在多位元組字元（如中文）的中間、或超出範圍，執行時 panic。
- Go 切在字元中間不報錯；Rust 會 panic。
