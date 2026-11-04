# 參數一定要標型別

## 本集目標

替函式加上參數，並記住每個參數都要寫出型別。

## 正文

### 參數的寫法

參數寫在函式名稱後面的小括號裡，格式是 `名稱: 型別`，有好幾個就用逗號隔開。呼叫時照順序給值：

```rust
fn print_total(price: i32, count: i32) {
    println!("{price} 元買 {count} 個，共 {} 元", price * count);
}

fn main() {
    print_total(30, 4);
    print_total(120, 2);
}
```

執行結果：

```text
30 元買 4 個，共 120 元
120 元買 2 個，共 240 元
```

呼叫時給的值，個數和型別都要跟定義一致。

### 型別不能省

`let` 可以讓編譯器推斷型別，但函式的參數**一定要寫出型別**，沒有例外。函式的參數列就像一份說明書：不用讀函式裡面，光看第一行就知道它要什麼、給錯了編譯器會立刻擋下來。

下面把 `count` 換成浮點數，型別對不上：

```rust,compile_fail=E0308
fn print_total(price: i32, count: i32) {
    println!("{price} 元買 {count} 個，共 {} 元", price * count);
}

fn main() {
    print_total(30, 2.5);
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:6:21
  |
6 |     print_total(30, 2.5);
  |     -----------     ^^^ expected `i32`, found floating-point number
  |     |
  |     arguments to this function are incorrect
  |
note: function defined here
 --> src/main.rs:1:4
  |
1 | fn print_total(price: i32, count: i32) {
  |    ^^^^^^^^^^^             ----------
```

箭頭指著 `2.5`，並說明它是 `i32` 該放的位置，卻給了浮點數。Rust 不會幫你把 `2.5` 悄悄捨去成 `2`。

### 跟 Go 比一比

Go 允許相鄰、型別相同的參數合併寫成 `price, count int`。Rust 沒有這個寫法，每個參數都要自己帶型別：

```go
func printTotal(price, count int) {}
```

照 Go 的習慣寫，會得到語法錯誤：

```rust,compile_fail
fn print_total(price, count: i32) {
    println!("{price} 元買 {count} 個，共 {} 元", price * count);
}

fn main() {
    print_total(30, 4);
}
```

編譯錯誤：

```text
error: expected one of `:`, `@`, or `|`, found `,`
 --> src/main.rs:1:21
  |
1 | fn print_total(price, count: i32) {
  |                     ^ expected one of `:`, `@`, or `|`
  |
help: if this is a parameter name, give it a type
  |
1 | fn print_total(price: TypeName, count: i32) {
  |                     ++++++++++
help: if this is a type, explicitly ignore the parameter name
  |
1 | fn print_total(_: price, count: i32) {
  |                ++
```

編譯器在 `price` 後面等一個 `:`，也就是型別標註。兩個 `help` 的建議都用不到，直接改成 `price: i32, count: i32` 就好。

另一個差別：Go 的參數是函式裡的區域變數，想改就改。Rust 的參數跟 `let` 建立的變數一樣，**預設不可變**：

```rust,compile_fail=E0384
fn countdown(n: i32) {
    n -= 1;
    println!("{n}");
}

fn main() {
    countdown(3);
}
```

編譯錯誤：

```text
error[E0384]: cannot assign to immutable argument `n`
 --> src/main.rs:2:5
  |
2 |     n -= 1;
  |     ^^^^^^ cannot assign to immutable argument
  |
help: consider making this binding mutable
  |
1 | fn countdown(mut n: i32) {
  |              +++
```

真的要在函式裡改，要在參數前面加 `mut`，跟 `let mut` 一樣：`fn countdown(mut n: i32)`。對 `i32` 這種數字來說，改的只是函式裡的那一份，呼叫的地方不受影響。

## 重點整理

- 參數寫成 `名稱: 型別`，用逗號隔開；呼叫時依序給值，個數與型別都要對得上。
- 函式的參數一定要標型別，不會推斷。
- 不能像 Go 那樣把相同型別的參數合併寫成 `a, b int`，每個參數各寫一次型別。
- 參數預設不可變；要在函式裡改，寫成 `mut n: i32`。
