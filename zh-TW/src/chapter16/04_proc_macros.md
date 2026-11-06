# 程序巨集與 derive 簡介

## 本集目標

知道程序巨集是「把一串程式碼變成另一串程式碼」的 Rust 函式，認得 derive、屬性、像函式這三種長相，並看過一個最小的 derive 長什麼樣子。

## 正文

### 跟 `macro_rules!` 差在哪

`macro_rules!` 是「比對樣子、換成別的樣子」。**程序巨集**（procedural macro）則是普通的 Rust 函式：收到一串程式碼（型別是 `TokenStream`），想怎麼分析都行，最後回傳另一串程式碼。能用 `if`、迴圈、讀整個型別的欄位，所以能做 `macro_rules!` 做不到的事。

代價是兩條規定：

- 程序巨集一定要放在**獨立的 crate**，`Cargo.toml` 寫 `[lib]` 加 `proc-macro = true`。因為編譯器得先把它編成程式、執行它，才能展開用到它的程式碼。
- 這種 crate 只能匯出巨集，不能放一般的函式與型別。

### 三種程序巨集

| 種類 | 怎麼用 | 定義時的標記 | 例子 |
| --- | --- | --- | --- |
| derive | `#[derive(名字)]` 加在 struct、enum 上，替它產生 `impl` | `#[proc_macro_derive(名字)]` | `Debug`、`Clone`（編譯器內建）、第 17 章第 9 集的 `Serialize` |
| 屬性 | `#[名字]` 加在函式、型別上，可以改寫整個項目 | `#[proc_macro_attribute]` | 第 15 章的 `#[tokio::main]` |
| 像函式 | `名字!(...)`，用起來像 `macro_rules!` | `#[proc_macro]` | 這本書沒有用到 |

第 5 章第 6 集起一直在用的 `#[derive(Debug)]`，就是替型別「寫出一份 `impl Debug`」。`#[tokio::main]` 則是把整個 `async fn main() { ... }` 收進去，改寫成第 15 章第 4 集看到的 `fn main()` 加 `block_on`。

### 最小的 derive

我們做一個 `#[derive(Describe)]`：加在 struct 上，替它產生一個 `describe` 方法，回傳型別的名字。先建兩個 package，主程式依賴放巨集的 crate：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
tea_macros = { path = "tea_macros" }
```

`tea_macros/Cargo.toml`：

```toml
[package]
name = "tea_macros"
version = "0.1.0"
edition = "2024"

[lib]
proc-macro = true
```

`tea_macros/src/lib.rs`：

```rust,ignore
use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let pos = tokens
        .iter()
        .position(|t| matches!(t, TokenTree::Ident(i) if i.to_string() == "struct"))
        .expect("Describe 只能用在 struct 上");
    let name = tokens[pos + 1].to_string();
    let code =
        format!("impl {name} {{ fn describe(&self) -> String {{ String::from(\"{name}\") }} }}");
    code.parse().expect("產生的程式碼有誤")
}
```

- `proc_macro` 是標準函式庫附的，不用另外加相依套件。`TokenStream` 是一串 token，`TokenTree` 是其中的一個。
- `#[proc_macro_derive(Describe)]` 宣告這個函式是名叫 `Describe` 的 derive 巨集。
- `input` 是被 derive 的整個 struct 的 token，我們找出 `struct` 這個字，它後面那一個 token 就是型別名稱。
- 最後把要產生的程式碼寫成字串，`parse` 成 `TokenStream` 回傳。編譯器會把它當成寫在 struct 旁邊的程式碼。

主程式用法跟 `#[derive(Debug)]` 一樣：

`src/main.rs`：

```rust,ignore
use tea_macros::Describe;

#[derive(Describe)]
struct Tea;

#[derive(Describe)]
struct Coffee;

fn main() {
    println!("{}", Tea.describe());
    println!("{}", Coffee.describe());
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
Tea
Coffee
```

`Tea` 和 `Coffee` 本身沒有寫 `describe`，方法是巨集在編譯時期替它們加上去的。

### 實務上不會自己這樣寫

上面靠「找 `struct` 後面那個 token」過日子，遇到 `pub struct`、泛型、欄位上的屬性就會出錯。實務上，寫程序巨集的人會用 `syn`（把 token 解析成語法樹）和 `quote`（用範本產生 token）這兩個 crate，它們不在這本書的清單裡，所以不展開。

身為使用者，你只要會用別人寫好的：第 17 章第 9 集的 `serde` 用 `#[derive(Serialize, Deserialize)]` 替型別產生轉成 JSON 的程式，第 18 章第 2 集的 `clap` 用 `#[derive(Parser)]` 替型別產生解析命令列的程式。

### 跟 Go 比一比

Go 的 `json:"name"` 是 struct 的 tag，`encoding/json` 在**執行時期**用反射（reflection）讀它，每次編碼都要走一遍。想在編譯前產生程式碼，得用 `go generate` 跑外部工具（像 `stringer`），產生的檔案要自己放進版本控制。

Rust 的 derive 在**編譯時期**就展開成一般程式碼，跟手寫的沒有差別：型別都被檢查過，執行時沒有反射的成本。不用另外跑工具，`cargo build` 就包辦了。

## 重點整理

- 程序巨集是 `TokenStream` 進、`TokenStream` 出的 Rust 函式，一定要放在獨立的 crate（`Cargo.toml` 寫 `[lib]` 加 `proc-macro = true`），這個 crate 只能匯出巨集。
- 三種：derive（`#[derive(名字)]`，`#[proc_macro_derive]`）、屬性（`#[名字]`，`#[proc_macro_attribute]`，例如 `#[tokio::main]`）、像函式（`名字!(...)`，`#[proc_macro]`）。
- `#[derive(Debug)]` 是替型別產生 `impl Debug`；巨集產生的程式碼在編譯時期展開，執行時沒有額外成本。
- 實務上寫程序巨集靠 `syn` 與 `quote`（本書不展開）；使用者只要會用別人提供的 derive，如 serde、clap。
- Go 靠 struct tag 加執行時期反射，或 `go generate` 外部工具；Rust 靠編譯時期展開。
