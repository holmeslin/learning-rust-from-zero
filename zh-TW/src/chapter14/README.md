# 並行

Go 寫並行很直接：`go` 開 goroutine、`chan` 傳訊息、`sync.Mutex` 加鎖，寫錯了就等 `-race` 在執行時抓。Rust 把同樣的工具都搬進標準函式庫：`thread::spawn` 開執行緒，`mpsc` channel 傳訊息，`Mutex`、`Arc`、`RwLock` 與 atomic 共用資料，`OnceLock` 與 `LazyLock` 做只跑一次的初始化。這一章從開執行緒、等執行緒教起，再逐一認識這些工具，最後用 `sync_channel` 與 worker pool 把它們組起來。

不一樣的是「寫錯了」發生在哪一段：Go 在程式跑起來之後才抓得到 data race，Rust 靠第 3、4 章的所有權與借用規則，加上 `Send`、`Sync` 兩個 trait，在編譯時期就擋下去。這一章會看到很多編譯錯誤，它們都是在替你把關，不是在找麻煩。
