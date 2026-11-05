# `?` 會自動呼叫 `From`

## 本集目標

知道 `?` 在遇到 `Err` 時會先用 `From` 轉換錯誤型別，並替自己的錯誤 enum 實作 `From`，讓不同來源的錯誤都能用 `?` 收進同一個型別。

## 正文

### 錯誤型別對不上

第 5 集的 `?` 只在兩邊的錯誤型別一樣時成功。如果函式回傳自己的錯誤 enum，而 `parse` 失敗給的是 `ParseIntError`，就對不上了：

```rust,compile_fail=E0277
#[derive(Debug)]
enum OrderError {
    BadQty(std::num::ParseIntError),
}

fn order_total(qty: &str) -> Result<u32, OrderError> {
    let qty = qty.parse::<u32>()?;
    Ok(qty * 30)
}

fn main() {
    println!("{:?}", order_total("2"));
}
```

編譯錯誤：

```text
error[E0277]: `?` couldn't convert the error to `OrderError`
 --> src/main.rs:7:33
  |
6 | fn order_total(qty: &str) -> Result<u32, OrderError> {
  |                              ----------------------- expected `OrderError` because of this
7 |     let qty = qty.parse::<u32>()?;
  |                   --------------^ the trait `From<ParseIntError>` is not implemented for `OrderError`
  |                   |
  |                   this can't be annotated with `?` because it has type `Result<_, ParseIntError>`
  |
note: `OrderError` needs to implement `From<ParseIntError>`
 --> src/main.rs:2:1
  |
2 | enum OrderError {
  | ^^^^^^^^^^^^^^^
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
```

`` `?` couldn't convert the error to `OrderError` ``：`?` 想把 `ParseIntError` 轉成 `OrderError`，找不到辦法。箭頭旁和 `note` 都說得很白：`OrderError` 沒有實作 `From<ParseIntError>`。最後一則 `note` 點明 `?` 會用 `From` 轉換錯誤，下一節就說這件事。

### `?` 其實會呼叫 `From`

`?` 遇到 `Err(e)` 時，不是原封不動地 `return Err(e)`，而是 `return Err(From::from(e))`：先用第 8 章第 13 集的 `From`，把 `e` 轉成函式回傳的錯誤型別。第 5 集的兩邊型別一樣，標準函式庫有「任何型別都能轉成自己」的 `From`，所以看不出來。

所以只要替 `OrderError` 實作 `From<ParseIntError>`，`?` 就會自動轉換。有幾種來源，就寫幾個 `From`：

```rust
#[derive(Debug)]
enum OrderError {
    TooMany(u32),
    BadQty(std::num::ParseIntError),
    BadExtra(std::str::ParseBoolError),
}

impl From<std::num::ParseIntError> for OrderError {
    fn from(e: std::num::ParseIntError) -> OrderError {
        OrderError::BadQty(e)
    }
}

impl From<std::str::ParseBoolError> for OrderError {
    fn from(e: std::str::ParseBoolError) -> OrderError {
        OrderError::BadExtra(e)
    }
}

fn order_total(qty: &str, extra: &str) -> Result<u32, OrderError> {
    let qty: u32 = qty.parse()?;
    let extra: bool = extra.parse()?;
    if qty > 10 {
        return Err(OrderError::TooMany(qty));
    }
    let unit = if extra { 40 } else { 30 };
    Ok(qty * unit)
}

fn main() {
    for (qty, extra) in [("2", "true"), ("x", "true"), ("2", "yes"), ("20", "false")] {
        match order_total(qty, extra) {
            Ok(total) => println!("{total} 元"),
            Err(OrderError::TooMany(n)) => println!("一次最多 10 杯，不能點 {n} 杯"),
            Err(OrderError::BadQty(e)) => println!("數量不對：{e}"),
            Err(OrderError::BadExtra(e)) => println!("加料欄位不對：{e}"),
        }
    }
}
```

執行結果：

```text
80 元
數量不對：invalid digit found in string
加料欄位不對：provided string was not `true` or `false`
一次最多 10 杯，不能點 20 杯
```

- `impl From<std::num::ParseIntError> for OrderError`：把 `ParseIntError` 包進 `BadQty` variant。兩個 `parse` 失敗時給的型別不同（`ParseIntError`、`ParseBoolError`），`?` 看錯誤的型別，挑對應的 `From`。
- `order_total` 裡的 `?` 完全沒變，轉換發生在背後；包進 variant 的原始錯誤沒有丟掉，`match` 還能取出來印（`{e}`）。
- `TooMany` 不是從別的錯誤轉來的，照舊自己 `return Err(...)`。
- 錯誤型別不是自己的 enum 也一樣：`Result<u32, String>` 的函式裡對 `parse` 用 `?`，要有 `From<ParseIntError> for String` 才行，標準函式庫沒有，所以也是 E0277。

### 跟 Go 比一比

Go 的 `return err` 不會轉換錯誤；想補充說明，要在每個 `return` 的地方自己包：

```go
qty, err := strconv.Atoi(qtyText)
if err != nil {
	return 0, fmt.Errorf("數量不對：%w", err)
}
```

Rust 把「怎麼包」寫在 `From` 一次，之後每個 `?` 都自動套用，不用在每個呼叫處重複。代價是要先寫好 `From`，而且包成什麼，由你替每個來源型別決定。

## 重點整理

- `?` 遇到 `Err(e)` 時是 `return Err(From::from(e))`：用 `From` 把錯誤轉成函式回傳的錯誤型別。
- 錯誤型別對不上又沒有 `From`，是 E0277 `` `?` couldn't convert the error to ... ``。
- 替自己的錯誤 enum 實作 `From<別人的錯誤>`，把它包進一個 variant；有幾種來源就寫幾個 `From`，`?` 依型別挑。
- 型別一樣時不用寫，因為任何型別都能轉成自己。
- 跟 Go 比：Go 在每個 `return` 手動 `fmt.Errorf("...%w", err)` 包裝；Rust 用 `From` 寫一次，`?` 自動套用。
