# `pub(crate)`

## 本集目標

用 `pub(crate)` 與 `pub(super)` 把東西只開放到剛好的範圍，再用 `pub use` 整理 library 對外的路徑。

## 正文

### `pub` 有大小之分

第 4 集的 `pub` 是「對所有人開放」。如果你寫的是 library，這個「所有人」包括別的 crate：任何 `pub` 的東西，就是你對使用者的承諾，改動它可能弄壞別人的程式。有些函式只是內部的小幫手，不想讓外面用到，就要比 `pub` 更小的開放範圍：

| 寫法 | 誰看得到 |
| --- | --- |
| 什麼都不寫 | 定義它的模組，與裡面的子模組 |
| `pub(super)` | 上面再加上上一層模組 |
| `pub(crate)` | 整個 crate 裡的每個地方，crate 外面看不到 |
| `pub` | 全部，包括別的 crate |

### `pub(crate)`：crate 裡面通用，外面看不到

`lib.rs` 裡有兩個函式：`total_for` 給使用者，`price_of` 是內部幫手：

`src/lib.rs`：

```rust,ignore
pub fn total_for(drink: &str, cups: u32) -> u32 {
    price_of(drink) * cups
}

pub(crate) fn price_of(drink: &str) -> u32 {
    if drink == "紅茶" { 30 } else { 45 }
}
```

`price_of` 在 `lib.rs` 自己的 crate 裡到處都能用；但 `main.rs` 是另一個 crate，用不到：

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::total_for("紅茶", 2));
    println!("{}", teashop::price_of("紅茶"));
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
error[E0603]: function `price_of` is private
 --> src/main.rs:3:29
  |
3 |     println!("{}", teashop::price_of("紅茶"));
  |                             ^^^^^^^^ private function
  |
note: the function `price_of` is defined here
 --> src/lib.rs:5:1
  |
5 | pub(crate) fn price_of(drink: &str) -> u32 {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

錯誤碼跟第 4 集一樣是 E0603，訊息說 `price_of` 是私有的：對 `main.rs` 來說，`pub(crate)` 跟沒寫沒有差別。把第二行 `println!` 刪掉就能編譯。

在只有 `main.rs` 的 binary 裡，沒有別的 crate，`pub` 與 `pub(crate)` 的效果一樣；差別只在 library 才看得出來。另外 library 裡的 `pub` 函式就算沒人呼叫也不會有 `dead_code` 警告（使用者可能會用），`pub(crate)` 沒被用到就會有，這剛好幫你找出沒用的內部函式。

### `pub(super)`：只開放到上一層

`pub(super)` 把範圍限在上一層模組。下面 `tax` 只有 `shop` 能用，連 `main` 都看不到：

```rust
mod shop {
    pub mod bill {
        pub(super) fn tax(price: u32) -> u32 {
            price / 10
        }
    }

    pub fn total(price: u32) -> u32 {
        price + bill::tax(price)
    }
}

fn main() {
    println!("{}", shop::total(100));
}
```

執行結果：

```text
110
```

`main` 若寫 `shop::bill::tax(100)`，會得到 E0603。還有更精確的 `pub(in crate::shop)`，指定哪個模組以內可見；`pub(super)`、`pub(crate)` 都是它的簡寫，實務上很少需要自己寫。

### `pub use`：整理對外的路徑

library 的內部怎麼分模組，使用者不需要知道。把模組藏起來（不寫 `pub`），再用 `pub use` 把想開放的名字**重新公開**到更短的路徑：

`src/lib.rs`：

```rust,ignore
mod menu {
    pub fn price(drink: &str) -> u32 {
        if drink == "紅茶" { 30 } else { 45 }
    }
}

pub use menu::price;
```

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::price("紅茶"));
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
30
```

`use menu::price;` 只讓目前的模組用得到 `price`；前面多加 `pub`，這個名字就連同路徑一起對外公開，使用者寫 `teashop::price` 就好。`menu` 本身沒有 `pub`，使用者寫 `teashop::menu::price` 會是 E0603（`module menu is private`），之後想把 `price` 搬到別的模組，只要改 `pub use` 那一行，使用者的程式不用動。

## 重點整理

- 開放範圍由小到大：不寫、`pub(super)`、`pub(crate)`、`pub`；`pub(crate)` 是整個 crate 內到處能用，別的 crate 看不到（E0603）。
- library 裡的 `pub` 是對使用者的承諾；內部幫手用 `pub(crate)`，沒被用到時還會有 `dead_code` 警告。
- `pub(super)` 只開放到上一層模組；更細的 `pub(in 路徑)` 很少用。
- `pub use 路徑;` 把名字重新公開到目前的位置，能藏起內部的模組結構，讓使用者用比較短的路徑。
