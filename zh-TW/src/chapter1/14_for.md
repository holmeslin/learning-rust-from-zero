# `for`、範圍與 `continue`

## 本集目標

用 `for i in 0..n` 和 `for i in 0..=n` 重複固定的次數，並學會 `continue` 與巢狀迴圈。

## 正文

### `for` 走訪一個範圍

Rust 的 `for` 不是三段式的 `for i := 0; i < n; i++`，而是「走訪一個範圍」。`0..3` 表示從 0 開始，到 3 **之前**結束：

```rust
fn main() {
    let n = 3;
    for i in 0..n {
        println!("i = {i}");
    }
}
```

執行結果：

```text
i = 0
i = 1
i = 2
```

這兩種寫法請**先照抄**，原理第 11 章才會解釋：

- `for i in 0..n`：`i` 從 0 數到 `n - 1`，共跑 `n` 圈，不包含 `n`。
- `for i in 0..=n`：多一個 `=`，表示**包含** `n`。

```rust
fn main() {
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("1 加到 100 是 {sum}");
}
```

執行結果：

```text
1 加到 100 是 5050
```

`i` 是每一圈自動建立的新變數，不需要事先宣告，也不用 `mut`。範圍的起點和終點可以是數字或變數。如果起點不小於終點（例如 `5..0`），迴圈一次都不會跑，Rust 不會自動倒著數。要倒數，目前用上一集的 `while`。

### `continue`

`continue` 跳過這一圈剩下的部分，直接進入下一圈，跟 Go 一樣：

```rust
fn main() {
    for i in 1..=6 {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}");
    }
}
```

執行結果：

```text
1
3
5
```

### 巢狀迴圈

迴圈裡面可以再放迴圈。內層迴圈會在外層的每一圈裡，完整跑一遍：

```rust
fn main() {
    for i in 1..=3 {
        for j in 1..=3 {
            println!("{i} x {j} = {}", i * j);
        }
    }
}
```

執行結果：

```text
1 x 1 = 1
1 x 2 = 2
1 x 3 = 3
2 x 1 = 2
2 x 2 = 4
2 x 3 = 6
3 x 1 = 3
3 x 2 = 6
3 x 3 = 9
```

`break` 和 `continue` 只影響**最內層**的那個迴圈：

```rust
fn main() {
    for i in 1..=3 {
        for j in 1..=3 {
            if j == 2 {
                break;
            }
            println!("i = {i}, j = {j}");
        }
    }
}
```

執行結果：

```text
i = 1, j = 1
i = 2, j = 1
i = 3, j = 1
```

內層每次遇到 `j == 2` 就結束，但外層照樣跑完三圈。想一次跳出外層，要用標籤，附錄一 b 會介紹。

### 跟 Go 比一比

Go 的三段式 `for`，Rust 一律改寫成範圍：

```go
for i := 0; i < n; i++ {}   // Rust：for i in 0..n
for i := 1; i <= n; i++ {}  // Rust：for i in 1..=n
```

最容易搞混的是終點：`0..n` 不包含 `n`，對應 `i < n`；要包含 `n` 必須多寫 `=`，對應 `i <= n`。少寫 `=` 不會報錯，只會少跑一圈。

## 重點整理

- `for i in 0..n` 跑 `n` 圈（不含 `n`），`for i in 0..=n` 含 `n`；兩種寫法先照抄，第 11 章解釋。
- 迴圈變數 `i` 是自動建立的，不用事先宣告。
- `continue` 跳到下一圈；`break` 和 `continue` 只作用在最內層迴圈。
- Rust 沒有三段式 `for`；範圍的終點預設不包含。
