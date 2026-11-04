# `&mut [T]`

## 本集目標

用 `&mut [T]` 讓函式修改陣列的元素（整個陣列或其中一段），並知道同一個陣列不能同時借出兩個 `&mut` slice。

## 正文

### 借出可變的 slice

`&[T]` 只能讀。要改元素，就借出 `&mut [T]`，規則跟第 3 集的 `&mut String` 一樣：

```rust
fn double_all(nums: &mut [i32]) {
    for i in 0..nums.len() {
        nums[i] *= 2;
    }
}

fn main() {
    let mut a = [1, 2, 3, 4];
    double_all(&mut a);
    println!("{a:?}");
    double_all(&mut a[1..3]);
    println!("{a:?}");
}
```

執行結果：

```text
[2, 4, 6, 8]
[2, 8, 12, 8]
```

- `let mut a`：陣列本身要是 `mut`。
- `nums: &mut [i32]`：函式簽名寫明會修改。
- `double_all(&mut a)`：呼叫時寫 `&mut`，編譯器把 `&mut` 陣列自動轉成 `&mut [i32]`（跟 `&陣列` 轉成 `&[T]` 一樣）。
- `double_all(&mut a[1..3])`：只讓函式改中間那一段，第 0 個和第 3 個元素不受影響。

slice 的長度是固定的，只能改裡面的值，不能增加或刪除元素。那要等第 7 章的 `Vec`。

### `&[T]` 不能改

參數寫成 `&[i32]`，卻想改元素，會被擋下：

```rust,compile_fail=E0594
fn double_all(nums: &[i32]) {
    for i in 0..nums.len() {
        nums[i] *= 2;
    }
}

fn main() {
    let a = [1, 2, 3, 4];
    double_all(&a);
    println!("{a:?}");
}
```

編譯錯誤：

```text
error[E0594]: cannot assign to `nums[_]`, which is behind a `&` reference
 --> src/main.rs:3:9
  |
3 |         nums[i] *= 2;
  |         ^^^^^^^^^^^^ `nums` is a `&` reference, so it cannot be written to
  |
help: consider changing this to be a mutable reference
  |
1 | fn double_all(nums: &mut [i32]) {
  |                      +++
```

`behind a & reference`：它在 `&` 後面，不能指定新值。照 `help` 把 `&[i32]` 改成 `&mut [i32]`，呼叫端也要跟著改：`let mut a`、`double_all(&mut a)`。

### 不能同時借兩段

借用規則是以整個陣列為單位，不是看範圍。就算兩段完全沒有重疊，同時借出兩個 `&mut` slice 也不行：

```rust,compile_fail=E0499
fn main() {
    let mut a = [1, 2, 3, 4];
    let left = &mut a[0..2];
    let right = &mut a[2..4];
    left[0] = 10;
    right[0] = 30;
    println!("{a:?}");
}
```

編譯錯誤：

```text
error[E0499]: cannot borrow `a` as mutable more than once at a time
 --> src/main.rs:4:22
  |
3 |     let left = &mut a[0..2];
  |                     - first mutable borrow occurs here
4 |     let right = &mut a[2..4];
  |                      ^ second mutable borrow occurs here
5 |     left[0] = 10;
  |     ------- first borrow later used here
  |
  = help: use `.split_at_mut(position)` to obtain two mutable non-overlapping sub-slices
```

編譯器只知道 `left` 和 `right` 都在借 `a`，不會去比對 `0..2` 和 `2..4` 有沒有重疊。`help` 提到的 `split_at_mut` 是標準函式庫裡專門把一段切成兩段可變 slice 的方法，本書不展開。解法照第 5 集的做法：先把 `left[0] = 10;` 寫在 `let left` 的後面，`left` 用完之後，再借出 `right`，兩段就能各改各的，最後 `a` 會是 `[10, 2, 30, 4]`。

### 這一章的參數型別

整章用到的借用寫法，可以整理成一張表：

| 想做什麼 | 字串 | 陣列 |
| --- | --- | --- |
| 只要讀 | `&str` | `&[T]` |
| 要改 | `&mut String` | `&mut [T]` |

「只要讀」的型別，傳字面值、slice 或是整個擁有者的 `&` 都收；「要改」的型別，呼叫端一定要寫 `&mut`，擁有者也要是 `mut`。

### 跟 Go 比一比

Go 的 slice 可以直接拿來改，也能同時拿兩段各改各的：

```go
a := [4]int{1, 2, 3, 4}
left := a[0:2]
right := a[2:4]
left[0] = 10
right[0] = 30 // 兩段各改各的，沒問題
```

而且 Go 的函式只要收到 `[]int`，就能改到呼叫端的陣列，呼叫的地方看不出來。Rust 要在簽名寫 `&mut [T]`、呼叫端寫 `&mut`；同一個陣列的兩段，不能同時各借一個 `&mut`。

## 重點整理

- `&mut [T]` 讓函式修改陣列的元素，整個陣列或 `&mut a[起..迄]` 一段都行；陣列要是 `mut`，呼叫端要寫 `&mut`；長度固定，只能改值，不能增減元素。
- 用 `&[T]` 去改元素是 E0594（`behind a & reference`），改成 `&mut [T]`。
- 同一個陣列，同時借出兩個 `&mut` slice 是 E0499，就算範圍不重疊；一段用完再借下一段。
- 第 4 章的參數型別：只要讀用 `&str`、`&[T]`，要改用 `&mut String`、`&mut [T]`。
- Go 的 slice 隨便改、兩段能同時改；Rust 簽名與呼叫端都要寫明 `&mut`，也不能同時借兩段。
