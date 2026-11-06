# `include_str!`

## 本集目標

會用 `include_str!` 在編譯時期把文字檔讀進程式，知道路徑怎麼算、檔案改了會發生什麼事，也會區分它跟執行時讀檔的差別。

## 正文

### 把檔案放進執行檔

飲料店的菜單放在文字檔裡比較好改。如果希望程式一編譯出來，菜單就在裡面、不用另外帶著檔案跑，可以用 `include_str!`：它在**編譯時期**讀進一個檔案，結果是 `&'static str`（第 10 章第 7 集），內容直接寫進執行檔。

先寫主程式，這時候還沒有菜單檔：

`src/main.rs`：

```rust,ignore
const MENU: &str = include_str!("menu.txt");

fn main() {
    for line in MENU.trim().split('\n') {
        println!("- {line}");
    }
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
error: couldn't read `src/menu.txt`: No such file or directory (os error 2)
 --> src/main.rs:1:20
  |
1 | const MENU: &str = include_str!("menu.txt");
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^
```

找不到檔案是編譯錯誤，沒有錯誤碼，而且在執行之前就發生。路徑 `"menu.txt"` 是**相對於這個原始檔**，不是相對於你執行指令的位置，所以是 `src/menu.txt`。把檔案補上：

`src/menu.txt`：

```text
紅茶 30
咖啡 45
奶茶 38
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
- 紅茶 30
- 咖啡 45
- 奶茶 38
```

### 檔案改了，要重新編譯

菜單已經被編進執行檔，改檔案不會影響已經編好的程式。不過 cargo 記得這支程式用到了 `src/menu.txt`，檔案一改，下次 `cargo run` 就會自動重新編譯：

`src/menu.txt`：

```text
紅茶 30
咖啡 45
奶茶 38
綠茶 28
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
- 紅茶 30
- 咖啡 45
- 奶茶 38
- 綠茶 28
```

### 路徑與相關的巨集

- 路徑相對於**呼叫巨集的那個原始檔**：寫在 `src/shop/mod.rs` 裡的 `include_str!("menu.txt")`，找的是 `src/shop/menu.txt`。想從 package 的最上層算起，可以用 `concat!(env!("CARGO_MANIFEST_DIR"), "/menu.txt")` 組出完整路徑。
- 檔案必須是合法的 UTF-8，否則是編譯錯誤。要嵌入任意位元組（圖片、字型），用 `include_bytes!`，結果是 `&'static [u8; N]`。
- `include!("檔案.rs")` 則是把另一個檔案當成 Rust 程式碼塞進來。上一集 `build.rs` 產生的程式碼，就是用 `include!` 讀進來的。
- 標準函式庫沒有「整個資料夾」的版本，這本書不展開。

跟執行時用 `std::fs` 讀檔（第 17 章）比起來，`include_str!` 的好處是只剩一個執行檔、不會有執行時找不到檔案、讀檔失敗的問題；代價是內容改了就得重新編譯，而且檔案越大，執行檔也越大。程式一直在改的設定，適合執行時讀；跟程式一起發行的固定資料（預設菜單、說明文字、範本），適合 `include_str!`。

### 跟 Go 比一比

Go 的對應物是 `//go:embed`：

```go
//go:embed menu.txt
var menu string
```

要在檔案最上面 `import _ "embed"`，註解必須緊貼在套件層級的變數上面，路徑相對於這個 `.go` 檔所在的資料夾，而且不能用 `..` 跑到外面去。想嵌入整個資料夾，Go 有 `embed.FS`。兩邊的精神一樣：檔案在編譯時期進入執行檔。差別是 Go 靠一行特殊的註解，Rust 的 `include_str!` 是普通的巨集，可以放在運算式裡，也可以搭配 `const`。

## 重點整理

- `include_str!("路徑")` 在編譯時期讀檔，結果是 `&'static str`，內容寫進執行檔；`include_bytes!` 對應任意位元組。
- 路徑相對於呼叫巨集的原始檔；找不到檔案或不是合法的 UTF-8，是編譯錯誤。
- 嵌入的檔案一改，cargo 會自動重新編譯；已經編好的執行檔不會跟著變。
- 固定的資料用 `include_str!`，經常要改的設定用執行時讀檔（第 17 章）。
- Go 用 `//go:embed` 註解加套件層級的變數；Rust 是普通的巨集。
