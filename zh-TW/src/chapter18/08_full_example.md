# 完整範例

## 本集目標

把這一章的零件組起來，做成一支完整的命令列工具：`clap` 解析引數與子命令、從檔案或 stdin 讀、錯誤對應到結束碼，再用 `cargo test` 把行為守住。

## 正文

### 檔案怎麼分

這一版的 `teashop` 有兩個子命令：`price` 查價格，`total` 加總一份訂單（給檔案就讀檔案，不給或寫 `-` 就讀 stdin）。程式分成幾個檔案：`src/cli.rs` 放引數的型別（第 2、3 集）、`src/orders.rs` 放菜單與訂單的計算（不碰命令列也不碰檔案）、`src/main.rs` 把它們串起來，`tests/cli.rs` 放整合測試。

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
use std::path::PathBuf;

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
    /// 算出一筆訂單的金額
    Price {
        /// 飲料名稱
        drink: String,
        /// 杯數
        #[arg(short, long, default_value_t = 1)]
        cups: u32,
    },
    /// 加總訂單檔，每行「飲料:杯數」；沒給檔案或寫 `-` 就讀 stdin
    Total { file: Option<PathBuf> },
}
```

### 計算與讀取分開

`sum_orders` 收 `impl BufRead`（第 17 章第 2 集）：檔案、stdin、記憶體裡的 `&[u8]` 都行，計算本身不知道資料從哪來，也不印任何東西：

`src/orders.rs`：

```rust,ignore
use std::io::{self, BufRead};

pub fn price_of(drink: &str) -> Option<u32> {
    match drink {
        "紅茶" => Some(30),
        "咖啡" => Some(45),
        "奶茶" => Some(38),
        _ => None,
    }
}

/// 一行「飲料:杯數」的金額；格式不對，或飲料不在菜單上，就是 `None`。
fn line_total(line: &str) -> Option<u32> {
    let (drink, cups) = line.split_once(':')?;
    Some(price_of(drink)? * cups.parse::<u32>().ok()?)
}

/// 加總一份訂單，回傳（總金額，看不懂的行號）。
pub fn sum_orders(reader: impl BufRead) -> io::Result<(u32, Vec<usize>)> {
    let mut total = 0;
    let mut bad = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        match line_total(&line?) {
            Some(amount) => total += amount,
            None => bad.push(i + 1),
        }
    }
    Ok((total, bad))
}
```

### `main` 把零件接起來

`run` 回傳 `Result<bool, …>`：`Ok(true)` 全部成功，`Ok(false)` 有看不懂的行（結束碼 1），`Err` 是真的做不下去（結束碼 3）。`clap` 自己處理的用法錯誤是 2。結果寫 stdout，訊息寫 stderr（第 5 集）。出錯的訊息只印 `kind()`，輸出才不依賴作業系統：

`src/main.rs`：

```rust,ignore
mod cli;
mod orders;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, Command};

/// 有給檔案（而且不是 `-`）就讀檔案，否則讀 stdin。
fn open(file: Option<PathBuf>) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match file {
        Some(path) if path.to_str() != Some("-") => {
            let file = File::open(&path)
                .map_err(|e| format!("開不了 {}：{:?}", path.display(), e.kind()))?;
            Ok(Box::new(BufReader::new(file)))
        }
        _ => Ok(Box::new(io::stdin().lock())),
    }
}

fn run(cli: Cli) -> Result<bool, Box<dyn Error>> {
    match cli.command {
        Command::Price { drink, cups } => {
            let Some(price) = orders::price_of(&drink) else {
                return Err(format!("沒有這個飲料：{drink}").into());
            };
            println!("{drink} × {cups} = {}", price * cups);
            Ok(true)
        }
        Command::Total { file } => {
            let (total, bad) = orders::sum_orders(open(file)?)?;
            println!("總共 {total} 元");
            for line in &bad {
                eprintln!("第 {line} 行看不懂");
            }
            Ok(bad.is_empty())
        }
    }
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(3)
        }
    }
}
```

`cargo run -- price 紅茶 -c 2` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop price '紅茶' -c 2`
紅茶 × 2 = 60
```

`cargo run -- total no-such.txt` 的輸出（省略編譯進度，結束碼 3）：

```text
     Running `target/debug/teashop total no-such.txt`
開不了 no-such.txt：NotFound
```

### 用測試守住行為

結束碼與管線要啟動真的程式才測得到：`tests/` 底下的整合測試（第 12 章第 12 集）可以用 `env!("CARGO_BIN_EXE_teashop")` 拿到編好的程式路徑：這個環境變數由 cargo 在編譯時期設定，`env!` 見第 16 章第 11 集。拿到路徑，就用第 17 章第 7 集的 `Command` 啟動它、餵 stdin、收輸出。`String::from_utf8_lossy` 把位元組轉成文字（第 13 章第 10 集），拿來跟字串比對：

`tests/cli.rs`：

```rust,ignore
use std::io::Write;
use std::process::{Command, Output, Stdio};

fn teashop(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_teashop"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("無法執行 teashop");
    let mut stdin = child.stdin.take().expect("沒有 stdin");
    stdin.write_all(input.as_bytes()).expect("寫入失敗");
    drop(stdin);
    child.wait_with_output().expect("等待失敗")
}

#[test]
fn total_reads_stdin_and_reports_bad_lines() {
    let output = teashop(&["total"], "紅茶:2\n咖啡:一杯\n");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "總共 60 元\n");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "第 2 行看不懂\n");
    assert_eq!(output.status.code(), Some(1));
}
```

`cargo test` 的輸出（省略編譯進度）：

```text
     Running unittests src/main.rs (target/debug/deps/teashop-6a6bb165f5e07371)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/cli.rs (target/debug/deps/cli-333a240a8a6eb114)

running 1 test
test total_reads_stdin_and_reports_bad_lines ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s
```

輸出裡 `teashop-`、`cli-` 後面的雜湊與秒數，每個人都不同。第一段是 `src/` 裡的單元測試（第 12 章第 10 集），這個專案沒有，所以是 0 個；第二段才是 `tests/cli.rs` 的整合測試。

### 還沒接上的零件

第 6 集的 `walk` 放進 `src/scan.rs`、在 `Command` 加一個 `Scan` variant，`run` 的 `match` 就會要求你補上分支：窮盡檢查替你列出所有漏接的地方。第 7 集的 Ctrl-C 則是把 `main` 改成 `#[tokio::main]`，批次處理的迴圈照那一集檢查 `CancellationToken`。這支工具跑得很快，用不上，所以這一集沒放進去。

## 重點整理

- 命令列工具的分工：`cli.rs` 只描述引數、`orders.rs` 只做計算、`main.rs` 把 `Result` 轉成輸出與結束碼；計算函式收 `impl BufRead`，檔案、stdin、`&[u8]` 都能傳，不印東西。
- 結束碼的約定：0 成功、1 資料有問題、2 用法錯誤（`clap`）、3 做不下去；成功的結果寫 stdout，訊息寫 stderr。
- 檔案或 stdin 二選一，用 `Box<dyn BufRead>`；輸出不依賴作業系統，錯誤只印 `kind()`。
- `tests/` 的整合測試可以用 `env!("CARGO_BIN_EXE_<名稱>")` 啟動編好的程式，驗證結束碼、stdout、stderr，也能餵 stdin。
- 新增子命令：`Command` 加 variant，`run` 的 `match` 補分支，編譯器會指出所有沒處理的地方。
