# `anyhow`

## 本集目標

用 `anyhow` 在應用程式裡不分種類地往上丟錯誤：`anyhow::Result`、`.context()` 加上說明、`bail!` 與 `ensure!`，並看懂三種印法（`{e}`、`{e:#}`、`{e:?}`）的差別。

## 正文

### 什麼錯誤都收

第 9 章第 10 集的 `Box<dyn Error>` 讓函式能收任何錯誤。`anyhow` 是它的加強版：`anyhow::Error` 一樣什麼錯誤都裝得下、`?` 會自動轉換，還多了「替錯誤加上說明」和錯誤鏈的顯示：

```toml
[dependencies]
anyhow = "1.0.104"
```

`anyhow::Result<T>` 就是 `Result<T, anyhow::Error>`，連錯誤型別都不用寫。`main` 也能回傳它：

```rust,exit=1
use anyhow::Result;

fn main() -> Result<()> {
    let n: u32 = "abc".parse()?;
    println!("{n}");
    Ok(())
}
```

執行結果：

```text
Error: invalid digit found in string
```

跟第 9 章第 11 集一樣：`main` 回傳 `Err`，程式印出 `Error: ` 加錯誤的內容，結束碼是 1。

### `.context()`：每一層都說清楚發生什麼事

只看到 `invalid digit found in string` 不知道是哪一份資料、哪一步。`.context("說明")` 把原本的錯誤包一層，附上你的說明；`.with_context(|| …)` 是閉包版，說明要用 `format!` 組出來時用它，只有出錯才會執行：

```rust
use anyhow::{Context, Result};

fn parse_price(text: &str) -> Result<u32> {
    let price: u32 = text.trim().parse().context("價格不是整數")?;
    Ok(price)
}

fn load_menu(lines: &[&str]) -> Result<u32> {
    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        let price = parse_price(line).with_context(|| format!("第 {} 行有問題", i + 1))?;
        total += price;
    }
    Ok(total)
}

fn main() {
    if let Err(e) = load_menu(&["30", "四十五"]) {
        println!("{e}");
        println!("{e:#}");
        println!("{e:?}");
    }
}
```

執行結果：

```text
第 2 行有問題
第 2 行有問題: 價格不是整數: invalid digit found in string
第 2 行有問題

Caused by:
    0: 價格不是整數
    1: invalid digit found in string
```

同一個錯誤，三種印法：

- `{e}` 只印最外層的一句：`第 2 行有問題`。
- `{e:#}` 把整條鏈用 `: ` 串成一行：`第 2 行有問題: 價格不是整數: invalid digit found in string`，適合寫進 log。
- `{e:?}` 分行，底下列出 `Caused by:` 與每一層的原因，編號 0 是離外層最近的一個。`main` 回傳 `Err` 時印的就是這種格式。

`context` 是 `Context` 這個 trait 的方法，要 `use anyhow::Context;`；它對 `Result` 與 `Option` 都能用（`None` 時變成錯誤）。

### `anyhow!`、`bail!`、`ensure!`

不是來自別的函式、要自己造錯誤時，用三個巨集（第 16 章第 1 集）：`anyhow!("…")` 建立錯誤，`bail!("…")` 等於 `return Err(anyhow!("…"))`，`ensure!(條件, "…")` 在條件不成立時 `bail!`：

```rust
use anyhow::{Result, anyhow, bail, ensure};

fn check(cups: u32) -> Result<u32> {
    ensure!(cups > 0, "至少要點一杯");
    if cups > 10 {
        bail!("一次最多 10 杯，不能點 {cups} 杯");
    }
    if cups == 7 {
        return Err(anyhow!("{cups} 杯沒有這種優惠"));
    }
    Ok(cups * 30)
}

fn main() {
    for cups in [2, 0, 11, 7] {
        match check(cups) {
            Ok(total) => println!("{total} 元"),
            Err(e) => println!("{e}"),
        }
    }
}
```

執行結果：

```text
60 元
至少要點一杯
一次最多 10 杯，不能點 11 杯
7 杯沒有這種優惠
```

訊息的格式跟 `format!` 一樣，可以直接寫 `{cups}`。

### 要判斷是哪種錯誤時

`anyhow::Error` 把原本的型別藏起來了，函式的簽名看不出它可能是什麼。需要時可以用 `e.downcast_ref::<自己的錯誤型別>()` 還原：是那個型別就得到 `Some(&…)`，不是就是 `None`。上一集用 `thiserror` 定義的 enum 能直接用 `?` 或 `.into()` 放進 `anyhow::Error`，處理的地方再 `downcast_ref` 拿回來 `match`。不過這是例外：常常要判斷種類的地方，就該直接回傳 `thiserror` 的 enum。

兩個小提醒：

- 環境變數 `RUST_BACKTRACE=1` 打開時，`{e:?}` 與 `main` 回傳 `Err` 的輸出最後會多一長串堆疊追蹤（`Stack backtrace:`）。書上的輸出都是沒設定的樣子。
- 用在應用程式（`main`、命令列工具、伺服器的最外層），函式庫的公開函式請回傳 `thiserror` 的型別，不要讓呼叫端只拿到一個看不出內容的 `anyhow::Error`。

### 跟 Go 比一比

```go
err = fmt.Errorf("第 %d 行有問題: %w", 2, err)
fmt.Println(err) // 第 2 行有問題: strconv.Atoi: parsing "四十五": invalid syntax
```

- Go 的 `%w` 包裝後，`err.Error()` 就是整條鏈用 `: ` 串起來的字串；`anyhow` 的 `{e}` 只印最外層，要 Go 那種一行，得寫 `{e:#}`。只印 `{e}` 是最常見的「怎麼少了原因」。
- Go 用 `errors.Is`、`errors.As` 在鏈裡找特定的錯誤；`anyhow` 對應的是 `downcast_ref`，不過用到它多半表示該換成 `thiserror` 的 enum 與 `match`。

## 重點整理

- `anyhow = "1.0.104"`；`anyhow::Result<T>` 是 `Result<T, anyhow::Error>`，`?` 會把任何錯誤轉進去；`main` 也能回傳它，出錯印 `Error: …`、結束碼 1。
- `.context("說明")`、`.with_context(|| …)` 替錯誤加一層說明（要 `use anyhow::Context;`）；`{e}` 只印最外層，`{e:#}` 一行串起整條鏈，`{e:?}` 分行列出 `Caused by:`。
- `anyhow!` 建立錯誤、`bail!` 直接回傳錯誤、`ensure!` 條件不成立就回傳錯誤。
- 用 `downcast_ref` 還原原本的型別；常常要判斷種類的地方改用 `thiserror`。應用程式用 `anyhow`，函式庫用 `thiserror`。
- 設了 `RUST_BACKTRACE=1`，錯誤的 Debug 輸出會多出堆疊追蹤。
