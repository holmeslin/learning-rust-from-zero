# `trait` 定義與實作

## 本集目標

用 `trait` 定義一組方法，再用 `impl 某trait for 型別` 讓不同型別各自實作；認得少實作一個方法時的 E0046。

## 正文

### 先定義 trait，再替型別實作

飲料店的奶茶、咖啡、白開水長得完全不一樣，卻都有「價錢」。`trait` 把這種共通的能力寫成一組方法簽名，對應 Go 的 `interface`：

```rust
struct Tea {
    toppings: u32,
}

struct Coffee {
    shots: u32,
}

struct Water;

trait Priced {
    fn price(&self) -> u32;
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
}

impl Priced for Water {
    fn price(&self) -> u32 {
        0
    }
}

fn main() {
    let tea = Tea { toppings: 2 };
    let coffee = Coffee { shots: 1 };
    let water = Water;
    println!("奶茶 {} 元", tea.price());
    println!("咖啡 {} 元", coffee.price());
    println!("白開水 {} 元", water.price());
}
```

執行結果：

```text
奶茶 45 元
咖啡 55 元
白開水 0 元
```

- `trait Priced { ... }` 定義一個 trait。裡面只寫方法的簽名：名字、參數、回傳型別，後面接分號，沒有方法本體。
- `impl Priced for Tea { ... }` 讀成「`Tea` 實作了 `Priced`」。方法簽名要跟 trait 裡寫的一模一樣，本體則在這裡才寫。
- 呼叫方式沒有變，還是第 5 章學過的 `tea.price()`。同一個 trait，不同的型別各有各的計算方式。
- `Water` 是第 5 章第 5 集的 unit struct，沒有欄位也能實作 trait。那一集說過它的用途要到第 8 章才出現，就是這裡：型別本身只是個名字，行為全部寫在 `impl` 裡。

### trait 的實作自成一個區塊

第 5 章第 11 集說過，`impl 型別 { ... }` 可以寫很多個區塊。`impl 某trait for 型別 { ... }` 則是另一種區塊：裡面只能放那個 trait 規定的方法，不能順便塞別的。

- 一個 trait 可以讓很多型別實作，上面的 `Priced` 就有三個。
- 一個型別也可以實作很多個 trait，各寫各的 `impl` 區塊。
- trait 裡可以有不只一個方法，實作的時候每一個都要寫。

### 少寫一個方法

下面的 `Priced` 多了一個 `name`，但 `Coffee` 的實作只寫了 `price`：

```rust,compile_fail=E0046
struct Coffee {
    shots: u32,
}

trait Priced {
    fn price(&self) -> u32;
    fn name(&self) -> String;
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        45 + self.shots * 10
    }
}

fn main() {
    let coffee = Coffee { shots: 1 };
    println!("{}", coffee.price());
}
```

編譯錯誤：

```text
error[E0046]: not all trait items implemented, missing: `name`
  --> src/main.rs:10:1
   |
 7 |     fn name(&self) -> String;
   |     ------------------------- `name` from trait
...
10 | impl Priced for Coffee {
   | ^^^^^^^^^^^^^^^^^^^^^^ missing `name` in implementation
```

訊息直接點名缺了 `name`，箭頭指著 trait 裡的那一行。在實作 trait 的地方，trait 要求的方法一個都不能少。反過來，在 `impl Priced for Coffee` 裡多寫 trait 沒有的方法，則是 E0407（`not a member of trait`），要放到 `impl Coffee { ... }` 才行。

## 重點整理

- `trait 名字 { fn 方法(&self) -> 型別; }` 定義一組方法簽名，方法後面是分號，沒有本體。
- `impl 某trait for 型別 { ... }` 讓型別實作 trait，每個方法的簽名都要跟 trait 一樣，本體寫在這裡。
- 呼叫跟一般方法一樣：`tea.price()`；同一個 trait 可以讓很多型別各自實作，一個型別也可以實作很多個 trait。
- unit struct 沒有欄位也能實作 trait（回收第 5 章第 5 集）。
- trait 要求的方法少實作一個是 E0046；多寫 trait 沒有的方法是 E0407。
