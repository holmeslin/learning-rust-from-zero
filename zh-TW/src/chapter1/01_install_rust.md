# 安裝 Rust

## 本集目標

用 `rustup` 把 Rust 裝好，並確認 `rustc` 與 `cargo` 都能執行。

## 正文

### rustup 是什麼

Go 是下載安裝檔，Rust 則是先裝一個叫 `rustup` 的工具，之後 Rust 的安裝、更新、切換版本都交給它。它會一次裝好三樣東西：

- `rustc`：編譯器。
- `cargo`：建置與套件管理工具。建立專案、編譯、執行都靠它，作用差不多是 `go mod`、`go build`、`go run` 合在一起。
- `clippy` 與 `rustfmt`：檢查程式寫法的工具和自動排版的工具，第 12 章會介紹。

### 安裝

macOS 或 Linux，在終端機貼上這一行：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows 請到 <https://www.rust-lang.org/tools/install> 下載 `rustup-init.exe` 執行。

安裝過程出現選項時，直接按 Enter 用預設值就好。裝完之後，**關掉終端機再開一個新的**，讓環境設定生效。

### 確認裝好了

```bash
rustc --version
cargo --version
```

執行結果：

```text
rustc 1.99.1 (58cf8f9c3 2026-10-13)
cargo 1.99.1 (5f94df478 2026-08-27)
```

本書以 Rust 1.99 為準。你看到的版本號碼小數點後面可能不同，沒有關係；如果主版本差很多，編譯器的訊息文字可能會跟書上不一樣。

### 之後怎麼更新

想升級到最新版，執行：

```bash
rustup update
```

### 編輯器

用什麼都可以。如果你已經在用 VS Code，裝 `rust-analyzer` 擴充套件就有補全和錯誤提示，地位相當於 Go 的 `gopls`。

## 重點整理

- Rust 用 `rustup` 安裝與管理版本，一次裝好 `rustc`、`cargo`、`clippy`、`rustfmt`。
- `rustc` 是編譯器；日常寫程式主要用 `cargo`。
- 用 `rustc --version` 和 `cargo --version` 確認安裝成功。
- 用 `rustup update` 更新。
