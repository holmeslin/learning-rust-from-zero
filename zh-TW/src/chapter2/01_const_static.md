# `const` 與 `static`

## 本集目標

學會用 `const` 與 `static` 定義寫在函式外面的常數，並知道 Rust 要求你明確標上型別。

## 正文

### `const`：永遠不變的值

`let` 建立的變數只能放在函式裡面。想要一個整支程式都能用、而且永遠不會變的值，就用 `const`，寫在 `fn main` 的外面：

```rust
const PASS_SCORE: i32 = 60;

fn main() {
    let score = 75;
    if score >= PASS_SCORE {
        println!("及格");
    } else {
        println!("不及格");
    }
}
```

執行結果：

```text
及格
```

`const` 有幾條規則：

- 一定要標型別。`PASS_SCORE: i32` 的 `: i32` 不能省，編譯器不會替你推斷。
- 名字慣例用全大寫、單字之間加底線。寫成小寫，編譯器會警告。
- 沒有 `mut`。`const` 永遠不能改。
- 值必須在編譯時期就算得出來。`24 * 60 * 60` 這種運算式可以，執行時才知道的值（例如讀進來的輸入）不行。

```rust
const SECONDS_PER_DAY: u32 = 24 * 60 * 60;

fn main() {
    println!("一天有 {SECONDS_PER_DAY} 秒");
    let days = 3;
    println!("{days} 天有 {} 秒", days * SECONDS_PER_DAY);
}
```

執行結果：

```text
一天有 86400 秒
3 天有 259200 秒
```

`const` 也可以寫在函式裡面，那樣就只有那個函式看得到。

忘了標型別，會得到編譯錯誤：

```rust,compile_fail
const LIMIT = 10;

fn main() {
    println!("{LIMIT}");
}
```

編譯錯誤：

```text
error: missing type for `const` item
 --> src/main.rs:1:12
  |
1 | const LIMIT = 10;
  |            ^ help: provide a type for the constant: `: i32`
```

編譯器連該補什麼型別都幫你猜好了，照著補上 `: i32` 就行。

### `static`：固定在記憶體裡的一個位置

`static` 的寫法跟 `const` 幾乎一樣：

```rust
static MAX_SEATS: u32 = 40;

fn main() {
    println!("最多 {MAX_SEATS} 個座位");
}
```

執行結果：

```text
最多 40 個座位
```

差別在記憶體裡的樣子：`const` 的值會被複製到每一個用到它的地方；`static` 則是整支程式只有一份，放在固定的位置，從程式開始到結束都在。對 `i32`、`u32` 這種小數字，兩者看不出差別，所以**先一律用 `const`**，看到 `static` 時認得它就好。

這一集的 `static` 是不可變的，想改它會得到編譯錯誤：

```rust,compile_fail=E0594
static MAX_SEATS: u32 = 40;

fn main() {
    MAX_SEATS = 50;
    println!("{MAX_SEATS}");
}
```

編譯錯誤：

```text
error[E0594]: cannot assign to immutable static item `MAX_SEATS`
 --> src/main.rs:4:5
  |
1 | static MAX_SEATS: u32 = 40;
  | --------------------- this `static` cannot be written to
...
4 |     MAX_SEATS = 50;
  |     ^^^^^^^^^^^^^^ cannot assign
```

想要「可以隨時修改的全域變數」，Rust 沒有一個簡單的寫法，因為多個執行緒同時改它會出問題。真的需要時要用比較安全的工具，第 14 章會介紹。

### 跟 Go 比一比

Go 的 `const` 可以不標型別，數字常數用在哪裡，就自動變成那裡需要的型別。Go 的套件層級 `var` 也可以直接改：

```go
const PassScore = 60 // 沒標型別，可以當 int 也可以當 float64
var counter = 0      // 套件層級的變數，任何函式都能改
```

Rust 兩個直覺都不成立：`const` 一定要標型別，也不會自動變成別的型別（第 1 章第 15 集的 `let x: f64 = 5;` 同樣會出錯）；全域的值預設改不了，沒有 Go 那種隨手改的 `var counter`。

## 重點整理

- `const NAME: 型別 = 值;` 寫在函式外面，永遠不變，一定要標型別，名字用全大寫加底線。
- `const` 的值必須在編譯時期算得出來，可以是運算式。
- `static` 寫法相同，整支程式只有一份；本書先用 `const`，`static` 認得就好。
- 不可變的 `static` 不能賦值（`E0594`）；可以修改的全域變數留到第 14 章。
- Go 的 `const` 可以不標型別、套件層級 `var` 可以改；Rust 都不行。
