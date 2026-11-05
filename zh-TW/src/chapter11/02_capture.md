# 擷取環境

## 本集目標

知道閉包可以直接使用外面的變數（擷取），預設是用借用的方式；理解借用規則在閉包身上照樣成立。

## 正文

### 函式看不到外面，閉包可以

函式裡面不能直接用外面的區域變數：在 `main` 裡面再寫一個 `fn with_tax`，用到 `main` 的 `tax`，編譯器會報 E0434（`can't capture dynamic environment in a fn item`），`help` 建議改用 `|| { ... }` 的閉包。閉包定義的位置在哪裡，就看得到那裡的變數：

```rust
fn main() {
    let tax = 5;
    let with_tax = |price: u32| price + tax;
    println!("{}", with_tax(30));
    println!("{}", with_tax(45));
}
```

執行結果：

```text
35
50
```

閉包把用到的外部變數收進自己裡面，這叫**擷取**（capture）。`with_tax` 沒有把 `tax` 當參數傳進來，卻能用它。

### 預設是借用

擷取預設不搬走變數，只是借用。閉包只是讀 `tax`，就借一個 `&`；既然是借用，第 4 章的規則照樣有效：閉包還要再用的時候，不能改動 `tax`：

```rust,compile_fail=E0506
fn main() {
    let mut tax = 5;
    let with_tax = |price: u32| price + tax;
    tax = 8;
    println!("{}", with_tax(30));
    println!("{tax}");
}
```

編譯錯誤：

```text
error[E0506]: cannot assign to `tax` because it is borrowed
 --> src/main.rs:4:5
  |
3 |     let with_tax = |price: u32| price + tax;
  |                    ------------         --- borrow occurs due to use in closure
  |                    |
  |                    `tax` is borrowed here
4 |     tax = 8;
  |     ^^^^^^^ `tax` is assigned to here but it was already borrowed
5 |     println!("{}", with_tax(30));
  |                    -------- borrow later used here
```

`with_tax` 借了 `tax`，直到最後一次呼叫才結束（第 4 章第 5 集的 NLL）。第 4 行改 `tax` 的時候借用還在，所以是 E0506，跟第 4 章 `let r = &x;` 之後改 `x` 是同一種錯誤。把 `tax = 8;` 移到最後一次 `with_tax(30)` 之後就沒事。

### 改動變數：借出 `&mut`

閉包主體要改動外面的變數，擷取的就是 `&mut`：

```rust
fn main() {
    let mut total = 0;
    let mut add = |price: u32| total += price;
    add(30);
    add(45);
    println!("共 {total}");
}
```

執行結果：

```text
共 75
```

`total` 要用 `let mut`；閉包 `add` 也要 `let mut`，因為呼叫它會改動它擷取的東西，原因第 4 集說明。`add` 持有 `total` 的 `&mut` 到最後一次呼叫為止，所以最後的 `println!` 讀得到。要是在兩次呼叫之間就想讀 `total`：

```rust,compile_fail=E0502
fn main() {
    let mut total = 0;
    let mut add = |price: u32| total += price;
    add(30);
    println!("{total}");
    add(45);
}
```

編譯錯誤：

```text
error[E0502]: cannot borrow `total` as immutable because it is also borrowed as mutable
 --> src/main.rs:5:16
  |
3 |     let mut add = |price: u32| total += price;
  |                   ------------ ----- first borrow occurs due to use of `total` in closure
  |                   |
  |                   mutable borrow occurs here
4 |     add(30);
5 |     println!("{total}");
  |                ^^^^^ immutable borrow occurs here
6 |     add(45);
  |     --- mutable borrow later used here
```

訊息指出 `total` 在閉包裡被可變借用（`first borrow occurs due to use of total in closure`），第 5 行想讀它，而 `add(45)` 還要用那個可變借用。「一個 `&mut` 或多個 `&`」這條規則，對閉包一樣適用。

### 編譯器替你選

你不用自己說要借 `&`、借 `&mut`，還是整個拿走。編譯器看閉包的主體做了什麼，選最少需要的那一種：只是讀就借 `&`，改動就借 `&mut`；主體要把值交出去或用掉（例如回傳它），才會拿走所有權，第 4 集再看。想強制拿走所有權，用下一集的 `move`。

### 跟 Go 比一比

Go 的閉包也會擷取外面的變數，但永遠是共用同一個變數，想讀想改都不受限：

```go
total := 0
add := func(price int) { total += price }
add(30)
fmt.Println(total) // 30
add(45)
```

同樣的寫法在 Rust 是 E0502（上一個範例）。Go 靠你自己小心，多個 goroutine 同時改要用 `-race` 才抓得到；Rust 把「閉包借走變數的期間，別處不能亂動」變成編譯器檢查。

## 重點整理

- 函式用不到外面的區域變數（E0434）；閉包可以，這叫擷取。
- 擷取預設是借用：只是讀就借 `&`，改動就借 `&mut`。借用規則照舊：閉包還要用的時候，不能改動被擷取的變數（E0506），改動型的閉包借走期間，別處也不能讀（E0502）。
- 閉包借用到最後一次呼叫為止（NLL）；會改動變數的閉包，要用 `let mut` 宣告。
- 編譯器看主體做了什麼，選最少需要的擷取方式：讀、改，或拿走所有權。
- 跟 Go 比：Go 的閉包永遠共用同一個變數，不受限；Rust 的借用規則在閉包上照樣檢查。
