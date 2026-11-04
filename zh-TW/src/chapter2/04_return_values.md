# 回傳值與最後一個運算式

## 本集目標

讓函式回傳值：用 `->` 寫出回傳型別，並用最後一個沒有分號的運算式當作回傳值。

## 正文

### `->` 與回傳型別

函式要回傳值，就在參數列後面加 `->` 和回傳型別。函式裡**最後一個運算式**如果沒有分號，它的值就是整個函式的回傳值，不用寫 `return`：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(3, 4);
    println!("{sum}");
}
```

執行結果：

```text
7
```

`a + b` 後面沒有分號，所以它的值 `7` 就被交出去。這跟第 1 章第 8 集的 `if` 區塊是同一個規則：區塊的最後一個沒有分號的運算式，就是這個區塊的值。

回傳型別可以是任何型別：

```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn main() {
    println!("{}", is_even(10));
    println!("{}", average(3.0, 4.5));
}
```

執行結果：

```text
true
3.75
```

最後一個運算式也可以是 `if`，兩個分支的型別要一樣：

```rust
fn bigger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", bigger(8, 5));
}
```

執行結果：

```text
8
```

函式不寫 `->`，就表示不回傳有意義的值，第 7 集會說明它回傳的是什麼。

### 結尾多一個分號就錯了

最容易出的錯，是習慣性在最後一行加分號：

```rust,compile_fail=E0308
fn add(a: i32, b: i32) -> i32 {
    a + b;
}

fn main() {
    println!("{}", add(3, 4));
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:1:27
  |
1 | fn add(a: i32, b: i32) -> i32 {
  |    ---                    ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     a + b;
  |          - help: remove this semicolon to return this value
```

編譯器說：函式要回傳 `i32`，你卻什麼值都沒交出去（`()` 就是「沒有值」，第 7 集解釋）。`help` 直接建議：把分號拿掉。為什麼一個分號就能讓值消失，下一集會講。

### 跟 Go 比一比

Go 的回傳一定要寫 `return`：

```go
func add(a, b int) int {
    return a + b
}
```

Rust 反過來：結尾的運算式不寫 `return`、不加分號，值就交出去。`return` 是留給「提早離開」用的，第 6 集再介紹。剛從 Go 轉過來，手會習慣在最後一行多打分號，編譯器會用上面那個錯誤提醒你。

## 重點整理

- 回傳型別寫在參數列後面：`fn add(a: i32, b: i32) -> i32`。
- 函式最後一個沒有分號的運算式，就是回傳值，不用 `return`。
- 最後一行多打分號，值就不見了，會得到 `E0308`（期待 `i32`，得到 `()`）。
- 不寫 `->` 的函式不回傳有意義的值，第 7 集解釋。
- Go 一定要 `return`；Rust 的 `return` 留給提早離開，第 6 集介紹。
