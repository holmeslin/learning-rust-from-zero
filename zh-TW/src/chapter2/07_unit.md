# unit 型別 `()`

## 本集目標

認識 unit 型別 `()`：它代表「沒有有意義的值」，沒寫 `->` 的函式回傳的就是它。

## 正文

### `()` 是什麼

Rust 裡每個運算式都有型別，連「沒有值」也有：unit 型別，寫成一對空的小括號 `()`。這個型別只有一個值，寫出來也是 `()`。

前面幾集的錯誤訊息裡已經出現過它：`` expected `i32`, found `()` ``，意思是「這裡應該有個 `i32`，結果拿到的是『沒有值』」。

### 沒寫 `->` 的函式

不寫 `->` 的函式，回傳型別其實就是 `()`。`fn greet() { ... }` 跟 `fn greet() -> () { ... }` 完全一樣，只是沒人會把 `-> ()` 寫出來。

這種函式沒有值可以交出來，拿它的結果當 `i32` 用，型別就對不上：

```rust,compile_fail=E0308
fn greet() {
    println!("哈囉");
}

fn main() {
    let n: i32 = greet();
    println!("{n}");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:6:18
  |
6 |     let n: i32 = greet();
  |            ---   ^^^^^^^ expected `i32`, found `()`
  |            |
  |            expected due to this
```

`()` 也不能直接用 `{}` 印出來：

```rust,compile_fail=E0277
fn greet() {
    println!("哈囉");
}

fn main() {
    let result = greet();
    println!("{result}");
}
```

編譯錯誤：

```text
error[E0277]: `()` doesn't implement `std::fmt::Display`
 --> src/main.rs:7:15
  |
7 |     println!("{result}");
  |               ^^^^^^^^ `()` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `()`
  = note: in format strings you may be able to use `{:?}` (or `{:#?}` for pretty-print) instead
```

`{}` 只能印「有定義好顯示方式」的型別（第 8 章會解釋這件事），`()` 沒有。第 11 集會學到另一種印法 `{:?}`，到時候就能把它印出來看了。實際寫程式幾乎不會需要印 `()`，知道它存在、看得懂錯誤訊息就夠了。

### 沒有 `else` 的 `if`

另一個會冒出 `()` 的地方：`if` 沒有 `else`，條件不成立時就沒有值可以給，所以整個 `if` 的型別是 `()`。拿它當值用，編譯器就會抱怨：

```rust,compile_fail=E0317
fn main() {
    let score = 75;
    let bonus = if score >= 60 { 10 };
    println!("{bonus}");
}
```

編譯錯誤：

```text
error[E0317]: `if` may be missing an `else` clause
 --> src/main.rs:3:17
  |
3 |     let bonus = if score >= 60 { 10 };
  |                 ^^^^^^^^^^^^^^^^^--^^
  |                 |                |
  |                 |                found here
  |                 expected integer, found `()`
  |
  = note: `if` expressions without `else` evaluate to `()`
  = help: consider adding an `else` block that evaluates to the expected type
```

`if` 的條件成立時給 `10`，不成立時沒有值，兩邊型別不一樣。補上 `else { 0 }` 就好。

## 重點整理

- `()` 是 unit 型別，意思是「沒有有意義的值」，這個型別只有一個值，也寫成 `()`。
- 沒寫 `->` 的函式回傳 `()`；結尾加分號的區塊、沒有 `else` 的 `if` 也是 `()`。
- 錯誤訊息出現 `` found `()` ``，通常是漏了回傳值，或結尾多了一個分號。
- `()` 不能用 `{}` 印出來；`{:?}` 第 11 集介紹。
