# `mod`

## 本集目標

用 `mod` 把檔案裡的程式，分成幾個有名字的區塊，並用 `::` 路徑呼叫區塊裡的東西。

## 正文

### 一個 `mod` 就是一個有名字的區塊

程式一長，函式名稱就容易撞名，也很難找東西。`mod 名稱 { ... }` 開一個區塊（**模組**，module），裡面可以放函式、`struct`、`enum`、常數，甚至別的 `mod`。從外面要用，寫 `模組名稱::項目名稱`，跟你早就看過的 `String::from`、`i32::MAX` 是同一種寫法：

```rust
mod menu {
    pub fn price(drink: &str) -> u32 {
        match drink {
            "紅茶" => 30,
            "奶茶" => 38,
            _ => 45,
        }
    }
}

mod bill {
    pub fn total(prices: &[u32]) -> u32 {
        let mut sum = 0;
        for p in prices {
            sum += p;
        }
        sum
    }
}

fn main() {
    let tea = menu::price("紅茶");
    let milk_tea = menu::price("奶茶");
    println!("紅茶 {tea}、奶茶 {milk_tea}");
    println!("合計 {}", bill::total(&[tea, milk_tea]));
}
```

執行結果：

```text
紅茶 30、奶茶 38
合計 68
```

`menu` 管價格，`bill` 管帳單，各自一個區塊。函式前面的 `pub` 先照抄，下一集才解釋；現在只要知道，少了它，模組外面就叫不到這個函式。

模組名稱跟變數、函式一樣，用小寫加底線。宣告的先後順序無所謂：`mod bill` 寫在 `mod menu` 前面也一樣能用。

### 整支程式是最上面一層

你的程式本身就是最外層的模組，名字叫 `crate`（根）。`mod` 在它底下分枝，形成一棵樹。上面這支程式的樹是：

```text
crate
├── menu
│   └── price
├── bill
│   └── total
└── main
```

`main` 也是掛在根底下的函式。第 2 集說過，crate 是編譯的單位，它最外層的模組就叫 `crate`。現在只要記得：**路徑可以從 `crate` 開始，一路走到想要的東西**。

### 模組裡面看不到外面的名字

Go 的同一個 package 裡，所有檔案的函式互相看得到。Rust 的每個 `mod` 是獨立的範圍：在 `bill` 裡面，沒辦法直接寫 `menu::price`。

```rust,compile_fail=E0433
mod menu {
    pub fn price(drink: &str) -> u32 {
        if drink == "紅茶" { 30 } else { 45 }
    }
}

mod bill {
    pub fn total_for(drink: &str, cups: u32) -> u32 {
        menu::price(drink) * cups
    }
}

fn main() {
    println!("{}", bill::total_for("紅茶", 2));
}
```

編譯錯誤：

```text
error[E0433]: cannot find module or crate `menu` in this scope
 --> src/main.rs:9:9
  |
9 |         menu::price(drink) * cups
  |         ^^^^ use of unresolved module or unlinked crate `menu`
```

在 `bill` 這個範圍裡找不到叫 `menu` 的模組，因為 `menu` 是在外面一層。訊息後面還有 `help`，建議加一行 `use crate::menu;`，第 5 集才學，現在改寫成**絕對路徑**：從 `crate` 開始，逐段寫出來。

```rust
mod menu {
    pub fn price(drink: &str) -> u32 {
        if drink == "紅茶" { 30 } else { 45 }
    }
}

mod bill {
    pub fn total_for(drink: &str, cups: u32) -> u32 {
        crate::menu::price(drink) * cups
    }
}

fn main() {
    println!("{}", bill::total_for("紅茶", 2));
}
```

執行結果：

```text
60
```

`crate::menu::price` 不管寫在哪個模組裡都指到同一個函式。`main` 在根上，所以它寫 `bill::total_for` 不用加 `crate::`，因為 `bill` 就在它旁邊。

### 模組可以巢狀

`mod` 裡面可以再放 `mod`，路徑就多一段。例如在 `mod shop` 裡放 `pub mod menu` 和 `pub mod bill`，從外面呼叫是 `shop::bill::total_for(...)`，從根開始的絕對路徑是 `crate::shop::menu::price`。一層模組一個 `::`，而且裡面那層也要 `pub`，外面才走得進去。

### 跟 Go 比一比

Go 的 package 由資料夾決定：檔案放進哪個資料夾、開頭寫哪個 `package`，就屬於哪個 package。Rust 的模組是在程式碼裡用 `mod` **宣告**出來的，一個檔案可以宣告好幾個模組；模組跟資料夾、檔案怎麼對應，第 6 集才說。

## 重點整理

- `mod 名稱 { ... }` 宣告模組，裡面放函式、型別、常數或別的 `mod`；從外面用 `模組::項目` 的路徑呼叫。
- 整支程式是最外層的模組，叫 `crate`；`crate::menu::price` 是從根開始的絕對路徑。
- 每個模組是獨立的範圍，裡面看不到外面（兄弟）的名字，要寫路徑；找不到是 E0433。
- 模組可以巢狀，路徑一層一個 `::`；模組裡的東西、巢狀的每一層，都要加 `pub` 外面才叫得到，下一集說明。
- 跟 Go 比：Go 的 package 由資料夾決定；Rust 的模組用 `mod` 宣告。
