# 實戰：Web 服務

這一章用 axum 把飲料店的菜單搬上網路。從一個回應「歡迎光臨」的伺服器開始，依序補上路由、從路徑與查詢字串取資料的 extractor、收發 JSON、在請求之間共用的 state、統一的錯誤回應、middleware，最後讓伺服器收到 Ctrl-C 時好好收工。

每一集都是一個小專案：能 `cargo run` 起來，再用 `curl` 試試看；也能用 `cargo test` 驗證，書上貼的 `curl` 結果與測試輸出都是實際跑出來的。測試不開伺服器、不佔連接埠，靠 tower 的 `oneshot` 直接把請求交給 `Router`，第 9 集才說明它的原理，前面幾集先照抄。這一章會用到第 15 章的 tokio、第 17 章的 serde 與 `thiserror`。
