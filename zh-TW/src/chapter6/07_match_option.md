# 用 `match` 處理 `Option`

## 本集目標

用 `match` 對 `Some` 和 `None` 各寫一個分支，取出 `Option` 裡的值，並知道 `Option<String>` 這類會 move 的值要怎麼 `match`。

## 正文

### `Some` 一個分支、`None` 一個分支

`Option` 只是一個 enum，所以處理它的方法跟上幾集完全一樣：

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

fn show_price(code: u32) {
    match find_price(code) {
        Some(price) => println!("{code} 號：{price} 元"),
        None => println!("{code} 號：沒有這個商品"),
    }
}

fn total(code: u32, qty: u32) -> u32 {
    match find_price(code) {
        Some(price) => price * qty,
        None => 0,
    }
}

fn main() {
    show_price(1);
    show_price(9);
    println!("{}", total(2, 3));
    println!("{}", total(9, 3));
}
```

執行結果：

```text
1 號：30 元
9 號：沒有這個商品
135
0
```

- `Some(price)`：只有值是 `Some` 才符合，括號裡的 `price` 拿到裡面的 `u32`。
- `None`：沒有資料，直接寫。
- `match` 是運算式，每個分支給出同一種型別的值：`total` 在有商品時回傳金額，沒有時回傳 0。

### 一定要處理 `None`

`match` 要窮盡，所以只寫 `Some` 不行：

```rust,compile_fail=E0004
fn find_price(code: u32) -> Option<u32> {
    if code == 1 {
        Some(30)
    } else {
        None
    }
}

fn main() {
    match find_price(9) {
        Some(price) => println!("{price} 元"),
    }
}
```

編譯錯誤：

```text
error[E0004]: non-exhaustive patterns: `None` not covered
  --> src/main.rs:10:11
   |
10 |     match find_price(9) {
   |           ^^^^^^^^^^^^^ pattern `None` not covered
   |
```

這就是上一集說的「Rust 沒有 `nil`」的實際效果：想用 `Some` 裡的值，就一定要面對 `None`，不可能漏掉。訊息後面的 `help` 建議補上 `None => todo!()`，記得換成真正的處理。

### 值會被 move 進變數

`Option<u32>` 裡的 `u32` 是 `Copy`，沒有後遺症。換成 `Option<String>` 就不一樣了：

```rust,compile_fail=E0382
fn main() {
    let nickname = Some(String::from("小美"));
    match nickname {
        Some(name) => println!("暱稱：{name}"),
        None => println!("沒有暱稱"),
    }
    println!("{:?}", nickname);
}
```

編譯錯誤：

```text
error[E0382]: borrow of partially moved value: `nickname`
 --> src/main.rs:7:22
  |
4 |         Some(name) => println!("暱稱：{name}"),
  |              ---- value partially moved here
...
7 |     println!("{:?}", nickname);
  |                      ^^^^^^^^ value borrowed here after partial move
  |
  = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
  |
4 |         Some(ref name) => println!("暱稱：{name}"),
  |              +++
```

`Some(name)` 把 `String` 從 `nickname` 裡拿出來放進 `name`，這是第 3 章的 move，`nickname` 被拿走了一部分（partial move），後面就不能再用。`help` 建議的 `ref name` 是另一種「只借用、不拿走」的寫法，這本書不用，先不管它。

只想看一眼、不想拿走，就像第 4 章一樣借用，對 `&nickname` 做 `match`：

```rust
fn main() {
    let nickname = Some(String::from("小美"));
    match &nickname {
        Some(name) => println!("暱稱：{name}，長度 {}", name.len()),
        None => println!("沒有暱稱"),
    }
    println!("{:?}", nickname);
}
```

執行結果：

```text
暱稱：小美，長度 6
Some("小美")
```

模式還是照樣寫 `Some(name)`，不用改成 `&Some(name)`；編譯器看到被比對的是參照，就讓 `name` 也變成參照（這裡是 `&String`）。用法跟第 4 章的參照一樣：直接印，或呼叫 `name.len()`。`match` 結束後 `nickname` 沒被拿走，所以最後一行還印得出來。

## 重點整理

- `match` 處理 `Option`：`Some(值) => ...`、`None => ...`，跟其他 enum 的規則相同。
- `Some(price)` 把裡面的值放進新變數；`None` 沒有資料。
- 只寫 `Some` 不寫 `None` 是 E0004：想用值，就一定得處理「沒有」的情況。
- `Option<String>` 這類不是 `Copy` 的值，`match nickname` 會把裡面的值 move 出來（E0382）；只想看就寫 `match &nickname`，綁到的變數是參照。
- 之後會看到的 `.unwrap()`、`.expect("...")`，是「`Some` 取值、`None` 就 panic」的簡寫，第 9 章第 4 集教。
