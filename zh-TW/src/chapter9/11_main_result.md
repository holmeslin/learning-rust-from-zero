# `main` 回傳 `Result`

## 本集目標

讓 `main` 回傳 `Result`，這樣 `main` 裡也能用 `?`；並知道出錯時程式會印什麼、結束碼是多少。

## 正文

### `main` 也能回傳 `Result`

第 5 集的 E0277 說過，`main` 回傳 `()`，所以不能用 `?`。其實 `main` 的回傳型別可以改成 `Result<(), E>`：

```rust,stdin=abc,noplayground,exit=1
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    let qty: u32 = line.trim().parse()?;
    println!("共 {} 元", qty * 30);
    Ok(())
}
```

輸入 `abc` 的執行結果：

```text
Error: ParseIntError { kind: InvalidDigit }
```

- `-> Result<(), Box<dyn std::error::Error>>`：成功時沒有值要回傳，所以是 `()`（第 2 章第 7 集）；失敗時是上一集的 `Box<dyn Error>`，什麼錯誤都收。
- `read_line(&mut line)?` 處理的是第 4 集的 `Result<usize, std::io::Error>`，`parse()?` 失敗時是 `ParseIntError`，兩種錯誤型別不同，上一集的 `Box<dyn Error>` 正好都收。第 1 章的 `.expect("讀取失敗")`，現在可以換成 `?`。
- 最後一行 `Ok(())`：沒有出錯的話，`main` 要自己回傳 `Ok`，括號裡的 `()` 是成功的值。輸入 `3` 就會走到這裡，印出 `共 90 元`，結束碼 0。

### 出錯時印什麼

`main` 收到 `Err` 時，程式會把錯誤印到 stderr，格式是 `Error: ` 加上 `{:?}`（`Debug`）的結果，然後以結束碼 1 結束，不是 panic 的 101。所以印出來的是 `ParseIntError { kind: InvalidDigit }`，不是 `invalid digit found in string`。

錯誤型別不一定要 `Box<dyn Error>`，只要實作 `Debug` 就行：

```rust,exit=1
fn main() -> Result<(), String> {
    println!("開始結帳");
    Err(String::from("庫存不足"))
}
```

執行結果：

```text
開始結帳
Error: "庫存不足"
```

`String` 的 `Debug` 格式會帶雙引號，所以訊息是 `Error: "庫存不足"`。程式先印 `開始結帳`，`main` 回傳 `Err` 才結束。這跟 panic 不同：`main` 是正常走到最後一行並回傳，只是回傳的值是 `Err`。

### 想印給使用者看，自己處理

`Error: ` 加 `Debug` 的格式是寫給開發者看的。想讓使用者看到 `{e}` 的說明文字，就像上一集那樣，在 `main` 裡自己 `match` 或 `if let Err(e)`，再 `println!("出錯了：{e}")`。想要別的結束碼，第 18 章第 4 集的 `ExitCode` 會教。

### 跟 Go 比一比

Go 的 `main` 沒有回傳值，出錯要自己決定怎麼結束：

```go
func main() {
	if err := run(); err != nil {
		log.Fatal(err)
	}
}
```

`log.Fatal` 印出訊息（預設帶時間）並以結束碼 1 結束。Rust 的 `main` 回傳 `Err` 就是同樣的效果，結束碼也是 1，不用自己寫 `log.Fatal`。差別在訊息：Go 印的是 `err.Error()` 的文字，Rust 印的是 `Debug` 格式，自己的錯誤 enum 會直接印出 `BadQty(ParseIntError { kind: InvalidDigit })` 這種寫給開發者看的樣子。

## 重點整理

- `main` 可以寫成 `fn main() -> Result<(), E>`：成功要自己回傳 `Ok(())`，這樣 `main` 裡就能用 `?`。
- `E` 常寫 `Box<dyn std::error::Error>`，可以收下各種錯誤；`E` 只要實作了 `Debug`。
- `main` 回傳 `Err` 時，stderr 印 `Error: ` 加 `{:?}` 的結果，結束碼是 1；panic 的結束碼是 101。
- 想印給使用者看的訊息（`Display`），在 `main` 裡自己處理錯誤；想要別的結束碼，第 18 章第 4 集的 `ExitCode`。
- 跟 Go 比：Go 的 `main` 沒有回傳值，要自己 `log.Fatal`；Rust 回傳 `Err` 就會以結束碼 1 結束，但印的是 `Debug` 格式。
