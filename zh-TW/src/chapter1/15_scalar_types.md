# 純量型別

## 本集目標

認識 Rust 的四類純量型別：整數、浮點數、`bool` 和 `char`。

## 正文

純量型別（scalar type）是只裝**一個值**的型別。Rust 有四類。

### 整數

整數型別用「`i` 或 `u` 加上位元數」命名。`i` 是有號（signed，可以是負數），`u` 是無號（unsigned，只能是 0 或正數）：

| 位元數 | 有號 | 無號 |
| --- | --- | --- |
| 8 | `i8` | `u8` |
| 16 | `i16` | `u16` |
| 32 | `i32` | `u32` |
| 64 | `i64` | `u64` |
| 128 | `i128` | `u128` |
| 依平台 | `isize` | `usize` |

`isize` 和 `usize` 的大小依電腦而定：在 64 位元的電腦上就是 64 位元。`usize` 專門用來表示「大小」和「索引」，第 2 章用到陣列時就會遇到它。

用 `i32::MIN`、`i32::MAX` 可以印出各型別的最小值與最大值，這個寫法先照抄，第 5 章會解釋：

```rust
fn main() {
    println!("i8: {} 到 {}", i8::MIN, i8::MAX);
    println!("u8: {} 到 {}", u8::MIN, u8::MAX);
    println!("i32: {} 到 {}", i32::MIN, i32::MAX);
    println!("i64: {} 到 {}", i64::MIN, i64::MAX);
}
```

執行結果：

```text
i8: -128 到 127
u8: 0 到 255
i32: -2147483648 到 2147483647
i64: -9223372036854775808 到 9223372036854775807
```

沒有標型別的整數預設是 `i32`。一般計算用 `i32` 就夠了。

### 浮點數

浮點數有 `f32` 和 `f64`，沒標型別時預設是 `f64`。數字要寫小數點才會被當成浮點數（指數、字尾的寫法，附錄一 a 會補充）：

```rust
fn main() {
    let price = 12.5;
    let count = 3.0;
    println!("{}", price * count);
    println!("{}", 7.0 / 2.0);
}
```

執行結果：

```text
37.5
3.5
```

第 7 集說過，想得到 `3.5` 要用浮點數，就是這樣寫。

### `bool` 與 `char`

`bool` 只有 `true` 和 `false`，比較運算的結果就是 `bool`。`char` 是單一個字元，用單引號：`'A'`、`'中'`。`char` 的細節留到第 18 集。

下面把各種型別放在一起，每個都標上型別：

```rust
fn main() {
    let a: i32 = 42;
    let b: u8 = 255;
    let c: usize = 3;
    let d: f64 = 2.5;
    let e: bool = a > 40;
    let f: char = '中';
    println!("{a} {b} {c} {d} {e} {f}");
}
```

執行結果：

```text
42 255 3 2.5 true 中
```

### 跟 Go 比一比

Go 的 `int` 在 64 位元電腦上是 64 位元，Rust 沒有 `int`，沒標型別的整數預設是 `i32`。想要跟著平台的大小，用 `isize` 或 `usize`；想要固定大小，就寫清楚 `i64`、`u8` 這些名字。

另一個差別是常數。Go 的 `var f float64 = 5` 可以過，因為常數 `5` 會自動變成 `float64`。Rust 的整數字面值和浮點數字面值是不同的東西，不會自動轉換：

```rust,compile_fail=E0308
fn main() {
    let x: f64 = 5;
    println!("{x}");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:2:18
  |
2 |     let x: f64 = 5;
  |            ---   ^ expected `f64`, found integer
  |            |
  |            expected due to this
  |
help: use a float literal
  |
2 |     let x: f64 = 5.0;
  |                   ++
```

要寫成 `5.0`。

## 重點整理

- 整數：`i8`～`i128`（有號）、`u8`～`u128`（無號）、`isize`、`usize`；預設是 `i32`。
- `usize` 用來表示大小與索引，第 2 章會用到。
- 浮點數：`f32`、`f64`；預設是 `f64`，數字要有小數點。
- `bool` 是 `true` 或 `false`；`char` 是單一字元，用單引號。
- `i32::MIN`、`i32::MAX` 可以查各整數型別的範圍，先照抄。
- 整數字面值和浮點數字面值不會自動互轉，`let x: f64 = 5;` 是編譯錯誤。
