# 短路求值

## 本集目標

知道 `&&`、`||` 只在需要的時候才算右邊，能用它擋掉會出錯的運算；也認得兩邊都會算的 `&` 和 `|`。

## 正文

### `&&` 與 `||` 會短路

`&&` 的左邊是 `false`，結果一定是 `false`，右邊就不算了；`||` 的左邊是 `true`，右邊也不算。這叫短路求值（short-circuit evaluation），Go 也一樣。用一個會印訊息的函式就看得出來：

```rust
fn check(name: &str, result: bool) -> bool {
    println!("檢查 {name}");
    result
}

fn main() {
    let a = check("庫存", false) && check("折扣", true);
    println!("{a}");
    let b = check("庫存", true) || check("折扣", true);
    println!("{b}");
}
```

執行結果：

```text
檢查 庫存
false
檢查 庫存
true
```

兩次都只印了「檢查 庫存」：`check("折扣", true)` 根本沒被呼叫。

### 用短路擋掉會出錯的運算

短路最常見的用法，是先檢查、再做可能出錯的事。第 2 章第 12 集說過，索引越界會 panic；把範圍檢查放在 `&&` 左邊，右邊的索引只有在安全的時候才會執行：

```rust
fn is_pricey(prices: &[u32], i: usize) -> bool {
    i < prices.len() && prices[i] > 40
}

fn main() {
    let prices = [30, 45, 38];
    println!("{}", is_pricey(&prices, 1));
    println!("{}", is_pricey(&prices, 5));
}
```

執行結果：

```text
true
false
```

`i` 是 5 的時候，`5 < 3` 是 `false`，`prices[5]` 不會被算。除以 0 也是同樣的寫法：`cups != 0 && total / cups > 20`。順序不能反過來，先算右邊就先出錯了。

第 6 章第 15 集的 let chains 一樣是由左到右、遇到 `false` 就停：`if let Some(p) = price && p > 40` 裡，`Some(p)` 沒比對成功，右邊的 `p > 40` 就不算。

### `&` 與 `|` 不會短路

`bool` 也能用 `&`、`|`，意思跟 `&&`、`||` 一樣，但兩邊一定都會算：

```rust
fn check(name: &str, result: bool) -> bool {
    println!("檢查 {name}");
    result
}

fn main() {
    let a = check("庫存", false) & check("折扣", true);
    println!("{a}");
    let b = check("庫存", true) | check("折扣", true);
    println!("{b}");
}
```

執行結果：

```text
檢查 庫存
檢查 折扣
false
檢查 庫存
檢查 折扣
true
```

這次「折扣」也檢查了。結果跟 `&&`、`||` 相同，差別只在右邊有沒有被算。拿它來「擋」就會出事：

```rust,exit=101
fn is_pricey(prices: &[u32], i: usize) -> bool {
    (i < prices.len()) & (prices[i] > 40)
}

fn main() {
    let prices = [30, 45, 38];
    println!("{}", is_pricey(&prices, 5));
}
```

執行結果（某一次）：

```text
thread 'main' (159160201) panicked at src/main.rs:2:27:
index out of bounds: the len is 3 but the index is 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

左邊明明是 `false`，右邊的 `prices[5]` 還是被算了，所以越界 panic。`thread 'main'` 後面括號裡的數字每次都不同，其他都一樣。

另外 `&`、`|` 的優先順序比 `==`、`>` 高，所以兩邊的比較要加括號（像上面的 `(i < prices.len())`），不加會得到編譯錯誤。

Go 的 `&`、`|` 只能用在整數；Rust 的 `bool` 也能用，所以讀到 `a & b` 時要想一下，兩邊是整數，還是兩邊都會算的 `bool`。要擋掉右邊，一律用 `&&` 和 `||`。

## 重點整理

- `a && b`：`a` 是 `false` 就不算 `b`；`a || b`：`a` 是 `true` 就不算 `b`。這跟 Go 一樣。
- 把檢查放在 `&&` 左邊，可以擋掉索引越界、除以 0 這類出錯的運算；順序不能反。
- let chains 也是由左到右、遇到 `false` 就停。
- `bool` 的 `&`、`|` 兩邊都會算，擋不住右邊的錯誤；而且優先順序比 `==`、`>` 高，要加括號。
- Go 的 `&`、`|` 只能用在整數，Rust 的 `bool` 也能用。
