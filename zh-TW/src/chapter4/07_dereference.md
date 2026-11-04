# 解參照 `*`

## 本集目標

知道 `*r` 取得參照指向的那個值（解參照，dereference），會在比較、以及用 `&mut` 修改時用到它，也知道它不能把借來的值搬走。

## 正文

### `*` 取出指向的值

`&` 把值包成參照，`*` 反過來，從參照取出它指向的值，叫**解參照**。比較兩個值的時候，就需要它：

```rust
fn main() {
    let x = 5;
    let r = &x;
    if *r == 5 {
        println!("r 指向的是 5");
    }
}
```

執行結果：

```text
r 指向的是 5
```

`r` 是 `&i32`，`*r` 才是 `i32`。拿 `&i32` 跟數字 `5` 直接比，編譯器不收：

```rust,compile_fail=E0277
fn main() {
    let x = 5;
    let r = &x;
    if r == 5 {
        println!("r 指向的是 5");
    }
}
```

編譯錯誤：

```text
error[E0277]: can't compare `&{integer}` with `{integer}`
 --> src/main.rs:4:10
  |
4 |     if r == 5 {
  |          ^^ no implementation for `&{integer} == {integer}`
  |
  = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
help: consider dereferencing here
  |
4 |     if *r == 5 {
  |        +
```

訊息說沒辦法比較 `&{integer}` 跟 `{integer}`。後面提到的 `PartialEq` 是第 8 章的 trait，先不用管，記得「參照和值是不同的型別，要比就先 `*`」。

### 用 `&mut` 改值

函式拿到 `&mut i32`，要改它指向的數字，得寫 `*n`；對 `&mut String` 寫 `*s = ...`，則是把指向的整個 `String` 換掉（不像 `push_str` 只是在後面接字），舊的那份就地被 drop（第 3 章第 4 集）：

```rust
fn add_one(n: &mut i32) {
    *n += 1;
}

fn reset(s: &mut String) {
    *s = String::from("重來");
}

fn main() {
    let mut count = 0;
    add_one(&mut count);
    add_one(&mut count);

    let mut s = String::from("哈囉，世界");
    reset(&mut s);
    println!("{count} {s}");
}
```

執行結果：

```text
2 重來
```

`*n += 1` 改的是 `main` 裡 `count` 本身，所以呼叫兩次之後是 2。如果忘了 `*`，寫成 `n += 1`，會得到 E0368（`+=` 不能用在 `&mut i32` 上），`help` 會建議補上 `*`。

### 很多時候可以省略

不是每個地方都要寫 `*`。印出參照，或是對參照呼叫 `len`、`push_str` 這類方法，編譯器會自動處理，所以前面幾集才不用寫。方法為什麼可以省略，第 5 章第 10 集解釋。需要自己寫 `*` 的，是拿值去比、對數字運算和指定。

### `*` 搬不走借來的值

`*r` 取出來的是借來的值，所有權還是別人的。`i32` 這種 `Copy` 型別取出來是複製，沒問題；`String` 不是 `Copy`，想把它整個搬走就會被擋下來：

```rust,compile_fail=E0507
fn main() {
    let s = String::from("哈囉");
    let r = &s;
    let t = *r;
    println!("{t}");
}
```

編譯錯誤：

```text
error[E0507]: cannot move out of `*r` which is behind a shared reference
 --> src/main.rs:4:13
  |
4 |     let t = *r;
  |             ^^ move occurs because `*r` has type `String`, which does not implement the `Copy` trait
  |
help: consider removing the dereference here
  |
4 -     let t = *r;
4 +     let t = r;
  |
help: consider cloning the value if the performance cost is acceptable
  |
4 -     let t = *r;
4 +     let t = r.clone();
  |
```

`cannot move out of *r which is behind a shared reference`：不能從 `&` 後面把值 move 出來。借用不轉移所有權，借來的人沒有資格把它拿走。

`help` 列了兩條出路：去掉 `*`（`t` 就只是另一個參照），或是 `clone` 一份新的。看你要的是「再借一次」，還是「要自己的一份」。

### 跟 Go 比一比

Go 的 `*p` 也是取出指標指向的值，寫法一樣。不一樣的是印出來的東西：

```go
x := 5
p := &x
fmt.Println(p)  // 0xc000012345，是位址
fmt.Println(*p) // 5
```

Rust 的 `println!("{r}")` 印的是 `r` 指向的值 `5`，不是位址。

## 重點整理

- `*r` 解參照：從參照取出它指向的值。比較、對數字運算與指定（`*n += 1`）都要自己寫 `*`；對 `&mut String` 寫 `*s = ...`，是整個換成新值。
- 印出參照、呼叫方法時不需要 `*`，編譯器會處理（第 5 章第 10 集解釋）。
- `&i32` 跟 `i32` 是不同型別，直接比較是 E0277；對 `&mut i32` 寫 `n += 1` 是 E0368。
- `*r` 不能把非 `Copy` 的值搬出來，是 E0507：借用不轉移所有權。
- Go 印指標得到位址，Rust 印參照得到值。
