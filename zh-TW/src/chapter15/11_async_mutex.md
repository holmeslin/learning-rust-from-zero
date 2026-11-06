# async 裡的 `Mutex`

## 本集目標

知道 async 程式裡有兩種 `Mutex`：標準函式庫的與 tokio 的；會用 `tokio::sync::Mutex` 的 `lock().await`，也分得出什麼時候用哪一種。

## 正文

### 鎖得很短，用標準函式庫的就夠了

第 14 章的 `Arc<Mutex<T>>` 在 async 裡一樣用得上：任務跟執行緒一樣，要共用資料就把 `Arc` clone 進去。拿鎖、改資料、放鎖都很快，而且拿著鎖的時候不 `.await`，標準函式庫的 `Mutex` 就行：

```rust
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let orders = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for id in 1..=3 {
        let orders = Arc::clone(&orders);
        handles.push(tokio::spawn(async move {
            sleep(Duration::from_millis(100 * id)).await;
            orders.lock().expect("櫃台壞了").push(id);
        }));
    }
    for handle in handles {
        handle.await.expect("店員出事了");
    }
    println!("{:?}", orders.lock().expect("櫃台壞了"));
}
```

執行結果：

```text
[1, 2, 3]
```

`orders.lock().expect(...).push(id)` 是一行搞定：guard 在這行結束就放鎖，中間沒有 `.await`。這種用法，標準函式庫的 `Mutex` 又簡單又快。

### 拿著鎖還要 `.await`：`tokio::sync::Mutex`

有些時候得拿著鎖等東西，例如整杯咖啡泡完之前，咖啡機都算「被佔用」。這時標準函式庫的 `Mutex` 有兩個問題：它的 guard 不是 `Send`，拿著它跨過 `.await`，放進 `tokio::spawn` 會編譯失敗（第 14 集會看到這個錯誤）；而且它的 `lock()` 會把整條執行緒卡住，不只是目前的任務。

tokio 提供自己的 `Mutex`，`lock()` 是 async：

```rust
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let machine = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for id in 1..=3 {
        let machine = Arc::clone(&machine);
        handles.push(tokio::spawn(async move {
            let mut served = machine.lock().await;
            println!("{id} 號開始用咖啡機");
            sleep(Duration::from_millis(100)).await;
            served.push(id);
            println!("{id} 號用完了");
        }));
    }
    for handle in handles {
        handle.await.expect("店員出事了");
    }
    println!("服務順序 {:?}", machine.lock().await);
}
```

執行結果：

```text
1 號開始用咖啡機
1 號用完了
2 號開始用咖啡機
2 號用完了
3 號開始用咖啡機
3 號用完了
服務順序 [1, 2, 3]
```

- `machine.lock().await` 回傳 guard。等不到鎖的任務停在 `.await`，執行緒讓給別人，不會卡住。
- 3 個任務都握著鎖睡 100 毫秒，所以只能逐一進行。tokio 的 `Mutex` 依排隊的先後把鎖交出去。這裡用 `current_thread`，任務依開出來的順序排隊，輸出才是固定的。
- tokio 的 guard 是 `Send`，可以放心跨過 `.await`。
- 沒有 `Result`：tokio 的 `Mutex` 沒有鎖中毒。任務拿著鎖 panic，鎖就放開，下一個 `lock().await` 照樣拿得到，跟第 14 章第 7 集的 `lock().expect(...)` 不同。
- `try_lock()` 不等，拿不到回傳 `Err`。
- 同一個任務對同一個 `Mutex` 連續 `lock().await` 兩次，第二次會永遠等下去（第一個 guard 還沒放）。

### 該用哪一種

- 拿著鎖的時間很短、期間沒有 `.await`：標準函式庫的 `Mutex`，簡單、快。不要讓它的 guard 活過 `.await`。
- 拿著鎖還要 `.await`：tokio 的 `Mutex`。
- 兩種的名字一樣，只差在 `use`：`use std::sync::Mutex;` 或 `use tokio::sync::Mutex;`，要看清楚。
- 也可以不共用：讓一個任務獨自擁有資料，其他任務用第 7 集的 channel 傳訊息請它處理，就不用鎖了。

### 跟 Go 比一比

Go 的 `sync.Mutex` 隨處都能用，一個 goroutine 卡在 `Lock()`，runtime 會把這條執行緒讓給別的 goroutine，所以拿著鎖睡覺不會連累不相干的 goroutine。

Rust 的 `std::sync::Mutex::lock()` 卡住的是作業系統的執行緒，這條執行緒上的任務全跟著停。所以 async 裡的鎖要分兩種：短暫鎖用標準函式庫的，拿著鎖等的用 tokio 的。Go 沒有這個選擇題。

## 重點整理

- 鎖很短、期間沒有 `.await`：`Arc<std::sync::Mutex<T>>` 就夠，guard 不要活過 `.await`。
- 拿著鎖還要 `.await`：`tokio::sync::Mutex`；`lock().await` 直接回傳 guard（沒有 `Result`、沒有鎖中毒），等鎖時只讓出任務，依排隊先後交鎖；guard 是 `Send`。
- `try_lock()` 不等；同一個任務連續 `lock().await` 兩次會永遠等下去。
- 另一個選項是讓一個任務擁有資料，其他任務用 channel 傳訊息。
- 跟 Go 比：Go 只有一種 `sync.Mutex`，卡住的只是 goroutine；Rust 的 std `Mutex` 會卡住整條執行緒，所以 async 裡有兩種可選。
