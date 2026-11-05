# 參數用 `impl Trait`

## 本集目標

用 `item: &impl Priced` 寫函式，讓任何實作了這個 trait 的型別都能傳進來，並知道函式裡面只能用 trait 提供的方法。

## 正文

### 收任何實作了 trait 的型別

前三集的 trait 都是一個型別一個型別地呼叫。真正的用處在這裡：寫一個函式，不管傳進來的是什麼，只要實作了 `Priced` 就收。做法是把參數的型別寫成 `impl Priced`：

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

fn print_price(item: &impl Priced) {
    println!("{} 元", item.price());
}

fn same_price(a: &impl Priced, b: &impl Priced) -> bool {
    a.price() == b.price()
}

fn main() {
    let tea = Tea { toppings: 4 };
    let coffee = Coffee { shots: 1 };
    print_price(&tea);
    print_price(&coffee);
    println!("{}", same_price(&tea, &coffee));
}
```

執行結果：

```text
55 元
55 元
true
```

- `&impl Priced` 讀成「任何實作了 `Priced` 的型別的參照」。`&` 是第 4 章的借用；寫成 `item: impl Priced`（沒有 `&`）就是把值 move 進函式，規則跟第 3 章一樣。
- `print_price(&tea)` 和 `print_price(&coffee)` 呼叫的是同一個函式，傳進去的型別不同。傳什麼型別，是編譯時期就決定好的。
- `same_price` 的兩個參數各自寫了一次 `impl Priced`，所以可以是不同的型別：奶茶（加四種料）跟咖啡（單份濃縮），都是 55 元。

這跟 Go 的 `func printPrice(p Pricer)` 做的事情一樣：函式不挑型別，只挑「能做什麼」。差別在於 Rust 的型別要先明確實作過 trait（第 2 集）。

### 函式裡只能用 trait 保證的東西

函式不知道 `item` 到底是哪個型別，只知道它實作了 `Priced`。所以裡面只能呼叫 `Priced` 的方法，不能讀欄位：

```rust,compile_fail=E0609
struct Coffee {
    shots: u32,
}

trait Priced {
    fn price(&self) -> u32;
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        45 + self.shots * 10
    }
}

fn print_shots(item: &impl Priced) {
    println!("{} 份濃縮", item.shots);
}

fn main() {
    let coffee = Coffee { shots: 1 };
    print_shots(&coffee);
}
```

編譯錯誤：

```text
error[E0609]: no field `shots` on type `&impl Priced`
  --> src/main.rs:16:29
   |
16 |     println!("{} 份濃縮", item.shots);
   |                                ^^^^^ unknown field
```

`` no field `shots` on type `&impl Priced` ``：換成 `Tea` 傳進來，就沒有 `shots` 這個欄位。這跟上一集預設方法不能讀欄位是同一個道理。想用的東西，就加進 trait 的方法裡，讓每個型別各自提供。

## 重點整理

- `fn f(item: &impl Priced)`：接受任何實作了 `Priced` 的型別的參照；沒有 `&` 的 `impl Priced` 是 move 進來。
- 傳進同一個函式的可以是不同型別；每個 `impl Priced` 參數各自獨立，兩個參數可以是不同型別。
- 函式裡只能用 trait 提供的方法，不能讀欄位（E0609）。
- 傳進去的型別沒有實作該 trait，是 E0277（第 2 集）。
