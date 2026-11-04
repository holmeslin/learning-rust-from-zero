# 借用不轉移所有權

## 本集目標

把 `&String` 傳進函式，函式讀完還給你：呼叫之後，原本的變數仍然是擁有者，照樣能用。

## 正文

### 第 3 章留下的問題

第 3 章第 11 集，為了算長度，函式得把 `String` 拿走、再連同結果一起還回來。現在改成讓函式「借」：

```rust
fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("哈囉");
    let n = length(&s);
    println!("{s} 的長度是 {n}");
}
```

執行結果：

```text
哈囉 的長度是 6
```

兩個地方有改動：參數型別從 `String` 變成 `&String`，呼叫時從 `length(s)` 變成 `length(&s)`。函式不用回傳 `s`，`main` 也不用接回來，`s` 一直都在。

### 為什麼 `s` 沒有被 drop

第 3 章第 9 集說過，`String` 傳進函式，在函式的 `}` 就被 drop。這次傳進去的是參照，`length` 的參數 `s` 只是一個指向 `main` 那個 `String` 的參照。`}` 一到，參照離開作用域，但 drop 的規則是「**擁有者**離開作用域才 drop 值」。參照不是擁有者，所以什麼也不會被釋放。

擁有者始終是 `main` 裡的 `s`，要等 `main` 結束才會 drop。這就是「借用不轉移所有權」：借出去的是使用權，所有權沒有動。

### 借幾次都可以

因為所有權沒有搬家，同一個值可以一直借出去：

```rust
fn show(s: &String) {
    println!("收到 {s}");
}

fn main() {
    let msg = String::from("哈囉");
    show(&msg);
    show(&msg);
    println!("main 還有 {msg}");
}
```

執行結果：

```text
收到 哈囉
收到 哈囉
main 還有 哈囉
```

第 3 章第 9 集的 `consume(msg)` 之後再用 `msg`，會得到 E0382。那則錯誤訊息還附了一段 `note`，建議把參數型別改成「borrow instead」，意思就是這個：函式只想讀，就別把所有權拿走。

### 參數再多也不麻煩

第 3 章第 11 集的 `total_length` 要回傳 `(String, String, usize)`。借用的版本：

```rust
fn total_length(a: &String, b: &String) -> usize {
    a.len() + b.len()
}

fn main() {
    let a = String::from("哈囉");
    let b = String::from("世界");
    let n = total_length(&a, &b);
    println!("{a} {b} 共 {n} 個位元組");
}
```

執行結果：

```text
哈囉 世界 共 12 個位元組
```

回傳值只有結果，簽名和呼叫端都短了很多。

### 忘了 `&` 會怎樣

呼叫時漏掉 `&`，傳進去的是 `String`，跟參數要的 `&String` 對不上：

```rust,compile_fail=E0308
fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("哈囉");
    let n = length(s);
    println!("{s} 的長度是 {n}");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:7:20
  |
7 |     let n = length(s);
  |             ------ ^ expected `&String`, found `String`
  |             |
  |             arguments to this function are incorrect
  |
note: function defined here
 --> src/main.rs:1:4
  |
1 | fn length(s: &String) -> usize {
  |    ^^^^^^ ----------
help: consider borrowing here
  |
7 |     let n = length(&s);
  |                    +
```

訊息說要的是 `&String`，給的是 `String`，`help` 直接指出要寫成 `&s`。函式的參數型別寫了 `&`，呼叫時也要自己寫 `&`，Rust 不會替你偷偷借。

## 重點整理

- 參數型別寫 `&String`，呼叫時傳 `&s`：函式只借用，不拿走所有權。
- 借用的參照離開作用域，不會 drop 值；擁有者還是原本的變數，呼叫之後照樣能用。
- 同一個值可以借出去很多次；第 3 章要靠回傳 tuple 交還所有權的麻煩，借用都省掉了。
- 漏掉呼叫端的 `&` 是 E0308（要 `&String`，給了 `String`），照 `help` 補上 `&` 即可。
