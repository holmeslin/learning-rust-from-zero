# `Box<dyn Error>`

## 本集目標

用 `Box<dyn std::error::Error>` 當錯誤型別，讓一個函式裡各種不同的錯誤都能用 `?` 收下，不必替每一種寫 `From`。

## 正文

### 什麼錯誤都收

上一集每多一種錯誤來源，就要多寫一個 `From`。有些程式（小工具、`main` 附近的流程）只想把錯誤印出來，不在乎是哪一種。這時錯誤型別可以寫成 `Box<dyn std::error::Error>`：任何實作了 `Error` 的型別（上一集）都裝得進去。`dyn Error` 是第 8 章第 15 集的 trait 物件，`Box` 先照抄，第 13 章第 1 集才解釋。

```rust
#[derive(Debug)]
struct TooMany(u32);

impl std::fmt::Display for TooMany {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "一次最多 10 杯，不能點 {} 杯", self.0)
    }
}

impl std::error::Error for TooMany {}

fn order_total(qty: &str, extra: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let qty: u32 = qty.parse()?;
    let extra: bool = extra.parse()?;
    if qty == 0 {
        return Err("至少要點一杯".into());
    }
    if qty > 10 {
        return Err(Box::new(TooMany(qty)));
    }
    let unit = if extra { 40 } else { 30 };
    Ok(qty * unit)
}

fn main() {
    for (qty, extra) in [
        ("2", "true"),
        ("x", "true"),
        ("0", "false"),
        ("20", "false"),
    ] {
        match order_total(qty, extra) {
            Ok(total) => println!("{total} 元"),
            Err(e) => println!("出錯了：{e}"),
        }
    }
}
```

執行結果：

```text
80 元
出錯了：invalid digit found in string
出錯了：至少要點一杯
出錯了：一次最多 10 杯，不能點 20 杯
```

- 回傳型別 `Result<u32, Box<dyn std::error::Error>>`：失敗的原因可以是任何一種錯誤。
- 兩個 `parse` 的 `?` 不用任何 `From`：`ParseIntError`、`ParseBoolError` 都實作了 `Error`，標準函式庫替所有 `Error` 型別準備好了「裝進 `Box<dyn Error>`」的轉換，`?` 自動套用。
- 自己的錯誤型別（`TooMany`）也行，前提是實作了 `Debug`、`Display`、`Error`（上一集）。`return Err(Box::new(TooMany(qty)))` 把它裝進去。
- 只有一段文字的簡單錯誤，`Err("至少要點一杯".into())` 就夠了：`.into()`（第 8 章第 13 集）靠函式的回傳型別知道要轉成什麼。
- 印出來用 `{e}`，會呼叫它真正型別的 `Display`，所以 `x` 的訊息是標準函式庫的 `invalid digit found in string`，`20` 的是 `TooMany` 的訊息。

### 想知道原本是哪種錯誤

裝進 `Box<dyn Error>` 之後，編譯器只知道「它是個錯誤」，不知道是哪一種。需要的時候，可以用 `downcast_ref` 問問看是不是某個型別：

```rust
#[derive(Debug)]
struct TooMany(u32);

impl std::fmt::Display for TooMany {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "一次最多 10 杯，不能點 {} 杯", self.0)
    }
}

impl std::error::Error for TooMany {}

fn check(qty: u32) -> Result<u32, Box<dyn std::error::Error>> {
    if qty > 10 {
        return Err(Box::new(TooMany(qty)));
    }
    Ok(qty)
}

fn main() {
    if let Err(e) = check(20) {
        match e.downcast_ref::<TooMany>() {
            Some(TooMany(n)) => println!("超量：{n} 杯"),
            None => println!("出錯了：{e}"),
        }
    }
}
```

執行結果：

```text
超量：20 杯
```

`downcast_ref::<TooMany>()` 回傳 `Option<&TooMany>`：真的是 `TooMany` 就是 `Some`，可以用模式取出裡面的值；不是就是 `None`。要靠這招分辨錯誤的種類，代表你需要的是上一集的 enum，所以 `Box<dyn Error>` 適合「只要印出來、記下來」的地方，需要依錯誤種類做不同處理的函式，仍然用自己的 enum。

### 跟 Go 比一比

Go 的 `error` 本來就是介面，每個函式的回傳型別都是 `error`，什麼錯誤都能放進去；要取回具體型別，用 `errors.As`：

```go
var tooMany TooMany
if errors.As(err, &tooMany) {
	fmt.Println("超量：", tooMany.N)
}
```

`Box<dyn Error>` 就是 Rust 版的 `error`，`downcast_ref` 對應 `errors.As`。差別在預設值：Go 一開始就是介面，Rust 預設是具體的型別，要「什麼都收」得自己選 `Box<dyn Error>`，同時放棄用 `match` 分辨種類的好處。

## 重點整理

- `Result<T, Box<dyn std::error::Error>>`：錯誤型別收下任何實作了 `Error` 的型別，`?` 會自動把各種錯誤裝進去，不必寫 `From`。
- 自己的錯誤要實作 `Debug`、`Display`、`Error`，用 `Err(Box::new(...))` 回傳；簡單文字用 `Err("訊息".into())`。
- `Box<dyn Error>` 只知道「是個錯誤」；要問是哪個型別，用 `e.downcast_ref::<型別>()`，回傳 `Option<&型別>`。
- 適合只要印出或記錄錯誤的地方；需要依錯誤種類分別處理，用自己的 enum（第 7 集）。
- 跟 Go 比：Go 的 `error` 天生是介面，`errors.As` 對應 `downcast_ref`；Rust 預設是具體型別，要選 `Box<dyn Error>` 才什麼都收。
