# `Cargo.toml`

## 本集目標

讀懂 `Cargo.toml` 的 `[package]` 與 `[dependencies]`，並分清 `cargo run`、`cargo build`、`cargo check` 各做什麼，以及編譯結果放在哪裡。

## 正文

### 專案長什麼樣子

第 1 章第 2 集用 `cargo new` 建過專案。這一章要認真看看裡面的東西，先另外建一個：

```bash
cargo new teashop
```

`cargo new teashop` 的輸出：

```text
    Creating binary (application) `teashop` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

它建好的資料夾是這樣（還有一個 `.git` 資料夾，先不管）：

```text
teashop/
├── .gitignore
├── Cargo.toml
└── src/
    └── main.rs
```

`.gitignore` 裡只有一行 `/target`，意思是編譯產生的東西不要放進版本控制。`Cargo.toml` 是專案設定檔，角色跟 Go 的 `go.mod` 很像。

### `[package]`：這個專案是誰

`Cargo.toml` 用 TOML 格式，一個 `[段落]` 底下放好幾行 `鍵 = 值`。先看 `[package]`：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `name`：專案名稱。編譯出來的程式也叫這個名字（`target/debug/teashop`）。名稱裡可以有連字號，像 `tea-shop`，但程式碼裡引用時要換成底線 `tea_shop`。
- `version`：版本號碼，像 `0.1.0`，三段數字依序是主版號、次版號、修訂號。自己寫程式用不太到，要發布給別人用，或別的專案要用到它的時候才有意義。
- `edition`：Rust 語言的「版本代號」。語言偶爾會有不能向下相容的改動，這類改動只套用在標了新 edition 的專案上，舊專案不受影響。第 6 章第 15 集的 let chains 就只有 `edition = "2024"` 才能用。本書用 2024。不同 edition 的專案可以放在一起用，各編各的。

`[package]` 還可以寫作者、說明、授權等欄位，用到再查[官方文件](https://doc.rust-lang.org/cargo/reference/manifest.html)就好。`[dependencies]` 是放外部套件的地方，現在是空的，第 14 集再用。

### 編譯與執行

`src/main.rs` 先放一支簡單的程式：

`src/main.rs`：

```rust
fn main() {
    println!("紅茶 30 元");
}
```

`cargo run` 會先編譯、再執行：

`cargo run` 的輸出：

```text
   Compiling teashop v0.1.0 (/home/you/teashop)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/teashop`
紅茶 30 元
```

前三行是 cargo 的進度報告（目錄路徑與秒數每個人不同），最後一行才是程式印出來的。`dev` 是預設的 **profile**：編譯快，但不做最佳化，還帶著除錯資訊，開發時用最合適。要交付成品、讓程式跑得快，加 `--release`，產出放在 `target/release/`，編譯會久很多。

第二次執行時，程式沒有改，所以沒有 `Compiling` 那行：

`cargo run` 的輸出：

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/teashop`
紅茶 30 元
```

### `cargo build` 與 `cargo check`

- `cargo build`：只編譯，不執行。結果放在 `target/debug/teashop`。
- `cargo check`：只檢查程式能不能編譯，不產生可執行檔，所以比 `cargo build` 快。邊寫邊確認有沒有錯，用它最省時間。

`cargo check` 的輸出：

```text
    Checking teashop v0.1.0 (/home/you/teashop)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
```

`target/` 資料夾就是 cargo 放所有編譯產物的地方（中間檔案、可執行檔都在裡面），體積可能很大，所以 `.gitignore` 才把它排除。想清掉重來，執行 `cargo clean`，整個 `target/` 就刪掉了。

## 重點整理

- `Cargo.toml` 是專案設定檔，用 TOML 格式；`[package]` 放 `name`、`version`、`edition`，`[dependencies]` 放相依套件。
- `edition` 是語言的版本代號，每個專案各自指定，不同 edition 的專案可以互相搭配；本書用 `"2024"`。
- `cargo run` 編譯並執行，`cargo build` 只編譯，`cargo check` 只檢查、不產生可執行檔，速度最快。
- 編譯結果都放在 `target/`，預設是 `dev` profile；`--release` 會做最佳化，輸出在 `target/release/`。
- `cargo new` 會順手建 `.gitignore`（內容是 `/target`）。
