# 關聯型別

## 本集目標

認得 trait 裡的 `type Item;`，會在 `impl` 裡指定它，並用 `Dispenser<Item = u32>` 在條件裡指定關聯型別；知道它跟 `From<T>` 這種泛型 trait 的差別。

## 正文

### trait 裡的型別佔位

取餐號碼機 `Tickets` 吐出數字，點單堆疊 `Orders` 吐出名字。兩個東西「取出來的型別」不一樣，卻都是「取出一個」。trait 可以用 `type Item;` 宣告「這裡有一個型別，叫 `Item`，由實作的型別決定」，這叫**關聯型別**（associated type）：

```rust
trait Dispenser {
    type Item;

    fn take(&mut self) -> Option<Self::Item>;
}

struct Tickets {
    next: u32,
    last: u32,
}

struct Orders {
    names: Vec<String>,
}

impl Dispenser for Tickets {
    type Item = u32;

    fn take(&mut self) -> Option<u32> {
        if self.next > self.last {
            None
        } else {
            let number = self.next;
            self.next += 1;
            Some(number)
        }
    }
}

impl Dispenser for Orders {
    type Item = String;

    fn take(&mut self) -> Option<String> {
        self.names.pop()
    }
}

fn sum_all<D: Dispenser<Item = u32>>(source: &mut D) -> u32 {
    let mut sum = 0;
    while let Some(n) = source.take() {
        sum += n;
    }
    sum
}

fn main() {
    let mut tickets = Tickets { next: 1, last: 3 };
    println!("{:?}", tickets.take());
    println!("{}", sum_all(&mut tickets));
    println!("{:?}", tickets.take());

    let mut orders = Orders {
        names: vec![String::from("紅茶"), String::from("咖啡")],
    };
    println!("{:?}", orders.take());
}
```

執行結果：

```text
Some(1)
5
None
Some("咖啡")
```

- trait 裡的 `type Item;` 只宣告名字。`Option<Self::Item>` 是「那個型別的 `Item`」：`Self` 代表正在實作這個 trait 的型別，`Self` 附錄一 e 才完整介紹，這一集只在 trait 定義裡這樣寫。
- `impl` 裡用 `type Item = u32;` 指定答案，其餘照常寫完整的型別：`Tickets` 的 `Item` 是 `u32`，所以 `take` 回傳 `Option<u32>`；`Orders` 的是 `String`。忘了寫 `type Item = ...;` 是 E0046（`missing: Item`）。

### 在條件裡指定關聯型別

`sum_all` 的 `D: Dispenser<Item = u32>` 讀成「任何 `Dispenser`，只要它取出來的是 `u32`」。尖括號裡的 `Item = u32` 就是在指定關聯型別。把 `Orders`（`Item` 是 `String`）傳進去，條件不符：

```rust,compile_fail=E0271
trait Dispenser {
    type Item;

    fn take(&mut self) -> Option<Self::Item>;
}

struct Orders;

impl Dispenser for Orders {
    type Item = String;

    fn take(&mut self) -> Option<String> {
        None
    }
}

fn sum_all<D: Dispenser<Item = u32>>(source: &mut D) -> u32 {
    let mut sum = 0;
    while let Some(n) = source.take() {
        sum += n;
    }
    sum
}

fn main() {
    let mut orders = Orders;
    println!("{}", sum_all(&mut orders));
}
```

編譯錯誤：

```text
error[E0271]: type mismatch resolving `<Orders as Dispenser>::Item == u32`
  --> src/main.rs:27:28
   |
27 |     println!("{}", sum_all(&mut orders));
   |                    ------- ^^^^^^^^^^^ type mismatch resolving `<Orders as Dispenser>::Item == u32`
   |                    |
   |                    required by a bound introduced by this call
   |
note: expected this to be `u32`
  --> src/main.rs:10:17
   |
10 |     type Item = String;
   |                 ^^^^^^
```

`` <Orders as Dispenser>::Item == u32 `` 讀成「`Orders` 當成 `Dispenser` 時的 `Item`，應該是 `u32`」。下面的 `note` 指向 `type Item = String;`，說「這裡本來該是 `u32`」。這裡只貼前半段，後面還有一則 `note`，指出條件寫在 `sum_all` 的哪裡。

### 跟 `From<T>` 有什麼不同

第 13 集的 `From<&str>`，尖括號是 trait 的型別參數：同一個型別可以實作很多次，`String` 就有 `From<&str>`、`From<char>` 等好幾個。關聯型別則不行，一個型別對這個 trait 只有**一個**答案，所以用的人不必另外寫 `Dispenser<u32>`，型別早就由實作決定了。標準函式庫裡常見：`Iterator` 的 `Item`（第 11 章）、`FromStr` 的 `Err`（第 14 集 `parse` 簽名裡的 `F::Err`），還有下一集 `Add` 的 `Output`。

## 重點整理

- trait 裡的 `type Item;` 是關聯型別：宣告一個由實作者決定的型別；在 trait 裡用 `Self::Item` 引用。
- `impl` 裡要用 `type Item = 型別;` 指定，少寫是 E0046；一個型別對一個 trait 只能指定一次。
- 條件裡可以寫 `D: Dispenser<Item = u32>` 限定關聯型別；不符是 E0271。
- 跟泛型 trait（`From<T>`）比：泛型 trait 同一個型別可以實作很多次；關聯型別每個型別只有一個答案。
- 標準函式庫的例子：`Iterator::Item`、`FromStr::Err`、`Add::Output`。
