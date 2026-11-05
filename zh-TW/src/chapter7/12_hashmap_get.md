# `get` 回傳 `Option`

## 本集目標

用 `get` 讀 `HashMap` 的值：回傳 `Option<&V>`，用 `match` 或 `if let` 處理有沒有；需要修改時用 `get_mut`。

## 正文

### `get`

`get(&鍵)` 找得到回傳 `Some(值的參照)`，找不到回傳 `None`。跟 `Vec` 的 `get` 同一個設計（第 3 集），鍵是 `String` 時直接傳 `&str` 就行：

```rust
use std::collections::HashMap;

fn main() {
    let mut prices: HashMap<String, u32> = HashMap::new();
    prices.insert(String::from("紅茶"), 30);
    prices.insert(String::from("咖啡"), 45);

    println!("{:?}", prices.get("紅茶"));
    println!("{:?}", prices.get("奶茶"));

    match prices.get("咖啡") {
        Some(price) => println!("咖啡 {price} 元"),
        None => println!("沒有咖啡"),
    }
    if let Some(price) = prices.get("奶茶") {
        println!("奶茶 {price} 元");
    } else {
        println!("沒有奶茶");
    }
}
```

執行結果：

```text
Some(30)
None
咖啡 45 元
沒有奶茶
```

- 回傳的是 `Option<&u32>`，所以 `Some(price)` 裡的 `price` 是 `&u32`，是借來的，`HashMap` 還擁有它。跟第 6 章第 16 集一樣：印出、做算術可以直接用，比較大小才需要 `*price`。
- 處理 `Option` 的方法就是第 6 章學的：`match`、`if let`。

### `get_mut`：改裡面的值

`get_mut` 回傳 `Option<&mut V>`，借出可以改的參照，改值要用 `*`（第 4 章第 7 集）。`HashMap` 要用 `let mut`：

```rust
use std::collections::HashMap;

fn main() {
    let mut prices: HashMap<String, u32> = HashMap::new();
    prices.insert(String::from("紅茶"), 30);
    if let Some(price) = prices.get_mut("紅茶") {
        *price += 5;
    }
    println!("{:?}", prices.get("紅茶"));
}
```

執行結果：

```text
Some(35)
```

`insert` 也能換值，但它要重新給整個值；`get_mut` 可以依照舊值算出新值。

### 還有 `map[鍵]` 的寫法

`prices["紅茶"]` 也能讀值，直接拿到值本身，不是 `Option`。但鍵不存在時它會 panic，所以只有確定鍵一定存在才這樣寫。平常用 `get`，由編譯器要求你處理沒有這個鍵的情況。

### 跟 Go 比一比

Go 讀 map 有兩種寫法，編譯器都不會要你檢查：

```go
prices := map[string]int{"紅茶": 30}
a := prices["奶茶"]     // 0：分不出是沒有這個鍵，還是價格剛好是 0
b, ok := prices["奶茶"] // ok 是 false，但不檢查 ok 也能編譯
if ok {
	fmt.Println(b)
}
fmt.Println(a)
```

- 只寫一個變數，找不到就拿到零值，跟「真的是 0」分不出來。Rust 沒有零值，找不到就是 `None`。
- `v, ok :=` 要自己記得檢查 `ok`；Rust 的 `get` 回傳 `Option`，不處理 `None` 就拿不到值，檢查是型別的一部分。

## 重點整理

- `map.get(&鍵)` 回傳 `Option<&V>`：找到是 `Some(值的參照)`，找不到是 `None`；鍵是 `String` 時可傳 `&str`。
- 用 `match` 或 `if let` 處理；`Some` 裡綁到的是參照，比較大小要 `*`。
- `map.get_mut(&鍵)` 回傳 `Option<&mut V>`，用 `*值 += ...` 修改；`map` 要 `let mut`。
- `map[鍵]` 會直接回傳值，但鍵不存在時 panic；一般用 `get`。
- 跟 Go 比：Go 的 `m[k]` 找不到給零值、`v, ok` 要自己檢查；Rust 的 `get` 把「可能沒有」放進 `Option`，不處理就不能用。
