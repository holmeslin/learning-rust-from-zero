# 用 tuple 回傳多個值

## 本集目標

讓函式用 tuple 一次回傳好幾個值，並看出它跟 Go 的多重回傳值有什麼不同。

## 正文

### 回傳 tuple

函式只能回傳一個值，但這個值可以是 tuple。回傳型別寫成 tuple 的型別，最後一個運算式也寫成 tuple：

```rust
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn main() {
    let result = divide(17, 5);
    println!("商 {}，餘 {}", result.0, result.1);
}
```

執行結果：

```text
商 3，餘 2
```

`divide` 回傳 `(i32, i32)`，第一個是商，第二個是餘數。呼叫的地方先把整個 tuple 放進 `result`，再用 `.0`、`.1` 取出來。下一集的「解構」會讓這一步更順手。

回傳的 tuple 就是普通的值，可以放進變數，也可以原封不動交給別的函式：

```rust
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn show(pair: (i32, i32)) {
    println!("商 {}，餘 {}", pair.0, pair.1);
}

fn main() {
    show(divide(17, 5));
    show(divide(100, 7));
}
```

執行結果：

```text
商 3，餘 2
商 14，餘 2
```

### 跟 Go 比一比

Go 的函式可以回傳多個值，寫法看起來很像：

```go
func divide(a, b int) (int, int) {
    return a / b, a % b
}

q, r := divide(17, 5)
```

但兩邊的做法其實不同，有兩個地方要注意。

第一，Go 回傳多個值，是語言本身的機制，並不是一個值。`result := divide(17, 5)` 在 Go 會編譯失敗（一個變數接不了兩個值），一定要同時寫兩個變數。Rust 回傳的是**一個** tuple 值，所以可以放進一個變數、當成參數傳來傳去。

第二，Go 習慣用 `(結果, error)` 回傳「成功或失敗」。Rust 不這樣做：函式可能失敗時，不是回傳一個 tuple 來夾帶錯誤，而是用專門的型別表示，第 6 章的 `Option` 與第 9 章的 `Result` 會介紹。tuple 只用在「這幾個值本來就是一組」的時候，例如商和餘數、座標的 x 和 y。

## 重點整理

- 回傳多個值，就回傳一個 tuple：`-> (i32, i32)`，最後一個運算式寫 `(a / b, a % b)`。
- 呼叫端用 `.0`、`.1` 取值；下一集學更順手的解構寫法。
- 回傳的 tuple 是一個值，可以放進變數或傳給別的函式。
- Go 的多重回傳值不是一個值；Rust 的 tuple 是。
- Go 的 `(結果, error)` 在 Rust 要用 `Option`、`Result`（第 6、9 章），不要用 tuple。
