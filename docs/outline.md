# 從零開始學 Rust — 全書目錄規劃（草案）

Rust 1.99 · edition 2024 · 繁中 · 給會寫 Go 的 Rust 新手 · 一集一個小概念 · 第一部只用 std，第二部開放 crate

固定句型（先照抄、之後才解釋）的完整清單見 `docs/style-guide.md`。本檔只標註各章需要特別注意的進度邊界。

# 第一部

## 第 1 章 基礎
01 安裝 Rust（rustup）· 02 第一個程式（`cargo new`、`cargo run`、`fn main`、`println!` 固定句型、註解）· 03 變數與 `{}` 佔位符 · 04 預設不可變與 `let mut` · 05 變數必須初始化（沒有零值）· 06 shadowing · 07 運算子：跟 Go 差在哪（沒有 `++`、複合賦值、整數除法、沒有隱式轉型）· 08 `if` 與 `if` 是運算式 · 09 讀取輸入（stdin 固定句型）· 10 文字轉數字（`parse` + `expect` 固定句型）· 11 綜合練習 · 12 `loop` 與 `break` 帶值 · 13 `while` · 14 `for`、範圍與 `continue`（含巢狀迴圈）· 15 純量型別（`i32` `i64` `u8` `usize` `f64` `bool` `char`）· 16 整數溢位（debug 會 panic，跟 Go 比一比）· 17 型別轉換 `as` · 18 `char`、跳脫字元與 raw string · 19 未使用變數警告與 `_x`

邊界：`match` 留到第 6 章，本章不教。浮點範例避開 3.14、2.718（clippy `approx_constant`）。

## 第 2 章 函式與複合型別
01 `const` 與 `static` · 02 函式 · 03 參數一定要標型別 · 04 回傳值與最後一個運算式 · 05 陳述式 vs 運算式（分號的意義）· 06 early `return` · 07 unit 型別 `()` · 08 tuple · 09 用 tuple 回傳多個值（跟 Go 比一比）· 10 解構與 `_` · 11 陣列 `[T; N]` · 12 索引、`usize` 與越界 panic · 13 `for` 走訪陣列 · 14 遞迴 · 15 `String` 初探（`String::from`、`push_str`、`len`）

邊界：`static` 只教不可變（edition 2024 取 `static mut` 的參照是硬錯誤），可變的全域狀態留到第 14 章。第 7 章以前一律用陣列，不用 `Vec`。

## 第 3 章 所有權
01 為什麼需要所有權（沒有 GC，跟 Go 比一比）· 02 stack 與 heap · 03 所有權三條規則 · 04 離開作用域就 drop · 05 move · 06 move 之後不能再用（E0382）· 07 `clone` · 08 `Copy` 型別 · 09 傳參數也會 move · 10 回傳值交還所有權 · 11 「交出去再拿回來」很麻煩

## 第 4 章 借用與切片
01 參照 `&` · 02 借用不轉移所有權 · 03 可變參照 `&mut` · 04 借用規則：多個 `&` 或一個 `&mut` · 05 參照用完就結束（NLL）· 06 懸置參照（E0106，第 10 章正式介紹）· 07 解參照 `*` · 08 字串 slice `&str` · 09 字串字面值就是 `&str` · 10 參數用 `&str` 而不是 `&String` · 11 陣列 slice `&[T]` · 12 `&mut [T]`

邊界：第 4、5 章回傳參照的函式只能有**一個**參照參數，讓省略規則永遠成立；struct 只擁有 `String`，不放參照。第 10 章再揭曉。

## 第 5 章 struct 與方法
01 `struct` · 02 建立與讀取欄位 · 03 欄位初始化簡寫 · 04 struct 更新語法 `..`（會 move）· 05 tuple struct 與 unit struct · 06 `#[derive(Debug)]` 與 `{:?}` `{:#?}` · 07 `impl` 與方法 · 08 `&self`、`&mut self`、`self`（跟 Go 的接收者比一比）· 09 關聯函式與 `new` 慣例 · 10 `.` 會自動參照與解參照 · 11 多個 `impl` 區塊

邊界：`derive` 的原理第 8 章解釋。`#[derive(Debug)]` 的 struct 每個欄位都要被讀到，否則 rustc 會報 `dead_code`。

