# 多個 `impl` 區塊

## 本集目標

知道同一個型別可以有好幾個 `impl` 區塊，效果跟寫在同一個區塊一樣，並看一遍這一章的 struct 與方法寫法。

## 正文

### 一個型別，好幾個 `impl`

同一個型別可以寫不只一個 `impl` 區塊，所有區塊裡的方法和關聯函式，都屬於這個型別：

```rust
struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    fn new(owner: String) -> Account {
        Account { owner, balance: 0 }
    }
}

impl Account {
    fn show(&self) {
        println!("{} 的餘額是 {}", self.owner, self.balance);
    }
}

impl Account {
    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }
}

fn main() {
    let mut account = Account::new(String::from("小美"));
    account.deposit(300);
    account.show();
}
```

執行結果：

```text
小美 的餘額是 300
```

三個區塊跟合併成一個完全一樣，`main` 看不出差別。`impl` 區塊寫在 `struct` 前面或後面都可以，區塊的順序也不影響，方法可以呼叫另一個區塊裡的方法。

### 為什麼要這樣設計

這一章的 struct 用一個 `impl` 就夠了。多個區塊的用處在後面幾章：

- 第 8 章的 `trait` 要用 `impl 某trait for 型別` 實作，是另外的區塊，跟這一章的 `impl 型別` 分開寫。
- 泛型型別的 `impl<T>`（第 8 章），也是自成一個區塊。
- 方法很多時，可以按用途分組：建立的放一塊、查詢的放一塊、修改的放一塊。

### 名字不能重複

不同區塊之間，同一個型別不能有兩個同名的方法或關聯函式：

```rust,compile_fail=E0592
struct Account {
    balance: i64,
}

impl Account {
    fn show(&self) {
        println!("餘額 {}", self.balance);
    }
}

impl Account {
    fn show(&self) {
        println!("{}", self.balance);
    }
}

fn main() {
    let account = Account { balance: 500 };
    println!("{}", account.balance);
}
```

編譯錯誤：

```text
error[E0592]: duplicate definitions with name `show`
  --> src/main.rs:6:5
   |
 6 |     fn show(&self) {
   |     ^^^^^^^^^^^^^^ duplicate definitions for `show`
...
12 |     fn show(&self) {
   |     -------------- other definition for `show`
```

兩個 `show` 同時屬於 `Account`，編譯器不知道要用哪一個，所以拒絕。

### 這一章的整理

這一章先定義有欄位的型別，再替型別加上方法：

| 想做什麼 | 寫法 |
| --- | --- |
| 定義型別 | `struct Account { owner: String, balance: i64 }` |
| 建立、讀取 | `Account { owner, balance: 0 }`、`account.balance` |
| 以舊值為底 | `Account { owner: ..., ..old }`（沒寫的欄位會 move 或複製） |
| 沒有欄位名稱 | `struct Meters(f64);`、`struct Marker;` |
| 印出來檢查 | `#[derive(Debug)]` 加 `{:?}`、`{:#?}` |
| 方法 | `impl Account { fn show(&self) }`，用 `account.show()` 呼叫 |
| 只能讀、修改、拿走 | `&self`、`&mut self`、`self` |
| 建立型別的值 | `fn new(...) -> Account`，用 `Account::new(...)` 呼叫 |

下一章的 `enum`，也一樣用 `impl` 加方法，寫法跟這一章完全相同。

## 重點整理

- 同一個型別可以有多個 `impl` 區塊，效果等於合併成一個；區塊的順序不影響。
- 多個區塊的用途在後面：`trait` 的實作（第 8 章）、泛型、以及把方法分組。
- 同一個型別不能有兩個同名的方法或關聯函式，跨區塊也一樣（E0592）。
- 這一章的寫法：`struct` 定義型別，`impl` 加方法，`&self`／`&mut self`／`self` 決定方法怎麼拿到值，`型別::new` 建立新值。
