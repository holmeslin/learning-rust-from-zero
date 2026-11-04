# 函式

## 本集目標

用 `fn` 定義自己的函式，並在 `main` 裡呼叫它。

## 正文

### 定義與呼叫

第 1 章每支程式都有的 `fn main() { ... }` 就是函式。自己的函式寫法一樣：`fn`、名稱、一對小括號、大括號。呼叫時寫名稱加小括號：

```rust
fn print_line() {
    println!("--------");
}

fn main() {
    print_line();
    println!("成績單");
    print_line();
}
```

執行結果：

```text
--------
成績單
--------
```

`main` 是程式開始的地方。其他函式要被 `main` 呼叫（直接或間接）才會執行，這點跟 Go 一樣。

### 順序不重要

函式可以寫在呼叫它的地方後面，編譯器會看完整個檔案才開始編譯，不需要先宣告：

```rust
fn main() {
    announce();
}

fn announce() {
    println!("歡迎光臨");
}
```

執行結果：

```text
歡迎光臨
```

Go 也是這樣。本書習慣把 `main` 放在最前面或最後面都可以，只要讀起來順就好。

### 跟 Go 比一比

Go 的函式名稱用駝峰：小寫開頭的 `sayHello` 只有同一個套件能用，大寫開頭的 `SayHello` 才會對外公開。Rust 的慣例不一樣：函式名稱用**小寫加底線**（snake_case），像 `say_hello`；能不能對外公開跟名字大小寫無關，而是靠 `pub` 關鍵字，第 12 章才會介紹。

照 Go 的習慣寫駝峰，程式還是能跑，但編譯器會警告：

```rust,warn
fn sayHello() {
    println!("哈囉");
}

fn main() {
    sayHello();
}
```

編譯警告：

```text
warning: function `sayHello` should have a snake case name
 --> src/main.rs:1:4
  |
1 | fn sayHello() {
  |    ^^^^^^^^ help: convert the identifier to snake case: `say_hello`
  |
  = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default
```

警告裡的 `help` 已經幫你想好改名。本書的範例都沒有警告，所以你會一直看到 `say_hello`、`print_line` 這種名字。

## 重點整理

- 定義函式：`fn 名稱() { ... }`；呼叫函式：`名稱();`。
- 程式從 `main` 開始執行。
- 函式寫在呼叫處的前面或後面都可以。
- 函式名稱用小寫加底線（snake_case），寫成駝峰會有警告；Go 靠大小寫決定公開與否，Rust 靠 `pub`（第 12 章）。
