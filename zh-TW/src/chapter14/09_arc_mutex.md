# `Arc<Mutex<T>>`

## 本集目標

把 `Arc` 與 `Mutex` 組起來，讓好幾條 `thread::spawn` 開出來的執行緒共用並修改同一份資料，並知道鎖要拿得越短越好。

## 正文

### 一個管共用，一個管修改

前兩集各解決一半的問題：

- `Arc<T>` 讓資料有好幾個擁有者，可以交給不同的執行緒，但只能讀。
- `Mutex<T>` 讓你只靠 `&` 就能修改資料，但一份資料只有一個擁有者，不能交給好幾條 `spawn` 出來的執行緒。

疊起來就是 `Arc<Mutex<T>>`：`Arc` 負責共用所有權，`Mutex` 負責輪流修改。這就是第 13 章第 7 集說的「多執行緒要換成 `Arc<Mutex<T>>`」，它是 `Rc<RefCell<T>>` 的多執行緒版：

- `Rc` 換成 `Arc`：計數改成原子操作，才能跨執行緒。
- `RefCell` 換成 `Mutex`：借用規則一樣是「多個讀或一個寫」，違反時 `RefCell` 立刻 panic，`Mutex` 是讓後來的執行緒等到鎖放開。

### 四位店員共用一台收銀機

收銀機 `Till` 記著營業額與筆數，兩個欄位放在同一把鎖裡，這樣一次 `lock()` 就能把它們一起改對：

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Till {
    total: u32,
    orders: u32,
}

fn main() {
    let till = Arc::new(Mutex::new(Till {
        total: 0,
        orders: 0,
    }));
    let mut handles = Vec::new();
    for price in [30, 45, 38, 55] {
        let till = Arc::clone(&till);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let mut till = till.lock().expect("收銀機壞了");
            till.total += price;
            till.orders += 1;
        }));
    }
    for handle in handles {
        handle.join().expect("店員出事了");
    }
    let till = till.lock().expect("收銀機壞了");
    println!("今天賣了 {} 元，共 {} 筆", till.total, till.orders);
}
```

執行結果：

```text
今天賣了 168 元，共 4 筆
```

- 每一圈 `let till = Arc::clone(&till);` 取一個複本，`move` 進那條執行緒。這個「在迴圈裡用同名變數遮蔽」的寫法，是 `Arc` 搭配 `spawn` 的慣用樣子。
- 執行緒裡的 `let mut till = till.lock()...` 又遮蔽了一次，這時 `till` 是 guard，用 `till.total += price` 改欄位，跟用 `&mut Till` 一樣。
- 全部 `join` 完，`main` 自己的 `till` 再 `lock()` 一次，讀出結果。加法跟順序無關，所以結果固定。

### 鎖拿得越短越好

`sleep` 是在 `lock()` 之前，代表「做飲料」不需要鎖，四位店員同時做，各自做完才排隊記帳，只在 `total` 與 `orders` 要改的那一刻拿鎖。如果把 `sleep` 搬到 `lock()` 後面，guard 要等到閉包結束才 drop，四位店員的 50 毫秒就變成輪流等：

```text
拿鎖之前做飲料：   [店員 1 做]
                   [店員 2 做]      同時進行，記帳只占一瞬間
                   [店員 3 做]
拿鎖之後才做：     [店員 1 做][店員 2 做][店員 3 做]   全部排隊
```

原則是：只在真的要碰共用資料的那幾行拿鎖，需要慢慢算的東西先在鎖外面算好。

另外要小心，兩次分開的 `lock()` 之間，別人可以插隊。「先讀庫存、判斷夠不夠、再扣庫存」如果寫成兩次 `lock()`，判斷之後、扣庫存之前，別的執行緒可能已經把庫存扣完了。這類問題要靠一次 `lock()` 把整套判斷和修改做完，第 15 集會再談。

## 重點整理

- `Arc<Mutex<T>>`：`Arc` 負責讓多條執行緒共有，`Mutex` 負責輪流修改；是多執行緒版的 `Rc<RefCell<T>>`。
- 慣用寫法：迴圈裡 `let x = Arc::clone(&x);`，再 `move` 進 `thread::spawn`；全部 `join` 之後，從 `main` 的那個 `Arc` 讀結果。
- 相關的欄位放進同一把 `Mutex`，一次 `lock()` 把它們一起改對。
- 鎖只在要碰共用資料的那幾行拿著，慢的工作放在拿鎖之前，不然執行緒會變成排隊。
- 判斷與修改要在同一次 `lock()` 裡完成，兩次 `lock()` 之間別人會插隊。
