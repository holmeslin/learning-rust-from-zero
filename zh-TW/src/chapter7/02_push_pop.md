# `push`、`pop` 與 `while let`

## 本集目標

用 `push` 在 `Vec` 尾端加元素、用 `pop` 從尾端拿走元素（回傳 `Option`），並用 `while let` 一路 `pop` 到空為止。

## 正文

### `push`：加在尾端

第 4 章第 12 集說過，陣列和 slice 不能增減元素。`Vec` 可以：

```rust
fn main() {
    let mut orders = Vec::new();
    orders.push(30);
    orders.push(45);
    orders.push(38);
    println!("{:?}", orders);
    println!("{}", orders.len());
}
```

執行結果：

```text
[30, 45, 38]
3
```

- `push` 會改動 `Vec` 本身，所以要 `let mut`，原因跟 `String` 的 `push_str` 一樣。
- 這次沒有標型別：第一個 `push(30)` 就讓編譯器知道這是 `Vec<i32>`。

### `pop`：從尾端拿走

`pop` 把最後一個元素拿出來，回傳 `Option`：有元素回傳 `Some(值)`，空的 `Vec` 回傳 `None`。

```rust
fn main() {
    let mut orders = vec![30, 45, 38];
    println!("{:?}", orders.pop());
    println!("{:?}", orders.pop());
    println!("{:?}", orders);
    orders.pop();
    println!("{:?}", orders.pop());
}
```

執行結果：

```text
Some(38)
Some(45)
[30]
None
```

最後一行是在空的 `Vec` 上 `pop`，得到 `None`，沒有 panic。「可能沒有」寫在回傳型別 `Option` 裡，就是第 6 章第 6 集學的事。倒數第二行的 `orders.pop();` 沒有接住回傳值，這樣寫沒問題。

### `while let`：一直 `pop` 到空

`if let` 只判斷一次，`while let` 是同一個寫法加上迴圈：每一圈都比對一次，比對得上就執行區塊，比對不上（這裡是 `None`）就結束迴圈。

```rust
fn main() {
    let mut todo = vec![String::from("洗衣服"), String::from("寫程式")];
    todo.push(String::from("吃飯"));
    while let Some(task) = todo.pop() {
        println!("處理：{task}");
    }
    println!("剩下 {} 件", todo.len());
}
```

執行結果：

```text
處理：吃飯
處理：寫程式
處理：洗衣服
剩下 0 件
```

- `Some(task)` 是模式，每一圈 `todo.pop()` 回傳 `Some` 就把裡面的 `String` 綁給 `task`，回傳 `None` 就離開迴圈。
- `pop` 從尾端拿，所以處理的順序跟放進去的順序相反：最後放的最先處理。這種「後進先出」的結構叫堆疊（stack），寫演算法時很常用。
- 沒有 `while let` 的話，要寫成 `loop` 加 `match`，遇到 `None` 就 `break`；`while let` 把這三件事收成一行。

### 跟 Go 比一比

Go 的 `append` 回傳新的 slice，一定要接回來；取尾端的元素也要自己算索引、自己切掉：

```go
v := []int{30, 45}
v = append(v, 38)   // 忘了寫 v = 就等於沒加
last := v[len(v)-1] // 空的 slice 會 panic
v = v[:len(v)-1]
fmt.Println(last, v) // 38 [30 45]
```

Rust 的 `push` 直接改動 `Vec`，不用接回來；`pop` 一次完成「取出」和「刪掉」，空的時候回傳 `None`，不會 panic。

## 重點整理

- `v.push(x)` 在尾端加一個元素；`Vec` 要用 `let mut` 建立。
- `v.pop()` 拿走最後一個元素，回傳 `Option<T>`：有元素是 `Some(值)`，空的是 `None`，不會 panic。
- `while let Some(x) = v.pop() { ... }` 一路 `pop` 到空為止；`while let` 是會重複的 `if let`，模式比對不上就結束。
- 從尾端放、從尾端拿，順序是後進先出（堆疊）。
- 跟 Go 比：Go 的 `append` 要接回變數，取尾端要自己算索引；Rust 的 `push`、`pop` 直接改動 `Vec`。
