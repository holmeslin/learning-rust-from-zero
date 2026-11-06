# `const fn`

## 本集目標

會寫 `const fn`，讓函式在編譯時期執行，用來算 `const`、`static` 的初始值與陣列長度，也能在編譯時期檢查設定值；並知道 `const fn` 裡有哪些事還不能做。

## 正文

### 編譯時期執行的函式

第 2 章的 `const` 只能放字面值和簡單的運算。想用函式算出來，要把函式宣告成 `const fn`：

```rust
const fn tray_price(cups: u32) -> u32 {
    cups * 30
}

const FULL_TRAY: u32 = tray_price(6);

fn main() {
    println!("{FULL_TRAY}");
    let cups = 3;
    println!("{}", tray_price(cups));
}
```

執行結果：

```text
180
90
```

- `FULL_TRAY` 的值是編譯器在編譯時期呼叫 `tray_price(6)` 算出來的，程式跑起來的時候，它就是常數 180，沒有計算。
- `const fn` 並不是「只能在編譯時期」的函式。同一個函式照樣可以在執行時呼叫，像 `tray_price(cups)`，`cups` 是執行時才有的值。

`const fn` 算出來的值可以用在需要編譯時期常數的地方，例如陣列的長度（附錄一 h 說過，陣列長度不能用普通變數）：`const SLOTS: usize = slots_for(10);` 之後，`[0u32; SLOTS]` 就是合法的陣列型別。

### 沒有標 `const` 的函式不能呼叫

在 `const` 的初始值裡，只能呼叫 `const fn`。普通函式不行，是 E0015：

```rust,compile_fail=E0015
fn price_of(cups: u32) -> u32 {
    cups * 30
}

const FULL_TRAY: u32 = price_of(6);

fn main() {
    println!("{FULL_TRAY}");
}
```

編譯錯誤：

```text
error[E0015]: cannot call non-const function `price_of` in constants
 --> src/main.rs:5:24
  |
5 | const FULL_TRAY: u32 = price_of(6);
  |                        ^^^^^^^^^^^
  |
note: function `price_of` is not const
 --> src/main.rs:1:1
  |
1 | fn price_of(cups: u32) -> u32 {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: calls in constants are limited to constant functions, tuple structs and tuple variants
```

訊息很明白：`price_of` 不是 const。要解決，就是在它前面加上 `const`。標準函式庫裡有不少 `const fn`：`String::new()`、`Vec::new()`、`Mutex::new(...)`、`AtomicU32::new(...)`，所以第 14 章第 13 集的 `static ORDERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());` 才能成立；`HashMap::new()` 不是 `const fn`，所以那一集說要改用 `LazyLock`。

### 在編譯時期檢查

`const fn` 裡面如果 panic，就會變成編譯錯誤，因為這個 panic 發生在編譯時期。利用這一點，可以把「不可能出錯的前提」寫成編譯時期的檢查：

```rust,compile_fail=E0080
const fn tray_price(cups: u32) -> u32 {
    cups * 30
}

const _: () = assert!(tray_price(6) == 100);

fn main() {
    println!("{}", tray_price(6));
}
```

編譯錯誤：

```text
error[E0080]: evaluation panicked: assertion failed: tray_price(6) == 100
 --> src/main.rs:5:15
  |
5 | const _: () = assert!(tray_price(6) == 100);
  |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `_` failed here
```

`const _: () = ...;` 是一個沒有名字的 `const`，不用拿來做什麼，只是為了讓編譯器去算右邊的運算式。算不出來就是 E0080（`evaluation panicked`）。如果把 `100` 改成 `180`，檢查通過，編譯器什麼也不會說，程式照常編譯。

### `const fn` 的限制

`const fn` 裡不能呼叫沒有標成 `const` 的函式，也不能做堆積配置：`String::from("紅茶")` 不是 `const fn`，在 `const fn` 裡呼叫是 E0015；`String::new()` 就可以。trait 的方法目前也不能在 `const fn` 裡呼叫，`for` 迴圈背後要呼叫 `Iterator` 的方法，所以 `for i in 1..=n` 也是 E0015（``cannot use `for` loop ... in constant functions``）。要重複就用 `while`：

```rust
const fn sum_to(n: u32) -> u32 {
    let mut i = 1;
    let mut sum = 0;
    while i <= n {
        sum += i;
        i += 1;
    }
    sum
}

const TOTAL: u32 = sum_to(10);

fn main() {
    println!("{TOTAL}");
}
```

執行結果：

```text
55
```

`if`、`match`、`while`、`let mut`、算術都能用；這已經足夠算出查表用的資料或檢查設定值。

### 跟 Go 比一比

Go 的 `const` 只能是字面值、其他常數，和少數內建函式（像 `len`）的運算。呼叫自己寫的函式不行：

```go
const fullTray = trayPrice(6)
```

編譯錯誤是 `trayPrice(6) (value of type int) is not constant`。Go 想在程式一開始算出值，只能用 `var fullTray = trayPrice(6)`，由執行時期的初始化去算，不是常數。Rust 的 `const fn` 讓一般的函式在編譯時期執行，算出來的值直接寫進執行檔。

## 重點整理

- `const fn` 讓函式能在編譯時期執行：算 `const`、`static` 的初始值、陣列長度。同一個函式在執行時照樣能呼叫。
- `const` 的初始值裡只能呼叫 `const fn`，普通函式是 E0015；`String::new()`、`Vec::new()`、`Mutex::new(...)` 是 `const fn`，`HashMap::new()`、`String::from(...)` 不是。
- `const fn` 裡 panic 會變成編譯錯誤 E0080；`const _: () = assert!(條件);` 可以當編譯時期的檢查。
- `const fn` 目前不能堆積配置、不能呼叫 trait 的方法（所以沒有 `for`，要用 `while`）；`if`、`match`、`while`、`let mut`、算術可以。
- Go 的 `const` 不能呼叫自己寫的函式；Rust 的 `const fn` 在編譯時期執行。
