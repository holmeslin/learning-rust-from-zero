# `Fn`、`FnMut`、`FnOnce`

## 本集目標

認得閉包的三種 trait：`Fn`、`FnMut`、`FnOnce`，知道編譯器怎麼依閉包主體決定它是哪一種，以及為什麼有的閉包要 `let mut`、有的只能呼叫一次。

## 正文

### 看主體怎麼用擷取到的東西

可以把閉包想成一個小 struct：欄位是它擷取的變數，呼叫它就像呼叫方法。第 5 章第 8 集學過方法的三種接收者，閉包也對應三種，編譯器依主體做了什麼，自動替閉包選一種：

| 主體對擷取到的東西 | trait | 呼叫時像 | 能呼叫幾次 |
| --- | --- | --- | --- |
| 只能讀 | `Fn` | `&self` | 很多次 |
| 改動 | `FnMut` | `&mut self` | 很多次 |
| 搬出去、用掉 | `FnOnce` | `self` | 一次 |

三種各一個：

```rust
fn main() {
    let tax = 5;
    let read = |price: u32| price + tax;

    let mut total = 0;
    let mut add = |price: u32| total += price;

    let name = String::from("小美");
    let give = || name;

    println!("{} {}", read(30), read(45));
    add(30);
    add(45);
    println!("共 {total}");
    let owner = give();
    println!("{owner}");
}
```

執行結果：

```text
35 50
共 75
小美
```

- `read` 只是讀 `tax`，是 `Fn`，想呼叫幾次都行。
- `add` 改動 `total`，是 `FnMut`。
- `give` 把 `name` 整個當回傳值交出去，是 `FnOnce`。

這些 trait 不用你寫在閉包上，編譯器自己推；要寫的地方在下一集，函式的參數與回傳型別。

### `FnMut`：要 `let mut`

呼叫 `FnMut` 閉包像呼叫 `&mut self` 的方法，所以閉包的變數本身也要是 `mut`，跟第 5 章第 8 集的 `&mut self` 方法一樣：

```rust,compile_fail=E0596
fn main() {
    let mut count = 0;
    let inc = || count += 1;
    inc();
    println!("{count}");
}
```

編譯錯誤：

```text
error[E0596]: cannot borrow `inc` as mutable, as it is not declared as mutable
 --> src/main.rs:4:5
  |
3 |     let inc = || count += 1;
  |                  ----- calling `inc` requires mutable binding due to mutable borrow of `count`
4 |     inc();
  |     ^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
3 |     let mut inc = || count += 1;
  |         +++
```

訊息說呼叫 `inc` 需要可變的綁定，因為它可變借用了 `count`；`help` 建議把 `inc` 改成 `let mut`。上一集的 `tick` 也是這個原因。

### `FnOnce`：只能呼叫一次

呼叫 `FnOnce` 閉包像呼叫 `self` 的方法，會把閉包整個用掉。再呼叫第二次，就是用了已經被 move 的值：

```rust,compile_fail=E0382
fn main() {
    let name = String::from("小美");
    let give = || name;
    println!("{}", give());
    println!("{}", give());
}
```

編譯錯誤：

```text
error[E0382]: use of moved value: `give`
 --> src/main.rs:5:20
  |
4 |     println!("{}", give());
  |                    ------ `give` moved due to this call
5 |     println!("{}", give());
  |                    ^^^^ value used here after move
  |
note: closure cannot be invoked more than once because it moves the variable `name` out of its environment
 --> src/main.rs:3:19
  |
3 |     let give = || name;
  |                   ^^^^
note: this value implements `FnOnce`, which causes it to be moved when called
 --> src/main.rs:4:20
  |
4 |     println!("{}", give());
  |                    ^^^^
```

`give` 第一次呼叫就把 `name` 交出去了，第二次已經沒東西可交。訊息把原因寫得很清楚：`closure cannot be invoked more than once because it moves the variable name out of its environment`，後面一則 `note` 說 `give` 實作的是 `FnOnce`，所以呼叫會把它 move 掉。

### 三種的關係

- 所有閉包都是 `FnOnce`：誰都能呼叫一次。
- 不搬走東西的閉包，也是 `FnMut`。
- 連改動都沒有的閉包，三種都是，所以 `Fn` 的閉包最「乖」，能用在最多地方。

這個關係在下一集很重要：函式要求「`FnOnce`」，所有閉包都能傳進去；要求「`Fn`」，只有不改動東西的閉包能傳。

另外，`move` 跟這三種是兩回事。`move` 決定「怎麼擷取」（拿走變數），三種 trait 看的是「主體怎麼用」：上一集 `move || println!("你好，{name}")` 只是讀 `name`，仍然是 `Fn`，呼叫幾次都行。

## 重點整理

- 閉包依主體怎麼用擷取的東西，自動屬於三種 trait 之一：只是讀的是 `Fn`、改動的是 `FnMut`、搬出去用掉的是 `FnOnce`。
- 可以想成呼叫時分別借 `&self`、`&mut self`、拿走 `self`：`Fn` 與 `FnMut` 能呼叫很多次，`FnOnce` 只能一次。
- `FnMut` 閉包的變數要 `let mut`，否則是 E0596；`FnOnce` 閉包呼叫第二次是 E0382（`closure cannot be invoked more than once`）。
- 包含關係：`Fn` 的閉包也是 `FnMut`、`FnOnce`；`FnMut` 的也是 `FnOnce`。
- `move` 只決定怎麼擷取，不決定是哪一種 trait。
