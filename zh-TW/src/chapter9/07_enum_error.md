# 用 enum 當錯誤型別

## 本集目標

用自己的 enum 當 `Result` 的錯誤型別，讓每一種失敗各有一個 variant；並替自己的型別實作 `FromStr`，讓它能 `parse`。

## 正文

### 為什麼不用 `String`

上一集的錯誤原因是 `String`。呼叫的人拿到之後只能印出來；想分辨「沒填」還是「填錯」，只能去比對文字，訊息一改就壞了。錯誤型別換成 enum：每種失敗各有一個 variant，需要的資料放進 variant 裡。呼叫的人用 `match` 分辨，而且 `match` 要窮盡（第 6 章第 4 集），之後新增一種失敗，所有漏處理的地方都會被編譯器挑出來。

```rust
#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

enum SizeError {
    Empty,
    Unknown(String),
}

fn parse_size(text: &str) -> Result<Size, SizeError> {
    match text {
        "S" => Ok(Size::Small),
        "M" => Ok(Size::Medium),
        "L" => Ok(Size::Large),
        "" => Err(SizeError::Empty),
        other => Err(SizeError::Unknown(String::from(other))),
    }
}

fn main() {
    for text in ["M", "", "XL"] {
        match parse_size(text) {
            Ok(size) => println!("{text:?} → {size:?}"),
            Err(SizeError::Empty) => println!("{text:?} → 還沒選杯型"),
            Err(SizeError::Unknown(other)) => println!("{text:?} → 沒有 {other} 這種杯型"),
        }
    }
}
```

執行結果：

```text
"M" → Medium
"" → 還沒選杯型
"XL" → 沒有 XL 這種杯型
```

- `SizeError` 就是第 6 章的 enum：`Empty` 不帶資料，`Unknown` 帶著填錯的文字。
- `parse_size` 回傳 `Result<Size, SizeError>`，成功是 `Size`，失敗是 `SizeError`。
- `match` 的模式可以一路寫進去：`Err(SizeError::Unknown(other))` 同時比對 `Err`、`Unknown`，並取出裡面的文字。
- `match text` 拿 `&str` 對字串字面值比對，跟第 6 章對整數比對是同一回事；最後的 `other =>` 是第 6 章第 5 集的綁定變數，接住其他所有文字。

### 實作 `FromStr`

第 8 章第 14 集說過，`parse` 會呼叫目標型別的 `from_str`（`FromStr` trait）。那時還不能自己寫，因為 `from_str` 回傳 `Result`。現在可以了：把上面的函式搬進 `impl std::str::FromStr for Size`，`Size` 就能 `parse`：

```rust
#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
enum SizeError {
    Empty,
    Unknown(String),
}

impl std::str::FromStr for Size {
    type Err = SizeError;

    fn from_str(text: &str) -> Result<Size, SizeError> {
        match text {
            "S" => Ok(Size::Small),
            "M" => Ok(Size::Medium),
            "L" => Ok(Size::Large),
            "" => Err(SizeError::Empty),
            other => Err(SizeError::Unknown(String::from(other))),
        }
    }
}

fn main() {
    let a: Size = "M".parse().expect("杯型有誤");
    println!("{a:?}");
    println!("{:?}", "L".parse::<Size>());
    match "XL".parse::<Size>() {
        Ok(size) => println!("{size:?}"),
        Err(SizeError::Empty) => println!("還沒選杯型"),
        Err(SizeError::Unknown(other)) => println!("沒有 {other} 這種杯型"),
    }
}
```

執行結果：

```text
Medium
Ok(Large)
沒有 XL 這種杯型
```

- `type Err = SizeError;`：告訴 `FromStr`，失敗時用哪個型別。它是第 8 章第 17 集的關聯型別，也就是第 8 章第 14 集 `parse` 簽名裡的 `F::Err`。
- `fn from_str(text: &str) -> Result<Size, SizeError>`：函式名稱、參數型別是固定的。trait 裡的回傳型別寫成 `Result<Self, Self::Err>`；`Self` 附錄一 e 才教，所以 `impl` 裡照舊寫出實際的型別。
- 方法本體跟 `parse_size` 一模一樣。寫好之後，`"M".parse::<Size>()` 和 `let a: Size = "M".parse()` 都能用，就像標準函式庫的 `i32`、`bool`。
- 這次 `SizeError` 也加了 `#[derive(Debug)]`。`.expect()` 失敗時要把 `Err` 裡的值用 `{:?}` 印出來，所以錯誤型別一定要有 `Debug`；沒有的話是 E0277（`doesn't implement Debug`），`help` 會直接建議你加上 `derive`。自己定義的錯誤型別，一開始就 `derive(Debug)`。

## 重點整理

- 錯誤型別用 enum：每一種失敗一個 variant，需要的資料放進 variant；呼叫的人用 `match` 分辨，新增 variant 時，沒處理的 `match` 會被編譯器挑出來。
- 模式可以寫成 `Err(SizeError::Unknown(other))`，一次比對 `Err` 與 variant 並取出資料。
- 實作 `FromStr`：`impl std::str::FromStr for 型別 { type Err = 錯誤型別; fn from_str(text: &str) -> Result<型別, 錯誤型別> { ... } }`，之後就能 `"...".parse::<型別>()`。
- `impl` 裡寫實際的型別，不寫 `Self`（附錄一 e 才教）。
- 錯誤型別要 `#[derive(Debug)]`，否則 `.expect()`、`.unwrap()` 會是 E0277。
