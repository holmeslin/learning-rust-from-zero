# 字串不能用索引

## 本集目標

知道 `String` 和 `&str` 不能寫 `s[0]`（E0277），並說得出原因：字串是 UTF-8，一個字元可能佔好幾個位元組。

## 正文

### `len` 是位元組數

第 2 章第 15 集看過：`len()` 數的是位元組，不是字元。英文字母佔 1 個，中文字佔 3 個：

```rust
fn main() {
    let en = String::from("tea");
    let zh = String::from("紅茶");
    println!("{} {}", en.len(), zh.len());
}
```

執行結果：

```text
3 6
```

`String` 裡存的是 UTF-8 編碼的位元組。「第 0 個位置」到底是第一個字元，還是第一個位元組？兩個答案不一樣，Rust 乾脆不讓你選。

### `s[0]` 編譯不過

```rust,compile_fail=E0277
fn main() {
    let s = String::from("紅茶");
    let first = s[0];
    println!("{first}");
}
```

編譯錯誤：

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:19
  |
3 |     let first = s[0];
  |                   ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
```

`` the type `str` cannot be indexed by `{integer}` ``：字串不能用整數當索引。後面還有 `help` 和 `note`，提到的 `chars` 和 `bytes` 下一集就學。

不讓索引的理由有兩個：

- 位元組不是字元。對「紅茶」寫 `s[0]`，如果可以，得到的只會是第一個位元組，一個沒有意義的數字，不是「紅」。
- 中文字佔 3 個位元組，英文字母佔 1 個。要找第 n 個字元，就得從頭一個個數過去；索引看起來像是一步到位，其實不是。

### 用位元組範圍切

第 4 章第 8 集的字串 slice 是可以用的，數字一樣是位元組位置：

```rust
fn main() {
    let s = String::from("紅茶 tea");
    println!("{}", &s[0..3]);
    println!("{}", &s[7..]);
}
```

執行結果：

```text
紅
tea
```

「紅」佔位元組 0 到 2，「茶」佔 3 到 5，空格是第 6 個，所以 `tea` 從第 7 個位元組開始。你得自己知道邊界在哪裡；切在字元中間會 panic（第 4 章第 8 集）。要逐字處理，用下一集的 `chars`。

### 跟 Go 比一比

Go 的 `len(s)` 也是位元組數，但 Go 允許對字串用索引：

```go
s := "紅茶"
fmt.Println(len(s))       // 6
fmt.Println(s[0])         // 231：一個位元組，編譯成功
fmt.Println(string(s[0])) // ç：不是「紅」
for i, r := range s {
	fmt.Println(i, string(r)) // 0 紅、3 茶：range 會一個字元一個字元走
}
```

- Go 的 `s[0]` 編譯成功，拿到的是位元組。中文字串的 `s[i]` 幾乎都是用錯，但編譯器不吭聲。Rust 直接拒絕。
- Go 的 `for range` 自動以字元為單位；Rust 沒有預設，要自己選 `chars`（字元）或 `bytes`（位元組），下一集學。

## 重點整理

- `String` 與 `&str` 不能用整數索引，`s[0]` 是 E0277。
- 原因：字串是 UTF-8，位元組不等於字元，一個字元可能佔 1～4 個位元組。
- `len()` 是位元組數；`&s[起..迄]` 用位元組位置切，切在字元中間會 panic。
- 想逐字處理要用 `chars()`，想逐位元組處理要用 `bytes()`（下一集）。
- 跟 Go 比：Go 的 `s[i]` 編譯成功、拿到位元組；Rust 直接拒絕；Go 的 `range` 自動走字元，Rust 要自己選。
