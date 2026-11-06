# `Rc<RefCell<T>>`

## 本集目標

把 `Rc` 和 `RefCell` 疊在一起，得到「多個擁有者，每個都能改」的共用資料；也知道它的代價，以及不該拿它來躲借用錯誤。

## 正文

### 多個擁有者，都能改

`Rc<T>` 能共用但不能改，`RefCell<T>` 能改但只有一個擁有者。疊起來，`Rc<RefCell<T>>` 兩件事都行。例子是一張儲值卡，媽媽和小明都拿它付錢：

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Card {
    balance: i32,
}

struct Member {
    name: String,
    card: Rc<RefCell<Card>>,
}

fn main() {
    let card = Rc::new(RefCell::new(Card { balance: 100 }));
    let mom = Member {
        name: String::from("媽媽"),
        card: Rc::clone(&card),
    };
    let kid = Member {
        name: String::from("小明"),
        card: Rc::clone(&card),
    };

    mom.card.borrow_mut().balance -= 30;
    kid.card.borrow_mut().balance -= 45;
    println!("{} 看到餘額 {}", mom.name, mom.card.borrow().balance);
    println!("{} 看到餘額 {}", kid.name, kid.card.borrow().balance);
    println!("擁有者：{}", Rc::strong_count(&card));
}
```

執行結果：

```text
媽媽 看到餘額 25
小明 看到餘額 25
擁有者：3
```

- `Rc<RefCell<Card>>` 由外往內讀：外層的 `Rc` 讓 `card` 有三個擁有者（`card`、`mom.card`、`kid.card`），內層的 `RefCell` 讓裡面的 `Card` 能改。
- `mom.card.borrow_mut().balance -= 30`：先 `borrow_mut()` 借出可寫的 `RefMut<Card>`，再改欄位。兩個人改的是同一張卡，所以最後兩邊看到的餘額一樣，都是 25。

### 閉包也能共用

兩個閉包都要寫進同一份記錄，每個閉包各 `move` 一個 `Rc` 進去（第 11 章第 3 集）：

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let log_a = Rc::clone(&log);
    let log_b = Rc::clone(&log);

    let order = move |drink: &str| log_a.borrow_mut().push(format!("點了 {drink}"));
    let cancel = move |drink: &str| log_b.borrow_mut().push(format!("取消 {drink}"));

    order("紅茶");
    order("咖啡");
    cancel("紅茶");
    println!("{:?}", log.borrow());
}
```

執行結果：

```text
["點了 紅茶", "點了 咖啡", "取消 紅茶"]
```

### 衝突時照樣 panic

借用規則在執行時期檢查，不管借的人是不是同一個擁有者：

```rust,exit=101
use std::cell::RefCell;
use std::rc::Rc;

struct Card {
    balance: i32,
}

fn main() {
    let card = Rc::new(RefCell::new(Card { balance: 100 }));
    let other = Rc::clone(&card);

    let reading = card.borrow();
    println!("媽媽看到 {}", reading.balance);
    other.borrow_mut().balance -= 45;
    println!("{}", reading.balance);
}
```

執行結果（某一次）：

```text
媽媽看到 100

thread 'main' (159738008) panicked at src/main.rs:14:11:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`reading` 還沒還，`other` 就要寫，`RefCell` 在執行時擋下來。編譯器看不出這兩個變數指向同一張卡，這種錯誤只有跑到那一行才會發現。

### 不要拿它躲借用錯誤

`Rc<RefCell<T>>` 適合的情況，是資料真的被好幾處共用、而且每一處都會改，例如單執行緒程式裡的共用狀態、一堆物件都要通知同一個記錄。它不是「編譯器不讓我借，包一層就過了」的捷徑：那只是把編譯錯誤換成執行時的 panic。

遇到借用錯誤，先回頭看所有權能不能重新安排：改傳 `&mut`、拆開 struct 的欄位、讓函式回傳新值。確定資料就是共用的，才用這個組合。它也只能在單一執行緒裡用，多執行緒要換成 `Arc<Mutex<T>>`（第 14 章第 9 集）。

## 重點整理

- `Rc<RefCell<T>>`：外層 `Rc` 提供多個擁有者，內層 `RefCell` 提供修改；`x.borrow_mut().欄位 = ...` 是常見寫法。
- 每個擁有者各自 `Rc::clone`，閉包要用就 `move` 一份進去。
- 借用規則在執行時期檢查：有 `borrow()` 還沒還就 `borrow_mut()`，會 panic（`RefCell already borrowed`），而且不管是哪個擁有者借的。
- 它不是躲避借用錯誤的捷徑，先想能不能重新安排所有權；只能用在單一執行緒。
