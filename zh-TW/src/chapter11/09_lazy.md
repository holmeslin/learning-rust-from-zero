# 迭代器是惰性的

## 本集目標

分清楚 adapter 與 consumer：adapter 只是把迭代器包起來，什麼都不做；要有 consumer 去呼叫 `next()`，事情才真的發生，而且元素是一個接一個被處理的。

## 正文

### adapter：包一層，還沒開始

迭代器的方法分兩種。第一種叫 **adapter**，接收一個迭代器，回傳另一個迭代器，例如 `map`（下一集正式教，先當成「對每個元素做某件事、產生新值」）。看看只呼叫 `map` 會怎樣：

```rust,warn
fn main() {
    let prices = [30, 45, 38];
    prices.iter().map(|p| {
        println!("算 {p}");
        p * 2
    });
    println!("結束");
}
```

編譯警告：

```text
warning: unused `Map` that must be used
 --> src/main.rs:3:5
  |
3 | /     prices.iter().map(|p| {
4 | |         println!("算 {p}");
5 | |         p * 2
6 | |     });
  | |______^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = prices.iter().map(|p| {
  |     +++++++
```

執行結果：

```text
結束
```

`算` 一次都沒印。編譯器的警告說得直接：`iterators are lazy and do nothing unless consumed`，迭代器是惰性的，不用掉就什麼都不做。`map` 只是記下「之後要對每個元素做這件事」，沒有人呼叫 `next()`，就沒有任何元素被處理。

### consumer：讓它真的跑

第二種叫 **consumer**，會去呼叫 `next()`，把元素一個個拉出來。`for` 就是一種 consumer：

```rust
fn main() {
    let prices = [30, 45, 38];
    let doubled = prices.iter().map(|p| {
        println!("算 {p}");
        p * 2
    });
    println!("還沒開始");
    for d in doubled {
        println!("拿到 {d}");
    }
}
```

執行結果：

```text
還沒開始
算 30
拿到 60
算 45
拿到 90
算 38
拿到 76
```

- `doubled` 建好的時候，什麼都還沒算，所以「還沒開始」最先印。
- 迴圈每拉一個元素，閉包才執行一次：`算 30`、`拿到 60`、`算 45`……是交錯的。不是先全部算完、再一次交給 `for`。

### 只做你要的份

惰性的好處是：沒被拉到的元素，閉包根本不會跑。用 `next()` 只拉兩次：

```rust
fn main() {
    let prices = [30, 45, 38];
    let mut doubled = prices.iter().map(|p| {
        println!("算 {p}");
        p * 2
    });
    println!("{:?}", doubled.next());
    println!("{:?}", doubled.next());
}
```

執行結果：

```text
算 30
Some(60)
算 45
Some(90)
```

38 沒被拉到，就沒有 `算 38`。後面的 `find`（第 15 集）找到答案就停，也是靠這個性質：不會在找到之後，還把剩下的元素都走一遍。

### 兩種各有哪些

| 類別 | 做什麼 | 本章會學到的 |
| --- | --- | --- |
| adapter | 回傳新的迭代器，惰性 | `map`、`filter`、`enumerate`、`zip`、`rev` |
| consumer | 呼叫 `next()` 拉元素，得到結果 | `for`、`next`、`nth`、`collect`、`sum`、`count`、`min`、`max`、`fold`、`find`、`any`、`all`、`position` |

寫迭代器的串接，要確認最後一個是 consumer。只串了 adapter 而沒有 consumer，就像上面第一個範例：編譯器警告，程式也沒做事。

## 重點整理

- adapter（`map`、`filter` 等）接收迭代器、回傳新的迭代器，是惰性的：只記下要做什麼，不處理任何元素。
- consumer（`for`、`next`、`collect`、`sum` 等）會呼叫 `next()` 拉元素，這時閉包才真的執行。
- 元素是一個接一個流過整條串接：每拉一個，就跑一次前面所有 adapter 的閉包；沒被拉到的元素不會被處理。
- 只有 adapter、沒有 consumer，編譯器會警告 `iterators are lazy and do nothing unless consumed`。
