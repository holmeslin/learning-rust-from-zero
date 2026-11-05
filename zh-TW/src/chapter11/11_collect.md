# `collect`

## 本集目標

用 `collect()` 把迭代器剩下的元素收進 `Vec`、`String`、`HashMap` 或 `Result`；知道一定要讓編譯器知道收成什麼型別，以及 `filter` 之後要用 `copied()` 才會收到元素本身。

## 正文

### 收成 `Vec`

`collect()` 是最常用的 consumer：把迭代器的元素全部拉出來，放進一個新的集合。但集合有很多種，收成哪一種，編譯器沒辦法替你猜：

```rust,compile_fail=E0283
fn main() {
    let prices = [30, 45, 38];
    let doubled = prices.iter().map(|p| p * 2).collect();
    println!("{:?}", doubled);
}
```

編譯錯誤：

```text
error[E0283]: type annotations needed
 --> src/main.rs:3:9
  |
3 |     let doubled = prices.iter().map(|p| p * 2).collect();
  |         ^^^^^^^                                ------- type must be known at this point
  |
  = note: the type must implement `FromIterator<i32>`
```

`type annotations needed`：要替 `doubled` 標型別。只貼了前半段，後面的 `help` 建議寫成 `Vec<_>`。標型別有兩種寫法，效果一樣：

```rust
fn main() {
    let prices = [30, 45, 38];
    let doubled: Vec<i32> = prices.iter().map(|p| p * 2).collect();
    let tripled = prices.iter().map(|p| p * 3).collect::<Vec<i32>>();
    let plus: Vec<_> = prices.iter().map(|p| p + 5).collect();
    println!("{doubled:?}");
    println!("{tripled:?}");
    println!("{plus:?}");
}
```

執行結果：

```text
[60, 90, 76]
[90, 135, 114]
[35, 50, 43]
```

第二種 `collect::<Vec<i32>>()` 用的是第 8 章第 8 集的 `::<型別>`。第三種的 `_` 在型別位置的意思是「這裡請編譯器自己填」：元素是 `i32`，編譯器早就知道了，只需要說「收成 `Vec`」。

### `filter` 之後：`&i32` 與 `copied()`

`iter()` 給的是 `&i32`，`map(|p| p * 2)` 算出新的 `i32`，但 `filter` 只是篩選，元素原封不動，收進去的就是參照：

```rust
fn main() {
    let prices = [30, 45, 38];
    let big: Vec<&i32> = prices.iter().filter(|p| **p >= 40).collect();
    let owned: Vec<i32> = prices.iter().filter(|p| **p >= 40).copied().collect();
    println!("{big:?}");
    println!("{owned:?}");
}
```

執行結果：

```text
[45]
[45]
```

`copied()` 是一個 adapter，把每個 `&i32` 複製成 `i32`；元素要是 `Copy` 型別（整數、`char`、`bool`）才能用。元素像 `String` 這種非 `Copy` 的，用 `cloned()`，會呼叫 `.clone()`。兩者都印成 `[45]`，只有型別不同：收了參照的 `Vec<&i32>` 還借著 `prices`，收了 `i32` 的 `Vec<i32>` 就跟 `prices` 沒關係了。

### 收成 `String`、`HashMap`

收成什麼，看型別標註。元素是 `char` 就能收成 `String`；元素是 `(鍵, 值)` 的 tuple 就能收成 `HashMap`：

```rust
use std::collections::HashMap;

fn main() {
    let code = String::from("A12-紅茶-3");
    let digits: String = code.chars().filter(|c| c.is_ascii_digit()).collect();
    println!("{digits}");

    let menu = [("紅茶", 30), ("咖啡", 45), ("奶茶", 38)];
    let prices: HashMap<_, _> = menu.into_iter().collect();
    println!("{:?}", prices.get("咖啡"));
}
```

執行結果：

```text
123
Some(45)
```

`is_ascii_digit` 判斷字元是不是 `0` 到 `9`。`HashMap<_, _>` 一樣讓編譯器填，鍵是 `&str`、值是 `i32`。

### 收成 `Result`

迭代器的元素如果是 `Result`，可以收成 `Result<Vec<_>, _>`：全部成功就得到 `Ok(Vec)`，只要有一個失敗，就停下來回傳第一個 `Err`：

```rust
fn main() {
    let good: Result<Vec<u32>, _> = ["30", "45"].iter().map(|s| s.parse::<u32>()).collect();
    let bad: Result<Vec<u32>, _> = ["30", "四十五"].iter().map(|s| s.parse::<u32>()).collect();
    println!("{good:?}");
    println!("{bad:?}");
}
```

執行結果：

```text
Ok([30, 45])
Err(ParseIntError { kind: InvalidDigit })
```

一次轉換一整串輸入，只要有一筆壞掉整批就算失敗，常這樣用。

## 重點整理

- `collect()` 把迭代器的元素收成集合，必須讓編譯器知道目標型別：`let v: Vec<i32> = ...collect();` 或 `collect::<Vec<i32>>()`，沒標是 E0283。
- 型別位置的 `_` 讓編譯器自己填：`Vec<_>`、`HashMap<_, _>`。
- `iter()` 加 `filter` 收到的是參照（`Vec<&i32>`）；要元素本身，在 `collect` 前加 `copied()`（`Copy` 型別）或 `cloned()`（例如 `String`）。
- 可以收成 `String`（元素是 `char`）、`HashMap`（元素是 `(鍵, 值)`）、`Result<Vec<_>, _>`（全部成功才是 `Ok`，遇到第一個 `Err` 就停）。
