# 處理 Ctrl-C

## 本集目標

知道按下 Ctrl-C 時程式會發生什麼事，會用 `tokio::signal::ctrl_c()` 加上 `CancellationToken` 讓程式做完手上的事再收工，並以結束碼 130 結束。

## 正文

### Ctrl-C 是一個訊號

在終端機按 Ctrl-C，終端機會送一個叫 SIGINT 的訊號給正在執行的程式。程式沒有特別處理的話，作業系統預設的動作是**立刻結束它**：不會跑任何 drop、緩衝區裡的資料不會寫出、寫到一半的檔案就留在那裡。批次處理訂單、寫報表這類工具，應該收到 Ctrl-C 時做完手上這一筆、印出進度再結束。

標準函式庫沒有處理訊號的功能。這一章已經有 tokio（第 15 章），它的 `tokio::signal::ctrl_c()` 是一個 async 函式，收到 Ctrl-C（Linux、macOS 的 SIGINT，Windows 的 Ctrl-C 事件）就完成。社群的 `ctrlc` crate 做同樣的事，不用 async，這本書不展開。

### 做完這筆再收工

下面是批次處理訂單的骨架：逐筆處理，每筆花 0.4 秒。啟動一個任務等 Ctrl-C，收到就把 `CancellationToken`（第 15 章第 10 集）設成已取消；主迴圈每次開始一筆之前先看一下，被取消就停。**手上正在做的那一筆會做完**：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "1.54.1", features = ["full"] }
tokio-util = "0.7.20"
```

`src/main.rs`：

```rust,ignore
use std::process::{Command, ExitCode};
use std::time::Duration;

use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

/// 示範用：書上沒辦法替你按 Ctrl-C，所以讓程式送一個 SIGINT 給自己，效果跟按 Ctrl-C 一樣。
fn press_ctrl_c() {
    Command::new("kill")
        .args(["-INT", &std::process::id().to_string()])
        .status()
        .expect("無法執行 kill");
}

#[tokio::main]
async fn main() -> ExitCode {
    let token = CancellationToken::new();
    let watcher = token.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("無法監聽 Ctrl-C");
        println!("收到 Ctrl-C，做完手上這筆就收工");
        watcher.cancel();
    });

    let orders = ["紅茶", "咖啡", "奶茶", "紅茶", "咖啡"];
    let mut done = 0;
    for (i, drink) in orders.iter().enumerate() {
        if token.is_cancelled() {
            break;
        }
        println!("處理第 {} 筆：{drink}", i + 1);
        if i == 1 {
            press_ctrl_c();
        }
        sleep(Duration::from_millis(400)).await;
        done += 1;
    }
    println!("已處理 {done} 筆，還有 {} 筆沒處理", orders.len() - done);
    if token.is_cancelled() {
        ExitCode::from(130)
    } else {
        ExitCode::SUCCESS
    }
}
```

`cargo run` 的輸出（省略編譯進度，結束碼 130）：

```text
     Running `target/debug/teashop`
處理第 1 筆：紅茶
處理第 2 筆：咖啡
收到 Ctrl-C，做完手上這筆就收工
已處理 2 筆，還有 3 筆沒處理
```

重點：

- `tokio::signal::ctrl_c().await` 放在自己的任務裡。它一完成就 `cancel()`，主迴圈靠 `is_cancelled()` 發現。
- `std::process::id()` 是自己的行程編號，`kill -INT 編號` 就是從命令列送 SIGINT。`kill` 是 Linux、macOS 的指令（跟第 17 章第 7 集的 `echo` 一樣，Windows 沒有），所以這個專案只在 Linux、macOS 照跑。
- 第 2 筆開始處理之後，`press_ctrl_c()` 才送出訊號，所以第 2 筆會做完，第 3 筆沒有開始。真實的專案把 `press_ctrl_c` 整段拿掉，使用者自己按。
- 結束碼用 130：shell 把「被 SIGINT 結束」記成 128 加 2。自己收完尾再結束的程式，也回報 130，呼叫它的腳本才知道「是使用者中斷的」，不是成功，也不是一般失敗。
- 如果要的是「立刻停」，不必做完這筆，改用第 15 章第 8 集的 `select!`，讓 `ctrl_c()` 跟手上的工作比賽，先完成的贏，輸的那個 `Future` 被丟掉。

### 安裝了處理程式之後

第一次等待 `ctrl_c()` 的時候，程式就為 SIGINT 裝上處理程式，**取代**作業系統的預設動作，而且在程式結束之前都不會還原（實測：送兩次 SIGINT，程式照常跑完）。也就是說，收到第一次 Ctrl-C 之後，如果主程式卡住了，使用者再按一次也殺不掉它。要有「再按一次就強制結束」的行為，得自己再等一次 `ctrl_c()`，收到就呼叫 `std::process::exit(130)`。

### 跟 Go 比一比

```go
ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
defer stop()
for i, drink := range orders {
	if ctx.Err() != nil {
		break
	}
	fmt.Println("處理第", i+1, "筆：", drink)
	time.Sleep(400 * time.Millisecond)
}
```

- `signal.NotifyContext` 讓 SIGINT 把 `context` 取消，用法跟 `CancellationToken` 一一對應：`ctx.Err() != nil` 是 `is_cancelled()`，`ctx.Done()` 是 `cancelled().await`。
- Go 呼叫 `stop()` 之後，SIGINT 恢復預設動作，再按一次就會殺掉程式；tokio 的處理程式一旦裝上就不會拿掉，要強制結束得自己寫。
- 沒有處理程式時，兩邊都是被直接殺掉：Go 的 `defer`、Rust 的 drop 都不會執行。

## 重點整理

- Ctrl-C 送出 SIGINT，預設動作是立刻結束程式，不執行 drop、不寫出緩衝區。
- 標準函式庫沒有訊號處理；用 tokio 的 `tokio::signal::ctrl_c().await`（收到就完成），社群也有 `ctrlc` crate（本書不展開）。
- 慣用寫法：在任務裡等 `ctrl_c()`，收到就 `cancel()` 一個 `CancellationToken`，工作迴圈每筆之前檢查 `is_cancelled()`，讓手上的事做完再收工；要立刻停則用 `select!`。
- 因 Ctrl-C 結束，結束碼慣例是 130（128 加訊號編號 2）。
- `ctrl_c()` 裝上的處理程式在程式結束前不會還原，要「再按一次強制結束」得自己再等一次；Go 的 `NotifyContext` 在 `stop()` 之後會還原預設動作。
