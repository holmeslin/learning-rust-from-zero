# 解構與 `_`

## 本集目標

用 `let (a, b) = ...` 一次把 tuple 拆成幾個變數，並用 `_` 略過不需要的值。

## 正文

### 解構 tuple

上一集得用 `result.0`、`result.1` 取值。更順手的寫法，是把 `let` 左邊寫成跟 tuple 同樣的形狀，裡面放變數名稱：

```rust
fn main() {
    let point = (3, 4);
    let (x, y) = point;
    println!("x = {x}, y = {y}");
}
```

執行結果：

```text
x = 3, y = 4
```

這叫**解構**（destructuring）：tuple 被拆開，第一個值放進 `x`，第二個值放進 `y`。`let` 左邊的 `(x, y)` 叫做**模式**（pattern），第 6 章會學到更多種模式。

拿來接函式回傳的 tuple 最方便，效果跟 Go 的 `q, r := divide(17, 5)` 一樣：

```rust
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    let (quotient, remainder) = divide(17, 5);
    println!("商 {quotient}，餘 {remainder}");
}
```

執行結果：

```text
商 3，餘 2
```

每個名字都可以各自決定要不要加 `mut`，例如 `let (mut x, y) = point;`。

### 用 `_` 略過不要的值

只想要其中一個值時，不需要的位置寫 `_`：

```rust
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    let (quotient, _) = divide(17, 5);
    println!("商 {quotient}");
}
```

執行結果：

```text
商 3
```

`_` 的意思是「這個位置有值，但我不要」。第 1 章第 19 集預告過單獨一個 `_`，就是這個用法。迴圈只想重複幾次、不在乎第幾圈時，也可以用它：

```rust
fn main() {
    for _ in 0..3 {
        println!("哈囉");
    }
}
```

執行結果：

```text
哈囉
哈囉
哈囉
```

第 1 章寫的 `for _i in 0..3` 也行，兩者的差別是：`_i` 是一個真的變數，只是名字前面的底線讓編譯器不要警告；單獨的 `_` 根本**不建立變數**，所以讀不到它：

```rust,compile_fail
fn main() {
    let (quotient, _) = (17, 5);
    println!("{quotient} {_}");
}
```

編譯錯誤：

```text
error: invalid format string: invalid argument name `_`
 --> src/main.rs:3:27
  |
3 |     println!("{quotient} {_}");
  |                           ^ invalid argument name in format string
  |
  = note: argument name cannot be a single underscore
```

這點跟 Go 的 `_` 相同：丟掉的值就是丟掉了。

### 形狀要對得上

左邊的模式和右邊的 tuple，元素個數必須一樣，不然就是編譯錯誤：

```rust,compile_fail=E0308
fn main() {
    let (a, b, c) = (1, 2);
    println!("{a} {b} {c}");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     let (a, b, c) = (1, 2);
  |         ^^^^^^^^^   ------ this expression has type `({integer}, {integer})`
  |         |
  |         expected a tuple with 2 elements, found one with 3 elements
  |
  = note: expected tuple `({integer}, {integer})`
             found tuple `(_, _, _)`
```

三個值只想要前兩個時，也要寫滿三個位置，不要的那個用 `_` 補上，例如 `let (a, b, _) = ...`。

## 重點整理

- `let (x, y) = tuple;` 把 tuple 拆成多個變數，叫解構；左邊的 `(x, y)` 是模式。
- 解構最適合接函式回傳的 tuple：`let (q, r) = divide(17, 5);`。
- `_` 略過不要的值，也不會有未使用警告；單獨的 `_` 不建立變數，讀不到。
- `_x` 是真的變數（只是不警告），`_` 不是。
- 模式的元素個數要跟 tuple 一樣多，否則是編譯錯誤。
