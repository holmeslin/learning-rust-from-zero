# `matches!`

## 本集目標

用 `matches!(值, 模式)` 一行得到「這個值符不符合模式」的 `bool`，取代只回傳 `true` / `false` 的 `match`。

## 正文

### 只想知道符不符合

第 3 集說過，`status == Status::Paid` 不能用。想得到 `bool`，可以用 `match` 每個分支各回 `true` 或 `false`，不過很囉嗦。標準函式庫提供一個巨集，把它縮成一行：

```rust
enum Status {
    Pending,
    Paid,
    Shipped,
}

fn is_done_long(status: &Status) -> bool {
    match status {
        Status::Paid | Status::Shipped => true,
        Status::Pending => false,
    }
}

fn is_done(status: &Status) -> bool {
    matches!(status, Status::Paid | Status::Shipped)
}

fn main() {
    let list = [Status::Pending, Status::Paid, Status::Shipped];
    for status in list {
        println!("{} {}", is_done_long(&status), is_done(&status));
    }
}
```

執行結果：

```text
false false
true true
true true
```

兩個函式的結果一模一樣。`matches!(值, 模式)` 的值就是 `bool`：符合模式是 `true`，不符合是 `false`。

- 名字後面有 `!`，跟 `println!` 一樣是巨集，第 16 章第 1 集才會講巨集，現在照寫就好。
- 第二個參數是模式，前面幾集學過的都能用，例如 `|`。
- 參數 `&Status` 的比對方式跟第 7 集一樣：對參照比對，模式照樣寫 `Status::Paid`。

### 範圍與 guard 也能用

模式的所有寫法都能放進去，包括範圍和 match guard：

```rust
fn main() {
    let price: Option<u32> = Some(150);
    let score: u32 = 85;
    println!("{}", matches!(price, Some(p) if p > 100));
    println!("{}", matches!(score, 60..=100));
    println!("{}", matches!(score, 0 | 100));
}
```

執行結果：

```text
true
true
false
```

`matches!(price, Some(p) if p > 100)` 把 `Some(p)` 配上條件 `p > 100`，整個還是 `bool`。

### 只回傳 `bool`

`matches!` 只回答「符不符合」，拿不到模式裡綁定的變數：`p` 只能在 `if` 條件裡用，`matches!` 外面看不到它。需要用到裡面的值，還是得用 `if let` 或 `match`。

和 `if` 搭在一起很自然：

```rust
enum Status {
    Pending,
    Paid,
}

fn main() {
    let list = [Status::Pending, Status::Paid];
    for status in list {
        if matches!(status, Status::Pending) {
            println!("還在等付款");
        } else {
            println!("款項已收");
        }
    }
}
```

執行結果：

```text
還在等付款
款項已收
```

## 重點整理

- `matches!(值, 模式)` 回傳 `bool`：符合模式是 `true`，否則 `false`。
- 它等於「各分支回 `true` / `false` 的 `match`」的簡寫，也繞過了 enum 不能用 `==` 的問題。
- 模式可以用 `|`、範圍，也能加 guard：`matches!(price, Some(p) if p > 100)`。
- `matches!` 是巨集（第 16 章第 1 集），這一章先照寫。
- 它拿不到模式裡的變數；需要用到值時，改用 `if let` 或 `match`。
