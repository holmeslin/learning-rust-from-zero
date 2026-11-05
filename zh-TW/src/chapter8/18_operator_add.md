# 運算子多載 `Add`

## 本集目標

替自己的型別實作 `std::ops::Add`，讓 `a + b` 能用；知道運算子只是 trait 方法的簡寫，而且 `+` 會把兩邊的值 move 進去。

## 正文

### `a + b` 就是呼叫 `add`

Rust 的運算子都對應一個 trait，放在 `std::ops`。`+` 對應 `Add`，替自己的型別實作它，就能對那個型別用 `+`。Go 沒有運算子多載，`+` 只能用在內建型別。

```rust
#[derive(Debug, Clone, Copy)]
struct Money {
    dollars: u32,
}

impl std::ops::Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Money {
        Money {
            dollars: self.dollars + other.dollars,
        }
    }
}

impl std::ops::Mul<u32> for Money {
    type Output = Money;

    fn mul(self, cups: u32) -> Money {
        Money {
            dollars: self.dollars * cups,
        }
    }
}

fn main() {
    let tea = Money { dollars: 45 };
    let coffee = Money { dollars: 55 };
    let total = tea + coffee;
    println!("{:?}", total);
    println!("{:?}", tea * 3);
    println!("{}", (tea + coffee).dollars);
}
```

執行結果：

```text
Money { dollars: 100 }
Money { dollars: 135 }
100
```

- `tea + coffee` 就是呼叫 `Add` 的 `add` 方法，把 `coffee` 當參數。`add` 裡把兩個 `u32` 欄位用 `+` 加起來。
- `type Output = Money;` 是上一集的關聯型別：它指定 `+` 的結果是什麼型別。`add` 的回傳型別要跟它一致。
- `Add` 預設右邊跟左邊同型別。`impl std::ops::Mul<u32> for Money` 在尖括號裡指定右邊是 `u32`（跟第 13 集的 `From<&str>` 一樣的寫法），所以 `tea * 3` 是 `Money` 乘 `u32`。
- 其他運算子也一樣：`-` 是 `Sub`、`/` 是 `Div`、`+=` 是 `AddAssign`，都在 `std::ops`；`==` 與 `<` 則是第 10、12 集的 `PartialEq`、`PartialOrd`。
- 沒有實作的型別用 `+`，會是 E0369（`cannot add Money to Money`），訊息還會提示「`Add` might be missing」。

### `+` 會 move 兩邊的值

`add` 的第一個參數是 `self`，不是 `&self`（第 5 章第 8 集）：呼叫之後，左邊的值就被拿走了。如果型別不是 `Copy`，左邊的變數用完就不能再用：

```rust,compile_fail=E0382
struct Money {
    dollars: u32,
}

impl std::ops::Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Money {
        Money {
            dollars: self.dollars + other.dollars,
        }
    }
}

fn main() {
    let tea = Money { dollars: 45 };
    let coffee = Money { dollars: 55 };
    let total = tea + coffee;
    println!("{}", total.dollars);
    println!("{}", tea.dollars);
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `tea`
  --> src/main.rs:20:20
   |
16 |     let tea = Money { dollars: 45 };
   |         --- move occurs because `tea` has type `Money`, which does not implement the `Copy` trait
17 |     let coffee = Money { dollars: 55 };
18 |     let total = tea + coffee;
   |                 ------------ `tea` moved due to usage in operator
19 |     println!("{}", total.dollars);
20 |     println!("{}", tea.dollars);
   |                    ^^^^^^^^^^^ value borrowed here after move
```

這跟第 5 章 `self` 方法的 E0382 一樣：`` `tea` moved due to usage in operator ``。這裡只貼前半段，後面的 `note` 寫著 `calling this operator moves the left-hand side`，也提示可以加 `Clone`。

上面第一個範例的 `Money` 有 `#[derive(Clone, Copy)]`（第 10 集），`+` 時複製一份進去，所以 `tea` 用完還能再用，`(tea + coffee).dollars` 才寫得出來。像 `Money` 這種只有整數欄位的型別，加上 `Copy` 最省事；有 `String` 欄位的型別沒辦法 `Copy`，`+` 之後左邊就是用掉了，就像第 7 章第 7 集的 `String + &str`。

## 重點整理

- 運算子對應 `std::ops` 裡的 trait：`+` 是 `Add`、`-` 是 `Sub`、`*` 是 `Mul`、`+=` 是 `AddAssign`；`a + b` 就是呼叫 `a.add(b)`。
- 實作 `impl std::ops::Add for Money`：寫 `type Output = Money;` 與 `fn add(self, other: Money) -> Money`。右邊的型別可以另外指定，例如 `Mul<u32>`。
- 沒有實作的型別用運算子是 E0369。
- `add` 的 `self` 是值，`+` 會 move 兩邊；型別不是 `Copy` 的話，用完就不能再用（E0382）。整數欄位的型別 `derive(Clone, Copy)` 最省事。
