# `&self`、`&mut self`、`self`

## 本集目標

分清楚方法第一個參數的三種寫法：`&self` 只能讀、`&mut self` 修改、`self` 拿走，知道呼叫端要怎麼配合，並能對照 Go 的接收者。

## 正文

### 三種寫法

上一集的 `&self` 只是其中一種。方法的第一個參數有三種寫法，對應第 3、4 章學過的三種「拿到值的方式」：

| 寫法 | 意思 | 呼叫之後 |
| --- | --- | --- |
| `&self` | 借用，只能讀 | 原本的值還在 |
| `&mut self` | 可變借用，可以改欄位 | 原本的值還在，內容可能變了 |
| `self` | 拿走所有權 | 原本的值被 move，不能再用 |

三種放在一起看：

```rust
struct Account {
    owner: String,
    balance: i64,
}

impl Account {
    fn show(&self) {
        println!("{} 的餘額是 {}", self.owner, self.balance);
    }

    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }

    fn close(self) -> i64 {
        println!("{} 的帳戶結清了", self.owner);
        self.balance
    }
}

fn main() {
    let mut account = Account {
        owner: String::from("小美"),
        balance: 500,
    };
    account.show();
    account.deposit(200);
    account.show();
    let rest = account.close();
    println!("領回 {rest}");
}
```

執行結果：

```text
小美 的餘額是 500
小美 的餘額是 700
小美 的帳戶結清了
領回 700
```

- `show(&self)`：只能讀，呼叫完 `account` 還在。
- `deposit(&mut self, ...)`：改 `self.balance`。呼叫端的 `account` 一定要用 `let mut`；忘了的話是 E0596（cannot borrow as mutable），`help` 會建議補上 `mut`。方法會不會改 struct，看簽名上有沒有 `&mut self` 就知道。
- `close(self)`：把整個 `Account` 拿走。結清之後帳戶就不存在了，所以只回傳餘額。

### `self` 會 move

`self` 拿走所有權，規則跟傳參數進函式一樣（第 3 章第 9 集）。呼叫之後再用原來的變數，就是 E0382：

```rust,compile_fail=E0382
struct Account {
    balance: i64,
}

impl Account {
    fn show(&self) {
        println!("{}", self.balance);
    }

    fn close(self) -> i64 {
        self.balance
    }
}

fn main() {
    let account = Account { balance: 500 };
    let rest = account.close();
    println!("領回 {rest}");
    account.show();
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `account`
  --> src/main.rs:19:5
   |
16 |     let account = Account { balance: 500 };
   |         ------- move occurs because `account` has type `Account`, which does not implement the `Copy` trait
17 |     let rest = account.close();
   |                        ------- `account` moved due to this method call
18 |     println!("領回 {rest}");
19 |     account.show();
   |     ^^^^^^^ value borrowed here after move
   |
note: `Account::close` takes ownership of the receiver `self`, which moves `account`
  --> src/main.rs:10:14
   |
10 |     fn close(self) -> i64 {
   |              ^^^^
```

訊息直接指出：`close` 的簽名是 `self`，呼叫時把 `account` 拿走了。這個寫法適合「用完就沒有了」的操作，例如結清帳戶、把整個值轉成別的東西。

### 跟 Go 比一比

Go 的方法有兩種接收者：

```go
func (a Account) Show() {
    fmt.Println(a.Owner, a.Balance) // 值接收者：a 是複製品
}

func (a *Account) Deposit(n int) {
    a.Balance += n // 指標接收者：改的是呼叫端那一份
}
```

對照起來有三個不一樣：

- Go 的值接收者會**複製**整個 struct，函式裡怎麼改都不影響呼叫端，呼叫之後原本的值照用。Rust 的 `self` 是 **move**，呼叫之後原本的值不能用。
- Go 的指標接收者能改欄位，但呼叫的地方看不出來，`a.Deposit(200)` 對任何可取址的變數都行。Rust 的 `&mut self` 要求變數是 `mut`。
- Go 沒有「只能讀的指標接收者」，要讀就得用值接收者，每次呼叫都複製一份。Rust 的 `&self` 只借不複製。

## 重點整理

- 方法第一個參數有三種寫法：`&self` 只能讀、`&mut self` 可改欄位、`self` 拿走整個值。
- `&mut self` 的方法，呼叫端的變數要是 `let mut`，否則 E0596。
- `self` 的方法呼叫之後，原本的值被 move，再用是 E0382。
- 只需要讀就用 `&self`，要改用 `&mut self`，用完就不要的操作才用 `self`。
- 跟 Go 比：Go 的值接收者複製、Rust 的 `self` 是 move；Go 的指標接收者呼叫端看不出會改，Rust 要 `mut`；Rust 的 `&self` 是 Go 沒有的只能讀的借用。
