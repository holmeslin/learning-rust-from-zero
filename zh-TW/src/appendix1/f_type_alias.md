# 型別別名 `type`

## 本集目標

用 `type 名字 = 型別;` 替既有的型別取別名，並分清楚別名跟第 5 章 tuple struct（newtype）的差別。

## 正文

### 替長型別取個短名字

`type` 替一個型別取另一個名字。第 7 章的 `HashMap<String, u32>` 在簽名裡出現好幾次的話，取個別名會短很多：

```rust
use std::collections::HashMap;

type Menu = HashMap<String, u32>;

fn price_of(menu: &Menu, name: &str) -> u32 {
    match menu.get(name) {
        Some(p) => *p,
        None => 0,
    }
}

fn main() {
    let mut menu = Menu::new();
    menu.insert(String::from("紅茶"), 30);
    menu.insert(String::from("咖啡"), 45);
    println!("{}", price_of(&menu, "咖啡"));
    println!("{}", price_of(&menu, "果汁"));
}
```

執行結果：

```text
45
0
```

`Menu` 跟 `HashMap<String, u32>` 是同一個型別，用在哪裡都行，連 `Menu::new()` 這種呼叫也可以。

### 別名不是新型別

別名只是另一個名字，不會產生新的型別，所以編譯器不會替你分辨它們。`Meters` 和 `Seconds` 都是 `f64` 的別名，把兩個參數放反了照樣編譯成功：

```rust
type Meters = f64;
type Seconds = f64;

fn speed(distance: Meters, time: Seconds) -> f64 {
    distance / time
}

fn main() {
    let d: Meters = 100.0;
    let t: Seconds = 20.0;
    println!("{}", speed(t, d));
}
```

執行結果：

```text
0.2
```

正確答案是 5，放反了得到 0.2，沒有任何警告。要讓編譯器攔下這種錯誤，要用第 5 章第 5 集的 tuple struct：`struct Meters(f64);` 跟 `struct Seconds(f64);` 是兩個不同的型別，放反是 E0308。

編譯錯誤訊息裡也只看得到原本的型別：把 `type Price = u32;` 的 `Price` 拿去給 `i64` 用，訊息寫的是 `expected i64, found u32`，不會出現 `Price`。

### 帶型別參數的別名

別名可以有型別參數。第 9 章的函式常寫 `Result<u32, OrderError>`，整個程式的錯誤型別都一樣時，可以把錯誤那一半固定下來：

```rust
#[derive(Debug)]
enum OrderError {
    OutOfStock,
}

type Result<T> = std::result::Result<T, OrderError>;

fn take(stock: u32) -> Result<u32> {
    if stock == 0 {
        return Err(OrderError::OutOfStock);
    }
    Ok(stock - 1)
}

fn main() {
    println!("{:?}", take(3));
    println!("{:?}", take(0));
}
```

執行結果：

```text
Ok(2)
Err(OutOfStock)
```

這個 `Result<T>` 蓋住了預設的 `Result`，所以右邊要寫完整路徑 `std::result::Result`。標準函式庫也這樣做：第 8 章 `Display` 骨架裡的 `std::fmt::Result`，就是 `Result<(), std::fmt::Error>` 的別名；第 9 章 `read_line` 的 `Result<usize, std::io::Error>`，也能寫成 `std::io::Result<usize>`。

另外 `impl` 裡的 `type Item = u32;`（第 8 章第 17 集）是關聯型別，跟這集的別名只是關鍵字相同，用途不一樣。

### 跟 Go 比一比

Go 有兩種寫法，行為不同：

```go
type Meters float64 // 新型別：不能跟 float64 混用
type Price = uint32 // 別名：就是 uint32
```

Rust 的 `type Meters = f64;` 對應的是第二種，只是別名。想要 Go 第一種「新型別」的效果，Rust 要用 tuple struct。

## 重點整理

- `type 名字 = 型別;` 替型別取別名，常用來縮短長型別，例如 `type Menu = HashMap<String, u32>;`。
- 別名跟原本的型別完全相同，可以互相取代，不會產生新型別；放反兩個同型別的參數，編譯器不會發現。
- 要讓編譯器分辨，用 tuple struct（newtype）；錯誤訊息裡只會出現原本的型別名稱。
- 別名可以帶型別參數：`type Result<T> = std::result::Result<T, MyError>;`；`std::fmt::Result`、`std::io::Result<T>` 都是這種別名。
- Go 的 `type X Y` 是新型別，Rust 的 `type X = Y;` 對應 Go 的 `type X = Y`。
