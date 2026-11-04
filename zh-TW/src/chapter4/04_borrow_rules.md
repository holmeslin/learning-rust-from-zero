# 借用規則：多個 `&` 或一個 `&mut`

## 本集目標

記住借用規則：同一時間，可以有很多個 `&`，或是只有一個 `&mut`，兩者不能並存；看到 E0499、E0502 時知道哪一條被違反。

## 正文

### 規則

同一個值，在同一時間：

- 可以有**任意多個** `&`（只能讀）；
- 或者只有**一個** `&mut`（可改）；
- 不能一邊有 `&`、一邊有 `&mut`。

簡單說：很多人可以一起讀，但寫的時候只能有一個人，而且沒有人在讀。

### 很多個 `&` 沒問題

```rust
fn main() {
    let s = String::from("哈囉");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");
}
```

執行結果：

```text
哈囉 哈囉
```

大家都只是讀，互不影響。

### 兩個 `&mut` 不行

```rust,compile_fail=E0499
fn main() {
    let mut s = String::from("哈囉");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{r1} {r2}");
}
```

編譯錯誤：

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:4:14
  |
3 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
4 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
5 |     println!("{r1} {r2}");
  |                -- first borrow later used here
```

E0499 是「同時借出兩個 `&mut`」。訊息指出三個位置：第一個可變借用在哪裡（`first mutable borrow occurs here`）、第二個在哪裡（`second mutable borrow occurs here`），以及第一個借用後來還在被用到的地方（`first borrow later used here`）。

### `&` 和 `&mut` 不能並存

先借出 `&s` 來讀，又想直接改 `s`：

```rust,compile_fail=E0502
fn main() {
    let mut s = String::from("哈囉");
    let r = &s;
    s.push_str("，世界");
    println!("{r}");
}
```

編譯錯誤：

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let r = &s;
  |             -- immutable borrow occurs here
4 |     s.push_str("，世界");
  |     ^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("{r}");
  |                - immutable borrow later used here
```

E0502 是「`&` 和 `&mut` 撞在一起」。`s.push_str(...)` 要改動 `s`，也就是借出 `&mut s`，可是 `r` 這個 `&` 後面還要用，兩個借用就重疊了。

反過來，先借出 `&mut`，之後連擁有者自己想讀都不行：

```rust,compile_fail=E0502
fn main() {
    let mut s = String::from("哈囉");
    let r = &mut s;
    println!("{s}");
    r.push_str("，世界");
}
```

編譯錯誤：

```text
error[E0502]: cannot borrow `s` as immutable because it is also borrowed as mutable
 --> src/main.rs:4:16
  |
3 |     let r = &mut s;
  |             ------ mutable borrow occurs here
4 |     println!("{s}");
  |                ^ immutable borrow occurs here
5 |     r.push_str("，世界");
  |     - mutable borrow later used here
```

`println!("{s}")` 讀 `s`，等於借出 `&s`，但 `r` 這個 `&mut` 後面還要用。可變參照在的時候，所有人都不能碰，包括擁有者。

### 為什麼要有這條規則

想像 `r` 正指著 `s` 的文字在讀，這時 `push_str` 發現原本的 heap 空間不夠，換到一塊更大的，舊的那塊就還掉了。`r` 還指著舊的位置，讀到的就是已經釋放的記憶體。

「有人在讀的時候，不准有人改」，就能保證這種事不會發生。Rust 把檢查放在編譯時期，所以跑起來的程式不會有這類錯誤。

解法是調整順序，先把一個借用用完，再借下一個。為什麼這樣就行，下一集說明。

### 跟 Go 比一比

Go 允許好幾個指標指著同一個變數，而且每個都能寫：

```go
msg := "哈囉"
p1 := &msg
p2 := &msg
*p1 += "，世界"
*p2 += "！" // 編譯沒問題
```

在單一 goroutine 裡這沒事；跨 goroutine 同時讀寫，才會是 data race，要靠 `go run -race` 在執行時抓。Rust 的借用規則在編譯時期就禁止「有人讀、有人寫」或「兩人同時寫」，data race 根本寫不出來。

## 重點整理

- 同一時間，同一個值可以有任意多個 `&`，或是只有一個 `&mut`，不能並存。
- 兩個 `&mut` 同時存在是 E0499；`&` 與 `&mut` 同時存在是 E0502。
- 直接呼叫 `s.push_str(...)` 也算借出 `&mut s`；有 `&mut` 的時候，擁有者自己也不能讀。
- 規則的目的：有人讀的時候不准有人改，避免讀到已經釋放的記憶體。解法是調整順序，先把一個借用用完再借下一個（下一集說明）。
- Go 的指標沒有這種限制，data race 要等執行時用 `-race` 才抓得到；Rust 在編譯時期擋下。
