# 索引、`usize` 與越界 panic

## 本集目標

用索引讀寫陣列的元素，知道索引的型別一定是 `usize`，以及索引超出範圍時會發生什麼事。

## 正文

### 用索引讀寫

跟 Go 一樣，陣列用 `陣列[索引]` 取值，索引從 0 開始。要改某個位置的值，陣列要用 `let mut`：

```rust
fn main() {
    let mut scores = [85, 92, 78];
    println!("{}", scores[0]);
    scores[1] = 95;
    println!("{scores:?}");
}
```

執行結果：

```text
85
[85, 95, 78]
```

### 索引一定是 `usize`

第 1 章第 15 集說過，`usize` 是專門用來表示「大小」和「索引」的整數型別。陣列的索引**只能是 `usize`**，拿 `i32` 來當索引會被擋下來：

```rust,compile_fail=E0277
fn main() {
    let scores = [85, 92, 78];
    let i: i32 = 1;
    println!("{}", scores[i]);
}
```

編譯錯誤：

```text
error[E0277]: the type `[{integer}]` cannot be indexed by `i32`
 --> src/main.rs:4:27
  |
4 |     println!("{}", scores[i]);
  |                           ^ slice indices are of type `usize` or ranges of `usize`
  |
  = help: the trait `SliceIndex<[{integer}]>` is not implemented for `i32`
```

訊息後面還有幾行補充（列出哪些型別能當索引），先不用管。要嘛一開始就把變數宣告成 `usize`，要嘛用第 1 章第 17 集的 `as` 轉換成 `scores[i as usize]`。走訪範圍時不用煩惱：`for k in 0..3` 的 `k` 拿去當索引，編譯器就會推斷它是 `usize`。

### 索引超出範圍

索引超過陣列的範圍，不會讀到亂七八糟的記憶體，而是程式 panic。如果索引是寫死的數字，編譯器看得出來，連程式都不讓你編譯：

```rust,compile_fail
fn main() {
    let scores = [85, 92, 78];
    println!("{}", scores[5]);
}
```

編譯錯誤：

```text
error: this operation will panic at runtime
 --> src/main.rs:3:20
  |
3 |     println!("{}", scores[5]);
  |                    ^^^^^^^^^ index out of bounds: the length is 3 but the index is 5
  |
  = note: `#[deny(unconditional_panic)]` on by default
```

索引如果要到執行時才會決定，例如使用者輸入的數字，編譯器就看不出來，要等到執行才會 panic：

```rust,stdin=5,noplayground,exit=101
fn main() {
    let scores = [85, 92, 78];
    println!("要看第幾筆（從 0 開始）？");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    let line = line.trim();
    let i: usize = line.parse().expect("請輸入整數");
    println!("{}", scores[i]);
}
```

輸入 `5` 的執行結果（某一次）：

```text
要看第幾筆（從 0 開始）？

thread 'main' (155803750) panicked at src/main.rs:8:20:
index out of bounds: the len is 3 but the index is 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

訊息說得很清楚：陣列長度是 3，卻要取索引 5。panic 的結束碼是 101。`thread 'main'` 後面括號裡的數字每次執行都不一樣，其他都相同。

### `usize` 沒有負數

`usize` 是無號整數，不會是負的，往下減過 0 就是溢位。第 1 章第 16 集提過，陣列的走訪很容易遇到它。下面的程式從最後一筆往前印，印完第 0 筆之後再減 1：

```rust,exit=101
fn main() {
    let scores = [85, 92, 78];
    let mut i: usize = 2;
    loop {
        println!("{}", scores[i]);
        i -= 1;
    }
}
```

執行結果（某一次）：

```text
78
92
85

thread 'main' (155803843) panicked at src/main.rs:6:9:
attempt to subtract with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```


### 跟 Go 比一比

Go 的陣列和 slice 越界也會 panic（訊息是 `index out of range [5] with length 3`，結束碼是 2），這點兩邊一樣。不同的是索引的型別：

```go
scores := [3]int{85, 92, 78}
i := 1 // int
fmt.Println(scores[i])
```

Go 的索引可以是任何整數型別，常用的 `int` 是有號的，所以會有「索引變成負數」的情況。Rust 的索引只收 `usize`，用 `i32` 要自己轉；而 `usize` 不可能是負的，往下減過 0 的當下就會 panic。

## 重點整理

- 用 `陣列[索引]` 讀寫，索引從 0 開始；要改值，陣列要用 `let mut`。
- 索引一定是 `usize`；`i32` 要用 `as usize` 轉換，否則是 `E0277`。
- 索引超出範圍：寫死的數字是編譯錯誤，執行時才算出來的會 panic（結束碼 101）。
- `usize` 沒有負數，往下減過 0 會溢位 panic。
- Go 的索引可以是有號的 `int`；Rust 只收 `usize`。
