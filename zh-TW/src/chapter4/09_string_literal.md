# 字串字面值就是 `&str`

## 本集目標

知道 `"哈囉"` 這種字串字面值的型別就是 `&str`：它是參照，不能修改，也不擁有文字。

## 正文

### 字面值的型別

第 2 章第 15 集說過，字串字面值不是 `String`，當時只說它的型別叫 `&str`。現在可以把它讀懂了：`&str` 就是上一集的字串 slice，只不過這一段文字不是借自某個 `String`，而是直接寫在程式裡：

```rust
fn main() {
    let s: &str = "哈囉";
    println!("{s} {}", s.len());
}
```

執行結果：

```text
哈囉 6
```

程式編譯之後，字面值文字和程式放在一起，位置是唯讀的，整個程式執行期間都在。`s` 只是指向它的參照。編譯器訊息裡偶爾會看到 `&'static str`，指的就是這種參照；`'static` 是什麼，第 10 章再說，現在把它當成 `&str` 的另一種寫法就好。

### 字面值不能改

因為 `&str` 是對唯讀文字的參照，沒有 `push_str` 這類會改動文字的方法：

```rust,compile_fail=E0599
fn main() {
    let mut s = "哈囉";
    s.push_str("，世界");
    println!("{s}");
}
```

編譯錯誤：

```text
error[E0599]: no method named `push_str` found for reference `&str` in the current scope
 --> src/main.rs:3:7
  |
3 |     s.push_str("，世界");
  |       ^^^^^^^^ method not found in `&str`
```

訊息說 `&str` 上找不到 `push_str`。想要能增長的文字，就要有一份自己擁有的 `String`：用 `String::from("哈囉")` 把字面值的文字複製進 heap，擁有者是你，才能改。這也是第 2 章第 15 集那個 E0308 的原因：`let name: String = "小明";` 要的是擁有文字的 `String`，給的卻是借來的 `&str`。

### 參照可以複製，也可以指向別處

第 3 章第 8 集把字串字面值列為 `Copy` 型別。現在知道原因了：`&str` 是參照，不擁有文字，複製它只是多一個指向同一段文字的參照：

```rust
fn main() {
    let a = "小明";
    let b = a;
    println!("{a} {b}");

    let mut c = "哈囉";
    println!("{c}");
    c = "再見";
    println!("{c}");
}
```

執行結果：

```text
小明 小明
哈囉
再見
```

`let mut c` 的 `mut`，讓變數 `c` 可以改成指向另一段文字，文字本身還是不能改。

### 字面值也能再切

`&str` 本身就是 slice，所以可以像上一集那樣再切一段：

```rust
fn main() {
    let s = "hello world";
    let hello = &s[0..5];
    println!("{hello}");
}
```

執行結果：

```text
hello
```

## 重點整理

- 字串字面值的型別是 `&str`，指向存在程式裡的唯讀文字，整個程式執行期間都有效。
- `&str` 不擁有文字，不能修改；沒有 `push_str`（E0599）。要能增長，用 `String::from("...")` 複製成自己擁有的 `String`。
- `&str` 是參照，所以可以複製（第 3 章第 8 集說字面值是 `Copy` 的原因）。
- `let mut c = "哈囉";` 的 `mut` 讓 `c` 能改指向別的文字，文字本身不能改。
- 字面值可以像 `String` 一樣用 `&s[起..迄]` 再切。
