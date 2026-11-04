# 寫作風格指南

撰寫或修改 `zh-TW/src/` 內容前請先讀完。名詞定義與譯名見根目錄 `GLOSSARY.md`，全書目錄見 `docs/outline.md`。

## 讀者與語氣

- 讀者**會寫 Go，但沒碰過 Rust**。不用解釋什麼是變數、迴圈、函式；要解釋的是 Rust 跟 Go 不一樣的地方。
- 繁體中文原生撰寫（台灣用語：程式、函式、變數、物件、預設、檔案、介面、迴圈、陣列、記憶體），不是翻譯腔。
- 口語、友善、短句，用「我們」「你」。不要說「這很簡單」「顯然」「不過如此」。
- 不寫成維基百科或規格書。先講直覺，再上程式碼。
- 中英文之間加半形空格；程式碼、識別字、指令用反引號。
- 原創撰寫。不參考、不改寫、不翻譯其他教學的文字，包括 The Rust Programming Language、Rust by Example，以及其他同名的 Rust 教學。

## 每一集的格式

檔名 `##_slug.md`（附錄 `a_slug.md`），標題與 `SUMMARY.md` 的標題一致：

```markdown
# <與 SUMMARY 相同的標題>

## 本集目標

一兩句話：讀完這集你會什麼。

## 正文

用 `###` 小標分段。每段一個重點，短範例 + 白話解說。

### 跟 Go 比一比（選用）

## 重點整理

- 3～5 條，條列本集真正教了什麼（AI 助教靠這段判斷進度）。
```

- 一集只教**一個**小概念。篇幅大約 40～150 行 markdown；第 1 章偏短。
- 每章的 `README.md` 只有 `# <章名>` 加一兩段導言，說明這章要學什麼、為什麼重要，不列目錄。

### 跟 Go 比一比

- 只有在 Rust 跟 Go **不一樣、而且會 Go 的人容易踩坑**時才加，放在正文最後一個小節。跟 Go 一樣的地方，正文裡一句話帶過就好。
- 可以放一小段 Go 程式碼對照，用 ```` ```go ```` 區塊（不會被驗證，但要是正確的 Go 1.27）。
- 重點是「Go 的直覺在這裡會錯在哪」，不是比較哪個語言比較好。

## 進度邊界（最重要）

- 範例與說明**只能使用 `SUMMARY.md` 中排在本集之前已教過的概念**，加上下方「固定句型」。
- 不得不提到後面的內容時，用一句話帶過並標明「第 X 章會正式介紹」，不展開。
- 不要「順便」介紹本集主題以外的語法。
- 讀者會 Go，所以「迴圈」「函式」這些**概念**不用教，但 Rust 的**語法**仍然要照進度：例如第 6 章之前不能用 `match`。
- `docs/outline.md` 各章下方的「邊界」是額外規定，寫該章前先看。

## 固定句型

以下寫法在正式解釋之前就可以照抄使用，結構保持不變，只改變數名稱與文字：

| 首次出現 | 固定句型 | 正式解釋 |
| --- | --- | --- |
| 第 1 章第 2 集 | `cargo new <名稱>`、`cargo run`；程式骨架 `fn main() { ... }` | 函式第 2 章；Cargo 第 12 章 |
| 第 1 章第 2 集 | `println!("文字")`、`println!("{x}")`、`println!("{}", 運算式)` | 巨集第 16 章 |
| 第 1 章第 9 集 | 讀一行輸入：見下方 | `&mut` 第 4 章；方法第 5 章；`Result` 第 9 章 |
| 第 1 章第 10 集 | 文字轉數字：見下方 | `FromStr` 第 8 章；`expect` 第 9 章 |
| 第 1 章第 14 集 | `for i in 0..n`、`for i in 0..=n` | `Iterator` 第 11 章 |
| 第 1 章第 15 集 | 型別的最小、最大值 `i32::MIN`、`i32::MAX`（`u8::MAX` 等） | 關聯常數第 5 章（關聯函式） |
| 第 2 章第 11 集 | `println!("{:?}", 陣列)` | `Debug` 第 8 章 |
| 第 2 章第 13 集 | `for x in 陣列` | `IntoIterator` 第 11 章 |
| 第 2 章第 15 集 | 方法呼叫 `s.len()`、`s.push_str("...")`；`String::from("...")` | 方法第 5 章 |
| 第 5 章第 6 集 | `#[derive(Debug)]`、`{:?}`、`{:#?}` | `derive` 第 8 章 |
| 第 7 章第 7 集 | `format!(...)` | 巨集第 16 章 |

讀一行輸入（每次要讀一行就重複這三行）：

```rust,ignore
let mut line = String::new();
std::io::stdin().read_line(&mut line).expect("讀取失敗");
let line = line.trim();
```

文字轉數字（輸入不是整數時，程式會 panic 並以結束碼 101 結束）：

```rust,ignore
let n: i32 = line.parse().expect("請輸入整數");
```

