# features

## 本集目標

看懂 `Cargo.toml` 的 `[features]`：替 library 加上可以開關的功能，並知道怎麼替相依的 crate 開啟 feature。

## 正文

### feature 是可以開關的功能

有些功能不是每個使用者都要，例如會員折扣。與其拆成兩個 library，可以把它做成一個 **feature**：預設不編譯，要用的人自己打開。

先在 `Cargo.toml` 的 `[features]` 宣告一個名字：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[features]
member = []

[dependencies]
```

`member = []` 的 `[]` 是這個 feature 要連帶打開的其他 feature，這裡先留空。再來，程式裡用 `#[cfg(feature = "member")]` 標出屬於這個 feature 的東西：

`src/lib.rs`：

```rust,ignore
pub fn price(drink: &str) -> u32 {
    if drink == "紅茶" { 30 } else { 45 }
}

#[cfg(feature = "member")]
pub fn member_price(drink: &str) -> u32 {
    price(drink) * 9 / 10
}
```

`#[cfg(feature = "名字")]` 先照抄，意思是「只有打開了 `名字` 這個 feature，才編譯後面那個項目」，第 10 集的 `#[cfg(test)]` 是同一種寫法。`cfg` 第 16 章第 10 集才正式教。

### 沒打開時，東西根本不存在

`main.rs` 呼叫 `member_price`，但沒有打開 feature：

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::price("紅茶"));
    println!("{}", teashop::member_price("紅茶"));
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
error[E0425]: cannot find function `member_price` in crate `teashop`
 --> src/main.rs:3:29
  |
3 |     println!("{}", teashop::member_price("紅茶"));
  |                             ^^^^^^^^^^^^ not found in `teashop`
  |
note: found an item that was configured out
 --> src/lib.rs:6:8
  |
5 | #[cfg(feature = "member")]
  |       ------------------ the item is gated behind the `member` feature
6 | pub fn member_price(drink: &str) -> u32 {
  |        ^^^^^^^^^^^^
```

對 `main.rs` 來說，沒打開 feature 時，`member_price` 就像沒寫過一樣，所以是 E0425（找不到函式）；`note` 很貼心地告訴你它被 `cfg` 排除了，而且是因為 `member` 這個 feature 沒打開。

### 打開 feature

用 `--features` 在指令上打開：

`cargo run --features member` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
30
27
```

兩個函式都能用了。也可以讓某些 feature **預設就打開**，在 `[features]` 裡寫 `default`：

```toml
[features]
default = ["member"]
member = []
```

使用者想關掉預設的，加 `--no-default-features`。feature 只能「加功能」：同一個程式裡，如果有兩個地方各自要求不同的 feature，cargo 會取聯集全部打開，所以 feature 不該拿來做「二選一」的設計。

### 替相依的 crate 開 feature

這個機制更常用在別人的 crate 上。許多 crate 把進階功能做成 feature，加相依套件時用大括號寫出要開哪些。例如第二部會用到的 `clap`，要打開 `derive` 才有後面介紹的寫法：

```toml
[dependencies]
clap = { version = "4.6.7", features = ["derive"] }
```

用 `cargo add clap --features derive` 也會寫出一樣的內容。反過來，`default-features = false` 會把相依套件預設打開的 feature 全部關掉，只留下你列出的，用來縮小編譯的範圍。

## 重點整理

- feature 是 `Cargo.toml` 的 `[features]` 裡宣告的名字，在程式裡用 `#[cfg(feature = "名字")]` 標出屬於它的項目（先照抄，`cfg` 第 16 章第 10 集教）。
- 沒打開的 feature，標了它的項目根本不會編譯，用到它是 E0425 之類的「找不到」。
- 用 `--features 名字` 打開；`default = [...]` 讓某些 feature 預設打開，`--no-default-features` 關掉。
- feature 只能累加，不要設計成二選一。
- 替相依套件開 feature 寫 `crate = { version = "...", features = [...] }`；`default-features = false` 關掉它預設的 feature。
