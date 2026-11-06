# async

Go 讓你隨手開幾萬個 goroutine，每一個都能放心地「等」。Rust 走另一條路：在函式前面寫 `async`、在要等的地方寫 `.await`，讓一條執行緒輪流照顧大量的任務。這一章先講 async 為什麼存在、`Future` 是什麼，再用 tokio 開任務、傳訊息、同時等好幾件事、設期限、取消，最後談 async 裡的鎖、為什麼不能阻塞 runtime、async 閉包，以及 `Send` 的限制。

跟 Go 最大的不同是：Rust 的標準函式庫沒有 runtime，`Future` 也不會自己動，一切都要寫明白。這會多一些儀式，換來的是所有「可能停下來」的地方都看得見，跨執行緒的錯誤也在編譯時期就被擋下。這一章是本書第一次用到 crate（tokio 與 tokio-util），版本固定在書中驗證過的版本。
