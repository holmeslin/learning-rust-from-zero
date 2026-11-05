# `sort` 與 `dedup`

## 本集目標

用 `sort` 把 `Vec` 排序、`reverse` 反轉、`dedup` 去掉連續的重複；知道要去掉所有重複得先 `sort`，以及 `f64` 的 `Vec` 不能直接 `sort`。

## 正文

### `sort` 與 `reverse`

`sort` 直接改動原本的 `Vec`，由小到大排，所以要 `let mut`，也沒有回傳值。`reverse` 同樣原地把順序反過來，兩個搭配就是由大到小：

```rust
fn main() {
    let mut prices = vec![45, 30, 38, 30];
    prices.sort();
    println!("{:?}", prices);
    prices.reverse();
    println!("{:?}", prices);

    let mut names = vec![
        String::from("紅茶"),
        String::from("咖啡"),
        String::from("奶茶"),
    ];
    names.sort();
    println!("{:?}", names);
}
```

執行結果：

```text
[30, 30, 38, 45]
[45, 38, 30, 30]
["咖啡", "奶茶", "紅茶"]
```

數字依大小排，字串依編碼的大小排（跟上一集 `BTreeMap` 的鍵一樣），所以「咖啡」排在最前面。

### `dedup`：只去掉連續的重複

`dedup` 把「跟前一個一樣」的元素刪掉。注意是**連續**的：不相鄰的重複它不管。要去掉所有重複，先 `sort` 讓相同的值排在一起，再 `dedup`：

```rust
fn main() {
    let mut orders = vec![30, 45, 30, 30, 45];
    orders.dedup();
    println!("{:?}", orders);
    orders.sort();
    orders.dedup();
    println!("{:?}", orders);
}
```

執行結果：

```text
[30, 45, 30, 45]
[30, 45]
```

第一次 `dedup` 只把中間連續的兩個 30 合成一個；排序之後，相同的值都靠在一起，第二次 `dedup` 才去得乾淨。

### 把 `HashMap` 的鍵排序之後走訪

第 14 集說過，要固定的走訪順序，可以把鍵收進 `Vec` 排序。先收集 `keys()`，`sort`，再依序走訪：

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert(String::from("紅茶"), 10);
    stock.insert(String::from("咖啡"), 4);
    stock.insert(String::from("奶茶"), 7);

    let mut names = Vec::new();
    for name in stock.keys() {
        names.push(name);
    }
    names.sort();
    for name in names {
        println!("{name}：{} 杯", stock[name]);
    }
}
```

執行結果：

```text
咖啡：4 杯
奶茶：7 杯
紅茶：10 杯
```

`names` 裡放的是借來的鍵，所以型別是 `Vec<&String>`。`stock[name]` 用第 12 集的 `map[鍵]` 讀值，這裡的鍵一定存在，所以安全。每次執行順序都一樣。

### `f64` 不能直接 `sort`

整數和字串都能 `sort`，浮點數的 `Vec` 卻不行：

```rust,compile_fail=E0277
fn main() {
    let mut scores: Vec<f64> = vec![2.5, 1.5, 3.0];
    scores.sort();
    println!("{:?}", scores);
}
```

編譯錯誤：

```text
error[E0277]: the trait bound `f64: Ord` is not satisfied
 --> src/main.rs:3:12
  |
3 |     scores.sort();
  |            ^^^^ the trait `Ord` is not implemented for `f64`
  |
```

`` the trait bound `f64: Ord` is not satisfied ``：`f64` 沒辦法完整地比大小。浮點數有個特殊值 NaN（不是數字），它跟任何數字比大小都沒有答案，所以排序的規則無法成立。訊息後面還有 `help` 和 `note`，不用細讀。要排浮點數得自己說明怎麼比，用 `sort_by`，那要用到閉包，第 11 章第 16 集再學。

### 跟 Go 比一比

Go 的 `sort.Float64s` 可以直接排浮點數，遇到 NaN 就把它排在最前面：

```go
s := []float64{2.5, 1.5, 3.0}
sort.Float64s(s) // [1.5 2.5 3]，Go 替你決定了 NaN 怎麼處理
```

Rust 不替你決定：NaN 該排哪裡，要寫出來。另外 Go 的 `slices.Compact` 跟 Rust 的 `dedup` 一樣，也只去掉相鄰的重複。

## 重點整理

- `v.sort()` 原地由小到大排序，`v` 要 `let mut`；`v.reverse()` 原地反轉，搭配起來是由大到小。
- 字串依編碼大小排序，中文字不是依筆畫或注音。
- `v.dedup()` 只去掉連續的重複；要去掉全部重複，先 `sort` 再 `dedup`。
- 要固定走訪 `HashMap` 的順序：收集 `keys()` 到 `Vec`、`sort`、再依序走訪，或直接用 `BTreeMap`。
- `Vec<f64>` 不能 `sort`（E0277，因為 NaN）；要用 `sort_by`，第 11 章學。
