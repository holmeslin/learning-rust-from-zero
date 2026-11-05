# `sum`、`count`、`min`、`max`

## 本集目標

用 `sum`、`count`、`min`、`max` 這幾個 consumer 算出總和、個數、最小與最大值；知道 `sum` 要標型別、`min` 與 `max` 回傳 `Option`，而且浮點數不能直接用 `min`、`max`。

## 正文

### 四個 consumer

四個方法都會把整個迭代器走完，只留下一個結果：

```rust
fn main() {
    let prices = [30, 45, 38];
    let total: i32 = prices.iter().sum();
    let expensive = prices.iter().filter(|p| **p >= 40).count();
    println!("共 {total} 元");
    println!("{expensive} 杯超過 40 元");
    println!("{:?}", prices.iter().min());
    println!("{:?}", prices.iter().max());
}
```

執行結果：

```text
共 113 元
1 杯超過 40 元
Some(30)
Some(45)
```

- `sum()` 把元素加起來。跟 `collect` 一樣，它能加出好幾種型別的結果，所以要標型別：`let total: i32 = ...sum();` 或 `sum::<i32>()`，沒標是 E0283（`type annotations needed`）。連乘用 `product()`，用法相同。
- `count()` 回傳元素的個數（`usize`）。通常接在 `filter` 後面，數有幾個符合；單純數陣列或 `Vec` 的長度，用 `len()` 就好。
- `min()`、`max()` 回傳最小、最大的元素，可是包在 `Option` 裡，下一段說明。

### `min`、`max` 回傳 `Option`

空的迭代器沒有最大值，所以回傳 `Option`：有元素是 `Some`，沒有是 `None`。用第 6 章的 `match` 或 `if let` 處理：

```rust
fn main() {
    let menu = [30, 45, 38];
    let empty: Vec<i32> = Vec::new();

    if let Some(price) = menu.iter().max() {
        println!("最貴 {price} 元");
    }
    match empty.iter().max() {
        Some(price) => println!("最貴 {price} 元"),
        None => println!("沒有任何飲料"),
    }
}
```

執行結果：

```text
最貴 45 元
沒有任何飲料
```

確定一定有元素時，可以用第 9 章的 `.expect("菜單不會是空的")`。

### `chars().count()`：數字元

第 7 章第 9 集用累加變數數字元，現在一行就好。`len()` 是位元組數，`chars().count()` 才是字元數：

```rust
fn main() {
    let name = String::from("紅茶拿鐵");
    println!("{}", name.len());
    println!("{}", name.chars().count());
}
```

執行結果：

```text
12
4
```

每個中文字佔 3 個位元組，四個字就是 12 個位元組。

### 浮點數不能用 `min`、`max`

`max` 要能比出大小的元素，也就是 `Ord`（第 8 章第 12 集）。`f64` 因為 NaN 只有 `PartialOrd`，第 7 章第 17 集的 `sort` 也是這個問題：

```rust,compile_fail=E0277
fn main() {
    let scores: [f64; 3] = [2.5, 1.5, 3.0];
    println!("{:?}", scores.iter().max());
}
```

編譯錯誤：

```text
error[E0277]: the trait bound `f64: Ord` is not satisfied
 --> src/main.rs:3:36
  |
3 |     println!("{:?}", scores.iter().max());
  |                                    ^^^ the trait `Ord` is not implemented for `f64`
```

訊息是 `` the trait bound `f64: Ord` is not satisfied ``，後面還有 `help` 與 `note`，不用細讀。浮點數的最大值可以用第 14 集的 `fold` 算；排序則是第 16 集的 `sort_by`。`sum` 對浮點數沒問題：`let total: f64 = scores.iter().sum();`。

### 跟 Go 比一比

Go 1.21 起有 `slices.Max`、`slices.Min`，遇到空的 slice 直接 panic：

```go
prices := []int{}
fmt.Println(slices.Max(prices)) // panic: slices.Max: empty list
```

Rust 的 `max()` 把「可能沒有」放進型別裡，回傳 `Option`，不處理就編不過；Go 要你自己記得先檢查長度。

## 重點整理

- `sum()`、`product()` 把元素加、乘起來；`sum` 要標型別（`let total: i32 = ...` 或 `sum::<i32>()`），沒標是 E0283。
- `count()` 回傳元素個數（`usize`），常接在 `filter` 後面；`s.chars().count()` 是字元數，`s.len()` 是位元組數。
- `min()`、`max()` 回傳 `Option`，空的迭代器是 `None`，用 `match`、`if let` 處理。
- 浮點數只有 `PartialOrd`，不能用 `min`、`max`（E0277），要改用 `fold`（第 14 集）；`sum` 浮點數沒問題。
- 跟 Go 比：`slices.Max` 遇到空 slice 會 panic；Rust 的 `max()` 回傳 `Option`。
