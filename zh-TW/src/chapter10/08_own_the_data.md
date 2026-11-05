# 常見錯誤：改成擁有資料就好

## 本集目標

認得生命週期最常見的兩種錯誤：回傳函式裡建立的值的參照、struct 或 `Vec` 裡的參照比資料活得久；知道多數時候的修法是改成擁有資料，以及什麼時候該借、什麼時候該擁有。

## 正文

### 症狀一：回傳函式裡建立的值

函式裡用 `format!` 組出一段新文字，想把它回傳：

```rust,compile_fail=E0515
fn make_label(name: &str) -> &str {
    let label = format!("[{name}]");
    &label
}

fn main() {
    let label = make_label("小美");
    println!("{label}");
}
```

編譯錯誤：

```text
error[E0515]: cannot return reference to local variable `label`
 --> src/main.rs:3:5
  |
3 |     &label
  |     ^^^^^^ returns a reference to data owned by the current function
```

`label` 是函式裡剛建立的 `String`，`}` 就 drop。第 2 集的標註、第 7 集的 `'static` 都救不了它：參照指向的資料根本活不到函式外面。修法是不要借，直接把 `String` 交出去，所有權交給呼叫端（第 3 章第 10 集）：

```rust
fn make_label(name: &str) -> String {
    format!("[{name}]")
}

fn main() {
    let label = make_label("小美");
    println!("{label}");
}
```

執行結果：

```text
[小美]
```

### 症狀二：struct 或 `Vec` 裡的參照，比資料活得久

每一圈迴圈都建立一個新的 `String`，用參照存進外面的 `Vec`：

```rust,compile_fail=E0597
struct Receipt<'a> {
    customer: &'a str,
    price: u32,
}

fn main() {
    let mut receipts = Vec::new();
    for (n, price) in [(1, 30), (2, 45)] {
        let customer = format!("{n} 號客人");
        receipts.push(Receipt {
            customer: &customer,
            price,
        });
    }
    for r in &receipts {
        println!("{} {}", r.customer, r.price);
    }
}
```

編譯錯誤：

```text
error[E0597]: `customer` does not live long enough
  --> src/main.rs:11:23
   |
 9 |         let customer = format!("{n} 號客人");
   |             -------- binding `customer` declared here
10 |         receipts.push(Receipt {
11 |             customer: &customer,
   |                       ^^^^^^^^^ borrowed value does not live long enough
...
14 |     }
   |     - `customer` dropped here while still borrowed
15 |     for r in &receipts {
   |              --------- borrow later used here
```

`customer` 是這一圈才建立的，圈結束就 drop；`receipts` 活在迴圈外面，還想留著指向它的參照。修法一樣：struct 自己擁有資料，欄位改成 `String`，`customer` 的所有權直接搬進去：

```rust
struct Receipt {
    customer: String,
    price: u32,
}

fn main() {
    let mut receipts = Vec::new();
    for (n, price) in [(1, 30), (2, 45)] {
        let customer = format!("{n} 號客人");
        receipts.push(Receipt { customer, price });
    }
    for r in &receipts {
        println!("{} {}", r.customer, r.price);
    }
}
```

執行結果：

```text
1 號客人 30
2 號客人 45
```

這就是第 5 章一開始就要求欄位放 `String` 的原因：struct 擁有自己的資料，想放進哪裡、傳到哪裡都行，不用管資料來自誰。

### 怎麼選：借，還是擁有

- 要存起來、放進 `Vec` 或別的 struct、回傳給呼叫端、活得比原本的資料久，或資料是函式、迴圈裡剛建好的：**擁有**，用 `String`。
- 只在函式裡看一眼就丟的參數：**借**，用 `&str`。
- struct 只是暫時指向別人給的文字、一定比那份文字先結束（像第 5 集的 `parse_receipt`）：借也可以，但要自己標生命週期。

預設從擁有開始，真的遇到「不想複製」的理由，再改成借。複製一份 `String` 的成本，多數程式根本感覺不到。

編譯器報生命週期錯誤時，不要急著加 `'static`、到處加 `<'a>`，也不要看到錯誤就 `.clone()`。先找出是哪個參照，比哪份資料活得久，再決定要借還是擁有。這一章的錯誤對照如下：

| 症狀 | 錯誤 | 做法 |
| --- | --- | --- |
| 兩個以上參照參數、回傳參照，沒標註 | E0106 | 標 `'a`（第 2、3 集），或回傳 `String` |
| 回傳函式裡建立的值的參照 | E0515 | 回傳擁有的值 |
| struct、`Vec` 裡的參照比資料活得久 | E0597 | 欄位改成 `String` |
| 回傳了簽名沒綁進 `'a` 的參數 | E0621 | 把它也綁進 `'a`，或改函式本體（第 3 集） |
| 結果用在輸入已經結束之後 | E0597 | 縮短使用的範圍，或改成擁有 |
| 方法回傳的參照活得不夠久 | E0597 | 回傳型別標 `'a`（第 6 集） |
| `impl Receipt` 沒帶生命週期 | E0726 | 補 `<'_>` 或 `impl<'a>` |
| 對區域變數的參照標 `'static` | E0515 | 不要這樣做（第 7 集） |

## 重點整理

- 回傳函式裡建立的值的參照，是 E0515；標 `'a` 或 `'static` 都沒用，改成回傳擁有的值（`String`）。
- struct 或 `Vec` 裡的參照比資料活得久，是 E0597；改成讓 struct 擁有資料，欄位用 `String`。
- 要存起來、回傳、活得比來源久的資料就擁有；只是在函式裡暫時看一眼的才借。預設從擁有開始。
- 不要用 `'static`、到處加 `<'a>` 或一律 `.clone()` 蓋掉錯誤；先弄清楚是哪個參照比哪份資料活得久。
- 生命週期的錯誤碼：E0106（沒說借自誰）、E0515（回傳了區域資料）、E0597（活得不夠久）、E0621（回傳了沒綁定的參數）、E0726（`impl` 缺生命週期）。
