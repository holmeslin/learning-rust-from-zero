# `clap` derive

## 本集目標

用 `clap` 的 `#[derive(Parser)]` 把「這個程式有哪些引數」寫成一個 struct，讓 `clap` 負責解析、檢查、產生 `--help` 與錯誤訊息。

## 正文

### 加入 `clap`

`clap` 是 Rust 最常用的命令列解析 crate。第 12 章第 15 集說過，它要打開 `derive` 這個 feature 才有下面的寫法：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = { version = "4.6.7", features = ["derive"] }
```

### 用 struct 描述引數

`#[derive(Parser)]` 是第 16 章第 4 集提過的 derive 巨集：看著 struct 的欄位，替它產生解析命令列的程式。欄位名稱、型別與屬性，就是你對引數的宣告：

`src/main.rs`：

```rust,ignore
use clap::Parser;

/// 飲料店的點單小工具
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// 飲料名稱
    drink: String,
    /// 杯數
    #[arg(short, long, default_value_t = 1)]
    cups: u32,
    /// 加冰
    #[arg(long)]
    iced: bool,
    /// 加料，可以寫很多次
    #[arg(long)]
    topping: Vec<String>,
    /// 備註
    #[arg(short, long)]
    note: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let ice = if cli.iced { "冰" } else { "熱" };
    println!("{} × {}（{ice}）", cli.drink, cli.cups);
    println!("加料：{:?}", cli.topping);
    match cli.note {
        Some(note) => println!("備註：{note}"),
        None => println!("沒有備註"),
    }
}
```

`use clap::Parser;` 把 trait `Parser` 帶進來，`Cli::parse()` 才找得到。`parse()` 讀命令列引數（用的是 `args_os()`，引數不是合法的 UTF-8 也只是一則錯誤訊息，不會 panic），解析成 `Cli`；失敗的話它自己印訊息並結束程式，所以回傳的一定是解析好的 `Cli`。

`cargo run -- 紅茶 -c 2 --iced` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop '紅茶' -c 2 --iced`
紅茶 × 2（冰）
加料：[]
沒有備註
```

`cargo run -- 奶茶 --topping 珍珠 --topping 仙草 --note 少糖` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop '奶茶' --topping '珍珠' --topping '仙草' --note '少糖'`
奶茶 × 1（熱）
加料：["珍珠", "仙草"]
備註：少糖
```

### 欄位怎麼變成引數

| 欄位寫法 | 命令列 |
| --- | --- |
| `drink: String` | 位置引數，必填，照順序放 |
| `#[arg(long)]` | `--欄位名稱`，欄位名稱的底線會變成連字號（`two_cups` 變 `--two-cups`） |
| `#[arg(short)]` | `-` 加欄位名稱的第一個字母，例如 `-c` |
| `bool` | 開關：有寫就是 `true`，不用給值 |
| `Option<T>` | 選填，沒寫就是 `None` |
| `Vec<T>` | 可以重複寫，每次一個值 |
| `default_value_t = 1` | 沒寫時用的預設值，所以 `cups` 不是必填 |

型別同時決定怎麼解析：`u32` 的值靠第 8 章的 `FromStr`（`parse` 背後那個），所以不用自己 `parse`，也不用自己處理錯誤。

### 免費的 `--help`、`--version` 與錯誤訊息

`///` 文件註解（第 12 章第 8 集）會變成說明文字，`#[command(version)]` 從 `Cargo.toml` 取版本號碼（`--version` 印 `teashop 0.1.0`）。

`cargo run -- --help` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop --help`
飲料店的點單小工具

Usage: teashop [OPTIONS] <DRINK>

Arguments:
  <DRINK>  飲料名稱

Options:
  -c, --cups <CUPS>        杯數 [default: 1]
      --iced               加冰
      --topping <TOPPING>  加料，可以寫很多次
  -n, --note <NOTE>        備註
  -h, --help               Print help
  -V, --version            Print version
```

使用者寫錯的時候，`clap` 的訊息比第 1 集手寫的詳細，結束碼一律是 2：

`cargo run -- 紅茶 --cups 兩杯` 的輸出（省略編譯進度，結束碼 2）：

```text
     Running `target/debug/teashop '紅茶' --cups '兩杯'`
error: invalid value '兩杯' for '--cups <CUPS>': invalid digit found in string

For more information, try '--help'.
```

`cargo run` 的輸出（省略編譯進度，結束碼 2）：

```text
     Running `target/debug/teashop`
error: the following required arguments were not provided:
  <DRINK>

Usage: teashop <DRINK>

For more information, try '--help'.
```

`--help` 與 `--version` 是正常結束（結束碼 0）。解析上，位置引數與選項可以交錯、順序不拘（`-c 2 紅茶` 也行），`--cups=2`、`-c2` 也都能認；選項拼錯時，訊息還會建議相近的名稱（`--cup` 會得到 `tip: a similar argument exists: '--cups'`）。只有 `Vec<T>` 欄位的選項能重複寫，`Option<T>` 的選項寫兩次是錯誤。另外 `Usage`、`Options`、`Print help` 這些固定字是英文的，`clap` 能自訂，本書不展開。

### 跟 Go 比一比

Go 的 `flag` 套件一個旗標寫一行，回傳指標，最後呼叫 `flag.Parse()`：

```go
cups := flag.Int("cups", 1, "杯數")
iced := flag.Bool("iced", false, "加冰")
flag.Parse()
drink := flag.Arg(0)
```

- `flag` 遇到第一個不是旗標的引數就停止解析：`./teashop 紅茶 --cups 2` 得到的 `flag.Args()` 是 `["紅茶", "--cups", "2"]`，`cups` 還是預設的 1，不會有任何錯誤。`clap` 的位置引數與選項可以交錯。
- `flag` 沒有短選項與長選項成對的概念（`-c` 與 `--cups`），也不檢查必填的位置引數，要自己 `flag.NArg()`。
- Go 的做法是一個旗標一行、指標回傳；`clap` 的做法是宣告一個型別，解析完拿到的就是有型別的 struct，欄位直接用。

## 重點整理

- `clap` 要 `features = ["derive"]`；`use clap::Parser;`，在 struct 上寫 `#[derive(Parser)]`，`Cli::parse()` 讀命令列、解析成 struct。
- 沒有屬性的欄位是必填的位置引數；`#[arg(short, long)]` 是 `-c` 與 `--cups`；`bool` 是開關，`Option<T>` 選填，`Vec<T>` 可以重複，`default_value_t` 給預設值。
- 值的型別決定解析方式，不合法就由 `clap` 報錯：印訊息到 stderr、結束碼 2。
- `///` 文件註解變成 `--help` 的說明，`#[command(version)]` 提供 `--version`。
- 跟 Go 比：`flag` 遇到第一個非旗標引數就停止解析，`clap` 沒有這個限制。
