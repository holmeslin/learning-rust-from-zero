# `Copy` 型別

## 本集目標

知道整數、`bool`、`char` 這類型別指定給另一個變數時是複製、不是 move，並分得出哪些型別是 `Copy`、哪些不是。

## 正文

### 整數不會 move

前幾集的 move 都發生在 `String` 身上。換成整數試試：

```rust
fn main() {
    let a = 5;
    let b = a;
    println!("{a} {b}");
}
```

執行結果：

```text
5 5
```

`a` 和 `b` 都能用。整數整個值就放在 stack 上，沒有 heap 資料要還，複製一份又快又沒有風險，所以 Rust 在 `let b = a;` 時直接複製。這樣的型別叫 **`Copy` 型別**。`Copy` 其實是一個 trait，第 8 章才會學，現在只要知道哪些型別有它。

### 哪些是 `Copy`

- 所有整數、浮點數，加上 `bool` 和 `char`。
- 元素全都是 `Copy` 的 tuple 和陣列，例如 `(i32, i32)`、`[i32; 3]`。
- 字串字面值（型別是 `&str`，第 4 章介紹）。

```rust
fn main() {
    let point = (3, 4);
    let scores = [85, 92, 78];
    let name = "小明";

    let point2 = point;
    let scores2 = scores;
    let name2 = name;

    println!("{point:?} {point2:?}");
    println!("{scores:?} {scores2:?}");
    println!("{name} {name2}");
}
```

執行結果：

```text
(3, 4) (3, 4)
[85, 92, 78] [85, 92, 78]
小明 小明
```

指定之後，原本的變數都還能用。

### 不是 `Copy` 的

`String` 不是 `Copy`：它擁有 heap 上的資料，複製欄位會讓兩個變數指到同一塊記憶體，所以只能 move。判斷的原則是：**只要型別擁有 heap 資料，就不是 `Copy`**。

這個原則也套用在 tuple 和陣列上，裡面只要有一個元素不是 `Copy`，整個 tuple 或陣列就不是：

```rust,compile_fail=E0382
fn main() {
    let order = (3, String::from("珍珠奶茶"));
    let backup = order;
    println!("{} {}", order.0, backup.0);
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `order`
 --> src/main.rs:4:23
  |
2 |     let order = (3, String::from("珍珠奶茶"));
  |         ----- move occurs because `order` has type `(i32, String)`, which does not implement the `Copy` trait
3 |     let backup = order;
  |                  ----- value moved here
4 |     println!("{} {}", order.0, backup.0);
  |                       ^^^^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let backup = order.clone();
  |                       ++++++++
```

`order` 的型別是 `(i32, String)`，因為有 `String`，整個 tuple 不是 `Copy`，`let backup = order;` 就變成 move。把 `String::from("珍珠奶茶")` 換成字串字面值 `"珍珠奶茶"`，tuple 就是 `Copy` 了，這段程式也就能編譯。

自己定義的 `struct` 預設也不是 `Copy`，第 8 章會說明怎麼讓它變成 `Copy`。

### 跟 Go 比一比

Go 的 `=` 永遠是複製：整數、陣列、struct 都整個複製一份。（slice、map、指標複製的是表頭或位址，底下的資料共用。）

Rust 的 `=` 要看型別。陣列 `[i32; 3]` 跟 Go 的 `[3]int` 一樣，整個複製：

```rust
fn main() {
    let a = [85, 92, 78];
    let mut b = a;
    b[0] = 0;
    println!("{a:?} {b:?}");
}
```

執行結果：

```text
[85, 92, 78] [0, 92, 78]
```

但換成 `[String; 2]`，同樣的 `let b = a;` 就是 move。所以在 Rust 看到 `=`，要想一下這個型別是不是 `Copy`：是就複製，不是就 move。

## 重點整理

- 整數、浮點數、`bool`、`char` 是 `Copy` 型別：指定給別的變數時複製一份，原本的變數還能用。
- 元素都是 `Copy` 的 tuple 和陣列也是 `Copy`；只要有一個 `String`，整體就不是。
- 字串字面值（`&str`）是 `Copy`；`String` 不是，`=` 會 move。
- 判斷依據是型別：擁有 heap 資料的型別不能是 `Copy`。
- Go 的 `=` 一律複製；Rust 的 `=` 對 `Copy` 型別複製，其他型別 move。
