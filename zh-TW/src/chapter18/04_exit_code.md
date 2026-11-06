# 結束碼 `ExitCode`

## 本集目標

知道結束碼是程式留給 shell 與其他程式看的結果，會從 `main` 回傳 `ExitCode`，並把不同的錯誤對應到固定的結束碼。

## 正文

### 結束碼是給別人看的

命令列工具的成敗，不是靠它印了什麼，而是靠結束碼：`0` 是成功，其他都是失敗。shell 的 `&&`、`if`、CI 的每個步驟，都是看這個數字決定下一步。慣例（沒有統一規定）是 `1` 一般失敗、`2` 用法錯誤（`clap` 用的就是這個）、`101` Rust 的 panic（第 9 章）、`130` 被 Ctrl-C 中斷（第 7 集）；其餘的工具自己定。`teashop` 的規定：`3` 是沒有這個飲料，`4` 是杯數太多。

### `process::exit` 與 `ExitCode`

第 1 集用的 `std::process::exit` 會立刻結束整個程式，`main` 裡還活著的值不會被 drop（第 17 章第 7 集）。從 `main` 回傳 `ExitCode` 就不一樣，`main` 是正常結束，該 drop 的都會 drop。下面的程式在 `main` 裡放一個 drop 時會印字的 `Till`，沒帶引數就回傳 `ExitCode`，有帶引數就呼叫 `exit`（`args().len()` 是引數個數，含程式名稱）：

`src/main.rs`：

```rust,ignore
use std::process::ExitCode;

struct Till;

impl Drop for Till {
    fn drop(&mut self) {
        println!("收銀機已關");
    }
}

fn main() -> ExitCode {
    let _till = Till;
    println!("開始營業");
    if std::env::args().len() > 1 {
        std::process::exit(3);
    }
    ExitCode::from(3)
}
```

`cargo run` 的輸出（省略編譯進度，結束碼 3）：

```text
     Running `target/debug/teashop`
開始營業
收銀機已關
```

`cargo run -- exit` 的輸出（省略編譯進度，結束碼 3）：

```text
     Running `target/debug/teashop exit`
開始營業
```

結束碼一樣是 3，差在「收銀機已關」：`exit` 那條路沒有 drop，`Till` 如果要把資料寫回檔案、關連線，就會漏掉。`ExitCode` 有 `ExitCode::SUCCESS`（0）、`ExitCode::FAILURE`（1）與 `ExitCode::from(n)`，`n` 是 `u8`，所以結束碼只能是 0 到 255。

### 把錯誤對應到結束碼

實際的工具會這樣分工：真正的工作寫在 `run`，回傳 `Result`；`main` 只負責呼叫它，把結果轉成輸出與結束碼。錯誤型別用第 17 章第 13 集的 `thiserror`，再替每個 variant 配一個結束碼：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
clap = { version = "4.6.7", features = ["derive"] }
thiserror = "2.0.21"
```

`src/main.rs`：

```rust,ignore
use std::process::ExitCode;

use clap::Parser;
use thiserror::Error;

/// 算出一筆訂單的金額
#[derive(Parser)]
struct Cli {
    /// 飲料名稱
    drink: String,
    /// 杯數
    #[arg(short, long, default_value_t = 1)]
    cups: u32,
}

#[derive(Debug, Error)]
enum Failure {
    #[error("沒有這個飲料：{0}")]
    UnknownDrink(String),
    #[error("一次最多 20 杯，你點了 {0} 杯")]
    TooManyCups(u32),
}

impl Failure {
    fn code(&self) -> u8 {
        match self {
            Failure::UnknownDrink(_) => 3,
            Failure::TooManyCups(_) => 4,
        }
    }
}

fn run(cli: &Cli) -> Result<u32, Failure> {
    let price = match cli.drink.as_str() {
        "紅茶" => 30,
        "咖啡" => 45,
        "奶茶" => 38,
        _ => return Err(Failure::UnknownDrink(cli.drink.clone())),
    };
    if cli.cups > 20 {
        return Err(Failure::TooManyCups(cli.cups));
    }
    Ok(price * cli.cups)
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(total) => {
            println!("{} × {} = {total}", cli.drink, cli.cups);
            ExitCode::SUCCESS
        }
        Err(failure) => {
            eprintln!("{failure}");
            ExitCode::from(failure.code())
        }
    }
}
```

`cargo run -- 烏龍` 的輸出（省略編譯進度，結束碼 3）：

```text
     Running `target/debug/teashop '烏龍'`
沒有這個飲料：烏龍
```

`cargo run -- 紅茶 -c 50` 的輸出（省略編譯進度，結束碼 4）：

```text
     Running `target/debug/teashop '紅茶' -c 50`
一次最多 20 杯，你點了 50 杯
```

成功時印結果、結束碼 0；`clap` 處理的用法錯誤是 2，加上這裡的 3、4，呼叫 `teashop` 的腳本就能依結束碼分辨出了什麼事。

### `main` 還能回傳什麼

`main` 的回傳型別不只 `ExitCode`：`()`（結束碼 0）、`Result<(), E>`（第 9 章第 11 集：`Err` 時印 `Error: ` 加 `Debug` 格式，結束碼 1，`E` 要實作 `Debug`）、`Result<ExitCode, E>` 都可以。它們都實作了標準函式庫的 `Termination`，要交代結束碼又要印自己格式的訊息，就像上面那樣回傳 `ExitCode`。

### 跟 Go 比一比

Go 的 `main` 沒有回傳值，要有結束碼只能 `os.Exit`，而它跟 `process::exit` 一樣，立刻結束，**`defer` 不會執行**。慣用寫法是把工作放進 `run`，`main` 只做 `os.Exit(run())`，讓 `run` 裡的 `defer` 在回傳時跑完。Rust 的 `main` 直接回傳 `ExitCode`，不用多包一層。

另外，Go 的 `panic` 結束碼是 2，跟用法錯誤撞在一起；Rust 的 panic 是 101，跟 `clap` 的 2、自己定義的小數字都分得開。

## 重點整理

- 結束碼 0 是成功，其他是失敗，shell 與 CI 靠它判斷；慣例上 1 是一般失敗、2 是用法錯誤、101 是 Rust 的 panic、130 是被 Ctrl-C 中斷。
- `std::process::exit(n)` 立刻結束，不執行 drop；`fn main() -> ExitCode` 是正常結束，該 drop 的都會 drop。
- `ExitCode::SUCCESS`、`ExitCode::FAILURE`、`ExitCode::from(n)`（`n` 是 `u8`）。
- 慣用結構：`run` 回傳 `Result`，錯誤 enum 配上 `code()` 方法，`main` 把結果轉成輸出與 `ExitCode`。
- `main` 能回傳 `()`、`ExitCode`、`Result<(), E>`，它們都實作了 `Termination`；Go 的 `os.Exit` 跟 `process::exit` 一樣不執行 `defer`。
