# `_` 與綁定變數

## 本集目標

用 `_` 或變數名稱接住「其餘全部」，並知道分支的順序，以及單獨寫的名字會帶來什麼問題。

## 正文

### 數字也要窮盡

整數的 `match` 同樣要窮盡，可是 `u32` 有四十幾億種值，不可能一個一個列：

```rust,compile_fail=E0004
fn window(n: u32) {
    match n {
        1 => println!("一號窗口：掛號"),
        2 => println!("二號窗口：批價"),
    }
}

fn main() {
    window(1);
}
```

編譯錯誤：

```text
error[E0004]: non-exhaustive patterns: `0_u32` and `3_u32..=u32::MAX` not covered
 --> src/main.rs:2:11
  |
2 |     match n {
  |           ^ patterns `0_u32` and `3_u32..=u32::MAX` not covered
  |
  = note: the matched value is of type `u32`
```

編譯器算得出漏掉的範圍：`0` 和 `3` 以上的所有數字，後面的 `help` 沒有貼出來。整數、字串字面值、`char` 都能直接寫成模式（像 `1`），但值太多，需要一個「其餘全部」的分支。

### `_` 與變數名稱

`_` 可以比對任何值，但不留下它，例如 `_ => println!("請洽櫃檯")`。第 2 章解構時看過這個 `_`：這個位置有值，但我不要。

想在這個分支裡用到那個值，就改寫成變數名稱：

```rust
fn window(n: u32) {
    match n {
        1 => println!("一號窗口：掛號"),
        2 => println!("二號窗口：批價"),
        other => println!("沒有 {other} 號窗口"),
    }
}

fn main() {
    window(1);
    window(7);
}
```

執行結果：

```text
一號窗口：掛號
沒有 7 號窗口
```

`other` 是你自己取的名字。它接住前面分支沒接住的所有值，並把值放進這個變數，效果就像 `let other = n;`，只能在這個分支裡用。名字取了卻沒用到，會有第 1 章第 19 集的未使用變數警告，這時候改寫 `_` 就好。

上一集的 `Discount::Percent(p)` 也是綁定變數，一樣的規則。`_` 也能放在別的模式裡，例如 `Discount::Percent(_)` 表示「不管折多少」。整數是 `Copy`，綁定之後原本的值照用；`String` 這類會 move 的型別要小心，第 7 集會遇到。

### 順序很重要

分支由上往下比，所以「其餘全部」一定要放最後。放在前面，後面的分支就永遠輪不到，編譯器會警告：

```rust,warn
fn window(n: u32) {
    match n {
        other => println!("{other} 號窗口"),
        1 => println!("一號窗口：掛號"),
    }
}

fn main() {
    window(1);
}
```

編譯警告：

```text
warning: unreachable pattern
 --> src/main.rs:4:9
  |
3 |         other => println!("{other} 號窗口"),
  |         ----- matches any value
4 |         1 => println!("一號窗口：掛號"),
  |         ^ no value can reach this
  |
  = note: `#[warn(unreachable_patterns)]` (part of `#[warn(unused)]`) on by default
```

### 單獨寫的名字是綁定

在 Go，`Pending` 單獨寫就能代表常數。Rust 的 variant 一定要加型別名稱，漏掉的話，`Paid` 會被當成一個綁定變數的名字，可以比對任何值：

```rust,compile_fail=E0170
enum Status {
    Pending,
    Paid,
}

fn describe(status: Status) {
    match status {
        Status::Pending => println!("等待付款"),
        Paid => println!("已付款"),
    }
}

fn main() {
    describe(Status::Pending);
    describe(Status::Paid);
}
```

編譯錯誤：

```text
error[E0170]: pattern binding `Paid` is named the same as one of the variants of the type `Status`
 --> src/main.rs:9:9
  |
9 |         Paid => println!("已付款"),
  |         ^^^^ help: to match on the variant, qualify the path: `Status::Paid`
  |
  = note: `#[deny(bindings_with_variant_name)]` on by default
```

這種寫法幾乎一定是筆誤，所以編譯器把它當成錯誤（E0170），`help` 直接告訴你要補上 `Status::`。後面還有幾則警告，都是這個筆誤的連鎖反應，不用理會。

## 重點整理

- 整數、字串等值很多的型別，`match` 一樣要窮盡，用 `_` 或變數名稱接住其餘全部。
- `_` 比對任何值但不建立變數；變數名稱（例如 `other`）比對任何值並把值放進變數，只能在該分支使用。
- 分支由上往下比，「其餘全部」要放最後，放前面會有 `unreachable pattern` 警告。
- `_` 也能用在別的模式裡面，例如 `Discount::Percent(_)`，略過 variant 裡的資料。
- variant 要寫成 `Status::Paid`；只寫 `Paid` 會變成綁定變數，是 E0170。
