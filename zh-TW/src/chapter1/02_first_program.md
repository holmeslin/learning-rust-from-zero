# 第一個程式

## 本集目標

用 `cargo new` 建立專案、`cargo run` 執行，寫出第一支會印出文字的 Rust 程式，並學會寫註解。

## 正文

### 建立專案

打開終端機，輸入：

```bash
cargo new hello
cd hello
cargo run
```

`cargo new hello` 會建立一個叫 `hello` 的資料夾，裡面是這樣：

```text
hello/
├── Cargo.toml
└── src/
    └── main.rs
```

`Cargo.toml` 是專案設定檔，角色像 Go 的 `go.mod`：

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`edition = "2024"` 是 Rust 語言的版本代號，先不用動它。`[dependencies]` 是放外部套件的地方，現在是空的。Cargo 的細節留到第 12 章。

### 第一個程式

`cargo new` 已經替你寫好 `src/main.rs`，內容就是下面這支：

```rust
fn main() {
    println!("Hello, world!");
}
```

執行結果：

```text
Hello, world!
```

`cargo run` 會先編譯再執行。在程式輸出之前，終端機還會多出 `Compiling`、`Finished`、`Running` 這幾行訊息（程式沒改過、不用重新編譯時，就不會有 `Compiling`），那是 cargo 在報告進度。本書的「執行結果」只列程式自己印出來的部分。

### 先照抄的骨架

`fn main() { ... }` 是每支程式都要有的骨架，請先**照抄**。它是函式的寫法，第 2 章才會解釋；你只要知道，程式會從 `main` 的大括號裡開始執行，這點跟 Go 一樣。

大括號裡的 `println!("Hello, world!");` 才是要電腦做的事：印出一行文字並換行，作用等於 Go 的 `fmt.Println`。`println!` 後面的 `!` 表示它是巨集，現在先把它當成一個特別的函式就好，第 16 章第 1 集會正式介紹。

兩個小地方跟 Go 不同：每一行陳述式的結尾要加分號 `;`；`println!` 不用 `import` 就能直接使用。

### 註解

```rust
fn main() {
    // 單行註解，從 // 到行尾
    println!("Hello"); // 也可以寫在程式碼後面

    /* 區塊註解
    可以跨好幾行 */
    println!("World");
}
```

執行結果：

```text
Hello
World
```

這跟 Go 一樣。唯一的小差別是 Rust 的區塊註解可以巢狀，`/* 外 /* 內 */ 外 */` 是合法的，所以用它把一大段含註解的程式碼暫時註解掉不會出事。文件註解 `///` 留到第 12 章。

### 跟 Go 比一比

同樣印出 `Hello, world!`，Go 要寫：

```go
package main

import "fmt"

func main() {
	fmt.Println("Hello, world!")
}
```

Rust 沒有 `package` 和 `import` 這兩行，但多了分號；縮排用 4 個空格，不是 Go 的 tab（本書的範例都是 `rustfmt` 排好的樣子）。

## 重點整理

- `cargo new <名稱>` 建立專案，`cargo run` 編譯並執行。
- `Cargo.toml` 是專案設定檔，程式碼寫在 `src/main.rs`。
- 程式骨架 `fn main() { ... }` 先照抄；程式從 `main` 的大括號裡開始執行。
- `println!("文字")` 印出一行文字並換行；每行陳述式結尾要加 `;`。
- 註解用 `//` 和 `/* */`。
