# `Send` 與 `Sync`

## 本集目標

知道 `Send` 與 `Sync` 各自保證什麼、哪些常見型別沒有它們，並看得懂「cannot be sent between threads safely」這類編譯錯誤。

## 正文

### 兩個標記 trait

前幾集的 E0277 都在說同一件事：型別沒有 `Send` 或 `Sync`。這兩個是 `std::marker` 裡的 trait，沒有任何方法，只是一個標記：

- `Send`：這個型別的值，可以安全地搬到另一條執行緒。`thread::spawn` 的簽名（第 3 集）有 `F: Send`，閉包搬進去的每一樣東西都要是 `Send`。
- `Sync`：好幾條執行緒可以同時拿著這個型別的 `&T`。`T` 是 `Sync`，就等於 `&T` 是 `Send`。範圍執行緒（第 4 集）借變數給執行緒，要的就是這個。

你不用自己寫 `impl Send for ...`：編譯器自動判斷。一個 struct 的每個欄位都是 `Send`，它就是 `Send`；只要有一個欄位不是，整個 struct 就不是。

### 常見型別

| 型別 | `Send` | `Sync` | 原因 |
| --- | --- | --- | --- |
| `i32`、`String`、`Vec<T>` | 是 | 是 | 普通的資料（`Vec<T>` 要看 `T`） |
| `Rc<T>` | 否 | 否 | 計數不是原子操作 |
| `RefCell<T>`、`Cell<T>` | 是 | 否 | 搬走沒問題；但兩條執行緒各拿一個 `&`，就能同時改，而它們不是原子操作 |
| `Arc<T>` | 要 `T` 是 `Send + Sync` | 同左 | 大家共用同一份，所以 `T` 本身得能被共用 |
| `Mutex<T>` | 要 `T` 是 `Send` | 同左 | 鎖把存取排成一次一條，`T` 只要搬得動就夠了 |

最後一列是 `Mutex` 的用處：`Cell` 與 `RefCell` 不是 `Sync`，放進 `Mutex` 之後，整個 `Mutex` 就是 `Sync`，可以共用了。

用兩個空函式，就能讓編譯器替你檢查某個型別有沒有這兩個標記：

```rust
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};

fn is_send<T: Send>() {}
fn is_sync<T: Sync>() {}

fn main() {
    is_send::<String>();
    is_sync::<String>();
    is_send::<Vec<u32>>();
    is_send::<RefCell<u32>>();
    is_send::<Cell<u32>>();
    is_send::<Arc<String>>();
    is_sync::<Arc<String>>();
    is_send::<Mutex<RefCell<u32>>>();
    is_sync::<Mutex<RefCell<u32>>>();
    println!("這些型別都符合");
}
```

執行結果：

```text
這些型別都符合
```

函式本體是空的，重點在 `T: Send`、`T: Sync` 這兩個 trait bound（第 8 章第 5 集）：型別不符合，編譯就失敗。`is_sync::<RefCell<u32>>()` 就不會編譯，但 `Mutex<RefCell<u32>>` 可以。

### 一個欄位不合格，整個型別就不合格

`Menu` 裡放了一個 `Rc`：

```rust,compile_fail=E0277
use std::rc::Rc;

struct Menu {
    name: Rc<String>,
}

fn is_send<T: Send>() {}

fn main() {
    is_send::<Menu>();
    let menu = Menu {
        name: Rc::new(String::from("夏季菜單")),
    };
    println!("{}", menu.name);
}
```

編譯錯誤：

```text
error[E0277]: `Rc<String>` cannot be sent between threads safely
  --> src/main.rs:10:15
   |
10 |     is_send::<Menu>();
   |               ^^^^ `Rc<String>` cannot be sent between threads safely
   |
   = help: within `Menu`, the trait `Send` is not implemented for `Rc<String>`
note: required because it appears within the type `Menu`
  --> src/main.rs:3:8
   |
 3 | struct Menu {
   |        ^^^^
note: required by a bound in `is_send`
  --> src/main.rs:7:15
   |
 7 | fn is_send<T: Send>() {}
   |               ^^^^ required by this bound in `is_send`
```

`help` 寫 `within Menu`，`note` 寫 `required because it appears within the type Menu`：問題出在欄位的 `Rc<String>`，是 `Menu` 被它拖累。看到這種訊息，先找出是哪個型別沒有 `Send`／`Sync`，再決定換型別（`Rc` 換 `Arc`），或是不要把它交給別的執行緒。

### `Arc<RefCell<T>>` 為什麼也不行

`Arc` 讓資料能共用，那 `Arc<RefCell<u32>>` 呢？把它交給 `thread::spawn`，也是同一類錯誤。用檢查函式看比較短：

```rust,compile_fail=E0277
use std::cell::RefCell;
use std::sync::Arc;

fn is_send<T: Send>() {}

fn main() {
    is_send::<Arc<RefCell<u32>>>();
}
```

編譯錯誤：

```text
error[E0277]: `RefCell<u32>` cannot be shared between threads safely
 --> src/main.rs:7:15
  |
7 |     is_send::<Arc<RefCell<u32>>>();
  |               ^^^^^^^^^^^^^^^^^ `RefCell<u32>` cannot be shared between threads safely
  |
  = help: the trait `Sync` is not implemented for `RefCell<u32>`
  = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
  = note: required for `Arc<RefCell<u32>>` to implement `Send`
note: required by a bound in `is_send`
 --> src/main.rs:4:15
  |
4 | fn is_send<T: Send>() {}
  |               ^^^^ required by this bound in `is_send`
```

`Arc<T>` 要 `Send`，需要 `T` 同時是 `Send` 與 `Sync`；`RefCell` 不是 `Sync`，所以整個 `Arc<RefCell<u32>>` 都不能搬走。訊息裡的 `required for Arc<RefCell<u32>> to implement Send` 說的就是這件事。解法是把 `RefCell` 換成 `Mutex`，也就是第 9 集的 `Arc<Mutex<T>>`。

自己替型別宣告 `Send`／`Sync`（`unsafe impl`）是可以的，但要保證執行緒安全的人是你，不是編譯器。`unsafe` 在第 16 章第 5 集教，第 6 集有一個替包了原始指標的型別寫 `unsafe impl Send` 的例子。

## 重點整理

- `Send`：值可以搬到別的執行緒；`Sync`：`&T` 可以同時被好幾條執行緒拿著（`T: Sync` 等於 `&T: Send`）。
- `thread::spawn` 要閉包是 `Send`；`thread::scope` 借用的東西要 `Sync`。兩個 trait 沒有方法，由編譯器依欄位自動推斷。
- `Rc` 兩個都不是；`RefCell`、`Cell` 是 `Send` 但不是 `Sync`；`Arc<T>` 要 `T` 同時是 `Send + Sync`；`Mutex<T>` 只要 `T` 是 `Send`，所以能把非 `Sync` 的型別變成可共用。
- struct 只要有一個欄位不是 `Send`／`Sync`，整個 struct 就不是；錯誤訊息的 `within` 與 `required because` 會指出源頭。
- 想確認某個型別，寫一個 `fn is_send<T: Send>() {}` 再呼叫它，讓編譯器回答。
