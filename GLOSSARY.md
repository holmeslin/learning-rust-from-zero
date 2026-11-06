# 從零開始學 Rust

寫給會 Go 的 Rust 新手的繁體中文 Rust 教學書；本檔統一書籍編排、AI 助教規格與書中 Rust 術語的用法。

## 書籍結構

**部**：
全書最上層的分組；第一部是語言基礎（只用 std），第二部是並行、async、進階主題與實戰（開放 crate）。
_Avoid_: 篇、卷

**章**：
一個主題的集合，例如「第 3 章 所有權」。
_Avoid_: 單元、課

**集**：
一個檔案，只教一個小概念，固定包含「本集目標」「正文」「重點整理」。
_Avoid_: 節、課、lesson

**附錄**：
屬於主線閱讀順序的補充章節；附錄一要在進入第二部之前讀完。

**跟 Go 比一比**：
集裡選用的小節，只在 Rust 跟 Go 不一樣、容易踩坑時出現。
_Avoid_: Go 對照、給 Gopher 的提醒

## 教學規格

**固定句型**：
書中要求讀者先照抄、之後才解釋原理的寫法，例如讀取輸入的 `read_line` 片段。
_Avoid_: 樣板、boilerplate

**固定題庫**：
`EXERCISES.md` 裡預先寫好的練習題，只涵蓋第 3、4 章。

**臨時題**：
AI 助教依讀者目前進度現場出的題目，必須明說不是固定題庫裡的題目。

## Rust 術語

原則：跟 Rust 關鍵字或生態系名稱相同的詞保留英文；其他翻成台灣用語，第一次出現時附英文。

保留英文：`unsafe`、FFI、`const fn`、`build.rs`、`trait`、trait bound、newtype、crate、package、workspace、feature、profile、prelude、`struct`、`enum`、`match`、slice、`Future`、`async`、variant、move、drop、adapter、consumer、lint、clone-on-write、channel、data race、race condition、atomic、worker pool、runtime、`await`（寫成 `.await`）、span、subscriber、stdin、stdout、stderr、SIGINT、SIGPIPE、handler、extractor、middleware、listener、axum 的 `Router`／`State`、tower 的 `Service`／`Layer`、Cargo 與它的指令名稱。

