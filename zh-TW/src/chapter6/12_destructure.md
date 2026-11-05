# 解構 struct 與 tuple

## 本集目標

用模式把 struct 拆開、在 `match` 裡比對欄位的值，並一次比對 tuple 裡的多個值。

## 正文

### 解構 struct

第 2 章用 `let (x, y) = point;` 拆 tuple。struct 也行，模式寫成「型別名稱加大括號」：

```rust
struct Member {
    name: String,
    city: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        city: String::from("台北"),
        points: 120,
    };
    let Member { name, city, points } = member;
    println!("{name} 住在 {city}，有 {points} 點");
}
```

執行結果：

```text
小美 住在 台北，有 120 點
```

大括號裡的欄位名稱，同時是新變數的名字，跟第 5 章第 3 集的欄位初始化簡寫是同一種寫法，方向相反：建立 struct 是把變數放進欄位，解構是把欄位放進變數。

- 想換名字：`Member { name: n, .. }`，把 `name` 欄位放進變數 `n`。
- 只要部分欄位：沒列到的欄位，一律用 `..` 補上，例如 `Member { points, .. }`。

不能既不列出欄位、也不寫 `..`：

```rust,compile_fail=E0027
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        points: 120,
    };
    let Member { name } = member;
    println!("{name}");
}
```

編譯錯誤：

```text
error[E0027]: pattern does not mention field `points`
  --> src/main.rs:11:9
   |
11 |     let Member { name } = member;
   |         ^^^^^^^^^^^^^^^ missing field `points`
   |
help: include the missing field in the pattern
   |
11 |     let Member { name, points } = member;
   |                      ++++++++
```

`help` 第一個建議是把 `points` 加進模式；後面還有兩個建議，其中一個就是用 `..` 略過其餘欄位。

### 在 `match` 裡比對欄位

欄位的位置也可以放字面值或其他模式，不只放變數名稱：

```rust
struct Member {
    name: String,
    city: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        city: String::from("台北"),
        points: 120,
    };
    match member {
        Member { points: 0, .. } => println!("還沒有點數"),
        Member { points, .. } => println!("有 {points} 點"),
    }
    println!("{} 住在 {}", member.name, member.city);
}
```

執行結果：

```text
有 120 點
小美 住在 台北
```

`points: 0` 要求 `points` 欄位剛好是 0。注意最後一行：模式只綁定了 `points`（`u32`，`Copy`），沒有碰 `name` 和 `city`，所以 `match` 之後 `member` 的 `String` 欄位還在。如果模式綁定 `name`，就是 move，跟第 7 集的 `Option<String>` 同樣的道理。

### 解構 tuple，一次比對兩個值

`match` 的對象可以是 tuple，一次看兩個值的組合。查兩個商品的價格，有幾種可能：

```rust
fn find_price(code: u32) -> Option<u32> {
    if code == 1 {
        Some(30)
    } else if code == 2 {
        Some(45)
    } else {
        None
    }
}

fn total(a: u32, b: u32) -> u32 {
    match (find_price(a), find_price(b)) {
        (Some(x), Some(y)) => x + y,
        (Some(x), None) | (None, Some(x)) => x,
        (None, None) => 0,
    }
}

fn main() {
    println!("{}", total(1, 2));
    println!("{}", total(1, 9));
    println!("{}", total(8, 9));
}
```

執行結果：

```text
75
30
0
```

- `(Some(x), Some(y))`：tuple 的兩個位置各放一個模式，模式可以一層套一層。
- 兩個都有，加起來；只有一個有，取那一個；都沒有，0。`match` 同樣要窮盡，四種組合（兩個都有、只有第一個有、只有第二個有、兩個都沒有）一個都不能少。
- `(Some(x), None) | (None, Some(x))`：第 11 集說過，`|` 兩邊綁定的變數名字和型別要一樣，這裡兩邊都是 `x`，都是 `u32`。

## 重點整理

- struct 解構：`let Member { name, city, points } = member;`，欄位名稱同時是新變數；換名字寫 `name: n`；只要部分欄位，其餘用 `..`。
- 模式沒有列出所有欄位、又沒有 `..`，是 E0027。
- 欄位的位置可以放字面值，例如 `Member { points: 0, .. }`；只綁定 `Copy` 的欄位，不會 move 其他欄位。
- tuple 解構可以放在 `match` 裡一次比對多個值，模式可以一層套一層，例如 `(Some(x), None)`。
- `|` 兩邊綁定的變數，名字和型別都要相同。
