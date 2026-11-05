# 把模組拆成多個檔案

## 本集目標

把 `mod menu { ... }` 搬進獨立的檔案，看懂 `mod menu;` 的意思，以及模組樹跟檔案放的位置怎麼對應。

## 正文

### `mod 名稱;`：內容在別的檔案

模組都寫在同一個檔案裡，很快就太長了。把大括號裡的內容搬到另一個檔案，原本的位置只留一行：

```rust,ignore
mod menu;
```

這行後面是分號，不是大括號，意思是「有個叫 `menu` 的模組，內容在 `menu.rs` 裡」。編譯器就會去 `src/menu.rs` 找。我們先把 `lib.rs` 寫好、檔案卻還沒建：

`src/lib.rs`：

```rust,ignore
pub mod menu;
```

`cargo check` 的輸出（省略編譯進度）：

```text
error[E0583]: file not found for module `menu`
 --> src/lib.rs:1:1
  |
1 | pub mod menu;
  | ^^^^^^^^^^^^^
  |
  = help: to create the module `menu`, create file "src/menu.rs" or "src/menu/mod.rs"
  = note: if there is a `mod menu` elsewhere in the crate already, import it with `use crate::...` instead
```

訊息告訴你：要建立 `src/menu.rs`（或 `src/menu/mod.rs`，這是舊寫法，老程式碼常見）。建好：

`src/menu.rs`：

```rust,ignore
pub fn price(drink: &str) -> u32 {
    match drink {
        "紅茶" => 30,
        "奶茶" => 38,
        _ => 45,
    }
}
```

檔案裡不需要再寫 `mod menu { }`：檔案本身就是模組的內容。`menu.rs` 裡的函式照樣要 `pub`，`lib.rs` 的 `pub mod` 也一樣，不加 `pub` 的話，`main.rs` 就走不進這個模組，規則跟第 4 集相同。

### 路徑沒變，只是搬了家

再加一個 `bill` 模組，用上一集的 `use` 引用 `menu`：

`src/bill.rs`：

```rust,ignore
use crate::menu::price;

pub fn total_for(drink: &str, cups: u32) -> u32 {
    price(drink) * cups
}
```

`src/lib.rs`：

```rust,ignore
pub mod bill;
pub mod menu;
```

`use crate::menu::price;` 跟模組寫在同一個檔案時一模一樣：路徑看的是**模組樹**，不是檔案放哪裡。模組搬進別的檔案，所有路徑都不用改。

### 子模組放進同名的資料夾

`menu` 底下想再分出 `size` 模組，`src/menu.rs` 裡宣告 `pub mod size;`，內容放在跟 `menu` 同名的資料夾裡：

```text
src/
├── bill.rs
├── lib.rs
├── main.rs
├── menu
│   └── size.rs
└── menu.rs
```

規則一句話：**模組 `a::b` 的檔案在 `src/a/b.rs`**，`a` 自己的檔案是 `src/a.rs`。再往下一層，就是 `src/a/b/c.rs`。

`src/menu.rs`：

```rust,ignore
pub mod size;

pub fn price(drink: &str) -> u32 {
    match drink {
        "紅茶" => 30,
        "奶茶" => 38,
        _ => 45,
    }
}
```

`src/menu/size.rs`：

```rust,ignore
pub fn extra(size: &str) -> u32 {
    match size {
        "M" => 5,
        "L" => 10,
        _ => 0,
    }
}
```

`src/main.rs`：

```rust,ignore
fn main() {
    let extra = teashop::menu::size::extra("L");
    println!("大杯奶茶：{} 元", teashop::bill::total_for("奶茶", 1) + extra);
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
大杯奶茶：48 元
```

### 跟 Go 比一比

Go 的 package 是一個資料夾：把 `.go` 檔放進去，就自動屬於那個 package，編譯的時候全部納入。Rust 剛好相反：**檔案放在 `src/` 底下不算數，一定要有 `mod` 宣告才會編譯**。沒被任何 `mod` 提到的 `.rs` 檔，編譯器完全不會看。Rust 一個檔案就是一個模組，而不是一個資料夾才是一個。

## 重點整理

- `mod 名稱;`（分號結尾）表示模組的內容在另一個檔案；找不到檔案是 E0583。
- 從 crate 根（`lib.rs` 或 `main.rs`）開始，一層一層用 `mod` 宣告，沒被宣告的 `.rs` 檔不會被編譯。
- 模組 `a::b` 的檔案放在 `src/a/b.rs`，`a` 自己是 `src/a.rs`；舊寫法是 `src/a/mod.rs`。
- 檔案裡直接寫模組的內容，不用再包 `mod 名稱 { }`；路徑看的是模組樹，不是檔案位置，搬進檔案不用改任何路徑。
- 跟 Go 比：Go 一個資料夾是一個 package、檔案自動納入；Rust 一個檔案是一個模組，要靠 `mod` 宣告。