`: i32` 的型別標註在第 1 章第 3 集就出現過；第 15 集之前，轉換的型別只用整數型別（`i32`、`i64`），第 15 集起才可換成 `u8`、`usize`、`f64` 等。

## 程式碼區塊

- 一律 `rustfmt`（edition 2024）格式，縮排 4 個空格。
- ```` ```rust ```` 區塊必須是**完整可編譯的程式**（有 `fn main`），會被 `tools/checkcode` 驗證：
  - `rustfmt --check` 通過；
  - `clippy-driver -D warnings` 通過：rustc 警告（`unused_variables`、`dead_code`…）與 clippy 警告都算失敗。初學寫法常見、刻意放行的 clippy lint 列在 `tools/checkcode/clippy-allow.txt`；
  - 執行必須以結束碼 0 結束，30 秒內。
- 需要特殊處理時，在 info string 加標註（逗號分隔，值裡面不能有空白）：

| 標註 | 用在 |
| --- | --- |
| `rust,compile_fail=E0382` | 故意展示編譯錯誤，錯誤碼必須相符。只有錯誤沒有錯誤碼時（例如語法錯誤）才寫不帶碼的 `compile_fail` |
| `rust,stdin=Andy,noplayground` / `rust,stdin=3\n5,noplayground` | 讀輸入的範例（`\n` 代表換行）；一定要同時加 `noplayground` |
| `rust,exit=101` | 故意 panic（Rust 的 panic 結束碼是 101） |
| `rust,no_run` | 會一直執行或需要外部環境，只編譯（mdBook 也會隱藏執行按鈕） |
| `rust,warn` | 故意展示編譯器警告，只有這種區塊允許警告 |
| `rust,test` | 用 `--test` 編譯並執行裡面的 `#[test]`（第 12 章起） |
| `rust,noplayground` | 不顯示執行按鈕（Playground 沒有的 crate、讀 stdin） |
| `rust,ignore` | 不完整的片段、多檔案範例 |

- 不使用 mdBook 的隱藏行（`# ` 開頭的行），程式碼全部給讀者看。
- 輸出區塊前一行固定寫「執行結果：」（輸出不固定時寫「執行結果（某一次）：」），編譯錯誤寫「編譯錯誤：」，編譯警告寫「編譯警告：」。讀 stdin 的範例要讓讀者知道輸入了什麼，標籤寫成「輸入 `Andy` 的執行結果：」（有好幾行輸入時寫「輸入 `小明` 和 `台北` 的執行結果：」）。
- 程式輸出用 ```` ```text ```` 區塊呈現，**必須與實際執行結果完全一致**（自己跑過再貼）。輸出順序不固定的（`HashMap` 走訪、執行緒），要在文中說明。
- 「執行結果」只放程式自己印出來的內容；示範 panic 時，連同 panic 訊息一起貼（stdout 在前、stderr 在後）。panic 訊息的第一行 `thread 'main' (數字) panicked at src/main.rs:行:欄:` 裡，括號中的數字每次執行都不同，所以這種區塊一律寫「執行結果（某一次）：」，並在文中說明。行號與欄號以讀者照抄、放進 `cargo new` 專案的 `src/main.rs` 為準。
- 展示編譯錯誤或警告時，訊息用 ```` ```text ```` 貼出 Rust 1.99 實際的訊息（路徑統一寫 `src/main.rs`，結尾的 `error: aborting due to…` 與 `For more information…` 可以省略），通常只貼 `error[E....]` 那幾行和箭頭指的位置就好；只貼前面幾行時，要在文中說明後面還有補充。
- 終端機指令用 ```` ```bash ````，`Cargo.toml` 內容用 ```` ```toml ````（這兩種不會被驗證）。
- 第一部只用 std。第二部用到的 crate 版本統一鎖在 checkcode 的共用 `Cargo.toml`，不得自行加 crate。
- 不要用 `ignore` 逃避驗證：能寫成完整程式就寫成完整程式。
- 浮點數範例避開 3.14、2.718 這類常數（clippy `approx_constant` 會直接報錯），改用 2.5、1.5 之類的值。
- `#[derive(Debug)]` 的 struct，每個欄位都要在程式裡被讀到（印出整個 struct 不算），否則 rustc 會報 `dead_code`。
- 中文一個字算兩欄寬，`println!` 的字串太長會被 rustfmt 拆成多行；訊息寫短一點。

## 正確性

- 以 Rust 1.99、edition 2024 為準。let chains（1.88）、async 閉包（1.85）都已穩定；`static mut` 取參照在 edition 2024 是錯誤；`extern` 區塊要寫 `unsafe extern "C"`，`no_mangle` 要寫 `#[unsafe(no_mangle)]`。
- 本機工具鏈若不是 1.99，用到 1.97 之後才有的功能前，先確認 CI 能過，不要憑印象。
- 不確定時看官方文件（`rustup doc --std`）或實際寫程式驗證。
- 完成後執行 `cargo run --manifest-path tools/checkcode/Cargo.toml -- zh-TW/src/<章節目錄>`，全部通過才算完成。
