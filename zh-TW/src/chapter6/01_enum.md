# `enum`

## 本集目標

用 `enum` 定義「只會是其中一種」的型別，用 `Status::Paid` 這樣的寫法建立值，並知道它跟 Go 用整數常數表示狀態有什麼不同。

## 正文

### 只有幾種可能的值

第 5 章的 `Order` 用 `paid: bool` 記錄付款了沒。如果訂單的狀態有三種：等待付款、已付款、已出貨，`bool` 就不夠用了。用整數（0、1、2）得自己記哪個數字代表什麼，用字串又可能拼錯。

`enum`（列舉）是為這種情況設計的：先把所有可能列出來，這個型別的值就只會是其中一種：

```rust
#[derive(Debug)]
enum Status {
    Pending,
    Paid,
    Shipped,
}

fn main() {
    let steps = [Status::Pending, Status::Paid, Status::Shipped];
    for step in steps {
        println!("{:?}", step);
    }
}
```

執行結果：

```text
Pending
Paid
Shipped
```

- `enum Status { ... }` 定義一個新的型別，叫 `Status`，跟 `struct` 一樣寫在函式外面。
- 大括號裡列出每一種可能，叫做 **variant**，用逗號隔開。型別名稱和 variant 名稱都用大寫開頭。
- 建立值時寫 `Status::Paid`：型別名稱、`::`、variant 名稱。這個 `::` 跟 `i32::MAX`、`String::from` 的 `::` 是同一個符號。
- 第一行的 `#[derive(Debug)]` 第 5 章第 6 集用過，enum 一樣照抄，加了才能用 `{:?}` 印出 variant 的名字。

### 當成一般型別用

`Status` 跟 `i32`、`String` 一樣是型別，能當函式的參數，也能放進 struct 的欄位：

```rust
#[derive(Debug)]
enum Status {
    Pending,
    Paid,
    Shipped,
}

struct Order {
    id: u32,
    status: Status,
}

fn show(status: Status) {
    println!("目前狀態：{:?}", status);
}

fn main() {
    let order = Order {
        id: 7,
        status: Status::Pending,
    };
    println!("訂單 {} 的狀態是 {:?}", order.id, order.status);
    show(Status::Paid);
    show(Status::Shipped);
}
```

執行結果：

```text
訂單 7 的狀態是 Pending
目前狀態：Paid
目前狀態：Shipped
```

欄位型別寫 `Status`，建立 `Order` 時就只能放 `Status` 的值。

### 型別擋得住不屬於它的值

函式要求 `Status`，傳整數進去就是 E0308：

```rust,compile_fail=E0308
#[derive(Debug)]
enum Status {
    Pending,
    Paid,
    Shipped,
}

fn show(status: Status) {
    println!("目前狀態：{:?}", status);
}

fn main() {
    show(Status::Paid);
    show(2);
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
  --> src/main.rs:14:10
   |
14 |     show(2);
   |     ---- ^ expected `Status`, found integer
   |     |
   |     arguments to this function are incorrect
```

整數 `2` 不是 `Status`，即使 `Shipped` 是第三個 variant 也一樣：variant 沒有對應的數字，要建立 `Status` 的值，只能寫出某個 variant 的名字。

### 跟 Go 比一比

Go 沒有 `enum`，慣例是整數型別加 `iota` 常數：

```go
type Status int

const (
    Pending Status = iota
    Paid
    Shipped
)

func show(s Status) { fmt.Println(s) }

func main() {
    show(Paid) // 印出 1，不是 Paid
    show(2)    // 編譯成功
    show(99)   // 編譯成功，沒有這個狀態，Go 也不管
}
```

Go 的 `Status` 只是整數換個名字，編譯器不知道「合法的值就這三個」。Rust 的 `enum` 差在三個地方：

- 任何整數都能變成 Go 的 `Status`；Rust 只有列出來的 variant 能成為 `Status` 的值。
- Go 印出來是數字，除非自己寫 `String()` 方法；Rust 加了 `#[derive(Debug)]` 就會印出 variant 的名字。
- Go 的 `var s Status` 沒給值就是零值 0，悄悄變成第一個常數；Rust 沒有零值（第 1 章第 5 集），一定要寫出是哪個 variant。

## 重點整理

- `enum 型別名稱 { variant, ... }` 定義只會是其中一種的型別；型別名稱與 variant 名稱都用大寫開頭。
- 用 `型別名稱::variant` 建立值，例如 `Status::Paid`；enum 前面一樣可以照抄 `#[derive(Debug)]`，才能用 `{:?}` 印出來。
- enum 是一般的型別，能當函式參數、struct 欄位、陣列元素；傳入其他型別（例如整數）是 E0308。
- 跟 Go 比：Go 的列舉只是整數加常數，任何整數都能轉成它，還有零值；Rust 的值只能是列出來的 variant，沒有零值。
- 怎麼判斷一個 `Status` 是哪個 variant，要用 `match`，第 3 集教。
