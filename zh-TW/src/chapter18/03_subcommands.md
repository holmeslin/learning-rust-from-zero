# 子命令

## 本集目標

用 `#[derive(Subcommand)]` 讓一支程式有好幾個子命令（像 `git commit`、`cargo run`），並用 `match` 處理每一個。

## 正文

### 一個 enum 就是一組子命令

工具功能一多，就會把它們分成子命令：`teashop menu` 列出菜單，`teashop price 紅茶 -c 2` 算金額。在 `clap` 裡，每個子命令是 enum 的一個 variant，欄位就是那個子命令自己的引數。為了不讓 `main.rs` 太長，這一集把解析引數的型別搬到 `src/cli.rs`（第 12 章第 6 集的做法），先看 `Cargo.toml` 與 `cli.rs`：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = { version = "4.6.7", features = ["derive"] }
```

`src/cli.rs`：

```rust,ignore
use clap::{Parser, Subcommand};

/// 飲料店的小工具
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// 列出菜單
    Menu,
    /// 算出一筆訂單的金額
    Price {
        /// 飲料名稱
        drink: String,
        /// 杯數
        #[arg(short, long, default_value_t = 1)]
        cups: u32,
    },
}
```

- `Cli` 沒有自己的引數，只有一個欄位 `command`，加上 `#[command(subcommand)]`，意思是「剩下的引數交給這個 enum 解析」。
- `#[derive(Subcommand)]` 的 enum：variant 名稱轉成小寫變成子命令名稱（`Menu` 是 `menu`，名稱有兩個字的 `OpenShop` 會是 `open-shop`）。沒有欄位的 `Menu` 不吃引數；有欄位的 `Price { drink, cups }` 就是上一集的欄位寫法，`#[arg]` 的屬性都一樣能用。
- `///` 文件註解變成子命令的說明。

### 用 `match` 分派

`main` 解析完，`match cli.command` 對每個 variant 各寫一個分支。菜單放在一個 `const` 陣列裡，用第 11 章的 `find` 查價格：

`src/main.rs`：

```rust,ignore
mod cli;

use clap::Parser;
use cli::{Cli, Command};

const MENU: [(&str, u32); 3] = [("紅茶", 30), ("咖啡", 45), ("奶茶", 38)];

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Menu => {
            for (name, price) in MENU {
                println!("{name} {price}");
            }
        }
        Command::Price { drink, cups } => match MENU.iter().find(|(name, _)| *name == drink) {
            Some((_, price)) => println!("{drink} × {cups} = {}", price * cups),
            None => {
                eprintln!("沒有這個飲料：{drink}");
                std::process::exit(1);
            }
        },
    }
}
```

`cargo run -- menu` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop menu`
紅茶 30
咖啡 45
奶茶 38
```

`cargo run -- price 紅茶 -c 2` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop price '紅茶' -c 2`
紅茶 × 2 = 60
```

`cargo run -- price 烏龍` 的輸出（省略編譯進度，結束碼 1）：

```text
     Running `target/debug/teashop price '烏龍'`
沒有這個飲料：烏龍
```

`match` 要窮盡（第 6 章）：之後在 `Command` 加一個 variant，沒有替它寫分支的話，`main` 就編譯不過，不會有哪個子命令被悄悄漏掉。

### 子命令的 `--help` 與錯誤訊息

`cargo run -- --help` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop --help`
飲料店的小工具

Usage: teashop <COMMAND>

Commands:
  menu   列出菜單
  price  算出一筆訂單的金額
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

每個子命令有自己的 `--help`（`teashop price --help`，寫成 `teashop help price` 也行，`clap` 自動加上 `help` 子命令）。子命令拼錯，`clap` 會報錯並建議相近的名稱；一個子命令都沒給，它就印出上面那份說明，兩種情況的結束碼都是 2：

`cargo run -- pirce 紅茶` 的輸出（省略編譯進度，結束碼 2）：

```text
     Running `target/debug/teashop pirce '紅茶'`
error: unrecognized subcommand 'pirce'

  tip: a similar subcommand exists: 'price'

Usage: teashop <COMMAND>

For more information, try '--help'.
```

### 跟 Go 比一比

Go 的標準函式庫沒有子命令，常見的寫法是每個子命令一個 `FlagSet`，再用 `switch` 看 `os.Args[1]`：

```go
switch os.Args[1] {
case "menu":
	menuCmd.Parse(os.Args[2:])
case "price":
	priceCmd.Parse(os.Args[2:])
default:
	fmt.Fprintln(os.Stderr, "未知的子命令")
	os.Exit(2)
}
```

- 新增子命令時，忘了在 `switch` 加一個 `case`，Go 照樣編譯，執行時才發現。Rust 的子命令是 enum，`match` 少一個分支就是編譯錯誤。
- 子命令的說明文字、`--help`、拼錯時的建議，Go 都得自己寫；`clap` 從 enum 與文件註解一起產生。

## 重點整理

- 子命令寫成 `#[derive(Subcommand)]` 的 enum，在 `Cli` 裡用 `#[command(subcommand)]` 的欄位接住它；variant 名稱轉小寫（`OpenShop` 變 `open-shop`）成為子命令名稱。
- 沒有欄位的 variant 不吃引數，有欄位的 variant，欄位就是子命令自己的引數，寫法跟上一集的 struct 相同。
- 用 `match cli.command` 分派，每個 variant 一個分支；窮盡性檢查讓新增子命令時不會漏掉處理。
- `clap` 替每個子命令產生 `--help`，也提供 `help` 子命令；子命令拼錯會報錯，沒給則印出說明，結束碼都是 2。
- 跟 Go 比：Go 要自己用 `FlagSet` 加 `switch` 分派，漏掉的 `case` 不會被編譯器發現。
