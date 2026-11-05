# `derive` 是什麼

## 本集目標

知道 `#[derive(...)]` 是請編譯器代寫 trait 的實作，並認得 `Debug`、`Clone`、`Copy`、`PartialEq`、`Eq`、`Hash`、`Default` 各自讓型別多了什麼能力。

## 正文

### `derive` 是請編譯器代寫 `impl`

第 5 章第 6 集的 `#[derive(Debug)]` 是先照抄的固定句型，現在揭曉：`derive` 是請編譯器依照欄位，自動寫出 `impl Debug for Member { ... }`。一行 `#[derive(...)]` 可以列好幾個 trait，用逗號隔開。

條件只有一個：每個欄位的型別，也要實作同一個 trait。

| trait | 讓型別能做什麼 | 要注意 |
| --- | --- | --- |
| `Debug` | `{:?}`、`{:#?}` | |
| `Clone` | `.clone()` 複製出一份新的 | 第 3 章的 `.clone()` 就是它 |
| `Copy` | `let b = a;` 之後 `a` 還能用 | 要同時 `Clone`；欄位全部是 `Copy` |
| `PartialEq` | `==`、`!=` | 第 6 章第 3 集 E0369 建議的就是它 |
| `Eq` | 宣告相等關係沒有例外 | 要同時 `PartialEq`；`f64` 沒有 |
| `Hash` | 能算出雜湊值 | `HashMap` 的鍵要 `Eq` 加 `Hash` |
| `Default` | `型別名稱::default()`，每個欄位取預設值 | 欄位都要有 `Default` |

七個放在一起用：

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Order {
    drink: String,
    cups: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coupon {
    off: u32,
}

fn main() {
    let a = Order {
        drink: String::from("紅茶"),
        cups: 2,
    };
    let b = a.clone();
    println!("{:?}", b);
    println!("{}", a == b);
    println!("{:?}", Order::default());

    let mut served: HashMap<Order, u32> = HashMap::new();
    served.insert(a, 1);
    println!("{:?}", served.get(&b));

    let c = Coupon { off: 10 };
    let d = c;
    println!("{:?} {:?} {}", c, d, c == d);
}
```

執行結果：

```text
Order { drink: "紅茶", cups: 2 }
true
Order { drink: "", cups: 0 }
Some(1)
Coupon { off: 10 } Coupon { off: 10 } true
```

- `a.clone()` 複製出整個 `Order`，連 `String` 也複製了一份，所以 `a` 和 `b` 是兩個獨立的值。`==` 逐欄位比較，所以是 `true`。
- `Order::default()` 的每個欄位取各自型別的預設：`String` 是空字串，`u32` 是 0。第 1 章說過 Rust 沒有零值，需要的話用 `Default` 明確要一個。
- `HashMap<Order, u32>` 能把 `Order` 當鍵，是因為 `Order` 有 `Eq` 和 `Hash`。第 7 章第 11 集說的「鍵要能比較相等、能算雜湊值」就是這兩個 trait。`f64` 沒有 `Eq`（NaN 不等於自己），所以也不能當鍵。
- `let d = c;` 之後 `c` 還能用，因為 `Coupon` 是 `Copy`：賦值時複製而不是 move。第 3 章第 8 集說 `Copy` 是 trait，自己的 struct 想要，就像這樣 `derive`。
- `Clone` 和 `Copy` 的差別：`.clone()` 是你明確呼叫、可能很花時間的複製；`Copy` 是賦值時自動發生的複製，所以只有便宜的型別才適合。

### `Copy` 的條件

`Order` 有 `String`，沒辦法 `Copy`（`String` 擁有 heap 上的資料）。硬加上 `Copy` 會是 E0204：

```rust,compile_fail=E0204
#[derive(Clone, Copy)]
struct Order {
    drink: String,
    cups: u32,
}

fn main() {
    let a = Order {
        drink: String::from("紅茶"),
        cups: 2,
    };
    let b = a;
    println!("{} {}", a.cups, b.cups);
}
```

編譯錯誤：

```text
error[E0204]: the trait `Copy` cannot be implemented for this type
 --> src/main.rs:2:8
  |
1 | #[derive(Clone, Copy)]
  |                 ---- in this derive macro expansion
2 | struct Order {
  |        ^^^^^
3 |     drink: String,
  |     ------------- this field does not implement `Copy`
```

訊息指出是 `drink` 這個欄位沒有實作 `Copy`。欄位全是整數、`bool`、`char` 這類 `Copy` 型別的 struct，才能 `derive(Copy)`。

### 跟 Go 比一比

Go 的 struct 預設就有這些能力，不用宣告：

```go
type Order struct {
	Drink string
	Cups  int
}

func main() {
	a := Order{Drink: "紅茶", Cups: 2}
	b := a
	fmt.Println(a == b)
	var zero Order
	served := map[Order]int{a: 1}
	fmt.Println(zero, served[b])
}
```

賦值會複製、`==` 可以比、有零值、能當 map 的鍵，全部自動。Rust 每一項都要 `derive` 才有，沒有 `derive` 的 struct，賦值是 move（第 3 章）、不能 `==`、不能 `{:?}`。換來的好處是：只有你明確要的能力才存在。

## 重點整理

- `#[derive(...)]` 請編譯器替型別寫出 trait 的實作；每個欄位的型別都要有該 trait。
- `Debug`（`{:?}`）、`Clone`（`.clone()`）、`PartialEq`（`==`）、`Default`（`型別::default()`）。
- `Copy`：賦值時複製，要同時 `Clone`，欄位必須全是 `Copy`；有 `String` 欄位是 E0204。
- `Eq` 是 `PartialEq` 的加強版（`f64` 沒有）；`Eq` 加 `Hash` 才能當 `HashMap` 的鍵。
- 跟 Go 比：Go 的 struct 自動能複製、`==`、有零值、能當 map 的鍵；Rust 全部要 `derive`。
