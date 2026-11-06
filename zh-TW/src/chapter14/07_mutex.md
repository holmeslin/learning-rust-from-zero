# `Mutex`

## 本集目標

用 `Mutex<T>` 讓多條執行緒輪流修改同一份資料：會 `lock()`、知道鎖是靠 guard 離開作用域才放開，也知道 `lock()` 為什麼回傳 `Result`。

## 正文

### 資料放在鎖裡面

`Mutex<T>`（mutual exclusion，互斥鎖）把資料 `T` 包在裡面。想碰資料，只有一條路：呼叫 `lock()` 拿鎖，拿到的是 `MutexGuard`，用起來像 `&mut T`（它實作了 `Deref` 與 `DerefMut`，第 13 章第 3 集）。同一時間只有一個執行緒拿得到鎖，其他人的 `lock()` 會等到鎖放開。

鎖什麼時候放開？guard 被 drop 的時候，就是第 13 章第 4 集的 `Drop`：

```rust
use std::sync::Mutex;

fn main() {
    let stock = Mutex::new(10);
    {
        let mut cups = stock.lock().expect("庫存鎖壞了");
        *cups -= 3;
        println!("拿著鎖，剩 {cups} 杯");
    }
    println!("鎖放開了，剩 {} 杯", stock.lock().expect("庫存鎖壞了"));
}
```

執行結果：

```text
拿著鎖，剩 7 杯
鎖放開了，剩 7 杯
```

- `stock` 沒有 `let mut`：`Mutex` 是內部可變性（第 13 章第 6 集），只拿 `&` 就能用鎖改資料。
- `lock()` 回傳 `Result`，先照固定句型 `.expect(...)`，下面說明為什麼是 `Result`。
- `cups` 是 guard，用 `*cups -= 3` 改裡面的值。內層的 `}` 讓 guard drop，鎖就放開，後面的 `lock()` 才拿得到；沒有任何「解鎖」的呼叫要記得寫。
- 如果 guard 一直不離開作用域，鎖就一直被占著，別人只能乾等。guard 的範圍越小越好。

### 多條執行緒輪流改

用上一章的範圍執行緒，讓四位店員各把一筆收入記進同一台收銀機。借用 `till` 不需要 `Arc`，因為 `scope` 保證執行緒比 `till` 先結束：

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let till = Mutex::new(0);
    let till_ref = &till;
    thread::scope(|s| {
        for price in [30, 45, 38, 55] {
            s.spawn(move || {
                *till_ref.lock().expect("收銀機壞了") += price;
            });
        }
    });
    println!("今天賣了 {} 元", till.lock().expect("收銀機壞了"));
}
```

執行結果：

```text
今天賣了 168 元
```

- 每條執行緒都先 `lock()` 再加，加完那一行結束，暫時的 guard 就 drop 了。加法跟順序無關，所以不管誰先誰後，總數固定是 168。
- 這裡有 `move`：迴圈變數 `price` 每圈都是新的，活不到 `scope` 結束，不 `move` 的話是 E0373（第 4 集預告過的）。但 `till` 本身不能 `move` 進去，第一個執行緒就把它搬走了。所以先 `let till_ref = &till;`，`move` 搬進去的是參照（參照是 `Copy`，每條執行緒各拿一份）。
- 編譯器不會讓你忘記 `lock()`：`till` 是 `Mutex<i32>`，不是 `i32`，不拿鎖就碰不到裡面的整數。

### 鎖中毒：`lock()` 為什麼回傳 `Result`

如果一條執行緒拿著鎖的時候 panic，資料可能改到一半，其他執行緒不該當作沒事。這時 `Mutex` 會標記成中毒（poisoned），之後的 `lock()` 回傳 `Err`：

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let till = Mutex::new(0);
    thread::scope(|s| {
        let barista = s.spawn(|| {
            let _guard = till.lock().expect("收銀機壞了");
            panic!("店員拿著鎖昏倒了");
        });
        if barista.join().is_err() {
            println!("店員昏倒了");
        }
    });
    println!("鎖中毒了嗎？{}", till.is_poisoned());
    match till.lock() {
        Ok(_) => println!("鎖正常"),
        Err(poisoned) => println!("裡面的值還在：{}", *poisoned.into_inner()),
    }
}
```

執行結果：

```text
店員昏倒了
鎖中毒了嗎？true
裡面的值還在：0
```

- 子執行緒的 panic 訊息照樣印在 stderr（第 2 集），這裡只貼 stdout。
- `.expect(...)` 的意思就是「別的執行緒在鎖裡昏倒了，那就連我一起 panic」，多數程式這樣做就夠了。想繼續用，`Err` 裡的 `into_inner()` 可以拿回 guard，自己決定資料還能不能信。

### 跟 Go 比一比

Go 的 `sync.Mutex` 跟它保護的資料是兩個獨立的欄位，兩者的關係只存在於註解與紀律：

```go
type Till struct {
	mu    sync.Mutex
	total int
}

func (t *Till) Add(price int) {
	t.mu.Lock()
	defer t.mu.Unlock()
	t.total += price
}
```

在別處直接寫 `t.total += price`，一樣編譯得過，漏掉的 `Lock` 要靠 `-race` 在執行時抓。Rust 的資料就放在 `Mutex` 裡面，不 `lock()` 根本拿不到；解鎖也不用 `defer`，guard 離開作用域就自動放開。

## 重點整理

- `Mutex::new(值)` 把資料包起來；`lock()` 回傳 `Result<MutexGuard<T>, _>`，guard 當 `&mut T` 用。
- 鎖在 guard 被 drop 時放開，沒有 `unlock()`；guard 活多久，鎖就占多久，範圍盡量小。
- `Mutex` 只需要 `&`（內部可變性），所以可以搭配範圍執行緒共用；迴圈變數要 `move`，共用的 `Mutex` 先取一個參照再 `move`。
- 持鎖的執行緒 panic，鎖中毒，之後的 `lock()` 回傳 `Err`；`.expect(...)` 讓整個程式一起停，`into_inner()` 可以硬拿回資料。
- 跟 Go 比：Go 的鎖與資料各管各的，漏鎖要等 `-race`；Rust 的資料鎖在 `Mutex` 裡，不拿鎖碰不到。
