# `let else`

## 本集目標

用 `let 模式 = 值 else { ... };` 在不符合模式時提早離開，符合的話，取出的變數可以在後面的程式裡直接用。

## 正文

### `if let` 的巢狀問題

要查兩個商品的價格，兩個都找到才加總。用 `if let` 寫，每多查一個就多一層縮排：

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

fn nested(a: u32, b: u32) {
    if let Some(x) = find_price(a) {
        if let Some(y) = find_price(b) {
            println!("共 {} 元", x + y);
        } else {
            println!("查無商品 {b}");
        }
    } else {
        println!("查無商品 {a}");
    }
}

fn flat(a: u32, b: u32) {
    let Some(x) = find_price(a) else {
        println!("查無商品 {a}");
        return;
    };
    let Some(y) = find_price(b) else {
        println!("查無商品 {b}");
        return;
    };
    println!("共 {} 元", x + y);
}

fn main() {
    nested(1, 2);
    nested(1, 9);
    flat(1, 2);
    flat(1, 9);
}
```

執行結果：

```text
共 75 元
查無商品 9
共 75 元
查無商品 9
```

`nested` 和 `flat` 做的事一模一樣。`flat` 用的是 `let else`，寫法是 `let 模式 = 值 else { ... };`：

- 值符合模式：模式裡的變數（`x`、`y`）建立好，**留在外層**，後面的程式直接用，不用再包進大括號。
- 值不符合：執行 `else` 區塊，區塊必須離開，這裡用的是 `return`。
- 結尾要加分號，因為整個還是一條 `let`。

這就是 Go 常見的「先處理失敗、提早離開」：`if err != nil { return }` 之後，主要的流程留在最外層。

### `else` 區塊一定要離開

`else` 區塊如果沒有離開，後面的 `p` 就可能沒有值，編譯器不接受：

```rust,compile_fail=E0308
fn main() {
    let price: Option<u32> = None;
    let Some(p) = price else {
        println!("查無商品");
    };
    println!("{p}");
}
```

編譯錯誤：

```text
error[E0308]: `else` clause of `let...else` does not diverge
 --> src/main.rs:3:30
  |
3 |       let Some(p) = price else {
  |  ______________________________^
4 | |         println!("查無商品");
5 | |     };
  | |_____^ expected `!`, found `()`
  |
  = note:   expected type `!`
          found unit type `()`
  = help: try adding a diverging expression, such as `return` or `panic!(..)`
  = help: ...or use `match` instead of `let...else`
```

`` expected `!`, found `()` ``：`!` 的意思是「這個區塊不會正常結束」。能做到的有 `return`、`break`、`continue`，以及會讓程式結束的 `panic!`。所以在迴圈裡，`let else` 常搭配 `continue` 跳過這一圈：

```rust
fn main() {
    let prices = [Some(30), None, Some(45)];
    for item in prices {
        let Some(price) = item else {
            continue;
        };
        println!("{price} 元");
    }
}
```

執行結果：

```text
30 元
45 元
```

迴圈跳過了 `None`，只印出有價格的兩筆。

### 跟 `if let` 怎麼選

- 要處理「符合」與「不符合」兩邊各自的事，用 `if let ... else`。
- 「不符合就離開」，主要流程在後面，用 `let else`。
- 想知道所有情況都被處理了，用 `match`。

## 重點整理

- `let 模式 = 值 else { ... };`：符合時變數留在外層可用；不符合時執行 `else` 區塊。
- `else` 區塊必須離開（`return`、`break`、`continue`、`panic!`），否則是 E0308（``expected `!` ``）。
- 多個「查不到就離開」的步驟，用 `let else` 可以避免 `if let` 一層層巢狀。
- 結尾的分號不能省。
