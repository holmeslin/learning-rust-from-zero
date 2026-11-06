# stdin/stdout 管線

## 本集目標

讓命令列工具能接在管線上：從 stdin 讀、把結果寫到 stdout、把訊息寫到 stderr，並處理對方提早關掉管線的情況。

## 正文

### 三個標準串流

每個行程一啟動就有三條現成的串流：stdin（輸入）、stdout（輸出）、stderr（錯誤訊息）。Unix 工具的習慣是：

- **結果**寫到 stdout，別的程式或檔案才接得到：`teashop … | sort`、`teashop … > report.txt`。
- **訊息**（警告、錯誤、進度）寫到 stderr：`println!` 是 stdout，`eprintln!` 是 stderr。這樣 `> report.txt` 時，警告仍然出現在螢幕上，不會混進報表裡。
- **輸入**沒指定檔案時從 stdin 讀，使用者就能 `cat orders.txt | teashop …` 或 `teashop … < orders.txt`。

### 一支從 stdin 讀的小工具

下面的程式讀 stdin，每行是 `飲料:杯數`，算出每一行與總金額。看不懂的行印到 stderr 並跳過，最後只要有看不懂的行，結束碼就是 1，讓呼叫的腳本知道「不是全部成功」：

```rust,stdin=紅茶:2\n咖啡:一杯\n奶茶:1,noplayground,exit=1
use std::io::{self, IsTerminal};
use std::process::ExitCode;

fn price_of(drink: &str) -> Option<u32> {
    match drink {
        "紅茶" => Some(30),
        "咖啡" => Some(45),
        "奶茶" => Some(38),
        _ => None,
    }
}

fn parse_order(line: &str) -> Option<(&str, u32, u32)> {
    let (drink, cups) = line.split_once(':')?;
    Some((drink, price_of(drink)?, cups.parse().ok()?))
}

fn main() -> ExitCode {
    if io::stdin().is_terminal() {
        eprintln!("用法：把訂單從 stdin 傳進來，每行「飲料:杯數」");
        return ExitCode::from(2);
    }
    let (mut total, mut bad) = (0, 0);
    for (i, line) in io::stdin().lines().enumerate() {
        let line = line.expect("讀取失敗");
        let Some((drink, price, cups)) = parse_order(&line) else {
            eprintln!("第 {} 行看不懂：{line}", i + 1);
            bad += 1;
            continue;
        };
        println!("{drink} × {cups} = {}", price * cups);
        total += price * cups;
    }
    println!("總共 {total} 元");
    if bad > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
```

輸入 `紅茶:2`、`咖啡:一杯` 和 `奶茶:1` 的執行結果：

```text
紅茶 × 2 = 60
奶茶 × 1 = 38
總共 98 元
第 2 行看不懂：咖啡:一杯
```

書上的輸出把 stdout 放前面、stderr 放後面；在終端機上兩者會交錯出現。

- `io::stdin().lines()` 是第 17 章第 2 集的簡寫，每次取一行，已經去掉換行。
- `line.split_once(':')` 在第一個 `:` 把字串切成前後兩段，回傳 `Option<(&str, &str)>`，沒有 `:` 就是 `None`。`parse_order` 的每一步（切開、查菜單、轉數字）失敗都用 `?` 提早回傳 `None`（第 9 章）。
- 結果用 `println!`，看不懂的行用 `eprintln!`，兩者互不干擾。
- `io::stdin().is_terminal()`（`IsTerminal`，標準函式庫從 1.70 起有）問 stdin 是不是接在終端機上。如果是，使用者是直接執行這支程式、沒有接管線，程式會一直等他打字，看起來像當機，所以先印用法並結束。這次輸入是管線餵進來的，所以是 `false`。
- 結束碼用上一集的 `ExitCode`：2 是用法錯誤，1 是有幾行沒成功。

把它編成 `total`，接在管線上（以下是 shell 指令，結果書上不驗證，但都實際跑過）：

