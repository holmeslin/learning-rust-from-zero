# 關聯函式與 `new` 慣例

## 本集目標

在 `impl` 裡定義沒有 `self` 的關聯函式，用 `型別名稱::函式名稱` 呼叫，並認得 `new` 這個慣例；也看懂 `String::from` 與 `i32::MAX` 就是同一類東西。

## 正文

### 沒有 `self` 的函式

`impl` 裡的函式，如果第一個參數不是 `self`，就叫**關聯函式**（associated function）。它屬於這個型別，卻不需要一個現成的值來呼叫，常用來「建立」這個型別的值。呼叫時寫 `型別名稱::函式名稱`：

```rust
struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    fn new(owner: String) -> Account {
        Account { owner, balance: 0 }
    }

    fn show(&self) {
        println!("{} 的餘額是 {}", self.owner, self.balance);
    }
}

fn main() {
    let account = Account::new(String::from("小美"));
    account.show();
}
```

執行結果：

```text
小美 的餘額是 0
```

- `Account::new(...)`：`::` 前面是型別名稱，不需要先有 `Account` 的值。
- `show(&self)` 是方法，用 `.` 呼叫；`new(owner: String)` 是關聯函式，用 `::` 呼叫。判斷標準只有一個：第一個參數是不是 `self`。
- `new` 裡用了第 3 集的簡寫 `owner`，餘額固定從 0 開始。呼叫的人不需要知道這件事，這是關聯函式的好處：把「怎麼建立」放在型別旁邊。

### `new` 只是慣例

`new` 不是 Rust 的關鍵字，也不是特殊的函式，只是大家約定俗成的名字：只要是「建立一個新的值」的關聯函式，就叫 `new`。取別的名字也可以，但大家都叫 `new`，別人讀你的程式比較容易懂。

要用 `::` 呼叫的東西，不能用 `.` 呼叫：

```rust,compile_fail=E0599
struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    fn new(owner: String) -> Account {
        Account { owner, balance: 0 }
    }
}

fn main() {
    let account = Account::new(String::from("小美"));
    let other = account.new(String::from("小華"));
    println!("{} {}", other.owner, other.balance);
}
```

編譯錯誤：

```text
error[E0599]: no method named `new` found for struct `Account` in the current scope
  --> src/main.rs:14:25
   |
 1 | struct Account {
   | -------------- method `new` not found for this struct
...
14 |     let other = account.new(String::from("小華"));
   |                         ^^^ this is an associated function, not a method
   |
   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in an impl for the type `Account`
  --> src/main.rs:7:5
   |
 7 |     fn new(owner: String) -> Account {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use associated function syntax instead
   |
14 -     let other = account.new(String::from("小華"));
14 +     let other = Account::new(String::from("小華"));
   |
```

`new` 沒有 `self`，不是方法，`account.new(...)` 找不到。訊息的 `note` 把原因講得很清楚。

### 你早就用過了

第 1 章起，你一直在寫 `String::new()`、`String::from("...")`，以及 `i32::MAX`。現在看得懂了：

- `String::new()`、`String::from(...)` 是 `String` 的關聯函式，用 `::` 呼叫，用來建立 `String`。
- `i32::MAX` 是**關聯常數**（associated constant）：寫在 `impl` 裡的 `const`，屬於型別，一樣用 `::` 取用。

自己的型別也能定義關聯常數：

```rust
struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    const MIN_BALANCE: i64 = -1000;

    fn new(owner: String) -> Account {
        Account { owner, balance: 0 }
    }

    fn can_withdraw(&self, amount: i64) -> bool {
        self.balance - amount >= Account::MIN_BALANCE
    }
}

fn main() {
    let account = Account::new(String::from("小美"));
    println!("提 500：{}", account.can_withdraw(500));
    println!("提 1500：{}", account.can_withdraw(1500));
    println!("{} 的最低餘額：{}", account.owner, Account::MIN_BALANCE);
}
```

執行結果：

```text
提 500：true
提 1500：false
小美 的最低餘額：-1000
```

關聯常數的名字照 `const` 的慣例用全大寫。方法裡用的時候，也要寫 `Account::MIN_BALANCE`，不能只寫 `MIN_BALANCE`。

## 重點整理

- `impl` 裡沒有 `self` 參數的函式叫關聯函式，用 `型別名稱::函式名稱(...)` 呼叫；有 `self` 的才是方法，用 `.` 呼叫。
- `new` 是慣例名稱，不是關鍵字：慣例上用來建立型別的值，把建立的細節（例如餘額從 0 開始）包在型別旁邊。
- 對值用 `.` 呼叫關聯函式會找不到（E0599）。
- `String::new()`、`String::from(...)` 是關聯函式；`i32::MAX` 是關聯常數（`impl` 裡的 `const`），用 `型別名稱::名稱` 取用。
