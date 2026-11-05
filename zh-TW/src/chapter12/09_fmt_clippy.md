# `cargo fmt` 與 `cargo clippy`

## 本集目標

用 `cargo fmt` 自動排版，用 `cargo clippy` 找出可以改進的寫法，並知道兩者在 CI 裡怎麼用。

## 正文

### `cargo fmt`：一個指令排好版

第 1 章第 1 集裝好的 `rustfmt`，用 `cargo fmt` 呼叫，會把整個專案的 `.rs` 檔都排成官方的標準樣子，作用跟 Go 的 `gofmt` 一樣。本書每一段程式碼都是排過版的。把排版亂掉的程式交給它：

`src/main.rs`：

```rust,ignore
fn main(){
let prices=[30,38,45];
    let mut total=0;
    for p in prices { total+=p; }
  println!("合計 {total} 元");
}
```

`cargo fmt` 之後的 `src/main.rs`：

```rust
fn main() {
    let prices = [30, 38, 45];
    let mut total = 0;
    for p in prices {
        total += p;
    }
    println!("合計 {total} 元");
}
```

縮排、空格、換行都被整理好了，程式的意思完全沒變。大家排版一致，就不會有人在審查程式碼時吵空格要放哪裡。

在 CI 上，常用 `cargo fmt --check`：它**不改檔案**，只檢查有沒有排好；格式不對的地方會列出差異，並以非 0 的結束碼結束，CI 因此失敗。

### `cargo clippy`：找出更好的寫法

`clippy` 是第 1 章第 19 集預告過的檢查工具。它比編譯器的警告看得更細，專門挑出「雖然能跑，但有更好寫法」的程式。下面這段，檢查 `Vec` 是不是空的，用了 `len() == 0`：

`src/main.rs`：

```rust,warn
fn main() {
    let orders: Vec<u32> = Vec::new();
    if orders.len() == 0 {
        println!("沒有訂單");
    }
}
```

這段程式能編譯、能執行。用 `cargo clippy`（用法跟 `cargo check` 一樣，只是多了 clippy 的檢查）：

`cargo clippy` 的輸出（省略編譯進度）：

```text
warning: length comparison to zero
 --> src/main.rs:3:8
  |
3 |     if orders.len() == 0 {
  |        ^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `orders.is_empty()`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.96.0/index.html#len_zero
  = note: `#[warn(clippy::len_zero)]` on by default

warning: `teashop` (bin "teashop") generated 1 warning (run `cargo clippy --fix --bin "teashop" -p teashop -- ` to apply 1 suggestion)
```

訊息格式跟編譯器的警告一樣：哪一行、為什麼，還有 `help` 直接給你改好的寫法 `orders.is_empty()`。下面的 `= help: for further information visit ...` 是這條規則（叫 `len_zero`）的說明網頁，網址裡的版本號碼會隨 clippy 版本變。

最後一行說可以執行 `cargo clippy --fix` 自動套用建議，改完要自己看一下結果。

### 在 CI 裡把警告當成錯誤

平常 clippy 的警告不會讓程式失敗。要在 CI 上強迫大家處理，在後面加 `-D warnings`（deny warnings，把警告升級成錯誤）：

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

這三行，就是多數 Rust 專案 CI 的基本內容：排版、clippy、測試都要通過。

有些警告你判斷真的沒問題，可以在程式裡用 `#[allow(...)]` 關掉，這種寫法附錄一 j 才介紹。

## 重點整理

- `cargo fmt` 把整個專案的程式碼排成標準格式；`cargo fmt --check` 只檢查不改檔，格式不對就以非 0 結束碼結束。
- `cargo clippy` 找出能編譯、但有更慣用寫法的程式，訊息附 `help` 與改法；`cargo clippy --fix` 能自動套用。
- CI 的常見組合：`cargo fmt --check`、`cargo clippy -- -D warnings`、`cargo test`。
- 關掉某條警告要用 `#[allow(...)]`（附錄一 j）。
