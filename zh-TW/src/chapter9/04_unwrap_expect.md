# `unwrap` 與 `expect`（揭曉固定句型）

## 本集目標

知道 `.expect("...")` 和 `.unwrap()` 就是「成功就取值、失敗就 panic」的 `match` 簡寫，並分得出什麼時候可以用。

## 正文

### 第 1 章的固定句型

第 1 章教你照抄 `parse` 加 `.expect("請輸入整數")`，當時說 `expect` 第 9 章才解釋。現在你會 `match` 了，先把它的作用手寫一遍：

```rust,exit=101
fn parse_qty(text: &str) -> u32 {
    match text.parse::<u32>() {
        Ok(n) => n,
        Err(e) => panic!("請輸入整數: {e:?}"),
    }
}

fn main() {
    println!("{}", parse_qty("30"));
    println!("{}", parse_qty("abc"));
}
```

執行結果（某一次）：

```text
30

thread 'main' (158084949) panicked at src/main.rs:4:19:
請輸入整數: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`Ok(n)` 取出值，`Err(e)` 就 `panic!`。`Err` 分支的型別是 `!`（第 1 集），可以搭配給 `u32` 的 `Ok` 分支。`.expect("訊息")` 做的就是這件事，少寫了整個 `match`：

```rust,exit=101
fn main() {
    let n: u32 = "abc".parse().expect("請輸入整數");
    println!("{n}");
}
```

執行結果（某一次）：

```text
thread 'main' (158085010) panicked at src/main.rs:2:32:
請輸入整數: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

訊息是你給的文字、冒號，加上 `Err` 裡的原因（用 `{:?}` 印出），跟上面手寫的一模一樣；只有位置不同，因為 `panic!` 寫在不同的地方。

### `unwrap`：不寫訊息

`.unwrap()` 是不帶訊息的 `expect`：

```rust,exit=101
fn main() {
    let n: u32 = "abc".parse().unwrap();
    println!("{n}");
}
```

執行結果（某一次）：

```text
thread 'main' (158085091) panicked at src/main.rs:2:32:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

訊息是固定的 `` called `Result::unwrap()` on an `Err` value ``，後面接 `Err` 裡的原因。出錯時只看得到這句，不知道你原本期望什麼，所以 `expect` 比較好找問題。

### `Option` 也能用

第 6 章第 7 集預告過：`Option` 的 `.unwrap()` 和 `.expect("...")`，就是「`Some` 取值、`None` 就 panic」。

```rust,exit=101
use std::collections::HashMap;

fn main() {
    let mut prices: HashMap<String, u32> = HashMap::new();
    prices.insert(String::from("紅茶"), 30);
    let tea = prices.get("紅茶").expect("菜單一定有紅茶");
    println!("紅茶 {tea} 元");
    let milk = prices.get("奶茶").unwrap();
    println!("奶茶 {milk} 元");
}
```

執行結果（某一次）：

```text
紅茶 30 元

thread 'main' (158085173) panicked at src/main.rs:8:35:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

對 `None`，`unwrap` 的訊息是 `` called `Option::unwrap()` on a `None` value ``；`expect` 則只印出你寫的文字，不會再多加什麼。

### 什麼時候可以用

`expect` 和 `unwrap` 不是壞東西，但它們把「失敗」變成「整支程式結束」。可以用在：

- 範例、小腳本、還在試的原型。
- 你有把握不會失敗：值是寫死的，或前一行剛檢查過。這時 `expect` 的訊息寫「為什麼我認為不會失敗」，例如 `expect("菜單一定有紅茶")`，不是寫給使用者看的提示。第 1 章的 `expect("請輸入整數")` 只是為了讓剛開始學的人讀得懂 panic 訊息，這個原則才是平常的寫法。
- 測試程式（第 12 章）。

使用者輸入、外部資料這類「本來就可能出錯」的地方，用上一集的 `match`，或下一集的 `?`。`unwrap_or` 之類一整排處理 `Option` 與 `Result` 的方法，第 11 章才教。

### 揭曉：`read_line` 也回傳 `Result`

第 1 章第 9 集的 `read_line(&mut line).expect("讀取失敗")`，`expect` 處理的也是 `Result`：

```rust,stdin=abc,noplayground
fn main() {
    let mut line = String::new();
    let result = std::io::stdin().read_line(&mut line);
    println!("{result:?}");
    println!("{line:?}");
}
```

輸入 `abc` 的執行結果：

```text
Ok(4)
"abc\n"
```

`read_line` 回傳 `Result<usize, std::io::Error>`：成功時是讀了幾個位元組，連換行字元也算（`abc` 加換行是 4）；失敗時是 `std::io::Error`，例如輸入串流壞了。`line` 的內容是 `"abc\n"`，所以第 1 章要用 `trim()` 去掉換行。

## 重點整理

- `.expect("訊息")`：`Ok` 取出值，`Err` 就 panic，訊息是你的文字、冒號和 `Err` 裡的原因；等於手寫 `match` 加 `panic!`。
- `.unwrap()` 是不帶訊息的版本，訊息固定是 `called Result::unwrap() on an Err value: ...`。
- `Option` 也有：`Some` 取值，`None` 就 panic；`unwrap` 的訊息是 `called Option::unwrap() on a None value`，`expect` 只印你的文字。
- 適合範例、原型、寫死或剛檢查過的值、測試；`expect` 的訊息寫「為什麼我認為不會失敗」。
- 使用者輸入與外部資料這類本來就可能出錯的地方，用 `match` 或 `?`。
- `read_line` 回傳 `Result<usize, std::io::Error>`，第 1 章的 `.expect("讀取失敗")` 處理的就是它。
