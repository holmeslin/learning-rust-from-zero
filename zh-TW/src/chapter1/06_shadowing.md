# shadowing

## 本集目標

用同一個名字再 `let` 一次（shadowing），建立一個新的變數蓋掉舊的，並分清楚它跟 `mut` 的差別。

## 正文

### 再 `let` 一次

同一個名字可以重複用 `let` 建立。後面的變數會**蓋住**前面的，這叫 shadowing（遮蔽）：

```rust
fn main() {
    let price = 100;
    let price = price + 20;
    println!("price = {price}");
    let price = price * 2;
    println!("price = {price}");
}
```

執行結果：

```text
price = 120
price = 240
```

每一個 `let price` 都是全新的變數，只是名字一樣。右邊的 `price + 20` 用的是前一個 `price`，算完之後才建立新的。

這不是改值。舊的 `price` 沒有被修改，只是從這一行開始，`price` 這個名字指到新的變數上。

### 內層區塊的 shadowing

在大括號 `{ }` 的區塊裡 shadow 一個名字，離開區塊之後，外面的變數就恢復原樣：

```rust
fn main() {
    let x = 1;
    {
        let x = 2;
        println!("內層 x = {x}");
    }
    println!("外層 x = {x}");
}
```

執行結果：

```text
內層 x = 2
外層 x = 1
```

### 跟 `mut` 的差別

shadowing 可以換型別，`mut` 不行：

```rust
fn main() {
    let x = 5;
    println!("{x}");
    let x = "五";
    println!("{x}");
}
```

執行結果：

```text
5
五
```

同樣的事情用 `mut` 做，就是型別不符的編譯錯誤：

```rust,compile_fail=E0308
fn main() {
    let mut x = 5;
    println!("{x}");
    x = "五";
    println!("{x}");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:4:9
  |
2 |     let mut x = 5;
  |                 - expected due to this value
3 |     println!("{x}");
4 |     x = "五";
  |         ^^^^ expected integer, found `&str`
```

shadowing 之後的變數一樣是不可變的（除非你寫 `let mut`）。它最常見的用途，是把「同一份資料」換個形式繼續用，例如把讀進來的文字去掉換行，或轉成數字，不必另外想新名字。第 9、10 集就會用到。

### 跟 Go 比一比

Go 的 `:=` 在同一個範圍裡不能重複宣告同一個名字：

```go
x := 1
x := 2 // 編譯錯誤：no new variables on left side of :=
```

只有進到內層區塊，Go 才能 shadow 外層的變數。Rust 連同一個區塊裡都可以，而且型別可以換。所以在 Rust 看到同名的 `let`，不要以為是寫錯：它是建立新變數，不是改舊變數。

## 重點整理

- 同名再寫一次 `let`，會建立新的變數蓋住舊的，這叫 shadowing。
- shadowing 不是改值，所以可以換型別；`mut` 只能改值，不能換型別。
- 內層區塊裡的 shadowing，離開區塊後就失效，外層恢復原樣。
- Go 同一個範圍裡不能重複 `:=` 同名變數，Rust 可以。
