# `FromStr` 與 `parse` 的原理

## 本集目標

知道 `parse` 靠目標型別的 `FromStr` 來轉換，所以同一段文字轉成不同型別，規則不一樣；並看得懂目標型別沒有 `FromStr` 的錯誤。

## 正文

### 目標型別決定怎麼轉

第 1 章第 10 集的固定句型 `let n: i32 = line.parse().expect(...)`，左邊的 `: i32` 就是在告訴 `parse`：要轉成什麼。`parse` 自己不知道怎麼轉，它去呼叫目標型別的 `from_str`，這個方法屬於 `FromStr` trait。每個型別有自己的一套規則：

```rust
fn main() {
    let a: i32 = "42".parse().expect("請輸入整數");
    let b: f64 = "2.5".parse().expect("請輸入小數");
    let c: bool = "true".parse().expect("請輸入 true 或 false");
    let d: char = "茶".parse().expect("請輸入單一字元");
    let e = "30".parse::<u8>().expect("請輸入 0 到 255");
    println!("{a} {b} {c} {d} {e}");
}
```

執行結果：

```text
42 2.5 true 茶 30
```

- `i32` 吃整數，`f64` 吃小數，`bool` 只吃 `true` 和 `false`，`char` 要剛好一個字元。這些型別都實作了 `FromStr`。
- 目標型別有兩種寫法：標在變數上（`let a: i32`），或寫在 `parse` 後面（`parse::<u8>()`，第 8 集的 `::<型別>`）。沒有任何一邊說明的話，編譯器不知道要轉成什麼。

`parse` 的簽名長這樣，現在已經看得懂大半：

```rust,ignore
fn parse<F: FromStr>(&self) -> Result<F, F::Err>
```

`parse` 是泛型方法，目標型別 `F` 的條件是 `F: FromStr`（第 5 集）。回傳的 `Result` 第 9 章才會學，`F::Err` 是 `F` 自己的錯誤型別，第 17 集會解釋。

### 回傳的其實是 `Result`

把 `parse` 的結果直接印出來，就看得到它到底是什麼：

```rust
fn main() {
    println!("{:?}", "42".parse::<i32>());
    println!("{:?}", "abc".parse::<i32>());
    println!("{:?}", "300".parse::<u8>());
    println!("{:?}", "1".parse::<bool>());
}
```

執行結果：

```text
Ok(42)
Err(ParseIntError { kind: InvalidDigit })
Err(ParseIntError { kind: PosOverflow })
Err(ParseBoolError)
```

成功的話是 `Ok(值)`，失敗是 `Err(原因)`；失敗的原因也由目標型別決定：整數是 `ParseIntError`（`InvalidDigit` 是出現了不是數字的字元，`PosOverflow` 是超出 `u8` 的範圍），`bool` 是 `ParseBoolError`。

`Result` 是第 9 章第 2 集的主題。現在知道一件事就夠了：固定句型裡的 `.expect("...")`，就是在處理它。成功時取出裡面的值，失敗時讓程式 panic。第 9 章第 4 集會揭曉 `expect` 的細節。

### 自己的型別沒有 `FromStr`

目標型別沒有實作 `FromStr`，就不能 `parse` 成它：

```rust,compile_fail=E0277
struct Money {
    dollars: u32,
}

fn main() {
    let price: Money = "30".parse().expect("請輸入金額");
    println!("{}", price.dollars);
}
```

編譯錯誤：

```text
error[E0277]: the trait bound `Money: FromStr` is not satisfied
 --> src/main.rs:6:29
  |
6 |     let price: Money = "30".parse().expect("請輸入金額");
  |                             ^^^^^ unsatisfied trait bound
  |
```

`` the trait bound `Money: FromStr` is not satisfied ``：箭頭指著 `parse`，意思是「`Money` 不知道怎麼從文字轉過來」。後面的 `help` 會列出有實作 `FromStr` 的型別，這裡省略。

要讓自己的型別也能 `parse`，就是替它實作 `FromStr`，寫一個 `from_str` 方法。它的回傳型別是 `Result`，要等學完第 9 章才有足夠的工具，所以這一集只看用法，不自己實作。

### 跟 Go 比一比

Go 每種型別用不同的函式：`strconv.Atoi`、`strconv.ParseFloat`、`strconv.ParseBool`。失敗的時候，Go 回傳零值加上一個 `err`：

```go
n, err := strconv.Atoi("abc")
fmt.Println(n, err)
```

這段會印出：

```text
0 strconv.Atoi: parsing "abc": invalid syntax
```

只拿 `n`、不看 `err` 的話，`abc` 就悄悄變成 0。Rust 只有一個 `parse`，用目標型別挑規則；失敗是 `Err`，不是某個預設值，`.expect` 會讓程式 panic，不會默默繼續。

## 重點整理

- `parse` 呼叫目標型別的 `from_str`（`FromStr` trait）；目標型別由變數標註或 `parse::<型別>()` 決定。
- 每個型別有自己的規則：`i32` 吃整數、`bool` 吃 `true` / `false`、`char` 吃單一字元；失敗的原因也不同（`ParseIntError`、`ParseBoolError`）。
- `parse` 回傳 `Result`（`Ok(值)` 或 `Err(原因)`），第 9 章正式介紹；固定句型的 `.expect` 就是在處理它。
- 目標型別沒有實作 `FromStr`，`parse` 會是 E0277。自己實作 `FromStr` 要回傳 `Result`，這一集不寫。
- 跟 Go 比：Go 每種型別一個函式，失敗時回傳零值加 `err`；Rust 一個 `parse`，失敗是 `Err`。
