# 帶資料的 variant

## 本集目標

讓 variant 各自帶著資料，認得 tuple 式和 struct 式兩種寫法，並知道這些資料要靠下一集的 `match` 才取得出來。

## 正文

### 每一種可能帶的資料不一樣

商品的折扣有三種：維持原價、打折（折 20%）、滿額才折（滿 1000 折 150）。每一種需要的資料都不同：打折要一個百分比，滿額折要門檻和折抵金額，原價什麼都不用。

用 struct 的話，只能把所有欄位都放進去，沒用到的欄位得塞個假值，欄位之間還可能互相矛盾，編譯器也看不出來。Rust 的做法是讓每個 variant 自己決定要帶什麼資料：

```rust,warn
#[derive(Debug)]
enum Discount {
    Regular,
    Percent(u32),
    Threshold { min: u32, off: u32 },
}

fn main() {
    let list = [
        Discount::Regular,
        Discount::Percent(20),
        Discount::Threshold {
            min: 1000,
            off: 150,
        },
    ];
    for discount in list {
        println!("{:?}", discount);
    }
}
```

編譯警告：

```text
warning: field `0` is never read
 --> src/main.rs:4:13
  |
4 |     Percent(u32),
  |     ------- ^^^
  |     |
  |     field in this variant
  |
  = note: `Discount` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default
help: consider changing the field to be of unit type to suppress this warning while preserving the field numbering, or remove the field
  |
4 -     Percent(u32),
4 +     Percent(()),
  |

warning: fields `min` and `off` are never read
 --> src/main.rs:5:17
  |
5 |     Threshold { min: u32, off: u32 },
  |     ---------   ^^^       ^^^
  |     |
  |     fields in this variant
  |
  = note: `Discount` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
```

執行結果：

```text
Regular
Percent(20)
Threshold { min: 1000, off: 150 }
```

先看三個 variant 的寫法，警告等一下再回頭看：

- `Regular`：不帶資料，跟上一集一樣。
- `Percent(u32)`：**tuple 式**。variant 名稱後面用小括號放型別，跟第 5 章的 tuple struct 一樣，可以放好幾個。建立時像呼叫函式：`Discount::Percent(20)`。
- `Threshold { min: u32, off: u32 }`：**struct 式**。大括號裡寫 `欄位名稱: 型別`，跟 struct 一樣。建立時也用欄位名稱：`Discount::Threshold { min: 1000, off: 150 }`。

三個 variant 帶的資料各不相同，但值的型別都是 `Discount`，所以放得進同一個陣列，也能傳給同一個函式的參數。variant 帶的型別會被檢查，`Discount::Percent("八折")` 跟傳錯參數型別一樣是 E0308。

### 放進去，卻讀不出來

資料放進 variant 之後，不能像 tuple struct 那樣用 `.0`，也不能像 struct 那樣用 `.off`：

```rust,compile_fail=E0609
enum Discount {
    Percent(u32),
    Threshold { min: u32, off: u32 },
}

fn main() {
    let a = Discount::Percent(20);
    println!("{}", a.0);
    let b = Discount::Threshold {
        min: 1000,
        off: 150,
    };
    println!("{}", b.off);
}
```

編譯錯誤：

```text
error[E0609]: no field `0` on type `Discount`
 --> src/main.rs:8:22
  |
8 |     println!("{}", a.0);
  |                      ^ unknown field

error[E0609]: no field `off` on type `Discount`
  --> src/main.rs:13:22
   |
13 |     println!("{}", b.off);
   |                      ^^^ unknown field
```

道理很簡單：`a` 的型別是 `Discount`，不一定是 `Percent`，也可能是 `Regular`，那時候根本沒有 `.0` 可以讀。編譯器不讓你假設它是哪一種。

要拿資料，得先確認「它是哪個 variant」，確認的同時再把資料取出來，這就是下一集 `match` 的工作。

回頭看前面的警告：``field `0` is never read``，意思是 `Percent` 裡的 `u32` 放進去了，卻沒有任何地方讀它；`min` 和 `off` 也一樣。`#[derive(Debug)]` 印出來的不算（第 5 章第 6 集）。下一集開始，每個 variant 的資料都會被 `match` 讀出來，警告就不會再出現。

## 重點整理

- variant 可以各自帶資料：不帶（`Regular`）、tuple 式（`Percent(u32)`）、struct 式（`Threshold { min: u32, off: u32 }`），同一個 enum 裡可以混著用。
- 建立值：`Discount::Percent(20)`、`Discount::Threshold { min: 1000, off: 150 }`；帶的資料型別會被檢查。
- 不同 variant 帶的資料不同，但都是同一個型別，能放進同一個陣列、同一個參數。
- 不能對 enum 的值用 `.0` 或 `.欄位名稱` 讀資料（E0609），因為編譯器不知道它是哪個 variant。
- 取出資料要用 `match`，下一集教；variant 的資料從來沒被讀過，rustc 會警告 `never read`。
