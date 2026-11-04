# 預設不可變與 `let mut`

## 本集目標

知道 `let` 建立的變數預設不能改值，要改值得寫成 `let mut`。

## 正文

### 預設不可變

在 Go 裡，變數建立之後想改就改。Rust 不一樣，`let` 建立的變數**預設不可變**（immutable），值設定之後就不能再改：

```rust,compile_fail=E0384
fn main() {
    let x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");
}
```

編譯錯誤：

```text
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("x = {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

這是編譯錯誤，不是執行時才出事，程式根本跑不起來。訊息最後的 `help` 已經告訴我們怎麼改。

### 用 `mut` 讓變數可以改

在 `let` 後面加上 `mut`（mutable，可變），這個變數就能重新賦值：

```rust
fn main() {
    let mut score = 60;
    println!("目前 {score} 分");
    score = score + 10;
    println!("加分後 {score} 分");
}
```

執行結果：

```text
目前 60 分
加分後 70 分
```

`mut` 只是讓值可以改，型別還是固定的：`score` 一開始是整數，之後就只能放整數。

`score = score + 10` 有更簡短的寫法，第 7 集會教，現在先照這樣寫。

### 為什麼預設不可變

看到沒有 `mut` 的變數，就知道它從建立到消失都不會變，讀程式時少一件要追蹤的事。需要變的才加 `mut`，這樣「哪些地方會變動」一眼就能看出來。這個習慣在後面的所有權與借用裡會一直派上用場。

### 跟 Go 比一比

Go 的區域變數沒有「不可變」這個選項，一律可以重新賦值：

```go
x := 5
x = 6 // 合法
```

所以寫 Rust 時，Go 的習慣會讓你常常撞到 `E0384`。不用急著每個變數都加 `mut`：先照一般方式寫 `let`，編譯器說需要再加。反過來，加了 `mut` 卻從來沒有改過值，編譯器也會警告你多餘了。

## 重點整理

- `let` 建立的變數預設不可變，重新賦值會得到編譯錯誤 `E0384`。
- 要讓變數能改值，寫成 `let mut 名稱 = 值;`。
- `mut` 只讓值可以改，型別不會變。
- Go 的變數預設可變，Rust 相反；需要改才加 `mut`。
