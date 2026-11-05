# 文件註解 `///` 與 `cargo doc`

## 本集目標

用 `///` 替函式與型別寫文件註解，用 `//!` 寫整個 crate 的說明，再用 `cargo doc` 產生網頁文件。

## 正文

### `///`：貼在項目前面的註解

第 1 章第 2 集留了一個伏筆：`//` 是給讀程式的人看的註解，而 `///` 是**文件註解**（doc comment），寫在函式、`struct`、欄位、`enum` 前面，內容用 Markdown 寫，工具會把它變成網頁文件。

`src/lib.rs`：

```rust,ignore
//! 飲料店的價格與帳單。
//!
//! 這個 crate 提供菜單價格，以及會員資料。

/// 回傳飲料的單價，單位是元。
///
/// 不在菜單上的飲料一律當作 45 元。
pub fn price(drink: &str) -> u32 {
    match drink {
        "紅茶" => 30,
        "奶茶" => 38,
        _ => 45,
    }
}

/// 一位會員。
pub struct Member {
    /// 會員名稱。
    pub name: String,
    /// 目前累積的點數。
    pub points: u32,
}
```

`///` 緊貼在後面那一個項目上，`price` 的說明就是前面那兩段。第一段（空行之前）會當成簡短摘要，列在清單裡；後面的是詳細內容。結構欄位也可以各寫一行 `///`。

### `//!`：說明「包住它的東西」

`//!` 是**內層文件註解**，說明的是它所在的那一層，不是後面的項目。放在 `lib.rs` 最上面，就是整個 crate 的簡介；放在某個模組檔案的最上面，就是那個模組的說明。上面 `lib.rs` 開頭那三行就是 crate 的簡介。

文件註解裡可以用 Markdown 的標題、清單、`行內程式碼`。Rust 社群有幾個慣用的小標題：

- `# Examples`：用法範例（第 13 集會看到，範例可以變成測試）。
- `# Panics`：什麼情況下會 panic。
- `# Errors`：回傳 `Err` 的情況。

標題名稱是英文，這是工具與社群的慣例，內文可以寫中文。想連結到別的項目，寫 ``[`price`]`` 就會自動連到那個項目的文件頁。

### `cargo doc`：產生網頁

`cargo doc` 的輸出（省略編譯進度）：

```text
   Generated /home/you/teashop/target/doc/teashop/index.html
```

`cargo doc --open` 會順便用瀏覽器打開。網頁放在 `target/doc/` 底下，打開 `target/doc/teashop/index.html`，就能看到：crate 的簡介、`price` 與 `Member` 的清單（摘要會顯示在旁邊）、點進去有完整說明與每個欄位的註解。

有兩件事要知道：

- 文件預設只收**公開**（`pub`）的項目，沒有 `pub` 的函式不會出現在網頁上；要連私有的也列出來，加 `--document-private-items`。
- 你用到的相依套件（第 14 集）的文件也會一起產生，所以在別人的 crate 裡用到什麼函式，直接翻它的網頁就知道。發布到 crates.io 的 crate，文件會自動放上 [docs.rs](https://docs.rs)，標準函式庫的文件則在 `doc.rust-lang.org/std`。

寫 library 時，替每個 `pub` 的項目加上一兩句 `///`，是使用者最先看到的東西。

## 重點整理

- `//` 是一般註解；`///` 是文件註解，貼在後面的函式、型別、欄位上，內容是 Markdown。
- `//!` 是內層文件註解，說明包住它的 crate 或模組，放在檔案最上面。
- 慣用的小標題：`# Examples`、`# Panics`、`# Errors`；第一段是簡短摘要。
- `cargo doc` 產生 `target/doc/` 底下的網頁，`--open` 順便打開；預設只收 `pub` 的項目，也會產生相依套件的文件。
