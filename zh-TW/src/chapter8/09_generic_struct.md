# 泛型 struct 與 `impl<T>`

## 本集目標

寫出自己的泛型 struct，用 `impl<T>` 替它加方法，並知道 `impl` 還能只替「符合條件的 `T`」或「某個特定型別」加方法。

## 正文

### 欄位的型別也能是 `T`

上一集的 `Option<T>`、`Vec<T>` 是別人寫好的泛型型別。自己寫的做法一樣：在名字後面宣告 `<T>`，欄位就能用 `T`。下面的 `Shelf` 是貨架，不管放什麼都行：

```rust
struct Tea {
    toppings: u32,
}

trait Priced {
    fn price(&self) -> u32;
}

impl Priced for Tea {
    fn price(&self) -> u32 {
        35 + self.toppings * 5
    }
}

struct Shelf<T> {
    items: Vec<T>,
}

impl<T> Shelf<T> {
    fn new() -> Shelf<T> {
        Shelf { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn count(&self) -> usize {
        self.items.len()
    }
}

impl<T: Priced> Shelf<T> {
    fn total(&self) -> u32 {
        let mut sum = 0;
        for item in &self.items {
            sum += item.price();
        }
        sum
    }
}

fn main() {
    let mut names: Shelf<String> = Shelf::new();
    names.add(String::from("紅茶"));
    names.add(String::from("咖啡"));
    println!("{}", names.count());

    let mut teas = Shelf::new();
    teas.add(Tea { toppings: 2 });
    teas.add(Tea { toppings: 0 });
    println!("{} 杯，共 {} 元", teas.count(), teas.total());
}
```

執行結果：

```text
2
2 杯，共 80 元
```

- `struct Shelf<T>`：`<T>` 宣告型別參數，欄位 `items: Vec<T>` 用它。`Shelf<String>` 放字串，`Shelf<Tea>` 放奶茶。
- `impl<T> Shelf<T>`：這裡有**兩個** `T`。`impl` 後面的 `<T>` 是宣告「接下來的 `T` 是型別參數」，`Shelf<T>` 才是真正要實作的型別。漏掉前面的 `<T>`，編譯器會把 `T` 當成一個叫 `T` 的具體型別，找不到而報錯。
- `new` 的回傳型別要寫完整的 `Shelf<T>`；`add` 的參數 `item: T` 就是「放進去的東西」。
- `Shelf::new()` 沒說 `T` 是什麼，要靠後面的使用推出來：`teas` 因為 `add` 了 `Tea`，編譯器就知道是 `Shelf<Tea>`。`names` 沒有線索的話，就像第 6 章的 `None`，要自己標型別。

### 條件放在 `impl` 上

第二個區塊是 `impl<T: Priced> Shelf<T>`：只有 `T` 實作了 `Priced` 的貨架，才有 `total` 方法。`teas` 是 `Shelf<Tea>`，`Tea` 有 `Priced`，所以呼叫得到；`names.total()` 就不行，編譯器會說 `` the method `total` exists for struct `Shelf<String>`, but its trait bounds were not satisfied ``。

這就是第 5 章第 11 集說的「多個 `impl` 區塊」的用途：同一個 struct，每個區塊的條件不一樣，方法也就不一樣。

還有一種：條件寫成具體的型別，例如 `impl Shelf<Tea> { ... }`，這個區塊的方法只有 `Shelf<Tea>` 才有，這時候 `impl` 後面不用寫 `<T>`，因為已經沒有型別參數了。

### 同一個貨架只能放同一種 `T`

`Shelf<T>` 的 `T` 一旦定下來，就不能變。放進去第一個值是 `String`，接著放整數就是 E0308：

```rust,compile_fail=E0308
struct Shelf<T> {
    items: Vec<T>,
}

impl<T> Shelf<T> {
    fn new() -> Shelf<T> {
        Shelf { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn count(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut shelf = Shelf::new();
    shelf.add(String::from("紅茶"));
    shelf.add(45);
    println!("{}", shelf.count());
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
  --> src/main.rs:22:15
   |
21 |     shelf.add(String::from("紅茶"));
   |     -----     -------------------- this argument has type `String`...
   |     |
   |     ... which causes `shelf` to have type `Shelf<String>`
22 |     shelf.add(45);
   |           --- ^^ expected `String`, found integer
   |           |
   |           arguments to this method are incorrect
```

訊息先指出第一次 `add` 讓 `shelf` 變成 `Shelf<String>`，第二次就不能放整數。這裡只貼前半段，後面的 `help` 建議 `.to_string()`，第 11 集才會學，不用照做。

想在同一個貨架放不同型別的東西，要用 `dyn Trait`，第 15 集再說。

## 重點整理

- `struct Shelf<T> { items: Vec<T> }`：在 struct 名字後面宣告 `<T>`，欄位就能用 `T`。
- `impl<T> Shelf<T> { ... }`：`impl` 後面的 `<T>` 是宣告，`Shelf<T>` 才是被實作的型別；方法的簽名要寫完整的 `Shelf<T>`。
- `impl<T: Priced> Shelf<T>`：只有符合條件的 `T`，貨架才有這些方法；`impl Shelf<Tea>` 則只有 `Shelf<Tea>` 有。
- 一個 `Shelf<T>` 的 `T` 只有一種，混放不同型別是 E0308。