```bash
printf '紅茶:2\n咖啡:一杯\n奶茶:1\n' | total > report.txt
```

螢幕上只出現 stderr 的 `第 2 行看不懂：咖啡:一杯`；stdout 的三行都進了 `report.txt`。反過來 `total 2> errors.txt < orders.txt`，錯誤進 `errors.txt`，結果留在螢幕上。

### 對方提早關掉管線

`teashop … | head -n 1` 這類管線，`head` 讀完第一行就結束，後面再寫的資料沒人收。下面的程式每 0.1 秒印一行：

```rust
use std::thread::sleep;
use std::time::Duration;

fn main() {
    for i in 1..=5 {
        println!("第 {i} 筆");
        sleep(Duration::from_millis(100));
    }
}
```

執行結果：

```text
第 1 筆
第 2 筆
第 3 筆
第 4 筆
第 5 筆
```

接上 `head -n 1`（假設編成 `slow`）：

```bash
slow | head -n 1
```

會看到：

```text
第 1 筆

thread 'main' (123456) panicked at …/library/std/src/io/stdio.rs:1168:9:
failed printing to stdout: Broken pipe (os error 32)
```

（執行緒編號每次不同，路徑夾著編譯器版本的雜湊，書上略過。）Rust 啟動時把 SIGPIPE 設成忽略，所以寫入已關閉的管線不會殺掉程式，而是得到 `BrokenPipe` 錯誤，`println!` 遇到錯誤就 panic，結束碼 101。這對正常使用者不是好行為：`| head` 本來就是正常用法。要處理的話，改用 `writeln!`（第 17 章第 1 集），自己看錯誤：

```rust
use std::io::{self, ErrorKind, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), io::Error> {
    let mut out = io::stdout().lock();
    for i in 1..=5 {
        match writeln!(out, "第 {i} 筆") {
            Ok(()) => {}
            Err(e) if e.kind() == ErrorKind::BrokenPipe => return Ok(()),
            Err(e) => return Err(e),
        }
        sleep(Duration::from_millis(100));
    }
    Ok(())
}
```

`BrokenPipe` 是下游不要了，安靜結束就好；其他錯誤照常回傳。接上 `head -n 1` 只看到第一行，沒有任何錯誤訊息，結束碼 0。

### 跟 Go 比一比

```go
fi, _ := os.Stdin.Stat()
if fi.Mode()&os.ModeCharDevice != 0 {
	fmt.Fprintln(os.Stderr, "stdin 是終端機")
}
```

- Go 要用 `Stat` 加 `ModeCharDevice` 判斷 stdin 是不是終端機，Rust 是一個方法 `is_terminal()`。
- 寫到已關閉的管線，Go 的行為跟傳統的 Unix 工具一樣：標準輸出或標準錯誤寫入失敗時，runtime 直接讓程式被 SIGPIPE 結束，悄悄死掉（接上 `head -n 1`，shell 看到的狀態是 141）。Rust 預設忽略 SIGPIPE，`println!` 會 panic；想要安靜結束，要自己處理 `BrokenPipe`。

## 重點整理

- 結果寫 stdout（`println!`），訊息寫 stderr（`eprintln!`），使用者才能 `>`、`|` 接結果、`2>` 另外處理訊息。
- 沒指定檔案時從 stdin 讀；`io::stdin().is_terminal()` 判斷 stdin 是不是終端機，是的話印用法，不要讓程式默默等待輸入。
- 看不懂的輸入行印到 stderr 後繼續，最後用 `ExitCode` 讓呼叫端知道有沒有全部成功。
- Rust 預設忽略 SIGPIPE：對方提早關掉管線時，`println!` 會 panic（結束碼 101）；改用 `writeln!` 並把 `ErrorKind::BrokenPipe` 當成正常結束。
- 跟 Go 比：Go 寫到關閉的管線會被 SIGPIPE 悄悄結束；Go 用 `Stat` 判斷終端機，Rust 用 `is_terminal()`。
