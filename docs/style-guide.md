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
- 可以放一小段 Go 程式碼對照，用 ```` ```go ```` 區塊（不會被驗證，但要是正確的 Go 1.27，縮排用 Tab，跟 `gofmt` 一致）。
- 重點是「Go 的直覺在這裡會錯在哪」，不是比較哪個語言比較好。

## 進度邊界（最重要）

- 範例與說明**只能使用 `SUMMARY.md` 中排在本集之前已教過的概念**，加上下方「固定句型」。
- 不得不提到後面的內容時，用一句話帶過並標明「第 X 章會正式介紹」，不展開。
- 不要「順便」介紹本集主題以外的語法。
- 讀者會 Go，所以「迴圈」「函式」這些**概念**不用教，但 Rust 的**語法**仍然要照進度：例如第 6 章之前不能用 `match`。
- `docs/outline.md` 各章下方的「邊界」是額外規定，寫該章前先看。
- **`Self` 從附錄一 e 起才能用。** 第 1～12 章與附錄一 a～d 的 `impl` 裡一律寫完整型別名稱（唯一的例外是第 8 章第 17 集、第 11 章第 7 集 trait 定義裡的 `Self::Item`，文中一句話帶過）。附錄一 e 之後（e 本身、f～j 與第二部全部）可以用 `Self`，也可以繼續寫完整名稱：兩種寫法編譯結果相同，clippy 的 `use_self` 預設關閉，所以混用不會被擋，不強制改寫。已經寫好的第 1～12 章不回頭改。

## 固定句型

以下寫法在正式解釋之前就可以照抄使用，結構保持不變，只改變數名稱與文字：

| 首次出現 | 固定句型 | 正式解釋 |
| --- | --- | --- |
| 第 1 章第 2 集 | `cargo new <名稱>`、`cargo run`；程式骨架 `fn main() { ... }` | 函式第 2 章；Cargo 第 12 章第 1 集 |
| 第 1 章第 2 集 | `println!("文字")`、`println!("{x}")`、`println!("{}", 運算式)` | 巨集第 16 章第 1 集 |
| 第 1 章第 9 集 | 讀一行輸入：見下方 | `&mut` 第 4 章；方法第 5 章；`Result` 第 9 章第 4 集；`stdin`、`read_line` 與緩衝第 17 章第 2 集 |
| 第 1 章第 10 集 | 文字轉數字：見下方 | `FromStr` 第 8 章；`expect` 第 9 章第 4 集 |
| 第 1 章第 14 集 | `for i in 0..n`、`for i in 0..=n` | `Iterator` 第 11 章第 8 集 |
| 第 1 章第 15 集 | 型別的最小、最大值 `i32::MIN`、`i32::MAX`（`u8::MAX` 等） | 關聯常數第 5 章（關聯函式） |
| 第 2 章第 11 集 | `println!("{:?}", 陣列)`（tuple、`()` 也能這樣印；也可寫成 `{x:?}`） | `Debug` 第 8 章 |
| 第 2 章第 13 集 | `for x in 陣列` | `IntoIterator` 第 11 章第 8 集 |
| 第 2 章第 15 集 | 方法呼叫 `s.len()`（`String` 與陣列都能用）、`s.push_str("...")`；`String::from("...")` | 方法第 5 章 |
| 第 3 章第 7 集 | 方法呼叫 `s.clone()`（寫法同上） | 方法第 5 章；`Clone` 第 8 章 |
| 第 4 章第 1 集 | 對參照呼叫方法：`r.len()`、`r.push_str("...")`（`r` 是 `&mut String`），`&str` 與 slice 的 `len` 也一樣；第 11 集起 `{:?}` 也能印 slice | 自動參照與解參照第 5 章第 10 集；`Debug` 第 8 章 |
| 第 5 章第 6 集 | `#[derive(Debug)]`、`{:?}`、`{:#?}` | `derive` 第 8 章 |
| 第 6 章第 14 集 | `matches!(值, 模式)` | 巨集第 16 章第 1 集 |
| 第 7 章第 1 集 | `vec![...]`；`{:?}` 也能印 `Vec`（`HashMap`、`HashSet`、`BTreeMap` 也能這樣印，但 `HashMap`、`HashSet` 的順序不固定） | 巨集第 16 章第 1 集；`Debug` 第 8 章 |
| 第 7 章第 7 集 | `format!(...)` | 巨集第 16 章第 1 集 |
| 第 7 章第 9 集 | `for` 後面直接放回傳迭代器的方法呼叫：`for c in s.chars()`、`for b in s.bytes()`、`for part in s.split(',')`、`for k in map.keys()`（不接 `.count()`、`.nth()`、`.rev()` 等迭代器的方法） | `Iterator` 第 11 章第 8 集 |
| 第 7 章第 11 集 | `use std::collections::HashMap;`（`HashSet`、`BTreeMap` 同） | `use` 第 12 章第 5 集 |
| 第 8 章第 11 集 | `impl std::fmt::Display for 型別 { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "...", ...) } }`（骨架照抄，只改型別名稱與 `write!` 的內容；enum 搭配 `match self`，每個分支一個 `write!`）；實作 `Display` 之後型別自動有 `.to_string()`；帶生命週期的 struct 寫成 `impl std::fmt::Display for Receipt<'_>`，第 10 章第 6 集起 `Formatter` 可以寫完整的 `std::fmt::Formatter<'_>` | `Result` 第 9 章第 8 集；`write!` 巨集第 16 章第 1 集；`'_` 第 10 章第 5、6 集 |
| 第 8 章第 15 集 | `Box<dyn Trait>`、`Box::new(...)`（照寫，不解釋 `Box`；第 11 章第 6 集起 `Box<dyn Fn(u32) -> u32>` 同樣照寫） | `Box` 第 13 章第 1 集 |
| 第 8 章第 17 集 | trait 定義裡的 `Self::Item`（只出現在 trait 定義；`impl` 裡仍寫完整型別） | `Self` 附錄一 e |
| 第 12 章第 2 集 | 函式前面的 `pub`（開放給別的 crate 使用，照抄） | `pub` 第 12 章第 4 集 |
| 第 12 章第 10 集 | `#[cfg(test)]`，只加在 `mod tests { use super::*; ... }` 上，照抄 | `cfg` 第 16 章第 10 集 |
| 第 12 章第 15 集 | `#[cfg(feature = "名字")]`，照抄 | `cfg` 第 16 章第 10 集 |
| 第 14 章第 1 集 | `thread::sleep(Duration::from_millis(n))`（睡 `n` 毫秒，搭配檔案最上面的 `use std::thread;`、`use std::time::Duration;`，照抄） | `Duration` 第 17 章第 6 集 |
| 第 15 章第 2 集 | `#[tokio::main]` 加 `async fn main() { ... }`（照抄）；`tokio::time::sleep(Duration::from_millis(n)).await`（async 版的睡眠，搭配 `use std::time::Duration;`、`use tokio::time::sleep;`） | `#[tokio::main]` 第 15 章第 4 集（`.await` 與 `sleep` 在第 2 集就說明） |

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
| `rust,test,noplayground` | 用 `--test` 編譯並執行裡面的 `#[test]`（第 12 章起）。Playground 只會執行 `main`，所以一律加 `noplayground`；不需要 `fn main`；要展示失敗的測試，再加 `exit=101` |
| `rust,noplayground` | 不顯示執行按鈕（Playground 沒有的 crate、讀 stdin） |
| `rust,ignore` | 不單獨驗證的區塊：不完整的片段、刻意排版亂掉的示範，以及多檔案專案裡的檔案（第 12 章起）。前面一行有路徑標籤的，改在專案裡驗證，見下一節「多檔案專案」；沒有標籤的就是完全不驗證，能寫成完整程式就不要用 |

- 不使用 mdBook 的隱藏行（`# ` 開頭的行），程式碼全部給讀者看。
- 輸出區塊前一行固定寫「執行結果：」（輸出不固定時寫「執行結果（某一次）：」），編譯錯誤寫「編譯錯誤：」，編譯警告寫「編譯警告：」。讀 stdin 的範例要讓讀者知道輸入了什麼，標籤寫成「輸入 `Andy` 的執行結果：」（有好幾行輸入時寫「輸入 `小明` 和 `台北` 的執行結果：」）。
- 程式輸出用 ```` ```text ```` 區塊呈現，**必須與實際執行結果完全一致**（自己跑過再貼）。輸出順序不固定的（`HashMap` 走訪、執行緒），要在文中說明。
- 「執行結果」只放程式自己印出來的內容；示範 panic 時，連同 panic 訊息一起貼（stdout 在前、stderr 在後）。panic 訊息的第一行 `thread 'main' (數字) panicked at src/main.rs:行:欄:` 裡，括號中的數字每次執行都不同，所以這種區塊一律寫「執行結果（某一次）：」，並在文中說明。行號與欄號以讀者照抄、放進 `cargo new` 專案的 `src/main.rs` 為準。
- 展示編譯錯誤或警告時，訊息用 ```` ```text ```` 貼出 Rust 1.99 實際的訊息（路徑統一寫 `src/main.rs`，結尾的 `error: aborting due to…` 與 `For more information…` 可以省略），通常只貼 `error[E....]` 那幾行和箭頭指的位置就好；只貼前面幾行時，要在文中說明後面還有補充。
- 終端機指令用 ```` ```bash ````，`Cargo.toml` 內容用 ```` ```toml ````（這兩種不會被驗證）。
- 第一部只用 std；第二部可以用 crate，見下一節「第二部使用 crate」。
- 不要用 `ignore` 逃避驗證：能寫成完整程式就寫成完整程式。
- 浮點數範例避開 3.14、2.718 這類常數（clippy `approx_constant` 會直接報錯），改用 2.5、1.5 之類的值。
- `#[derive(Debug)]` 的 struct，每個欄位都要在程式裡被讀到（印出整個 struct 不算），否則 rustc 會報 `dead_code`。
- 中文一個字算兩欄寬，`println!` 的字串太長會被 rustfmt 拆成多行；訊息寫短一點。
- `push_str` 的參數至少要兩個字元：`s.push_str("！")` 這種只有一個字元的字串，會被 clippy `single_char_add_str` 擋下，而 `push` 要到第 7 章第 7 集才教，那之後單一字元用 `push`。
- 記憶體、所有權的示意圖放在 ```` ```text ```` 區塊，只用 ASCII 符號（`[ ]`、`--->`）加中文標註，不用方框線字元（中英文字寬不同，對不齊）。示意圖不是程式輸出，前面不寫「執行結果：」。
- 概念用語 move、drop 保留英文、不加反引號；程式裡真的會寫出來的名稱（`.clone()`、`Copy`、`Drop`）才加反引號。
- 借用相關的範例：
  - 第 4 章第 10 集起，只需要讀的字串參數一律寫 `&str`，不寫 `&String`（clippy `ptr_arg` 放行，只是讓第 2～9 集教 `&String` 用）；要改動的才寫 `&mut String`。
  - 第 7 章第 4 集之前，不寫 `for x in &a`、`for x in slice` 這種走訪參照的迴圈，改用 `for i in 0..a.len()` 加索引（clippy `needless_range_loop` 已放行）。
  - 回傳參照的函式只放一個參照參數（第 4、5 章，第 10 章第 2 集起沒有這個限制）；示範 E0106 時，用沒有參數的 `fn dangle() -> &String`。
  - 展示借用錯誤的 `compile_fail` 區塊，要讓參照在衝突「之後」還被用到，否則 NLL 會讓它編譯成功。
  - 展示錯誤時，盡量別讓訊息夾帶無關的警告：變數沒被改到就不要寫 `mut`（`unused_mut`），被指定的值要讀過一次（`unused_assignments`），貼出來的訊息才乾淨。
- struct 與方法相關的範例：
  - 附錄一 e 才教 `Self`，`impl` 裡的型別一律寫完整名稱：`fn new(owner: String) -> Account`、`Account { owner, balance: 0 }`。
  - struct 的欄位只放擁有的型別（`String`、數字、別的 struct），不放參照；示範 E0106 時才寫 `name: &str`（第 10 章第 5 集起可以放參照，見「生命週期相關的範例」）。
  - `rust` 區塊裡不要寫 `Member { name: name }` 這種多餘的欄位名稱（clippy `redundant_field_names` 不在放行清單）；要對照時只用行內程式碼。
  - 方法不要取名 `to_string`（clippy `inherent_to_string`）；`is_` 開頭的方法要用 `&self`（`wrong_self_convention`）。
  - struct 建立式的大括號內容超過約 18 欄寬，rustfmt 就會拆成每個欄位一行（像 `Account { balance: 500 }` 這麼短才能留在同一行）；`println!` 的引數（含格式字串）合計超過約 60 欄寬也會被拆開，改成兩行 `println!`。
- enum 與模式相關的範例：
  - variant 帶的資料要被 `match` 或 `if let` 讀出來；`#[derive(Debug)]` 印出來不算讀過，沒讀到的欄位 rustc 會報 `dead_code`（跟 struct 欄位同一條規則）。第 6 章第 2 集刻意用 `rust,warn` 區塊展示這個警告：「編譯警告：」區塊只接受整段貼出（fill.py 不收節錄），所以那個範例只放兩個帶資料的 variant，警告才不會太長。
  - 每個 variant 都要在程式裡建立過（例如 `for x in [A, B, C]`），否則會有 `variant never constructed`；不要用 `let _ = Status::Paid;` 這種湊數的寫法。
  - 函式不要回傳字串字面值（`-> &str` 沒有參照參數可以借，要 `'static`，第 10 章第 7 集才教）。要把 enum 轉成文字，在 `match` 分支裡直接 `println!`，或在 `main` 裡寫 `let text = match ... { ... => "文字" };`。
  - clippy 會擋下「有更短寫法」的程式：`matches!(x, None)`、`matches!(x, Some(_))`（`redundant_pattern_matching`）；`matches!(c, 'a'..='z')`（`manual_is_ascii_check`）；連續整數的 `1 | 2 | 3 | 4`（`manual_range_patterns`，改用不連續的數字，或直接寫範圍）；`if let` 裡面再包一層沒有 `else` 的 `if`（`collapsible_if`，edition 2024）；`Some(x) => Some(f(x)), None => None`（`manual_map`）；`Some(v) => v, None => 預設值`（`manual_unwrap_or` 或 `manual_unwrap_or_default`）。展示巢狀 `if let` 時，內外層都要有 `else`。
  - `match x { Some(p) => ..., _ => {} }` 沒問題（`single_match` 在放行清單），可以拿來引出 `if let`。
  - rustfmt 對簡單的 `if a { x } else { y }` 沒有固定規則：函式本體或 `let` 右邊放得下就會摺成一行（`else if` 串不會），`match` 分支的區塊裡則一律拆成多行；let chains 放不下一行時，排成 `if let ... ⏎ && 條件 ⏎ {`。先照 checkcode 的結果寫。
  - E0004 的訊息（尤其 `Option`）會夾帶 `/rustc/<雜湊>/library/...` 路徑，只貼前 6 到 7 行（到 `|` 或 `= note: the matched value` 為止），並在文中說明後面還有 `note`、`help`。節錄後要用 `fill.py --check` 確認，不要再用 `--fill` 蓋回去。
