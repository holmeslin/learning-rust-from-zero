# 參照循環與 `Weak`

## 本集目標

知道 `Rc` 的參照循環會讓值永遠不被 drop，會用 `Weak`（`Rc::downgrade`、`upgrade`）打破循環。

## 正文

### 循環：永遠不 drop

兩家店互相登記對方是合作店，各自用 `Rc` 持有對方。`Shop` 實作 `Drop`，被 drop 時印「打烊」：

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Shop {
    name: String,
    partner: RefCell<Option<Rc<Shop>>>,
}

impl Drop for Shop {
    fn drop(&mut self) {
        println!("{} 打烊", self.name);
    }
}

fn open(name: &str) -> Rc<Shop> {
    Rc::new(Shop {
        name: String::from(name),
        partner: RefCell::new(None),
    })
}

fn main() {
    let lone = open("戊店");
    let a = open("丙店");
    let b = open("丁店");
    *a.partner.borrow_mut() = Some(Rc::clone(&b));
    *b.partner.borrow_mut() = Some(Rc::clone(&a));
    let (x, y) = (Rc::strong_count(&a), Rc::strong_count(&b));
    println!("{} 單獨營業", lone.name);
    println!("擁有者：丙店 {x}，丁店 {y}");
}
```

執行結果：

```text
戊店 單獨營業
擁有者：丙店 2，丁店 2
戊店 打烊
```

`main` 結束時，只有「戊店打烊」印出來，丙店和丁店都沒有。

丙店有兩個擁有者：變數 `a`，以及丁店裡的 `partner`。`main` 結束，`a` 被 drop，計數從 2 變成 1，不是 0，所以丙店沒被 drop；丁店同理。兩家店各被對方抓著，誰也放不掉，這就是**參照循環**（reference cycle）。這塊記憶體要到程式結束才還給系統，叫記憶體洩漏（memory leak）。Rust 不認為洩漏是不安全的，所以編譯器不會擋，只能靠你設計時避開。

### `Weak`：不算擁有者的參照

打破循環的辦法，是讓其中一邊不算擁有者。`Weak<T>` 是**弱參照**（weak reference），`Rc` 的擁有者則叫**強參照**（strong reference）：

- `Rc::downgrade(&rc)` 從 `Rc` 做出一個 `Weak`：強參照數不變，另外記一個弱參照數（`Rc::weak_count`）。
- 只有強參照數決定值要不要 drop；弱參照不攔著它。
- `Weak` 不能直接用。呼叫 `upgrade()` 會得到 `Option<Rc<T>>`：值還在，就是 `Some`，同時多一個強參照；值已經被 drop，就是 `None`。
- `Weak::new()` 建立一個沒有指向任何東西的 `Weak`，`upgrade()` 永遠是 `None`。

總部（`Hq`）擁有分店（強參照），分店只用弱參照指回總部：

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Hq {
    name: String,
    stores: RefCell<Vec<Rc<Store>>>,
}

struct Store {
    name: String,
    hq: RefCell<Weak<Hq>>,
}

impl Drop for Hq {
    fn drop(&mut self) {
        println!("{} 關閉", self.name);
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        println!("{} 打烊", self.name);
    }
}

fn hq_name(store: &Store) -> String {
    match store.hq.borrow().upgrade() {
        Some(hq) => hq.name.clone(),
        None => String::from("（沒有總部）"),
    }
}

fn main() {
    let store = Rc::new(Store {
        name: String::from("信義店"),
        hq: RefCell::new(Weak::new()),
    });
    println!("總部：{}", hq_name(&store));

    let hq = Rc::new(Hq {
        name: String::from("台北總部"),
        stores: RefCell::new(Vec::new()),
    });
    hq.stores.borrow_mut().push(Rc::clone(&store));
    *store.hq.borrow_mut() = Rc::downgrade(&hq);
    println!("總部：{}", hq_name(&store));
    let (s, w) = (Rc::strong_count(&hq), Rc::weak_count(&hq));
    println!("台北總部：強參照 {s}，弱參照 {w}");

    drop(hq);
    println!("總部：{}", hq_name(&store));
}
```

執行結果：

```text
總部：（沒有總部）
總部：台北總部
台北總部：強參照 1，弱參照 1
台北總部 關閉
總部：（沒有總部）
信義店 打烊
```

- 剛建立的分店，`Weak::new()` 還沒指向總部，`upgrade()` 是 `None`；`Rc::downgrade(&hq)` 之後，分店指回總部：總部的強參照仍是 1（只有 `hq`），弱參照是 1。
- `drop(hq)` 讓總部的強參照變 0，總部就被 drop，印出「關閉」，它的 `stores` 也跟著 drop，分店少一個強參照。分店之後再 `upgrade()` 就是 `None`。
- `main` 結束時才輪到 `store` 被 drop，印出「打烊」，一個都沒洩漏。

### 跟 Go 比一比

Go 的 GC 會處理循環：兩個物件互相指著，只要沒人從外面指到它們，GC 就會回收。所以 Go 寫雙向鏈結、父子互指都不用多想。Rust 的 `Rc` 只算次數，循環會洩漏；父子結構要自己決定誰強誰弱，通常是「父擁有子（強），子指回父（弱）」。

## 重點整理

- `Rc` 互相持有（A 抓著 B、B 抓著 A）會形成參照循環：強參照數永遠不到 0，值不會被 drop，記憶體洩漏；編譯器不會擋。
- `Rc::downgrade(&rc)` 做出 `Weak<T>`：不算擁有者，只增加弱參照數；`Rc::strong_count`、`Rc::weak_count` 分別讀出兩種數量。
- `weak.upgrade()` 回傳 `Option<Rc<T>>`：值還在是 `Some`，已經 drop 是 `None`；`Weak::new()` 的 `upgrade()` 永遠是 `None`。
- 常見用法：父擁有子（`Rc`），子指回父（`Weak`），避免循環。
- 跟 Go 比：Go 的 GC 會回收循環；Rust 的 `Rc` 不會。
