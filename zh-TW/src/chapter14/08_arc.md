# `Arc`

## 本集目標

知道 `Rc` 不能跨執行緒（E0277），改用 `Arc` 讓好幾條執行緒共用同一份資料，並知道 `Arc` 跟 `Rc` 一樣只能讀。

## 正文

### 先試 `Rc`：編譯器擋下來

第 13 章第 5 集說過，`Rc` 只能在單一執行緒裡用。把它交給 `thread::spawn` 看看：

```rust,compile_fail=E0277
use std::rc::Rc;
use std::thread;

fn main() {
    let menu = Rc::new(String::from("夏季菜單"));
    let shared = Rc::clone(&menu);
    let handle = thread::spawn(move || {
        println!("{shared}");
    });
    handle.join().expect("店員出事了");
    println!("{menu}");
}
```

編譯錯誤：

```text
error[E0277]: `Rc<String>` cannot be sent between threads safely
 --> src/main.rs:7:32
  |
7 |       let handle = thread::spawn(move || {
  |                    ------------- ^------
  |                    |             |
  |  __________________|_____________within this `{closure@src/main.rs:7:32: 7:39}`
  | |                  |
  | |                  required by a bound introduced by this call
8 | |         println!("{shared}");
9 | |     });
  | |_____^ `Rc<String>` cannot be sent between threads safely
  |
  = help: within `{closure@src/main.rs:7:32: 7:39}`, the trait `Send` is not implemented for `Rc<String>`
```

重點是第一行：`Rc<String>` cannot be sent between threads safely，下面 `help` 說 `Send` 這個 trait 沒有為 `Rc<String>` 實作。`Send` 先讀成「這個型別的值可以安全地交給另一條執行緒」，第 11 集才正式解釋。這裡只貼前半段，後面兩則 `note` 指出是閉包裡的 `shared` 造成的，還有 `spawn` 要求 `Send` 的位置，不用管。

為什麼不行？`Rc` 的計數是普通的加減。兩條執行緒同時 `clone` 或 drop，計數可能算錯：算少了，資料被提早釋放；算多了，永遠不釋放。單一執行緒不會有這個問題，所以 `Rc` 才這麼便宜。

### 換成 `Arc`

`Arc<T>`（atomic reference counting）的用法跟 `Rc` 幾乎一模一樣：`Arc::new`、`Arc::clone(&a)`、`Arc::strong_count`。差別在計數用的是 atomic 操作（第 12 集），多條執行緒同時加減也不會算錯：

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let menu = Arc::new(String::from("夏季菜單"));
    println!("擁有者：{}", Arc::strong_count(&menu));
    let mut handles = Vec::new();
    for shop in 1..=3 {
        let menu = Arc::clone(&menu);
        handles.push(thread::spawn(move || format!("{shop} 號店用 {menu}")));
    }
    for handle in handles {
        println!("{}", handle.join().expect("店員出事了"));
    }
    println!("擁有者：{}", Arc::strong_count(&menu));
}
```

執行結果：

```text
擁有者：1
1 號店用 夏季菜單
2 號店用 夏季菜單
3 號店用 夏季菜單
擁有者：1
```

- 每一圈用 `Arc::clone(&menu)` 多一個擁有者，`move` 進那一圈的執行緒；三家店讀的是同一份菜單，沒有複製字串。
- `Arc<String>` 可以直接當 `String` 用（`{menu}`、`.len()`），因為它實作了 `Deref`（第 13 章第 3 集）。
- 每條執行緒做完，閉包連同它的 `Arc` 複本一起 drop。三條都 `join` 完之後，計數回到 1，所以最後印的是 1。
- 跟第 3 集的差別：`.clone()` 是各拿一份獨立的資料；`Arc::clone` 是大家共用同一份，只多一個擁有者。

### `Arc` 也只能讀

`Arc` 跟 `Rc` 一樣，多個擁有者共用一份資料，所以不給改（第 13 章第 5 集）：

```rust,compile_fail=E0594
use std::sync::Arc;

fn main() {
    let stock = Arc::new(10);
    let shared = Arc::clone(&stock);
    *stock -= 3;
    println!("{shared}");
}
```

編譯錯誤：

```text
error[E0594]: cannot assign to data in an `Arc`
 --> src/main.rs:6:5
  |
6 |     *stock -= 3;
  |     ^^^^^^^^^^^ cannot assign
  |
  = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<i32>`
```

要共用又要改，下一集把 `Arc` 跟 `Mutex` 組起來。

### 什麼時候用 `Arc`、什麼時候用 `Rc`

- 資料要交給別的執行緒，一定要 `Arc`；`Rc` 會被編譯器擋下。
- 只在單一執行緒內共用，用 `Rc`：`Arc` 的原子操作比較慢，不必付這筆錢。
- 資料活得比執行緒久、而且只是借來讀，優先考慮第 4 集的 `thread::scope`，連 `Arc` 都不需要。

## 重點整理

- `Rc` 不能交給 `thread::spawn`（E0277，`Rc<…>` cannot be sent between threads safely），因為它的計數不是原子操作。
- `Arc<T>` 用法同 `Rc`：`Arc::new`、`Arc::clone(&a)`、`Arc::strong_count`；每條執行緒 `move` 進一個複本，大家共用同一份資料。
- `Arc::clone` 只是計數加 1，不複製資料；最後一個擁有者離開，資料才被 drop。
- `Arc<T>` 跟 `Rc<T>` 一樣只能讀，要改是 E0594；共用又要改，搭配 `Mutex`（下一集）。
- 單一執行緒用 `Rc`，跨執行緒用 `Arc`；借得動的情況，優先用 `thread::scope`。
