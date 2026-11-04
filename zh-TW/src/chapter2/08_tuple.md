# tuple

## 本集目標

把幾個值綑成一個 tuple，用 `.0`、`.1` 取出裡面的值。

## 正文

### 建立與讀取

tuple 用小括號把好幾個值包在一起，值之間用逗號隔開，**每個值的型別可以不一樣**。要取出第幾個值，就在變數後面加 `.0`、`.1`、`.2`，編號從 0 開始：

```rust
fn main() {
    let order = (3, 12.5, true);
    println!("數量：{}", order.0);
    println!("單價：{}", order.1);
    println!("已付款：{}", order.2);
}
```

執行結果：

```text
數量：3
單價：12.5
已付款：true
```

這個 tuple 裡的三個值，型別依序是 `i32`、`f64`、`bool`。tuple 本身的型別就是把它們照順序寫在小括號裡：`(i32, f64, bool)`。要標型別時這樣寫：

```rust
fn main() {
    let point: (i32, i32) = (3, 4);
    println!("x = {}, y = {}", point.0, point.1);
}
```

執行結果：

```text
x = 3, y = 4
```

第 7 集的 `()` 其實就是「零個值的 tuple」，所以它的寫法才會是一對空括號。

### 長度固定、可以修改

tuple 建立之後，長度和每個位置的型別就固定了，不能新增或刪除元素。想修改裡面的值，變數要用 `let mut`，再對 `.0`、`.1` 賦值：

```rust
fn main() {
    let mut point = (3, 4);
    point.0 = 10;
    println!("x = {}, y = {}", point.0, point.1);
}
```

執行結果：

```text
x = 10, y = 4
```

### 編號超出範圍，編譯就會擋下

tuple 的長度是型別的一部分，所以編號超出範圍，不用等到執行就會被編譯器發現：

```rust,compile_fail=E0609
fn main() {
    let point = (3, 4);
    println!("{}", point.2);
}
```

編譯錯誤：

```text
error[E0609]: no field `2` on type `({integer}, {integer})`
 --> src/main.rs:3:26
  |
3 |     println!("{}", point.2);
  |                          ^ unknown field
  |
  = note: available fields are: `0`, `1`
```

## 重點整理

- tuple 用小括號和逗號建立，例如 `(3, 12.5, true)`；每個位置的型別可以不同。
- 型別寫成 `(i32, f64, bool)`，用 `.0`、`.1` 取值，編號從 0 開始。
- 長度與每個位置的型別建立後就固定；要修改，用 `let mut` 再對 `.0` 賦值。
- 編號超出範圍是編譯錯誤（`E0609`）。
- `()` 是零個值的 tuple。
