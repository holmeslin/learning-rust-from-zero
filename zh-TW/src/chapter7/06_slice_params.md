# 參數用 `&[T]`

## 本集目標

知道只需要讀 `Vec` 的函式，參數要寫 `&[T]`：`Vec`、陣列、`Vec` 的一段都能傳進去；要增減元素才寫 `&mut Vec<T>`。

## 正文

### `&[T]` 什麼都收

第 4 章第 11 集用 `&[T]` 接陣列。`Vec` 也一樣，傳 `&v` 就行：

```rust
fn total(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}

fn main() {
    let v = vec![30, 45, 38];
    let a = [10, 20];
    println!("{}", total(&v));
    println!("{}", total(&a));
    println!("{}", total(&v[1..]));
}
```

執行結果：

```text
113
30
83
```

- `total(&v)`：`&Vec<i32>` 在需要 `&[i32]` 的地方，編譯器會自動轉換。原理第 13 章第 3 集的 `Deref` 才解釋，跟第 4 章第 10 集 `&String` 變 `&str` 是同一類事，現在照寫就好。
- `total(&a)`、`total(&v[1..])`：陣列的參照、`Vec` 的後兩個元素也行。
- 函式裡的 `for num in nums` 是上一集 `for x in &v` 的同一種寫法，`nums` 本來就是參照，不用再加 `&`。

### 為什麼不寫 `&Vec<i32>`

參數寫成 `&Vec<i32>` 的話，只收得下 `Vec`，陣列和 slice 都傳不進去：

```rust,compile_fail=E0308
fn total(nums: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in nums {
        sum += num;
    }
    sum
}

fn main() {
    let a = [10, 20];
    println!("{}", total(&a));
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
  --> src/main.rs:11:26
   |
11 |     println!("{}", total(&a));
   |                    ----- ^^ expected `&Vec<i32>`, found `&[{integer}; 2]`
   |                    |
   |                    arguments to this function are incorrect
   |
   = note: expected reference `&Vec<i32>`
              found reference `&[{integer}; 2]`
note: function defined here
  --> src/main.rs:1:4
   |
 1 | fn total(nums: &Vec<i32>) -> i32 {
   |    ^^^^^ ---------------
```

跟第 4 章第 10 集的 `&String` 與 `&str` 同一個道理：參數寫 `&[T]`，能收的比 `&Vec<T>` 多，函式內容完全不用改。

### 要增減元素：`&mut Vec<T>`

`&mut [T]` 能改元素，但 slice 的長度固定，不能 `push`。函式要增減元素，得收 `&mut Vec<T>`：

```rust
fn add_order(orders: &mut Vec<i32>, price: i32) {
    orders.push(price);
}

fn main() {
    let mut orders = vec![30, 45];
    add_order(&mut orders, 38);
    println!("{:?}", orders);
}
```

執行結果：

```text
[30, 45, 38]
```

選參數型別，整理成一張表：

| 函式對 `Vec` 要做的事 | 參數型別 |
| --- | --- |
| 只要讀 | `&[T]` |
| 要改元素，不增減 | `&mut [T]` |
| 要增減元素 | `&mut Vec<T>` |
| 要拿走所有權 | `Vec<T>` |

### 搭配 slice pattern

第 6 章第 13 集留了一個伏筆：slice pattern 配上 `&[T]` 參數，函式就能處理任意長度的 `Vec`：

```rust
fn describe(nums: &[i32]) {
    match nums {
        [] => println!("沒有訂單"),
        [only] => println!("只有一筆：{only}"),
        [first, .., last] => println!("第一筆 {first}，最後一筆 {last}"),
    }
}

fn main() {
    let mut orders: Vec<i32> = Vec::new();
    describe(&orders);
    orders.push(30);
    describe(&orders);
    orders.push(45);
    orders.push(38);
    describe(&orders);
}
```

執行結果：

```text
沒有訂單
只有一筆：30
第一筆 30，最後一筆 38
```

## 重點整理

- 只需要讀的 `Vec` 參數寫 `&[T]`：`&Vec`、`&陣列`、`&v[a..b]` 都能傳；`&Vec<T>` 只收 `Vec`（陣列傳進去是 E0308）。
- `&Vec<T>` 會自動轉成 `&[T]`，原理第 13 章第 3 集解釋。
- 要改元素、不增減，用 `&mut [T]`；要 `push` 或 `pop`，用 `&mut Vec<T>`；要拿走所有權，用 `Vec<T>`。
- `&[T]` 可以搭配 slice pattern，用 `match` 依長度分情況處理。
