# trait 物件 `dyn Trait`

## 本集目標

用 `&dyn Trait` 與 `Box<dyn Trait>` 把不同型別放進同一個陣列、同一個回傳值，並知道它跟 `impl Trait`、泛型的差別：型別在執行時期才決定。

## 正文

### 不同型別，放在一起

前面幾集的 `impl Trait` 和 `<T: Trait>`，每一次出現都對應**一個**固定的型別：第 7 集的函式不能一下回傳 `Tea`、一下回傳 `Coffee`，第 9 集的 `Shelf<T>` 也只能放一種 `T`。想打破這個限制，要用 `dyn Trait`：

```rust
struct Tea {
    toppings: u32,
}

struct Coffee {
    shots: u32,
}

trait Priced {
    fn price(&self) -> u32;
}

impl Priced for Tea {
    fn price(&self) -> u32 {
        35 + self.toppings * 5
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        45 + self.shots * 10
    }
}

fn print_price(item: &dyn Priced) {
    println!("{} 元", item.price());
}

fn pick(want_tea: bool) -> Box<dyn Priced> {
    if want_tea {
        Box::new(Tea { toppings: 0 })
    } else {
        Box::new(Coffee { shots: 1 })
    }
}

fn main() {
    let tea = Tea { toppings: 2 };
    let coffee = Coffee { shots: 1 };
    let order: [&dyn Priced; 2] = [&tea, &coffee];
    let mut total = 0;
    for item in order {
        total += item.price();
    }
    println!("共 {} 元", total);
    print_price(&tea);

    println!("{} 元", pick(false).price());
    println!("{} 元", pick(true).price());
}
```

執行結果：

```text
共 100 元
45 元
55 元
35 元
```

- `&dyn Priced` 讀成「某個實作了 `Priced` 的東西的參照」，**是哪個型別，等到執行時期才知道**。`dyn` 是 dynamic（動態）的縮寫，這樣的東西叫 **trait 物件**（trait object）。
- `[&dyn Priced; 2]` 是陣列，兩個元素的實際型別不同：一個 `Tea`、一個 `Coffee`。因為元素的型別都是 `&dyn Priced`，所以放得進同一個陣列，也能用同一個 `for` 走訪。
- `print_price(item: &dyn Priced)` 跟第 4 集的 `&impl Priced` 看起來很像，差別在後面的原理。
- `pick` 回傳 `Box<dyn Priced>`，解決了第 7 集的 E0308：兩個分支回傳不同的型別，卻都是 `Box<dyn Priced>`。`Box::new(...)` 先照抄，`Box` 第 13 章才正式介紹；現在只要知道，它把值放到 heap 上，外面包一層，讓大小固定；對 `Box` 用 `.` 呼叫方法（`pick(false).price()`）也行，原理一樣第 13 章說明。第 7 集那個錯誤訊息建議的 `Box<dyn Priced>`，就是這個寫法。
- `dyn Priced` 本身不能單獨當參數寫，`item: dyn Priced` 是 E0277（大小在編譯時期不知道）：`Tea` 和 `Coffee` 的大小不一樣，所以一定要放在 `&` 或 `Box` 後面。

### 編譯時期決定 vs 執行時期決定

兩種做法的差別：

- `impl Priced`、`<T: Priced>`：編譯器替每一種實際用到的型別各編一份函式，呼叫哪個方法在編譯時期就決定了，執行時沒有額外的成本。但是每次只能是一種型別。
- `dyn Priced`：函式只有一份，呼叫 `item.price()` 時，才從物件身上查出該執行哪個型別的方法。多了一點點查表的成本，換來「同一個位置，可以是不同型別」的彈性。

大部分時候用 `impl Trait` 或 `<T: Trait>` 就夠了，需要把不同型別放在一起、或依條件回傳不同型別時，才改用 `dyn`。不是每個 trait 都能做成 `dyn`（例如方法會回傳自己這個型別的 trait），細節這裡不展開。

### 跟 Go 比一比

Go 的介面值天生就是動態的：

```go
items := []Pricer{Tea{Toppings: 2}, Coffee{Shots: 1}}
for _, item := range items {
	fmt.Println(item.Price())
}
```

只要變數的型別是介面，裡面放什麼都行，不用任何標記。Rust 預設是編譯時期決定的，要動態就得自己寫 `dyn`，而且要放在 `&` 或 `Box` 後面。Go 的直覺是「介面就是動態」，在 Rust 要記得：`impl Priced` 看起來像介面，卻不是動態的。

## 重點整理

- `&dyn Trait`、`Box<dyn Trait>` 是 trait 物件：實際型別到執行時期才知道，所以不同型別可以放在同一個陣列、同一個回傳值。
- `[&dyn Priced; 2]` 能放 `&tea` 和 `&coffee`；`-> Box<dyn Priced>` 能依條件回傳不同型別（`Box::new(...)` 先照抄，`Box` 第 13 章）。
- `dyn Trait` 一定要放在 `&` 或 `Box` 後面，直接寫 `item: dyn Priced` 是 E0277。
- `impl Trait` / `<T: Trait>` 是編譯時期決定，沒有額外成本，每次一種型別；`dyn Trait` 是執行時期查表，可以混放。
- 跟 Go 比：Go 的介面值天生動態；Rust 的 `impl Trait` 不是，要用 `dyn Trait`。
