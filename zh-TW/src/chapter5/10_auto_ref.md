# `.` 會自動參照與解參照

## 本集目標

知道用 `.` 呼叫方法時，編譯器會依方法的 `self` 寫法，自動在前面補上 `&`、`&mut` 或 `*`；也知道這個機制補不了什麼。

## 正文

### 值、`&`、`&mut` 都能用 `.`

第 7 集的 `account.show()`，`show` 要的是 `&self`，但 `account` 是一個 `Account`，不是參照。我們沒有寫 `(&account).show()`，編譯器卻接受了。原因是：**用 `.` 呼叫方法時，編譯器會看方法的第一個參數需要什麼，自動在前面補上 `&`、`&mut`，或是 `*`，補到對得上為止**。這叫自動參照與解參照（automatic referencing and dereferencing）。

```rust
struct Account {
    balance: i64,
}

impl Account {
    fn show(&self) {
        println!("餘額 {}", self.balance);
    }

    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }
}

fn main() {
    let mut account = Account { balance: 0 };
    account.deposit(500);

    let shared = &account;
    shared.show();
    Account::show(&account);
}
```

執行結果：

```text
餘額 500
餘額 500
```

- `account.deposit(500)`：`account` 是 `Account`，`deposit` 要 `&mut self`，編譯器補上 `&mut account`。
- `shared.show()`：`shared` 已經是 `&Account`，剛好對上 `&self`，什麼都不用補。
- `Account::show(&account)`：這是方法呼叫的**完整寫法**，`self` 就是第一個參數，要自己寫 `&`。`shared.show()` 跟它做的是同一件事。

`.` 方便的地方，就是讓你不用在每次呼叫前想「這裡要幾個 `&`」。

### 欄位，以及對參照呼叫 `len`

讀寫欄位也會自動解參照。第 8 集 `deposit` 裡的 `self.balance += amount`，`self` 是 `&mut Account`，卻沒有寫 `*`，就是這個原因：`.` 會自己從參照走到它指向的 struct。

第 4 章你一直在對參照呼叫方法，現在可以解釋了：

```rust
fn main() {
    let mut s = String::from("哈囉");
    let r = &mut s;
    r.push_str("世界");
    println!("{}", r.len());
}
```

執行結果：

```text
12
```

`push_str` 要的是 `&mut self`，`r` 本來就是 `&mut String`，直接對上；`len` 要的是 `&self`，編譯器自動把 `&mut String` 當成 `&String`。

### 補得了寫法，補不了權限

自動補 `&`、`&mut`、`*` 只是省掉打字，不會幫你變出不存在的權限。手上只有 `&Account`，卻呼叫要 `&mut self` 的方法：

```rust,compile_fail=E0596
struct Account {
    balance: i64,
}

impl Account {
    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }
}

fn bonus(account: &Account) {
    account.deposit(10);
}

fn main() {
    let account = Account { balance: 500 };
    bonus(&account);
    println!("{}", account.balance);
}
```

編譯錯誤：

```text
error[E0596]: cannot borrow `*account` as mutable, as it is behind a `&` reference
  --> src/main.rs:12:5
   |
12 |     account.deposit(10);
   |     ^^^^^^^ `account` is a `&` reference, so it cannot be borrowed as mutable
   |
help: consider changing this to be a mutable reference
   |
11 | fn bonus(account: &mut Account) {
   |                    +++
```

`&` 是只能讀的借用，沒辦法變成 `&mut`。`help` 建議把參數型別改成 `&mut Account`，呼叫端也要跟著用 `&mut`（第 4 章第 3 集）。

呼叫要 `self` 的方法也一樣：`self` 是拿走所有權，但借來的值你沒有資格拿走：

```rust,compile_fail=E0507
struct Account {
    balance: i64,
}

impl Account {
    fn close(self) -> i64 {
        self.balance
    }
}

fn settle(account: &Account) -> i64 {
    account.close()
}

fn main() {
    let account = Account { balance: 500 };
    println!("領回 {}", settle(&account));
}
```

編譯錯誤：

```text
error[E0507]: cannot move out of `*account` which is behind a shared reference
  --> src/main.rs:12:5
   |
12 |     account.close()
   |     ^^^^^^^ ------- `*account` moved due to this method call
   |     |
   |     move occurs because `*account` has type `Account`, which does not implement the `Copy` trait
   |
note: `Account::close` takes ownership of the receiver `self`, which moves `*account`
  --> src/main.rs:6:14
   |
 6 |     fn close(self) -> i64 {
   |              ^^^^
```

這跟第 4 章第 7 集的 E0507 是同一件事：不能把 `&` 後面的值 move 走。這裡只貼前半段，後面還有一則提到 `Clone` 的 `note`，那是第 8 章的內容，先不用管。

## 重點整理

- 用 `.` 呼叫方法時，編譯器依方法的 `self` 寫法，自動在前面補 `&`、`&mut` 或 `*`；所以值、`&`、`&mut` 都能直接用 `.` 呼叫。
- 讀寫欄位也會自動解參照，所以 `self.balance += amount` 不用寫 `*`。
- 完整寫法是 `型別名稱::方法名(...)`，`self` 當第一個參數，要自己寫 `&` 或 `&mut`（例如 `Account::show(&account)`）。
- 第 4 章對 `&String`、`&mut String` 直接呼叫 `len`、`push_str`，就是這個機制。
- 自動補的只是寫法，不是權限：只有 `&` 卻呼叫 `&mut self` 的方法是 E0596；對借來的值呼叫 `self` 的方法是 E0507。
