# 參數用 `&str` 而不是 `&String`

## 本集目標

知道只需要讀字串的函式，參數要寫 `&str`：字面值、`String`、slice 都能傳進去，比 `&String` 通用。

## 正文

### `&String` 收不了字面值

第 2 集把參數寫成 `&String`。這樣的函式，只能接 `String` 的參照，字面值傳不進去：

```rust,compile_fail=E0308
fn shout(s: &String) {
    println!("{s}！");
}

fn main() {
    shout("加油");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:6:11
  |
6 |     shout("加油");
  |     ----- ^^^^^^ expected `&String`, found `&str`
  |     |
  |     arguments to this function are incorrect
  |
  = note: expected reference `&String`
             found reference `&'static str`
note: function defined here
 --> src/main.rs:1:4
  |
1 | fn shout(s: &String) {
  |    ^^^^^ ----------
```

要的是 `&String`，給的是 `&str`。`note` 裡的 `&'static str` 是字面值的完整型別，第 9 集提過，把它當成 `&str` 就好。

### 改成 `&str`

把參數型別改成 `&str`，三種來源都能傳：

```rust
fn shout(s: &str) {
    println!("{s}！");
}

fn main() {
    let owned = String::from("加油");
    shout("加油");
    shout(&owned);
    shout(&owned[0..3]);
}
```

執行結果：

```text
加油！
加油！
加！
```

- `shout("加油")`：字面值本來就是 `&str`。
- `shout(&owned)`：傳的是 `&String`，編譯器在需要 `&str` 的地方會自動轉換。原理第 13 章的 `Deref` 才會解釋，現在照寫就好。
- `shout(&owned[0..3])`：slice 本來就是 `&str`，這裡切出「加」（中文一個字 3 個位元組）。

反過來不行：`&str` 不能變成 `&String`，因為字面值和 slice 後面根本沒有 `String`。所以參數寫 `&str`，能收的比 `&String` 多，函式的內容完全不用改。

### 回傳 slice 的函式也一樣

第 8 集的 `prefix` 也可以改成 `&str`：

```rust
fn prefix(s: &str, n: usize) -> &str {
    &s[0..n]
}

fn main() {
    let owned = String::from("hello world");
    println!("{}", prefix(&owned, 5));
    println!("{}", prefix("hello world", 5));
}
```

執行結果：

```text
hello
hello
```

仍然只有一個參照參數 `s`，所以回傳的 `&str` 借自它，不用額外標註。

### 第 1 章的 `trim` 也是 `&str`

第 1 章讀輸入的固定句型，最後一行是 `let line = line.trim();`。現在可以看懂它了：`trim` 回傳一個 `&str`，是 `line` 那個 `String` 去掉頭尾空白和換行後的一段 slice，沒有複製。新的 `line` 是 `&str`，舊的 `String` 還在，只是名字被 shadowing 蓋住了（第 1 章第 6 集）。

### 怎麼選參數型別

| 函式對字串要做的事 | 參數型別 |
| --- | --- |
| 只要讀 | `&str` |
| 要改動、增長 | `&mut String` |
| 要拿走所有權 | `String` |

只要讀就寫 `&str`，是 Rust 的慣例。後面章節的函式，只需要讀的字串參數都會這樣寫。

## 重點整理

- 只需要讀的字串參數寫 `&str`，不寫 `&String`：字面值、`&String`、字串 slice 都能傳。
- `&String` 會自動轉成 `&str`（原理第 13 章解釋）；`&str` 不能轉成 `&String`。
- 回傳 `&str` 的函式只放一個參照參數，回傳值借自它。
- 第 1 章的 `line.trim()` 回傳的就是借自原 `String` 的 `&str`。
- 選型別：只要讀用 `&str`，要改用 `&mut String`，要拿走用 `String`。
