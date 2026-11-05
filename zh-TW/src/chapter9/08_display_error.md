# 實作 `Display` 與 `Error`

## 本集目標

替自己的錯誤型別實作 `Display`（給使用者看的說明）與 `std::error::Error`（宣告它是錯誤），讓它用起來跟標準函式庫的錯誤一樣。

## 正文

### `Display`：錯誤要說人話

上一集的錯誤只能靠 `match` 逐一分辨，印出來也只有 `{:?}` 的 `Unknown("XL")`。替它實作第 8 章第 11 集的 `Display`，就有了給使用者看的訊息，`{}` 能直接印；骨架照抄，enum 搭配 `match self`。接著再加一行 `impl std::error::Error`：

```rust
#[derive(Debug)]
enum SizeError {
    Empty,
    Unknown(String),
}

impl std::fmt::Display for SizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SizeError::Empty => write!(f, "還沒選杯型"),
            SizeError::Unknown(text) => write!(f, "沒有 {text} 這種杯型"),
        }
    }
}

impl std::error::Error for SizeError {}

fn report(e: &dyn std::error::Error) {
    println!("出錯了：{e}");
}

fn main() {
    let empty = SizeError::Empty;
    let xl = SizeError::Unknown(String::from("XL"));
    println!("{empty}");
    println!("{xl}");
    println!("{xl:?}");
    report(&empty);
    if let Err(e) = "abc".parse::<u32>() {
        report(&e);
    }
}
```

執行結果：

```text
還沒選杯型
沒有 XL 這種杯型
Unknown("XL")
出錯了：還沒選杯型
出錯了：invalid digit found in string
```

- `{empty}`、`{xl}` 用的是 `Display`：給使用者看的說明。`{xl:?}` 用的是 `Debug`：給寫程式的人看的，格式固定。
- 第 8 章第 11 集說 `fmt` 回傳的 `Result` 第 9 章才解釋：`std::fmt::Result` 其實就是 `Result<(), std::fmt::Error>`，寫入成功是 `Ok(())`（第 2 章第 7 集的 `()`），失敗是 `Err(std::fmt::Error)`。`write!` 回傳的正是這個，所以最後一行 `write!` 不加分號，直接當回傳值。
- 上一集每個 `match` 分支各寫一次的訊息，現在集中在 `Display` 一處，呼叫的人不用再重複寫。

### `Error`：宣告「我是錯誤」

`impl std::error::Error for SizeError {}` 的大括號是空的：`Error` 的方法都有預設實作（第 8 章第 3 集），所以什麼都不用寫。它只做一件事，宣告這個型別是錯誤。標準函式庫的錯誤，像 `ParseIntError`，也都實作了 `Error`。

這有什麼用？`report` 的參數是第 8 章第 15 集的 `&dyn std::error::Error`：只要實作了 `Error`，不管是自己的 `SizeError` 還是標準函式庫的 `ParseIntError`，都能傳進同一個函式。第 10、11 集會看到，`?` 和 `main` 也靠它收下各式各樣的錯誤。

`Error` 還有一個選用的 `source` 方法，預設是沒有。它用來回報成因：這個錯誤是被哪個底層錯誤引起的。現在用不到。

### `Error` 要求先有 `Debug` 和 `Display`

一個 trait 可以要求實作它的型別，先實作別的 trait。`Error` 要求的是 `Debug` 和 `Display`。少了 `Display` 的話：

```rust,compile_fail=E0277
#[derive(Debug)]
struct SoldOut;

impl std::error::Error for SoldOut {}

fn main() {
    println!("{:?}", SoldOut);
}
```

編譯錯誤：

```text
error[E0277]: `SoldOut` doesn't implement `std::fmt::Display`
 --> src/main.rs:4:28
  |
4 | impl std::error::Error for SoldOut {}
  |                            ^^^^^^^ unsatisfied trait bound
  |
help: the trait `std::fmt::Display` is not implemented for `SoldOut`
 --> src/main.rs:2:1
  |
2 | struct SoldOut;
  | ^^^^^^^^^^^^^^
note: required by a bound in `std::error::Error`
```

`` `SoldOut` doesn't implement `std::fmt::Display` ``：箭頭指著 `impl std::error::Error for SoldOut`，最後一行 `note` 說 `Error` 有這個要求。少了 `Debug` 也是 E0277，所以自己的錯誤型別，三件事一起做：`#[derive(Debug)]`、`impl Display`、`impl Error`。

## 重點整理

- 錯誤型別要 `#[derive(Debug)]`，再自己 `impl std::fmt::Display`（給使用者看的說明，骨架照抄）。
- `impl std::error::Error for 型別 {}`：大括號可以是空的，只是宣告它是錯誤；標準函式庫的錯誤也都實作了它。
- 實作了 `Error` 的型別，都能當成 `&dyn std::error::Error` 傳遞。
- `Error` 要求型別先有 `Debug` 和 `Display`，缺一個是 E0277。
- `Error` 的 `source` 方法選用，預設沒有。
