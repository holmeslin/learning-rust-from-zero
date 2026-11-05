# `HashMap` 基礎

## 本集目標

用 `HashMap<K, V>` 存「鍵對應值」的資料：會建立、`insert`、用 `len` 看數量，並知道同一個鍵再 `insert` 會覆蓋舊值。

## 正文

### 先 `use`，再建立

`HashMap` 對應 Go 的 `map`。它不在預設就看得到的名字裡，使用前要先加一行 `use`，把它引進來。這行請先照抄，`use` 第 12 章第 5 集才會解釋；`HashSet`、`BTreeMap` 之後也一樣。

```rust
use std::collections::HashMap;

fn main() {
    let mut prices: HashMap<String, u32> = HashMap::new();
    prices.insert(String::from("紅茶"), 30);
    println!("{:?}", prices);
    prices.insert(String::from("咖啡"), 45);
    println!("{}", prices.len());
}
```

執行結果：

```text
{"紅茶": 30}
2
```

- `HashMap<String, u32>` 讀成「鍵是 `String`、值是 `u32` 的 `HashMap`」。尖括號裡的兩個型別是泛型（第 8 章），先把整個當成一個型別的名字。
- `HashMap::new()` 建立空的，`insert(鍵, 值)` 放進一組。鍵和值都是 move 進去的，所以 `String` 放進去之後，原本那個變數就不能再用了（第 3 章）。
- `{:?}` 印出來是 `{"紅茶": 30}`。這裡刻意只印一組：`HashMap` 裡的順序不固定，放了好幾組之後印出來的順序會變，第 14 集再說。
- 沒有 map 的字面值寫法，先用 `insert` 逐組放進去。

### 同一個鍵再放，會蓋掉舊值

`insert` 會回傳舊的值，包在 `Option` 裡：原本沒有這個鍵是 `None`，有的話是 `Some(舊值)`。

```rust
use std::collections::HashMap;

fn main() {
    let mut prices: HashMap<String, u32> = HashMap::new();
    println!("{:?}", prices.insert(String::from("紅茶"), 30));
    println!("{:?}", prices.insert(String::from("紅茶"), 35));
    println!("{:?}", prices);
}
```

執行結果：

```text
None
Some(30)
{"紅茶": 35}
```

一個鍵只會有一個值。第二次 `insert` 把 30 換成 35，並把舊的 30 還給你；不需要舊值的話，不接住回傳值就好。

### 查有沒有、刪掉

`contains_key` 問有沒有這個鍵，`remove` 刪掉一組並回傳舊值（同樣是 `Option`）。兩個方法的參數都是參照，鍵是 `String` 時直接傳 `&str` 就行：

```rust
use std::collections::HashMap;

fn main() {
    let mut prices: HashMap<String, u32> = HashMap::new();
    prices.insert(String::from("紅茶"), 30);
    prices.insert(String::from("咖啡"), 45);
    println!("{}", prices.contains_key("咖啡"));
    println!("{:?}", prices.remove("咖啡"));
    println!("{}", prices.contains_key("咖啡"));
    println!("{}", prices.len());
}
```

執行結果：

```text
true
Some(45)
false
1
```

### 什麼型別能當鍵

鍵的型別要能比較相等、能算雜湊值：整數、`bool`、`char`、`String`、`&str` 都可以，浮點數 `f64` 不行。這個條件在第 8 章的 trait 才會講清楚，現在遇到再回頭查就好。

## 重點整理

- 使用前先寫 `use std::collections::HashMap;`（先照抄，`use` 第 12 章解釋）。
- `HashMap<K, V>` 是鍵對應值；`HashMap::new()` 建立空的，`insert(鍵, 值)` 放入，`len()` 是組數。
- `insert` 會把鍵和值 move 進去；同一個鍵再 `insert` 會覆蓋舊值，並回傳 `Option`（`None` 或 `Some(舊值)`）。
- `contains_key(&鍵)` 問有沒有，`remove(&鍵)` 刪除並回傳 `Option`；鍵是 `String` 時可以傳 `&str`。
- `HashMap` 裡的順序不固定，所以這本書印整個 `HashMap` 時只放一組。
