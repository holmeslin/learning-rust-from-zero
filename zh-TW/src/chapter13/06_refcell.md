# `RefCell<T>` 與內部可變性

## 本集目標

知道什麼是內部可變性：只拿 `&self` 也能修改資料；會用 `RefCell` 的 `borrow` 與 `borrow_mut`，並知道它把借用規則的檢查搬到執行時期，違反就 panic。

## 正文

### `&self` 卻想改資料

收銀機 `Till` 要記下每一筆銷售，所以 `record` 要改內部的 `Vec`。如果方法拿的是 `&self`：

```rust,compile_fail=E0596
struct Till {
    sales: Vec<u32>,
}

impl Till {
    fn record(&self, amount: u32) {
        self.sales.push(amount);
    }
}

fn main() {
    let till = Till { sales: Vec::new() };
    till.record(30);
    println!("{}", till.sales.len());
}
```

編譯錯誤：

```text
error[E0596]: cannot borrow `self.sales` as mutable, as it is behind a `&` reference
 --> src/main.rs:7:9
  |
7 |         self.sales.push(amount);
  |         ^^^^^^^^^^ `self` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
6 |     fn record(&mut self, amount: u32) {
  |                +++
```

`help` 建議改成 `&mut self`，多半也真的該這樣改。但有時候改不了：方法的簽名是別人定的（第 8 章第 11 集的 `Display::fmt` 拿的就是 `&self`），或是物件被好幾個擁有者共用（上一集的 `Rc` 只給 `&`）。這時要用**內部可變性**（interior mutability）：外面看是 `&`，裡面的資料卻可以改。

### `RefCell`：借用規則改在執行時期檢查

```rust
use std::cell::RefCell;

struct Till {
    sales: RefCell<Vec<u32>>,
}

impl Till {
    fn record(&self, amount: u32) {
        self.sales.borrow_mut().push(amount);
    }

    fn total(&self) -> u32 {
        self.sales.borrow().iter().sum()
    }
}

fn main() {
    let till = Till {
        sales: RefCell::new(Vec::new()),
    };
    till.record(30);
    till.record(45);
    println!("共 {} 筆，{} 元", till.sales.borrow().len(), till.total());
}
```

執行結果：

```text
共 2 筆，75 元
```

- `RefCell::new(值)` 把值包起來。`borrow()` 借出只能讀的 `Ref`，`borrow_mut()` 借出可寫的 `RefMut`，兩個都像參照一樣用（它們實作了 `Deref`）。
- `till` 沒有寫 `let mut`，`record` 也只拿 `&self`，資料照樣改得動。
- 規則跟第 4 章第 4 集一樣：同時間可以有很多個 `borrow()`，或只有一個 `borrow_mut()`，不能混著來。差別在**什麼時候檢查**：普通參照在編譯時期擋下，`RefCell` 在程式執行時才檢查。
- `Ref` 與 `RefMut` 離開作用域（被 drop）時，才把借用還回去。像 `till.record(30)` 裡的 `borrow_mut()` 只是這一行的暫時值，行尾就還了。

### 違反規則：panic

如果上一個借用還沒還，又要借寫：

```rust,exit=101
use std::cell::RefCell;

fn main() {
    let stock = RefCell::new(10);
    let first = stock.borrow_mut();
    println!("先借到 {first}");
    let second = stock.borrow_mut();
    println!("{second}");
}
```

執行結果（某一次）：

```text
先借到 10

thread 'main' (159732656) panicked at src/main.rs:7:24:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- `first` 存在變數裡，一直活到 `main` 結束，借用就一直沒還，第二次 `borrow_mut()` 就 panic，訊息是 `RefCell already borrowed`，並指出是哪一行。panic 訊息裡括號中的數字每次都不同。
- 修法是讓第一個借用早點還：縮小它所在的區塊，或用第 4 集的 `drop(first)`。
- 不想 panic，可以用 `try_borrow_mut()`，它回傳 `Result`：已經被借著就是 `Err`。

編譯錯誤變成執行時的 panic，是 `RefCell` 的代價：程式碼寫對了，編譯器就替你保證沒事；用了 `RefCell`，這份保證要靠你自己顧好。能用普通的 `&`、`&mut` 解決的，就不要用它。

### 跟 Go 比一比

Go 的指標隨時都能寫，沒有人檢查。Rust 的 `&` 預設只能讀，`RefCell` 是你主動選的例外，而且有人在執行時把關：它比較像單執行緒版的 `sync.Mutex`，只是同一個地方重複「加鎖」不會卡死，而是立刻 panic，直接告訴你哪一行。

## 重點整理

- 內部可變性：外面是 `&`，裡面的資料照樣能改。要改的資料放進 `RefCell<T>`。
- `borrow()` 得到只能讀的 `Ref`，`borrow_mut()` 得到可寫的 `RefMut`；`Ref` 與 `RefMut` 被 drop 時才還回借用。
- 借用規則照舊（多個讀或一個寫），但改在執行時期檢查，違反就 panic（`RefCell already borrowed`）；`try_borrow_mut()` 回傳 `Result`。
- 用得到它的時機：方法簽名固定是 `&self`，或資料被多個擁有者共用。
- 跟 Go 比：Go 的指標隨時能寫；Rust 預設只能讀，`RefCell` 是主動選的例外，違反就 panic。
