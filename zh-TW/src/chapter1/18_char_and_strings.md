# `char`、跳脫字元與 raw string

## 本集目標

認識 `char` 型別、字串裡的跳脫字元，以及不需要跳脫的 raw string。

## 正文

### `char` 是單一個字元

`char` 用**單引號**，裝的是一個 Unicode 字元，中文字和 emoji 都可以：

```rust
fn main() {
    let letter = 'A';
    let han = '中';
    let smile = '😀';
    println!("{letter} {han} {smile}");
}
```

執行結果：

```text
A 中 😀
```

`'A'` 和 `"A"` 是不同的東西：單引號是 `char`，雙引號是字串。兩個不能直接比較。每個 `char` 固定佔 4 個位元組（byte），所以任何一個 Unicode 字元都放得下。

### `char` 和數字互轉

`char` 用 `as` 轉成整數，可以拿到它的編碼；反過來，只有 `u8` 可以用 `as` 轉成 `char`：

```rust
fn main() {
    let c = 'A';
    let code = c as u32;
    println!("{code}");
    let next = (c as u8 + 1) as char;
    println!("{next}");
}
```

執行結果：

```text
65
B
```

`c as u8 + 1` 是 `'A'` 的編碼加 1，再轉回 `char`，就是下一個字母。

### 跳脫字元

字串裡有些字元沒辦法直接打出來，要用反斜線 `\` 開頭的跳脫字元（escape）：

| 寫法 | 意思 |
| --- | --- |
| `\n` | 換行 |
| `\t` | Tab |
| `\\` | 反斜線本身 |
| `\"` | 雙引號 |
| `\'` | 單引號 |
| `\u{1F600}` | 用編碼指定的 Unicode 字元 |

```rust
fn main() {
    println!("第一行\n第二行");
    println!("路徑：C:\\Users");
    println!("他說：\"你好\"");
    println!("笑臉：\u{1F600}");
}
```

執行結果：

```text
第一行
第二行
路徑：C:\Users
他說："你好"
笑臉：😀
```

這些跟 Go 大致相同，只有 Unicode 的寫法不一樣：Go 要寫 `\u` 加 4 位數或 `\U` 加 8 位數（例如 `\U0001F600`），Rust 把編碼放在大括號裡，1 到 6 位數都可以。

### raw string：不跳脫

路徑、正規表達式這類有很多反斜線的內容，一個一個加跳脫很煩。字串前面加 `r`，就變成 raw string，裡面的反斜線不會被當成跳脫：

```rust
fn main() {
    let path = r"C:\Users\andy\notes.txt";
    println!("{path}");
    let quote = r#"他說："你好""#;
    println!("{quote}");
}
```

執行結果：

```text
C:\Users\andy\notes.txt
他說："你好"
```

如果內容裡有雙引號，就在前後各加一個 `#`：`r#"..."#`。`#` 可以加更多個，只要頭尾的數量一樣。

### 跟 Go 比一比

Go 的單引號字元是 `rune`，其實是整數，可以直接做加減，`'a' + 1` 得到 98。Rust 的 `char` 不是數字，不能直接運算：

```rust,compile_fail=E0369
fn main() {
    let c = 'a';
    let d = c + 1;
    println!("{d}");
}
```

編譯錯誤：

```text
error[E0369]: cannot add `{integer}` to `char`
 --> src/main.rs:3:15
  |
3 |     let d = c + 1;
  |             - ^ - {integer}
  |             |
  |             char
```

要算就像前面那樣，先 `as u8`，算完再 `as char`。

Go 的 raw string 用反引號 `` `...` ``，Rust 用 `r"..."`。

## 重點整理

- `char` 用單引號，一個 `char` 是一個 Unicode 字元，固定佔 4 個位元組。
- `char as u32` 取得編碼；只有 `u8` 可以用 `as` 轉成 `char`。
- 跳脫字元：`\n`、`\t`、`\\`、`\"`、`\'`、`\u{...}`。
- raw string 寫成 `r"..."`，不處理跳脫；內容有雙引號時寫成 `r#"..."#`。
- Go 的 `rune` 是整數，可以直接運算；Rust 的 `char` 不是，不能 `'a' + 1`。