- 集合相關的範例：
  - 第 7 章第 6 集起，只讀的 `Vec` 參數寫 `&[T]`，不寫 `&Vec<T>`（clippy `ptr_arg` 放行，只是讓第 6 集示範 E0308 用）；要 `push`、`pop` 才寫 `&mut Vec<T>`。第 6 集之前沒有把 `Vec` 傳進函式的範例。
  - clippy 會擋下這些寫法：`let v = vec![...]` 只被讀、索引、`get`、`for x in &v` 或傳成 `&[T]`（`useless_vec`，要不改成陣列，要不讓範例真的 `push` 過）；`v.get(0)`（`get_first`，改用 `first()` 或換成別的索引）；`for (_, v) in &map`（`for_kv_map`，改用 `values()`）；`v.len() == 0`（`len_zero`，用 `is_empty()`）；`loop { match v.pop() { Some(x) => ..., None => break } }`（`while_let_loop`，直接寫 `while let`）；單獨一個 `format!("{s}")`（`useless_format`）。
  - 第 7 章第 9 集起，`for` 後面可以直接放回傳迭代器的方法呼叫（`s.chars()`、`s.bytes()`、`s.split(',')`、`map.keys()`、`map.values()`），不展開原理（`Iterator` 第 11 章第 8 集）。第 11 章之前不要接 `.count()`、`.nth()`、`.rev()`、`.enumerate()`、`.collect()` 這些迭代器的方法：數字元就用 `for` 加累加變數。
  - `HashMap`、`HashSet` 的走訪順序每次執行都不同，貼了整份輸出的話，`fill.py --check` 會時好時壞。驗證過的範例只印跟順序無關的結果（`get`、`len`、加總、計數）；要印整個 map，就只放一組，或改用 `BTreeMap`。需要展示「順序會變」時，輸出標籤寫成「某一次的輸出（順序每次可能不同）：」，fill.py 不認得這個標籤，所以不會比對；整本書只有第 7 章第 14 集與第 14 章第 1 集（執行緒交錯）這樣用，並在文中說明順序不固定。
  - 第 7 章第 11 集起，`use std::collections::HashMap;` 這類固定句型放在檔案最上面、`fn main` 之前。
- trait 與泛型相關的範例：
  - 領域沿用飲料店：`Tea`、`Coffee`、`Water`、`Cookie`，trait 叫 `Priced`、`Named`，泛型容器叫 `Shelf<T>`，金額是 `Money`。避開 The Rust Book 的 `Summary`、`Draw`、`largest`、`Point<T>`、`Wrapper`。
  - `Self` 附錄一 e 才教，`impl` 裡一律寫完整型別名稱：`fn from(name: &str) -> Member`、`fn add(self, other: Money) -> Money`、`fn new() -> Shelf<T>`。唯一的例外是第 8 章第 17 集：trait 定義裡的 `Self::Item`（`type Item;` 一定要這樣引用），文中一句話帶過「`Self` 代表實作這個 trait 的型別，附錄一 e 才完整介紹」，`impl` 區塊裡仍然寫完整型別。
  - `Result`、`Ok`、`Err` 第 9 章才教，第 8 章只出現三處：`impl Display` 骨架裡的 `std::fmt::Result`（固定句型）、`parse` 的結果（照舊用 `.expect()`）、第 14 集用 `{:?}` 印出 `parse` 的結果來觀察。不自己回傳 `Result`，所以第 14 集不實作 `FromStr`（第 9 章第 7 集才實作）。
  - 路徑一律寫完整：`std::fmt::Display`、`std::fmt::Formatter`、`std::ops::Add`、`std::str::FromStr`、`std::cmp::Ordering`；`use` 只有第 7 章教的 `use std::collections::HashMap;` 一類。`Formatter` 在第 10 章第 6 集之前不寫 `<'_>`（生命週期第 10 章，沒有也能通過 clippy）。`Ordering` 只用 `{:?}` 印，不在程式裡寫出它的路徑。
  - 第 8 章第 2 集的 `&impl Priced` 比第 4 集早出現，用一句話帶過（「現在讀成任何實作了該 trait 的型別的參照」）；第 15 集的 `Box<dyn Trait>`、`Box::new(...)` 同樣只當作照抄的寫法。
  - 只實作 `From` 與 `Display`，不手寫 `Into`、`ToString`（clippy `from_over_into`、`to_string_trait_impl`）；`.into()` 要有型別標註，否則是 E0283。`.to_string()` 不要直接寫在 `println!` 的引數裡（`to_string_in_format_args`），先 `let` 起來。
  - 手寫的 `Default`、`Clone` 如果只是「每個欄位取預設值／複製」，會被 `derivable_impls` 擋下，要用 `derive`。`derive` 與手寫不要混用在同一組成對的 trait 上（例如 `derive(PartialOrd)` 加手寫 `Ord`，`derive_ord_xor_partial_ord`）。
  - 其他會被 clippy 擋下的寫法：對 `Copy` 型別呼叫 `.clone()`（`clone_on_copy`）、`&a == &b`（`op_ref`）、`String::from(字串變數)` 與 `i64::from(i64 的值)`（`useless_conversion`）、`Marker::default()` 用在 unit struct（`default_constructed_unit_structs`）、`impl Add` 的本體裡用 `-`（`suspicious_arithmetic_impl`）。
  - dead_code：每個 trait、每個 trait 方法（包含有預設本體的）都要在程式裡被呼叫過。欄位只被 `derive(Debug)`、`derive(Clone)`、`derive(Default)` 用到不算讀過；會比較欄位的 derive（`PartialEq`、`Hash`、`PartialOrd`）算讀過。
  - 編譯錯誤訊息裡的 `help` 若列出「other types implement trait」的清單（E0277 的 `FromStr`、`Eq`）、夾帶 `/rustc/<雜湊>/library/...` 路徑，或建議了還沒教的東西（`Box<dyn Trait>`、`.to_string()`），只貼前面幾行，並在文中說明後面還有 `help` 與 `note`、先不用管。
  - E0204、E0117 的訊息很短，可以整段貼；E0271、E0382（運算子）、E0308（泛型 `T`、`impl Trait`）貼前 10 到 13 行，到箭頭指的位置為止，後面的 `note` 在文中交代一句。節錄用 `fill.py --check` 驗證，不要再 `fill` 蓋回去。
- 錯誤處理相關的範例：
  - 領域沿用飲料店：`split_bill`、`order_total`、`Size`（`S` / `M` / `L`）、`SizeError`、`OrderError`、`TooMany`。避開 The Rust Book 的 `File::open`、`read_username_from_file`、`Guess`。不用 `std::fs`（第 17 章第 3 集），錯誤的來源只有 `parse`（`std::num::ParseIntError`、`std::str::ParseBoolError`）、`read_line`（`std::io::Error`）與自己定義的型別。
  - 路徑照樣寫完整：`std::error::Error`、`std::str::FromStr`、`std::num::ParseIntError`、`Box<dyn std::error::Error>`；沒有新的 `use`。`impl` 裡不寫 `Self`：`fn from_str(text: &str) -> Result<Size, SizeError>`，`type Err = SizeError;`。
  - 第 11 章第 18 集之前不用 `Option` / `Result` 的方法（`unwrap_or`、`ok_or`、`map_err`、`is_ok`、`is_some`、`ok()`），只有 `unwrap`、`expect`（第 9 章第 4 集）；要把 `Option` 轉成 `Result`，用 `match`；要給預設值，`match` 的 `Err` 分支寫成區塊（先印提醒再回傳預設值）。
  - clippy 會擋下這些寫法：`match r { Ok(v) => v, Err(e) => return Err(e) }`、`if let Err(e) = r { return Err(e); }`、對 `Option` 的 `let Some(x) = o else { return None; }`（`question_mark`）；展示「`?` 之前」的寫法時，用兩層巢狀 `match`，內層與外層的 `Err(e) => Err(e)` 不會被擋。`Ok(v) => v, Err(_) => 0`（`manual_unwrap_or`）；`match r { Ok(_) => true, Err(_) => false }` 與 `matches!(r, Ok(_))`（`redundant_pattern_matching`）；`Some(5).unwrap()`、對字面值 `Ok(...)` 呼叫 `unwrap`（`unnecessary_literal_unwrap`，`"42".parse().unwrap()` 不會）；`Ok(x?)`（`needless_question_mark`）；`if let Some(a) = m.get(k) { Some(*a + 1) } else { None }`（`manual_map`）。
  - 呼叫回傳 `Result` 的函式卻不處理，是 rustc 的 `unused_must_use`，要用 `rust,warn` 展示，並整段貼出警告。
  - 輸出標籤：panic（`exit=101`）寫「執行結果（某一次）：」；`main` 回傳 `Err` 時結束碼是 1（`exit=1`），輸出是 `Error: ` 加 `Debug` 格式，沒有執行緒編號，寫「執行結果：」（stdin 版本寫「輸入 `abc` 的執行結果：」）。
  - 展示 `?` 的 E0277（`couldn't convert the error`）要寫 `qty.parse::<u32>()?`；寫成 `let qty: u32 = qty.parse()?;` 而且回傳型別是自訂 enum、這個 enum 還沒有任何自己寫的 `From`（只剩「任何型別都能轉成自己」那個）時，編譯器會推論成 E0271（`type mismatch resolving`），訊息不一樣。
  - 錯誤型別一律 `#[derive(Debug)]`（`.expect()`、`.unwrap()` 要 `Debug`，缺的話是 E0277）；variant 帶的資料要被 `match` 或 `Display` 讀出來，`derive(Debug)` 不算讀過。
  - `impl std::error::Error for 型別 {}` 的大括號留空，不寫 `source`、不提過時的 `description`；E0277 的 `note` 若夾帶 `/rustc/<雜湊>/library/...` 路徑，只貼到 `note:` 那一行為止。