| 書中用語 | 英文 | 不要用 |
| --- | --- | --- |
| 所有權 | ownership | 擁有權 |
| 擁有者 | owner | 所有者、持有者 |
| 作用域 | scope | 域 |
| 借用 | borrowing | 租借 |
| 參照 | reference | 引用 |
| 可變參照 | mutable reference | 可變引用 |
| 解參照 | dereference | 解引用 |
| 生命週期 | lifetime | 生命周期、壽命 |
| 閉包 | closure | 閉合 |
| 擷取 | capture | 捕獲、捕捉 |
| 組合子 | combinator | 組合器 |
| 惰性 | lazy | 懶惰、延遲 |
| 模式 | pattern | 樣式 |
| 模式比對 | pattern matching | 模式匹配 |
| 解構 | destructuring | 解包、拆包 |
| 綁定 | binding | 繫結、捆綁 |
| 窮盡 | exhaustive | 詳盡、窮舉 |
| 分支（`match` 的） | arm | 臂 |
| 巨集 | macro | 宏 |
| 智慧指標 | smart pointer | 智能指針 |
| 泛型 | generics | 範型 |
| 模組 | module | 模塊 |
| 函式 | function | 函數 |
| 型別 | type | 類型 |
| 變數 | variable | 變量 |
| 不可變 | immutable | 唯讀 |
| 執行緒 | thread | 線程 |
| 並行 | concurrency | 並發 |
| 迭代器 | iterator | 迭代子 |
| 字元 | character | 字符 |
| 位元組 | byte | 字節 |
| 溢位 | overflow | 溢出 |
| 預設 | default | 默認 |
| 複本 | copy | 拷貝 |
| 程式 | program | 程序（例外：「程序巨集」保留「程序」，因為那裡指的是 procedural，不是 program） |
| 堆疊（stack） | stack | 棧 |
| 堆積（heap） | heap | 堆 |
| 懸置參照 | dangling reference | 懸垂引用、野指標 |
| 內部可變性 | interior mutability | 內部可變 |
| 參照計數 | reference counting | 引用計數 |
| 強參照 | strong reference | 強引用 |
| 弱參照 | weak reference | 弱引用 |
| 參照循環 | reference cycle | 循環引用 |
| 記憶體洩漏 | memory leak | 內存洩漏、記憶體洩露 |
| 解參照轉換 | deref coercion | 解引用強制轉換 |
| 關聯函式 | associated function | 靜態方法 |
| 關聯型別 | associated type | 相關類型 |
| 預設方法 | default method | 預設實作 |
| 覆寫 | override | 重寫、覆蓋 |
| 型別參數 | type parameter | 類型參數 |
| trait 物件 | trait object | 特徵物件 |
| 孤兒規則 | orphan rule | 孤立規則 |
| 可見性 | visibility | 能見度 |
| 屬性 | attribute | 特性 |
| 文件註解 | doc comment | 說明註解 |
| 單元測試 | unit test | 模組測試 |
| 整合測試 | integration test | 集成測試 |
| 相依套件 | dependency | 依賴 |
| 字面值 | literal | 字面量 |
| 標籤（`break` 用的） | label | 標號 |
| 短路求值 | short-circuit evaluation | 短路運算 |
| 型別別名 | type alias | 類型別名 |
| 截斷 | truncation | 截取 |
| 互斥鎖 | mutex | 互斥體 |
| 死結 | deadlock | 死鎖 |
| 背壓 | backpressure | 反壓 |
| 原子操作 | atomic operation | 原子運算 |
| 範圍執行緒 | scoped thread | 作用域線程 |
| 生產者、消費者 | producer、consumer | 生產方、消費方 |
| 鎖中毒 | poisoned | 鎖污染 |
| 任務 | task（`tokio::spawn` 開出來的） | 協程、工作項 |
| 輪詢 | polling（`Future::poll`） | 輪巡 |
| 取消 | cancellation | 撤銷、終止 |
| 逾時 | timeout | 超時 |
| 非同步 | asynchronous | 異步 |
| 協作式 | cooperative | 合作式 |
| 搶佔 | preemption | 搶占 |
| 排程 | scheduling | 調度 |
| 宣告式巨集 | declarative macro（`macro_rules!`） | 聲明式宏 |
| 程序巨集 | procedural macro | 過程宏 |
| 展開 | expand（巨集的） | 擴展 |
| 片段類型 | fragment specifier（`expr`、`ident`） | 片段指示符 |
| 衛生 | hygiene（巨集的） | 衛生性 |
| 原始指標 | raw pointer（`*const T`、`*mut T`） | 裸指標、生指標 |
| 未定義行為 | undefined behavior | 未定義的行為 |
| 條件編譯 | conditional compilation（`#[cfg]`） | 條件式編譯 |
| const 泛型 | const generics | 常數泛型 |
| 序列化 | serialize | 串行化 |
| 反序列化 | deserialize | 反串行化 |
| 緩衝區 | buffer | 緩存（那是 cache） |
| 環境變數 | environment variable | 環境變量 |
| 行程 | process（作業系統的） | 進程、程序 |
| 子行程 | child process | 子進程 |
| 亂數 | random number | 隨機數 |
| 種子 | seed（亂數的） | 種籽 |
| 錯誤鏈 | error chain（`source()` 一層層往下） | 錯誤堆疊 |
| 引數 | argument（命令列的，也指呼叫函式時傳的值） | 參數（那是 parameter） |
| 位置引數 | positional argument（`teashop 紅茶` 的 `紅茶`） | 位置參數 |
| 選項 | option（`--cups 2`、`-c 2`） | 選項參數、開關（`bool` 的那種可以說「開關」） |
| 子命令 | subcommand（`cargo run` 的 `run`） | 子指令 |
| 結束碼 | exit code | 退出碼、返回碼、離開碼 |
| 管線 | pipe（shell 的 `\|`） | 管道 |
| 訊號 | signal（SIGINT、SIGPIPE） | 信號 |
| 符號連結 | symbolic link（symlink） | 軟連結、符號鏈接 |
| 連接埠 | port（`127.0.0.1:3000` 的 3000） | 端口 |
| 優雅關閉 | graceful shutdown | 平滑關機 |