## 第 6 章 enum 與模式比對
01 `enum` · 02 帶資料的 variant · 03 `match` · 04 `match` 必須窮盡 · 05 `_` 與綁定變數 · 06 `Option<T>`（沒有 nil，跟 Go 比一比）· 07 用 `match` 處理 `Option` · 08 `if let` · 09 `let else` · 10 match guard · 11 `|` 與範圍模式 · 12 解構 struct 與 tuple · 13 slice pattern · 14 `matches!` · 15 let chains（edition 2024）· 16 enum 的方法

## 第 7 章 集合
01 `Vec` 與 `vec!` · 02 `push`、`pop` 與 `while let` · 03 讀取元素：索引 vs `get` · 04 走訪 `Vec`（`&v`、`&mut v`）· 05 持有參照時不能 `push`（E0502）· 06 參數用 `&[T]` · 07 `String` 深入：`push`、`+`、`format!` · 08 字串不能用索引（跟 Go 比一比）· 09 `chars` 與 `bytes` · 10 常用字串方法 · 11 `HashMap` 基礎 · 12 `get` 回傳 `Option`（跟 Go 的 comma-ok 比一比）· 13 `entry` 與 `or_insert` · 14 走訪 `HashMap`（順序不固定）· 15 `HashSet` · 16 `BTreeMap`（有序）· 17 `sort` 與 `dedup`

邊界：`entry` 只用 `or_insert` / `or_default`，帶閉包的版本留到第 11 章。

## 第 8 章 trait 與泛型
01 `trait` 定義與實作 · 02 必須明確實作（跟 Go 的隱式介面比一比）· 03 預設方法 · 04 參數用 `impl Trait` · 05 trait bound `<T: Trait>` · 06 `where` 子句 · 07 回傳 `impl Trait` · 08 泛型函式 · 09 泛型 struct 與 `impl<T>` · 10 `derive` 是什麼（`Debug` `Clone` `Copy` `PartialEq` `Eq` `Hash` `Default`）· 11 `Display` 與 `to_string` · 12 `PartialOrd` 與 `Ord` · 13 `From` 與 `Into` · 14 `FromStr` 與 `parse` 的原理 · 15 trait 物件 `dyn Trait`（`Box` 第 13 章正式介紹）· 16 孤兒規則 · 17 關聯型別 · 18 運算子多載 `Add`

## 第 9 章 錯誤處理
01 `panic!` · 02 `Result<T, E>` · 03 用 `match` 處理 `Result` · 04 `unwrap` 與 `expect`（揭曉固定句型）· 05 `?` 運算子 · 06 `?` 也能用在 `Option` · 07 用 enum 當錯誤型別（含自己實作 `FromStr`，兌現第 8 章第 14 集）· 08 實作 `Display` 與 `Error` · 09 `?` 會自動呼叫 `From` · 10 `Box<dyn Error>` · 11 `main` 回傳 `Result` · 12 什麼時候 panic、什麼時候回傳 `Result`（跟 Go 的 `if err != nil` 比一比）

邊界：`Option` / `Result` 的組合子（`map`、`and_then`…）用到閉包，留到第 11 章；`unwrap_or`、`ok_or`、`map_err` 等方法也一併留到第 11 章。不用 `std::fs`（第 17 章），錯誤來源只有 `parse` 與 `read_line`。

## 第 10 章 生命週期
01 為什麼需要生命週期 · 02 標註語法 `'a` · 03 函式的生命週期 · 04 省略規則 · 05 struct 持有參照 · 06 方法的生命週期 · 07 `'static` · 08 常見錯誤：改成擁有資料就好

## 第 11 章 閉包與迭代器
01 閉包 · 02 擷取環境 · 03 `move` 閉包 · 04 `Fn`、`FnMut`、`FnOnce` · 05 閉包當參數 · 06 回傳閉包 · 07 `Iterator` 與 `next` · 08 `iter`、`iter_mut`、`into_iter`（揭曉 `for`）· 09 迭代器是惰性的 · 10 `map` 與 `filter` · 11 `collect` · 12 `sum`、`count`、`min`、`max` · 13 `enumerate`、`zip`、`rev` · 14 `fold` · 15 `find`、`any`、`all`、`position` · 16 `sort_by_key` 與 `sort_by` · 17 自己實作 `Iterator` · 18 `Option` / `Result` 的組合子 · 19 `or_insert_with` 與 `and_modify`

## 第 12 章 模組、Cargo 與測試
01 `Cargo.toml` · 02 binary crate 與 library crate · 03 `mod` · 04 `pub` 與可見性 · 05 `use` 與路徑 · 06 把模組拆成多個檔案 · 07 `pub(crate)` · 08 文件註解 `///` 與 `cargo doc` · 09 `cargo fmt` 與 `cargo clippy` · 10 `#[test]` 與 `cargo test` · 11 `assert_eq!` 與 `#[should_panic]` · 12 `tests/` 整合測試 · 13 doc test · 14 加入 crate 與 `Cargo.lock` · 15 features · 16 workspace

