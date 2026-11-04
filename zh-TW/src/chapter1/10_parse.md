# 文字轉數字

## 本集目標

把讀進來的文字轉成整數，這樣才能拿來計算。

## 正文

### 文字轉數字的固定句型

上一集說過，讀進來的永遠是文字。要把它轉成整數，在 `trim` 之後再多寫一行。**現在也先照抄**：

```rust,stdin=18,noplayground
fn main() {
    println!("請輸入年齡：");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    let line = line.trim();
    let age: i32 = line.parse().expect("請輸入整數");
    println!("明年你就 {} 歲了", age + 1);
}
```

輸入 `18` 的執行結果：

```text
請輸入年齡：
明年你就 19 歲了
```

新增的只有這一行：

```rust,ignore
let age: i32 = line.parse().expect("請輸入整數");
```

- `line.parse()` 嘗試把文字轉成數字。
- `expect("請輸入整數")` 的意思是「轉換失敗就印出這句話，然後讓程式停下來」。它的原理第 9 章解釋。
- 轉成什麼型別，由變數的型別標註決定。這裡寫 `i32`，就是轉成 `i32`；改成 `i64`，就轉成 `i64`。

變數名稱和提示文字都可以換，但整行的結構請保持不變。

### 型別標註不能省

`parse` 可以轉成很多種型別，如果不告訴它要轉成哪一種，編譯器就沒辦法決定：

```rust,compile_fail=E0284,stdin=5,noplayground
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    let line = line.trim();
    let n = line.parse().expect("請輸入整數");
    println!("{n}");
}
```

編譯錯誤：

```text
error[E0284]: type annotations needed
 --> src/main.rs:5:9
  |
5 |     let n = line.parse().expect("請輸入整數");
  |         ^        ----- type must be known at this point
```

這裡只列出前幾行，後面還有幾行補充說明，最後的 `help` 會建議你加上型別。照做就是：`let n: i32 = ...`。

### 輸入不是整數會怎樣

`trim` 去掉前後的空白，所以 `42` 前後有空格或換行都沒關係。負號可以，像 `-12` 就能轉成功。但如果輸入的根本不是整數，例如 `abc` 或 `3.5`，轉換就會失敗，程式會 **panic**（當掉並結束）：

```rust,stdin=abc,exit=101,noplayground
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    let line = line.trim();
    let n: i32 = line.parse().expect("請輸入整數");
    println!("{n}");
}
```

輸入 `abc` 的話，程式不會印出算出來的數字，而是印出錯誤訊息。

執行結果（某一次）：

```text
thread 'main' (155566833) panicked at src/main.rs:5:31:
請輸入整數: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

第一行 `thread 'main'` 後面括號裡的數字每次執行都不一樣，其他部分都一樣。訊息裡有你寫在 `expect` 裡的那句話，還有轉換失敗的原因。panic 的程式結束碼是 101。這個階段，輸入不合法就讓程式停下來是可以的，不用處理它；怎麼好好處理錯誤，第 9 章才會教。

### 跟 Go 比一比

Go 的 `strconv.Atoi` 會回傳數字和 `error` 兩個值：

```go
n, err := strconv.Atoi(line)
if err != nil {
    panic(err)
}
```

Rust 的 `parse` 把成功與失敗包在同一個值裡，`.expect(...)` 的作用就像上面這個 `if err != nil { panic(err) }`：失敗就 panic。差別是 Go 可以用 `n, _ := strconv.Atoi(line)` 把錯誤默默丟掉，拿到一個 `0`；Rust 要拿到數字，就一定得選一種方式處理失敗的情況（`expect` 就是其中一種），沒辦法假裝沒看到。另外，Go 的 panic 結束碼是 2，Rust 是 101。

## 重點整理

- 把文字轉成整數的固定句型：`let n: i32 = line.parse().expect("請輸入整數");`，先照抄，原理之後解釋。
- 型別標註決定要轉成什麼型別，不能省略。
- 輸入不是整數時，程式會 panic，結束碼是 101；這個階段不用處理它。
- `trim` 之後才 `parse`，前後的空白和換行才不會讓轉換失敗。
