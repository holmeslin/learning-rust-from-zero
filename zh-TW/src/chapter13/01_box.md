# `Box<T>`

## 本集目標

知道 `Box<T>` 是把值放進 heap 的智慧指標，會用 `Box::new` 與 `*`，也看懂前面幾章先照抄的 `Box<dyn Trait>`。

## 正文

### 把值放進 heap

第 3 章說過，整數放 stack，`String` 的文字內容放 heap。想自己決定「這個值要放 heap」，就用 `Box::new`：

```rust
fn main() {
    let price = Box::new(30);
    println!("{price}");
    let total = *price + 5;
    println!("{total}");
}
```

執行結果：

```text
30
35
```

- `Box::new(30)` 把 `30` 放到 heap 上，`price` 的型別是 `Box<i32>`，讀成「裝著 `i32` 的 Box」。
- stack 上只剩一個指向 heap 的指標：

```text
stack              heap
price: [ ptr ] ---> 30
```

- `{price}` 直接印得出 `30`，因為 `Box<T>` 印出來的就是裡面的值。
- 要拿到裡面的值，用第 4 章的 `*`：`*price` 就是那個 `30`。

`Box` 是**智慧指標**（smart pointer）：用起來像參照，但它**擁有**指向的資料，還多了一些本事，例如離開作用域時自動釋放 heap 上的那一塊。`Box<i32>` 不是 `i32`，所以不能直接拿來算數：

```rust,compile_fail=E0369
fn main() {
    let price = Box::new(30);
    let total = price + 5;
    println!("{total}");
}
```

編譯錯誤：

```text
error[E0369]: cannot add `{integer}` to `Box<{integer}>`
 --> src/main.rs:3:23
  |
3 |     let total = price + 5;
  |                 ----- ^ - {integer}
  |                 |
  |                 Box<{integer}>
  |
```

沒有「`Box<i32>` 加整數」這種加法，所以是 E0369。這裡只貼前半段，後面的 `note` 指向標準函式庫內部的檔案路徑，先不用管。

### Box 擁有它指向的值

所有權的規則跟 `String` 一模一樣：`let b = a;` 之後擁有者換成 `b`，`a` 不能再用（E0382）；heap 上那一塊，只會在最後的擁有者離開作用域時釋放一次。

### 前面照抄的 `Box<dyn Trait>`

第 8 章第 15 集、第 9 章第 10 集、第 11 章第 6 集，你都照抄過 `Box<dyn ...>`，現在可以解釋了。`dyn Priced` 的大小不固定：`Tea` 和 `Coffee` 的大小可以不一樣，所以它不能直接放進 `Vec`，也不能當變數。`Box` 的大小固定，永遠只是一個指標，所以 `Box<dyn Priced>` 放得進去：

```rust
struct Tea;

struct Coffee;

trait Priced {
    fn price(&self) -> u32;
}

impl Priced for Tea {
    fn price(&self) -> u32 {
        35
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        45
    }
}

fn main() {
    let mut menu: Vec<Box<dyn Priced>> = Vec::new();
    menu.push(Box::new(Tea));
    menu.push(Box::new(Coffee));
    let mut total = 0;
    for item in &menu {
        total += item.price();
    }
    println!("共 {total} 元");
}
```

執行結果：

```text
共 80 元
```

- `Box::new(Tea)` 把 `Tea` 放到 heap，再交給 `Vec<Box<dyn Priced>>`：每個元素都是指標，大小一樣，指向的東西可以不同。
- `item.price()` 沒有寫 `*`：用 `.` 呼叫方法時，編譯器會穿過 `Box` 找到裡面的值（原理在第 3 集）。
- 第 9 章的 `Box<dyn std::error::Error>`、第 11 章的 `Box<dyn Fn(u32) -> u32>`，都是同一個道理：把「型別在執行時期才知道」的東西，用固定大小的 `Box` 裝起來。

`Box` 最常見的用途有三個：裝 trait 物件（這一集）、讓遞迴型別有固定大小（下一集），以及值很大、又要搬來搬去時，只搬指標、不搬整份資料。

### 跟 Go 比一比

Go 的指標可以隨便複製，每一份都指向同一塊記憶體：

```go
p := new(int)
*p = 30
q := p // p 與 q 指向同一塊，都能讀寫
fmt.Println(*p, *q)
```

`Box` 看起來像 `new`，規則卻不一樣：它是**唯一**的擁有者，`let q = price;` 之後 `price` 就不能用了，也不可能是 nil（沒有值要寫 `Option<Box<T>>`）。想讓好幾個變數同時擁有同一份資料，要等第 5 集的 `Rc`。

## 重點整理

- `Box::new(值)` 把值放進 heap，變數本身只是 stack 上的一個指標；型別 `Box<T>` 讀成「裝著 `T` 的 Box」。
- `Box` 是智慧指標：擁有指向的資料，離開作用域時釋放；用 `*` 取出裡面的值，`Box<i32>` 不能直接拿來算數（E0369）。
- 所有權規則照舊：`let b = a;` 會 move（E0382），heap 上的資料只釋放一次。
- `Box<dyn Trait>` 就是這個 `Box`：大小固定，所以能放進 `Vec`、當回傳值；前面照抄的 `Box<dyn Error>`、`Box<dyn Fn(...)>` 同理。
- 跟 Go 比：Go 的指標可以複製、可以是 nil；`Box` 只有一個擁有者，不會是 nil。
