# `|` 與範圍模式

## 本集目標

用 `|` 讓一個分支涵蓋好幾個模式，用 `a..=b` 比對一段範圍，並用 `名字 @ 模式` 在比對的同時把值留下來。

## 正文

### `|`：任一個都行

好幾個模式要做同一件事時，用 `|` 連起來，意思是「或」：

```rust
enum Status {
    Pending,
    Paid,
    Shipped,
}

fn main() {
    let list = [Status::Pending, Status::Paid, Status::Shipped];
    for status in list {
        match status {
            Status::Pending => println!("還不能出貨"),
            Status::Paid | Status::Shipped => println!("款項已收"),
        }
    }
}
```

執行結果：

```text
還不能出貨
款項已收
款項已收
```

這相當於 Go 的 `case Paid, Shipped:`。`|` 也能放在模式裡面，例如 `Some(1 | 3 | 5)` 表示 `Some` 裡面是 1、3、5 其中一個。如果 `|` 兩邊有綁定變數，兩邊都要有同樣的名字和型別，第 12 集會看到例子。

### 範圍 `a..=b`

整數和 `char` 可以用範圍當模式。`a..=b` 包含 `b`，跟第 1 章 `for` 的範圍寫法一樣；`90..` 表示 90 以上：

```rust
fn grade(score: u32) -> char {
    match score {
        90.. => 'A',
        80..=89 => 'B',
        60..=79 => 'C',
        0..=59 => 'D',
    }
}

fn main() {
    for score in [95, 85, 70, 30] {
        println!("{score} 分是 {}", grade(score));
    }
}
```

執行結果：

```text
95 分是 A
85 分是 B
70 分是 C
30 分是 D
```

四個範圍加起來剛好涵蓋所有 `u32`，所以不需要 `_`。編譯器會真的去算，把 `80..=89` 寫成 `81..=89` 看看：

```rust,compile_fail=E0004
fn grade(score: u32) -> char {
    match score {
        90.. => 'A',
        81..=89 => 'B',
        60..=79 => 'C',
        0..=59 => 'D',
    }
}

fn main() {
    println!("{}", grade(85));
}
```

編譯錯誤：

```text
error[E0004]: non-exhaustive patterns: `80_u32` not covered
 --> src/main.rs:2:11
  |
2 |     match score {
  |           ^^^^^ pattern `80_u32` not covered
  |
  = note: the matched value is of type `u32`
```

縫隙被抓出來了：`80_u32` 沒有分支接住。不含右端的 `a..b`、不限下界的 `..=b` 也都能寫。

### `@`：比對範圍，同時留下值

只寫 `90..=100`，分支裡拿不到實際的分數。想比對範圍、又要用那個值，在範圍前面加「名字 `@`」：

```rust
fn report(score: u32) {
    match score {
        s @ 90..=100 => println!("{s} 分：優等"),
        s @ 60..=89 => println!("{s} 分：及格"),
        s => println!("{s} 分：不及格"),
    }
}

fn main() {
    report(95);
    report(70);
    report(30);
}
```

執行結果：

```text
95 分：優等
70 分：及格
30 分：不及格
```

`s @ 90..=100` 的意思是：這個值如果落在 90 到 100，就把整個值綁定到 `s`。最後一個分支只有 `s`，是第 5 集的「其餘全部」。`@` 也能放在模式裡面，例如 `Some(n @ 1..=9)`。

## 重點整理

- `|` 把多個模式連成「或」：`Status::Paid | Status::Shipped`；也能用在模式裡面，例如 `Some(1 | 3 | 5)`。
- 範圍模式：`80..=89`（含右端）、`90..`（90 以上）、`..=59`、`a..b`；本書用在整數與 `char`。
- 範圍同樣要窮盡；涵蓋整個型別時不用 `_`，留下縫隙就是 E0004。
- `名字 @ 模式`：比對模式，成功時把整個值綁定到名字，例如 `s @ 90..=100`。
