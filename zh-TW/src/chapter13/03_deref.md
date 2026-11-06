# `Deref`

## 本集目標

知道 `*` 能用在 `Box` 上是因為 `Deref`，會替自己的型別實作 `Deref`，並看懂「`&String` 自動變 `&str`」的原理：解參照轉換（deref coercion）。

## 正文

### `*` 背後是 `Deref`

`Box` 能用 `*`，是因為它實作了標準函式庫的 `Deref` trait。自己的型別沒實作的話，`*` 就不能用：

```rust,compile_fail=E0614
struct Sleeve<T>(T);

fn main() {
    let number = Sleeve(5);
    println!("{}", *number);
}
```

編譯錯誤：

```text
error[E0614]: type `Sleeve<{integer}>` cannot be dereferenced
 --> src/main.rs:5:20
  |
5 |     println!("{}", *number);
  |                    ^^^^^^^ can't be dereferenced
```

`Sleeve` 是杯套，裡面裝著一個值。替它實作 `Deref`（要改動裡面的值，再加 `DerefMut`）：

```rust
use std::ops::{Deref, DerefMut};

struct Sleeve<T>(T);

impl<T> Deref for Sleeve<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Sleeve<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn show(name: &str) {
    println!("{name}（{} 位元組）", name.len());
}

fn main() {
    let mut stock = Sleeve(10);
    *stock -= 3;
    println!("{}", *stock);

    let name = Sleeve(String::from("紅茶"));
    println!("{}", name.len());
    show(&name);
}
```

執行結果：

```text
7
6
紅茶（6 位元組）
```

- `type Target = T;` 是第 8 章第 17 集的關聯型別，說明「解參照之後得到什麼型別」；`deref` 回傳內部值的參照 `&T`。
- `*stock` 其實是 `*(stock.deref())`：先呼叫 `deref` 拿到 `&i32`，再用第 4 章的 `*` 取出值。`deref` 回傳參照而不是值，所以 `*` 不會把裡面的值 move 走。
- 要寫入（`*stock -= 3`）就得有 `DerefMut`，`deref_mut` 回傳 `&mut T`。
- `name.len()`：`Sleeve<String>` 沒有 `len`，編譯器就 deref 成 `String` 再找。第 5 章第 10 集說 `.` 會自動補 `&`、`&mut`、`*`，補到對得上為止；有了 `Deref`，它還會逐層 deref 下去找。
- `show(&name)`：傳進去的是 `&Sleeve<String>`，函式要 `&str`，編譯器自動轉換，這就是下一節的主角。

### 解參照轉換

型別 `T` 實作了 `Deref<Target = U>`，編譯器在需要 `&U` 的地方，就會把 `&T` 自動轉成 `&U`，而且可以連續轉好幾次：`&Sleeve<String>` → `&String` → `&str`。這叫解參照轉換（deref coercion）。三種轉法：

- `&T` 轉成 `&U`：`T` 實作 `Deref<Target = U>`。
- `&mut T` 轉成 `&mut U`：`T` 實作 `DerefMut<Target = U>`。
- `&mut T` 轉成 `&U`：能寫的參照可以降成只能讀的，反過來不行。

前面幾章先照抄的寫法，原理全在這裡：

- `&String` 變 `&str`（第 4 章第 10 集）：`String` 實作了 `Deref<Target = str>`。
- `&Vec<T>` 變 `&[T]`（第 7 章第 6 集）：`Vec<T>` 實作了 `Deref<Target = [T]>`。
- `&Box<String>` 變 `&str`：`Box<T>` 的 `Target` 是 `T`，先變 `&String`，再變 `&str`。

方法也一樣。第 7 章第 10 集的 `trim`、`split`、`contains`，其實是 `str` 的方法，不是 `String` 的：

```rust,compile_fail=E0599
fn main() {
    let owned = String::from("  紅茶  ");
    println!("[{}]", String::trim(&owned));
}
```

編譯錯誤：

```text
error[E0599]: no associated function or constant named `trim` found for struct `String` in the current scope
 --> src/main.rs:3:30
  |
3 |     println!("[{}]", String::trim(&owned));
  |                              ^^^^ associated function or constant not found in `String`
  |
```

這裡只貼前半段，後面的 `note` 與 `help` 先不用管，`help` 寫著 `the function trim is implemented on str`。`String::trim` 找不到，`owned.trim()` 卻行得通：`.` 在 `String` 上找不到 `trim`，就 deref 成 `str` 再找。`Vec` 也是：`first`、`sort`、`contains` 是 slice 的方法，`Vec` 靠 `Deref` 借用。

解參照轉換只發生在「參照」上：`show(owned)` 傳的是 `String` 本身，是 E0308，要寫 `show(&owned)`。

### 跟 Go 比一比

Go 的 struct 嵌入（embedding）會把內層方法「提升」到外層，看起來很像 `Deref`：

```go
type Cup struct{ Name string }

func (c Cup) Label() string { return c.Name }

type Sleeve struct{ Cup } // Sleeve 也有 Label()
```

但 `Deref` 的設計是給智慧指標用的：「我是一個指標，指向另一個東西」。拿它來模擬繼承，讓 `Sleeve` 假裝是 `Cup`，是 Rust 社群不鼓勵的用法。想重用別的型別的方法，通常是欄位加上自己寫的方法，或是實作 trait。

## 重點整理

- `*` 靠 `Deref` trait：`*x` 就是 `*(x.deref())`；`type Target` 是解參照後的型別，`deref` 回傳 `&Target`。
- 要用 `*` 寫入，再實作 `DerefMut`（`deref_mut` 回傳 `&mut Target`）。
- 用 `.` 找不到方法時，編譯器會 deref 下去找：`String` 找得到 `str` 的方法，`Vec` 找得到 slice 的方法。
- 解參照轉換：需要 `&U` 的地方，`&T`（`T: Deref<Target = U>`）自動轉成 `&U`，可以連續轉，例如 `&Box<String>` → `&String` → `&str`；只對參照有效。前面照抄的 `&String` → `&str`、`&Vec<T>` → `&[T]` 都是它。
- 跟 Go 比：嵌入會提升方法，`Deref` 不是拿來模擬繼承的。
