# 走訪 `Vec`（`&v`、`&mut v`）

## 本集目標

用 `for x in &v` 讀每個元素、`for x in &mut v` 修改每個元素，並知道不加 `&` 的 `for x in v` 會把整個 `Vec` move 掉。

## 正文

### `for x in &v`：讀

第 2 章第 13 集的 `for x in 陣列` 也能用在 `Vec` 上。想走訪之後還要繼續用 `Vec`，在前面加 `&`，借出一個只能讀的參照：

```rust
fn main() {
    let prices = vec![30, 45, 38];
    let mut total = 0;
    for price in &prices {
        println!("{price}");
        total += price;
    }
    println!("共 {total}");
    println!("{:?}", prices);
}
```

執行結果：

```text
30
45
38
共 113
[30, 45, 38]
```

- `&prices` 是借用，走訪完 `prices` 還在，最後一行照樣能印。
- 迴圈變數 `price` 是 `&i32`，指向 `Vec` 裡的元素。`{price}` 印出來是數字本身；`total += price` 這種算術可以直接寫。
- 陣列和 `&[T]` 也能這樣寫：`for x in &a`、`for x in slice`。第 4 章教的 `for i in 0..a.len()` 加索引的寫法照樣可用，但走訪全部元素時，直接 `for x in &v` 不會算錯範圍。

### `for x in &mut v`：改

要改動每個元素，借出 `&mut`，`Vec` 要用 `let mut`。迴圈變數變成 `&mut i32`，要改它指向的值，用第 4 章第 7 集的 `*`：

```rust
fn main() {
    let mut prices = vec![30, 45, 38];
    for price in &mut prices {
        *price += 5;
    }
    println!("{:?}", prices);
}
```

執行結果：

```text
[35, 50, 43]
```

### `for x in v`：整個交出去

前面不加 `&`，`for` 會拿走整個 `Vec`，逐一把元素交給迴圈變數。迴圈結束，`Vec` 就用完了：

```rust,compile_fail=E0382
fn main() {
    let prices = vec![30, 45, 38];
    for price in prices {
        println!("{price}");
    }
    println!("{:?}", prices);
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `prices`
 --> src/main.rs:6:22
  |
2 |     let prices = vec![30, 45, 38];
  |         ------ move occurs because `prices` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     for price in prices {
  |                  ------ `prices` moved due to this implicit call to `.into_iter()`
...
```

訊息說 `prices` 在 `for` 迴圈那裡 move 掉了，後面再用就是 E0382。後面還有 `note` 和 `help`：`help` 直接建議改成 `for price in &prices`。訊息裡的 `into_iter` 是 `for` 背後呼叫的方法，第 11 章第 8 集會揭曉，現在不用管。

三種寫法整理起來：

| 寫法 | 迴圈變數 | 走訪之後 `Vec` |
| --- | --- | --- |
| `for x in &v` | `&T`，只能讀 | 還能用 |
| `for x in &mut v` | `&mut T`，可以改 | 還能用 |
| `for x in v` | `T`，拿到元素本身 | 已經 move，不能用 |

### 跟 Go 比一比

Go 的 `range` 給的是元素的複本，改它不會影響 slice：

```go
prices := []int{30, 45, 38}
for _, p := range prices {
	p += 5 // 只改到複本，prices 沒變
}
fmt.Println(prices) // [30 45 38]
```

Rust 的 `for x in &mut v` 給的是指向元素的 `&mut`，改的就是 `Vec` 裡的那個元素；而且在迴圈裡不能同時用別的方式動 `Vec`（下一集）。

## 重點整理

- `for x in &v` 走訪 `Vec`，`x` 是 `&T`，走訪完 `v` 還能用；陣列和 `&[T]` 也能這樣寫。
- `for x in &mut v` 可以修改每個元素，`x` 是 `&mut T`，用 `*x` 改值；`v` 要 `let mut`。
- `for x in v` 會把 `Vec` move 進迴圈，之後不能再用（E0382）；想繼續用就加 `&`。
- 跟 Go 比：Go 的 `range` 給元素的複本，改了沒用；Rust 的 `&mut` 改的是原本的元素。
