# shadowing 陷阱

## 本集目標

認得 shadowing（第 1 章第 6 集）最常見的三個陷阱，並知道什麼時候該用它、什麼時候改用 `mut` 或新名字。

## 正文

### 迴圈裡的 shadowing 不會累加

想把每一杯的價錢加起來，卻在迴圈裡寫了 `let`：

```rust,warn
fn main() {
    let prices = [30, 45, 38];
    let mut total = 0;
    for price in prices {
        let total = total + price;
        println!("目前 {total}");
    }
    println!("總共 {total}");
}
```

編譯警告：

```text
warning: variable does not need to be mutable
 --> src/main.rs:3:9
  |
3 |     let mut total = 0;
  |         ----^^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default
```

執行結果：

```text
目前 30
目前 45
目前 38
總共 0
```

迴圈裡的 `let total` 每一圈都是新變數，算完就丟，外面的 `total` 從頭到尾沒變。程式能編譯，但有線索：外面的 `let mut total` 從來沒被改過，所以編譯器警告「不需要 `mut`」。看到這個警告，先想想是不是 shadowing 搞的鬼。要累加，把迴圈裡的 `let` 拿掉，寫成 `total += price;`。

### shadowing 會讓 `mut` 失效

新的 `let` 建立的是新變數，不會繼承 `mut`。要改它，要重新寫 `let mut`：

```rust,compile_fail=E0384
fn main() {
    let mut count = 1;
    count += 1;
    let count = count * 2;
    count += 1;
    println!("{count}");
}
```

編譯錯誤：

```text
error[E0384]: cannot assign twice to immutable variable `count`
 --> src/main.rs:5:5
  |
4 |     let count = count * 2;
  |         ----- first assignment to `count`
5 |     count += 1;
  |     ^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
4 |     let mut count = count * 2;
  |         +++
```

反過來說，這個特性也能故意拿來用：變數改完值之後，寫一行 `let count = count;`，後面它就改不動了。

### 名字沒變，型別變了

shadowing 可以換型別，所以名字相同、型別不同的情況很常見。第 1 章的固定句型 `let line = line.trim();` 之後，`line` 就從 `String` 變成 `&str`：

```rust,compile_fail=E0599
fn main() {
    let mut line = String::from("  紅茶 \n");
    let line = line.trim();
    line.push_str("好喝");
    println!("{line}");
}
```

編譯錯誤：

```text
error[E0599]: no method named `push_str` found for reference `&str` in the current scope
 --> src/main.rs:4:10
  |
4 |     line.push_str("好喝");
  |          ^^^^^^^^ method not found in `&str`
  |
note: there's an earlier shadowed binding `line` of type `String` that has method `push_str` available
 --> src/main.rs:2:9
  |
2 |     let mut line = String::from("  紅茶 \n");
  |         ^^^^^^^^ `line` of type `String` that has method `push_str` defined earlier here
3 |     let line = line.trim();
  |         ---- earlier `line` shadowed here with type `&str`
```

第四行的 `line` 已經是 `&str`，沒有 `push_str`。`note` 特地指出：前面有一個同名、型別是 `String` 的變數被蓋掉了。

### 什麼時候用，什麼時候不用

- 適合：同一份資料換個形式繼續用（`trim`、`parse`），或改完值之後凍結；換型別時不用再想新名字。
- 不適合：要累加、要在迴圈裡更新的值，用 `let mut` 加 `+=`。兩個同名變數離得太遠、型別又不同，讀的人很容易看錯，換個新名字比較清楚。
- 注意：shadowing 不會讓舊變數消失。第 3 章第 4 集說過，舊變數要到離開作用域才 drop，借著它的參照照樣有效。想早點釋放，要靠區塊縮短它的作用域。

### 跟 Go 比一比

Go 的內層區塊用 `:=` 也有一模一樣的陷阱：

```go
total := 0
for _, price := range prices {
	total := total + price // 新的 total，外層沒變
	fmt.Println(total)
}
fmt.Println(total) // 0
```

差別是 Go 在同一個範圍裡不能重複 `:=`，所以陷阱只出現在內層區塊；Rust 同一層也能 shadow，要多留意。

## 重點整理

- 迴圈裡的 `let total = total + price;` 是新變數，外面的 `total` 不會變；要累加用 `let mut` 加 `+=`。外面的 `mut` 沒被改過時，編譯器會警告「不需要 `mut`」。
- shadowing 之後的變數預設不可變，要改它得重新寫 `let mut`；改完值後 `let x = x;` 可以把它凍結。
- 名字相同不代表型別相同；方法不存在時，編譯器的 `note` 會指出被蓋掉的同名變數。
- 被蓋掉的舊變數不會消失，借著它的參照照樣有效，要到離開作用域才 drop。
- Go 的內層 `:=` 也有同樣的陷阱，Rust 連同一層的 `let` 也會 shadow。