- 生命週期相關的範例：
  - 領域沿用飲料店：`name_or`、`first_part`、`tail`、`Receipt`（`customer`、`drink`）、`Status`、`Drink`。避開 The Rust Book 的 `longest`、`ImportantExcerpt`、`first_word`。
  - clippy `needless_lifetimes` 不在放行清單：只有一個參照參數、省略規則補得出來的函式，`rust` 區塊裡不能寫 `<'a>`（`<'a, 'b>` 裡只用一次的名字也會被擋）。要示範「展開之後的樣子」，用 ```` ```text ```` 區塊（第 4 集）或行內程式碼；真的要寫標註的範例，必須有兩個以上的參照參數，而且回傳值只綁其中一部分（`name_or<'a>(name: &'a str, fallback: &'a str)`、`first_part<'a>(line: &'a str, sep: &str)`）。
  - `const`、`static` 裡不要寫 `&'static str`，參照預設就是 `'static`（clippy `redundant_static_lifetimes`）；函式回傳型別與 struct 欄位可以寫。
  - 簽名裡出現帶生命週期的 struct，一律寫 `Receipt<'_>`（`-> Option<Receipt<'_>>`）：省略不寫，rustc 1.99 預設會警告 `mismatched_lifetime_syntaxes`，checkcode 會判失敗。`impl` 寫 `impl Receipt<'_>`；方法簽名要用到 `'a` 時才寫 `impl<'a> Receipt<'a>`（clippy 不會抱怨）。第 10 章第 6 集起 `Display` 骨架可以寫完整的 `std::fmt::Formatter<'_>`，之前的章節維持原本的骨架。
  - 示範 E0597 時，結果要在內層區塊結束「之後」還被用到，否則 NLL 會讓它編譯成功。內層區塊寫成 `let r = { let s = ...; 借用 s 的運算式 };` 這種區塊值的形式，不寫先宣告後賦值的 `let r;`（第 1 章第 5 集說本書不用它，clippy `needless_late_init` 也會擋）。不要同時示範好幾個不同的錯誤。
  - 1.99 的訊息形狀：兩個參照參數回傳參照是 E0106，`help` 只建議 `<'a>`（`'a` 標在全部參照上）；沒有參照參數的 `-> &str` 是 E0106，`help` 先建議 `'static`、再建議回傳擁有的值；標了 `'a` 或 `'static` 卻回傳區域變數的參照是 E0515（`cannot return reference to local variable`，沒有 `help`）；回傳了沒有綁進 `'a` 的參數是 E0621，`help` 建議替它標 `'a`；`impl Receipt`（缺 `<'_>`）是 E0726，`help` 建議 `<'_>`；呼叫端結果超出某個輸入的範圍是 E0597。這些訊息都很短，整段貼出。
  - 進度邊界：不寫生命週期 bound（`'a: 'b`、`T: 'a`、`T: 'static`）、`for<'a>`、`dyn Trait + 'a`；不用 `Self`、閉包與迭代器的方法。`T: 'static` 只在第 7 集用一句話指向第 14 章第 3 集。

- 閉包與迭代器相關的範例：
  - 領域沿用飲料店：`prices`、`menu`、`orders`、`Member`（`name`、`points`）、`Tickets`（取餐號碼，第 17 集沿用第 8 章第 17 集的 `Dispenser`）。避開 The Rust Book 的 `Inventory`、`Shirt`、`Rectangle`、`Counter`、`shoes_in_size`、`expensive_closure`、`add_one`。
  - 用到哪些方法，照 `SUMMARY.md` 的順序：`.iter()` 第 7 集起出現（先當成「依序借出每個元素的參照」，一句話帶過，第 8 集才細講）；`map`、`filter` 第 10 集；`collect`、`copied()`、`cloned()` 第 11 集；`sum`、`count`、`min`、`max` 第 12 集；`enumerate`、`zip`、`rev` 第 13 集；`fold` 第 14 集；`find`、`any`、`all`、`position` 第 15 集；`sort_by_key`、`sort_by` 第 16 集。第 9 集用 `map` 展示惰性，只說「下一集正式教」；第 10 集的結果用 `for` 消耗（`collect` 還沒教）。
  - `Option` / `Result` 的方法（`unwrap_or`、`ok_or`、`map_err`、`is_some`、`is_ok`、`ok()`、`map`、`and_then`…）第 18 集才教：第 7～17 集裡 `max()`、`find()`、`position()` 回傳的 `Option`，用 `match`、`if let` 或 `.expect(...)` 處理。`collect::<Result<Vec<_>, _>>()` 是 `collect` 的用法，不算這類方法，第 11 集可以出現。
  - 閉包參數的層數：`iter()` 的元素是 `&T`；`map`、`any`、`all`、`position` 的閉包拿到 `&T`（數字寫 `*p`）；`filter`、`find` 拿到 `&&T`（數字寫 `**p`，缺一個 `*` 是 E0308）。元素是 struct、`&str` 時，用 `.` 讀欄位、呼叫方法會自動解參照，不用寫 `*`。本書不寫 `|&&p|` 這種在參數裡拆參照的寫法，第 10 集只用一句話說「別人的程式碼常這樣寫，意思相同」。
  - clippy `map_clone` 會擋 `.map(|x| *x)`，要寫 `.copied()`；`.iter().copied().collect()`（中間沒有別的 adapter）被 `iter_cloned_collect` 擋，所以 `copied()` 放在 `filter` 後面示範。
  - 型別位置的 `_`（`Vec<_>`、`HashMap<_, _>`）第 11 集起可用，教法是「請編譯器自己填」；`collect`、`sum` 沒有型別標註是 E0283（`type annotations needed`）。
  - `FnMut` 的參數寫 `mut f: F`（第 2 章第 3 集教過的 `mut` 參數）；`Box<dyn Fn(u32) -> u32>` 與 `Box::new(...)` 延續第 8 章第 15 集的固定句型（第 6 集）；`std::cmp::Reverse(...)` 寫完整路徑，不加 `use`；`impl Iterator<Item = u32>` 當回傳型別（第 17 集）。
  - 不寫 `Self`：`impl Iterator for Tickets` 裡寫 `type Item = u32;` 與 `fn next(&mut self) -> Option<u32>`。第 7 集用 `text` 區塊展示標準函式庫的 `Iterator` 定義，才會出現 `Option<Self::Item>`，並用一句話指向附錄一 e（跟第 8 章第 17 集同一個例外）。
  - clippy 會擋下這些寫法（都實測過）：
    - `fold(0, |a, b| a + b)`（`unnecessary_fold`，改用 `sum`；`fold` 的範例要用 tuple、`String` 或 `f64::max` 當累積值）；
    - `.filter(..).next()`（`filter_next`，改用 `find`）；`.any(|x| *x == y)`（`manual_contains`，`any` 的條件要用比大小或方法）；
    - `.sort_by(|a, b| a.k.cmp(&b.k))` 與 `b.k.cmp(&a.k)`（`unnecessary_sort_by`，改用 `sort_by_key`，由大到小用 `std::cmp::Reverse(...)`）；整數的 `sort_by(|a, b| b.cmp(a))`、字串欄位 `a.name.cmp(&b.name)`、`partial_cmp(...).expect(...)` 不會被擋；
    - 手動計數器加 `for x in &v`（`explicit_counter_loop`）；`.nth(0)`（`iter_nth_zero`）；直接對陣列寫 `v.iter().next()`（`iter_next_slice`，先 `let mut it = v.iter();`）；`loop { match it.next() {..} }`（`while_let_loop`）。`while let Some(x) = it.next()` 沒問題（`while_let_on_iterator` 在放行清單）；
    - `|x| double(x)`、`.map_err(|e| E::Bad(e))`（`redundant_closure`，直接傳函式名稱或 tuple variant）；`(|x| x + 1)(3)`（`redundant_closure_call`）；
    - `or_insert_with(Vec::new)`（`unwrap_or_default`，改用 `or_default()`）；`unwrap_or_else(|| 0)`（`unnecessary_lazy_evaluations`）；`and_then(|x| Some(..))`（`bind_instead_of_map`）；`map_or(false, ..)`（`unnecessary_map_or`）；對字面值 `Some(5).unwrap_or(0)`（`unnecessary_literal_unwrap`）；先 `is_some()` 再 `unwrap()`（`unnecessary_unwrap`）；
    - `let v = vec![...]` 只被走訪或排序（`useless_vec`，先 `Vec::new()` 再 `push`，`vec_init_then_push` 已放行）。
  - 只串了 adapter、沒有 consumer，是 rustc 的 `unused_must_use`（`iterators are lazy and do nothing unless consumed`），第 9 集用 `rust,warn` 展示，整段貼出。
  - 1.99 的訊息形狀：閉包第二次用別的型別呼叫是 E0308（帶 `closure was earlier called with an argument of type`）；函式裡用外面的區域變數是 E0434（`help` 建議閉包）；閉包借著變數時改它是 E0506、讀它是 E0502（`borrow occurs due to use in closure`）；`move` 之後再用是 E0382（`value moved into closure here`，`help` 建議 `.clone()`）；閉包活得比變數久是 E0373（區塊版寫 `current block`，函式版寫 `current function`，`help` 直接建議 `move`）；沒有 `let mut` 就呼叫 `FnMut` 閉包是 E0596；`FnOnce` 呼叫兩次是 E0382（`closure cannot be invoked more than once`）；要求 `Fn` 卻傳會改動的閉包是 E0594（`change this to accept FnMut instead of Fn`，不是 E0525）；`if`／`else` 各回傳一個擷取了變數的閉包是 E0308（`no two closures, even if identical, have the same type`，`help` 建議 `Box<dyn Fn>`）；`collect`、`sum` 沒標型別是 E0283（訊息夾 `/rustc/<雜湊>/` 路徑，只貼前 7 行）；`Vec<u32>` 收到 `&u32` 是 E0277；`f64` 用 `max`、`sort_by_key` 是 E0277（`the trait bound f64: Ord is not satisfied`，貼前 5 行）；`sort_by_key(|m| &m.name)` 是沒有錯誤碼的 `lifetime may not live long enough`（用不帶碼的 `compile_fail`）；少寫 `type Item` 或 `next` 是 E0046；`1..=3.rev()` 是 E0689。
  - 兩個沒有擷取變數的閉包，在 `if`／`else` 裡回傳 `impl Fn` 反而編譯成功（會變成函式指標）；示範 E0308 時，兩個閉包都要擷取變數（`move |price| price - off`）。`let add_fee = |price| price + 5;` 後面 `let total: u32 = add_fee(30);` 會是 E0271（先被推成 `i32`），要先宣告 `let price: u32 = 30;` 再傳。

- 模組、Cargo 與測試相關的範例（第 12 章）：
  - 領域沿用飲料店：package 叫 `teashop`，模組 `menu`、`bill`，函式 `price`、`total_for`，`Member`（`name`、`points`）。避開 The Rust Book 的 `restaurant`、`front_of_house`、`hosting`、`Guess`、`add_two`。
  - 單檔案的 `rust` 區塊（內聯 `mod`）：模組裡的函式、型別都要加 `pub` 才叫得到；bin crate 裡 `pub` 擋不住 `dead_code`，所以每個 `pub` 項目都要在 `main` 裡被呼叫過，沒用到的 `use` 是 `unused_imports`。`mod` 裡面看不到外面的名字，要寫 `crate::` 路徑（E0433）。
  - 多檔案或需要 `cargo` 才能展示的內容（`src/lib.rs`、`mod 名稱;`、`tests/`、`Cargo.toml`、workspace）：每個檔案前一行寫「`路徑`：」（路徑放在行內程式碼裡，後面接全形冒號），再接程式區塊：`.rs` 用 `rust,ignore`，`Cargo.toml` 用 `toml`，目錄樹用 `text`（目錄樹可以用 `├──`、`└──`，節點只寫英文名稱，免得中英文寬度對不齊）。單一集裡的檔案一路接續，後面的同名檔案會覆蓋前面的，就像讀者一路在改它；沒寫 `Cargo.toml` 時，預設是 package `teashop`、版本 `0.1.0`、edition 2024。
  - cargo 指令的輸出放 `text` 區塊，前一行寫「`cargo test` 的輸出：」；省略 `Compiling`、`Finished` 等進度行時寫「`cargo test` 的輸出（省略編譯進度）：」；輸出帶執行緒編號（失敗的測試）時再加「某一次」：「`cargo test` 的輸出（省略編譯進度，某一次）：」。不要用「執行結果：」，那是 `fill.py` 認得的標籤，專給單檔 `rust` 區塊用。路徑一律寫成 `/home/you/teashop`，並在文中說明路徑、秒數與 `teashop-` 後面的雜湊每個人不同。
  - 這些 cargo 輸出與多檔案範例，由撰寫者在暫時建立的專案裡實測、逐字比對（把每一段的檔案依序寫進暫時專案，再執行指令，工具是 `projfill.py`）。`checkcode` 另外驗證專案本身：編譯、clippy、rustfmt、`cargo test` 的成功或失敗跟書上貼的相符，但不逐字比對輸出，規則見下一節「多檔案專案」。
  - 測試區塊用 `rust,test,noplayground`，內容像 `src/lib.rs`：函式加 `#[cfg(test)] mod tests { use super::*; ... }`。`#[cfg(test)]` 與 `#[cfg(feature = "名字")]` 都是 `#[cfg]`，要到第 16 章第 10 集才正式教，在第 12 章當固定句型照抄；`#[test]`、`assert!`、`assert_eq!` 在第 10、11 集教。
  - 函式前面的 `pub` 第 4 集才解釋，但第 2、3 集的範例不得不用，一句話帶過「先照抄」；`use` 在第 5 集之前只有第 7 章的固定句型，第 3 集 E0433 的 `help` 建議 `use crate::menu;`，文中說第 5 集才學。
  - 會被 `cargo` 或 clippy 拿來示範的訊息：E0603（`function ... is private`；從別的 crate 用 `pub(crate)` 項目也是 E0603；`module ... is private` 且 `function ... is not publicly re-exported`）、E0616（讀私有欄位）、E0451（用大括號建立有私有欄位的 struct）、E0433（`cannot find module or crate`，`help` 建議 `use crate::...`）、E0432（`unresolved import`）、E0583（`file not found for module`，只貼到 `note`，省略 `For more information` 與 `could not compile`）、E0425（`feature` 沒打開，`note` 有 `the item is gated behind the ... feature`）、E0369 與 E0277（`assert_eq!` 缺 `PartialEq` 與 `Debug`）。clippy 的 `len_zero` 說明網址帶的版本號是 clippy 的版本（目前是 `rust-1.96.0`），不是 Rust 的版本。
  - `///` 範例裡含三個反引號時（第 13 集），外層的程式區塊用四個反引號圍住（````` ````rust,ignore `````）。單檔檢查（checkcode 的 `rust` 區塊、`fill.py`）不會把四個反引號的區塊當成 `rust` 區塊；專案驗證看得懂圍欄的長度，前面有路徑標籤的話，照樣是專案裡的檔案。
  - 第 12 章的 crate：`Cargo.toml` 的 `[dependencies]` 展示只能用第二部清單內的 crate，版本與 `tools/checkcode/deps/Cargo.toml` 一致（`rand = "0.10.3"`、`clap = { version = "4.6.7", features = ["derive"] }`）；第一部不寫實際呼叫 crate 的 `rust` 區塊。
  - 排版亂掉的示範不能用 `rust`（checkcode 會因 `rustfmt --check` 擋下）：用 `rust,ignore`，後面接「`cargo fmt` 之後的 `src/main.rs`：」與排好的 `rust` 區塊；checkcode 會確認 rustfmt 真的把前一個區塊排成後面那個。展示 clippy 警告用 `rust,warn`（`len_zero` 這類不在放行清單的 lint），輸出標籤用「`cargo clippy` 的輸出（省略編譯進度）：」。

- 附錄一相關的範例（a～j）：
  - 進度邊界：可用第 1～12 章的全部內容與固定句型，不用第 13 章起的東西（`Deref`、`Rc`、執行緒、`size_of`……）。附錄一各集可以引用字母較前的集（i 用 g 的 `{:b}`），要預告字母較後的集時只用一句話（a 的 `{:x}` 標明「附錄一 g 才教」），不展開。非得提到書裡完全沒教的東西（`char::from_u32`），一句話帶過並寫「這本書不展開」，不寫範例。`TryFrom` 第 16 章第 13 集才教，附錄一 i 只用一句話指向它。
  - `Self` 在 e 之前（a～d）不用，從 e 起可用（見「進度邊界」）；`#[allow(...)]`、`#[deny(...)]` 在 j 之前不用，需要避開警告時改寫範例，不要靠屬性。
  - 領域沿用飲料店：`prices`、`Account`、`Status`、`Size`、`Member`、`Menu`、`Money`、`Meters`／`Seconds`（第 5 章第 5 集的 tuple struct 對照）。
  - clippy 的雷（都實測過）：`010`（`zero_prefixed_literal`，只有 clippy 會擋，rustc 照常編譯，所以用 `rust,warn` 展示，fill.py 只抓 rustc 的警告，clippy 的訊息改用文字說明）；對字面值 `300 as u8`（`unnecessary_cast`，另外 rustc 的 deny lint `overflowing_literals` 也會擋），轉換要用有型別的變數；`let x = x;` 只在 `x` 沒被改過時才會被 `redundant_locals` 擋，先 `let mut` 改過再 `let x = x;` 凍結沒事；`'outer: loop { loop { break 'outer; } }` 外層從不重複（`never_loop`，deny），外層要真的會迴圈；`println!("{}", format!(...))`（`format_in_format_args`）；`println!("{0} {1}", "a", "b")` 引數是字面值（`print_literal`）；`stock > 0 && println!(...)` 不是 clippy 而是 E0308（`println!` 回傳 `()`）。
  - 附錄一 g 的輸出會刻意對不齊：寬度算字元數，中文字寬度是兩個英文字母。這種範例的輸出照實貼，文中要說明為什麼歪。
  - 附錄一 j 之前遇到「不需要的 `mut`」之類的警告，用 `rust,warn` 展示，並讓警告成為教學的線索（d 的迴圈 shadowing 會得到 `unused_mut`）。
  - 1.99 的訊息形狀（實測）：沒宣告的標籤是 E0426；`Self` 放在普通函式裡是 E0411（`Self not allowed in a function`）；陣列長度用變數是 E0435（`help` 建議 `const`，貼整段）；shadowing 之後 `push_str` 不存在是 E0599，訊息帶 `there's an earlier shadowed binding` 的 `note`；對 shadow 之後的不可變變數 `+=` 是 E0384；`u32 as char` 是 E0604、`i32 as bool` 是 E0054；字面值放不下是沒有錯誤碼的錯誤（`literal out of range for ...`，`overflowing_literals`）；`#![deny(unused_variables)]` 產生沒有錯誤碼的錯誤；`[u32; 3]` 對 `[u32; 4]` 是 E0308；`type Price = u32;` 後 `impl Price` 是 E0390（本書沒用到）。
  - 「跟 Go 比一比」用在：a（`010`、`1e3`）、b（`switch` 裡的 `break`）、d（內層 `:=`）、f（`type X Y` 是新型別）、g（`%8s` 靠右）、h（`[]int` 同時是擁有者與一段資料）、i（浮點數轉整數放不下）、j（未使用變數是錯誤）。c（短路）與 e（`Self`）沒有值得單獨成節的差異。

- 智慧指標相關的範例（第 13 章）：
  - 領域沿用飲料店：`Price`（價格樹）、`Step`（製作步驟）、`Tea`／`Coffee`、`Priced`（`Box`）；`Sleeve<T>`（杯套，`Deref`）；`Cup`（`Drop`，印「洗杯子」）；`Menu`／`Branch`（`Rc`）；`Till`（`RefCell`）；`Card`／`Member`（`Rc<RefCell<T>>`）；`Shop`、`Hq`／`Store`（`Weak`）；`Kiosk`（`Cell`）；`normalize`（`Cow`）。避開 The Rust Book 的 cons list（`List`、`Cons`、`Nil`）、`MyBox`、`CustomSmartPointer`、`Messenger`／`LimitTracker`、`Node`／`leaf`／`branch`。
  - 第 13 章只用 std（大綱沒有指定 crate）。`use` 可以用（第 12 章第 5 集），`Self` 可寫可不寫；不寫執行緒、`size_of`（附錄二）、`unsafe`、`Pin`。`Arc` 只在第 5、7 集各用一句話指向第 14 章第 8、9 集，不展開；「`Rc`、`RefCell`、`Cell` 只能在單一執行緒用」一句話帶過，不談 `Send`／`Sync`。
  - 第 1、2 集用到「`&Box<T>` 傳給要 `&T` 的參數」，只說「編譯器自動轉換，原理下一集」，第 3 集才解釋解參照轉換。前面各章指向第 13 章的地方，都已改成指名第 1 集（`Box`）、第 3 集（`Deref`）、第 4 集（`Drop`）。
  - 用語：參照計數（reference counting）、強參照（strong reference，`Rc` 的擁有者）、弱參照（weak reference，`Weak`）、參照循環（reference cycle）、記憶體洩漏（memory leak）、內部可變性（interior mutability）、解參照轉換（deref coercion，前面各章稱「自動轉換」）、clone-on-write（寫入時才複製）。第 5 集先用「擁有者數量」讀 `strong_count`，第 8 集才引入強、弱參照。
  - 觀察 drop 與計數用 `impl Drop`（`println!`）與 `Rc::strong_count`／`Rc::weak_count`，輸出文字寫成「擁有者：2」「強參照 1，弱參照 1」。`Drop` 的 struct 欄位要在 `drop` 裡被讀到（`self.name`），否則 `dead_code`。
  - `RefCell` 重複借用的 panic，1.99 的訊息是 `RefCell already borrowed`（不是舊版的 `BorrowMutError`），所以用 `rust,exit=101`，標籤寫「執行結果（某一次）：」，並在文中說明括號中的數字會變。`Weak` 的 `{:?}` 印成 `(Weak)`，沒有資訊，不要拿來展示。
  - clippy 會擋下這些寫法（都實測過）：參數寫 `&Box<T>`（`borrowed_box`，函式要 `&T`，讓呼叫端靠解參照轉換傳 `&boxed`）；`&*b` 這種自動解參照就辦得到的寫法（`explicit_auto_deref`）；`vec![Rc::new(...); n]`（`rc_clone_in_vec_init`）；`drop(&x)` 與 `drop(Copy 的值)`（`drop_ref`、`drop_copy`，rustc 另有 `dropping_references`、`dropping_copy_types` 警告）；`drop(x)` 而 `x` 的型別沒有 `Drop`（`drop_non_drop`，被 drop 的型別要有 `Drop`，`Rc`、`Box`、`Vec` 都算）；`Box::new(Vec::new())`（`box_default`）。不擋：`Vec<Box<dyn Trait>>` 加 `push`（`dyn` 沒有固定大小，`vec_box` 不管；`vec_init_then_push` 已放行）、`Rc<RefCell<T>>`、`x.clone()` 對 `Rc`（`clone_on_ref_ptr` 預設關閉，書中仍寫 `Rc::clone(&x)`）。
  - `Cow` 的回傳型別一律寫 `Cow<'_, str>`：寫成 `Cow<str>` 會被 rustc 1.99 警告 `mismatched_lifetime_syntaxes`（跟 `Receipt<'_>` 同一條規則，checkcode 判失敗）。
  - 1.99 的訊息形狀（實測）：遞迴型別沒有 `Box` 是 E0072（`recursive without indirection`，`help` 建議 `Box<Price>`；`Option<Step>` 的 struct 一樣）；`Box<i32> + 5` 是 E0369，`note` 夾 `/rustc/<雜湊>/library/alloc/src/boxed.rs` 路徑，貼前 8 行；對沒有 `Deref` 的型別寫 `*x` 是 E0614（整段貼，很短）；`String::trim(&s)` 是 E0599（`help` 說 `the function trim is implemented on str`，在 `note` 之後，只貼前 6 行）；`x.drop()` 是 E0040（整段貼）；`Drop` 加 `Copy` 是 E0184；`*rc -= 3`（`Rc<i32>`）是 E0594（`help` 說 `DerefMut` 沒有實作，整段貼）；`&self` 方法裡對 `Vec` 欄位 `push` 是 E0596（`help` 建議 `&mut self`）；`Cell<String>` 呼叫 `get` 是 E0599（`String: Copy` 沒有滿足，整段貼）。
  - 篇幅：每集 101～150 行（第 8 集 150 行最長）。省行數的辦法：用 unit struct（`struct Tea;`）、把「規則照舊」的 E0382 改成一句話、每個 `compile_fail` 只貼前半段。

- 並行相關的範例（第 14 章）：
  - 領域沿用飲料店：`Till`（`total`、`orders`）、`cups`／`beans`、`menu`、`board`、`NEXT_TICKET`、`SHOP_NAME`、`ORDERS`、worker pool 的 `job_tx`／`done_tx`。避開 The Rust Book 的 `hi number … from the spawned thread!`、`vec![1, 2, 3]` 的 `v`、`Mutex::new(5)` 的 `m`、10 條執行緒加總的 `counter`、`ThreadPool`／`Worker`／`Job`、`val` 的 `hi`、`from`、`the`、`thread` 訊息。
  - 只用 std。`use` 可用（`use std::thread;`、`use std::sync::{Arc, Mutex, mpsc};`），不用 `Self`、不寫 async、不用 crate。`Duration` 是固定句型（見上表），`Instant`、`std::time` 的其他東西不用；`Ordering` 只寫 `Ordering::SeqCst`，其他選項（`Relaxed`、`Acquire`、`Release`）一句話帶過「本書不展開」；`Send`／`Sync` 在第 11 集之前只說「先讀成能安全交給別的執行緒」；`unsafe`、`static mut` 只在文字帶過（E0133，第 16 章第 5 集）；`Condvar`、`Barrier`、`thread::park`、`thread::Builder`、`recv_timeout`、`compare_exchange` 不教，最多提名字。
  - **輸出要固定**（這章最大的風險）：
    - 執行緒裡面不 `println!`。結果靠 `join` 的回傳值或 channel 帶回主執行緒，由主執行緒依 handle 的順序印出；好幾條執行緒的結果先收進 `Vec` 再 `sort`，或只印跟順序無關的值（總和、`len()`、`is_ok()`、`> 0`）。
    - 第 14 章第 1 集刻意展示交錯：標籤寫「某一次的輸出（順序每次可能不同）：」，`fill.py` 不比對，貼的是實際跑出來的其中一次；除了這一處，不要再用。
    - 需要「主執行緒先、子執行緒後」時，靠 `sleep` 拉開至少 100 毫秒的差距（慢的一方 200 毫秒、快的一方不睡）；整個區塊的睡眠總和不超過約 1 秒。不印 `ThreadId`、經過時間、`available_parallelism`。
    - 子執行緒 panic 的訊息帶 `thread '<unnamed>' (數字)`，`fill.py` 只把 `thread 'main' (N)` 正規化，所以不能讓它出現在「執行結果」區塊：子執行緒的 panic 一律用 `match handle.join()` 或 `.is_err()` 接住（結束碼 0，stderr 不進「執行結果」），並在文中用行內程式碼說明 stderr 會多印什麼。不要寫會讓子執行緒 panic 又讓 `main` 以 `exit=101` 結束的範例。
    - 每個會開執行緒的範例，寫完用 `fill.py --check` 連跑 5 次以上確認輸出穩定（兩份同時跑也要穩）。
  - 會永遠卡住的範例（忘了 `drop(tx)`、死結）用 `rust,no_run`，後面不接輸出區塊（checkcode 只編譯，rustfmt 與 clippy 仍要乾淨）；文中用文字或 `text` 區塊說明會看到什麼。實測：編譯後執行幾秒再中止。
  - Go 的輸出區塊（`go run -race main.go` 的輸出）：標籤寫「`go run -race main.go` 的輸出（節錄）：」，`fill.py` 不認得，不會比對；貼實測的其中一次，位址與 goroutine 編號每次不同，文中要說明。Go 區塊用 Tab 縮排，要在 Go 1.27 實際跑過。
  - `lock()`、`read()`、`write()`、`join()`、`send()`、`recv()` 都回傳 `Result`，一律 `.expect("…")`（訊息寫成「收銀機壞了」「店員出事了」「櫃台不見了」這種領域用語）；`tx.send(x);` 與 `handle.join();` 單獨一行是 `unused_must_use`（clippy `-D warnings` 擋）。`thread::spawn(...)` 不接 handle 沒問題。
  - 慣用寫法：迴圈裡 `let x = Arc::clone(&x);` 用同名遮蔽再 `move`；`thread::scope` 裡迴圈變數（`for price in …`）每圈新建，活不到範圍結束，要借它還是得 `move`（不然 E0373），共用的 `Mutex` 先 `let till_ref = &till;` 再 `move`（參照是 `Copy`，直接 `move` `till` 本身會被第一條搬走）。
  - clippy 實測：`let _ = m.lock();` 被 `let_underscore_lock`（deny）擋；`Arc::new(RefCell::new(..))` 被 `arc_with_non_send_sync` 擋（要展示 `Arc<RefCell<T>>` 不能跨執行緒，寫成 `is_send::<Arc<RefCell<u32>>>()` 的 `compile_fail`）；`n.store(1, Ordering::Acquire)`、`n.load(Ordering::Release)` 是 rustc 的 deny lint `invalid_atomic_ordering`。不擋：`Mutex<u32>`／`Mutex<bool>`（`mutex_atomic` 預設關）、`RwLock` 的 `write()` 只用來讀、`static NEXT: AtomicU32`、`thread::spawn(函式名稱)`、`static ORDERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());`、`Receiver` 的 `for x in rx`。
  - rustfmt：`use std::collections::HashMap;` 要排在 `use std::sync::LazyLock;` 之前（`use` 依字母排序）；`use std::sync::{Arc, Mutex, mpsc};`（大寫在小寫前）；`println!` 引數超過寬度就拆成每行一個，所以 `println!("店員那邊 {}，這邊 {cups}", handle.join().expect("店員出事了"))` 要拆開或先 `let` 起來。
  - 1.99 的訊息形狀（實測）：`thread::spawn` 借區域變數是 E0373（`note: function requires argument type to outlive 'static`，`help` 直接建議 `move`；迴圈裡同時又借 `&mut` 還會多一個 E0499）；`Rc` 交給 `spawn` 是 E0277（`Rc<String>` cannot be sent between threads safely，`help` 寫 `the trait Send is not implemented`，後面兩則 `note` 夾 `/rustc/<雜湊>/library/std/src/thread/functions.rs`，只貼前 14 行）；`Arc<RefCell<T>>` 與範圍執行緒裡的 `Cell`／`RefCell` 是 E0277（`cannot be shared between threads safely`，`help` 寫 `the trait Sync is not implemented`，`note` 建議 `RwLock` 或 atomic）；`join` 之後再 `join` 是 E0382（`takes ownership of the receiver self`）；`send` 之後再用值是 E0382；範圍執行緒兩條都 `&mut` 同一個變數是 E0499（迴圈裡是 `was mutably borrowed here in the previous iteration of the loop`），一改一讀是 E0502，兩者的 `note` 都夾 `/rustc/<雜湊>/library/std/src/thread/scoped.rs`，只貼前 12～15 行；`static` 放 `Cell` 是 E0277（`shared static variables must have a type that implements Sync`）；`static` 初始值呼叫非 const 函式是 E0015（`note: consider wrapping this expression in LazyLock::new(|| ...)`）；`static mut` 讀寫要 `unsafe` 是 E0133；`static` 不可變直接改是 E0594。
  - 執行時的行為（都實測過）：`recv()` 在通道關閉且沒有訊息時回傳 `Err(RecvError)`，`try_recv()` 是 `Err(Empty)` 或 `Err(Disconnected)`；`send` 在 `Receiver` 被 drop 後回傳 `Err`（`SendError`，`.0` 是沒送出去的值）；`join` 拿到子執行緒 panic 是 `Err`；持鎖時 panic 之後 `is_poisoned()` 是 `true`、`lock()` 是 `Err`，`PoisonError::into_inner()` 可以拿回 guard；`try_lock()` 在同一條執行緒已持鎖時是 `Err`，`{:?}` 印成 `"WouldBlock"`（有引號）；`sync_channel(2)` 的第 3 筆 `send` 會卡住、`sync_channel(0)` 第 1 筆就卡住，`mpsc::channel()` 不卡；`try_send` 滿了是 `Err(TrySendError::Full(值))`；`AtomicU8` 從 250 `fetch_add(10)` 回傳 250、結果是 4（環繞）；`OnceLock::set` 第二次回傳 `Err(值)`；`LazyLock` 多條執行緒同時第一次存取，初始化閉包只跑一次；`while let Ok(x) = rx.lock().expect(..).recv() { … }` 的 guard 活到迴圈本體結束（edition 2024 也一樣），3 位工人處理 6 筆各 100 毫秒的工作，`let job = …recv();` 約 200 毫秒、`while let` 約 600 毫秒；同一條執行緒對同一個 `Mutex` 連續 `lock()` 兩次，在 macOS 上會卡住（文件只說「可能 panic 或死結」，書裡照文件寫）；兩把鎖相反順序加 `sleep` 一定卡住。
  - 「跟 Go 比一比」第 14 章用在：01（goroutine 很輕、`thread::spawn` 是作業系統執行緒）、02（沒 `recover` 的 goroutine panic 讓整個程式結束；`WaitGroup` 帶不回結果）、03（Go 閉包共用變數）、05（`close` vs drop 最後一個 `Sender`；送指標兩邊共用）、06（誰來 `close`）、07（`sync.Mutex` 跟資料分開）、12（`Add` 回傳新值、沒有 `Ordering`）、13（`init()`、`sync.OnceValue`）、14（runtime 的 `all goroutines are asleep - deadlock!`）、15（`-race`）、16（`make(chan T, n)` 對應 `sync_channel`，`channel()` 是無限的）、17（多個消費者直接 `range` 同一個 channel）。04、08、09、10、11 沒有值得單獨成節的差異。Go 版的行為都在 Go 1.27 跑過：`go run -race` 對計數器印 `WARNING: DATA RACE`、`Found 2 data race(s)`、`exit status 66`；兩把鎖相反順序加 `WaitGroup` 印 `fatal error: all goroutines are asleep - deadlock!`、`exit status 2`；`make(chan int, 2)` 讓送的人在第 3 筆卡住。
  - 篇幅：每集 89～153 行（第 15 集 153 行最長）。省行數的辦法：編譯錯誤只貼前半段（用 `tr.sh` 節錄，`fill.py --check` 接受前綴節錄）、「規則照舊」的錯誤（`join` 兩次、`move` 之後再用）改成一句話。

- async 相關的範例（第 15 章，第一次用 crate）：
  - 領域沿用飲料店：`brew`、`serve_set`、`tea_price`、`Cup`（`Drop`，印「杯子收走了」，用來展示取消）、`machine`／`till`／`orders`（共用資料）、`sent`（計數）。避開 Tokio 官方教學的 mini-redis、`say_world`、`hello`、`count`，以及 The Rust Book 的 async 章節（`trpl`、`page_title`、`slow`、`race`、`hello from task`）。
  - 用到的 crate 只有 `tokio`（`features = ["full"]`）與 `tokio-util`（第 10 集的 `CancellationToken`），路徑照「第二部使用 crate」寫成 `tokio::...`、`tokio_util::...`；`tokio::select!`、`tokio::spawn` 這類路徑也算。不用 `futures`、`async-trait`；`join!`、`Pin`、`LocalSet`、`unbounded_channel`、`select!` 的 `biased;` 最多一句話帶過。`use tokio::time::sleep;`、`use tokio::task::JoinSet;`、`use tokio::sync::mpsc;` 放檔案最上面，跟其他 `use` 依字母排序（rustfmt 會檢查）。
  - 第 1 集不寫 async；`#[tokio::main]` 加 `async fn main()` 與 `tokio::time::sleep(Duration::from_millis(n)).await` 是固定句型（見上表），第 2 集起照抄，第 4 集解釋；`async { }` 區塊第 3 集教，`tokio::spawn` 第 5 集教。
  - **輸出要固定**（比照第 14 章，但 async 多幾個陷阱）：
    - 任務之間靠 `sleep` 拉開至少 100 毫秒再比先後（`select!` 兩個分支不要同時就緒，預設會隨機挑）；整個區塊的睡眠總和不超過約 1 秒。不印經過時間、不用 `Instant`、不印執行緒編號。
    - 任務可以在裡面 `println!`，但要靠上面的時間差決定順序；一次要收好幾個結果時，依 `JoinHandle` 的順序等，或收進 `Vec` 再 `sort`。
    - 順序要靠排程決定的示範（`Mutex` 排隊、單執行緒卡住、`mpsc` 背壓），用 `#[tokio::main(flavor = "current_thread")]`：任務依開出來的順序排程，連跑 20 次都固定。
    - 任務 panic 一律用 `handle.await`、`join_next` 的 `Err` 接住（`JoinError::is_panic()`）。panic 訊息的執行緒名稱是 `thread 'tokio-rt-worker' (數字)`（tokio 1.54.1），印在 stderr，`fill.py` 在結束碼 0 時只貼 stdout，所以不會進「執行結果」；要在文中提到它，只用行內程式碼，並說明數字與名稱會變。
    - 每個範例寫完，用 `fill.py --check` 連跑兩次以上（任務的順序不能時好時壞）。`fill.py` 對用到 crate 的區塊也有效：它把區塊放進跟 checkcode 同一份 `deps` 設定（同一份 `Cargo.toml`、`Cargo.lock` 與 `deps/target` 快取）的暫時 package 裡，用 `cargo build` 編譯執行，所以訊息的路徑一樣是 `src/main.rs`。
  - 會永遠卡住的範例（忘了 `drop(tx)`、同一個任務 `lock().await` 兩次）用文字說明，已實測：程式不會結束、沒有錯誤訊息（要貼成區塊就用 `rust,no_run`）。
  - `rust,warn`：只呼叫 async 函式、沒有 `.await`，是 rustc 的 `unused_must_use`（`unused implementer of Future that must be used`，`futures do nothing unless you .await or poll them`）。`fill.py` 的「編譯警告：」與「執行結果：」可以接在同一個 `rust,warn` 區塊後面。
  - rustfmt：`.await` 串了好幾層的方法鏈（`set.join_next().await.expect(..).expect(..)`）超過寬度就拆成每行一個，先拆成兩個 `let`；`timeout(Duration::from_millis(100), sleep(Duration::from_millis(300)))` 這類引數太長會被拆開，先把引數放進變數；`println!` 引數多的時候用小函式或 `Vec` 整理。
  - clippy 實測：本章寫出來的範例沒有觸發新的 lint（`let` chains、`while let Some(x) = set.join_next().await`、`std::thread::sleep` 放在 async 區塊裡、`Vec::new()` 加 `push` 都不擋）。
  - 1.99、tokio 1.54.1 的訊息形狀（實測）：把 `async fn` 的結果當成 `u32` 用是 E0308（`expected u32, found future`，整段很短）；普通函式裡的 `.await` 是 E0728；`async fn main` 沒有 `#[tokio::main]` 是 E0752；`tokio::spawn(async { 借區域變數 })` 是 E0373（`async block may outlive the current function`，`help` 建議 `async move`，整段貼）；同一個 `Future` `.await` 兩次是 E0382（`note` 夾 `/rustc/<雜湊>/library/core/src/future/into_future.rs`，文中只用一句話）；`JoinSet` 的任務結果型別不同是 E0271（只貼前 8 行，後面的 `note` 夾 `~/.cargo/registry/src/.../tokio-1.54.1/...` 路徑）；舊式閉包（`|n| async move { 改外面的變數 }`）是 E0507（貼前 15 行；後面還有 E0382 與 `help`），去掉 `move` 是 E0373 加一個沒有錯誤碼的 `captured variable cannot escape FnMut closure body`；任務不是 `Send`（`Rc` 或 std 的 `MutexGuard` 跨過 `.await`）是沒有錯誤碼的 `future cannot be sent between threads safely`，要用不帶碼的 `compile_fail`，只貼到第二則 `note`（`note: future is not Send as this value is used across an await`），後面的 `note: required by a bound in tokio::spawn` 夾 `~/.cargo/registry/src/...` 的絕對路徑。
  - 執行時的行為（都實測過）：`rx.recv().await` 通道關閉後回傳 `None`；`mpsc::channel(0)` panic（`mpsc bounded channel requires buffer > 0`）；`select!` 所有分支都停用又沒寫 `else` 會 panic（`all branches are disabled and there is no else branch`）；`timeout` 逾時是 `Err(Elapsed)`，Display 是 `deadline has elapsed`、Debug 是 `Elapsed(())`；`tokio::sync::Mutex` 的 `lock()` 沒有 `Result`，任務拿著鎖 panic 之後下一個 `lock().await` 照樣拿得到；同一個任務連續 `lock().await` 兩次會永遠等；`spawn_blocking` 已經開始跑的閉包，`abort()` 沒有用（`.await` 照樣拿到 `Ok`）；丟掉 `JoinSet` 取消還沒做完的任務；`join_all()` 回傳完成順序，任何一個任務 panic 它也 panic（結束碼 101）；在 async 裡對另一個 runtime 呼叫 `block_on` 會 panic（`Cannot start a runtime from within a runtime`，訊息夾 `~/.cargo/registry/src/...` 的路徑，所以不貼成區塊）；`#[tokio::main]` 底下 `main` 本體是 `block_on` 在目前執行緒跑，不需要 `Send`，只有 `tokio::spawn` 才有。
  - Go 的輸出區塊：本章沒有貼 Go 的輸出（Go 區塊只當程式對照，都在 Go 1.27 跑過：三杯飲料 `go` 加 channel 的順序是咖啡、奶茶、紅茶；`runtime.GOMAXPROCS(1)` 加一個空迴圈 goroutine，主程式照樣醒來；`WithTimeout` 逾時的 `err` 是 `context deadline exceeded`；`WithCancel` 之後 `ctx.Err()` 是 `context canceled`）。
  - 「跟 Go 比一比」第 15 章用在：01（runtime 內建、可搶佔、`go f()` 立刻開跑）、02（函式沒有顏色）、03（`go f()` 立刻開跑、丟掉 `Future` 就取消）、05（`JoinHandle`、goroutine panic 讓整個程式結束）、06（goroutine 沒有人能丟掉，`JoinSet` 丟掉就取消）、07（`make(chan T, n)` 對應 `channel(n)`、沒有 `close`、`channel(0)` panic）、08（`select` 的 `case` 是 channel 操作，Rust 的分支是整個 `Future`）、09（`context.WithTimeout` 要函式配合）、10（`context` 對照表）、11（Go 只有一種鎖）、12（Go 的 runtime 自動處理卡住的 goroutine）。04、13、14 沒有 Go 直覺會錯的地方。
  - 篇幅：每集 110～149 行。省行數的辦法：靠散文加離線驗證就夠的行為（一萬個任務、`try_send`、`spawn` 之後就開跑、`MutexGuard` 版的 `Send` 錯誤）不貼成區塊；同一種錯誤只貼一次；`compile_fail` 的訊息只貼前半段。

- 進階語言功能相關的範例（第 16 章）：
  - 領域沿用飲料店：`tray_price`／`FULL_TRAY`（`const fn`）、`prices!`／`order!`／`drink_fn!`／`say!`／`total!`（巨集）、`Tray<const N>`（杯架）、`CupsPtr`／`CUPS`（`unsafe`）、`Size`／`SizeError`（`TryFrom`）、`SHOP_NAME`、`menu.txt`。避開 The Rust Book 的 `hello_macro`／`HelloMacro`／`vec!` 的簡化版 `my_vec!`、`split_at_mut` 的重寫、FFI 的 `abs`、`static mut COUNTER`、`r1`／`r2` 的原始指標範例。
  - 第 16 章用到的 crate 只有標準函式庫；第 4 集的程序巨集只用 `proc_macro`（不用 `syn`、`quote`），做成 `path` 相依的獨立 crate：根的 `Cargo.toml` 寫 `tea_macros = { path = "tea_macros" }`，`tea_macros/Cargo.toml` 寫 `[lib]` 加 `proc-macro = true`，檔案前面一行用路徑標籤（`` `tea_macros/src/lib.rs`： ``），不需要 workspace。找型別名稱的辦法是 `tokens.iter().position(...)` 找 `struct`，再取下一個 token；用 `format!` 組出程式碼字串、`.parse()` 成 `TokenStream`。
  - 巨集（第 1～3 集）：
    - 巨集的錯誤大多沒有錯誤碼（用不帶碼的 `compile_fail`）：``no rules expected `,` ``、`unexpected end of macro invocation`（後面都帶 `note: while trying to match meta-variable`）、``variable `x` is still repeating at this depth``、``meta-variable `a` repeats 3 times, but `b` repeats 2 times``、``recursion limit reached while expanding `forever!` ``、``cannot find macro `double` in this scope``（定義在呼叫之後，會多一則 `unused macro definition` 警告）、`` `$name:expr` is followed by `-`, which is not allowed for `expr` fragments ``、`the ? macro repetition operator does not take a separator`、`2 positional arguments in format string, but there is 1 argument`；`{name}` 找不到變數則是 E0425。
    - clippy 與 rustc 會擋：`println![...]`、`matches!{...}` 這類不慣用的括號（`nonstandard_macro_braces`，只能在文字提，不能貼成 `rust` 區塊）；沒被呼叫的 `macro_rules!`（`unused_macros`）；`match x { 1..=5 => true, _ => false }`（`match_like_matches_macro`，展開對照用 `text` 區塊）；巨集本體的 `let mut` 在某次展開沒被改到（例如重複零次的 `prices!()`）會有 `unused_mut`，所以每個範例呼叫都要帶至少一組引數。
    - 展開結果在運算式位置又要放 `let` 時，規則本體用 `{{ ... }}`；巨集必須定義在使用之前。
  - `unsafe`（第 5～7 集）：
    - 不要執行會造成未定義行為的程式（`get_unchecked` 越界、解參照已失效的指標），只用文字說明；每個 `unsafe` 範例都附安全的包裝（先檢查前提）。
    - edition 2024：`unsafe fn` 的本體不再自動算 `unsafe`，裡面不安全的操作要再包 `unsafe { }`，不包是 `unsafe_op_in_unsafe_fn`（rustc 預設警告，`-D warnings` 判失敗）；對 `static mut` 取參照是 deny lint `static_mut_refs`（沒有錯誤碼，`println!("{CUPS}")` 也算取參照，要先 `let total = unsafe { CUPS };` 複製出來）；`extern` 區塊不寫 `unsafe` 是沒有錯誤碼的 `extern blocks must be unsafe`；`#[no_mangle]` 要寫 `#[unsafe(no_mangle)]`（否則 `unsafe attribute used without unsafe`）。
    - clippy：`unsafe fn`、`unsafe trait` 都要有 `# Safety` 文件小節（`missing_safety_doc`，私有的也要）；`0 as *const u32` 被 `zero_ptr` 擋，改用 `std::ptr::null()`；`unsafe` 區塊裡沒有任何不安全的操作是 `unused_unsafe`，示範「`unsafe` 不關掉借用檢查」的 E0499 時，要用 `get_unchecked_mut` 這種真的需要 `unsafe` 的呼叫。
    - 訊息形狀（實測）：E0133 有三種（`call to unsafe function ... is unsafe and requires unsafe block`、`use of mutable static is unsafe and requires unsafe block`、`dereference of raw pointer is unsafe and requires unsafe block`，後者的 `note` 列出原始指標的五種危險），都很短；`unsafe trait` 少寫 `unsafe impl` 是 E0200；原始指標進 `thread::spawn` 是 E0277（`` `*mut u32` cannot be sent between threads safely ``，貼前 14 行，後面的 `note` 夾 `/rustc/<雜湊>/library/std/src/thread/functions.rs`）。
    - 閉包只擷取用到的欄位：閉包裡直接寫 `*ptr.0 += 5` 只會搬進欄位 `*mut u32`，替包裝型別寫的 `unsafe impl Send` 派不上用場，仍是 E0277；範例一律透過方法 `ptr.add_five()` 使用整個包裝型別。
    - FFI 只呼叫每個平台的 C 標準函式庫都有、輸出固定的函式：`toupper`、`strlen`、`atoi`；用 `c"..."` 字串字面值（`manual_c_str_literals` 會擋位元組陣列的寫法）；不貼 callback、不連結自訂的 C 程式庫；`std::ffi::{c_char, c_int}`。
  - const 泛型與 `const fn`（第 8、9 集）：
    - const 泛型的參數型別只能是整數、`bool`、`char`（`&'static str` 是沒有錯誤碼的 `` `&'static str` is forbidden as the type of a const generic parameter ``）；型別裡寫 `[u32; N * 2]` 是沒有錯誤碼的 `generic parameters may not be used in const operations`；`total::<n>`（`n` 是 `let` 變數）同時有 E0435 與 E0747 兩個錯誤，所以只在文字提 E0435，不貼成 `compile_fail`；不同 `N` 的型別混用是 E0308（`` expected `3`, found `2` ``）。
    - `const fn`：在 `const` 初始值裡呼叫普通函式是 E0015（`note: function ... is not const`）；`const fn` 裡寫 `for` 是 E0015（``cannot use `for` loop on ... in constant functions``）、`String::from` 也是 E0015，`while` 可以；`const _: () = assert!(...)` 失敗是 E0080（`evaluation panicked: assertion failed: ...`，很短）；`const fn` 裡 `assert!(.., "訊息")` 失敗時，訊息後面還有幾則 `erroneous constant encountered`，所以示範用 `const _` 的版本。
  - `#[cfg]`（第 10 集）：
    - **單檔的輸出只能依賴所有平台都相同的條件**：`fill.py` 在作者的 macOS 上跑，CI 在 Linux 上跑，checkcode 只看結束碼，所以平台相關的輸出會悄悄通過 CI，卻違反「執行結果逐字相符」。範例只用 `debug_assertions`（`cargo run` 一律成立）與 `target_pointer_width = "64"`；`unix`、`windows`、`target_os` 只放在表格或文字裡，不貼成輸出。
    - `--release` 的結果不貼成區塊（checkcode、`fill.py` 都只編譯 debug），用 `rustc -C opt-level=3 -C debug-assertions=off` 實測過會印 `release`。
    - `unexpected_cfgs`（打錯條件名稱、沒宣告的 feature）只有 cargo 會檢查（單檔的 rustc 沒有 `--check-cfg`，不會警告），所以示範用專案：`src/main.rs` 寫成 `rust,warn` 區塊，後面接 `cargo run` 的輸出；警告的 `help` 很長，整段貼。
    - `cfg!(...)` 的兩個分支都會編譯，`else { "no" }` 配 `let x: u32` 是 E0308；`#[cfg(any())]` 這種永遠不成立的項目不做型別檢查，但仍要能解析（語法錯誤照樣報錯）。
  - `build.rs` 與 `include_str!`（第 11、12 集）是多檔案專案：
    - `build.rs` 的輸出只用寫死的值，不用 `std::env`（第 17 章第 5 集）、`std::fs`（第 3 集）、`std::path`（第 4 集）、時間（第 6 集），也會讓輸出不固定；`cargo::` 雙冒號寫法；`cargo::warning=...` 會在 cargo 輸出裡印成 `warning: teashop@0.1.0: ...`，沒有 `Compiling` 以外的前綴。`build.rs` 與 `src/main.rs` 都用 `rust,ignore` 加路徑標籤，後面接 `cargo run` 的檢查點。
    - `include_str!` 的專案依序放：缺檔的 `cargo run`（``error: couldn't read `src/menu.txt`: No such file or directory (os error 2)``，沒有錯誤碼，這是 Unix 的訊息）、補上 `src/menu.txt`（`text` 區塊加路徑標籤，projfill 認得任何語言的檔案）、再改一次內容後的 `cargo run`（cargo 自動重編）。
    - **專案的輸出區塊還是 `TBD` 的時候，checkcode 會判失敗**：檢查點看貼出的 `text`（沒有 `error` 行就必須成功），`TBD` 會被當成「預期成功」。先用 `projfill.py` 填好輸出，再跑 checkcode。
  - `TryFrom` 與 `TryInto`（第 13 集）：1.99 的 `TryFromIntError` 的 `{:?}` 是 `TryFromIntError(PosOverflow)` 或 `TryFromIntError(NegOverflow)`（不是舊版的 `TryFromIntError(())`）；`.try_into()` 沒有型別標註是 E0283；`u8::from(300_i32)` 的 E0277 `help` 會夾 `/rustc/<雜湊>/library/core/...` 路徑，所以只用文字提；自己的錯誤型別用 unit struct（`struct SizeError;`）。
  - 「跟 Go 比一比」第 16 章用在：01（`fmt` 是 `...any` 的函式，格式錯了要靠 `go vet`）、04（struct tag 加執行時期反射、`go generate`）、05（`unsafe` 套件 vs 區塊）、06（Go 的 `*T` 對應 Rust 的參照）、07（cgo）、08（泛型只有型別參數）、09（`const` 不能呼叫函式）、10（build constraint 以檔案為單位）、11（`go generate`）、12（`//go:embed`）、13（轉換悄悄截斷）。02、03 沒有（Go 沒有巨集）。Go 版的行為都在 Go 1.27 跑過：`fmt.Printf("%d 賣 %d 元\n", "紅茶")` 輸出 `%!d(string=紅茶) 賣 %!d(MISSING) 元`、cgo 的 `C.strlen` 是 6、`const x = f()` 是 `... is not constant`、`//go:embed` 範例、`uint8(int32(300))` 是 44。
  - 篇幅：每集 87～154 行（第 5、10 集最長）。省行數的辦法：靠散文加離線驗證就夠的範例改成一句話（`get_unchecked` 的 E0133、`const` 陣列長度、切片版的 `total`、`?` 搭配 `Box<dyn Error>`）、補上的修正版用 `rust,ignore` 片段、編譯錯誤只貼前半段。

- 進階 std 與常用 crate 相關的範例（第 17 章）：
  - 領域沿用飲料店：`teashop-demo`（資料夾）、`menu.txt`、`Order`／`OrderError`／`MenuError`、`TEASHOP_DISCOUNT`、`drink`、`price`。避開 The Rust Book 的 `Guess`、serde 與 tracing 文件的 `Point`／`Person`／`shape`、thiserror 與 anyhow 文件的 `DataStoreError`／`Config`。
  - **輸出不能依賴平台**（延續第 16 章第 10 集的規則）：`fill.py` 在作者的 macOS 上跑，CI 在 Linux 上跑，checkcode 只看結束碼，平台相關的輸出會悄悄通過 CI。所以：不印 `current_dir()`、`temp_dir()`、`env::vars()` 的全部、`Instant`／`SystemTime` 本身的值、行程編號、作業系統的錯誤訊息（`{e}` 的 `No such file or directory (os error 2)`）。改印不變的性質（`is_absolute()`、`>=` 的比較）、`e.kind()`（`NotFound`）、自己造的 `io::Error::new(ErrorKind::NotFound, "…")` 的訊息。路徑分隔符號以 Linux、macOS 的 `/` 為準，文中說明 Windows 不同；`Command` 的範例只用 `echo`、`sh`、`cat`（Linux、macOS 都有，輸出一樣），文中說明 Windows 沒有。
  - 檔案範例都在目前的工作目錄建一個 `teashop-demo` 資料夾、最後 `remove_dir_all`（fill.py 與 checkcode 都在每個區塊自己的暫存目錄執行，所以不會留下東西）；第 4 集才教 `Path`、第 5 集才教 `env::temp_dir()`，之前路徑一律寫字串；第 5 集之後要放暫存檔就用 `temp_dir()`。函式回傳 `io::Result` 時，`main` 寫成 `fn main() -> Result<(), std::io::Error>` 加 `?`。
  - 標準函式庫的 panic 訊息位置會是 `/rustc/<雜湊>/library/…`（例如 `Duration` 的減法溢位），不貼成輸出區塊，用文字說明；編譯錯誤裡有 `/rustc/<雜湊>/…` 或 `~/.cargo/registry/src/…` 路徑的（E0599 缺 `use std::io::Write;`、`rand` 缺 `use rand::RngExt;`），只貼前幾行，並說明後面還有 `help`。
  - 每次都不同的輸出（`rand`、`Uuid::new_v4()`、`Instant`）：能的話只印固定的性質（長度、範圍檢查、`bool`）；真的要給讀者看實際數字（第 11 集第一個範例），用 `fill.py` 不認得的標籤「某一次的輸出（每次都不同）：」，貼一次真實執行的結果。要固定的亂數用 `StdRng::seed_from_u64(種子)`，並說明只在同一版本成立。
  - **`rand` 的區塊一律加 `noplayground`**（隨機輸出每次不同，Run 按鈕的結果跟書上對不上；Playground 其實也是 0.10.3，這是作者在第 17 章 issue 的要求）。`Command` 的區塊也加 `noplayground`（沒辦法在 Playground 確認它允許啟動外部程式）。其他 crate 的區塊不加。
  - `tracing`：`tracing-subscriber` 不在 crate 清單，所以第 10 集手寫了一個約 45 行的 `Subscriber`，放在檔案最下面、標註「先照抄，不用看懂」，輸出沒有時間戳記所以固定。沒有 subscriber 時什麼都不印，也要在文中說清楚。subscriber 的 `enabled` 用 `*metadata.level() <= Level::INFO`（`Level` 越瑣碎越「大」）。作者若決定把 `tracing-subscriber` 加進清單，這一集要重寫。
  - `serde_json`：`Value` 的物件鍵依字母排序（`BTreeMap`，沒開 `preserve_order`）；`derive(Serialize)` 的 struct 依欄位宣告順序；範例不放 `HashMap` 欄位（順序不固定），用 `BTreeMap`。只 `derive(Deserialize)`、再用 `{:?}` 印的 struct，欄位照樣算沒讀過（`dead_code`）：要嘛同時 `derive(Serialize)`，要嘛真的讀欄位。
  - `thiserror`：同一個檔案同時 `use std::error::Error;`（trait）與 `use thiserror::Error;`（derive 巨集）不衝突；`#[from]` 只能用在只有一個欄位的 variant，要多帶資訊就用具名欄位加 `#[source]`；範例自己造 `io::Error` 讓訊息固定。
  - `anyhow`：設了 `RUST_BACKTRACE=1`，`{e:?}` 與 `main` 回傳 `Err` 的輸出會多出 `Stack backtrace:`（實測），書上的輸出都是沒設定的樣子，`fill.py --check` 前先確認環境沒設。`main` 回傳 `Err` 的範例寫 `rust,exit=1`，標籤「執行結果：」。
  - clippy 與 rustfmt 實測（都被擋過）：`read` 不用回傳值（`unused_io_amount`，deny）；`for line in reader.lines() { if let Ok(line) = line { … } }`（`manual_flatten`）；`d.subsec_nanos() / 1_000_000`（`duration_subsec`）；`b <= u8::MAX`（`absurd_extreme_comparisons`，deny）；對字串字面值 `Path::new(..).join("/c")`（`join_absolute_paths`，變數不擋）；用了 `stdin().lines()` 卻 `use std::io::BufRead;`（`unused_imports`，`stdin().lock().lines()` 才需要）；`println!` 引數超過寬度、`let` 右邊的 `if … else` 與 `for … in [長陣列]` 都會被 rustfmt 拆成多行，先照 checkcode 的結果寫。
  - 1.99 的訊息形狀（實測）：沒有 `use std::io::Write;` 呼叫 `write_all` 是 E0599；`let s: &str = path;`（`&Path`）是 E0308；edition 2024 不包 `unsafe` 呼叫 `env::set_var` 是 E0133（整段很短，`note: consult the function's documentation…`）；`rand` 0.10 沒有 `thread_rng`（E0425）與 `gen_range`（E0599），`random_range` 要 `use rand::RngExt;`（`use rand::Rng;` 是 E0599，而且 `unused_imports`）。
  - 驗證過的事實：`read` 讀到結尾回傳 `Ok(0)`；`lines()` 去掉 `\n` 與 `\r\n`；`BufWriter` 在 drop 前不寫出；`read_to_string` 遇到不合法的 UTF-8 是 `InvalidData`（`stream did not contain valid UTF-8`）；`Path::new("shop//menu") == Path::new("shop/menu")` 是 `true`；`Path::new("archive.tar.gz").extension()` 是 `Some("gz")`；`Path::new("shop").join("/tmp/tea.txt")` 是 `/tmp/tea.txt`；`env::var` 沒設定是 `Err(NotPresent)`，空字串是 `Ok("")`；`Duration::from_secs(90)` 的 `{:?}` 是 `90s`、`from_millis(1500)` 是 `1.5s`；`checked_sub` 小減大是 `None`；`UNIX_EPOCH.duration_since(之後的時間)` 是 `Err`；`Command::output()` 在結束碼不是 0 時仍回傳 `Ok`；找不到程式是 `NotFound`；餵 `cat` 後忘了 `drop(stdin)` 會永遠卡住；`process::exit` 不執行 drop；JSON 最後多一個逗號，`serde_json` 的錯誤是 `trailing comma at line 1 column 20`（欄數依輸入而不同）；`Option` 欄位缺了是 `None`，其他欄位缺了是 `missing field`；`Uuid::parse_str` 接受大小寫與沒有連字號的寫法；`anyhow` 的 `{e:#}` 用 `: ` 串起整條鏈。Go 1.27 都跑過：`bufio.Scanner` 一行超過 64 KiB 讓 `Scan()` 回傳 `false`、`Err()` 是 `bufio.Scanner: token too long`；`os.ReadDir` 已依檔名排序；`filepath.Join("shop", "/tmp/tea.txt")` 是 `shop/tmp/tea.txt`；`filepath.Ext` 帶點；`exec.Command(...).Output()` 在結束碼不是 0 時回傳 `err`；`json.Unmarshal` 缺欄位悄悄是零值、鍵名不分大小寫、小寫欄位被略過、解到 `any` 的數字是 `float64`；`slog.Info` 沒設定也會印（帶時間）。
  - 「跟 Go 比一比」第 17 章用在：01（`io.EOF` 與 `n > 0` 同時成立、`Write` 寫不完一定回傳錯誤、trait 要先 `use`）、02（`Scanner` 64 KiB 上限、`Flush`）、03（`ReadDir` 已排序、權限參數、`defer f.Close()`）、04（`Join` 遇到絕對路徑、`Ext` 帶點）、05（`Getenv` 分不出沒設定與空字串、`Setenv` 是普通函式）、06（`Duration` 可為負、`time.Now()` 兼具兩種時鐘）、07（`Output()` 在結束碼不是 0 時回傳 `err`）、08（解到 `any` 的數字一律 `float64`）、09（缺欄位悄悄是零值）、10（`slog` 預設會印、沒有 span）、11（`IntN` 從 0 起算、`rand.Seed` 不再需要）、14（`%w` 的字串含整條鏈，`{e}` 只印最外層）；12、13 沒有加。
  - 篇幅：每集 126～162 行（第 9 集最長）。省行數的辦法：靠散文加離線驗證就夠的範例改成一句話（`read_to_string` 的第二次讀取、`Path` 的 `components` 之外的拆解、`serde` 的帶資料 variant、`cups` 多餘欄位、`skip_serializing_if` 與 `deny_unknown_fields`、`anyhow` 的 `downcast_ref`）、表格改成一句列舉（`fs` 的其他函式）。

## 多檔案專案

第 12 章起，有些範例分散在好幾個檔案，要靠 `cargo` 才能展示。checkcode 依「路徑標籤」把一集裡的區塊組成暫時的 Cargo 專案，驗證它們編譯得過、clippy 沒警告、rustfmt 通過，書上有 `cargo test` 的地方測試真的會跑。第 18～20 章的 CLI、Web、SQLite 專案用同一套標籤。

**一集一個專案。** 一個 `.md` 檔是一條由上往下的時間線，不跨集：後面的標籤覆蓋前面同名的檔案，就像讀者一路在改它；沒有刪除檔案的寫法，要換一個專案就另開一集。沒有任何 `.rs` 檔、也沒有檢查點的集（例如只展示 `Cargo.toml`）不算專案。

**三種標籤**，都是獨佔一行、行內程式碼加全形冒號 `：` 結尾，句尾可以多一段全形括號的（說明），標籤與區塊之間可以有空行；後面緊接著的圍欄區塊才算數：

| 標籤 | 後面的區塊 | 意思 |
| --- | --- | --- |
| `` `src/lib.rs`： `` | 任何語言，通常是 `rust,ignore`、`toml` | 專案裡這個檔案現在的內容。路徑只能有英數與 `_ . / -`，檔名要有副檔名，不能是絕對路徑或含 `..`，相對於專案根目錄（workspace 成員寫 `teashop/src/lib.rs`） |
| `` `cargo run` 的輸出（省略編譯進度）： `` | `text` | 檢查點。這一刻把所有檔案寫進暫時專案，執行這條指令，結果要跟區塊相符。指令要是 `cargo` 開頭，可以帶 `--features`、`-p` 之類的參數；括號裡的說明不影響驗證 |
| `` `cargo fmt` 之後的 `src/main.rs`： `` | `rust` | 前一個版本排版亂掉的示範（見下）：這個檔案接著變成區塊的內容 |

標籤拼錯（例如半形冒號、少了反引號）不會報錯，那一行就只是普通文字，後面的區塊也就不會進專案。改完標籤要看 checkcode 最後印出的專案、檔案、檢查點數量有沒有變。

**哪些區塊是專案的檔案：**

- 前面有路徑標籤的區塊，內容一律進專案。`rust,ignore` 的 `.rs` 檔除了進專案，還要過 rustfmt（用 stdin，所以 `pub mod menu;` 這種還沒建檔的模組宣告不會出錯）。
- 帶路徑標籤、而且不是 `ignore` 的 `rust` 區塊（完整的 `main.rs`、`rust,test,noplayground` 的 `lib.rs`），同時照「程式碼區塊」一節的規則單檔驗證；它們的 `exit=`、`stdin=` 只給單檔驗證用，專案以檢查點為準。
- 沒有路徑標籤的 `rust,ignore` 還是完全不驗證（片段，例如 `mod menu;` 那一行）。
- `rust,ignore` 的 `.rs` 檔，後面（同一集）一定要有檢查點，否則專案沒有被編譯過，等於沒驗證，checkcode 會報錯。
- 沒寫 `Cargo.toml` 時，預設是 package `teashop`、版本 `0.1.0`、edition 2024。寫了（`Cargo.toml` 或 workspace 的 `Cargo.toml`）就以書上為準，不另加預設。

**檢查點怎麼判定。** 看 `text` 區塊裡有沒有 `error` 開頭的行（`error[E0603]: …`、`error: test failed…`）或 `test result: FAILED`：

- 沒有：指令必須成功。`cargo run` 與 `cargo test` 也會實際執行（30 秒內，超過算失敗；不給 stdin），結束碼必須是 0。
- 有：指令必須失敗，而且書上出現的每個 `error[E....]` 錯誤碼，實際輸出都要有。只有 `error:` 沒有錯誤碼的（例如 `a bin target must be available`），只要求失敗。
- 輸出的文字不逐字比對（雜湊、秒數、路徑、執行緒編號每次都不同），逐字比對由撰寫者用 `projfill.py` 做。
- 支援 `run`、`test`、`check`、`build`、`doc`、`clippy`；`cargo new`、`cargo add`、`cargo fmt` 這類改檔案、不編譯的指令不驗證，不算檢查點。`clippy` 當 `check` 驗（lint 本來就會跑）。
- 會一直執行的程式（伺服器）不能放 `cargo run` 的輸出區塊，會逾時失敗；改展示 `cargo build` 或 `cargo test`。
- 每個檢查點各自判定、互不影響：前一個檢查點失敗（書上本來就貼錯誤，或驗證失敗），後面的照常檢查。

**lint 跟單檔一樣嚴。** rustc 與 clippy 警告都算失敗，套用 `clippy-allow.txt` 的放行清單；書上的 `Cargo.toml` 不必（也不要）寫 `[lints]`。專案裡只要目前有一個 `rust,warn` 的檔案，這個檢查點就允許警告。

**排版亂掉的示範。** `rust,ignore` 區塊後面緊接「`cargo fmt` 之後的 `路徑`：」時，這個區塊不檢查 rustfmt，改成檢查 rustfmt 排出來的結果跟後面那個區塊完全相同；除此之外，所有 `rust,ignore` 的 `.rs` 檔都要是 rustfmt 排好的樣子。

**固定版本的 crate。** 專案的 `Cargo.toml` 的 `[dependencies]` 寫了共用依賴清單（`tools/checkcode/deps/Cargo.toml`）裡的 crate 時，checkcode 用 `deps/Cargo.lock` 當專案的鎖定檔，編譯也共用 `deps/target` 的快取。書上寫的版本要跟 `deps/Cargo.toml` 一致；專案編完後，鎖定檔裡出現 `deps/Cargo.lock` 沒有的 crate 或版本，檢查點就失敗。

**失敗怎麼讀。** 失敗訊息的「檔名:行號」是檢查點的 `text` 區塊（rustfmt 與「沒有檢查點」的錯誤則是那個檔案的區塊）；訊息會列出 cargo 診斷指到的專案檔案，連同它的區塊在這一集的行號，例如 `有問題的檔案：src/main.rs（第 23 行）、src/lib.rs（第 11 行）`，後面是 cargo 的輸出。

## 第二部使用 crate

第二部（第 13 章起）的範例可以用下列 crate，也只能用這些：`tokio`、`tokio-util`、`clap`、`axum`、`tower`、`http-body-util`、`serde`、`serde_json`、`tracing`、`rand`、`uuid`、`thiserror`、`anyhow`、`rusqlite`（bundled）。

- **照常寫 ```` ```rust ````，不用加標註。** checkcode 看程式裡有沒有出現 `<crate>::`（連字號換成底線，例如 `tokio::`、`serde_json::`、`http_body_util::`；`use tokio::time::sleep;` 和 `#[tokio::main]` 都算）：有的話，這個區塊就放進共用依賴 package 一起編譯，沒有的話照第一部的方式單檔編譯。清單外的 crate（例如 `reqwest::`）不會被認出來，單檔編譯時找不到 crate，區塊直接失敗。
- 區塊要用到 crate 的名字，一定要寫成 `crate名::` 的路徑形式（`use tokio::spawn;` 或 `tokio::spawn(...)`）。`use tokio as t;` 這種別名寫法不會被認出來。
- rustfmt、clippy（`-D warnings` 與 `clippy-allow.txt` 放行清單）、結束碼、`stdin`、`no_run`、`warn`、`test`、`compile_fail=E....` 的規則跟 std 區塊完全相同；一個區塊失敗不會影響其他區塊。
- Playground 沒有的 crate 要加 `noplayground`；不確定某個 crate 在 Playground 上能不能跑，先到 Playground 試一次。checkcode 不檢查這件事。2026-11 查過 rust-lang/rust-playground 的 `compiler/base/Cargo.toml`：`tokio`（1.54.1，含 `full`）、`tokio-util`（0.7.20）、`tower`、`http-body-util`、`serde`、`serde_json`、`tracing`、`rand`、`uuid`、`thiserror`、`anyhow` 都有，版本跟 `deps/Cargo.toml` 相同；`clap` 有（4.6.7，清單裡開了 `unstable-doc`，`clap_derive` 也單獨列在清單裡，但沒有實際在 Playground 上跑過 derive）；`axum` 與 `rusqlite` 沒有，用到它們的區塊要加 `noplayground`。第 15 章只用 tokio 與 tokio-util，所以不加 `noplayground`（只比對了 crate 清單與版本，沒有逐一在 Playground 上執行）。
- 多檔案專案（第 18～20 章的實戰章）的 `Cargo.toml` 也只能用這份清單，版本要與 `deps/Cargo.toml` 一致，規則見上一節「多檔案專案」。
- 版本統一鎖在 `tools/checkcode/deps/Cargo.toml`，`Cargo.lock` 一併提交。新增 feature 或升級版本：改該檔，執行 `cargo update --manifest-path tools/checkcode/deps/Cargo.toml`，再把 `Cargo.lock` 一起提交。不得在書中另外加清單外的 crate。
- 該檔的 `[lints.clippy]` 必須與 `tools/checkcode/clippy-allow.txt` 一致；改放行清單時兩邊一起改（checkcode 的測試會檢查）。
- 第一次編譯所有 crate 需要幾分鐘（`rusqlite` 的 bundled 要有 C 編譯器），之後會使用 `tools/checkcode/deps/target` 裡的快取。CI 也快取這個目錄。

## 正確性

- 以 Rust 1.99、edition 2024 為準。let chains（1.88）、async 閉包（1.85）都已穩定；`static mut` 取參照在 edition 2024 是錯誤；`extern` 區塊要寫 `unsafe extern "C"`，`no_mangle` 要寫 `#[unsafe(no_mangle)]`。
- 本機工具鏈若不是 1.99，用到 1.97 之後才有的功能前，先確認 CI 能過，不要憑印象。
- 不確定時看官方文件（`rustup doc --std`）或實際寫程式驗證。
- 完成後執行 `cargo run --manifest-path tools/checkcode/Cargo.toml -- zh-TW/src/<章節目錄>`，全部通過才算完成。
