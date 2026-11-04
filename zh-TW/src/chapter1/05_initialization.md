# 變數必須初始化

## 本集目標

知道 Rust 沒有「零值」：變數在被讀取之前，一定要先有值。

## 正文

### 沒給值就不能用

在 Go 裡，`var count int` 之後，`count` 自動是 `0`。Rust 沒有這回事。變數可以先宣告、不給值，但只要在給值之前讀它，就是編譯錯誤：

```rust,compile_fail=E0381
fn main() {
    let count: i32;
    println!("{count}");
}
```

編譯錯誤：

```text
error[E0381]: used binding `count` isn't initialized
 --> src/main.rs:3:16
  |
2 |     let count: i32;
  |         ----- binding declared here but left uninitialized
3 |     println!("{count}");
  |                ^^^^^ `count` used here but it isn't initialized
  |
help: consider assigning a value
  |
2 |     let count: i32 = 42;
  |                    ++++
```

這裡的 `let count: i32;` 因為沒有值可以推斷型別，所以一定要標 `i32`。

### 想要 0，就自己寫

解法很直接：建立變數時就給值。Go 會替你填的零值，在 Rust 要自己明講：

```rust
fn main() {
    let count = 0;
    let name = "";
    println!("count = {count}, name = [{name}]");
}
```

執行結果：

```text
count = 0, name = []
```

整數想從 0 開始就寫 `0`，空字串寫 `""`，`bool` 就寫 `false`。

先宣告、晚一點才給值，在 Rust 其實也合法，只要編譯器能確定「讀取之前一定給過值」。不過這種寫法很少需要，本書不用它。

### 跟 Go 比一比

Go 的變數永遠有值：

```go
var count int
fmt.Println(count) // 0
```

零值很方便，但也會把「忘了給值」的 bug 藏起來：程式默默拿 0 去算，不會有任何提醒。Rust 選擇讓編譯器當場擋下，變數只要被讀到，就一定是你明確放進去的值。

Rust 連 Go 的 `nil` 也沒有，這件事第 6 章會談。

## 重點整理

- Rust 沒有零值；變數沒給值就讀取，會得到編譯錯誤 `E0381`。
- 沒有初始值可以推斷時，必須標型別，例如 `let count: i32;`。
- 想要 0、空字串、`false`，建立變數時自己寫出來。
- 比起 Go，Rust 的變數被讀到時，一定是你明確給過的值。
