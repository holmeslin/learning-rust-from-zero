# `std::env::args`

## 本集目標

知道命令列的引數怎麼交給程式、用 `std::env::args()` 讀出來，並看出自己手寫解析的極限。

## 正文

這一章要把前面學過的東西組起來，寫出一支真正的命令列工具：飲料店的 `teashop`。第一版不靠任何 crate，只用標準函式庫，先看看最原始的做法能走多遠。

### 引數從哪裡來

用 `cargo new teashop` 建好專案之後，執行：

```bash
cargo run -- 紅茶 2
```

`--` 是 cargo 的約定：它前面的是給 cargo 的選項，後面的原封不動交給你的程式。直接執行編好的 `target/debug/teashop 紅茶 2` 效果一樣。程式裡用 `std::env::args()` 拿到它們：

`src/main.rs`：

```rust,ignore
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    println!("{args:?}");
}
```

`cargo run -- 紅茶 2` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop '紅茶' 2`
["紅茶", "2"]
```

`Running` 那行是 cargo 印的，它把非 ASCII 的引數加上單引號來顯示，只是顯示的樣子：程式收到的引數沒有引號。

`args()` 回傳一個迭代器，每個元素是 `String`（第 11 章）。**第一個元素是程式自己的名稱**（你是怎麼啟動它的，這裡是 `target/debug/teashop`），真正的引數從第二個開始，所以先 `skip(1)` 跳過第一個（它跟 `map`、`filter` 一樣是迭代器的方法），再 `collect` 成 `Vec<String>`。

引數一律是字串：`2` 在這裡是 `"2"`，要當數字用得自己 `parse`。shell 會先用空白把一行切開，要把含空白的飲料名稱當成一個引數，得加引號，例如 `"珍珠 奶茶"`。

### 第一版：兩個固定位置的引數

飲料名稱在前、杯數在後。引數個數不對、杯數不是整數、飲料不在菜單上，都要印出說明然後結束。錯誤訊息用 `eprintln!` 印：它跟 `println!` 一樣，只是寫到 stderr，第 5 集再細講。結束用 `std::process::exit(2)`（第 17 章第 7 集）。這集三種錯誤都用結束碼 2，意思是「引數有問題」，第 4 集再把它們分開：

`src/main.rs`：

```rust,ignore
fn price_of(drink: &str) -> Option<u32> {
    match drink {
        "紅茶" => Some(30),
        "咖啡" => Some(45),
        "奶茶" => Some(38),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [drink, cups] = args.as_slice() else {
        eprintln!("用法：teashop <飲料> <杯數>");
        std::process::exit(2);
    };
    let Ok(cups) = cups.parse::<u32>() else {
        eprintln!("杯數要是整數：{cups}");
        std::process::exit(2);
    };
    let Some(price) = price_of(drink) else {
        eprintln!("沒有這個飲料：{drink}");
        std::process::exit(2);
    };
    println!("{drink} × {cups} = {}", price * cups);
}
```

`args.as_slice()` 把 `Vec` 當成 slice，就能用第 6 章的 slice 模式 `[drink, cups]` 一次檢查個數、取出兩個元素；個數不是 2，走 `let else` 的 `else` 離開。

`cargo run -- 紅茶 2` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop '紅茶' 2`
紅茶 × 2 = 60
```

三種出錯的情況，訊息都從 stderr 出來，程式以結束碼 2 結束。Linux 與 macOS 的 `cargo run` 不會印出結束碼，所以書上把它寫在標題的括號裡：

`cargo run -- 紅茶` 的輸出（省略編譯進度，結束碼 2）：

```text
     Running `target/debug/teashop '紅茶'`
用法：teashop <飲料> <杯數>
```

`cargo run -- 紅茶 兩杯` 的輸出（省略編譯進度，結束碼 2）：

```text
     Running `target/debug/teashop '紅茶' '兩杯'`
杯數要是整數：兩杯
```

`cargo run -- 烏龍 1` 的輸出（省略編譯進度，結束碼 2）：

```text
     Running `target/debug/teashop '烏龍' 1`
沒有這個飲料：烏龍
```

### 手寫解析的極限

這一版能用，但再多一點需求就吃力了：

- 引數的位置是死的。想讓使用者寫 `--cups 2` 或 `-c 2`、順序可以對調，就得自己掃整份清單、比對字串。
- 沒有 `--help`。使用者打 `teashop --help`，只會被當成「引數個數不對」，得到上面那一行用法。
- 每一種錯誤的訊息、每一個要檢查的情況都得自己寫。

這些都是每個命令列工具要的功能，下一集換成 crate `clap`，宣告「我要哪些引數」，剩下的由它處理。

另外，`args()` 要求每個引數都是合法的 UTF-8，否則會 panic（實測：``called `Result::unwrap()` on an `Err` value: "\xFF"``，結束碼 101）。作業系統其實允許任意的位元組，需要處理這種情況（例如檔名）時，改用 `std::env::args_os()`，元素是 `OsString`。

### 跟 Go 比一比

```go
cups, err := strconv.Atoi(os.Args[2])
```

- Go 的 `os.Args[0]` 同樣是程式名稱，真正的引數從 `os.Args[1]` 開始，Rust 也一樣；不同的是 Go 的 `os.Args` 是切片，`os.Args[2]` 沒給就 panic，Rust 的 `args()` 是迭代器，沒有索引，一定得用 `next`、`collect` 之類的方法取出來。
- Go 的標準函式庫有 `flag` 套件，能解析 `-cups 2`；Rust 的標準函式庫沒有，命令列解析交給 crate，最常用的就是 `clap`。
- Go 的字串可以是任意位元組，所以 `os.Args` 不會因為編碼出事；Rust 的 `args()` 遇到不是 UTF-8 的引數會 panic，要用 `args_os()` 才安全。

## 重點整理

- `cargo run -- 引數…` 的 `--` 之後交給你的程式；`std::env::args()` 回傳每個元素是 `String` 的迭代器，第一個是程式自己的名稱，所以常寫 `args().skip(1).collect::<Vec<String>>()`。
- 引數一律是字串，數字要自己 `parse`；用 `args.as_slice()` 搭配 slice 模式 `[a, b]` 能同時檢查個數與取值。
- 錯誤訊息用 `eprintln!` 寫到 stderr，用 `std::process::exit(2)` 以結束碼 2 結束。
- 手寫解析的引數位置固定、沒有 `--help`、錯誤訊息都要自己寫，所以實務上用 `clap`（第 2 集）。
- `args()` 遇到不是 UTF-8 的引數會 panic，改用 `args_os()`；Go 的 `os.Args` 沒有這個問題，標準函式庫也有 `flag`，Rust 沒有。
