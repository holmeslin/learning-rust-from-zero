# `From` 與 `Into`

## 本集目標

實作 `From` 讓型別能從別的型別轉換出來，並知道實作之後自動多了 `.into()`；認得 `String::from`、`i64::from` 都是同一個 trait。

## 正文

### 自己實作 `From`

`From` 是「從別的型別轉換出來」的 trait。下面讓 `Member` 能從 `&str` 轉換出來，叫名字就建立一個點數 0 的會員：

```rust
struct Member {
    name: String,
    points: u32,
}

impl From<&str> for Member {
    fn from(name: &str) -> Member {
        Member {
            name: String::from(name),
            points: 0,
        }
    }
}

fn main() {
    let a = Member::from("小美");
    let b: Member = "小華".into();
    println!("{} {}", a.name, a.points);
    println!("{} {}", b.name, b.points);
}
```

執行結果：

```text
小美 0
小華 0
```

- `impl From<&str> for Member` 讀成「`Member` 可以從 `&str` 轉換出來」。`From` 後面的尖括號是來源型別，就像 `Vec<T>` 的 `T`。
- `from` 沒有 `self`，是第 5 章第 9 集的關聯函式，用 `Member::from("小美")` 呼叫。回傳型別寫完整的 `Member`。
- `"小華".into()` 是反過來的寫法：從手上的值出發，要變成什麼型別，由左邊的型別標註 `let b: Member` 決定。
- 只需要實作 `From`，`.into()` 就自動有了；反過來不行，所以永遠實作 `From`。
- `.into()` 一定要讓編譯器知道目標型別。`let a = "小美".into();` 沒有標型別，是 E0283（`type annotations needed`）。

### 其實早就用過了

第 2 章的 `String::from("哈囉")`，就是標準函式庫替 `String` 實作的 `From<&str>`。第 1 章第 7 集 `help` 建議的 `i64::from(a)` 也是同一個 trait：

```rust
fn greet(name: impl Into<String>) {
    let name: String = name.into();
    println!("歡迎 {name}");
}

fn main() {
    let text = String::from("哈囉");
    let small: i32 = 30;
    let big = i64::from(small);
    let ratio = f64::from(small);
    let tea: String = "奶茶".into();
    println!("{text} {big} {ratio} {tea}");
    greet("小美");
    greet(String::from("小華"));
}
```

執行結果：

```text
哈囉 30 30 奶茶
歡迎 小美
歡迎 小華
```

- `i64::from(small)`、`f64::from(small)`：整數轉成範圍更大的整數或浮點數，不會出錯、也不會損失，所以標準函式庫都有 `From`。
- `greet` 的參數 `impl Into<String>` 讀成「任何能轉成 `String` 的東西」。傳 `&str` 或 `String` 都行，函式裡面 `.into()` 一次，就拿到 `String`。想同時收字面值跟 `String` 時，常這樣寫。

### `From` 只管不會失敗的轉換

反過來，把 `i64` 轉成 `i32` 可能放不下，標準函式庫就沒有提供 `From`：

```rust,compile_fail=E0277
fn main() {
    let big: i64 = 30;
    let small = i32::from(big);
    println!("{small}");
}
```

編譯錯誤：

```text
error[E0277]: the trait bound `i32: From<i64>` is not satisfied
 --> src/main.rs:3:17
  |
3 |     let small = i32::from(big);
  |                 ^^^ the trait `From<i64>` is not implemented for `i32`
  |
  = help: `i32` implements trait `From<T>`:
            From<bool>
            From<i16>
            From<i8>
            From<u16>
            From<u8>
```

`` the trait bound `i32: From<i64>` is not satisfied ``。後面的 `help` 列出 `i32` 能從哪些型別轉換：`i8`、`i16`、`u8`、`u16`、`bool`，都是不會出錯的。要轉成範圍較小的整數，用第 1 章第 17 集的 `as`，並自己負責截斷的後果。

## 重點整理

- `impl From<來源> for 目標 { fn from(x: 來源) -> 目標 { ... } }`：讓目標型別能從來源轉換出來，用 `目標::from(x)` 呼叫。
- 實作 `From` 就自動有 `.into()`；永遠實作 `From`，不要自己寫 `Into`。
- `.into()` 要靠標註或上下文知道目標型別，否則是 E0283。
- `String::from("...")`、`i64::from(x)` 都是標準函式庫的 `From`；`From` 只用在不會失敗、不損失的轉換，`i32::from(i64 的值)` 是 E0277。
- 參數寫 `impl Into<String>`，能同時收 `&str` 和 `String`。
