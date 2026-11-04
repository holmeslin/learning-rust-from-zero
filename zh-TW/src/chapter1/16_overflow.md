# 整數溢位

## 本集目標

知道整數算出來的結果超過型別範圍（溢位）時，Rust 在 debug 編譯下會 panic，不會像 Go 一樣悄悄繞回去。

## 正文

### 什麼是溢位

每個整數型別都有範圍，例如 `u8` 只能裝 0 到 255。算出來的結果超出範圍，就是**溢位**（overflow）。`255 + 1` 在 `u8` 裡放不下，Rust 要怎麼處理？

### 編譯器看得出來的溢位

如果編譯器在編譯時期就能算出會溢位，直接給編譯錯誤：

```rust,compile_fail
fn main() {
    let x: u8 = 255;
    let y = x + 1;
    println!("{y}");
}
```

編譯錯誤：

```text
error: this arithmetic operation will overflow
 --> src/main.rs:3:13
  |
3 |     let y = x + 1;
  |             ^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
```

### 執行時才發生的溢位

多數情況下，編譯器沒辦法事先知道數字會變多少，溢位要到執行時才會發生。下面這支程式每圈減 1，一路減到 0 之後再減 1，`u32` 裝不下 -1：

```rust,exit=101
fn main() {
    let mut n: u32 = 3;
    loop {
        println!("{n}");
        n -= 1;
    }
}
```

執行結果（某一次）：

```text
3
2
1
0

thread 'main' (155624576) panicked at src/main.rs:5:9:
attempt to subtract with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

程式印到 `0` 之後就 panic 了，結束碼是 101。`thread 'main'` 後面括號裡的數字每次執行都不一樣，其他都一樣。

這是 `cargo run` 預設的 debug 編譯行為：溢位會 panic，把 bug 當場抓出來。無號整數往下減過 0 是最常見的情況。

如果改用 `cargo run --release`（最佳化編譯，要追求速度時才用），為了不拖慢速度，溢位的檢查會被拿掉，結果會**繞回去**：`u32` 的 0 減 1 變成 4294967295。同一支程式在兩種編譯下行為不同，所以不要依賴繞回去的結果。

### 真的想要繞回去

需要繞回去的行為時（例如雜湊計算），要明確呼叫 `wrapping_add`、`wrapping_sub` 這類方法。方法第 5 章才會正式介紹，現在只要知道有這些名字，溢位不會悄悄發生，要繞回去得自己明講。

### 跟 Go 比一比

Go 的整數溢位**永遠**悄悄繞回去，不會報錯也不會 panic：

```go
var n uint8 = 255
n++
fmt.Println(n) // 0
```

從 Go 轉過來時，這是第一個要改的直覺：Rust 的 debug 編譯會把溢位當成 bug 抓出來。後面學到陣列索引時，會再遇到「`usize` 減到負數就 panic」的情況。

## 重點整理

- 整數結果超出型別範圍就是溢位。
- 編譯器看得出來的溢位是編譯錯誤；執行時才發生的，debug 編譯（`cargo run`）會 panic，結束碼是 101。
- release 編譯（`--release`）不檢查溢位，結果會繞回去，所以不要依賴繞回去的行為。
- 要繞回去，要明確呼叫 `wrapping_add` 這類方法（第 5 章介紹方法）。
- Go 的整數溢位永遠悄悄繞回去，Rust 的 debug 編譯會 panic。
