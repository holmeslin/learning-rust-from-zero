# enum 的方法

## 本集目標

用 `impl` 幫 enum 加方法與關聯函式，並學會在方法裡對 `self` 做 `match`。

## 正文

### 跟 struct 一模一樣

第 5 章的 `impl`、`&self`、`&mut self`、關聯函式，對 enum 完全適用。方法裡最常見的動作，就是對 `self` 做 `match`：

```rust
enum Discount {
    Regular,
    Percent(u32),
    Threshold { min: u32, off: u32 },
}

impl Discount {
    fn apply(&self, price: u32) -> u32 {
        match self {
            Discount::Regular => price,
            Discount::Percent(p) => price * (100 - p) / 100,
            Discount::Threshold { min, off } => {
                if price >= *min {
                    price - off
                } else {
                    price
                }
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
        println!("{}", discount.apply(1000));
    }
}
```

執行結果：

```text
1000
800
850
```

前面的 `final_price(1000, discount)` 變成了 `discount.apply(1000)`，折扣的計算規則跟 `Discount` 放在一起，不用再到處找。

### `match self` 拿到的是參照

方法的第一個參數是 `&self`，所以 `match self` 比對的是 `&Discount`。跟第 7 集的 `match &nickname` 一樣，綁到的 `p`、`min`、`off` 都是參照（`&u32`）。

- `price * (100 - p) / 100`、`price - off`：`u32` 跟 `&u32` 做算術，可以直接寫。
- 比較不行，要用第 4 章第 7 集的 `*` 取出值，上面的 `price >= *min` 就是這樣來的。少了 `*` 的話：

```rust,compile_fail=E0308
enum Discount {
    Threshold { min: u32, off: u32 },
}

impl Discount {
    fn apply(&self, price: u32) -> u32 {
        match self {
            Discount::Threshold { min, off } => {
                if price >= min { price - off } else { price }
            }
        }
    }
}

fn main() {
    let d = Discount::Threshold { min: 800, off: 150 };
    println!("{}", d.apply(1000));
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:9:29
  |
9 |                 if price >= min { price - off } else { price }
  |                    -----    ^^^ expected `u32`, found `&u32`
  |                    |
  |                    expected because this is `u32`
```

`` expected `u32`, found `&u32` ``，`help` 建議在 `min` 前面加 `*`，照做就好。

### `&mut self` 與 `*self =`

要換掉 enum 目前是哪個 variant，用 `&mut self`，再用 `*self = ...` 整個換成新值（`*` 同樣是第 4 章的解參照）。下面的 `Status` 再加一個關聯函式 `from_code`，回傳 `Option<Status>`：

```rust
#[derive(Debug)]
enum Status {
    Pending,
    Paid,
    Shipped,
}

impl Status {
    fn from_code(code: u32) -> Option<Status> {
        match code {
            0 => Some(Status::Pending),
            1 => Some(Status::Paid),
            2 => Some(Status::Shipped),
            _ => None,
        }
    }

    fn advance(&mut self) {
        *self = match self {
            Status::Pending => Status::Paid,
            Status::Paid | Status::Shipped => Status::Shipped,
        };
    }
}

fn main() {
    let mut status = Status::Pending;
    status.advance();
    println!("{:?}", status);
    println!("{:?}", Status::from_code(2));
    println!("{:?}", Status::from_code(9));
}
```

執行結果：

```text
Paid
Some(Shipped)
None
```

- `from_code` 沒有 `self`，是關聯函式，用 `Status::from_code(...)` 呼叫；它把整數轉成 `Option<Status>`，認得的回傳 `Some`，不認得的回傳 `None`。
- `advance` 要改自己，用 `&mut self`；呼叫端的變數要 `let mut`（第 5 章第 8 集）。`match self` 算出新的 variant，`*self =` 再把它放回去。

## 重點整理

- enum 用 `impl` 加方法與關聯函式，規則跟 struct 相同：`&self` 只能讀、`&mut self` 修改、沒有 `self` 的是關聯函式。
- 方法裡常對 `self` 做 `match`；對 `&self` 比對時，綁到的變數是參照，算術可以直接用，比較要加 `*`（少了是 E0308）。
- 要換成別的 variant：`&mut self` 搭配 `*self = 新值`。
- 關聯函式能回傳 `Option<Status>`，例如 `Status::from_code(2)`，把整數轉成 enum。
