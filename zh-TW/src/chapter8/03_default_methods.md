# 預設方法

## 本集目標

在 trait 裡替方法寫好預設的本體，讓實作的型別可以直接沿用，也可以自己覆寫；知道預設方法能呼叫同一個 trait 的其他方法。

## 正文

### 在 trait 裡寫好本體

上一集的 trait 方法都只有簽名。方法也可以直接在 trait 裡把本體寫好，這叫**預設方法**（default method）。型別實作 trait 時沒有另外寫，就沿用預設的版本。Go 的 `interface` 只能放簽名，沒有這個功能。

```rust
struct Tea {
    toppings: u32,
}

struct Coffee {
    shots: u32,
}

trait Priced {
    fn price(&self) -> u32;

    fn discounted(&self) -> u32 {
        self.price() * 90 / 100
    }
}

impl Priced for Tea {
    fn price(&self) -> u32 {
        35 + self.toppings * 5
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        45 + self.shots * 10
    }

    fn discounted(&self) -> u32 {
        self.price()
    }
}

fn main() {
    let tea = Tea { toppings: 2 };
    let coffee = Coffee { shots: 1 };
    println!("奶茶 {} → {}", tea.price(), tea.discounted());
    println!("咖啡 {} → {}", coffee.price(), coffee.discounted());
}
```

執行結果：

```text
奶茶 45 → 40
咖啡 55 → 55
```

- `price` 沒有本體，每個型別一定要自己寫；`discounted` 有本體（打九折），是預設方法。
- `Tea` 的 `impl` 只寫了 `price`，所以 `tea.discounted()` 用預設的計算方式：45 乘 90 除 100，整數除法捨去小數，得到 40。
- `Coffee` 自己也寫了 `discounted`，這叫**覆寫**（override）：咖啡不打折，直接回傳 `self.price()`。`coffee.discounted()` 用的是 `Coffee` 自己的版本。覆寫時簽名一樣要跟 trait 的一致。

### 預設方法只能用 trait 裡有的東西

預設方法的本體寫在 trait 裡，這時候還不知道將來是哪個型別來實作，所以只能呼叫 trait 裡有的方法，像上面的 `self.price()`。它沒辦法寫 `self.toppings` 去讀欄位，因為不是每個型別都有這個欄位。

需要讀的資料，就做成一個必須實作的方法（像 `price`），讓每個型別自己提供；預設方法再靠這個方法算出需要的東西。

### 全部都有預設也可以

如果 trait 的方法全部都有預設，`impl` 區塊可以是空的：

```rust
struct Tea {
    toppings: u32,
}

trait Welcome {
    fn welcome(&self) -> String {
        String::from("歡迎光臨")
    }
}

impl Welcome for Tea {}

fn main() {
    let tea = Tea { toppings: 2 };
    println!("{}，加了 {} 種配料", tea.welcome(), tea.toppings);
}
```

執行結果：

```text
歡迎光臨，加了 2 種配料
```

`impl Welcome for Tea {}` 這一行不能省：沒有它，`Tea` 就不算實作 `Welcome`（上一集的規則），`tea.welcome()` 也呼叫不到。

## 重點整理

- 在 trait 裡直接寫方法本體，就是預設方法；實作的型別沒寫，就沿用預設。
- 在 `impl` 裡寫一個同名的方法就是覆寫，簽名要跟 trait 一致。
- 預設方法可以呼叫同一個 trait 的其他方法（例如 `self.price()`），但不能讀型別的欄位。
- 沒有本體的方法一定要實作（E0046）；所有方法都有預設時，`impl 某trait for 型別 {}` 可以是空的，但這一行不能省。
