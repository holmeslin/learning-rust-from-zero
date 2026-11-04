# `for` 走訪陣列

## 本集目標

用 `for x in 陣列` 逐一走訪陣列的元素，並知道需要索引時怎麼寫。

## 正文

### 走訪每個元素

第 1 章的 `for i in 0..n` 走訪的是一個範圍。把範圍換成陣列，就是走訪陣列裡的每個元素：

```rust
fn main() {
    let scores = [85, 92, 78];
    for score in scores {
        println!("{score}");
    }
}
```

執行結果：

```text
85
92
78
```

每一圈，`score` 會是陣列裡的下一個值，從第 0 個走到最後一個。跟 `i` 一樣，它是每一圈自動建立的變數，不需要事先宣告。`for x in 陣列` 這個寫法請**先照抄**，原理第 11 章才會解釋。

走訪陣列最常見的用途，是累加和找極值：

```rust
fn main() {
    let scores = [85, 92, 78];
    let mut total = 0;
    let mut best = scores[0];
    for score in scores {
        total += score;
        if score > best {
            best = score;
        }
    }
    println!("總分 {total}，最高分 {best}");
}
```

執行結果：

```text
總分 255，最高分 92
```

### 需要索引的時候

`for score in scores` 拿不到「這是第幾個」。需要索引時，改成走訪範圍，再用索引取值：

```rust
const DAYS: usize = 3;

fn main() {
    let names = ["週一", "週二", "週三"];
    let scores = [85, 92, 78];
    for i in 0..DAYS {
        println!("{}：{}", names[i], scores[i]);
    }
}
```

執行結果：

```text
週一：85
週二：92
週三：78
```

範圍的上限要跟陣列的長度一致。如果 `DAYS` 寫得比陣列長，多跑的那一圈就會越界 panic（上一集講過）。陣列也能用 `.len()` 直接問長度，第 15 集會看到這個寫法；第 11 章會學到更方便、不會寫錯上限的做法。

### 跟 Go 比一比

Go 的 `range` 會同時給索引和元素，只寫一個變數時拿到的是**索引**：

```go
for _, score := range scores {} // 元素
for i := range scores {}        // 索引
```

Rust 的 `for` 只給元素：`for score in scores` 的 `score` 就是值本身。照 Go 的習慣寫 `for i, score in scores`，會得到語法錯誤：

```rust,compile_fail
fn main() {
    let scores = [85, 92, 78];
    for i, score in scores {
        println!("{i} {score}");
    }
}
```

編譯錯誤：

```text
error: unexpected `,` in pattern
 --> src/main.rs:3:10
  |
3 |     for i, score in scores {
  |          ^
  |
help: try adding parentheses to match on a tuple
  |
3 |     for (i, score) in scores {
  |         +        +
```

`help` 建議加括號，是想把 `(i, score)` 當成 tuple 來解構（第 10 集），但陣列的元素是整數，不是 tuple，照改一樣編譯不過。想同時用索引和元素，就用上面的 `for i in 0..DAYS` 再 `scores[i]`。

## 重點整理

- `for x in 陣列` 逐一走訪元素，`x` 是每一圈的值（先照抄，第 11 章解釋）。
- 累加、找最大值，都是「走訪 + 更新 `let mut` 變數」。
- 需要索引時，用 `for i in 0..長度` 再寫 `陣列[i]`；上限要跟陣列長度一致。
- Go 的 `range` 只寫一個變數會拿到索引；Rust 的 `for x in 陣列` 拿到的是元素，不能寫成 `for i, x in`。
