# `timeout`

## 本集目標

用 `tokio::time::timeout` 替任何 `Future` 設期限，看懂回傳的 `Result<T, Elapsed>`；知道逾時其實就是取消，而且只在 `.await` 的地方才斷得掉。

## 正文

### 設一個期限

客人最多願意等 300 毫秒。`timeout(期限, future)` 把一個 `Future` 包起來，`.await` 的結果是 `Result<T, Elapsed>`：期限內做完是 `Ok(結果)`，超過期限是 `Err(Elapsed)`：

```rust
use std::time::Duration;
use tokio::time::{sleep, timeout};

struct Cup(&'static str);

impl Drop for Cup {
    fn drop(&mut self) {
        println!("{}的杯子收走了", self.0);
    }
}

async fn brew(drink: &'static str, wait: u64) -> &'static str {
    let _cup = Cup(drink);
    sleep(Duration::from_millis(wait)).await;
    drink
}

#[tokio::main]
async fn main() {
    for (drink, wait) in [("咖啡", 100), ("紅茶", 500)] {
        match timeout(Duration::from_millis(300), brew(drink, wait)).await {
            Ok(cup) => println!("{cup}準時送到"),
            Err(e) => println!("{drink}：{e}"),
        }
    }
}
```

執行結果：

```text
咖啡的杯子收走了
咖啡準時送到
紅茶的杯子收走了
紅茶：deadline has elapsed
```

- 咖啡 100 毫秒就好，是 `Ok`。
- 紅茶要 500 毫秒，300 毫秒一到就是 `Err(Elapsed)`。`Elapsed` 印出來是英文的 `deadline has elapsed`，用 `{:?}` 印是 `Elapsed(())`。
- 逾時之後，包在裡面的 `Future` 被丟掉，就是取消：紅茶的 `brew` 沒有做完，但它的區域變數 `_cup` 照常 drop，所以看得到「紅茶的杯子收走了」。這跟第 8 集 `select!` 丟掉落選分支是同一件事，`timeout` 可以想成「`Future` 跟一個計時器賽跑」。

### 內層自己也有 `Result`

被包的 `Future` 本身常常就回傳 `Result`（查資料庫、發請求），加上 `timeout` 的 `Result`，就有兩層。最外層是「有沒有逾時」，裡面才是工作自己的結果：

```rust
use std::time::Duration;
use tokio::time::timeout;

async fn find_price(drink: &str) -> Result<u32, String> {
    match drink {
        "紅茶" => Ok(38),
        _ => Err(format!("沒有{drink}")),
    }
}

#[tokio::main]
async fn main() {
    for drink in ["紅茶", "橘子汁"] {
        match timeout(Duration::from_millis(100), find_price(drink)).await {
            Ok(Ok(price)) => println!("{drink}：{price} 元"),
            Ok(Err(msg)) => println!("查詢失敗：{msg}"),
            Err(_) => println!("查詢逾時"),
        }
    }
}
```

執行結果：

```text
紅茶：38 元
查詢失敗：沒有橘子汁
```

### 只在 `.await` 的地方斷得掉

取消是靠「丟掉 `Future`」，而丟掉的機會只出現在 `Future` 停在 `.await`、把控制權還給 runtime 的時候。如果裡面一直霸佔著執行緒，期限就形同虛設：

```rust
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let result = timeout(Duration::from_millis(100), async {
        std::thread::sleep(Duration::from_millis(300));
        38
    })
    .await;
    println!("{result:?}");
}
```

執行結果：

```text
Ok(38)
```

期限是 100 毫秒，但結果是 `Ok(38)`：`std::thread::sleep` 把整條執行緒睡死 300 毫秒，這段時間 runtime 根本沒有機會處理計時器。在 async 裡呼叫會卡住執行緒的函式，就是第 12 集的主題；改用 `tokio::time::sleep`，就會照期限逾時。

### 跟 Go 比一比

Go 用 `context.WithTimeout` 設期限，但 Go 不會替你停下函式，函式自己要去看 `ctx.Done()`：

```go
func brew(ctx context.Context, drink string, wait time.Duration) (string, error) {
	select {
	case <-time.After(wait):
		return drink, nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

ctx, cancel := context.WithTimeout(context.Background(), 300*time.Millisecond)
defer cancel()
cup, err := brew(ctx, "紅茶", 500*time.Millisecond) // err 是 context deadline exceeded
```

沒有把 `ctx` 傳進去、或是函式裡沒有檢查 `ctx.Done()`，期限就完全沒用。Rust 的 `timeout` 包的是 `Future` 本身，被包的 async 函式不用知道有期限，也不用傳任何東西，到期它就被丟掉。代價是前面看到的：只有停在 `.await` 的地方才斷得掉。

## 重點整理

- `timeout(期限, future).await` 回傳 `Result<T, Elapsed>`：期限內做完是 `Ok`，逾時是 `Err(Elapsed)`（`deadline has elapsed`）。
- 逾時等於丟掉內層 `Future`，也就是取消，區域變數照常 drop；內層本身回傳 `Result` 時會變成兩層 `Ok(Ok(..))`、`Ok(Err(..))`、`Err(_)`。
- 取消只發生在 `.await` 的地方：在 async 裡用 `std::thread::sleep` 這類卡住執行緒的函式，期限不會生效。
- 跟 Go 比：Go 的 `context` 要函式自己配合檢查；`timeout` 包住任何 `Future`，不需要配合。
