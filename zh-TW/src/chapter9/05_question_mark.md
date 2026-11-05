# `?` 運算子

## 本集目標

用 `?` 取代巢狀的 `match`：成功就取出值繼續往下，失敗就把錯誤原樣交給呼叫的人。

## 正文

### 層層巢狀的 `match`

函式裡有兩個步驟都可能失敗：數量和單價都要從文字轉成數字。用 `match` 寫，每多一步就多一層：

```rust
fn order_total(qty: &str, price: &str) -> Result<u32, std::num::ParseIntError> {
    match qty.parse::<u32>() {
        Ok(q) => match price.parse::<u32>() {
            Ok(p) => Ok(q * p),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn main() {
    println!("{:?}", order_total("3", "30"));
    println!("{:?}", order_total("x", "30"));
    println!("{:?}", order_total("3", "y"));
}
```

執行結果：

```text
Ok(90)
Err(ParseIntError { kind: InvalidDigit })
Err(ParseIntError { kind: InvalidDigit })
```

`std::num::ParseIntError` 是 `parse` 整數失敗時的錯誤型別。每個 `Err(e) => Err(e)` 都只是把錯誤原封不動地交出去，卻佔了兩行。

### `?`：成功往下，失敗就回傳

在回傳 `Result` 的函式裡，運算式後面加一個 `?`：

```rust
fn order_total(qty: &str, price: &str) -> Result<u32, std::num::ParseIntError> {
    let qty: u32 = qty.parse()?;
    let price: u32 = price.parse()?;
    Ok(qty * price)
}

fn main() {
    println!("{:?}", order_total("3", "30"));
    println!("{:?}", order_total("x", "30"));
    println!("{:?}", order_total("3", "y"));
}
```

執行結果：

```text
Ok(90)
Err(ParseIntError { kind: InvalidDigit })
Err(ParseIntError { kind: InvalidDigit })
```

結果跟上一個版本完全一樣。`?` 的規則只有兩條：

- 運算式是 `Ok(值)`：整個 `qty.parse()?` 的值就是 `值`，程式繼續往下。
- 運算式是 `Err(e)`：函式立刻 `return Err(e)`，後面的程式都不執行。

所以 `?` 等於上一節那段 `match`：`Ok(v) => v`，`Err(e) => return Err(e)`。最後一行要自己包成 `Ok(qty * price)`，因為函式回傳的是 `Result`，不是 `u32`。型別標註 `: u32` 讓 `parse` 知道要轉成什麼，跟第 8 章第 14 集一樣。

`?` 做的事，跟 Go 的 `if err != nil { return err }` 一樣，第 12 集再好好比。

### `?` 只能用在回傳 `Result` 的函式

`?` 的「失敗就 return」要有地方 return。放在回傳 `()` 的 `main` 裡，編譯不過：

```rust,compile_fail=E0277
fn main() {
    let n: u32 = "30".parse()?;
    println!("{n}");
}
```

編譯錯誤：

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:2:30
  |
1 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
2 |     let n: u32 = "30".parse()?;
  |                              ^ cannot use the `?` operator in a function that returns `()`
  |
help: consider adding return type
  |
1 ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
2 |     let n: u32 = "30".parse()?;
3 |     println!("{n}");
4 +     Ok(())
  |
```

`` the `?` operator can only be used in a function that returns `Result` or `Option` ``：`main` 回傳的是 `()`，沒辦法 `return Err(...)`。`help` 建議的 `Result<(), Box<dyn std::error::Error>>` 是第 10、11 集的內容，現在先忽略。目前先這樣做：另外寫一個回傳 `Result` 的函式，放會失敗的部分，在 `main` 裡用 `match` 處理。

另外，`?` 兩邊的錯誤型別要對得上：上面的函式回傳 `ParseIntError`，兩個 `parse` 失敗時的錯誤也剛好是 `ParseIntError`。對不上的時候怎麼辦，第 9 集會解決。

## 重點整理

- `運算式?`：在回傳 `Result` 的函式裡，`Ok(值)` 就取出值，`Err(e)` 就立刻 `return Err(e)`。
- 多個步驟都可能失敗時，`?` 取代巢狀的 `match`；最後要用 `Ok(...)` 包起結果。
- `?` 只能用在回傳 `Result`（或 `Option`，下一集）的函式裡；在回傳 `()` 的 `main` 裡是 E0277。
- `?` 兩邊的錯誤型別要對得上，對不上的處理方式第 9 集再說。
- `?` 對應 Go 的 `if err != nil { return err }`。
