# 實戰：SQLite

這一章用 rusqlite 把飲料店的訂單存進 SQLite 資料庫。從開資料庫開始，依序學 `execute`、`query_row`、`prepare` 加 `query_map`、參數綁定、交易、`NULL` 與 `Option`，最後把第 19 章放在記憶體裡的訂單換成資料庫，伺服器重開也不會消失。

這一章假設你寫過 SQL，只會用到最基本的幾種敘述。前七集的範例都是單檔的完整程式，大多用記憶體資料庫，第 1 集開檔案的範例會在結束前刪掉檔案，執行完不留任何東西；第 8 集是小專案，用第 19 章的 `oneshot` 測試。rusqlite 不在 Playground 的 crate 清單裡，所以這一章的範例沒有執行按鈕，請在本機用 `cargo run` 試。這一章會用到第 9 章的 `Result`、第 14 章的 `Arc<Mutex<T>>`、第 17 章的檔案與路徑，以及第 19 章的 axum。
