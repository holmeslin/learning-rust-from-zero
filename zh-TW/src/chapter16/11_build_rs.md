# `build.rs` 簡介

## 本集目標

知道 `build.rs` 是什麼、cargo 什麼時候執行它，並會用它輸出 `cargo::` 指示，把環境變數與自訂的 `cfg` 條件交給編譯器。

## 正文

### 編譯之前先跑的程式

在 package 的最上層（跟 `Cargo.toml` 同一層）放一個叫 `build.rs` 的檔案，cargo 會自動發現它：**編譯你的 package 之前，先把 `build.rs` 編成程式、執行一次**。`Cargo.toml` 不用改。

它是一支普通的 Rust 程式，有自己的 `fn main`。跟 cargo 溝通的方式，是用 `println!` 印出以 `cargo::` 開頭的指示，cargo 讀了這些行，再決定怎麼編譯你的 package。常見的用途有：

- 編譯隨附的 C 程式碼，再連結進來（常配合 `cc` 這個 crate，本書不展開）。
- 在編譯時期產生程式碼，例如由協定檔產生對應的型別。
- 偵測環境，決定要打開哪些條件。

### 第一個 `build.rs`

下面的 `build.rs` 印了五行指示：

`build.rs`：

```rust,ignore
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-env=SHOP_NAME=飲料店");
    println!("cargo::rustc-check-cfg=cfg(happy_hour)");
    println!("cargo::rustc-cfg=happy_hour");
    println!("cargo::warning=build.rs 執行了");
}
```

- `cargo::rerun-if-changed=build.rs`：只有 `build.rs` 變了才重跑。沒寫這一行，package 裡任何檔案變了，cargo 都會重跑它。
- `cargo::rustc-env=名稱=值`：替編譯器設一個環境變數，程式裡用 `env!("名稱")` 讀。
- `cargo::rustc-cfg=名稱`：定義一個自訂的條件 `名稱`，程式裡可以寫 `cfg!(名稱)`、`#[cfg(名稱)]`（上一集的 `cfg`）。
- `cargo::rustc-check-cfg=cfg(名稱)`：告訴編譯器這個名稱是合法的，不然上一集的 `unexpected_cfgs` 警告會出現。
- `cargo::warning=訊息`：在 cargo 的輸出裡印一則警告。`build.rs` 自己的 `println!` 輸出會被 cargo 吃掉，要讓人看到訊息，就靠它。

主程式用得到這些：

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", env!("SHOP_NAME"));
    if cfg!(happy_hour) {
        println!("現在是 happy hour");
    }
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
warning: teashop@0.1.0: build.rs 執行了
     Running `target/debug/teashop`
飲料店
現在是 happy hour
```

- `env!("SHOP_NAME")` 在**編譯時期**讀環境變數，結果是 `&'static str`，值直接寫進程式。環境變數不存在，是編譯錯誤，不是 panic。
- `cfg!(happy_hour)` 成立，是因為 `build.rs` 印了 `rustc-cfg=happy_hour`。
- `warning` 那一行就是 `build.rs` 印的，前面的 `teashop@0.1.0` 是 package 名稱與版本。

為了示範，這支 `build.rs` 把五行都寫死了。真的專案裡，這一行通常是先偵測環境（作業系統、有沒有某個函式庫……）才決定要不要印。

### 其他要知道的事

- `OUT_DIR` 是 cargo 替 `build.rs` 準備的資料夾，`build.rs` 產生的檔案放在那裡，主程式再用 `include!` 之類的巨集讀進來。
- `build.rs` 自己的相依套件寫在 `Cargo.toml` 的 `[build-dependencies]`，跟程式本身的 `[dependencies]` 分開。
- `cargo::` 開頭（兩個冒號）的寫法從 Rust 1.77 起支援；更早的單冒號 `cargo:` 現在仍能用，看到舊專案不用意外。
- `build.rs` 跟程序巨集（第 4 集）一樣，會在你編譯的時候執行任意程式。編譯來路不明的 crate，等於執行它的 `build.rs`，要有警覺。
- `build.rs` 是最後手段。能用 `cfg`、feature、`const fn` 解決的，就不必寫它：它會拖慢編譯，也讓專案更難讀。

### 跟 Go 比一比

Go 沒有編譯前會自動跑的程式。要在編譯前產生程式碼，習慣是在原始檔裡寫 `//go:generate` 註解，開發者要自己執行 `go generate`，產生的檔案再提交進版本控制，`go build` 不會替你跑。Rust 的 `build.rs` 是 `cargo build` 的一部分，需要的時候 cargo 自己會先跑，產生的檔案放在 `OUT_DIR`，不用提交。

## 重點整理

- 放在 package 最上層的 `build.rs`，cargo 在編譯 package 之前先編譯並執行它，不用改 `Cargo.toml`。
- 用 `println!("cargo::…")` 跟 cargo 溝通：`rerun-if-changed=檔案`、`rustc-env=名稱=值`（程式用 `env!`）、`rustc-cfg=名稱`（搭配 `rustc-check-cfg=cfg(名稱)`）、`warning=訊息`。
- 常見用途：編譯 C 程式碼、產生程式碼、偵測環境決定 `cfg`；產生的檔案放在 `OUT_DIR`；`build.rs` 的相依套件寫在 `[build-dependencies]`。
- `build.rs` 與程序巨集一樣會在編譯時執行任意程式；能用 `cfg`、feature、`const fn` 解決就不要寫它。
- Go 的 `go generate` 要手動執行、產物要提交；`build.rs` 是 `cargo build` 的一部分。
