[![CC BY-NC-ND 4.0](https://img.shields.io/badge/content-CC%20BY--NC--ND%204.0-blue)](https://creativecommons.org/licenses/by-nc-nd/4.0/)
[![CC0 1.0](https://img.shields.io/badge/code-CC0%201.0-blue)](https://creativecommons.org/publicdomain/zero/1.0/)

# 從零開始學 Rust

寫給會 Go 的人的 Rust 入門教學（Rust 1.99、edition 2024），請閱讀[線上版](https://holmeslin.github.io/learning-rust-from-zero/zh-TW/)。

## 本機建置

需要 [rustup](https://rustup.rs/)（會依 `rust-toolchain.toml` 自動使用 Rust 1.99）與 [mdBook](https://github.com/rust-lang/mdBook) 0.5.4：

```bash
cargo run --manifest-path tools/checkcode/Cargo.toml -- zh-TW/src   # 驗證書中所有 Rust 範例
cd zh-TW && mdbook serve                                             # 本機預覽
```

### 範例程式碼標註

書中 ```` ```rust ```` 區塊預設要通過 `rustfmt --check`、`clippy-driver -D warnings`，並且執行成功。可在 info string 加上標註：

| 標註 | 意義 |
| --- | --- |
| `rust,compile_fail=E0382` | 必須編譯失敗，而且錯誤碼相符 |
| `rust,stdin=3\n5,noplayground` | 執行時餵入 stdin（`\n` 代表換行，值內不可有空白） |
| `rust,exit=101` | 預期的結束碼（例如 panic） |
| `rust,no_run` | 只編譯，不執行（伺服器、signal） |
| `rust,warn` | 允許編譯器警告 |
| `rust,test` | 以 `--test` 編譯並執行測試 |
| `rust,noplayground` | 網頁上不顯示執行按鈕 |
| `rust,ignore` | 不驗證（片段、多檔案範例） |

### 第二部的範例怎麼使用 crate

第一部只用 std。第二部（第 13 章起）的範例可以用 tokio、tokio-util、clap、axum、tower、http-body-util、serde、serde_json、tracing、rand、uuid、thiserror、anyhow、rusqlite（bundled）。範例照常寫 ```` ```rust ````，不必加標註：程式裡出現 `tokio::` 這類 `<crate>::` 路徑，checkcode 就會把區塊放進 `tools/checkcode/deps/` 的共用依賴 package，所有用到 crate 的區塊一次批次編譯與 lint，再逐一執行；上表的標註規則照樣適用。清單以外的 crate 會驗證失敗。

版本統一鎖在 `tools/checkcode/deps/Cargo.toml`，`Cargo.lock` 一併提交。升級版本時改該檔，再執行 `cargo update --manifest-path tools/checkcode/deps/Cargo.toml`。第一次驗證會花幾分鐘編譯所有 crate，之後使用 `tools/checkcode/deps/target` 的快取，CI 也會快取這個目錄。

詳細規則見 `docs/style-guide.md`。

## 授權

除非另有標示，本教學的文字、圖片與非程式碼內容以 [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) 授權。

本教學中的程式碼範例以 [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/) 釋出，可自由複製、修改與使用。
