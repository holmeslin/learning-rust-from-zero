# 陣列 slice `&[T]`

## 本集目標

用 `&a[起..迄]` 取得陣列的一段，知道型別是 `&[T]`，並讓函式用 `&[T]` 參數接收任意長度的陣列或其中一段。

## 正文

### 取一段陣列

陣列也能切，寫法跟字串 slice 一樣：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let s = &a[1..4];
    println!("{s:?} {}", s.len());
}
```

執行結果：

```text
[20, 30, 40] 3
```

`s` 的型別是 `&[i32]`，讀成「`i32` 的 slice」：指向陣列其中一段、不擁有元素的參照。`{:?}` 可以直接印 slice，`len` 回傳這一段的長度，用法跟陣列一樣。

### 為什麼需要 slice

陣列的長度是型別的一部分：`[i32; 3]` 和 `[i32; 5]` 是不同的型別。想寫一個函式處理陣列，如果參數寫死長度，換個長度就傳不進去了：

```rust,compile_fail=E0308
fn sum(nums: &[i32; 3]) -> i32 {
    nums[0] + nums[1] + nums[2]
}

fn main() {
    let a = [1, 2, 3];
    let b = [10, 20, 30, 40, 50];
    println!("{}", sum(&a));
    println!("{}", sum(&b));
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:9:24
  |
9 |     println!("{}", sum(&b));
  |                    --- ^^ expected an array with a size of 3, found one with a size of 5
  |                    |
  |                    arguments to this function are incorrect
  |
note: function defined here
 --> src/main.rs:1:4
  |
1 | fn sum(nums: &[i32; 3]) -> i32 {
  |    ^^^ ---------------
```

### 參數寫 `&[T]`

把參數改成 `&[i32]`，不管陣列多長都能收：

```rust
fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for i in 0..nums.len() {
        total += nums[i];
    }
    total
}

fn main() {
    let a = [1, 2, 3];
    let b = [10, 20, 30, 40, 50];
    println!("{}", sum(&a));
    println!("{}", sum(&b));
    println!("{}", sum(&b[1..3]));
}
```

執行結果：

```text
6
150
50
```

- `sum(&a)`、`sum(&b)`：傳的是整個陣列的參照，編譯器在需要 `&[i32]` 的地方會自動轉換，跟上一集 `&String` 變成 `&str` 是同一類事，現在照寫就好。
- `sum(&b[1..3])`：只傳其中一段。
- 函式裡用 `nums.len()` 取得長度，再用索引 `nums[i]` 讀每個元素，跟陣列的寫法一樣。

### 一樣是借用

slice 借著陣列，借用規則照樣適用。slice 還要用的時候去改陣列，會被擋下：

```rust,compile_fail=E0506
fn main() {
    let mut a = [10, 20, 30];
    let s = &a[0..2];
    a[0] = 99;
    println!("{s:?} {a:?}");
}
```

編譯錯誤：

```text
error[E0506]: cannot assign to `a[_]` because it is borrowed
 --> src/main.rs:4:5
  |
3 |     let s = &a[0..2];
  |              - `a[_]` is borrowed here
4 |     a[0] = 99;
  |     ^^^^^^^^^ `a[_]` is assigned to here but it was already borrowed
5 |     println!("{s:?} {a:?}");
  |                - borrow later used here
```

這次錯誤碼是 E0506，意思是「對還在被借用的東西指定新值」。解法跟之前一樣：先把 `s` 用完，再改 `a`。

索引超出 slice 的長度，或是切的範圍超出陣列，一樣會在執行時 panic，跟第 2 章第 12 集的陣列越界相同。

### 跟 Go 比一比

Go 的 `[]int` 也叫 slice，但行為差很多：

```go
a := [5]int{10, 20, 30, 40, 50}
s := a[1:4]
s[0] = 99        // a[1] 也變成 99
s = append(s, 0) // 容量夠，直接蓋掉 a[4]
fmt.Println(a)   // [10 99 30 40 0]
```

Go 的 slice 帶著容量，改動和 `append` 會悄悄影響底下共用的陣列。Rust 的 `&[T]` 只是借來的一段，只能讀、沒有容量、不能變長；要改的話用 `&mut [T]`（下一集），而且受借用規則管：改的時候不會有別人在讀。

## 重點整理

- `&a[起..迄]` 取得陣列 slice，型別是 `&[T]`；slice 是借用，不擁有元素。
- 陣列長度是型別的一部分，`&[i32; 3]` 收不了別的長度（E0308）；`&[i32]` 什麼長度都能收，也能只傳其中一段。
- `len`、索引、`{:?}` 都能用在 slice 上，越界在執行時 panic；slice 還要用時，不能改被借用的陣列（E0506）。
- 只要讀的陣列參數寫 `&[T]`，編譯器會把 `&陣列` 自動轉成 `&[T]`。
- Go 的 slice 共用底層陣列、能 `append`；Rust 的 `&[T]` 只是借來的一段，只能讀。
