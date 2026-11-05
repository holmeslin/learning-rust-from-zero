# `impl` 與方法

## 本集目標

用 `impl` 區塊幫 struct 定義方法，第一個參數寫 `&self`，用 `account.show()` 的形式呼叫。

## 正文

### 把函式放進 `impl`

到目前為止，要處理 struct，只能寫獨立的函式。**方法**（method）則是屬於某個型別的函式，寫在 `impl` 區塊裡：

```rust
struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    fn show(&self) {
        println!("{} 的餘額是 {}", self.owner, self.balance);
    }
}

fn main() {
    let account = Account {
        owner: String::from("小美"),
        balance: 500,
    };
    account.show();
}
```

執行結果：

```text
小美 的餘額是 500
```

- `impl Account { ... }`：這個區塊裡的函式，都屬於 `Account`。`impl` 是 implementation（實作）的縮寫。
- 方法的**第一個參數是 `self`**，代表「呼叫這個方法的那個值」。`&self` 是借用它：只能讀，不拿走。
- 方法裡面讀欄位，一定要寫 `self.owner`，不能只寫 `owner`。
- 呼叫時寫 `account.show()`。`.` 前面的 `account` 會自動成為 `self`，括號裡不用再傳它。

它做的事情，跟寫成 `fn show(account: &Account)` 的獨立函式一樣；差別是呼叫的寫法，以及函式跟型別綁在一起。

### 方法也可以有其他參數和回傳值

`self` 之後可以接任意多個參數，用逗號隔開，回傳值照常寫 `->`。同一個 `impl` 裡的方法，彼此可以用 `self.方法名()` 互相呼叫：

```rust
struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    fn is_overdrawn(&self) -> bool {
        self.balance < 0
    }

    fn can_pay(&self, amount: i64) -> bool {
        self.balance >= amount
    }

    fn show(&self) {
        if self.is_overdrawn() {
            println!("{} 透支了", self.owner);
        } else {
            println!("{} 的餘額是 {}", self.owner, self.balance);
        }
    }
}

fn main() {
    let account = Account {
        owner: String::from("小美"),
        balance: 500,
    };
    account.show();
    println!("付得起 300 嗎？{}", account.can_pay(300));
    println!("付得起 800 嗎？{}", account.can_pay(800));

    let broke = Account {
        owner: String::from("小華"),
        balance: -50,
    };
    broke.show();
}
```

執行結果：

```text
小美 的餘額是 500
付得起 300 嗎？true
付得起 800 嗎？false
小華 透支了
```

`can_pay(&self, amount: i64)`：呼叫時只傳 `amount`，`self` 是 `.` 前面的那個值，所以寫成 `account.can_pay(300)`。

### 你早就用過方法了

前面幾章先照抄的 `s.len()`、`s.push_str("...")`、`s.clone()`，還有第 1 章讀輸入的 `read_line`、`trim`、`parse`，全都是方法。它們是標準函式庫的作者寫在 `impl` 裡的，我們用 `.` 呼叫，跟 `account.show()` 完全一樣。

現在也能解釋，為什麼只有 `push_str` 要 `let mut s`：`len` 的第一個參數是 `&self`，只能讀；`push_str` 的第一個參數是 `&mut self`，要改 `String`，所以變數得是 `mut`。這一集的方法都只是讀資料，所以都用 `&self`；`&mut self` 和 `self` 下一集講。

## 重點整理

- `impl 型別名稱 { ... }` 定義屬於這個型別的方法；方法是第一個參數為 `self` 的函式。
- `&self` 借用呼叫方法的那個值；方法裡用 `self.欄位` 讀欄位，用 `self.方法名()` 呼叫其他方法。
- 呼叫寫 `值.方法名(其他參數)`；`.` 前面的值會自動成為 `self`，不用再傳。
- 方法可以有其他參數和回傳值，寫法跟一般函式一樣。
- 第 1～3 章照抄的 `len`、`push_str`、`clone`、`read_line` 等，都是別人寫好的方法；這一集只用 `&self`（只能讀），`&mut self` 與 `self` 下一集講。
