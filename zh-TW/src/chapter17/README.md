# 進階 std 與常用 crate

寫真正的程式，遲早要碰到外面的世界：讀寫檔案、組路徑、看環境變數、量時間、呼叫別的程式。這一章前半先把標準函式庫這一塊走完：`Read` 與 `Write` 是所有輸入輸出共用的介面，往下接 `BufRead`、`std::fs`、`Path`、`std::env`、`std::time` 與 `Command`。

後半換成每個 Rust 專案幾乎都會裝的 crate：`serde` 與 `serde_json` 處理 JSON，`tracing` 記錄事件，`rand` 產生亂數，`uuid` 產生識別碼，最後 `thiserror` 與 `anyhow` 讓錯誤處理不再那麼囉嗦。後面的實戰章就是用這些東西組起來的。
