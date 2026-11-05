# `panic!`

## 本集目標

用 `panic!` 讓程式在「沒辦法再做下去」的時候當場停下來，並知道 panic 會印出什麼、結束碼是多少。

## 正文

### 你早就遇過 panic

前面的章節已經碰過好幾次：陣列索引越界（第 2 章第 12 集）、整數溢位（第 1 章第 16 集）、`Vec` 的索引越界（第 7 章第 3 集）、`parse` 遇到不是數字的輸入。這些都是 panic：程式發現再做下去不對了，印出訊息就結束。標準函式庫在這些地方呼叫的，就是你也能自己呼叫的 `panic!`。

### 自己呼叫 `panic!`

```rust,exit=101
fn split_bill(total: u32, people: u32) -> u32 {
    if people == 0 {
        panic!("人數不能是 0");
    }
    total / people
}

fn main() {
    println!("每人 {} 元", split_bill(120, 4));
    println!("每人 {} 元", split_bill(120, 0));
}
```

執行結果（某一次）：

```text
每人 30 元

thread 'main' (158077355) panicked at src/main.rs:3:9:
人數不能是 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- `panic!("...")` 括號裡的格式字串跟 `println!` 一樣，`panic!("沒有 {code} 號商品")` 也行。
- 第一次呼叫正常印出，第二次 panic。訊息送到 stderr，所以出現在 stdout 的輸出之後。
- 訊息第一行 `thread 'main' (數字) panicked at src/main.rs:3:9:`：括號裡的數字每次執行都不同，`3:9` 是 `panic!` 所在的行與欄。第二行是你寫的訊息。
- panic 之後，`main` 後面的程式都不會執行，結束碼是 101，不是 0。
- 其實不加判斷，`total / 0` 也會 panic（訊息是 `attempt to divide by zero`）。自己 `panic!` 只是讓訊息更清楚。

### panic 的型別是 `!`

`panic!` 之後不會有任何值回來，所以編譯器讓它出現在任何型別的位置，不必湊一個值：

```rust,exit=101
fn price_of(code: u32) -> u32 {
    if code == 1 {
        30
    } else if code == 2 {
        45
    } else {
        panic!("沒有 {code} 號商品");
    }
}

fn main() {
    println!("{}", price_of(1));
    println!("{}", price_of(2));
    println!("{}", price_of(9));
}
```

執行結果（某一次）：

```text
30
45

thread 'main' (158077422) panicked at src/main.rs:7:9:
沒有 9 號商品
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`price_of` 要回傳 `u32`，前兩個分支給 `30`、`45`，最後一個分支卻是 `panic!`，沒有 `u32`，也能編譯成功。這個「永遠不會有值」的型別寫作 `!`（never），第 4 集手寫 `expect` 就會用到這個特性。

### panic 時發生什麼

Rust 預設會沿著呼叫的順序往回走，把還活著的值一個個 drop（第 3 章第 4 集），再結束程式。想知道是從哪一連串呼叫走到這裡，執行時加上環境變數：

```bash
RUST_BACKTRACE=1 cargo run
```

訊息最後那行 `note` 說的就是這件事。`todo!()`（還沒寫）、`unreachable!()`（不可能走到）是行為一樣、訊息不同的 panic。

`panic!` 表示「程式有 bug，繼續下去只會更糟」。如果失敗是預料得到的，例如使用者輸入錯了，就不該 panic，而是讓呼叫的人決定怎麼辦。這是下一集 `Result` 的工作。

### 跟 Go 比一比

Go 也有 `panic`，連用法都很像：

```go
func splitBill(total, people int) int {
	if people == 0 {
		panic("人數不能是 0")
	}
	return total / people
}
```

差別有兩點：

- 結束碼：Go 的 panic 結束碼是 2，Rust 是 101。寫腳本檢查結束碼的時候要記得。
- Go 可以在 `defer` 裡用 `recover()` 把 panic 接住，有些程式庫會這樣用。Rust 標準函式庫雖然有 `std::panic::catch_unwind`，但它不是一般的錯誤處理工具，沒有人會拿它來取代 `if err != nil`。

## 重點整理

- `panic!("訊息")` 讓程式印出訊息後立刻結束，格式字串跟 `println!` 一樣；結束碼是 101。
- 訊息送到 stderr，開頭是 `thread 'main' (數字) panicked at 檔案:行:欄:`，數字每次不同。
- `panic!` 的型別是 `!`，不會有值回來，所以能放在任何型別的位置（例如 `else` 分支）。
- panic 會沿著呼叫往回走、drop 沿途的值；`RUST_BACKTRACE=1` 可以看呼叫路徑。
- `panic!` 用在「程式有 bug、無法繼續」；預料得到的失敗改用 `Result`（下一集）。
- 跟 Go 比：Go 的 panic 結束碼是 2、可以 `recover`；Rust 是 101，正常程式不接 panic。
