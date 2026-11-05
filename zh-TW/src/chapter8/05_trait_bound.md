# trait bound `<T: Trait>`

## 本集目標

用 `fn f<T: Priced>(item: &T)` 替型別參數加上 trait 條件；知道它跟 `impl Trait` 參數的差別：可以要求兩個參數是同一個型別。

## 正文

### 另一種寫法

上一集的 `fn print_price(item: &impl Priced)`，還有另一種寫法：

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

fn print_price<T: Priced>(item: &T) {
    println!("{} 元", item.price());
}

fn pair_price<T: Priced>(a: &T, b: &T) -> u32 {
    a.price() + b.price()
}

fn main() {
    let tea = Tea { toppings: 4 };
    let other_tea = Tea { toppings: 0 };
    let coffee = Coffee { shots: 1 };
    print_price(&tea);
    print_price(&coffee);
    println!("{}", pair_price(&tea, &other_tea));
}
```

執行結果：

```text
55 元
55 元
90
```

- `<T: Priced>` 宣告一個**型別參數** `T`，後面的 `: Priced` 是條件，叫 **trait bound**：`T` 一定要實作 `Priced`。`T` 只是名字，習慣用單一的大寫字母。
- 參數 `item: &T` 讀成「`T` 的參照」。`print_price` 的效果跟上一集的 `&impl Priced` 一樣：上一集的寫法，其實是這個寫法的簡寫。
- 型別參數是第 8 集的主題，這裡先把 `<T: Priced>` 讀成「某個實作了 `Priced` 的型別，叫它 `T`」。Go 的 `func PrintPrice[T Pricer](item T)` 就是同樣的東西。

### 兩個參數要同一個型別

差別在於 `T` 是名字，可以用好幾次。`pair_price` 的兩個參數都寫 `&T`，表示它們必須是**同一個**型別：兩杯奶茶可以加，一杯奶茶加一杯咖啡不行。

上一集的 `same_price(a: &impl Priced, b: &impl Priced)` 是兩個各自獨立的條件，傳不同型別沒問題。要求同型別，就只能寫成 `<T: Priced>`。把奶茶跟咖啡傳給 `pair_price` 看看：

```rust,compile_fail=E0308
struct Tea;

struct Coffee;

trait Priced {
    fn price(&self) -> u32;
}

impl Priced for Tea {
    fn price(&self) -> u32 {
        45
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        55
    }
}

fn pair_price<T: Priced>(a: &T, b: &T) -> u32 {
    a.price() + b.price()
}

fn main() {
    let tea = Tea;
    let coffee = Coffee;
    println!("{}", pair_price(&tea, &coffee));
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
  --> src/main.rs:28:37
   |
28 |     println!("{}", pair_price(&tea, &coffee));
   |                    ----------       ^^^^^^^ expected `&Tea`, found `&Coffee`
   |                    |
   |                    arguments to this function are incorrect
   |
   = note: expected reference `&Tea`
              found reference `&Coffee`
```

`T` 被第一個參數 `&tea` 定成 `Tea`，第二個參數就必須也是 `&Tea`，傳 `&Coffee` 是 E0308。注意兩個型別都有實作 `Priced`，錯的不是 trait 條件，而是兩個型別不一樣。訊息只貼前半段，後面還有一則 `note`，指出 `pair_price` 定義在哪裡。

### 該用哪一種

- 只是要「收任何實作了 `Priced` 的東西」，用 `impl Priced`，比較短。
- 需要說「這兩個是同一個型別」，或之後要在別的地方（例如回傳值）用到這個型別，才寫 `<T: Priced>`。

兩種寫法在編譯後沒有差別。

## 重點整理

- `fn f<T: Priced>(item: &T)`：`T` 是型別參數，`: Priced` 是 trait bound，條件是 `T` 要實作 `Priced`。
- 上一集的 `impl Priced` 參數是這種寫法的簡寫，效果相同。
- `T` 可以用很多次：`a: &T, b: &T` 要求兩個參數是同一個型別，不同型別是 E0308。
- 只需要「任何實作了 trait 的東西」時用 `impl Trait`；要表達「同一個型別」才用 `<T: Trait>`。
