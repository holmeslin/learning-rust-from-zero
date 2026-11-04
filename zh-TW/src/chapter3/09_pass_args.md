# 傳參數也會 move

## 本集目標

知道把值傳給函式的參數，規則跟 `let b = a;` 一樣：`String` 會 move 進函式，`Copy` 型別只會複製。

## 正文

### 把 `String` 傳進函式

呼叫函式時，傳進去的值會指定給參數。這跟 `let` 指定是同一件事，所以 `String` 會 move：

```rust
fn consume(s: String) {
    println!("收到 {s}");
}

fn main() {
    let msg = String::from("哈囉");
    consume(msg);
    println!("main 結束");
}
```

執行結果：

```text
收到 哈囉
main 結束
```

`consume(msg)` 那一刻，`String` 的所有權交給參數 `s`。`consume` 的 `}` 一到，`s` 離開作用域，值就被 drop（第 4 集）。所以函式回來之後，`msg` 指的那份資料已經不在了，不能再用：

```rust,compile_fail=E0382
fn consume(s: String) {
    println!("收到 {s}");
}

fn main() {
    let msg = String::from("哈囉");
    consume(msg);
    println!("{msg}");
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `msg`
 --> src/main.rs:8:16
  |
6 |     let msg = String::from("哈囉");
  |         --- move occurs because `msg` has type `String`, which does not implement the `Copy` trait
7 |     consume(msg);
  |             --- value moved here
8 |     println!("{msg}");
  |                ^^^ value borrowed here after move
  |
note: consider changing this parameter type in function `consume` to borrow instead if owning the value isn't necessary
 --> src/main.rs:1:15
  |
1 | fn consume(s: String) {
  |    -------    ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
help: consider cloning the value if the performance cost is acceptable
  |
7 |     consume(msg.clone());
  |                ++++++++
```

`value moved here` 指向 `consume(msg)`。訊息後面的 `note` 建議改成「借用」，那是第 4 章的內容，現在先不用管。

### `Copy` 型別照常複製

整數傳進函式，一樣是複製，呼叫的地方還能繼續用：

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let n = 5;
    let d = double(n);
    println!("{n} {d}");
}
```

執行結果：

```text
5 10
```

### 想保留，就 `clone`

如果呼叫之後還要用，可以傳一份複本進去：

```rust
fn consume(s: String) {
    println!("收到 {s}");
}

fn main() {
    let msg = String::from("哈囉");
    consume(msg.clone());
    println!("main 還有 {msg}");
}
```

執行結果：

```text
收到 哈囉
main 還有 哈囉
```

這樣能編譯，但每呼叫一次就複製一份，只是為了讓函式「看一眼」，有點浪費。接下來兩集先換個做法，第 4 章再給出更好的辦法。

### 跟 Go 比一比

Go 把字串或 slice 傳給函式，呼叫端照樣能用：

```go
func consume(s string) {
    fmt.Println("收到", s)
}

msg := "哈囉"
consume(msg)
fmt.Println(msg) // 照常能用
```

Rust 的 `String` 傳進函式，所有權就跟著過去了，呼叫端不能再碰。從 Go 轉過來，寫函式時要先想一件事：這個函式是要「拿走」這份資料，還是只是要看一看？

## 重點整理

- 傳參數就是指定給參數：`String` 會 move 進函式，函式結束時被 drop，呼叫端不能再用（E0382）。
- `Copy` 型別（整數等）傳進函式只複製，呼叫端照用。
- 想在呼叫後繼續用，可以傳 `msg.clone()`，但要付複製的成本。
- Go 傳參數後原變數照用；Rust 的 `String` 傳出去就沒了。
