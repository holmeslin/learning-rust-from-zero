# `match`

## 本集目標

用 `match` 依 enum 是哪個 variant 走不同的分支，同時把 variant 裡的資料取出來，並知道 `match` 本身是運算式。

## 正文

### `==` 不能用

要判斷一個 `Status` 是不是 `Paid`，Go 的直覺是 `==`：

```rust,compile_fail=E0369
#[derive(Debug)]
enum Status {
    Pending,
    Paid,
    Shipped,
}

fn main() {
    let status = Status::Paid;
    if status == Status::Paid {
        println!("已付款");
    }
}
```

編譯錯誤：

```text
error[E0369]: binary operation `==` cannot be applied to type `Status`
  --> src/main.rs:10:15
   |
10 |     if status == Status::Paid {
   |        ------ ^^ ------------ Status
   |        |
   |        Status
```

`==` 對自己定義的型別預設不能用，要先實作 `PartialEq`，第 8 章才教，訊息後面 `help` 建議的 `#[derive(PartialEq)]` 先不用管。enum 本來就有更好的工具：`match`。

### `match` 的寫法

```rust
enum Status {
    Pending,
    Paid,
    Shipped,
}

fn describe(status: Status) {
    match status {
        Status::Pending => println!("等待付款"),
        Status::Paid => println!("已付款，準備出貨"),
        Status::Shipped => println!("已出貨"),
    }
}

fn main() {
    describe(Status::Pending);
    describe(Status::Paid);
    describe(Status::Shipped);
}
```

執行結果：

```text
等待付款
已付款，準備出貨
已出貨
```

- `match status { ... }`：拿 `status` 去跟大括號裡的每個分支比。
- 每個分支（arm）寫成 `模式 => 運算式,`。`Status::Paid` 這種寫在 `=>` 左邊的東西叫**模式**（pattern），第 2 章解構 tuple 時看過這個詞。
- 分支由上往下比，第一個符合的就執行，其他分支跳過，不會往下掉到下一個分支。這點跟 Go 的 `switch` 一樣（Go 要寫 `fallthrough` 才會往下掉）；不一樣的是下一集的窮盡檢查，以及 `match` 是運算式。

### 取出 variant 裡的資料

上一集放進 variant 的資料，用 `match` 取出來：

```rust
enum Discount {
    Regular,
    Percent(u32),
    Threshold { min: u32, off: u32 },
}

fn final_price(price: u32, discount: Discount) -> u32 {
    match discount {
        Discount::Regular => price,
        Discount::Percent(p) => price * (100 - p) / 100,
        Discount::Threshold { min, off } => {
            if price >= min {
                price - off
            } else {
                price
            }
        }
    }
}

fn main() {
    let list = [
        Discount::Regular,
        Discount::Percent(20),
        Discount::Threshold { min: 800, off: 150 },
    ];
    for discount in list {
        println!("{}", final_price(1000, discount));
    }
}
```

執行結果：

```text
1000
800
850
```

- `Discount::Percent(p)`：模式的寫法跟建立值一樣，括號裡的 `p` 是新變數，裝著 `Percent` 裡的數字，只能在這個分支裡用。
- `Discount::Threshold { min, off }`：struct 式的模式列出欄位名稱，把值放進同名的變數，跟第 5 章第 3 集的欄位初始化簡寫是同一種寫法，方向相反。
- 分支的右邊可以是一整個區塊 `{ ... }`，區塊最後一個運算式就是這個分支的值。

### `match` 是運算式

第 1 章第 8 集說過 `if` 是運算式，`match` 也是：整個 `match` 的值，就是被選到的那個分支的值。`final_price` 就是把 `match` 直接當成函式最後的運算式，沒有 `return`。要放進變數也行，`let text = match status { ... };`，分號照寫。

所以每個分支的值型別必須一樣。上面三個分支都是 `u32`；如果其中一個分支寫成字串，就是 E0308。

上一集的 `never read` 警告不見了，因為每個 variant 的資料現在都被讀出來用了。

## 重點整理

- `match 值 { 模式 => 運算式, ... }`：由上往下比，第一個符合的分支執行，不會往下掉到下一個分支。
- `==` 不能直接比較自己定義的 enum（E0369），判斷是哪個 variant 用 `match`。
- 模式 `Discount::Percent(p)`、`Discount::Threshold { min, off }` 會把 variant 裡的資料放進新變數，只能在該分支使用。
- `match` 是運算式，值就是被選到的分支的值，每個分支的值型別要一樣；分支右邊也可以是 `{ ... }` 區塊。
- 沒有列到的 variant 會怎樣，下一集講。
