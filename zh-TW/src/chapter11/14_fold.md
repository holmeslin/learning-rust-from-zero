# `fold`

## 本集目標

用 `fold(初始值, |累積值, 元素| ...)` 把整個迭代器收成一個值，並認得它就是「帶著累加變數的 `for` 迴圈」。

## 正文

### 一個累加變數，一路帶著走

第 12 集說浮點數不能用 `max`。換個辦法：先寫成熟悉的迴圈，用一個變數記著「目前最大的」。評分是 `f64`，`f64::MIN` 是最小的 `f64`，當成起點：

```rust
fn main() {
    let scores = [4.5, 3.0, 4.8];
    let mut best = f64::MIN;
    for s in scores {
        best = best.max(s);
    }
    println!("最高 {best}");
}
```

執行結果：

```text
最高 4.8
```

`best.max(s)` 是 `f64` 自己的方法，回傳兩個數字比較大的那個。`fold` 把這個模式包起來：第一個引數是起點，第二個是閉包，每個元素呼叫一次，閉包拿到「目前累積的值」和「這一個元素」，回傳的新值就是下一輪的累積值：

```rust
fn main() {
    let scores = [4.5, 3.0, 4.8];
    let best = scores.iter().copied().fold(f64::MIN, f64::max);
    println!("最高 {best}");
}
```

執行結果：

```text
最高 4.8
```

- 起點 `f64::MIN` 就是 `let mut best = f64::MIN;`；閉包的第一個參數相當於 `best`，第二個參數相當於 `s`。
- `f64::max` 是函式名稱，簽名正好是「兩個 `f64` 進去、一個 `f64` 出來」，第 5 集說過可以直接當閉包傳。
- `copied()` 把 `&f64` 換成 `f64`（第 11 集），這樣元素的型別才跟累積值一樣。
- 最後 `fold` 回傳最後一輪的累積值。

### 累積值可以是任何型別

累積值不一定是數字，tuple 也行。下面一趟走完，同時算出個數和總和：

```rust
fn main() {
    let prices = [30, 45, 38];
    let (count, total) = prices.iter().fold((0, 0), |(n, sum), p| (n + 1, sum + p));
    println!("{count} 杯，共 {total} 元，平均 {} 元", total / count);
}
```

執行結果：

```text
3 杯，共 113 元，平均 37 元
```

起點 `(0, 0)` 是 `(個數, 總和)`；閉包的第一個參數寫成 `(n, sum)`，直接把 tuple 拆開（第 6 章第 12 集），回傳新的 tuple。

累積值也可以是 `String`。每一輪拿到上一輪的 `String`，加一段字，再交回去，所以參數要寫成 `mut acc`：

```rust
fn main() {
    let menu = ["紅茶", "咖啡", "奶茶"];
    let line = menu.iter().fold(String::new(), |mut acc, name| {
        if !acc.is_empty() {
            acc.push_str(" / ");
        }
        acc.push_str(name);
        acc
    });
    println!("{line}");
}
```

執行結果：

```text
紅茶 / 咖啡 / 奶茶
```

閉包的主體最後一行是 `acc`，這是回傳值：`fold` 會把它當成下一輪的累積值。忘了寫的話，閉包回傳 `()`，型別對不上，是 E0308。

### 什麼時候不用 `fold`

純粹加總用 `sum()`，找最大最小用 `max()`、`min()`（不是浮點數的話），數個數用 `count()`。這幾個 consumer 都是 `fold` 的特例，更好讀；clippy 看到 `fold(0, |a, b| a + b)` 還會直接建議你改成 `sum()`。`fold` 用在這些方法做不到的地方：浮點數的最大值、同時算好幾個值、把元素串成一個新的東西。

## 重點整理

- `fold(起點, |累積值, 元素| 新的累積值)`：每個元素呼叫一次閉包，回傳值當下一輪的累積值，最後回傳最終的累積值。
- 等於 `let mut acc = 起點; for x in ... { acc = 新值; }`；累積值的型別由起點決定，可以是 tuple、`String` 等。
- 閉包要回傳新的累積值（最後一個運算式），忘了寫就是 E0308；累積值要修改時，參數寫 `mut acc`。
- 浮點數的最大值：`.copied().fold(f64::MIN, f64::max)`；純加總、找最大最小、數個數，用 `sum`、`max`、`min`、`count` 更好。
