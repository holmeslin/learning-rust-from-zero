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

## 固定句型

以下寫法在正式解釋之前就可以照抄使用，結構保持不變，只改變數名稱與文字：

| 首次出現 | 固定句型 | 正式解釋 |
| --- | --- | --- |
| 第 1 章第 2 集 | `cargo new <名稱>`、`cargo run`；程式骨架 `fn main() { ... }` | 函式第 2 章；Cargo 第 12 章 |
| 第 1 章第 2 集 | `println!("文字")`、`println!("{x}")`、`println!("{}", 運算式)` | 巨集第 16 章 |
| 第 1 章第 9 集 | 讀一行輸入：見下方 | `&mut` 第 4 章；方法第 5 章；`Result` 第 9 章第 4 集 |
| 第 1 章第 10 集 | 文字轉數字：見下方 | `FromStr` 第 8 章；`expect` 第 9 章第 4 集 |
| 第 1 章第 14 集 | `for i in 0..n`、`for i in 0..=n` | `Iterator` 第 11 章 |
| 第 1 章第 15 集 | 型別的最小、最大值 `i32::MIN`、`i32::MAX`（`u8::MAX` 等） | 關聯常數第 5 章（關聯函式） |
| 第 2 章第 11 集 | `println!("{:?}", 陣列)`（tuple、`()` 也能這樣印；也可寫成 `{x:?}`） | `Debug` 第 8 章 |
| 第 2 章第 13 集 | `for x in 陣列` | `IntoIterator` 第 11 章 |
| 第 2 章第 15 集 | 方法呼叫 `s.len()`（`String` 與陣列都能用）、`s.push_str("...")`；`String::from("...")` | 方法第 5 章 |
| 第 3 章第 7 集 | 方法呼叫 `s.clone()`（寫法同上） | 方法第 5 章；`Clone` 第 8 章 |
| 第 4 章第 1 集 | 對參照呼叫方法：`r.len()`、`r.push_str("...")`（`r` 是 `&mut String`），`&str` 與 slice 的 `len` 也一樣；第 11 集起 `{:?}` 也能印 slice | 自動參照與解參照第 5 章第 10 集；`Debug` 第 8 章 |
| 第 5 章第 6 集 | `#[derive(Debug)]`、`{:?}`、`{:#?}` | `derive` 第 8 章 |
| 第 6 章第 14 集 | `matches!(值, 模式)` | 巨集第 16 章 |
| 第 7 章第 1 集 | `vec![...]`；`{:?}` 也能印 `Vec`（`HashMap`、`HashSet`、`BTreeMap` 也能這樣印，但 `HashMap`、`HashSet` 的順序不固定） | 巨集第 16 章；`Debug` 第 8 章 |
| 第 7 章第 7 集 | `format!(...)` | 巨集第 16 章 |
| 第 7 章第 9 集 | `for` 後面直接放回傳迭代器的方法呼叫：`for c in s.chars()`、`for b in s.bytes()`、`for part in s.split(',')`、`for k in map.keys()`（不接 `.count()`、`.nth()`、`.rev()` 等迭代器的方法） | `Iterator` 第 11 章 |
| 第 7 章第 11 集 | `use std::collections::HashMap;`（`HashSet`、`BTreeMap` 同） | `use` 第 12 章第 5 集 |
| 第 8 章第 11 集 | `impl std::fmt::Display for 型別 { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "...", ...) } }`（骨架照抄，只改型別名稱與 `write!` 的內容；enum 搭配 `match self`，每個分支一個 `write!`）；實作 `Display` 之後型別自動有 `.to_string()`；帶生命週期的 struct 寫成 `impl std::fmt::Display for Receipt<'_>`，第 10 章第 6 集起 `Formatter` 可以寫完整的 `std::fmt::Formatter<'_>` | `Result` 第 9 章第 8 集；`write!` 巨集第 16 章；`'_` 第 10 章第 5、6 集 |
| 第 8 章第 15 集 | `Box<dyn Trait>`、`Box::new(...)`（照寫，不解釋 `Box`） | `Box` 第 13 章 |
| 第 8 章第 17 集 | trait 定義裡的 `Self::Item`（只出現在 trait 定義；`impl` 裡仍寫完整型別） | `Self` 附錄一 e |

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
  - 第 7 章第 9 集起，`for` 後面可以直接放回傳迭代器的方法呼叫（`s.chars()`、`s.bytes()`、`s.split(',')`、`map.keys()`、`map.values()`），不展開原理（`Iterator` 第 11 章）。不要接 `.count()`、`.nth()`、`.rev()`、`.enumerate()`、`.collect()` 這些迭代器的方法：數字元就用 `for` 加累加變數。
  - `HashMap`、`HashSet` 的走訪順序每次執行都不同，貼了整份輸出的話，`fill.py --check` 會時好時壞。驗證過的範例只印跟順序無關的結果（`get`、`len`、加總、計數）；要印整個 map，就只放一組，或改用 `BTreeMap`。需要展示「順序會變」時，輸出標籤寫成「某一次的輸出（順序每次可能不同）：」，fill.py 不認得這個標籤，所以不會比對；整本書只有第 7 章第 14 集這樣用，並在文中說明順序不固定。
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
  - 領域沿用飲料店：`split_bill`、`order_total`、`Size`（`S` / `M` / `L`）、`SizeError`、`OrderError`、`TooMany`。避開 The Rust Book 的 `File::open`、`read_username_from_file`、`Guess`。不用 `std::fs`（第 17 章），錯誤的來源只有 `parse`（`std::num::ParseIntError`、`std::str::ParseBoolError`）、`read_line`（`std::io::Error`）與自己定義的型別。
  - 路徑照樣寫完整：`std::error::Error`、`std::str::FromStr`、`std::num::ParseIntError`、`Box<dyn std::error::Error>`；沒有新的 `use`。`impl` 裡不寫 `Self`：`fn from_str(text: &str) -> Result<Size, SizeError>`，`type Err = SizeError;`。
  - 第 11 章之前不用 `Option` / `Result` 的方法（`unwrap_or`、`ok_or`、`map_err`、`is_ok`、`is_some`、`ok()`），只有 `unwrap`、`expect`（第 9 章第 4 集）；要把 `Option` 轉成 `Result`，用 `match`；要給預設值，`match` 的 `Err` 分支寫成區塊（先印提醒再回傳預設值）。
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
  - 進度邊界：不寫生命週期 bound（`'a: 'b`、`T: 'a`、`T: 'static`）、`for<'a>`、`dyn Trait + 'a`；不用 `Self`、閉包與迭代器的方法。`T: 'static` 只在第 7 集用一句話指向第 14 章。

