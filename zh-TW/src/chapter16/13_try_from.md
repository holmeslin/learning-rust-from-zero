# `TryFrom` 與 `TryInto`

## 本集目標

會用 `T::try_from(x)` 與 `x.try_into()` 做「可能失敗」的轉換，從 `Result` 看出成功或失敗；會替自己的型別實作 `TryFrom`；並分得清楚 `From`、`TryFrom`、`as` 各自該用在哪裡。

## 正文

### 放不下，就回傳 `Err`

第 8 章第 13 集的 `From` 用在一定轉得過去的地方，例如 `i64::from(30_i32)`。附錄一 i 的 `as` 則是不管放不放得下，一律截斷。中間還缺一種：**轉得過去就轉，轉不過去就告訴我**。這就是 `TryFrom`，轉換的結果是 `Result`：

```rust
fn main() {
    let big: i32 = 300;
    println!("{}", big as u8);
    println!("{:?}", u8::try_from(big));
    println!("{:?}", u8::try_from(200_i32));
    println!("{:?}", u8::try_from(-1_i32));
}
```

執行結果：

```text
44
Err(TryFromIntError(PosOverflow))
Ok(200)
Err(TryFromIntError(NegOverflow))
```

- `big as u8` 悄悄截斷成 44，沒有任何警告。
- `u8::try_from(big)` 回傳 `Err`，300 放不下 `u8`。錯誤型別是 `TryFromIntError`，`{:?}` 印出來會帶「太大」（`PosOverflow`）或「太小」（`NegOverflow`）；用 `{}` 印是一句英文：`number too large to fit in target type`。
- 放得下就是 `Ok(200)`。

`TryFrom` 與 `TryInto` 都在 prelude 裡，不用 `use`。

### `try_into`

跟 `From` 與 `Into` 的關係一樣，實作了 `TryFrom`，就自動得到 `TryInto`：`x.try_into()` 等於 `T::try_from(x)`。跟 `.into()` 一樣，要有型別標註，沒有標註是 E0283：

```rust
fn main() {
    let small: Result<u8, _> = 300_i32.try_into();
    println!("{small:?}");
    let n: u8 = 200_i32.try_into().expect("放得下 u8");
    println!("{n}");
}
```

執行結果：

```text
Err(TryFromIntError(PosOverflow))
200
```

`Result<u8, _>` 的 `_` 請編譯器自己填（第 11 章第 11 集）。結果是 `Result`，所以用 `match`、`expect` 都行，函式回傳 `Result` 的話也能用 `?`：`let cups = u8::try_from(n)?;` 會把 `TryFromIntError` 轉成函式宣告的錯誤型別（例如 `Box<dyn std::error::Error>`，第 9 章第 10 集），跟其他錯誤的用法完全一樣。

### 替自己的型別實作 `TryFrom`

第 9 章第 7 集的 `FromStr` 是「從文字轉成我的型別，可能失敗」。數字也一樣：杯子的容量是毫升，只有三種規格，其他數字都不行。實作 `TryFrom<u32>`：

```rust
#[derive(Debug)]
enum Size {
    S,
    M,
    L,
}

#[derive(Debug)]
struct SizeError;

impl TryFrom<u32> for Size {
    type Error = SizeError;

    fn try_from(ml: u32) -> Result<Size, SizeError> {
        match ml {
            250 => Ok(Size::S),
            350 => Ok(Size::M),
            500 => Ok(Size::L),
            _ => Err(SizeError),
        }
    }
}

fn main() {
    println!("{:?}", Size::try_from(350));
    let bad: Result<Size, _> = 400_u32.try_into();
    println!("{bad:?}");
    println!("{:?}", Size::try_from(250));
    println!("{:?}", Size::try_from(500));
}
```

執行結果：

```text
Ok(M)
Err(SizeError)
Ok(S)
Ok(L)
```

- `impl TryFrom<u32> for Size`：從 `u32` 轉成 `Size`。
- `type Error = SizeError;` 指定失敗時的錯誤型別，跟 `FromStr` 的 `type Err` 同一種寫法（第 9 章第 7 集），只是名字叫 `Error`。
- `try_from` 的回傳型別是 `Result<Size, SizeError>`，用 `match`（第 6 章）決定成功還是失敗。
- 不用自己寫 `TryInto`：實作了 `TryFrom`，`400_u32.try_into()` 就是白送的。

### 該用哪一個

| 情況 | 用哪個 |
| --- | --- |
| 一定放得下（`i32` 轉 `i64`） | `From`、`.into()` |
| 可能放不下，要知道有沒有失敗 | `TryFrom`、`.try_into()` |
| 放不下也要截斷（雜湊、位元運算） | `as`（附錄一 i） |

常見的用法還有 `usize::try_from(n)`：把可能是負數的 `i32` 轉成索引；以及 `char::try_from(0x2615_u32)`，結果是 `Ok('☕')`，附錄一 i 的 `u32 as char` 是 E0604，這就是它的安全版本。`char::try_from(0xD800_u32)`（不是合法字元的編碼）則是 `Err`。

### 跟 Go 比一比

Go 的整數轉換 `uint8(big)` 跟 Rust 的 `as` 一樣，放不下就悄悄截斷，標準函式庫也沒有「檢查範圍的轉換」。想知道放不放得下，只能自己先判斷，或自己寫函式：

```go
func toCups(n int32) (uint8, error) {
	if n < 0 || n > math.MaxUint8 {
		return 0, fmt.Errorf("%d 放不下 uint8", n)
	}
	return uint8(n), nil
}
```

Rust 把這個模式放進標準函式庫：`u8::try_from(n)` 就是上面那個函式，每個整數型別之間都有，還能搭配 `?`。

## 重點整理

- `T::try_from(x)` 與 `x.try_into()` 是可能失敗的轉換，回傳 `Result`；整數放不下時是 `Err(TryFromIntError)`。`TryFrom` 與 `TryInto` 都在 prelude。
- 實作 `TryFrom<A> for B` 就自動有 `TryInto`：`type Error = 錯誤型別;` 加 `fn try_from(value: A) -> Result<B, 錯誤型別>`。`.try_into()` 跟 `.into()` 一樣要型別標註（E0283）。
- 選擇：一定放得下用 `From`，可能失敗用 `TryFrom`，要截斷才用 `as`。
- 常見用法：`usize::try_from(n)` 轉索引、`char::try_from(u32)` 轉字元。
- Go 的轉換會悄悄截斷，沒有標準的檢查版本，要自己寫範圍判斷；Rust 內建 `TryFrom`。
