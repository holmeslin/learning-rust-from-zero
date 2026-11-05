# slice pattern

## 本集目標

用 `[first, .., last]` 這類模式，依 slice 或陣列的形狀（長度、頭尾元素）分支，並用 `rest @ ..` 取出剩下的部分。

## 正文

### 依長度與頭尾分支

第 4 章的 `&[T]` 可以直接 `match`，模式寫成中括號，裡面放每個元素的模式：

```rust
fn describe(nums: &[i32]) {
    match nums {
        [] => println!("空的"),
        [x] => println!("只有一個：{x}"),
        [first, .., last] => println!("頭 {first}，尾 {last}"),
    }
}

fn main() {
    describe(&[]);
    describe(&[7]);
    describe(&[1, 2, 3]);
}
```

執行結果：

```text
空的
只有一個：7
頭 1，尾 3
```

- `[]`：長度剛好是 0。
- `[x]`：長度剛好是 1，那個元素放進 `x`。
- `[first, .., last]`：至少兩個元素，第一個放進 `first`、最後一個放進 `last`。中間的 `..` 代表「任意多個元素，我不要」，零個也算。
- 三個分支合起來涵蓋所有長度，所以不用 `_`。

被比對的 `nums` 是參照，所以 `x`、`first`、`last` 都是 `&i32`，跟第 7 集 `match &nickname` 的道理一樣。用起來沒差別：能直接印，也能直接跟整數運算。

### `rest @ ..`：取出剩下的部分

想要剩下的元素，不是略過它，就用第 11 集的 `@`，把 `..` 綁定到名字。拿來寫遞迴很順手：

```rust
fn sum(nums: &[i32]) -> i32 {
    match nums {
        [] => 0,
        [first, rest @ ..] => first + sum(rest),
    }
}

fn main() {
    println!("{}", sum(&[1, 2, 3, 4]));
}
```

執行結果：

```text
10
```

`[first, rest @ ..]`：第一個元素放進 `first`，其餘放進 `rest`，`rest` 是 `&[i32]`，可以直接傳回 `sum`。空的 slice 回傳 0，遞迴就停了。

### 一樣要窮盡

漏掉長度，編譯器會指出缺哪一種：

```rust,compile_fail=E0004
fn describe(nums: &[i32]) {
    match nums {
        [] => println!("空的"),
        [x] => println!("只有一個：{x}"),
    }
}

fn main() {
    describe(&[1, 2]);
}
```

編譯錯誤：

```text
error[E0004]: non-exhaustive patterns: `&[_, _, ..]` not covered
 --> src/main.rs:2:11
  |
2 |     match nums {
  |           ^^^^ pattern `&[_, _, ..]` not covered
  |
  = note: the matched value is of type `&[i32]`
```

`&[_, _, ..]` 的意思是「至少兩個元素的 slice」，正是沒有分支接住的長度。slice 的長度要執行時才知道，所以一定得涵蓋所有長度，不能假設它夠長。

### 陣列的長度是型別的一部分

陣列 `[i32; 3]` 的長度編譯時期就知道，模式的元素個數必須剛好等於 3，就不需要處理其他長度：

```rust
fn main() {
    let arr = [10, 20, 30];
    let [a, b, c] = arr;
    println!("{a} {b} {c}");
    match arr {
        [10, y, z] => println!("10 開頭，後面是 {y} 和 {z}"),
        _ => println!("其他"),
    }
}
```

執行結果：

```text
10 20 30
10 開頭，後面是 20 和 30
```

`let [a, b, c] = arr;` 跟第 2 章拆 tuple 一樣。`match` 裡的 `10` 是字面值模式，只比對第一個元素是不是 10，所以還是需要 `_` 接住其他情況。`Vec` 第 7 章才登場，到時候把 `&v` 傳給 `&[T]` 的參數，函式裡就能用同樣的模式。

## 重點整理

- slice 與陣列能用中括號模式比對：`[]`、`[x]`、`[first, .., last]`；`..` 代表任意多個元素（零個也算）。
- `rest @ ..` 把剩下的元素綁定成 slice，常用來寫遞迴，例如 `[first, rest @ ..] => first + sum(rest)`。
- 對 `&[T]` 比對時，綁到的變數是參照（`&i32`），用法跟平常一樣。
- slice 的長度執行時才知道，`match` 要涵蓋所有長度（E0004，例如 `&[_, _, ..]` not covered）。
- 陣列長度固定，模式的元素個數要剛好等於陣列長度；`let [a, b, c] = arr;` 可以直接拆。
