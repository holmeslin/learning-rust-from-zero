# 孤兒規則

## 本集目標

知道 `impl 某trait for 某型別` 的兩邊至少要有一邊是自己寫的（孤兒規則，E0117），並學會用 newtype 包一層來繞過它。

## 正文

### 兩邊至少要有一邊是自己的

目前實作過的 trait，不是自己定義的（第 1～7 集的 `Priced`），就是標準函式庫的 trait，配上自己的型別（第 11 集的 `Display for Tea`）。還有一種情形：替標準函式庫的型別，實作自己的 trait。這也可以：

```rust
trait Label {
    fn label(&self) -> String;
}

impl Label for u32 {
    fn label(&self) -> String {
        format!("{} 元", self)
    }
}

impl Label for bool {
    fn label(&self) -> String {
        if *self {
            String::from("有貨")
        } else {
            String::from("缺貨")
        }
    }
}

fn main() {
    let price: u32 = 45;
    println!("{}", price.label());
    println!("{}", true.label());
}
```

執行結果：

```text
45 元
有貨
```

`u32` 和 `bool` 是標準函式庫的型別，但 `Label` 是自己的 trait，所以可以替它們實作。

不行的是**兩邊都不是自己的**：標準函式庫的 trait，加上標準函式庫的型別。例如想替 `Option<u32>` 實作 `Display`：

```rust,compile_fail=E0117
impl std::fmt::Display for Option<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Some(n) => write!(f, "剩 {} 件", n),
            None => write!(f, "缺貨"),
        }
    }
}

fn main() {
    let stock: Option<u32> = Some(3);
    println!("{}", stock);
}
```

編譯錯誤：

```text
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
 --> src/main.rs:1:1
  |
1 | impl std::fmt::Display for Option<u32> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^-----------
  |                            |
  |                            `Option` is not defined in the current crate
  |
  = note: impl doesn't have any local type before any uncovered type parameters
  = note: for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
  = note: define and implement a trait or new type instead
```

`` only traits defined in the current crate can be implemented for types defined outside of the crate ``：trait 跟型別至少要有一個，是定義在目前這個 crate 裡的。crate 是第 12 章的主題，現在先把「你寫的整個程式」當成一個 crate。這條規則叫**孤兒規則**（orphan rule），兩邊都不是自己的實作，就像沒人認領的孤兒。訊息最後一則 `note` 建議了出路：自己定義 trait（像上面的 `Label`），或定義新的型別（下一節）；其他 `note` 先不用管。

為什麼要有這條規則？如果允許，兩個不同的 crate 各自替 `Option<u32>` 實作 `Display`，合在一起編譯的時候，編譯器不知道該用哪一個。孤兒規則保證：一組「trait 加型別」的實作，只會寫在一個地方。

### 包一層：newtype

自己要的是 `Option<u32>` 的顯示方式，那就用第 5 章第 5 集的 tuple struct，把它包成自己的型別。這個手法叫 **newtype**：

```rust
struct Stock(Option<u32>);

impl std::fmt::Display for Stock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            Some(n) => write!(f, "剩 {} 件", n),
            None => write!(f, "缺貨"),
        }
    }
}

fn main() {
    println!("{}", Stock(Some(3)));
    println!("{}", Stock(None));
}
```

執行結果：

```text
剩 3 件
缺貨
```

- `Stock` 是自己定義的型別，所以能實作 `Display`。裡面包的 `Option<u32>` 用 `self.0` 取出來。
- `match self.0` 不會 move：`Option<u32>` 是 `Copy` 型別（第 3 章第 8 集）。
- 包了一層的型別，是全新的型別，原本 `Option<u32>` 的方法不會自動跟著過來，需要的話要自己寫。

### 跟 Go 比一比

Go 更嚴格：連標準型別的方法都不能自己加。

```go
type Price int

func (p Price) Label() string { return fmt.Sprint(int(p), " 元") }
```

Go 不能寫 `func (n int) Label()`，要先 `type Price int`，才能替它加方法。Rust 的 newtype 做法很像。不同的是，Rust 可以替 `u32` 這類標準型別實作你自己寫的 trait（像上面的 `Label for u32`），Go 沒有這個選項。

## 重點整理

- 替型別實作 trait，trait 和型別至少要有一個是自己程式裡定義的，否則是 E0117（孤兒規則）。
- 可以：自己的 trait 配任何型別、任何 trait 配自己的型別。不行：兩邊都來自標準函式庫，例如 `Display` 配 `Option<u32>`。
- 規則的理由：避免兩個 crate 替同一組 trait 與型別各寫一份實作。
- 繞過的辦法是 newtype：用 tuple struct 包一層，`self.0` 取出裡面的值，替新型別實作 trait。
- 跟 Go 比：Go 連標準型別都不能直接加方法，要先 `type Price int`；Rust 可以替標準型別實作你自己寫的 trait。
