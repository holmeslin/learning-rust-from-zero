# 讀取元素：索引 vs `get`

## 本集目標

知道讀 `Vec` 的元素有兩種方法：`v[i]` 越界會 panic，`v.get(i)` 回傳 `Option`；並分得出什麼時候用哪一種。

## 正文

### 索引 `v[i]`

`Vec` 讀元素的寫法跟第 2 章第 12 集的陣列相同：從 0 開始，索引要是 `usize`：

```rust
fn main() {
    let mut scores = vec![85, 92];
    scores.push(78);
    let i = 2;
    println!("{}", scores[0]);
    println!("{}", scores[i]);
}
```

執行結果：

```text
85
78
```

超出範圍就 panic，跟 Go 一樣。不一樣的是時機：陣列的長度寫在型別裡，`a[10]` 這種寫死的索引在編譯時期就會被擋下來；`Vec` 的長度要到執行時才知道，所以到執行時才 panic：

```rust,exit=101
fn main() {
    let mut scores = vec![85, 92];
    scores.push(78);
    println!("{}", scores[10]);
}
```

執行結果（某一次）：

```text
thread 'main' (157654543) panicked at src/main.rs:4:26:
index out of bounds: the len is 3 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`thread 'main'` 後面的數字每次執行都不一樣。訊息說得很清楚：長度是 3，索引是 10。

### `get`：回傳 `Option`

不確定索引在不在範圍內，就用 `get`。它回傳 `Option`：在範圍內是 `Some(元素)`，超出範圍是 `None`，不會 panic。

```rust
fn main() {
    let mut scores = vec![85, 92];
    scores.push(78);
    println!("{:?}", scores.get(1));
    println!("{:?}", scores.get(10));
    match scores.get(10) {
        Some(score) => println!("第 11 筆是 {score}"),
        None => println!("沒有第 11 筆"),
    }
    if let Some(score) = scores.get(2) {
        println!("第 3 筆加 5 分：{}", score + 5);
    }
}
```

執行結果：

```text
Some(92)
None
沒有第 11 筆
第 3 筆加 5 分：83
```

`get` 回傳的是 `Option<&i32>`，裡面是元素的參照，所以綁到的 `score` 是 `&i32`。跟第 6 章第 16 集一樣：拿來做算術可以直接寫（`score + 5`），要比較大小才需要 `*score`。

### 怎麼選

- 索引一定在範圍內（例如 `for i in 0..scores.len()` 裡的 `i`），用 `scores[i]`，簡潔。
- 索引來自外面（使用者輸入、別的計算結果），用 `get`，用 `match` 或 `if let` 處理 `None`。

### 非 `Copy` 的元素不能直接取出

整數是 `Copy`，`scores[0]` 直接複製一份。`String` 不是，`names[0]` 想把元素「拿出來」，等於把它從 `Vec` 裡 move 走：

```rust,compile_fail=E0507
fn main() {
    let names = vec![String::from("小明"), String::from("小華")];
    let first = names[0];
    println!("{first}");
}
```

編譯錯誤：

```text
error[E0507]: cannot move out of index of `Vec<String>`
 --> src/main.rs:3:17
  |
3 |     let first = names[0];
  |                 ^^^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
  |
help: consider borrowing here
  |
3 |     let first = &names[0];
  |                 +
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let first = names[0].clone();
  |                         ++++++++
```

`Vec` 還擁有 `names[0]`，不能讓它被搬走。照 `help` 的建議，借用就好：`let first = &names[0];`。第 4 章第 7 集也看過 E0507，同樣是「從借來或是被擁有的地方 move 出東西」。

## 重點整理

- `v[i]` 讀第 `i` 個元素，索引要是 `usize`；超出範圍在執行時 panic（`Vec` 沒有編譯時期的檢查）。
- `v.get(i)` 回傳 `Option<&T>`：範圍內是 `Some(元素的參照)`，超出範圍是 `None`。
- 索引一定在範圍內用 `v[i]`；索引來自外面用 `get`，再用 `match` 或 `if let` 處理。
- 對非 `Copy` 的元素寫 `let x = v[0];` 會想把元素 move 出來，編譯錯誤 E0507；借用 `&v[0]` 即可。
