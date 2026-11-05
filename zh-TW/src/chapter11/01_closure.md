# 閉包

## 本集目標

寫出 `|參數| 運算式` 這種閉包（closure），存進變數、像函式一樣呼叫，並知道參數與回傳型別通常不用標。

## 正文

### 沒有名字的函式

飲料店每杯外送加 5 元。第 2 章的函式這樣寫：

```rust
fn add_fee(price: u32) -> u32 {
    price + 5
}

fn main() {
    println!("{}", add_fee(30));
}
```

執行結果：

```text
35
```

閉包是另一種寫法（Go 的 `func(price int) int { ... }` 匿名函式就是同一類東西）：沒有 `fn` 和名字，參數用一對 `|` 圈起來，後面接運算式，整個存進變數：

```rust
fn main() {
    let add_fee = |price: u32| price + 5;
    println!("{}", add_fee(30));
    println!("{}", add_fee(45));
}
```

執行結果：

```text
35
50
```

- `|price: u32|` 是參數，型別標在冒號後面，跟函式一樣。
- `price + 5` 是運算式，它的值就是回傳值，不用 `return`，也不用寫 `-> u32`。
- `add_fee` 是普通的變數，裡面放的是閉包；呼叫時寫 `add_fee(30)`，跟呼叫函式一模一樣。

### 型別通常不用標

函式的參數一定要標型別；閉包的參數和回傳值，編譯器看使用的方式就推得出來，所以可以省略：

```rust
fn main() {
    let add_fee = |price| price + 5;
    let price: u32 = 30;
    println!("{}", add_fee(price));
}
```

執行結果：

```text
35
```

`add_fee` 的 `price` 沒標型別，編譯器從 `add_fee(price)` 傳進去的是 `u32`，推出閉包的參數與回傳值都是 `u32`。

不標型別不代表「什麼型別都收」。型別一旦被第一次呼叫決定，就固定了：

```rust,compile_fail=E0308
fn main() {
    let show = |price| println!("{price} 元");
    show(30);
    show("三十");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:4:10
  |
4 |     show("三十");
  |     ---- ^^^^^^ expected integer, found `&str`
  |     |
  |     arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `{integer}`
 --> src/main.rs:3:10
  |
3 |     show(30);
  |     ---- ^^ expected because this argument is of type `{integer}`
  |     |
  |     in this closure call
note: closure parameter defined here
 --> src/main.rs:2:17
  |
2 |     let show = |price| println!("{price} 元");
  |                 ^^^^^
```

編譯器第一次看到 `show(30)`，就把 `price` 定成整數；第二次傳 `&str` 就對不上。`note` 還特地指出「閉包稍早是用 `{integer}` 呼叫的」。想要同一段邏輯處理不同型別，要用第 8 章的泛型函式，閉包沒有這種彈性。

### 主體不只一行

主體要放好幾行，就用 `{ }` 包起來，跟函式的區塊一樣，最後一個運算式是回傳值。沒有參數就寫空的 `||`：

```rust
fn main() {
    let welcome = || println!("歡迎光臨");
    let checkout = |price: u32| {
        let fee = 5;
        println!("餐點 {price} 元，外送費 {fee} 元");
        price + fee
    };
    welcome();
    let total = checkout(30);
    println!("共 {total} 元");
}
```

執行結果：

```text
歡迎光臨
餐點 30 元，外送費 5 元
共 35 元
```

### 閉包不是函式

閉包用起來像函式，但它的型別不一樣：每個閉包都有自己獨一無二、寫不出名字的型別，所以不能像 `fn add_fee(u32) -> u32` 那樣直接在簽名裡寫「某個閉包」。要把閉包傳進函式、從函式回傳，得靠 trait：第 4 集認識 `Fn`、`FnMut`、`FnOnce`，第 5、6 集再用它們。

閉包真正跟函式不同的地方，是它可以直接用外面的變數，下一集就來看。

## 重點整理

- 閉包寫成 `|參數| 運算式`，存進變數，用 `名字(引數)` 呼叫；主體有好幾行就用 `{ }`，沒有參數寫 `||`。
- 參數與回傳型別可以不標，編譯器從使用方式推出來；型別被第一次呼叫決定後就固定，換一種型別呼叫是 E0308。
- 每個閉包都有自己的型別，寫不出名字；傳進函式、從函式回傳要靠 `Fn` 系列的 trait（第 4～6 集）。
- 閉包可以用外面的變數，下一集說明。
