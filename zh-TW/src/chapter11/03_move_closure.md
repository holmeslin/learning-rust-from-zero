# `move` 閉包

## 本集目標

用 `move` 讓閉包拿走它用到的變數，知道這對 `String` 是 move、對整數是複製，並認得什麼時候非用 `move` 不可。

## 正文

### `move`：整個拿走

在閉包前面加 `move`，不管主體怎麼用，擷取的變數一律拿走所有權。下面的 `name` 搬進了 `greet`，閉包現在是它的擁有者。第 3 章的 move 規則照舊：搬走之後，原本的變數不能再用：

```rust,compile_fail=E0382
fn main() {
    let name = String::from("小美");
    let greet = move || println!("你好，{name}");
    greet();
    println!("{name}");
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `name`
 --> src/main.rs:5:16
  |
2 |     let name = String::from("小美");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
3 |     let greet = move || println!("你好，{name}");
  |                 -------                  ---- variable moved due to use in closure
  |                 |
  |                 value moved into closure here
4 |     greet();
5 |     println!("{name}");
  |                ^^^^ value borrowed here after move
```

訊息跟第 3 章第 6 集的 E0382 很像，多了兩處標記：`value moved into closure here` 指出閉包，`variable moved due to use in closure` 指出是哪個變數。這裡只貼前半段，後面的 `help` 建議「先 clone 再搬進閉包」，需要兩邊都有的時候這樣做。

### `Copy` 型別：搬進去的是複本

整數是 `Copy`（第 3 章第 8 集），`move` 搬進去的就是一份複本，外面的變數不受影響，也不會失效：

```rust
fn main() {
    let mut count = 0;
    let mut tick = move || {
        count += 1;
        println!("閉包裡的 count 是 {count}");
    };
    tick();
    tick();
    println!("外面的 count 還是 {count}");
}
```

執行結果：

```text
閉包裡的 count 是 1
閉包裡的 count 是 2
外面的 count 還是 0
```

閉包裡的 `count` 是自己的那一份，每次呼叫都接著上一次加；外面的 `count` 從頭到尾沒變。沒有 `move` 的話（上一集），閉包改的就是外面那一個。

### 什麼時候非用 `move` 不可

閉包預設只是借用。但如果閉包活得比被借的變數久，借用就撐不住。下面的 `name` 在內層區塊結束就 drop，閉包卻被帶到區塊外面：

```rust,compile_fail=E0373
fn main() {
    let greet = {
        let name = String::from("小美");
        || println!("你好，{name}")
    };
    greet();
}
```

編譯錯誤：

```text
error[E0373]: closure may outlive the current block, but it borrows `name`, which is owned by the current block
 --> src/main.rs:4:9
  |
4 |         || println!("你好，{name}")
  |         ^^                  ---- `name` is borrowed here
  |         |
  |         may outlive borrowed value `name`
  |
note: block requires argument type to outlive `'1`
 --> src/main.rs:2:9
  |
2 |     let greet = {
  |         ^^^^^
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword
  |
4 |         move || println!("你好，{name}")
  |         ++++
```

`closure may outlive the current block, but it borrows name`：閉包可能活得比區塊久，卻借著區塊裡的 `name`。`help` 直接給了答案，加上 `move`，讓閉包自己擁有 `name`：

```rust
fn main() {
    let greet = {
        let name = String::from("小美");
        move || println!("你好，{name}")
    };
    greet();
    greet();
}
```

執行結果：

```text
你好，小美
你好，小美
```

把閉包從函式回傳（第 6 集）、交給別的執行緒（第 14 章），都是同樣的情況：閉包要離開現在的範圍，就得自己擁有用到的東西，所以要加 `move`。

### 跟 Go 比一比

Go 的閉包永遠共用外面的變數，沒有「複製進去」這回事：

```go
count := 0
tick := func() { count++ }
tick()
tick()
fmt.Println(count) // 2
```

Rust 的 `move` 閉包拿到的是複本（整數）或所有權（`String`）。同樣的 `tick` 加了 `move` 之後，外面的 `count` 還是 0。想在閉包外面看到變化，就不能 `move`，讓閉包借 `&mut`，或把結果當回傳值帶出來。

## 重點整理

- `move || ...` 讓閉包拿走用到的變數；`String` 這類非 `Copy` 的型別被搬走，原本的變數不能再用（E0382）。
- `Copy` 型別（整數）用 `move` 搬進去的是複本，閉包改它，外面的變數不變。
- 閉包要活得比被借的變數久（離開區塊、從函式回傳、交給別的執行緒），必須 `move`，否則是 E0373。
- 跟 Go 比：Go 的閉包永遠共用變數；Rust 要共用就別 `move`，要獨立就 `move`。
