# `CancellationToken`

## 本集目標

用 `CancellationToken` 通知一個或一群任務「該收工了」，讓任務自己決定怎麼收尾；也跟 Go 的 `context` 比一比。

## 正文

### 一個「該收工了」的訊號

第 8、9 集的 `select!` 與 `timeout` 是「丟掉 `Future`」：任務做到一半被收走，沒機會說一聲再走。有時候希望任務自己收尾，例如打烊時把手上那杯做完、把帳結清。這時用 `CancellationToken`，它在 crate tokio-util 裡：

```bash
cargo add tokio-util
```

```toml
[dependencies]
tokio = { version = "1.54.1", features = ["full"] }
tokio-util = "0.7.20"
```

用法是：把 token 交給任務，任務用 `select!` 同時等「工作」跟「收工訊號」：

```rust
use std::time::Duration;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

async fn grind_beans(token: CancellationToken) -> u32 {
    let mut cups = 0;
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("收到打烊訊號，收工");
                return cups;
            }
            _ = sleep(Duration::from_millis(200)) => cups += 1,
        }
    }
}

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let worker = tokio::spawn(grind_beans(token.clone()));
    sleep(Duration::from_millis(500)).await;
    token.cancel();
    let cups = worker.await.expect("店員出事了");
    println!("今天磨了 {cups} 杯");
}
```

執行結果：

```text
收到打烊訊號，收工
今天磨了 2 杯
```

- `CancellationToken::new()` 建立一個訊號；`token.clone()` 得到共用同一個訊號的另一份，交給任務用。
- `token.cancel()` 發出訊號。`token.cancelled()` 是一個 `Future`，訊號發出之後就完成，所以能放進 `select!` 當分支。
- 任務收到訊號之後，自己印訊息、回傳磨好的杯數，想做什麼收尾都行。主程式 `worker.await` 拿到的就是這個回傳值。
- 訊號發出之後，`token.is_cancelled()` 回傳 `true`；已經取消的 token 再 `cancel()` 沒有影響，`cancelled().await` 會立刻完成。
- 只想包一個 `Future` 的話，`token.run_until_cancelled(future).await` 回傳 `Option<T>`：做完是 `Some(結果)`，先被取消是 `None`。

### 一組任務共用訊號：`child_token`

`token.child_token()` 做出一個子訊號。取消父的，所有子的跟著取消；只取消子的，父的跟其他子的不受影響：

```rust
use tokio_util::sync::CancellationToken;

fn states(tokens: &[&CancellationToken]) -> Vec<bool> {
    tokens.iter().map(|t| t.is_cancelled()).collect()
}

#[tokio::main]
async fn main() {
    let shop = CancellationToken::new();
    let counter = shop.child_token();
    let kitchen = shop.child_token();
    counter.cancel();
    println!("{:?}", states(&[&shop, &counter, &kitchen]));
    shop.cancel();
    println!("{:?}", states(&[&shop, &counter, &kitchen]));
}
```

執行結果：

```text
[false, true, false]
[true, true, true]
```

`states` 依序回報「店、櫃台、廚房」有沒有被取消。整間店（`shop`）是父，櫃台與廚房各是子：櫃台先收，只有櫃台是 `true`；整間店打烊，三個都是 `true`。把 token 分成一棵樹，就能「只停某一部分」或「全部一起停」。

### 兩種取消

- 丟掉 `Future`：`select!` 的落選分支、`timeout` 逾時、丟掉 `JoinSet`、對 `JoinHandle` 呼叫 `abort()`。任務在下一個 `.await` 的地方被收走，來不及做收尾（只有 `Drop` 會跑）。
- `CancellationToken`：任務自己看到訊號，收尾之後才結束。要在收工前把事情做完，用這個。

### 跟 Go 比一比

同樣的磨豆子，Go 用 `context` 寫：

```go
func grindBeans(ctx context.Context) int {
	cups := 0
	for {
		select {
		case <-ctx.Done():
			fmt.Println("收到打烊訊號，收工")
			return cups
		case <-time.After(200 * time.Millisecond):
			cups++
		}
	}
}

ctx, cancel := context.WithCancel(context.Background())
// ... 開 goroutine 跑 grindBeans(ctx)
cancel()
```

對照：

| Go | tokio-util |
| --- | --- |
| `context.WithCancel(parent)` | `CancellationToken::new()`、`parent.child_token()` |
| `cancel()` | `token.cancel()` |
| `<-ctx.Done()` | `token.cancelled().await` |
| `ctx.Err()` | 沒有對應，只有 `is_cancelled()` 回傳 `bool` |

Go 的直覺要修正的地方：

- `context` 身兼三職：取消、期限（`WithTimeout`）、帶值（`WithValue`）。`CancellationToken` 只管取消；期限用第 9 集的 `timeout`，要帶資料就用參數或 `Arc`，沒有 `WithValue` 這種東西。
- Go 慣例把 `ctx` 放在每個函式的第一個參數、一路傳下去；token 只是普通的值，要不要傳、傳給誰，由你決定，編譯器不會強迫。
- 父子關係一樣：Go 的 `WithCancel(parent)` 取消父的連帶取消子，反過來不會；`child_token` 也是。
- Go 只有「協作」這一種取消，goroutine 不看 `ctx.Done()` 就停不下來；Rust 還有前面「丟掉 `Future`」那一類，不需要任務配合。

## 重點整理

- `tokio_util::sync::CancellationToken`（crate tokio-util）：`new()`、`clone()`（共用同一個訊號）、`cancel()`、`cancelled().await`、`is_cancelled()`、`run_until_cancelled(future)`（回傳 `Option<T>`）。
- 任務用 `select!` 等 `token.cancelled()`，收到訊號後自己收尾再結束；已取消的 token 再 `cancel()` 無妨。
- `child_token()`：取消父的，子的全部取消；取消子的，父的不受影響。
- 兩種取消：丟掉 `Future`（`select!`、`timeout`、drop `JoinSet`、`abort()`）立刻收走，沒有收尾；token 讓任務自己收尾。
- 跟 Go 比：`context` 還管期限與值，token 只管取消；`ctx.Err()` 沒有對應；Go 只有協作式取消，Rust 還有丟掉 `Future`。
