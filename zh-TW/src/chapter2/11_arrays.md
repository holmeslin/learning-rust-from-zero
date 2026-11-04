# 陣列 `[T; N]`

## 本集目標

建立固定長度的陣列，認識它的型別 `[T; N]`，並學會用 `{:?}` 把整個陣列印出來。

## 正文

### 建立陣列

陣列用中括號，裡面放好幾個值，值之間用逗號隔開。要把整個陣列印出來，格式寫 `{:?}`，這個寫法**先照抄**，原理第 8 章才會解釋：

```rust
fn main() {
    let scores = [85, 92, 78];
    println!("{:?}", scores);
}
```

執行結果：

```text
[85, 92, 78]
```

`{:?}` 跟 `{}` 一樣，可以把變數名稱直接寫進去：`{scores:?}`。

陣列裡的值，型別**必須全部一樣**，這點跟 tuple 不同。

### 型別 `[T; N]`

陣列的型別寫成 `[元素型別; 長度]`。三個 `i32` 的陣列是 `[i32; 3]`：

```rust
fn main() {
    let scores: [i32; 3] = [85, 92, 78];
    println!("{scores:?}");
}
```

執行結果：

```text
[85, 92, 78]
```

長度是型別的一部分，所以 `[i32; 3]` 和 `[i32; 4]` 是不同的型別。陣列建立之後長度就固定了，不能增加或刪除元素，這點跟 tuple 一樣。

### 重複同一個值

要建立「很多個一樣的值」，寫 `[值; 長度]`：

```rust
const SEATS: usize = 6;

fn main() {
    let seats = [false; SEATS];
    println!("{seats:?}");
}
```

執行結果：

```text
[false, false, false, false, false, false]
```

長度要寫在編譯時期就知道的數字，所以用第 1 集的 `const` 剛剛好。長度的型別是 `usize`，下一集會說明。如果長度是執行時才知道的變數，就不行：

```rust,compile_fail=E0435
fn main() {
    let n = 5;
    let seats = [false; n];
    println!("{seats:?}");
}
```

編譯錯誤：

```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:25
  |
3 |     let seats = [false; n];
  |                         ^ non-constant value
  |
help: consider using `const` instead of `let`
  |
2 -     let n = 5;
2 +     const n: /* Type */ = 5;
  |
```

`help` 建議改成 `const`，只有長度其實是寫死的數字時才用得上。長度要到執行時才知道的資料，要用第 7 章的 `Vec`。

### `{:?}` 也能印 tuple 和 `()`

第 7 集留了一個伏筆：`()` 不能用 `{}` 印，換成 `{:?}` 就可以。tuple 也一樣：

```rust
fn main() {
    let point = (3, 4);
    println!("{point:?}");
    println!("{:?}", ());
}
```

執行結果：

```text
(3, 4)
()
```

### 跟 Go 比一比

Go 的 `var a [3]int` 會自動填 0；Rust 沒有零值（第 1 章第 5 集），要自己寫 `[0; 3]`。

印出來的樣子也略有不同。Go 用空格隔開，Rust 的 `{:?}` 用逗號：

```go
a := [3]int{85, 92, 78}
fmt.Println(a) // [85 92 78]
```

Rust 印出來是 `[85, 92, 78]`。

另外，Go 日常大多用 slice，很少直接用陣列。Rust 的陣列也一樣，只有「長度真的固定」才用（例如一週七天）；要增減元素，用第 7 章的 `Vec`。

## 重點整理

- 陣列寫成 `[85, 92, 78]`，所有元素型別相同，長度固定。
- 型別是 `[元素型別; 長度]`，例如 `[i32; 3]`；長度是型別的一部分。
- `[值; 長度]` 建立重複的值，長度要是編譯時期已知的數字，可以用 `const`。
- `println!("{:?}", 陣列)` 先照抄，第 8 章解釋；也可以寫成 `{陣列名:?}`，tuple 和 `()` 也能這樣印。
- Rust 沒有零值，陣列要自己給初始值；會增減的資料用 `Vec`（第 7 章）。
