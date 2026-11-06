# `std::process::Command`

## 本集目標

用 `Command` 從 Rust 程式啟動外部程式、傳引數與環境變數、收集輸出與結束碼，並分清楚「啟動失敗」與「跑完但失敗」是兩回事。

## 正文

`Command` 採建構者風格，一路用方法串下去來描述要跑的程式，最後呼叫 `output()`、`status()` 或 `spawn()` 才真的啟動。這一集的外部程式（`echo`、`sh`、`cat`）是 Linux 與 macOS 都有的指令，Windows 沒有，範例沒辦法在那裡照跑。

### 執行並收集輸出

```rust,noplayground
use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .arg("紅茶")
        .arg("30")
        .output()
        .expect("無法執行 echo");
    println!("{}", output.status.success());
    println!("{:?}", String::from_utf8(output.stdout));
    println!("{}", output.stderr.len());
}
```

執行結果：

```text
true
Ok("紅茶 30\n")
0
```

- `Command::new("echo")` 是程式名稱，`arg` 加一個引數；一次加多個用 `args(["a", "b"])`。
- `output()` 等程式跑完，回傳 `Result<Output, io::Error>`。`Output` 有三個欄位：`status`（結束狀態）、`stdout`、`stderr`，後兩個都是 `Vec<u8>`，所以文字要用第 1 集的 `String::from_utf8` 轉。
- 引數逐一直接交給程式，**不經過 shell**：`arg("*")` 就是字元 `*`，不會展開成檔名；`arg("a | b")` 也只是含空白與直線的引數。

### 啟動失敗與跑完但失敗

兩種失敗在 Rust 裡落在不同的地方：

```rust,noplayground
use std::io::ErrorKind;
use std::process::Command;

fn main() {
    let output = Command::new("sh")
        .args(["-c", "echo 沒有紅茶了 >&2; exit 3"])
        .output()
        .expect("無法執行 sh");
    println!("{}", output.status.success());
    println!("{:?}", output.status.code());
    println!("{:?}", String::from_utf8(output.stderr));

    match Command::new("no-such-program").output() {
        Ok(_) => println!("執行了"),
        Err(e) if e.kind() == ErrorKind::NotFound => println!("找不到程式：{:?}", e.kind()),
        Err(e) => println!("其他錯誤：{e}"),
    }
}
```

執行結果：

```text
false
Some(3)
Ok("沒有紅茶了\n")
找不到程式：NotFound
```

- `sh -c "…"` 跑一小段 shell 指令。它印了一行到 stderr、以結束碼 3 結束，但 `output()` 仍然回傳 `Ok`：程式有跑起來，只是它自己覺得失敗。要自己看 `status.success()`，或用 `status.code()` 取結束碼。
- `code()` 是 `Option<i32>`：被訊號殺掉（例如 Ctrl-C）的行程沒有結束碼，就是 `None`。
- 找不到程式，連啟動都沒辦法，才是 `Err`，`kind()` 是 `NotFound`（跟第 3 集檔案的錯誤同一個型別）。

### 餵 stdin、讀 stdout

要跟程式來回溝通，用 `spawn()` 啟動，並用 `Stdio::piped()` 把它的標準輸入輸出接成管線：

```rust,noplayground
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("無法執行 cat");
    let mut stdin = child.stdin.take().expect("沒有 stdin");
    stdin
        .write_all("紅茶\n咖啡\n".as_bytes())
        .expect("寫入失敗");
    drop(stdin);
    let output = child.wait_with_output().expect("等待失敗");
    println!("{:?}", String::from_utf8(output.stdout));
    println!("{}", output.status.success());
}
```

執行結果：

```text
Ok("紅茶\n咖啡\n")
true
```

- `cat` 把收到的東西原樣印出來。`child.stdin` 是 `Option`，`take()` 把它拿出來，才能寫（第 1 集的 `write_all`）。
- **`drop(stdin)` 不能漏**：`cat` 要等到輸入關閉才知道沒有更多資料，而管線的寫入端要 drop 才會關。忘了的話，`wait_with_output` 會永遠等下去（實測：程式卡住，沒有任何訊息）。
- `wait_with_output()` 等行程結束，連同輸出一起收回來。

### 設定環境變數

第 5 集說 `set_var` 是 `unsafe`，要給外部程式設定，用 `env`，只影響那個子行程，不碰自己的環境，所以是安全的：

```rust,noplayground
use std::process::Command;

fn main() {
    let output = Command::new("sh")
        .args(["-c", "echo 模式：$TEASHOP_MODE"])
        .env("TEASHOP_MODE", "dev")
        .output()
        .expect("無法執行 sh");
    print!("{}", String::from_utf8(output.stdout).expect("不是 UTF-8"));
}
```

執行結果：

```text
模式：dev
```

類似的還有 `current_dir`（設定工作目錄）與 `env_remove`。至於結束**自己**，`std::process::exit(3)` 會立刻以結束碼 3 結束整個程式，**不會執行 drop**（實測：區域變數的 `Drop` 沒跑）；比較體面的結束碼寫法是第 18 章第 4 集的 `ExitCode`。

### 跟 Go 比一比

```go
out, err := exec.Command("sh", "-c", "echo 紅茶; exit 3").Output()
// out 是 "紅茶\n"，err 是 exit status 3
```

- Go 的 `Output()` 在結束碼不是 0 時回傳 `err`（`*exec.ExitError`），Rust 的 `output()` 只要程式跑起來了就是 `Ok`，失敗與否要自己檢查 `status`。只寫 `.expect()` 的 Rust 程式，不會發現被呼叫的程式其實失敗了。
- Go 與 Rust 都不經過 shell，要用 `*`、`|` 就得自己啟動 `sh -c`；這也是 `sh -c` 加上使用者輸入會有命令注入風險的原因，使用者的字串請當 `arg` 傳，不要拼進 `sh -c` 的字串。

## 重點整理

- `Command::new(程式).arg(..).args([..])` 描述要跑的程式，`output()` 跑完並收回 `Output { status, stdout, stderr }`；`stdout`、`stderr` 是 `Vec<u8>`。
- 引數不經過 shell，`*`、`|` 只是字元。
- 啟動失敗是 `Err(io::Error)`（找不到是 `NotFound`）；跑完但失敗是 `Ok`，要看 `status.success()` 或 `status.code()`（被訊號殺掉是 `None`）。
- `spawn()` 加 `Stdio::piped()` 可以餵 stdin、讀 stdout；寫完要 `drop` 掉 `child.stdin`，否則對方永遠等不到結束；`wait_with_output()` 等結束並收輸出。
- `env`、`current_dir` 只影響子行程；`std::process::exit(n)` 立刻結束自己且不執行 drop。
