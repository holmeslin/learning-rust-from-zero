# 模組、Cargo 與測試

前面的程式都塞在一個 `main.rs` 裡，也一路留下幾個先照抄的東西：`use std::collections::HashMap;`、函式前面的 `pub`、`cargo new` 建出來的 `Cargo.toml`。這一章把它們都解釋清楚：怎麼用 `mod` 把程式分成模組、拆成多個檔案，用 `pub` 與 `use` 決定誰看得到什麼；Cargo 怎麼管理 crate、相依套件與 `Cargo.lock`、feature 與 workspace。

後半章是讓程式「長大之後還穩」的工具：文件註解與 `cargo doc`、`cargo fmt` 與 `cargo clippy`、單元測試、整合測試與 doc test。Go 的 `go test`、`gofmt`、`go vet` 都有對應的東西，但 Rust 的模組、可見性與 crate 的分法不太一樣，值得慢慢看。
