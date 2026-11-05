# `enumerate`、`zip`、`rev`

## 本集目標

用 `enumerate` 取得索引、用 `zip` 把兩個迭代器配成一對、用 `rev` 倒著走，並能把它們跟前面學的 adapter 串在一起。

## 正文

### `enumerate`：多給一個索引

Go 的 `for i, v := range` 本來就有索引。Rust 的 `for x in ...` 只給元素，想要索引，就在後面加 `enumerate()`，每個元素變成 `(索引, 元素)` 的 tuple：

```rust
fn main() {
    let menu = ["紅茶", "咖啡", "奶茶"];
    for (i, name) in menu.iter().enumerate() {
        println!("{}. {name}", i + 1);
    }
}
```

執行結果：

```text
1. 紅茶
2. 咖啡
3. 奶茶
```

- `for (i, name) in ...` 的 `(i, name)` 是模式，把每個 tuple 拆開，跟第 7 章第 13 集的 `for (name, city) in members` 是同一種寫法。
- 索引從 0 開始，型別是 `usize`；要從 1 起算，自己 `i + 1`。
- 第 2 章第 13 集說過有「不會寫錯上限」的做法，就是這個：不必寫 `for i in 0..menu.len()` 再 `menu[i]`，既不會漏掉也不會越界。

### `zip`：兩個迭代器配成一對

`zip` 把兩個迭代器一一配對，每次給出 `(甲的元素, 乙的元素)`，有一邊先走完就停：

```rust
fn main() {
    let names = ["紅茶", "咖啡", "奶茶"];
    let prices = [30, 45];
    for (name, price) in names.iter().zip(prices.iter()) {
        println!("{name} {price} 元");
    }
}
```

執行結果：

```text
紅茶 30 元
咖啡 45 元
```

`prices` 只有兩個，所以第三個「奶茶」沒有配對，直接被丟掉。長度對不上不會 panic，也不會有任何提醒，兩邊長度要一致時，得自己留意。

### `rev`：倒著走

`rev()` 讓迭代器從最後一個開始走。範圍、`chars()`、陣列的 `iter()` 都能用：

```rust
fn main() {
    for i in (1..=3).rev() {
        println!("倒數 {i}");
    }

    let name = String::from("紅茶拿鐵");
    let reversed: String = name.chars().rev().collect();
    println!("{reversed}");

    let menu = ["紅茶", "咖啡", "奶茶"];
    for (i, item) in menu.iter().enumerate().rev() {
        println!("{i} {item}");
    }
}
```

執行結果：

```text
倒數 3
倒數 2
倒數 1
鐵拿茶紅
2 奶茶
1 咖啡
0 紅茶
```

- `(1..=3).rev()`：範圍要先用括號括起來，再接 `.rev()`；寫成 `1..=3.rev()` 會變成對 `3` 呼叫方法，是 E0689，`help` 會建議加括號。
- `chars().rev().collect()` 把字元倒過來收成新的 `String`，是倒轉字串最短的寫法。
- `enumerate().rev()` 保留原本的索引（2、1、0）；如果寫成 `rev().enumerate()`，索引會是倒過來之後重新數的 0、1、2。adapter 的順序會影響結果。

## 重點整理

- `enumerate()` 把元素變成 `(索引, 元素)`，索引從 0 開始、型別是 `usize`；`for (i, x) in v.iter().enumerate()` 取代 `for i in 0..v.len()` 加 `v[i]`。
- `a.zip(b)` 把兩個迭代器配成 `(甲, 乙)`，其中一邊走完就停，長度不同不會 panic，多的元素被丟掉。
- `rev()` 倒著走，可用在範圍（要加括號 `(1..=3).rev()`）、`chars()`、`iter()`。
- adapter 的順序有差：`enumerate().rev()` 保留原索引，`rev().enumerate()` 重新編號。
