# doc test

## 本集目標

在文件註解裡寫程式範例，讓 `cargo test` 把它們當成測試執行，並知道這只對 library crate 有效。

## 正文

### 文件裡的範例會被執行

第 8 集說過，`///` 裡的小標題 `# Examples` 底下可以放範例。範例用三個反引號圍起來，Markdown 的程式區塊語法，放在 `///` 裡面：

`src/lib.rs`：

````rust,ignore
/// 計算 `cups` 杯 `drink` 的總價，單位是元。
///
/// # Examples
///
/// ```
/// let total = teashop::total_for("奶茶", 2);
/// assert_eq!(total, 76);
/// ```
pub fn total_for(drink: &str, cups: u32) -> u32 {
    let price = match drink {
        "紅茶" => 30,
        "奶茶" => 38,
        _ => 45,
    };
    price * cups
}
````

`cargo doc` 會把這段範例顯示在網頁上，`cargo test` 也會跑它：

`cargo test` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-32c5571f53fea115)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests teashop

running 1 test
test src/lib.rs - total_for (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

all doctests ran in 0.49s; merged doctests compilation took 0.12s
```

`Doc-tests teashop` 底下多了一個測試，名字是「檔案 - 函式（第幾行）」，括號裡的行號，是範例開頭那行三個反引號在 `lib.rs` 的第幾行。上面的 `unittests` 是 0 個，因為這個 `lib.rs` 沒有單元測試。最後一行是所有範例編譯與執行花的時間，每次不同。

這就是 doc test 的好處：**文件上的範例不會悄悄過時**。有人改了 `total_for` 的行為，範例的 `assert_eq!` 失敗，`cargo test` 就會報錯，逼你連文件一起更新。

### 每個範例是一支獨立的小程式

doc test 的規則，跟第 12 集的整合測試很像：

- 每個範例各自編譯成一支小程式，**把你的 library 當外部 crate 使用**，所以要寫 `teashop::total_for`，或在範例裡先 `use teashop::total_for;`，而且只能用 `pub` 的東西。
- 範例不用自己寫 `fn main`，工具會把整段包進 `fn main() { ... }`。
- 範例沒有 panic 就算通過；用 `assert_eq!` 檢查結果，就跟一般測試一樣。

用 `cargo test --doc` 可以只跑 doc test。

### 圍欄上的標記

三個反引號後面可以加標記，改變這個範例怎麼處理：

| 標記 | 意思 |
| --- | --- |
| `no_run` | 只編譯，不執行（會一直跑或需要外部環境的範例） |
| `ignore` | 不編譯、不執行 |
| `should_panic` | 範例要 panic 才算通過 |
| `text` | 不是 Rust 程式碼，只是一段文字 |

範例裡以 `# ` 開頭的行，網頁上會隱藏，但編譯的時候仍然存在，適合放 `fn main` 之類不想讓讀者分心的樣板。

### 只對 library crate 有效

**doc test 只對 library crate 有效。** 只有 `main.rs` 的專案，`///` 裡的範例不會被執行：`cargo test` 不會有 `Doc-tests` 那一段，單獨執行 `cargo test --doc` 會直接說 `no library targets found in package`（這個 package 沒有 library）。想讓範例被測試，就得把邏輯放進 `lib.rs`，這也是第 2 集建議的分法。

## 重點整理

- 文件註解裡用三個反引號圍起來的程式碼，`cargo test` 會編譯並執行，叫 doc test；範例壞了測試就失敗，所以文件不會過時。
- 每個範例是獨立的小程式，把 library 當外部 crate 用，要寫 `teashop::...`，只能用 `pub` 的東西；不用自己寫 `fn main`。
- 圍欄標記：`no_run`、`ignore`、`should_panic`、`text`；`# ` 開頭的行在網頁上隱藏、編譯時保留。
- `cargo test --doc` 只跑 doc test。
- doc test 只對 library crate 有效，只有 `main.rs` 的專案不會跑，`cargo test --doc` 會說找不到 library。