邊界：doc test 只對 library crate 有效，第 13 集是多檔案範例（`ignore`）。

## 附錄一
a 數字字面值（`1_000`、`0x`、`0b`、`10u8`）· b 標籤 `'outer` · c 短路求值 · d shadowing 陷阱 · e `Self` · f 型別別名 `type` · g 格式化進階（`{:>8}`、`{:.2}`）· h 陣列、slice 與 `Vec` 的差別 · i `as` 的截斷規則 · j 屬性 `#[...]` 與 `#[allow]`

# 第二部

## 第 13 章 智慧指標
01 `Box<T>` · 02 遞迴型別需要 `Box` · 03 `Deref` · 04 `Drop` · 05 `Rc<T>` · 06 `RefCell<T>` 與內部可變性 · 07 `Rc<RefCell<T>>` · 08 參照循環與 `Weak` · 09 `Cell<T>` · 10 `Cow` 初探

## 第 14 章 並行
01 `thread::spawn` · 02 `join` · 03 把資料 `move` 進執行緒 · 04 `thread::scope` · 05 `mpsc` channel · 06 多個生產者 · 07 `Mutex` · 08 `Arc` · 09 `Arc<Mutex<T>>` · 10 `RwLock` · 11 `Send` 與 `Sync` · 12 atomic · 13 `OnceLock` 與 `LazyLock` · 14 死結 · 15 data race 為什麼編譯不過（跟 Go 的 `-race` 比一比）· 16 `sync_channel` 與背壓 · 17 worker pool

## 第 15 章 async
01 為什麼需要 async（跟 goroutine 比一比）· 02 `async fn` 與 `.await` · 03 `Future` 是惰性的 · 04 tokio 與 `#[tokio::main]` · 05 `tokio::spawn` · 06 `JoinSet` · 07 `tokio::sync::mpsc` · 08 `select!` · 09 `timeout` · 10 `CancellationToken`（跟 Go 的 context 比一比）· 11 async 裡的 `Mutex` · 12 不要阻塞 runtime：`spawn_blocking` · 13 async 閉包 · 14 跨 `.await` 的 `Send` 限制

## 第 16 章 進階語言功能
01 巨集是什麼（揭曉 `println!`）· 02 `macro_rules!` · 03 重複 `$(...)*` · 04 程序巨集與 derive 簡介 · 05 `unsafe` · 06 原始指標 · 07 FFI 與 `unsafe extern "C"` · 08 const 泛型 · 09 `const fn` · 10 `#[cfg]` 條件編譯 · 11 `build.rs` 簡介 · 12 `include_str!`（跟 Go 的 `//go:embed` 比一比）

## 第 17 章 進階 std 與常用 crate
01 `Read` 與 `Write` · 02 `BufRead` 與 `BufReader` · 03 `std::fs` 檔案操作 · 04 `Path` 與 `PathBuf` · 05 `std::env` · 06 `std::time` · 07 `std::process::Command` · 08 `serde` 與 `serde_json` · 09 derive `Serialize` / `Deserialize` · 10 `tracing` · 11 `rand` · 12 `uuid` · 13 `thiserror` · 14 `anyhow`

## 第 18 章 實戰：CLI 工具
01 `std::env::args` · 02 `clap` derive · 03 子命令 · 04 結束碼 `ExitCode` · 05 stdin/stdout 管線 · 06 走訪目錄 · 07 處理 Ctrl-C · 08 完整範例

## 第 19 章 實戰：Web 服務
01 第一個 axum 伺服器 · 02 路由 · 03 extractor：`Path` 與 `Query` · 04 JSON API · 05 共享狀態 `State` · 06 錯誤回應 `IntoResponse` · 07 middleware（tower）· 08 graceful shutdown · 09 用 `oneshot` 測試

## 第 20 章 實戰：SQLite
01 `Connection::open` 與 bundled · 02 `execute` · 03 `query_row` · 04 `prepare` 與 `query_map` · 05 參數綁定 · 06 交易 · 07 `NULL` 與 `Option` · 08 在 axum 裡使用

## 附錄二
a 記憶體配置與 `size_of` · b 零成本抽象 · c trait 物件 vs 泛型的取捨 · d `Pin` 初探 · e 效能分析工具簡介
