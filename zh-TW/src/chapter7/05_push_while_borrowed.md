# 持有參照時不能 `push`（E0502）

## 本集目標

知道為什麼手上還有元素的參照時，不能對 `Vec` 呼叫 `push`（E0502），以及怎麼改寫才合規。

## 正文

### 一個看起來沒問題的程式

先取出第一個元素的參照，再 `push`，最後用那個參照：

```rust,compile_fail=E0502
fn main() {
    let mut prices = vec![30, 45, 38];
    let first = &prices[0];
    prices.push(50);
    println!("第一杯 {first}");
}
```

編譯錯誤：

```text
error[E0502]: cannot borrow `prices` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let first = &prices[0];
  |                  ------ immutable borrow occurs here
4 |     prices.push(50);
  |     ^^^^^^^^^^^^^^^ mutable borrow occurs here
5 |     println!("第一杯 {first}");
  |                       ----- immutable borrow later used here
```

這就是第 4 章第 4 集的借用規則：`first` 是 `&`，`push` 需要 `&mut`，兩個同時存在就被擋下。第 4 章第 8 集的字串 slice 加 `push_str` 也是同一種錯誤。

### 為什麼要擋

`Vec` 的元素排在一整塊連續的記憶體裡。這塊記憶體滿了，`push` 就會要一塊更大的新記憶體、把所有元素搬過去，再釋放舊的那塊：

```text
push 之前：
  prices ---> [ 30 | 45 | 38 ]          （已經滿了）
  first  ---> 30

push 之後：
  prices ---> [ 30 | 45 | 38 | 50 | .. ]  （新的一塊）
  first  ---> 已經釋放的舊記憶體           （懸置參照）
```

如果編譯器放行，`first` 就會指向已經釋放的記憶體，也就是第 4 章第 6 集的懸置參照。所以規則是：只要還有人借著元素，就不能 `push`。這跟 `push` 這次有沒有真的搬家無關，編譯器看不出來，一律擋下。

### 兩種改法

參照在 `push` 之前用完（第 4 章第 5 集的 NLL），順序調換就行：

```rust
fn main() {
    let mut prices = vec![30, 45, 38];
    let first = &prices[0];
    println!("第一杯 {first}");
    prices.push(50);
    println!("{:?}", prices);
}
```

執行結果：

```text
第一杯 30
[30, 45, 38, 50]
```

或者根本不借，把值複製出來。`i32` 是 `Copy`，`prices[0]` 直接得到一個數字，跟 `Vec` 就沒關係了：

```rust
fn main() {
    let mut prices = vec![30, 45, 38];
    let first = prices[0];
    prices.push(50);
    println!("第一杯 {first}，共 {} 杯", prices.len());
}
```

執行結果：

```text
第一杯 30，共 4 杯
```

### 走訪的時候也不能 `push`

`for x in &v` 整個迴圈期間都借著 `v`，所以在迴圈裡 `push` 同樣是 E0502：

```rust,compile_fail=E0502
fn main() {
    let mut prices = vec![30, 45];
    for price in &prices {
        prices.push(*price + 10);
    }
    println!("{:?}", prices);
}
```

編譯錯誤：

```text
error[E0502]: cannot borrow `prices` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:9
  |
3 |       for price in &prices {
  |       -            -------
  |       |            |
  |       |            immutable borrow occurs here
  |  _____|            immutable borrow later used here
  | |
4 | |         prices.push(*price + 10);
  | |         ^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
5 | |     }
  | |_____- this for loop borrows `prices` immutably, preventing mutation within its body
  |
  = help: consider using an index-based loop instead, or collecting modifications into a separate collection
```

要在走訪的過程中增加元素，得另外準備一個 `Vec` 存新的，走訪完再處理。

### 跟 Go 比一比

Go 的 `append` 在容量不夠時一樣會搬家，但沒有人擋你：

```go
v := []int{30, 45, 38}
first := &v[0]
v = append(v, 50) // 容量不夠，v 搬到新的陣列
*first = 99       // 改到的是舊陣列
fmt.Println(v[0]) // 30
```

這段 Go 程式編譯成功，也不會 panic，只是 `first` 悄悄跟 `v` 脫鉤了，改了沒效果。Rust 把這種錯誤擋在編譯時期。

## 重點整理

- 手上還有元素的參照（`&v[0]`、走訪中的 `for x in &v`）時，不能對 `v` 呼叫 `push`，編譯錯誤 E0502。
- 原因：`push` 可能搬家，舊的參照會變成懸置參照；編譯器不看實際有沒有搬家，一律擋。
- 改法：參照用完再 `push`（NLL），或先把值複製出來（`Copy` 型別）。
- 在 `for x in &v` 裡不能 `push`；要增加元素，另外用一個 `Vec` 收。
- 跟 Go 比：Go 的 `append` 搬家之後，舊的指標悄悄脫鉤；Rust 在編譯時期擋下。
