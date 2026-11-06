# `Vec` 與 `vec!`

## 本集目標

知道 `Vec<T>` 是長度可以變動的陣列，會用 `vec!` 和 `Vec::new()` 建立，並用 `len`、`is_empty` 看長度，用 `{:?}` 印出內容。

## 正文

### 長度可以變的陣列

第 2 章的陣列，長度寫在型別裡，建立之後就固定了。實際的資料常常事先不知道有幾筆：今天賣了幾杯飲料、使用者輸入了幾個名字。這種資料用 `Vec`。

`Vec<i32>` 讀成「`i32` 的 `Vec`」：排成一排的 `i32`，隨時可以增加或減少。`<i32>` 跟第 6 章的 `Option<u32>` 一樣，先把 `Vec<i32>` 整個當成一個型別的名字，泛型第 8 章才會講。

### 用 `vec!` 建立

```rust
fn main() {
    let prices = vec![30, 45, 38];
    println!("{:?}", prices);
    println!("{}", prices.len());
    println!("{}", prices.is_empty());
}
```

執行結果：

```text
[30, 45, 38]
3
false
```

- `vec![...]` 用中括號列出元素，寫法很像陣列字面值。`vec!` 是巨集，跟 `println!` 同一類，第 16 章第 1 集才會正式介紹，現在把它當成「建立 `Vec` 的寫法」。
- `{:?}` 可以直接印 `Vec`，格式跟陣列一樣。
- `len()` 是元素個數，`is_empty()` 問它是不是空的。

### 用 `Vec::new()` 建立空的

```rust
fn main() {
    let empty: Vec<u32> = Vec::new();
    let zeros = vec![0; 5];
    println!("{:?} {}", empty, empty.len());
    println!("{:?}", zeros);
}
```

執行結果：

```text
[] 0
[0, 0, 0, 0, 0]
```

- `Vec::new()` 是第 5 章第 9 集的關聯函式，跟 `String::new()` 同一類，建立一個空的 `Vec`。空的 `Vec` 裡沒有元素可以讓編譯器推斷型別，所以要像這樣標上 `Vec<u32>`。下一集開始 `push` 元素之後，通常就不用標了。
- `vec![0; 5]` 跟陣列的 `[0; 5]` 一樣，是五個 `0`。

### `Vec` 擁有它的元素

`Vec` 的元素放在 heap 上，`Vec` 是它們的擁有者。第 3 章的規則照樣適用：`let b = a;` 是 move，`Vec` 不是 `Copy` 型別。

```rust,compile_fail=E0382
fn main() {
    let a = vec![1, 2, 3];
    let b = a;
    println!("{:?} {:?}", a, b);
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:4:27
  |
2 |     let a = vec![1, 2, 3];
  |         - move occurs because `a` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     let b = a;
  |             - value moved here
4 |     println!("{:?} {:?}", a, b);
  |                           ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let b = a.clone();
  |              ++++++++
```

訊息跟第 3 章第 6 集的 `String` 一樣：`a` 的所有權已經交給 `b`。真的需要兩份，就寫 `a.clone()`。離開作用域時，`Vec` 和它所有的元素會一起 drop。

### 跟 Go 比一比

Go 的 slice 賦值不會複製資料：

```go
a := []int{1, 2, 3}
b := a
b[0] = 99
fmt.Println(a) // [99 2 3]：a 和 b 共用同一份資料
```

Rust 的 `let b = a;` 是 move，`a` 不能再用，所以不會有兩個名字偷偷共用同一份資料。

## 重點整理

- `Vec<T>` 是長度可以變動的陣列，元素放在 heap；`<T>` 是泛型（第 8 章），先把 `Vec<i32>` 當成一個型別的名字。
- `vec![30, 45, 38]` 建立有元素的 `Vec`，`vec![0; 5]` 是五個 `0`；`Vec::new()` 建立空的，沒有元素可以推斷型別時要標型別。
- `len()` 是元素個數，`is_empty()` 問是不是空的，`{:?}` 可以印整個 `Vec`。
- `Vec` 擁有元素，不是 `Copy`：`let b = a;` 會 move（E0382）；要兩份就 `.clone()`。
- 跟 Go 比：Go 的 slice 賦值共用底層資料；Rust 的 `Vec` 賦值是 move。
