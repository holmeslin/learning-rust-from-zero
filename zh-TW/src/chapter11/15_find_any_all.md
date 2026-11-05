# `find`、`any`、`all`、`position`

## 本集目標

用 `find` 找第一個符合條件的元素、`position` 找它的位置、`any` 與 `all` 判斷有沒有或是不是全部符合；知道它們找到答案就停，而且閉包參數的層數各不相同。

## 正文

### `find` 與 `position`

兩個都是「找第一個符合條件的」：`find` 回傳元素，`position` 回傳元素的位置（`usize`）。找不到都是 `None`，所以用 `match` 或 `if let` 處理：

```rust
fn main() {
    let prices = [30, 45, 38];
    match prices.iter().find(|p| **p > 40) {
        Some(p) => println!("第一杯超過 40 元的是 {p} 元"),
        None => println!("沒有超過 40 元的"),
    }
    match prices.iter().position(|p| *p > 40) {
        Some(i) => println!("在第 {i} 個位置"),
        None => println!("沒有"),
    }
}
```

執行結果：

```text
第一杯超過 40 元的是 45 元
在第 1 個位置
```

注意閉包參數的層數不一樣：

| 方法 | 閉包拿到 | `iter()` 搭配數字時 |
| --- | --- | --- |
| `filter`、`find` | 元素的參照 | `&&i32`，寫 `**p` |
| `any`、`all`、`position` | 元素本身 | `&i32`，寫 `*p` |

`find` 跟 `filter` 一樣，只是要看一眼元素，不能拿走；`any`、`all`、`position` 拿到元素本身。元素是 `&str` 這類型別、用方法判斷時，編譯器自動解參照（第 5 章第 10 集），就不用管幾層：

```rust
fn main() {
    let names = ["小明", "阿華", "小美"];
    println!("{:?}", names.iter().find(|n| n.starts_with("小")));
    println!("{:?}", names.iter().position(|n| n.starts_with("阿")));
}
```

執行結果：

```text
Some("小明")
Some(1)
```

第 7 章第 10 集的字串 `find` 是在字串裡找子字串，回傳位置；迭代器的 `find` 是在元素裡找符合條件的那一個，回傳元素本身，兩個同名但不一樣。

### `any` 與 `all`

`any` 問「有沒有至少一個符合」，`all` 問「是不是全部都符合」，回傳 `bool`：

```rust
fn main() {
    let prices = [30, 45, 38];
    println!("{}", prices.iter().any(|p| *p > 40));
    println!("{}", prices.iter().all(|p| *p > 40));
    println!("{}", prices.iter().all(|p| *p > 20));
}
```

執行結果：

```text
true
false
true
```

### 找到就停

四個都會在答案確定時立刻停下來，不會把剩下的元素走完，這是第 9 集的惰性。`any` 遇到第一個 `true` 就停，`all` 遇到第一個 `false` 就停，`find` 與 `position` 找到就停：

```rust
fn main() {
    let prices = [30, 45, 38];
    let pricey = prices.iter().any(|p| {
        println!("檢查 {p}");
        *p > 40
    });
    println!("{pricey}");
}
```

執行結果：

```text
檢查 30
檢查 45
true
```

45 就是答案，38 沒有被檢查。想要「全部走一遍」的話，才用 `filter` 加 `count`。

### 跟 Go 比一比

Go 1.21 起，`slices.IndexFunc` 找位置、`slices.ContainsFunc` 判斷有沒有，找不到的時候 `IndexFunc` 回傳 -1：

```go
prices := []int{30, 45, 38}
i := slices.IndexFunc(prices, func(p int) bool { return p > 50 })
fmt.Println(i) // -1，忘了檢查就拿去當索引
```

Rust 的 `position` 回傳 `Option<usize>`，沒有 -1 這種魔術數字；不處理 `None` 就拿不到位置。

## 重點整理

- `find(|x| 條件)` 回傳第一個符合的元素、`position(|x| 條件)` 回傳它的位置，都是 `Option`，找不到是 `None`。
- `any(|x| 條件)` 有沒有至少一個符合、`all(|x| 條件)` 是不是全部符合，回傳 `bool`。
- 四個都在答案確定時停止，不會走完剩下的元素。
- 閉包參數：`find` 與 `filter` 拿到元素的參照（`iter()` 搭配數字是 `&&i32`，寫 `**p`）；`any`、`all`、`position` 拿到元素本身（`&i32`，寫 `*p`）。
- 跟 Go 比：Go 找不到回傳 -1；Rust 回傳 `Option`。迭代器的 `find` 與字串的 `find`（第 7 章）不一樣。
