# `Option<T>`

## 本集目標

知道 Rust 沒有 `nil`，改用 `Option<T>` 把「可能沒有值」寫進型別：認得 `Some(值)` 和 `None`，並知道 `Option<T>` 不能直接當 `T` 用。

## 正文

### 沒有 `nil`

第 1 章說過 Rust 沒有零值，連 Go 的 `nil` 也沒有。可是「可能沒有值」的情況天天遇到：查商品編號找不到、使用者沒填暱稱。Rust 用標準函式庫裡的一個 enum 來表示，叫 `Option<T>`，它的定義長這樣：

```rust,ignore
enum Option<T> {
    None,
    Some(T),
}
```

這就是前三集學的東西：兩個 variant，`None` 不帶資料，表示沒有；`Some` 帶一個值，表示有。

`<T>` 是泛型，第 8 章才會講。現在把 `Option<u32>` 整個當成一個型別的名字，讀成「可能有一個 `u32`」；換成 `Option<String>` 就是「可能有一個 `String`」。`Option`、`Some`、`None` 不用另外引入，也不用寫成 `Option::Some`，直接寫就行。

### 回傳 `Option` 的函式

```rust
fn find_price(code: u32) -> Option<u32> {
    if code == 1 {
        Some(30)
    } else if code == 2 {
        Some(45)
    } else {
        None
    }
}

fn main() {
    let tea = find_price(1);
    let missing = find_price(9);
    let nothing: Option<u32> = None;
    println!("{:?}", tea);
    println!("{:?}", missing);
    println!("{:?}", nothing);
}
```

執行結果：

```text
Some(30)
None
None
```

- `find_price` 找得到就回傳 `Some(30)`，找不到回傳 `None`。呼叫端拿到的永遠是 `Option<u32>`。
- `Option` 也能用 `{:?}` 印，印出來是 `Some(30)` 或 `None`。
- 單獨寫 `None` 時編譯器不知道裡面該是什麼型別，所以 `nothing` 要標上 `Option<u32>`。`find_price(9)` 不用標，因為函式簽名已經說了。

### 不能直接當 `u32` 用

`tea` 是「可能有一個 `u32`」，不是 `u32`，拿去做運算會被擋下來：

```rust,compile_fail=E0369
fn find_price(code: u32) -> Option<u32> {
    if code == 1 {
        Some(30)
    } else {
        None
    }
}

fn main() {
    let tea = find_price(1);
    let total = tea + 10;
    println!("{total}");
}
```

編譯錯誤：

```text
error[E0369]: cannot add `{integer}` to `Option<u32>`
  --> src/main.rs:11:21
   |
11 |     let total = tea + 10;
   |                 --- ^ -- {integer}
   |                 |
   |                 Option<u32>
```

`` cannot add `{integer}` to `Option<u32>` ``：`Option<u32>` 沒辦法跟整數相加。想用裡面的 `u32`，就得先處理「其實沒有」的情況，編譯器不會幫你假裝一定有。怎麼處理，下一集用 `match` 示範。

### 跟 Go 比一比

Go 用三種方式表示「沒有」，編譯器都不管你有沒有檢查：

```go
var p *int
fmt.Println(*p) // 編譯成功，執行時 panic: nil pointer dereference

prices := map[string]int{"紅茶": 30}
a := prices["咖啡"]     // 0：分不出是沒有，還是價格就是 0
b, ok := prices["咖啡"] // 要自己記得檢查 ok，不檢查也能編譯
fmt.Println(a, b, ok)
```

- Go 的 `nil` 指標：忘了檢查，到執行時才 panic。Rust 沒有 `nil`，`Option<T>` 跟 `T` 是不同型別，沒處理就不能用，錯誤在編譯時期就擋下。
- Go 取不到 map 的值，會得到零值，跟真的是 0 分不出來。Rust 的「沒有」是獨立的 `None`，不是某個假值。
- Go 的 `(值, ok)`、`(值, error)` 靠回傳值的慣例，檢查全憑自覺。Rust 把「可能沒有」寫在型別上，檢查變成編譯器的工作。

## 重點整理

- Rust 沒有 `nil`；「可能沒有值」用 `Option<T>` 表示，它是 enum：`None`（沒有）和 `Some(T)`（有一個值）。
- `<T>` 是泛型（第 8 章）；先把 `Option<u32>` 當成「可能有一個 `u32`」的型別名字。
- `Some`、`None` 直接寫，不用 `Option::`；單獨寫 `None` 時，要靠型別標註讓編譯器知道裡面是什麼。
- `Option<u32>` 不是 `u32`，不能直接相加或使用（E0369），一定要先處理 `None` 的情況。
- 跟 Go 比：Go 的 `nil`、零值、`(值, ok)` 都靠自己記得檢查；Rust 把檢查放進型別，由編譯器強制。
