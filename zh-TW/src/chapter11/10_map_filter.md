# `map` 與 `filter`

## 本集目標

用 `map` 把每個元素轉換成別的值，用 `filter` 留下符合條件的元素，把兩個 adapter 串起來，並看懂閉包參數的型別（`filter` 多一層參照）。

## 正文

### `map`：每個元素換成別的

`map` 接收一個閉包，對每個元素呼叫它，用回傳值當作新的元素。轉出來的型別可以跟原本不同：

```rust
fn main() {
    let prices = [30, 45, 38];
    for price in prices.iter().map(|p| p + 5) {
        println!("加外送費 {price}");
    }
    for label in prices.iter().map(|p| format!("{p} 元")) {
        println!("{label}");
    }
}
```

執行結果：

```text
加外送費 35
加外送費 50
加外送費 43
30 元
45 元
38 元
```

`iter()` 的元素是 `&i32`，所以閉包的參數 `p` 是 `&i32`；`p + 5` 直接算得出 `i32`。第二個 `map` 把每個數字換成 `String`。上一集說過 adapter 是惰性的，這裡靠 `for` 這個 consumer 才真的執行。

### `filter`：只留下符合的

`filter` 的閉包回傳 `bool`，`true` 的元素留下，`false` 的丟掉。直接照 `map` 的樣子寫：

```rust,compile_fail=E0308
fn main() {
    let prices = [30, 45, 38];
    for price in prices.iter().filter(|p| p >= 40) {
        println!("{price}");
    }
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:3:48
  |
3 |     for price in prices.iter().filter(|p| p >= 40) {
  |                                                ^^ expected `&&_`, found integer
  |
  = note: expected reference `&&_`
                  found type `{integer}`
```

`expected &&_, found integer`：`p` 是兩層參照。原因是 `iter()` 的元素已經是 `&i32`，而 `filter` 只是要「看一眼」元素，不能把元素拿走，所以閉包拿到的是「元素的參照」，也就是 `&&i32`。要比較大小，得用第 4 章第 7 集的 `*` 解開兩層：

```rust
fn main() {
    let prices = [30, 45, 38];
    for price in prices.iter().filter(|p| **p >= 40) {
        println!("{price}");
    }
}
```

執行結果：

```text
45
```

別人的程式碼常把參數寫成 `|&&p| p >= 40`，意思相同，是另一種拆開參照的寫法；本書一律用 `**p`。`map` 的閉包拿到的是元素本身，所以只有一層。元素是 struct 的話，用 `.` 讀欄位或呼叫方法時，編譯器會自動解參照（第 5 章第 10 集），就不需要寫 `*`。

### 串起來

adapter 回傳的還是迭代器，所以可以接著再串。每個 adapter 寫成一行，讀起來像一條管線。下面找出點數夠多的會員，換成顯示用的文字：

```rust
struct Member {
    name: String,
    points: u32,
}

impl Member {
    fn new(name: &str, points: u32) -> Member {
        Member {
            name: String::from(name),
            points,
        }
    }
}

fn main() {
    let mut members = Vec::new();
    members.push(Member::new("小明", 120));
    members.push(Member::new("小華", 80));
    members.push(Member::new("小美", 200));

    let vip = members
        .iter()
        .filter(|m| m.points >= 100)
        .map(|m| format!("{}（{} 點）", m.name, m.points));
    for text in vip {
        println!("{text}");
    }
}
```

執行結果：

```text
小明（120 點）
小美（200 點）
```

`filter` 先丟掉小華，`map` 再把剩下的兩位轉成文字，順序很重要：先篩再轉，`map` 的閉包只會對留下來的元素執行。

## 重點整理

- `map(|x| 新值)`：對每個元素呼叫閉包，用回傳值當新元素，型別可以改變。
- `filter(|x| 條件)`：條件為 `true` 的元素留下，其餘丟掉。
- 兩者都是 adapter（惰性），要 consumer（例如 `for`）才會執行；adapter 之後還能接 adapter，通常一行寫一個。
- 閉包參數的型別：`iter()` 配 `map` 是 `&T`；`filter` 的閉包拿到「元素的參照」，`iter()` 配 `filter` 就是 `&&T`，要寫 `**p`（讀 struct 欄位與呼叫方法則不用 `*`）。
- 先 `filter` 再 `map`，轉換只會做在留下來的元素上。
