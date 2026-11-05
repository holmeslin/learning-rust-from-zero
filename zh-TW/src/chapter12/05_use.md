# `use` 與路徑

## 本集目標

看懂並寫出 `use`：用它把長路徑縮短，認得 `crate`、`self`、`super` 這些路徑的起點，也知道為什麼 `String`、`Vec` 不用 `use`。

## 正文

### 揭曉：`use std::collections::HashMap;`

第 7 章第 11 集要你先照抄這一行。沒有它會怎樣？

```rust,compile_fail=E0433
fn main() {
    let mut stock = HashMap::new();
    stock.insert("紅茶", 10);
    println!("{}", stock.len());
}
```

編譯錯誤：

```text
error[E0433]: cannot find type `HashMap` in this scope
 --> src/main.rs:2:21
  |
2 |     let mut stock = HashMap::new();
  |                     ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |
```

`HashMap` 不在預設就看得到的名字裡，編譯器找不到，`help` 直接寫出該加哪一行。這行 `use` 的意思是：**把路徑最後一段的名字，帶進目前的範圍，之後只寫名字就好**。

`use` 只是縮寫，不是「一定要匯入才能用」。完整路徑隨時可以寫，第 8 章起一直出現的 `std::fmt::Display` 就是完整路徑：

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock.insert("紅茶", 10);

    let mut seen = std::collections::HashSet::new();
    seen.insert("奶茶");

    println!("{:?} {}", stock.get("紅茶"), seen.len());
}
```

執行結果：

```text
Some(10) 1
```

`HashMap` 用 `use` 引進來，`HashSet` 這次直接寫完整路徑，兩種都行。這跟 Go 的 `import` 不同：Go 沒 `import` 就不能用，Rust 的 `use` 只是讓你少打字。

### 路徑從哪裡開始

路徑的第一段，決定它是絕對路徑還是相對路徑：

| 寫法 | 從哪裡開始 |
| --- | --- |
| `crate::menu::price` | 目前這個 crate 的根（絕對路徑） |
| `std::fmt::Display` | 另一個 crate，這裡是標準函式庫 `std`（絕對路徑） |
| `self::price` | 目前所在的模組（相對路徑） |
| `super::menu::price` | 目前模組的上一層（相對路徑） |

`use` 也可以搭配這些路徑，而且**寫在哪個模組裡，名字就只在那個模組裡看得到**。這就解決了第 3 集的 E0433：

```rust
mod menu {
    pub fn price(drink: &str) -> u32 {
        if drink == "紅茶" { 30 } else { 45 }
    }
}

mod bill {
    use super::menu::price;

    pub fn total_for(drink: &str, cups: u32) -> u32 {
        price(drink) * cups
    }
}

use bill::total_for;

fn main() {
    println!("{}", total_for("紅茶", 2));
}
```

執行結果：

```text
60
```

`bill` 裡的 `use super::menu::price;` 從上一層（根）找到 `menu`，再引進 `price`，所以 `total_for` 裡直接寫 `price(drink)`。最下面的 `use bill::total_for;` 寫在根，所以 `main` 看得到。換成 `use crate::menu::price;` 也一樣，效果相同。

`use` 的路徑寫錯（例如把 `collections` 拼成 `collection`）是 E0432，訊息會指出「在 `std` 裡找不到 `collection`」；沒用到的 `use` 跟沒用到的變數一樣，rustc 會警告 `unused_imports`。

### 幾種常用的寫法

同一個地方引進好幾個名字，用大括號。名字撞到了，用 `as` 改名。習慣上，引進**模組**再寫 `模組::名稱`，比直接引進函式更看得出來源。下面把第 8 章第 11 集的 `Display` 骨架縮短了：

```rust
use std::fmt;

struct Price(u32);

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} 元", self.0)
    }
}

fn main() {
    println!("{}", Price(30));
}
```

執行結果：

```text
30 元
```

`use std::fmt;` 之後，`fmt::Display`、`fmt::Formatter`、`fmt::Result` 都能短寫。這裡刻意引進 `fmt` 模組，而不是 `use std::fmt::Result;`：後者會蓋掉預設的 `Result`，同一個檔案裡其他地方的 `Result` 意思就變了。其他幾種寫法：

```rust,ignore
use std::collections::{HashMap, HashSet}; // 一次引進兩個
use std::fmt::Result as FmtResult; // 改名
use super::*; // * 引進那一層全部的東西
```

`*` 會把整層的名字都倒進來，不容易看出名字從哪來，平常少用；第 10 集寫測試時有個固定用法會用到它。

### 為什麼 `String` 不用 `use`

`String`、`Vec`、`Option`、`Some`、`None`、`Result`、`Box` 這些，編譯器替每個檔案預先引進了（叫 **prelude**），所以不用寫 `use`。`println!`、`vec!`、`format!` 這類巨集也一樣。`HashMap` 不在名單裡，所以要自己 `use`。

## 重點整理

- `use 路徑;` 把最後一段名字帶進目前範圍，之後只寫名字；它只是縮寫，完整路徑隨時可以寫。
- 路徑可以從 `crate`（根）、別的 crate（如 `std`）、`self`（目前模組）、`super`（上一層）開始。
- `use` 寫在哪個模組，名字就只在那個模組裡有效；路徑寫錯是 E0432，沒找到名字是 E0433。
- 常用寫法：`{A, B}` 一次引進多個、`as` 改名、`*` 引進整層（少用）；習慣引進模組再寫 `fmt::Display`。
- `String`、`Vec`、`Option`、`Result`、`Box` 等在 prelude 裡，不用 `use`。
