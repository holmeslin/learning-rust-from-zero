# 可變參照 `&mut`

## 本集目標

用 `&mut` 借出可以修改的參照，並知道 `&` 只能讀、`&mut` 才能改。

## 正文

### 借來改

`&` 借來的東西只能讀。要讓函式改動，就借出**可變參照**（mutable reference），寫成 `&mut`：

```rust
fn add_world(s: &mut String) {
    s.push_str("，世界");
}

fn main() {
    let mut msg = String::from("哈囉");
    add_world(&mut msg);
    println!("{msg}");
}
```

執行結果：

```text
哈囉，世界
```

有三個地方都要寫對：

- `let mut msg`：要被改的變數本身得是 `mut`。
- `s: &mut String`：參數型別是「指向 `String` 的可變參照」。
- `add_world(&mut msg)`：呼叫時明講「我借出可變的」。

第 3 章第 10 集為了改 `String`，得寫成 `let msg = cheer(msg);`，一路 move 進去、再 move 出來。`&mut` 不用，函式直接改原本那一份，也沒有回傳值。

### 第 1 章的 `&mut` 就是這個

第 1 章讀輸入的那三行裡，有一句 `read_line(&mut line)`。現在看得懂了：`read_line` 要把讀到的文字放進 `line`，所以我們借出 `line` 的可變參照給它；`line` 一開始就是 `let mut line = String::new();`，因為它要被改。

### `&` 不能改

用 `&String` 去改，編譯器會擋下來：

```rust,compile_fail=E0596
fn add_world(s: &String) {
    s.push_str("，世界");
}

fn main() {
    let msg = String::from("哈囉");
    add_world(&msg);
    println!("{msg}");
}
```

編譯錯誤：

```text
error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
 --> src/main.rs:2:5
  |
2 |     s.push_str("，世界");
  |     ^ `s` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
1 | fn add_world(s: &mut String) {
  |                  +++
```

訊息說 `*s` 在 `&` 參照後面，不能當成可變的來借。`*s` 是「`s` 指向的那個 `String`」，`*` 第 7 集才會講，現在只要看 `help`：把 `&String` 改成 `&mut String`。

### 變數本身要是 `mut`

另一個常見的錯，是借出 `&mut`，變數卻不是 `mut`：

```rust,compile_fail=E0596
fn add_world(s: &mut String) {
    s.push_str("，世界");
}

fn main() {
    let msg = String::from("哈囉");
    add_world(&mut msg);
    println!("{msg}");
}
```

編譯錯誤：

```text
error[E0596]: cannot borrow `msg` as mutable, as it is not declared as mutable
 --> src/main.rs:7:15
  |
7 |     add_world(&mut msg);
  |               ^^^^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
6 |     let mut msg = String::from("哈囉");
  |         +++
```

這跟第 2 章第 15 集忘了寫 `mut` 的 E0596 是同一個錯。`push_str` 要改動 `String`，就得有它的可變參照；而 `msg` 一開始就宣告成不可變，當然借不出可變的。

### 兩種借法

| 寫法 | 型別 | 能做什麼 |
| --- | --- | --- |
| `&s` | `&String` | 只能讀 |
| `&mut s` | `&mut String` | 可以讀，也可以改；`s` 必須是 `let mut` |

### 跟 Go 比一比

Go 的指標讀寫都行，簽名和呼叫都只有一個 `&`：

```go
func addWorld(s *string) {
    *s += "，世界"
}

msg := "哈囉"
addWorld(&msg)
```

光看 `addWorld(&msg)`，看不出 `msg` 會不會被改。Rust 把「只能讀」和「可以改」分成 `&` 與 `&mut`，函式簽名和呼叫端都要寫明。讀 `add_world(&mut msg)` 就知道：這一行之後，`msg` 可能變了。

## 重點整理

- `&mut` 借出可變參照；函式簽名寫 `&mut String`，呼叫時寫 `&mut msg`，變數要用 `let mut`。
- 用 `&mut` 改，不必靠回傳值交還所有權；第 1 章的 `read_line(&mut line)` 就是這個用法。
- 用 `&String` 去改，是 E0596（`behind a & reference`），改成 `&mut String`。
- 變數不是 `mut` 卻借出 `&mut`，也是 E0596（`not declared as mutable`），加上 `mut`。
- Go 的指標讀寫都行；Rust 用 `&` 與 `&mut` 區分只能讀與可以改，呼叫端也要寫明。
