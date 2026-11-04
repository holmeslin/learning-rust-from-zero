# `String` 初探

## 本集目標

用 `String::from` 建立 `String`，用 `push_str` 增加文字、用 `len` 查長度，並分清楚 `String` 和字串字面值。

## 正文

### 建立 `String`

到目前為止，字串都是直接寫在程式裡的 `"哈囉"`，叫做字串字面值。它寫死了，不能增加、不能修改。想要能變長的文字，用 `String`：

```rust
fn main() {
    let greeting = String::from("哈囉");
    println!("{greeting}");
}
```

執行結果：

```text
哈囉
```

`String::from("...")` 把一段文字放進 `String`，請先照抄，它背後的寫法第 5 章才會解釋。第 1 章讀輸入時看過的 `String::new()`，建立的就是空的 `String`。

### `push_str` 與 `len`

`push_str` 把文字接在 `String` 的後面，`len` 回傳長度。這種「變數名稱、點、名字、小括號」的寫法叫呼叫方法，先照抄，第 5 章會解釋：

```rust
fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{s}");
    println!("{}", s.len());
}
```

執行結果：

```text
Hello, world
12
```

`push_str` 會直接改動 `s`，所以 `s` 要用 `let mut` 建立。少了 `mut` 就會被擋下來：

```rust,compile_fail=E0596
fn main() {
    let s = String::from("Hello");
    s.push_str(", world");
    println!("{s}");
}
```

編譯錯誤：

```text
error[E0596]: cannot borrow `s` as mutable, as it is not declared as mutable
 --> src/main.rs:3:5
  |
3 |     s.push_str(", world");
  |     ^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut s = String::from("Hello");
  |         +++
```

訊息裡的 `borrow`（借用）第 4 章才會講，現在只要照 `help` 的建議加上 `mut` 就好。

`len` 數的是**位元組**（byte），不是字元。中文字在 `String` 裡佔 3 個位元組：

```rust
fn main() {
    let s = String::from("哈囉");
    println!("{}", s.len());
}
```

執行結果：

```text
6
```

「哈囉」兩個字，`len` 是 6。怎麼數字元，第 7 章會學到。

陣列也有 `len`，寫法一樣，回傳的是元素的個數：

```rust
fn main() {
    let scores = [85, 92, 78];
    println!("{}", scores.len());
}
```

執行結果：

```text
3
```

### 字串字面值不是 `String`

`"小明"` 和 `String::from("小明")` 是兩個不同的型別，不能互相取代：

```rust,compile_fail=E0308
fn main() {
    let name: String = "小明";
    println!("{name}");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:2:24
  |
2 |     let name: String = "小明";
  |               ------   ^^^^^^ expected `String`, found `&str`
  |               |
  |               expected due to this
  |
help: try using a conversion method
  |
2 |     let name: String = "小明".to_string();
  |                              ++++++++++++
```

訊息說：要的是 `String`，給的是 `&str`。字串字面值的型別就是 `&str`，第 4 章才會介紹，現在只要記住：要 `String`，就用 `String::from`。`help` 建議的 `.to_string()` 是另一個做法，第 8 章才會學到，先不用管。

這一集的 `String` 都只在 `main` 裡用。把 `String` 交給函式、或指定給另一個變數，會有新的規則，第 3 章會用 `String` 來講所有權。

### 跟 Go 比一比

Go 的字串不能改，`s += ", world"` 其實是做出一個新字串，再讓 `s` 指向它。Rust 的 `String` 可以原地增長：`push_str` 直接改動原本那個，所以才需要 `mut`。另一個差別是 Rust 有兩種字串型別：寫死的字面值，以及可以增長的 `String`。Go 只有一種 `string`，不用分。`len` 的結果則兩邊一樣：Go 的 `len(s)` 也是位元組數。

## 重點整理

- `String` 是可以增長的文字，用 `String::from("...")` 建立；`String::new()` 是空的。
- `s.push_str("...")` 把文字接到後面，`s` 要用 `let mut`；`s.len()` 回傳位元組數，不是字元數；陣列的 `len` 是元素個數。方法呼叫的寫法先照抄，第 5 章解釋。
- 字串字面值（型別 `&str`，第 4 章介紹）不是 `String`；要 `String` 就寫 `String::from`。
- 這一集只在 `main` 裡使用 `String`；傳給函式、指定給別的變數的規則，第 3 章介紹。
- Go 的字串不可變，Rust 的 `String` 可以原地改；兩邊的 `len` 都是位元組數。
