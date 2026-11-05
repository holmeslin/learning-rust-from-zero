# atomic

## 本集目標

用 `AtomicU32`、`AtomicBool` 這類 atomic 型別，在不加鎖的情況下讓多條執行緒安全地改同一個數字或旗標，並用它做出可以修改的全域 `static`。

## 正文

### 一個數字，不用鎖

只是要替一個計數加 1，拿鎖、改、放鎖有點小題大作。`std::sync::atomic` 提供 `AtomicU32`、`AtomicBool`、`AtomicUsize` 等型別：每一次讀、寫、加減，都是 CPU 一次做完、不會被別的執行緒攔腰打斷的原子操作（atomic operation）。

它們的方法全都只要 `&self`（內部可變性，跟 `Cell` 一樣，但是執行緒安全），所以可以放在 `static` 裡。第 2 章第 1 集說「可以修改的全域變數留到第 14 章」，答案之一就是它：

```rust
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

static NEXT_TICKET: AtomicU32 = AtomicU32::new(1);

fn take_ticket() -> u32 {
    NEXT_TICKET.fetch_add(1, Ordering::SeqCst)
}

fn main() {
    let mut handles = Vec::new();
    for _ in 0..4 {
        handles.push(thread::spawn(take_ticket));
    }
    let mut tickets = Vec::new();
    for handle in handles {
        tickets.push(handle.join().expect("店員出事了"));
    }
    tickets.sort();
    println!("四位客人拿到的號碼：{tickets:?}");
    println!("下一個是 {}", NEXT_TICKET.load(Ordering::SeqCst));
}
```

執行結果：

```text
四位客人拿到的號碼：[1, 2, 3, 4]
下一個是 5
```

- `fetch_add(1, ...)` 加 1，回傳加之前的值，所以四條執行緒拿到的號碼互不相同。哪一條拿到哪個號碼不固定，所以先收集、`sort` 再印。
- `load` 讀目前的值，`store` 寫入新值；還有 `fetch_sub`、`swap`、`compare_exchange` 等，用法類似。
- 每個方法都要一個 `Ordering` 參數，決定這次操作跟其他記憶體讀寫之間有什麼先後保證。先一律寫 `Ordering::SeqCst`（最嚴格、最容易想對）；`Relaxed`、`Acquire`、`Release` 是進階的效能調整，這本書不展開。注意這個 `Ordering` 在 `std::sync::atomic`，跟第 8 章 `std::cmp::Ordering` 同名，是兩個不同的型別。
- `thread::spawn(take_ticket)`：函式名稱也能當閉包交給 `spawn`。
- atomic 加到溢位時環繞，不會 panic（`AtomicU8` 從 250 加 10 會變成 4），跟第 1 章第 16 集的整數溢位不一樣。

### 為什麼 `static` 只能放 atomic 這種型別

`static` 的值全程式共用，每條執行緒都看得到，所以它的型別必須是 `Sync`（第 11 集）。換成 `Cell` 就不行：

```rust,compile_fail=E0277
use std::cell::Cell;

static COUNT: Cell<u32> = Cell::new(0);

fn main() {
    COUNT.set(1);
    println!("{}", COUNT.get());
}
```

編譯錯誤：

```text
error[E0277]: `Cell<u32>` cannot be shared between threads safely
 --> src/main.rs:3:15
  |
3 | static COUNT: Cell<u32> = Cell::new(0);
  |               ^^^^^^^^^ `Cell<u32>` cannot be shared between threads safely
  |
  = help: the trait `Sync` is not implemented for `Cell<u32>`
  = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicU32` instead
  = note: shared static variables must have a type that implements `Sync`
```

最後一則 `note` 說得很直接：`shared static variables must have a type that implements Sync`，前一則 `note` 順便建議了 `RwLock` 或 `AtomicU32`。想改的全域值，可以放 atomic，或是第 7 集的 `Mutex`（下一集會看到）；另一種 `static mut` 要靠 `unsafe` 才碰得到（E0133），第 16 章再談。

### 旗標：叫另一條執行緒收工

`AtomicBool` 常拿來當旗標。店員一直在做飲料，直到店長宣布打烊：

```rust
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let closing = AtomicBool::new(false);
    let cups = AtomicU32::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            while !closing.load(Ordering::SeqCst) {
                cups.fetch_add(1, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(10));
            }
        });
        thread::sleep(Duration::from_millis(100));
        closing.store(true, Ordering::SeqCst);
    });
    println!("打烊了，有做出飲料嗎？{}", cups.load(Ordering::SeqCst) > 0);
}
```

執行結果：

```text
打烊了，有做出飲料嗎？true
```

店員每一圈先看旗標；店長睡 100 毫秒後 `store(true, ...)`，店員下一圈就收工，`scope` 才能結束。做了幾杯每次不同，所以只印「有沒有做」。

什麼時候用 atomic、什麼時候用 `Mutex`？atomic 只能保護單一個數字或旗標；要一次改好幾個欄位、或讓它們保持一致（第 9 集的 `Till` 營業額加筆數），就要 `Mutex`。

### 跟 Go 比一比

Go 的 `sync/atomic` 也有 `atomic.Uint32` 這種型別，但有兩個地方不一樣：

```go
var next atomic.Uint32
next.Store(1)
got := next.Add(1) // got 是 2：Go 回傳加完之後的值
```

Rust 的 `fetch_add` 回傳加之前的值，拿來當號碼牌比較順手，但從 Go 轉過來會算錯一個數。另外 Go 的 atomic 操作沒有 `Ordering` 參數，一律是最嚴格的順序；Rust 要你自己選，先全寫 `SeqCst` 就等於 Go 的行為。

## 重點整理

- atomic 型別（`AtomicU32`、`AtomicBool`、`AtomicUsize` 等）在不加鎖的情況下安全地讀寫單一個值；方法只要 `&self`。
- 常用方法：`new`、`load`、`store`、`fetch_add`、`fetch_sub`、`swap`；每個都要 `Ordering`，先一律用 `Ordering::SeqCst`。
- `static` 的型別必須是 `Sync`，所以可以修改的全域值用 atomic（或 `Mutex`），放 `Cell` 是 E0277。
- `fetch_add` 回傳加之前的值，溢位時環繞，不 panic。跟 Go 比：Go 的 `Add` 回傳加完的值，也沒有 `Ordering` 參數。
- 只有單一個數字或旗標才用 atomic；多個欄位要一致，用 `Mutex`。
