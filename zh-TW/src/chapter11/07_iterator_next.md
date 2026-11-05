# `Iterator` 與 `next`

## 本集目標

知道迭代器就是實作了 `Iterator` trait 的東西，核心只有 `next()` 一個方法；會手動呼叫 `next()`，並用 `nth(n)` 取第 n 個元素。

## 正文

### 只有一個方法的 trait

標準函式庫的 `Iterator` 長這樣（簡化過）：

```text
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

這就是第 8 章第 17 集的 `Dispenser`：關聯型別 `Item` 是每次取出來的東西，`next` 每呼叫一次就取出一個，沒東西了回傳 `None`。實作了這個 trait，就是迭代器；`Self` 附錄一 e 才完整介紹，這裡讀成「正在實作這個 trait 的型別」。

### 手動呼叫 `next`

`Vec` 和陣列的 `.iter()` 會回傳迭代器，先當成「從頭開始、依序借出每個元素的參照」，下一集再細講。我們自己呼叫 `next`：

```rust
fn main() {
    let prices = [30, 45, 38];
    let mut it = prices.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
```

執行結果：

```text
Some(30)
Some(45)
Some(38)
None
```

- 每次 `next()` 回傳 `Some(下一個)`，第四次沒有了，回傳 `None`。
- `it` 要用 `let mut`：`next` 是 `&mut self` 方法，迭代器自己記著走到哪裡，每呼叫一次就改動一次。沒有 `mut` 是 E0596，跟第 4 集的 `FnMut` 閉包同一個道理。
- 這裡的 `Item` 是 `&i32`，所以 `Some(30)` 裡面其實是指向 30 的參照；`{:?}` 印出來看不出差別。

### 一直 `next` 到 `None`

重複呼叫 `next` 直到 `None`，正好是第 7 章第 2 集 `while let` 的用途：

```rust
fn main() {
    let prices = [30, 45, 38];
    let mut it = prices.iter();
    while let Some(price) = it.next() {
        println!("{price}");
    }
}
```

執行結果：

```text
30
45
38
```

這就是 `for price in prices.iter()` 在做的事，第 8 集揭曉。

### `nth`：跳到第 n 個

迭代器用過的元素就沒了，往前走了就回不去。`nth(n)` 是便利方法：丟掉前 n 個，回傳下一個。第 7 章第 8 集說字串不能用索引，想取第 n 個字元，這就是做法：

```rust
fn main() {
    let name = String::from("紅茶拿鐵");
    println!("{:?}", name.chars().nth(2));
    println!("{:?}", name.chars().nth(10));

    let mut it = name.chars();
    println!("{:?}", it.nth(1));
    println!("{:?}", it.next());
}
```

執行結果：

```text
Some('拿')
None
Some('茶')
Some('拿')
```

- `nth(2)` 從 0 算起，取到第三個字元「拿」；超出範圍是 `None`，不會 panic。
- 第二組用同一個迭代器：`nth(1)` 丟掉「紅」、取走「茶」，所以後面的 `next()` 接著拿到「拿」。
- `nth(0)` 就是 `next()`，clippy 會要求你直接寫 `next()`。
- 取第 n 個要從頭數到第 n 個，字串很長時，會比陣列的 `a[n]` 慢。

## 重點整理

- 迭代器是實作了 `Iterator` 的東西，核心是 `fn next(&mut self) -> Option<Self::Item>`：有下一個回傳 `Some`，沒有就回傳 `None`；`Item` 是關聯型別（第 8 章第 17 集）。
- `next` 要 `&mut self`，手動使用迭代器時，變數要 `let mut`，否則是 E0596。
- `while let Some(x) = it.next()` 可以走完整個迭代器；`for` 就是這個迴圈的簡寫（第 8 集）。
- 迭代器走過的元素不會再回來；`nth(n)` 丟掉前 n 個再取下一個，`name.chars().nth(n)` 是取第 n 個字元的做法。
