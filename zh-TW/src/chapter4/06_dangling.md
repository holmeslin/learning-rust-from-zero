# 懸置參照

## 本集目標

知道什麼是懸置參照（dangling reference）：參照還在，它指向的值卻已經被 drop。Rust 在編譯時期就擋下它（E0106、E0597），改法是回傳擁有的值。

## 正文

### 指著已經消失的東西

參照借的是別人的值。如果那個值先被 drop 了，參照就指著一塊已經釋放的記憶體，這就是**懸置參照**。很多語言在這裡會出現難追的 bug，Rust 乾脆不讓它編譯。

最常見的寫法，是函式想把自己建立的 `String` 用參照回傳：

```rust,compile_fail=E0106
fn dangle() -> &String {
    let s = String::from("哈囉");
    &s
}

fn main() {
    let r = dangle();
    println!("{r}");
}
```

編譯錯誤：

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:16
  |
1 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
1 | fn dangle() -> &'static String {
  |                 +++++++
help: instead, you are more likely to want to return an owned value
  |
1 - fn dangle() -> &String {
1 + fn dangle() -> String {
  |
```

`s` 是函式裡的區域變數，`dangle` 的 `}` 一到就被 drop；要是真的把 `&s` 交出去，呼叫端拿到的參照，就會指著已釋放的記憶體。

訊息說 `missing lifetime specifier`（缺少生命週期標註）。編譯器想問的是：回傳的這個參照，是借自誰的？參數裡沒有任何東西可以借，它就沒辦法交代。生命週期第 10 章才會正式介紹，現在不用深究；`help` 先建議了 `'static`，那不是我們要的，請忽略它，看下一段 `help`：`instead, you are more likely to want to return an owned value`，意思是你比較可能是想回傳擁有的值。

### 改成回傳擁有的值

把回傳型別改成 `String`，直接把 `String` 交出去，所有權交給呼叫端（第 3 章第 10 集）：

```rust
fn no_dangle() -> String {
    let mut s = String::from("哈囉");
    s.push_str("，世界");
    s
}

fn main() {
    let r = no_dangle();
    println!("{r}");
}
```

執行結果：

```text
哈囉，世界
```

函式裡建立的東西，要交給呼叫端，就靠回傳所有權；不要想著借出去。

### 能回傳的參照，是借自參數的

回傳參照本身不是問題，要看它借自誰。傳進函式的參照，指向的是呼叫端的值，函式結束後它還活著，借出去不會懸空。這類函式從第 8 集起會看到。

本書這幾章寫的、會回傳參照的函式，一律只放**一個**參照參數，編譯器就知道回傳的參照借自它。如果有兩個以上的參照參數，編譯器分不出借自哪一個，也會報 E0106，第 10 章再解決。

### 區塊結束也一樣

不只函式，區塊也會造成懸置參照。內層區塊的變數，在內層的 `}` 就被 drop：

```rust,compile_fail=E0597
fn main() {
    let r = {
        let s = String::from("哈囉");
        &s
    };
    println!("{r}");
}
```

編譯錯誤：

```text
error[E0597]: `s` does not live long enough
 --> src/main.rs:4:9
  |
2 |     let r = {
  |         - borrow later stored here
3 |         let s = String::from("哈囉");
  |             - binding `s` declared here
4 |         &s
  |         ^^ borrowed value does not live long enough
5 |     };
  |     - `s` dropped here while still borrowed
```

`borrowed value does not live long enough`：`s` 活得不夠久。區塊的值是 `&s`，被放進外層的 `r`，可是 `s` 在區塊的 `}` 就被 drop 了。規則可以這樣記：**參照不能比它借的東西活得久**。

### 跟 Go 比一比

這個錯誤，Go 程式設計師很容易覺得莫名其妙，因為 Go 允許這樣寫：

```go
func dangle() *string {
    s := "哈囉"
    return &s // 沒問題
}
```

Go 發現 `s` 的位址被帶出函式，會自動把 `s` 放到 heap，再靠垃圾回收決定什麼時候釋放。Rust 沒有垃圾回收，值在擁有者的 `}` 就釋放，所以不能把區域變數的參照帶出去，要回傳值本身。

## 重點整理

- 懸置參照：參照還在，指向的值已經被 drop。規則是參照不能比它借的東西活得久，Rust 在編譯時期就擋下它。
- 函式回傳區域變數的參照，是 E0106（`missing lifetime specifier`）；參照活得比內層區塊的變數久，是 E0597（`does not live long enough`）。
- 函式裡建立的值要交出去，改成回傳擁有的值（回傳 `String`）。
- 回傳的參照要借自傳進來的參數；本書這幾章回傳參照的函式只放一個參照參數，生命週期第 10 章正式介紹。
- Go 會把逃出函式的區域變數搬到 heap，所以 `return &s` 沒問題；Rust 沒有垃圾回收，不行。
