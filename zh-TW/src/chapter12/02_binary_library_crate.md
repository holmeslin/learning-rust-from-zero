# binary crate 與 library crate

## 本集目標

分清 crate 與 package，認得 `src/main.rs` 與 `src/lib.rs` 兩個起點，並讓同一個專案同時有可執行的程式，以及可以被引用的函式庫。

## 正文

### crate 是編譯的單位

編譯器一次編譯的東西叫 **crate**。第 8 章第 16 集說過，你寫的整個程式是一個 crate；crate 分兩種：

- **binary crate**：編譯成可以執行的程式，一定要有 `fn main`。
- **library crate**：編譯成函式庫，給別人（或自己的 binary）使用，沒有 `main`。

而 **package** 是 `Cargo.toml` 加上它管理的一個或多個 crate。目前為止，你的每個專案都是一個 package，裡面只有一個 binary crate。

### 靠檔名決定是哪一種

Cargo 看 `src/` 底下有哪個檔案，決定 crate 的種類：

| 檔案 | crate | 名稱 |
| --- | --- | --- |
| `src/main.rs` | binary crate 的起點 | 跟 package 的 `name` 一樣 |
| `src/lib.rs` | library crate 的起點 | 跟 package 的 `name` 一樣 |
| `src/bin/report.rs` | 另一個 binary crate | 檔名（這裡是 `report`） |

`cargo new teashop` 建的是 `src/main.rs`；`cargo new --lib teashop` 建的是 `src/lib.rs`，裡面預設放一個 `add` 函式和一小段測試，測試的寫法第 10 集才學。

如果 package 只有 `lib.rs`，就沒有可以執行的東西：

`src/lib.rs`：

```rust,ignore
pub fn total_for(price: u32, cups: u32) -> u32 {
    price * cups
}
```

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
```

這個 `pub` 的意思是「開放給 crate 外面用」，第 4 集會正式講，現在先照抄。對這個 package 執行 `cargo run`：

`cargo run` 的輸出：

```text
error: a bin target must be available for `cargo run`
```

library 沒有 `main`，所以沒東西好跑。它是要給別的 crate 呼叫的。

### 同一個 package 放兩種 crate

在 `src/` 裡再加一個 `main.rs`，一個 package 就同時有 library 與 binary：

```text
teashop/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── main.rs
```

`lib.rs` 不用改。`main.rs` 是另一個 crate，**把 library 當成外部的 crate 來用**，用 package 的名字 `teashop` 當路徑的開頭：

`src/main.rs`：

```rust,ignore
fn main() {
    let total = teashop::total_for(30, 2);
    println!("共 {total} 元");
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
共 60 元
```

`teashop::total_for` 的 `teashop` 不是模組，是 `lib.rs` 那個 crate 的名字。所以 `lib.rs` 裡的東西要加 `pub`，`main.rs` 才叫得到，原因第 4 集說明。

這是 Rust 專案常見的分法：**把邏輯放在 `lib.rs`，`main.rs` 只負責讀輸入、印結果**。好處有兩個：library 可以被測試（第 12 集的整合測試只能測 library），也可以被別的專案引用。

### 跟 Go 比一比

Go 用檔案開頭的 `package main` 決定是不是可執行檔，同一個資料夾裡只能有一種 package。Rust 看的是檔名：`main.rs` 與 `lib.rs`，而且可以並存。

名詞也容易混淆：Go 的 `module`（`go.mod`）比較接近 Rust 的 package，Go 的 `package` 比較接近 Rust 的模組（`mod`），Rust 的 crate 則是「一次編譯的東西」，Go 沒有直接對應的名詞。

## 重點整理

- crate 是編譯的單位：binary crate 有 `fn main`、編譯成可執行檔；library crate 沒有 `main`、給別的 crate 使用。
- package 是 `Cargo.toml` 加上底下的 crate；`src/main.rs` 是 binary 的起點，`src/lib.rs` 是 library 的起點，`src/bin/` 可以放更多 binary。
- 只有 library 的 package 不能 `cargo run`。
- 同一個 package 可以同時有 `lib.rs` 與 `main.rs`；`main.rs` 用 package 的名字（`teashop::...`）引用 library，library 裡要加 `pub` 才被看得到。
- 常見分法：邏輯放 `lib.rs`，`main.rs` 只負責輸入輸出。
