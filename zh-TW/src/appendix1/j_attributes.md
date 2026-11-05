# 屬性 `#[...]` 與 `#[allow]`

## 本集目標

認得屬性（attribute）的兩種寫法 `#[...]` 與 `#![...]`，並用 `#[allow]`、`#[deny]` 調整單一條警告的等級。

## 正文

### 屬性是什麼

`#[...]` 寫在函式、struct、`mod`、陳述式前面，替後面那個東西附上給編譯器看的資訊（第 12 章第 10 集提過）。前面用過的有 `#[derive(Debug)]`（第 5 章）、`#[test]`、`#[should_panic]`、`#[cfg(test)]`（第 12 章）。這一集的 `#[allow(...)]` 用來調整警告。

### 用 `#[allow]` 關掉一條警告

第 5 章第 6 集看過：`#[derive(Debug)]` 的 struct，欄位如果從沒被讀過，rustc 會警告：

```rust,warn
#[derive(Debug)]
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        points: 10,
    };
    println!("{}", member.name);
}
```

編譯警告：

```text
warning: field `points` is never read
 --> src/main.rs:4:5
  |
2 | struct Member {
  |        ------ field in this struct
3 |     name: String,
4 |     points: u32,
  |     ^^^^^^
  |
  = note: `Member` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default
```

警告的最後一行 `#[warn(dead_code)]` 就是這條檢查的名字，叫 lint。確定這個欄位先留著沒問題，就在 struct 前面加 `#[allow(dead_code)]`：

```rust
#[derive(Debug)]
#[allow(dead_code)]
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        points: 10,
    };
    println!("{}", member.name);
}
```

執行結果：

```text
小美
```

`allow` 的意思是「這條 lint 在這裡不要報」，括號裡可以放好幾條，用逗號分開。

### 管得到哪裡

屬性只管它後面的那一個東西：寫在函式前面就管整個函式，寫在 `mod` 前面管整個模組，寫在 `let` 前面只管那一行。

`#![allow(...)]` 多了一個驚嘆號，管的是它所在的整個檔案（crate），只能寫在檔案最上面。範圍太大，通常只用在整個 crate 都適用的設定；原則是範圍越小越好。

### `deny` 與 `expect`

lint 有四個常用等級：`allow`（不報）、`warn`（警告，多數 lint 的預設）、`deny`（當成錯誤）、`forbid`（比 `deny` 嚴，裡面不能再改回去）。`deny` 能把你在乎的警告升級成編譯錯誤：

```rust,compile_fail
#![deny(unused_variables)]

fn main() {
    let x = 5;
    println!("哈囉");
}
```

編譯錯誤：

```text
error: unused variable: `x`
 --> src/main.rs:4:9
  |
4 |     let x = 5;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
note: the lint level is defined here
 --> src/main.rs:1:9
  |
1 | #![deny(unused_variables)]
  |         ^^^^^^^^^^^^^^^^
```

這是沒有錯誤碼的錯誤，`note` 指出 lint 等級是在第一行定義的。第 12 章第 9 集的 `cargo clippy -- -D warnings`，就是把所有警告都升級成錯誤。

另一個是 `#[expect(dead_code)]`：跟 `allow` 一樣不報，但如果之後這條警告不再出現（欄位終於被用到了），它會反過來警告「預期落空」，提醒你把這個屬性拿掉。

### clippy 的 lint

clippy 的 lint 名字前面有 `clippy::`。第 12 章第 9 集看過的 `len_zero`，確定這裡要寫成 `orders.len() == 0` 的話：

```rust
#[allow(clippy::len_zero)]
fn is_empty_order(orders: &[u32]) -> bool {
    orders.len() == 0
}

fn main() {
    println!("{}", is_empty_order(&[]));
}
```

執行結果：

```text
true
```

名字打錯的話，編譯器會警告 `unknown lint`，還會建議正確的名字，所以不會悄悄失效。

看到警告，先想能不能改程式（刪掉沒用的、補上漏掉的、第 1 章第 19 集的 `_x`），改不了、又確定沒問題，才用 `#[allow]`，最好留一行註解寫原因。`#![allow(warnings)]` 會關掉整個檔案的警告，不要用。

### 跟 Go 比一比

Go 的未使用變數、未使用 import 是編譯錯誤，編譯器沒有任何開關可以關掉，只能刪掉或寫 `_ = x`。Rust 預設只是警告，可以用屬性調整等級。Go 的 linter 要靠 `//nolint` 這類註解，每個工具各有各的寫法；Rust 的 `#[allow]` 是語言本身的一部分，lint 名字也會被檢查。

## 重點整理

- `#[...]` 是屬性，附在後面那個東西上；`#![...]` 管它所在的整個檔案。
- `#[allow(lint名字)]` 關掉一條 lint，`#[deny(...)]` 把它當成錯誤，`warn` 是警告，`forbid` 比 `deny` 嚴；`#[expect(...)]` 在警告不再出現時會提醒你拿掉。
- 警告裡的 `#[warn(...)]` 就是 lint 的名字；clippy 的 lint 要寫 `clippy::名字`。
- 屬性只管它後面的那一個東西，範圍越小越好；不要用 `#![allow(warnings)]`。
- 先改程式碼，真的需要才 `#[allow]`。
- Go 的未使用變數是編譯錯誤、關不掉；Rust 預設只是警告，可以用屬性調整。