## 第二部使用 crate

第二部（第 13 章起）的範例可以用下列 crate，也只能用這些：`tokio`、`tokio-util`、`clap`、`axum`、`tower`、`http-body-util`、`serde`、`serde_json`、`tracing`、`rand`、`uuid`、`thiserror`、`anyhow`、`rusqlite`（bundled）。

- **照常寫 ```` ```rust ````，不用加標註。** checkcode 看程式裡有沒有出現 `<crate>::`（連字號換成底線，例如 `tokio::`、`serde_json::`、`http_body_util::`；`use tokio::time::sleep;` 和 `#[tokio::main]` 都算）：有的話，這個區塊就放進共用依賴 package 一起編譯，沒有的話照第一部的方式單檔編譯。清單外的 crate（例如 `reqwest::`）不會被認出來，單檔編譯時找不到 crate，區塊直接失敗。
- 區塊要用到 crate 的名字，一定要寫成 `crate名::` 的路徑形式（`use tokio::spawn;` 或 `tokio::spawn(...)`）。`use tokio as t;` 這種別名寫法不會被認出來。
- rustfmt、clippy（`-D warnings` 與 `clippy-allow.txt` 放行清單）、結束碼、`stdin`、`no_run`、`warn`、`test`、`compile_fail=E....` 的規則跟 std 區塊完全相同；一個區塊失敗不會影響其他區塊。
- Playground 沒有的 crate 要加 `noplayground`；不確定某個 crate 在 Playground 上能不能跑，先到 Playground 試一次。checkcode 不檢查這件事。
- 版本統一鎖在 `tools/checkcode/deps/Cargo.toml`，`Cargo.lock` 一併提交。新增 feature 或升級版本：改該檔，執行 `cargo update --manifest-path tools/checkcode/deps/Cargo.toml`，再把 `Cargo.lock` 一起提交。不得在書中另外加清單外的 crate。
- 該檔的 `[lints.clippy]` 必須與 `tools/checkcode/clippy-allow.txt` 一致；改放行清單時兩邊一起改（checkcode 的測試會檢查）。
- 第一次編譯所有 crate 需要幾分鐘（`rusqlite` 的 bundled 要有 C 編譯器），之後會使用 `tools/checkcode/deps/target` 裡的快取。CI 也快取這個目錄。

## 正確性

- 以 Rust 1.99、edition 2024 為準。let chains（1.88）、async 閉包（1.85）都已穩定；`static mut` 取參照在 edition 2024 是錯誤；`extern` 區塊要寫 `unsafe extern "C"`，`no_mangle` 要寫 `#[unsafe(no_mangle)]`。
- 本機工具鏈若不是 1.99，用到 1.97 之後才有的功能前，先確認 CI 能過，不要憑印象。
- 不確定時看官方文件（`rustup doc --std`）或實際寫程式驗證。
- 完成後執行 `cargo run --manifest-path tools/checkcode/Cargo.toml -- zh-TW/src/<章節目錄>`，全部通過才算完成。
