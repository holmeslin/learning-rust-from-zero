# 陣列、slice 與 `Vec` 的差別

## 本集目標

把陣列 `[T; N]`、slice `&[T]` 與 `Vec<T>` 並排比較：誰擁有資料、長度何時決定、能不能增減，並學會三者之間的轉換與選擇。

## 正文

### 三種型別並排

| | 陣列 `[T; N]` | `Vec<T>` | slice `&[T]`、`&mut [T]` |
| --- | --- | --- | --- |
| 擁有資料嗎 | 擁有，元素直接放在變數裡 | 擁有，元素放在 heap | 不擁有，借別人的一段 |
| 長度 | 型別的一部分，編譯時期固定 | 執行時期決定，可變 | 借的那一段，執行時期才知道 |
| 增減元素 | 不行 | 行（`push`、`pop`） | 不行（`&mut [T]` 能改元素） |
| `let b = a;` | 元素是 `Copy` 就複製 | move | 複製的只是參照 |

三者的關係：`Vec` 是擁有者，slice 是借來的一段，陣列是長度固定的擁有者。slice 可以借自陣列，也可以借自 `Vec`。

### 長度什麼時候決定

陣列的長度是型別的一部分，要在編譯時期就知道。長度放在變數裡，編譯器不接受：

```rust,compile_fail=E0435
fn main() {
    let n = 3;
    let a = [0; n];
    println!("{:?}", a);
}
```

編譯錯誤：

```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:17
  |
3 |     let a = [0; n];
  |                 ^ non-constant value
  |
help: consider using `const` instead of `let`
  |
2 -     let n = 3;
2 +     const n: /* Type */ = 3;
  |
```

`help` 建議改成 `const`（第 2 章第 1 集），長度寫死就是陣列。長度要到執行時期才知道，就用 `Vec`：`vec![0; n]` 的 `n` 可以是變數。

### 互相轉換

```rust
fn main() {
    let array = [30, 45, 38];
    let mut vec = Vec::from(array);
    vec.push(50);
    let slice = &vec[1..3];
    let copy = slice.to_vec();
    println!("{array:?} {vec:?} {copy:?}");
}
```

執行結果：

```text
[30, 45, 38] [30, 45, 38, 50] [45, 38]
```

- 陣列變 `Vec`：`Vec::from(array)`，用的是第 8 章第 13 集的 `From`。元素是複製進去的，原本的陣列還能用。
- slice 變 `Vec`：`slice.to_vec()`，複製 slice 裡的元素，得到新的、擁有資料的 `Vec`。
- 陣列或 `Vec` 變 slice：不用轉換，`&array`、`&vec[..]` 就是（第 7 章第 6 集）。slice 只是借著，所以不能增減，也不能比原本的資料活得久。

### 該用哪一個

- 長度固定、而且很小（一週七天、座標的兩個值）：陣列。
- 長度執行時才知道，或要增減元素：`Vec`。
- 函式只需要讀或改元素：參數寫 `&[T]`、`&mut [T]`，陣列、`Vec` 都能傳進去（第 7 章第 6 集）；要增減元素才寫 `&mut Vec<T>`。
- 函式要交出一批新資料：回傳 `Vec<T>`。

### 跟 Go 比一比

Go 的 `[3]int` 是陣列，跟 Rust 一樣是值，指定給別的變數會複製，長度也是型別的一部分。差別在 `[]int`：它一個型別身兼 Rust 的兩個角色。

```text
Go 的 []int     [指標 | 長度 | 容量] ---> 底層陣列
                擁有者和「一段資料」是同一個東西，append 可能換掉底層陣列，
                舊的 slice 卻還指著舊的

Rust 的 Vec<T>  [指標 | 長度 | 容量] ---> heap 上的元素（擁有者）
Rust 的 &[T]    [指標 | 長度]        ---> 借來的一段（不能增減）
```

Rust 把「擁有」和「借一段來看」拆成兩個型別，借用規則（第 4 章）保證：有 slice 借著的時候，`Vec` 不能 `push`（第 7 章第 5 集的 E0502），不會出現 Go 那種「append 之後舊 slice 還指著舊資料」的情況。

## 重點整理

- 陣列 `[T; N]`：長度是型別的一部分，編譯時期固定，元素直接放在變數裡；元素是 `Copy` 時，整個陣列是 `Copy`。
- `Vec<T>`：擁有元素，元素在 heap，長度執行時期決定、能增減；指定給別的變數會 move。
- slice `&[T]`、`&mut [T]`：借來的一段，不擁有、不能增減；陣列和 `Vec` 都能借出 slice。
- 陣列長度要是常數（E0435）；長度到執行時期才知道，用 `Vec`。
- 轉換：陣列變 `Vec` 用 `Vec::from(array)`，slice 變 `Vec` 用 `.to_vec()`，變成 slice 只要 `&`。
- 函式只是要讀或改元素，參數用 slice；Go 的 `[]int` 同時是擁有者和一段資料，Rust 拆成 `Vec` 與 `&[T]`。
