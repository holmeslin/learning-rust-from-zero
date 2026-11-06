# 跨 `.await` 的 `Send` 限制

## 本集目標

知道 `tokio::spawn` 要求任務是 `Send`，而判斷標準是「跨過 `.await` 還活著的值」；看得懂「future cannot be sent between threads safely」，並用縮短作用域修好。

## 正文

### spawn 要求任務是 `Send`

`tokio::spawn` 的簽名裡有這一段（`'static` 在第 5 集說過）：

```text
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
```

多執行緒的 runtime 讓任務在 `.await` 停住之後，換到另一條工作執行緒繼續。所以任務停住的時候，它留下來的所有東西，都得能安全搬去別的執行緒，也就是 `Send`（第 14 章第 11 集）。

### 跨過 `.await` 的 `Rc`

`Rc` 不是 `Send`（第 14 章第 8 集）。把它放進任務，又讓它活過 `.await`：

```rust,compile_fail
use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        let order = Rc::new(String::from("紅茶"));
        sleep(Duration::from_millis(100)).await;
        println!("{order}");
    });
    handle.await.expect("店員出事了");
}
```

編譯錯誤：

```text
error: future cannot be sent between threads safely
   --> src/main.rs:7:18
    |
  7 |       let handle = tokio::spawn(async {
    |  __________________^
  8 | |         let order = Rc::new(String::from("紅茶"));
  9 | |         sleep(Duration::from_millis(100)).await;
 10 | |         println!("{order}");
 11 | |     });
    | |______^ future created by async block is not `Send`
    |
    = help: within `{async block@src/main.rs:7:31: 7:36}`, the trait `Send` is not implemented for `Rc<String>`
note: future is not `Send` as this value is used across an await
   --> src/main.rs:9:43
    |
  8 |         let order = Rc::new(String::from("紅茶"));
    |             ----- has type `Rc<String>` which is not `Send`
  9 |         sleep(Duration::from_millis(100)).await;
    |                                           ^^^^^ await occurs here, with `order` maybe used later
```

只貼到第二則 `note`，後面還有一則 `note: required by a bound in tokio::spawn`，指向 tokio 內部定義的位置，說明 `spawn` 要求 `Send`。這個錯誤沒有錯誤碼，所以上面的區塊標的是不帶碼的 `compile_fail`。讀訊息的順序是：

1. 第一行 `future cannot be sent between threads safely`：任務這個 `Future` 不是 `Send`。
2. `help` 說哪個型別害的：`Rc<String>` 沒有實作 `Send`。
3. `future is not Send as this value is used across an await`：指出兇手 `order`，以及它「跨過」的 `.await` 在哪一行。

重點是「跨過」：只有在 `.await` 的前後都還活著的值，才會被存進任務裡，才需要是 `Send`。

### 修法：讓它在 `.await` 之前就結束

不需要跨過 `.await` 的話，就在 `.await` 之前讓它消失：用區塊縮短它的作用域（第 3 章）：

```rust
use std::rc::Rc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        {
            let order = Rc::new(String::from("紅茶"));
            println!("{order}");
        }
        sleep(Duration::from_millis(100)).await;
        println!("好了");
    });
    handle.await.expect("店員出事了");
}
```

執行結果：

```text
紅茶
好了
```

`order` 在內層區塊結束就 drop 了，`.await` 的時候已經不存在，任務裡留下的只有 `Send` 的東西，編譯器就放行。真的要跨過 `.await`，就換成 `Send` 的版本：`Rc` 換成 `Arc`。

### 標準函式庫的 `MutexGuard` 也一樣

第 11 集說過，標準函式庫的 `MutexGuard` 不是 `Send`。拿著它跨過 `.await`，`spawn` 會出一模一樣的錯誤，連 `note` 的形狀都相同，只是 `help` 那一行變成 `the trait Send is not implemented for std::sync::MutexGuard<'_, u32>`，`note` 裡的 `has type` 指向 `t.lock()` 得到的 guard。修法有兩個：

- 把鎖用在一個區塊裡，算出結果、放掉鎖，再 `.await`，跟上面縮短 `Rc` 的作用域是同一招。
- 真的需要拿著鎖 `.await`，就換成 tokio 的 `Mutex`（第 11 集），它的 guard 是 `Send`。

### 沒有 `spawn`，就沒有這個限制

這條限制來自 `tokio::spawn`。`#[tokio::main]` 底下 `main` 的本體是用 `block_on` 在目前這條執行緒上跑的（第 4 集），不需要搬到別的執行緒，所以直接寫在 `main` 裡、跨過 `.await` 的 `Rc` 是可以編譯的。一旦把這段程式放進 `tokio::spawn`，限制就出現。

任務裡真的有不能跨執行緒的資料，又不想改，tokio 另有 `LocalSet` 與 `spawn_local` 可以把任務固定在同一條執行緒，這本書不展開。

## 重點整理

- `tokio::spawn` 要求 `F: Future + Send + 'static`：任務可能在 `.await` 之後換到別的工作執行緒。
- 判斷標準是「跨過 `.await` 還活著的值」是否都是 `Send`；`Rc`、標準函式庫的 `MutexGuard` 都不是。
- 錯誤是 `future cannot be sent between threads safely`（沒有錯誤碼）；看 `note: future is not Send as this value is used across an await`，找出是哪個值、跨過哪個 `.await`。
- 修法：用區塊縮短作用域，讓它在 `.await` 之前 drop；或換成 `Send` 的版本（`Arc`、tokio 的 `Mutex`）。
- 直接寫在 `main` 裡的程式沒有這個限制，限制來自 `tokio::spawn`。
