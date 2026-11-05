# 回傳閉包

## 本集目標

用 `-> impl Fn(u32) -> u32` 從函式回傳閉包，知道為什麼要加 `move`；當不同分支要回傳不同的閉包、或要把好幾個閉包放進同一個 `Vec`，改用 `Box<dyn Fn(u32) -> u32>`。

## 正文

### 回傳 `impl Fn`

第 8 章第 7 集說過，有些型別寫不出名字，回傳 `impl Trait` 幾乎是唯一的寫法，閉包就是。下面的函式依折扣金額，做出一個專屬的折扣閉包：

```rust
fn make_discount(off: u32) -> impl Fn(u32) -> u32 {
    move |price| price - off
}

fn main() {
    let member = make_discount(10);
    let coupon = make_discount(30);
    println!("{}", member(100));
    println!("{}", coupon(100));
}
```

執行結果：

```text
90
70
```

`member` 與 `coupon` 是兩個閉包，各自帶著不同的 `off`。

這裡一定要 `move`：`off` 是函式的參數，函式一結束就消失，閉包卻要被帶出去。少了 `move`，編譯器會報 E0373，跟第 3 集一樣的錯誤，只是這次 `note` 寫的是 `closure is returned here`。

### 不同分支，不同的型別

第 8 章第 7 集說過，`impl Trait` 只能是同一種型別。每個閉包的型別都是獨一無二的，所以 `if` 和 `else` 各寫一個擷取了變數的閉包，兩個型別就對不上了：

```rust,compile_fail=E0308
fn make_rule(member: bool, off: u32, percent: u32) -> impl Fn(u32) -> u32 {
    if member {
        move |price| price - off
    } else {
        move |price| price * percent / 100
    }
}

fn main() {
    let rule = make_rule(true, 5, 90);
    println!("{}", rule(100));
}
```

編譯錯誤：

```text
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:5:9
  |
2 | /     if member {
3 | |         move |price| price - off
  | |         ------------------------
  | |         |
  | |         the expected closure
  | |         expected because of this
4 | |     } else {
5 | |         move |price| price * percent / 100
  | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected closure, found a different closure
6 | |     }
  | |_____- `if` and `else` have incompatible types
  |
  = note: expected closure `{closure@src/main.rs:3:9: 3:21}`
             found closure `{closure@src/main.rs:5:9: 5:21}`
  = note: no two closures, even if identical, have the same type
  = help: consider boxing your closure and/or using it as a trait object
```

重點在 `no two closures, even if identical, have the same type`：就算寫得一模一樣，兩個閉包也是不同的型別。這裡只貼前半段，後面的 `help` 給了方向：改成回傳 `Box<dyn Fn(u32) -> u32>`，並把每個閉包用 `Box::new(...)` 包起來。這就是第 8 章第 15 集的 trait 物件：型別在執行時期才決定，所以不同的閉包可以放在同一個型別底下。`Box::new(...)` 照樣先照抄，第 13 章才解釋：

```rust
fn make_rule(member: bool, off: u32, percent: u32) -> Box<dyn Fn(u32) -> u32> {
    if member {
        Box::new(move |price| price - off)
    } else {
        Box::new(move |price| price * percent / 100)
    }
}

fn main() {
    println!("{}", make_rule(true, 5, 90)(100));
    println!("{}", make_rule(false, 5, 90)(100));
}
```

執行結果：

```text
95
90
```

### 把好幾個閉包放進 `Vec`

同樣的道理，`Vec` 的元素必須是同一種型別。想放好幾個不同的閉包，元素型別就寫 `Box<dyn Fn(u32) -> u32>`：

```rust
fn main() {
    let off = 10;
    let mut rules: Vec<Box<dyn Fn(u32) -> u32>> = Vec::new();
    rules.push(Box::new(move |price| price - off));
    rules.push(Box::new(|price| price * 9 / 10));

    let mut price = 100;
    for rule in &rules {
        price = rule(price);
        println!("套用後 {price} 元");
    }
}
```

執行結果：

```text
套用後 90 元
套用後 81 元
```

### 跟 Go 比一比

Go 的函式值可以隨便回傳、隨便放進 slice，不用多想：

```go
func makeDiscount(off int) func(int) int {
	return func(price int) int { return price - off }
}
```

`off` 會被編譯器搬到 heap 上，活到閉包沒人用為止。Rust 要你自己交代：用 `move` 讓閉包擁有 `off`；型別一樣就回傳 `impl Fn`；不同的閉包要放在一起，就用 `Box<dyn Fn>`。

## 重點整理

- 從函式回傳閉包：`-> impl Fn(u32) -> u32`；閉包用到參數時，一定要 `move`，否則是 E0373。
- 每個閉包型別獨一無二，`impl Fn` 又只能是同一種型別，所以不同分支回傳不同的閉包是 E0308（`no two closures, even if identical, have the same type`）。
- 要回傳不同的閉包，或把好幾個閉包放進 `Vec`，用 `Box<dyn Fn(u32) -> u32>`，每個閉包用 `Box::new(...)` 包起來（照抄，第 13 章解釋 `Box`）。
- 跟 Go 比：Go 的函式值隨便回傳；Rust 要用 `move` 交代誰擁有擷取的變數，型別不同時還得裝箱。
