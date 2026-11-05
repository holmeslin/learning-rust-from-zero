# `match` 必須窮盡

## 本集目標

知道 `match` 少列任何一種可能，程式就編譯不過（E0004），以及這個規定在新增 variant 時幫了什麼忙。

## 正文

### 漏掉一種可能

上一集的 `describe` 把三個 variant 都處理了。假設過了一陣子，`Status` 多了一個 `Cancelled`，`describe` 卻沒有跟著改：

```rust,compile_fail=E0004
enum Status {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

fn describe(status: Status) {
    match status {
        Status::Pending => println!("等待付款"),
        Status::Paid => println!("已付款，準備出貨"),
        Status::Shipped => println!("已出貨"),
    }
}

fn main() {
    describe(Status::Cancelled);
}
```

編譯錯誤：

```text
error[E0004]: non-exhaustive patterns: `Status::Cancelled` not covered
  --> src/main.rs:9:11
   |
 9 |     match status {
   |           ^^^^^^ pattern `Status::Cancelled` not covered
   |
note: `Status` defined here
  --> src/main.rs:1:6
   |
 1 | enum Status {
   |      ^^^^^^
...
 5 |     Cancelled,
   |     --------- not covered
   = note: the matched value is of type `Status`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
12 ~         Status::Shipped => println!("已出貨"),
13 ~         Status::Cancelled => todo!(),
   |
```

訊息裡有三個重點：

- ``non-exhaustive patterns: `Status::Cancelled` not covered``：這個 `match` 沒有**窮盡**（exhaustive），`Cancelled` 沒有分支接得住。
- `note` 指向 enum 的定義，用 `not covered` 標出漏掉的 variant。
- `help` 建議補上 `Status::Cancelled => todo!()`。`todo!()` 是「這裡還沒寫」的佔位，執行到會 panic，先不用管，直接寫出真正的分支就好。

### 窮盡的意思

**窮盡**是指：這個型別每一個可能的值，都至少會被一個分支接住。編譯器會逐一檢查，檢查不過就不編譯，所以不會有「執行到才發現某個 variant 沒人處理」這種事。補上漏掉的分支：

```rust
enum Status {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

fn describe(status: Status) {
    match status {
        Status::Pending => println!("等待付款"),
        Status::Paid => println!("已付款，準備出貨"),
        Status::Shipped => println!("已出貨"),
        Status::Cancelled => println!("已取消"),
    }
}

fn main() {
    describe(Status::Pending);
    describe(Status::Paid);
    describe(Status::Shipped);
    describe(Status::Cancelled);
}
```

執行結果：

```text
等待付款
已付款，準備出貨
已出貨
已取消
```

variant 帶資料時，模式要把資料也涵蓋：`Discount::Percent(p)` 接得住任何 `u32`，所以一個分支就夠。

### 新增 variant 時，編譯器幫你找

窮盡真正有用的時機，是改程式的時候。程式裡有十個地方對 `Status` 做 `match`，新增 `Cancelled` 之後，這十個地方全部變成編譯錯誤，每一個都標出行號。把錯誤清完，就確定沒有任何地方漏掉新的狀態。

### 跟 Go 比一比

Go 的 `switch` 不檢查有沒有列完：

```go
func describe(s Status) {
    switch s {
    case Pending:
        fmt.Println("等待付款")
    case Paid:
        fmt.Println("已付款，準備出貨")
    case Shipped:
        fmt.Println("已出貨")
    }
}
```

之後新增一個 `Cancelled` 常數，這段照樣編譯、照樣執行，只是傳入 `Cancelled` 時什麼都不會印。要靠工具發現漏掉的 case，得另外裝 linter（例如 `exhaustive`），而且 `default` 一加，連 linter 也不會提醒你。

Rust 把這個檢查放進編譯器，而且不能關掉。想讓某些情況「其他全部一起處理」，要明講，下一集的 `_` 就是這個用途；不過它也會讓這個檢查失效：之後新增的 variant 會悄悄掉進 `_`。所以自己定義的 enum，建議把每個 variant 都寫出來，不要用 `_` 偷懶。

## 重點整理

- `match` 必須窮盡：型別每一個可能的值，都要有分支接住；少一個就是 E0004，訊息會用 `not covered` 指出漏掉哪個。
- `help` 建議的 `todo!()` 是暫時佔位，執行到會 panic；正確做法是寫出真正的分支。
- 新增 variant 之後，所有沒處理它的 `match` 都會編譯失敗，不會漏改。
- 跟 Go 比：Go 的 `switch` 不檢查有沒有列完，新增常數後舊的 `switch` 照常編譯。
- `_` 可以接住「其他全部」，下一集教；但它會讓新增的 variant 悄悄掉進去，自己的 enum 建議逐一寫出每個 variant。
