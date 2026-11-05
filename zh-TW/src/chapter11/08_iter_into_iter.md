# `iter`、`iter_mut`、`into_iter`（揭曉 `for`）

## 本集目標

分清楚 `iter()`、`iter_mut()`、`into_iter()` 各自拿到什麼，知道 `for` 迴圈其實是在反覆呼叫 `next()`，並看懂前面所有「先照抄」的 `for` 為什麼能運作。

## 正文

### 三個方法，三種拿法

`Vec` 有三個方法能得到迭代器，差別跟第 7 章第 4 集的三種 `for` 一樣：

```rust
fn main() {
    let mut names = Vec::new();
    names.push(String::from("小明"));
    names.push(String::from("小華"));

    for name in names.iter() {
        println!("{name}");
    }
    for name in names.iter_mut() {
        name.push_str("同學");
    }
    println!("{names:?}");
    for name in names.into_iter() {
        println!("拿到 {name}");
    }
}
```

執行結果：

```text
小明
小華
["小明同學", "小華同學"]
拿到 小明同學
拿到 小華同學
```

| 方法 | 每次拿到 | `for` 的簡寫 | 走完之後的 `Vec` |
| --- | --- | --- | --- |
| `iter()` | `&T`，只能讀 | `for x in &v` | 還在 |
| `iter_mut()` | `&mut T`，可以改 | `for x in &mut v` | 還在 |
| `into_iter()` | `T`，元素本身 | `for x in v` | 已經 move |

`into_iter` 的參數是 `self`，所以呼叫之後 `names` 就不能再用，這也是第 7 章第 4 集 E0382 訊息裡提到 `into_iter` 的原因。

### 揭曉 `for`

`for x in 東西 { ... }` 其實是先呼叫 `東西.into_iter()` 得到迭代器，再反覆呼叫 `next()`，直到 `None`。下面兩個迴圈做的事完全一樣：

```rust
fn main() {
    let prices = [30, 45, 38];
    for price in prices {
        println!("{price}");
    }

    let mut it = prices.into_iter();
    while let Some(price) = it.next() {
        println!("{price}");
    }
}
```

執行結果：

```text
30
45
38
30
45
38
```

能放在 `for` 後面的東西，都實作了 `IntoIterator` trait，它的方法就是 `into_iter()`。`Vec<T>`、`&Vec<T>`、`&mut Vec<T>` 是三個不同的型別，各自實作了 `IntoIterator`，所以 `for x in v`、`for x in &v`、`for x in &mut v` 呼叫到的 `into_iter()` 不同，拿到的是 `T`、`&T`、`&mut T`；迭代器本身也實作了 `IntoIterator`（回傳自己），所以 `for x in v.iter()` 也行。

### 之前先照抄的 `for`

現在可以把前面幾章留下的疑問一次解開：

- `for x in 陣列`（第 2 章第 13 集）：陣列實作了 `IntoIterator`，每圈拿到元素本身。
- `for c in s.chars()`、`for part in s.split(',')`、`for k in map.keys()`（第 7 章）：這些方法回傳的就是迭代器。
- `for i in 0..n`、`0..=n`（第 1 章第 14 集）：範圍 `0..n` 本身就是迭代器，可以直接呼叫 `next()`：

```rust
fn main() {
    let mut turns = 0..3;
    println!("{:?}", turns.next());
    println!("{:?}", turns.next());
    for i in turns {
        println!("剩下 {i}");
    }
}
```

執行結果：

```text
Some(0)
Some(1)
剩下 2
```

`turns` 記著走到哪裡：前兩次 `next()` 拿走 0 和 1，`for` 接手時只剩下 2。

## 重點整理

- `iter()` 拿 `&T`、`iter_mut()` 拿 `&mut T`、`into_iter()` 拿 `T` 並 move 掉整個集合；分別對應 `for x in &v`、`&mut v`、`v`。
- `for x in 東西 { ... }` 就是呼叫 `東西.into_iter()`，再反覆 `next()` 到 `None`，也就是第 7 集的 `while let`。
- `for` 後面能放任何實作了 `IntoIterator` 的東西：集合、`&集合`、`&mut 集合`、迭代器本身。
- 前面先照抄的 `for` 都是這回事：陣列、`chars()`、`split()`、`keys()` 與範圍 `0..n`，每一個都是（或能變成）迭代器。
