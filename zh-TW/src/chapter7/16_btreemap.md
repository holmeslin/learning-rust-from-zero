# `BTreeMap`（有序）

## 本集目標

知道 `BTreeMap<K, V>` 跟 `HashMap` 用法幾乎一樣，差別是它永遠依鍵排序，所以印出來、走訪的順序都是固定的；並會用 `first_key_value`、`last_key_value`。

## 正文

### 用法跟 `HashMap` 一樣

`BTreeMap` 同樣要先 `use`，`new`、`insert`、`get`、`entry`、走訪的寫法都跟 `HashMap` 相同。下面把第 13 集的計數搬過來，只換了型別，就能印出整個 map，而且每次順序都一樣：

```rust
use std::collections::BTreeMap;

fn main() {
    let orders = ["紅茶", "咖啡", "紅茶", "紅茶", "咖啡"];
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    for order in orders {
        *counts.entry(String::from(order)).or_insert(0) += 1;
    }
    println!("{:?}", counts);
    for (name, count) in &counts {
        println!("{name}：{count} 杯");
    }
}
```

執行結果：

```text
{"咖啡": 2, "紅茶": 3}
咖啡：2 杯
紅茶：3 杯
```

- `{:?}` 印出 `{"咖啡": 2, "紅茶": 3}`，走訪也是同樣的順序：依鍵由小到大。
- 中文字是依 Unicode 編碼的大小排，不是依筆畫也不是依注音，所以「咖啡」排在「紅茶」前面。需要依注音或筆畫排序，標準函式庫沒有現成的做法。
- 鍵的型別要能比大小（整數、`char`、`String`、`&str` 都可以，`f64` 不行）。這個條件在第 8 章的 trait 才會講清楚。

### 最小與最大的鍵

因為鍵是排好的，要找最小或最大的鍵很快。`first_key_value()` 和 `last_key_value()` 回傳 `Option<(&K, &V)>`：空的 map 是 `None`，否則是 `Some((鍵, 值))`。

```rust
use std::collections::BTreeMap;

fn main() {
    let mut menu: BTreeMap<u32, String> = BTreeMap::new();
    menu.insert(45, String::from("咖啡"));
    menu.insert(30, String::from("紅茶"));
    menu.insert(38, String::from("奶茶"));
    println!("{:?}", menu);
    match menu.first_key_value() {
        Some((price, name)) => println!("最便宜：{name} {price} 元"),
        None => println!("沒有品項"),
    }
    match menu.last_key_value() {
        Some((price, name)) => println!("最貴：{name} {price} 元"),
        None => println!("沒有品項"),
    }
}
```

執行結果：

```text
{30: "紅茶", 38: "奶茶", 45: "咖啡"}
最便宜：紅茶 30 元
最貴：咖啡 45 元
```

這次鍵是價錢，放入的順序是 45、30、38，印出來是由小到大的 30、38、45。`Some((price, name))` 是第 6 章的模式，裡面套了 tuple 的模式。

### 怎麼選

- 只是存取、不在意順序：`HashMap`，一般來說比較快。
- 需要固定的順序、要依鍵由小到大走訪、或要找最小最大的鍵：`BTreeMap`。

Go 內建的 map 不排序，要有序就得自己把鍵取出來排。Rust 的標準函式庫兩種都有。

## 重點整理

- `BTreeMap<K, V>` 用法跟 `HashMap` 相同（`insert`、`get`、`entry`、走訪），使用前先 `use std::collections::BTreeMap;`。
- 它永遠依鍵由小到大排序，所以 `{:?}` 與走訪的順序是固定的；中文字依 Unicode 編碼排。
- `first_key_value()`、`last_key_value()` 回傳 `Option<(&K, &V)>`，找最小與最大的鍵。
- 鍵的型別要能比大小（`f64` 不行）。
- 不在意順序用 `HashMap`；需要排序用 `BTreeMap`。
