# 巨集是什麼（揭曉 `println!`）

## 本集目標

知道名字後面的 `!` 代表巨集：巨集在編譯時期把一段程式碼換成另一段程式碼，所以 `println!` 才能收任意個引數、還能在編譯時期檢查格式字串。也會分辨 Rust 的兩大類巨集。

## 正文

### 名字後面的 `!`

從第 1 章起，我們就一直在寫 `println!`，後來又遇到 `vec!`、`format!`、`matches!`、`assert_eq!`、`write!`。它們名字後面都有 `!`，這個 `!` 表示：**這是巨集（macro），不是函式。**

巨集的原理，一句話就說完：它收到一段程式碼，吐出另一段程式碼。編譯器看到巨集呼叫，會先把它**展開**（expand）成真正的程式碼，再照平常的流程編譯。展開發生在型別檢查之前，所以巨集拿到的是「寫在原始檔裡的文字」，不是算好的值：

```rust
fn main() {
    let total = 3;
    println!("{}", stringify!(total + 1));
    println!("{}", total + 1);
}
```

執行結果：

```text
total + 1
4
```

`stringify!` 把收到的程式碼原封不動變成字串，所以印出 `total + 1`，連 `total` 是多少都沒看。普通函式做不到：函式只會拿到算好的 `4`。

### 為什麼 `println!` 非得是巨集

函式做不到的事，`println!` 剛好都需要：

- **引數個數不固定。** 函式的簽名把引數個數寫死了，多傳一個是 E0061。Rust 的函式沒有「可變引數」，`println!("{name}")`、`println!("{} 賣 {}", a, b)` 要收不同數量的引數，只能靠巨集。
- **編譯時期檢查格式字串。** 格式字串裡的 `{}` 跟引數個數對不上，編譯就失敗，程式根本跑不起來：

```rust,compile_fail
fn main() {
    println!("{} 賣 {} 元", "紅茶");
}
```

編譯錯誤：

```text
error: 2 positional arguments in format string, but there is 1 argument
 --> src/main.rs:2:15
  |
2 |     println!("{} 賣 {} 元", "紅茶");
  |               ^^    ^^      ------
```

這個錯誤沒有錯誤碼，所以 `compile_fail` 後面不用寫碼。

- **看得到你寫的變數名稱。** `{name}` 是巨集在編譯時期讀到文字 `name`，把它當成變數去找；函式不可能知道呼叫端有哪些變數。沒有這個變數，錯誤就跟平常寫 `name` 一樣是 E0425（找不到）。

標準函式庫的 `println!` 其實是用下一集要教的 `macro_rules!` 寫成的，格式字串由編譯器內建的 `format_args!` 檢查。`format!` 是同一套東西，只是結果回傳 `String`，不印出來。`write!` 也是：

```rust
use std::fmt::Write;

fn main() {
    let mut out = String::new();
    write!(out, "{} 元", 30).expect("寫入失敗");
    println!("{out}");
}
```

執行結果：

```text
30 元
```

`write!(目的地, 格式字串, 引數…)`：格式字串的規則跟 `println!` 一樣，只多了第一個引數，指定寫到哪裡。第 8 章第 11 集的 `Display` 骨架裡，`write!(f, ...)` 就是寫進 `f`。這裡的 `String` 也能當目的地，前提是先 `use std::fmt::Write;`。`write!` 回傳 `Result`（第 9 章），所以這裡接 `expect`。

### 呼叫巨集的括號

`println!("hi")`、`println!["hi"]`、`println!{"hi"}` 三種括號編譯器都接受，意思一樣。慣例是 `vec![...]` 用中括號，其他用小括號；clippy 的 `nonstandard_macro_braces` 會提醒你別亂換。

### 巨集有哪幾種

- **宣告式巨集**（declarative macro）：用 `macro_rules!` 寫的，長得像 `match`：比對收到的程式碼，換成對應的程式碼。`println!`、`vec!`、`matches!`、`assert_eq!` 都是這一種。第 2、3 集教怎麼寫。
- **程序巨集**（procedural macro）：用一般的 Rust 程式寫成，輸入輸出都是「一串程式碼」。有三種長相：`#[derive(Debug)]` 這種 derive、`#[tokio::main]` 這種屬性、`名字!(...)` 這種像函式的。第 4 集介紹。

巨集不是萬靈丹。能用函式解決的事就用函式，函式有清楚的簽名和錯誤訊息；只有函式做不到（引數個數不固定、要在編譯時期檢查、要產生重複的程式碼）時，才輪到巨集。

### 跟 Go 比一比

Go 的 `fmt.Println` 是普通函式，靠 `...any`（任意個 `any`）收不固定個數的引數。格式字串錯了，編譯照樣成功，要到執行時才看到怪輸出：

```go
fmt.Printf("%d 賣 %d 元\n", "紅茶")
```

輸出是 `%!d(string=紅茶) 賣 %!d(MISSING) 元`。`go vet` 看得出來這個錯誤，但它是另外一個工具，不跑就沒有。Go 沒有巨集，要產生重複的程式碼只能靠 `go generate` 這類外部工具，產生的檔案也得自己管理；Rust 的巨集由編譯器內建支援，不用另外跑東西。

## 重點整理

- 名字後面有 `!` 的是巨集。巨集收到程式碼、吐出程式碼，由編譯器在型別檢查之前展開，所以拿到的是程式碼的文字，不是算好的值（`stringify!(total + 1)` 得到 `"total + 1"`）。
- `println!` 要收不固定個數的引數、在編譯時期檢查格式字串、讀得到 `{name}` 這種變數名稱，這三件事函式都做不到，所以是巨集。`format!`、`write!` 一樣。
- `write!(目的地, 格式字串, 引數…)` 的格式字串跟 `println!` 相同，目的地可以是 `Formatter` 或 `String`（要 `use std::fmt::Write;`）。
- 巨集分宣告式（`macro_rules!`，第 2、3 集）與程序巨集（derive、屬性、像函式的，第 4 集）；括號 `()`、`[]`、`{}` 都能呼叫，慣例只有 `vec![]` 用中括號。
- Go 的 `fmt` 是 `...any` 的普通函式，格式錯了要靠 `go vet`；Rust 在編譯時期就擋下。
