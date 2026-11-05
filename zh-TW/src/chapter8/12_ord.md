# `PartialOrd` 與 `Ord`

## 本集目標

用 `derive` 讓型別能用 `<`、`>` 比大小，並知道 `sort` 與 `BTreeMap` 的鍵要的是 `Ord`；說得出 `f64` 為什麼只有 `PartialOrd`。

## 正文

### `derive` 之後就能比大小

`<`、`>`、`<=`、`>=` 背後是 `PartialOrd`，完整的排序是 `Ord`。`PartialOrd` 要同時有 `PartialEq`，`Ord` 要同時有 `Eq` 和 `PartialOrd`，所以四個一起 `derive`：

```rust
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    points: u32,
    name: String,
}

fn main() {
    println!("{}", Size::Small < Size::Large);
    println!("{:?}", Size::Medium.cmp(&Size::Small));

    let mut board = vec![
        Entry {
            points: 95,
            name: String::from("小明"),
        },
        Entry {
            points: 80,
            name: String::from("小華"),
        },
        Entry {
            points: 80,
            name: String::from("小美"),
        },
    ];
    board.sort();
    for entry in &board {
        println!("{} {}", entry.points, entry.name);
    }

    let mut stock: BTreeMap<Size, u32> = BTreeMap::new();
    stock.insert(Size::Large, 3);
    stock.insert(Size::Small, 8);
    println!("{:?}", stock);
}
```

執行結果：

```text
true
Greater
80 小美
80 小華
95 小明
{Small: 8, Large: 3}
```

- 比的順序由 `derive` 決定：enum 照 variant 宣告的順序，`Small < Medium < Large`；struct 照欄位由上到下，先比 `points`，一樣才比 `name`，字串依編碼大小（第 7 章第 17 集）。
- `cmp` 回傳 `Ordering`，是個 enum，三個 variant：`Less`、`Equal`、`Greater`，用 `{:?}` 就能印。
- `Vec` 的 `sort` 要元素有 `Ord`。第 7 章第 17 集只能排整數和字串，現在自己的型別也行：小美、小華同分，就依名字排，小美在小華前面。
- `BTreeMap` 的鍵要有 `Ord`（第 7 章第 16 集），所以 `stock` 是依 `Size` 的順序排好的：`Small` 在 `Large` 前面。

### 為什麼還有一個 `Partial`

浮點數有個特殊值 NaN，它跟任何數比大小都沒有答案。所以 `f64` 的比較可能「比不出來」：

```rust
fn main() {
    println!("{:?}", 1.5.partial_cmp(&2.5));
    println!("{:?}", f64::NAN.partial_cmp(&1.0));
}
```

執行結果：

```text
Some(Less)
None
```

`PartialOrd` 的 `partial_cmp` 回傳 `Option<Ordering>`：比得出來是 `Some(Less)`，比不出來是 `None`。`Partial` 就是「有些值可以比不出來」。`Ord` 的 `cmp` 保證任兩個值都比得出來，直接回傳 `Ordering`。

所以 `f64` 只有 `PartialOrd`，沒有 `Ord`：`Vec<f64>` 不能 `sort`（第 7 章第 17 集）、`f64` 也不能當 `BTreeMap` 的鍵。裡面有 `f64` 欄位的 struct 也一樣，最多只能 `derive` 到 `PartialOrd`：

```rust,compile_fail=E0277
#[derive(PartialEq, PartialOrd)]
struct Score {
    value: f64,
}

fn main() {
    let a = Score { value: 2.5 };
    let b = Score { value: 1.5 };
    println!("{}", a > b);
    let mut list = vec![a, b];
    list.sort();
    println!("{}", list.len());
}
```

編譯錯誤：

```text
error[E0277]: the trait bound `Score: Ord` is not satisfied
  --> src/main.rs:11:10
   |
11 |     list.sort();
   |          ^^^^ unsatisfied trait bound
   |
```

`a > b` 沒問題，`PartialOrd` 就夠了；`sort` 才要 `Ord`，所以是 E0277。這裡只貼前半段，後面還有 `help` 和 `note`。要排浮點數，得用 `sort_by` 自己說明怎麼比，那要用到閉包，第 11 章第 16 集再學。

## 重點整理

- `PartialOrd` 讓型別能用 `<`、`>`、`<=`、`>=`，`Ord` 是完整的排序；一起 `derive`：`PartialEq, Eq, PartialOrd, Ord`。
- `derive` 的比法：enum 照 variant 宣告順序，struct 照欄位由上到下。
- `cmp` 回傳 `Ordering`（`Less`、`Equal`、`Greater`）；`partial_cmp` 回傳 `Option<Ordering>`，NaN 比不出來是 `None`。
- `Vec::sort` 和 `BTreeMap` 的鍵要 `Ord`；`f64` 只有 `PartialOrd`，含 `f64` 的 struct 也不能 `sort`（E0277）。
