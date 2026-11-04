# 運算子：跟 Go 差在哪

## 本集目標

認得 Rust 的運算子，並知道它跟 Go 唯一會讓人踩到的差別：沒有 `++` 和 `--`。

## 正文

### 算術運算子

`+`、`-`、`*`、`/`、`%` 的行為跟 Go 一樣。兩個整數相除，結果還是整數，小數會直接捨去（往 0 的方向）；`%` 的結果跟被除數同號：

```rust
fn main() {
    println!("{}", 7 + 2);
    println!("{}", 7 / 2);
    println!("{}", 7 % 2);
    println!("{}", -7 / 2);
    println!("{}", -7 % 2);
}
```

執行結果：

```text
9
3
1
-3
-1
```

想得到 `3.5` 這種帶小數的結果，要用浮點數，第 15 集會介紹。

### 複合賦值

`+=`、`-=`、`*=`、`/=`、`%=` 也跟 Go 一樣。前提是變數要用 `let mut` 建立：

```rust
fn main() {
    let mut n = 10;
    n += 5;
    println!("{n}");
    n *= 2;
    println!("{n}");
    n %= 7;
    println!("{n}");
}
```

執行結果：

```text
15
30
2
```

第 4 集的 `score = score + 10` 就可以寫成 `score += 10`。

### 比較與邏輯運算子

比較用 `==`、`!=`、`<`、`>`、`<=`、`>=`，邏輯用 `&&`、`||`、`!`，全部跟 Go 一樣，結果是 `true` 或 `false`：

```rust
fn main() {
    let a = 5;
    let b = 8;
    println!("{}", a < b);
    println!("{}", a == b);
    println!("{}", a != b && b > 6);
}
```

執行結果：

```text
true
false
true
```

### 不同型別不能混著運算

兩邊的型別必須一樣，Rust 不會替你悄悄轉換。`i32` 和 `i64` 拿來比較，就是編譯錯誤：

```rust,compile_fail=E0308
fn main() {
    let a: i32 = 5;
    let b: i64 = 5;
    if a == b {
        println!("same");
    }
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:4:13
  |
4 |     if a == b {
  |        -    ^ expected `i32`, found `i64`
  |        |
  |        expected because this is `i32`
  |
help: you can convert `a` from `i32` to `i64`, matching the type of `b`
  |
4 |     if i64::from(a) == b {
  |        ++++++++++ +
```

這點 Go 也一樣，不同整數型別不能直接運算。轉換的寫法第 17 集會教；`help` 裡建議的 `i64::from` 要到第 8 章才會解釋，先不用管它。

### 跟 Go 比一比

Go 有 `i++` 和 `i--`，Rust **沒有**。照 Go 的習慣寫，會得到編譯錯誤：

```rust,compile_fail
fn main() {
    let mut count = 0;
    count++;
    println!("{count}");
}
```

編譯錯誤：

```text
error: Rust has no postfix increment operator
 --> src/main.rs:3:10
  |
3 |     count++;
  |          ^^ not a valid postfix operator
  |
help: use `+= 1` instead
  |
3 -     count++;
3 +     count += 1;
  |
```

要加 1 就寫 `count += 1`，減 1 就寫 `count -= 1`。

## 重點整理

- `+`、`-`、`*`、`/`、`%` 跟 Go 一樣；整數相除會捨去小數。
- 複合賦值 `+=`、`-=`、`*=`、`/=`、`%=` 需要 `let mut` 的變數。
- 比較與邏輯運算子（`==`、`!=`、`<`、`&&`、`||`、`!` 等）跟 Go 一樣。
- 不同型別不能混著運算，也不會自動轉換。
- Rust 沒有 `++` 和 `--`，用 `+= 1` 和 `-= 1`。
