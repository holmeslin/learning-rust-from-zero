# 閉包當參數

## 本集目標

寫出接收閉包的函式：用 `F: Fn(u32) -> u32` 這類 trait bound 收閉包，依函式怎麼用閉包選 `Fn`、`FnMut`、`FnOnce`，並知道函式名稱也能傳進去。

## 正文

### `F: Fn(u32) -> u32`

第 1 集說過，閉包的型別寫不出名字，所以接收閉包的函式用泛型加 trait bound（第 8 章第 5 集）。條件寫成 `Fn(參數型別) -> 回傳型別`，小括號、箭頭的格式跟函式型別一樣：

```rust
fn apply_discount<F: Fn(u32) -> u32>(price: u32, rule: F) -> u32 {
    rule(price)
}

fn main() {
    let off = 5;
    println!("{}", apply_discount(100, |p| p - off));
    println!("{}", apply_discount(100, |p| p * 9 / 10));
}
```

執行結果：

```text
95
90
```

`F` 讀成「某個閉包，能用一個 `u32` 呼叫、回傳 `u32`」。每次呼叫 `apply_discount`，傳進去的閉包不同，主體跟擷取的 `off` 也不同。`rule: impl Fn(u32) -> u32`（第 8 章第 4 集）、`where F: Fn(u32) -> u32`（第 6 集）的寫法，意思都一樣。

### 選哪一種：看函式怎麼用閉包

三個 trait 都能寫在這個位置，選擇的原則是：**函式要求越少越好**。要求越少，能接受的閉包就越多，這是上一集的包含關係。

| 函式裡怎麼用閉包 | 該寫 | 能接受的閉包 |
| --- | --- | --- |
| 只呼叫一次 | `FnOnce` | 全部 |
| 呼叫很多次，閉包可以改動東西 | `FnMut` | `FnMut` 與 `Fn` |
| 呼叫很多次，而且不允許改動 | `Fn` | 只有 `Fn` |

呼叫很多次又要改動狀態的例子：

```rust
fn repeat<F: FnMut()>(times: u32, mut action: F) {
    for _ in 0..times {
        action();
    }
}

fn main() {
    let mut served = 0;
    repeat(3, || served += 1);
    println!("出了 {served} 杯");
}
```

執行結果：

```text
出了 3 杯
```

`action` 是 `FnMut`，呼叫它要借 `&mut`，所以參數要寫 `mut action`（第 2 章第 3 集的 `mut` 參數）。就跟上一集的 `let mut inc` 同一個道理。

要求寫得比閉包做的事還嚴，編譯器就擋下來。下面的函式要求 `Fn`，卻傳了會改動 `count` 的閉包：

```rust,compile_fail=E0594
fn call_twice<F: Fn()>(action: F) {
    action();
    action();
}

fn main() {
    let mut count = 0;
    call_twice(|| count += 1);
    println!("{count}");
}
```

編譯錯誤：

```text
error[E0594]: cannot assign to `count`, as it is a captured variable in a `Fn` closure
 --> src/main.rs:8:19
  |
1 | fn call_twice<F: Fn()>(action: F) {
  |                                - change this to accept `FnMut` instead of `Fn`
...
7 |     let mut count = 0;
  |         --------- `count` declared here, outside the closure
8 |     call_twice(|| count += 1);
  |     ---------- -- ^^^^^^^^^^ cannot assign
  |     |          |
  |     |          in this closure
  |     expects `Fn` instead of `FnMut`
```

訊息說 `count` 是 `Fn` 閉包裡被擷取的變數，不能指定值，並且指向函式簽名：`change this to accept FnMut instead of Fn`。改成 `F: FnMut()`（參數也要加 `mut`）就好。

### 函式名稱也能傳

只要簽名對得上，一般的函式也能當成參數傳進去，不用包成閉包：

```rust
fn double(price: u32) -> u32 {
    price * 2
}

fn apply_discount<F: Fn(u32) -> u32>(price: u32, rule: F) -> u32 {
    rule(price)
}

fn main() {
    println!("{}", apply_discount(30, double));
}
```

執行結果：

```text
60
```

寫成 `|p| double(p)` 是多此一舉，clippy 會提醒你直接傳 `double`。

## 重點整理

- 接收閉包的函式用泛型加 bound：`F: Fn(u32) -> u32`；`impl Fn(u32) -> u32` 與 `where` 的寫法等價。
- 該寫哪一種 trait，看函式怎麼用閉包：只呼叫一次寫 `FnOnce`，呼叫很多次且要改動寫 `FnMut`（參數 `mut f: F`），呼叫很多次而且不允許改動寫 `Fn`。要求越少，能接受的閉包越多。
- 要求比閉包做的事嚴格，是編譯錯誤：把改動變數的閉包傳給 `Fn`，是 E0594，`help` 會建議改成 `FnMut`。
- 簽名吻合的函式名稱也能直接當閉包傳，不用再包一層 `|p| double(p)`。
