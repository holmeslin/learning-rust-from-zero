# `if let`

## 本集目標

只在乎一種情況時，用 `if let 模式 = 值 { ... }` 取代只有一個分支有事做的 `match`，並知道它省掉了什麼檢查。

## 正文

### 只有一個分支有事做

查到價格才要印，查不到什麼都不用做。用 `match` 寫，另一個分支只好留空：

```rust
fn main() {
    let price: Option<u32> = Some(30);
    match price {
        Some(p) => println!("單價 {p} 元"),
        _ => {}
    }
}
```

執行結果：

```text
單價 30 元
```

`_ => {}` 的 `{}` 是空區塊，值是第 2 章的 `()`，意思是「其他情況什麼都不做」。這種寫法很常見，所以有更短的版本。

### `if let`

```rust
fn main() {
    let price: Option<u32> = Some(30);
    if let Some(p) = price {
        println!("單價 {p} 元");
    }
}
```

執行結果：

```text
單價 30 元
```

- `if let 模式 = 值`：值符合模式，就執行大括號裡的程式；不符合就跳過。
- 這裡的 `=` 是單一個等號，左邊是模式、右邊是被比對的值，不是比較。寫法跟 Go 的 `if v, ok := m[k]; ok { ... }` 很像，只是「有沒有」由模式自己判斷。
- `p` 只能在大括號裡用。

後面可以接 `else`，符合與否各做一件事。跟 `if` 一樣，整個 `if let` 是運算式，兩邊的值型別要相同：

```rust
fn main() {
    let price: Option<u32> = None;
    let total = if let Some(p) = price { p * 3 } else { 0 };
    println!("三份共 {total} 元");
}
```

執行結果：

```text
三份共 0 元
```

### 比對 enum 的某一個 variant

`if let` 對所有 enum 都能用。想只處理某一種折扣，其他都略過：

```rust
enum Discount {
    Regular,
    Percent(u32),
    Threshold { min: u32, off: u32 },
}

fn main() {
    let list = [
        Discount::Regular,
        Discount::Percent(20),
        Discount::Threshold { min: 800, off: 150 },
    ];
    for discount in list {
        if let Discount::Percent(p) = discount {
            println!("折 {p}%");
        } else if let Discount::Threshold { min, off } = discount {
            println!("滿 {min} 折 {off}");
        } else {
            println!("原價");
        }
    }
}
```

執行結果：

```text
原價
折 20%
滿 800 折 150
```

`else if let` 可以一路接下去，不過分支一多，就是 `match` 的工作了。

### 取捨：不檢查窮盡

`if let` 不要求窮盡。這是優點也是代價：只在乎一種情況時不用寫沒意義的分支；但之後 `Discount` 新增 variant，這個 `if let` 不會有任何編譯錯誤，新的 variant 悄悄走進 `else`，編譯器不會提醒你。

所以：只在乎一種情況，用 `if let`；每一種都要有對應的處理，或希望新增 variant 時被編譯器提醒，用 `match`。

`if let` 的值是 `Option<String>` 這類會 move 的型別時，規則跟上一集一樣：`if let Some(name) = nickname` 會把 `String` 拿走，只想看就對 `&nickname` 做 `if let`。

## 重點整理

- `if let 模式 = 值 { ... }`：符合就執行，不符合就跳過；`=` 是單一個等號，不是比較。
- 模式裡綁定的變數只能在大括號裡用；後面可以接 `else`、`else if let`，整個 `if let` 是運算式。
- 它等於「只有一個有用分支、其餘 `_ => {}`」的 `match` 的簡寫。
- `if let` 不要求窮盡，新增 variant 時不會被編譯器提醒；要逐一處理就用 `match`。
- 對 `Option<String>` 這類會 move 的值，規則跟 `match` 一樣：只想看就對 `&值` 做 `if let`。
