# 死結

## 本集目標

知道死結（deadlock）是怎麼發生的、為什麼 Rust 的編譯器擋不住它，並會用固定的上鎖順序與 `try_lock` 避開它。

## 正文

### 編譯器不管死結

前面幾集看到，Rust 能在編譯時期擋下 data race。但「兩條執行緒互相等對方放鎖」不是 data race，不會讀到壞資料，只是永遠等下去。這是邏輯錯誤，編譯器看不出來，所以 Rust 保證記憶體安全，不保證沒有死結。

最典型的情況是兩把鎖、相反的順序。店員要補杯子與咖啡豆，要同時拿到兩把鎖：

```rust,no_run
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn restock(first: &Mutex<u32>, second: &Mutex<u32>) {
    let mut a = first.lock().expect("鎖壞了");
    thread::sleep(Duration::from_millis(100));
    let mut b = second.lock().expect("鎖壞了");
    *a += 1;
    *b += 1;
}

fn main() {
    let cups = Mutex::new(10);
    let beans = Mutex::new(10);
    thread::scope(|s| {
        s.spawn(|| restock(&cups, &beans));
        s.spawn(|| restock(&beans, &cups));
    });
    println!(
        "杯子 {}，豆子 {}",
        cups.lock().expect("鎖壞了"),
        beans.lock().expect("鎖壞了")
    );
}
```

這支程式只編譯、不執行（`no_run`），因為它不會結束：

```text
店員 A：拿著 cups，等 beans（被 B 拿著）
店員 B：拿著 beans，等 cups（被 A 拿著）
```

兩條執行緒各拿一把鎖，`sleep` 讓對方有時間拿到另一把，然後雙方都在等對方放。程式不會有任何輸出、也不會報錯，就是卡住，只能 Ctrl-C。有 `sleep` 的時候，卡住是必然；拿掉 `sleep`，也只是「大部分時候」剛好錯開，偶爾卡住，更難查。

### 修法一：每個人都用同一個上鎖順序

只要所有執行緒都先拿 `cups`、再拿 `beans`，就不可能出現「互相等」：後來的人在第一把鎖就被擋住，輪到他的時候，另一把鎖早已放開。

```rust
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn restock(first: &Mutex<u32>, second: &Mutex<u32>) {
    let mut a = first.lock().expect("鎖壞了");
    thread::sleep(Duration::from_millis(100));
    let mut b = second.lock().expect("鎖壞了");
    *a += 1;
    *b += 1;
}

fn main() {
    let cups = Mutex::new(10);
    let beans = Mutex::new(10);
    thread::scope(|s| {
        s.spawn(|| restock(&cups, &beans));
        s.spawn(|| restock(&cups, &beans));
    });
    println!(
        "杯子 {}，豆子 {}",
        cups.lock().expect("鎖壞了"),
        beans.lock().expect("鎖壞了")
    );
}
```

執行結果：

```text
杯子 12，豆子 12
```

跟上面只差一行：第二條執行緒也寫 `restock(&cups, &beans)`。順序要靠約定，例如整個程式規定「`cups` 一定在 `beans` 前面」。

### 其他修法

- 一次只拿一把鎖：用完第一把（讓 guard drop）再拿第二把，就不會有「握著一把等另一把」。
- 不要在握著鎖的時候，去呼叫可能也要拿鎖的函式。同一條執行緒對同一個 `Mutex` 連續 `lock()` 兩次，是自己等自己：標準函式庫的說明是「可能 panic，也可能死結」，在某些平台上就是無聲卡死。
- 用 `try_lock()`：拿不到就立刻回傳 `Err`，不會等，可以改做別的事、稍後再試。

```rust
use std::sync::Mutex;

fn main() {
    let till = Mutex::new(0);
    let guard = till.lock().expect("收銀機壞了");
    match till.try_lock() {
        Ok(_) => println!("拿到鎖了"),
        Err(e) => println!("拿不到：{e:?}"),
    }
    drop(guard);
    println!("放開之後拿得到嗎？{}", till.try_lock().is_ok());
}
```

執行結果：

```text
拿不到："WouldBlock"
放開之後拿得到嗎？true
```

`Err` 的 `WouldBlock` 表示「現在有人拿著」。第 10 集 `RwLock` 的 `try_write()` 也是同樣的設計。

### 跟 Go 比一比

Go 的 `sync.Mutex` 也不允許重複上鎖，同樣的兩把鎖、相反順序一樣會卡死。差別在 Go 的 runtime 會幫你喊出來：

```go
go func() { restock(&cups, &beans) }()
go func() { restock(&beans, &cups) }()
wg.Wait()
```

這段程式（`main` 在 `wg.Wait()` 等兩個 goroutine）會印出 `fatal error: all goroutines are asleep - deadlock!`，接著列出每個 goroutine 卡在哪裡，然後以結束碼 2 結束。但這個偵測只在「所有 goroutine 都卡住」時才會觸發；程式裡只要還有別的 goroutine 在跑（例如一個 HTTP 伺服器），Go 一樣無聲卡著。Rust 開的是作業系統執行緒，標準函式庫沒有這種偵測，卡住就是卡住，要靠自己守紀律或看程式碼找出來。

## 重點整理

- 死結：兩條以上的執行緒各握一把鎖、又互相等對方的鎖，永遠等下去。它不是 data race，編譯器看不出來。
- 典型成因：兩把鎖相反的順序；握著鎖又去呼叫會拿同一把鎖的函式（同一條執行緒對同一個 `Mutex` 連續 `lock()` 兩次也一樣）。
- 修法：所有執行緒用同一個上鎖順序；一次只握一把鎖；`try_lock()` 拿不到就先做別的事。
- 死結的程式沒有輸出、沒有錯誤訊息，只是卡住；本書用 `rust,no_run` 展示。
- 跟 Go 比：Go 的 runtime 會在「所有 goroutine 都卡住」時報 `all goroutines are asleep - deadlock!`；Rust 沒有偵測，卡住就是卡住。
