# `thiserror`

## 本集目標

用 `thiserror` 的 `#[derive(Error)]` 取代第 9 章手寫的 `Display`、`Error`、`From`，會用 `#[error("…")]`、`#[from]`、`#[source]`，也知道錯誤鏈怎麼走。

## 正文

### 把手寫的部分交給 derive

第 9 章為了一個錯誤 enum，要手寫 `Display`、`impl Error`（第 9 章第 8 集）和每種來源一個 `From`（第 9 章第 9 集），全是套路。`thiserror` 把這些套路變成屬性，錯誤型別還是普通的 enum：

```toml
[dependencies]
thiserror = "2.0.21"
```

下面是第 9 章第 9 集的 `OrderError` 改用 `thiserror` 的樣子，`order_total` 與 `main` 的做法一模一樣：

```rust
use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
enum OrderError {
    #[error("一次最多 10 杯，不能點 {0} 杯")]
    TooMany(u32),
    #[error("數量不對：{0}")]
    BadQty(#[from] std::num::ParseIntError),
    #[error("加料欄位不對：{0}")]
    BadExtra(#[from] std::str::ParseBoolError),
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
            Err(e) => println!("{e}（有原因：{}）", e.source().is_some()),
        }
    }
}
```

執行結果：

```text
80 元
數量不對：invalid digit found in string（有原因：true）
加料欄位不對：provided string was not `true` or `false`（有原因：true）
一次最多 10 杯，不能點 20 杯（有原因：false）
```

- `#[derive(Error)]` 替型別實作 `std::error::Error`。`Error` 要求同時有 `Debug` 與 `Display`，`Debug` 照舊自己 derive。
- `#[error("…")]` 產生 `Display`：字串裡用 `{0}`、`{1}` 取 tuple 欄位，具名欄位用 `{名稱}`，格式跟 `format!` 一樣。
- `#[from]` 做兩件事：替這種來源產生 `From`（所以 `?` 會自動轉換，第 9 章第 9 集），並把它登記成這個錯誤的「原因」（`source()`）。所以 `BadQty` 的 `source()` 有值，`TooMany` 沒有。
- 檔案上面同時有 `use std::error::Error;`（trait，為了呼叫 `source()`）與 `use thiserror::Error;`（derive 巨集）。它們同名但在不同的名稱空間，可以同時 `use`。

### 具名欄位與錯誤鏈

`#[from]` 只能用在「只有一個欄位」的 variant。想在原因之外多帶資訊（第幾行、哪個檔案），就用具名欄位，並在原因欄位上標 `#[source]`（不產生 `From`，建立時自己填）：

```rust
use std::error::Error;
use std::io::ErrorKind;
use thiserror::Error;

#[derive(Debug, Error)]
enum MenuError {
    #[error("讀不到菜單 {name}")]
    Read {
        name: String,
        #[source]
        source: std::io::Error,
    },
    #[error("第 {line} 行的價格不對")]
    BadPrice {
        line: usize,
        #[source]
        source: std::num::ParseIntError,
    },
}

fn load(name: &str) -> Result<u32, MenuError> {
    if name != "menu.txt" {
        let source = std::io::Error::new(ErrorKind::NotFound, "檔案不見了");
        return Err(MenuError::Read {
            name: name.to_string(),
            source,
        });
    }
    let price = "三十"
        .parse::<u32>()
        .map_err(|source| MenuError::BadPrice { line: 2, source })?;
    Ok(price)
}

fn main() {
    for name in ["old.txt", "menu.txt"] {
        let e = load(name).expect_err("應該失敗");
        println!("{e}");
        let mut cause = e.source();
        while let Some(inner) = cause {
            println!("  原因：{inner}");
            cause = inner.source();
        }
    }
}
```

執行結果：

```text
讀不到菜單 old.txt
  原因：檔案不見了
第 2 行的價格不對
  原因：invalid digit found in string
```

- 最外層的訊息說「哪裡出事」，`source()` 逐層往裡走，直到最底下的原因（回傳 `None`）。這就是錯誤鏈：每一層只描述自己知道的事，不重複底下的訊息。
- 範例用 `std::io::Error::new(ErrorKind::NotFound, "…")` 自己造一個 `io::Error`，訊息才不依作業系統而不同。
- `map_err(|source| …)` 是第 11 章第 18 集的組合子，欄位名稱跟變數同名，所以簡寫成 `source`。
- 另外還有 `#[error(transparent)]`：`Display` 與 `source()` 都直接交給裡面那個錯誤，適合只是轉包一層的 variant，這時 `{e}` 印出來就是底下錯誤的訊息。

### 什麼時候用

`thiserror` 用在**呼叫端要判斷是哪種錯誤**的地方：函式庫、模組的邊界、需要對不同錯誤做不同處理的程式。因為錯誤是 enum，呼叫端可以 `match` 它的 variant，編譯器會檢查有沒有漏掉（第 6 章第 4 集）。它只是 derive，展開出來的就是你本來會手寫的程式碼，公開的 API 裡看不到 `thiserror` 的型別。

如果程式只是想把任何錯誤往上丟、最後印給使用者看，不在乎是哪一種，就不值得替每一種寫 enum，下一集的 `anyhow` 是為這種情況設計的。兩者常常一起用：函式庫用 `thiserror`，應用程式的 `main` 附近用 `anyhow`。

## 重點整理

- `thiserror = "2.0.21"`；`#[derive(Debug, Error)]` 替錯誤型別實作 `std::error::Error`，省掉手寫 `Display`、`Error`、`From`。
- `#[error("…")]` 產生 `Display`，`{0}` 取 tuple 欄位、`{名稱}` 取具名欄位；`#[from]` 產生 `From` 並登記為 `source()`，只能用在單一欄位的 variant；`#[source]` 只登記原因；`#[error(transparent)]` 直接轉交給裡面的錯誤。
- `Error::source()` 一層層往下走就是錯誤鏈；每一層只說自己這層的訊息。
- 呼叫端要 `match` 錯誤種類時用 `thiserror`；只想往上丟、印出來就好，用 `anyhow`。
