# tuple struct 與 unit struct

## 本集目標

認識另外兩種 struct：欄位沒有名字的 tuple struct，和完全沒有欄位的 unit struct，並知道它們各自適合什麼場合。

## 正文

### tuple struct

有名字的 struct 之外，還有一種「欄位沒有名字」的寫法：型別名稱後面接小括號，括號裡只寫型別。取值跟 tuple 一樣用 `.0`、`.1`：

```rust
struct Meters(f64);
struct Rgb(u8, u8, u8);

fn main() {
    let length = Meters(2.5);
    let orange = Rgb(255, 128, 0);
    println!("長度 {} 公尺", length.0);
    println!("R={} G={} B={}", orange.0, orange.1, orange.2);
}
```

執行結果：

```text
長度 2.5 公尺
R=255 G=128 B=0
```

建立的寫法也像呼叫函式：`Meters(2.5)`、`Rgb(255, 128, 0)`。

### 為什麼不直接用 tuple

tuple struct 跟 tuple 最大的差別是：**它有自己的型別名稱**。`Meters` 跟 `Seconds` 就算裡面都是一個 `f64`，也是兩個不同的型別，不能混用：

```rust,compile_fail=E0308
struct Meters(f64);
struct Seconds(f64);

fn show_length(length: Meters) {
    println!("{} 公尺", length.0);
}

fn main() {
    let time = Seconds(9.5);
    show_length(time);
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
  --> src/main.rs:10:17
   |
10 |     show_length(time);
   |     ----------- ^^^^ expected `Meters`, found `Seconds`
   |     |
   |     arguments to this function are incorrect
   |
note: function defined here
  --> src/main.rs:4:4
   |
 4 | fn show_length(length: Meters) {
   |    ^^^^^^^^^^^ --------------
```

如果兩個都只是 `f64`，把時間當成長度傳進去，編譯器看不出來；包成不同的 tuple struct，就會被攔下。這種「用 struct 包住一個值、換一個型別名稱」的做法很常見。

怎麼選：欄位有兩三個以上、意思不明顯，就用有名字的 struct；只包一個值，或是欄位的意思一看就懂（像 RGB 三個顏色），用 tuple struct 就夠了。

### unit struct

完全沒有欄位的 struct，連括號都不用寫，叫 **unit struct**：

```rust
struct Marker;

fn main() {
    let _marker = Marker;
    println!("建立了一個沒有欄位的值");
}
```

執行結果：

```text
建立了一個沒有欄位的值
```

`Marker` 沒有任何資料可以存，單純是一個型別。現在看起來沒什麼用處，它的用途是第 8 章之後：幫型別加上行為時，有時候只需要一個「名字」，不需要資料。這一集先認得這個寫法就好。

`let _marker` 開頭的底線，是第 1 章第 19 集教的寫法，表示這個變數故意不用。

## 重點整理

- tuple struct：`struct Meters(f64);`，欄位沒有名字，用 `.0`、`.1` 取值，建立時寫成 `Meters(2.5)`。
- tuple struct 有自己的型別名稱：`Meters` 和 `Seconds` 就算內容型別相同也不能混用（E0308）。
- 欄位多或意思不明顯時用有名字的 struct；只包一個值，或意思一看就懂時用 tuple struct。
- unit struct：`struct Marker;`，沒有欄位，建立時寫 `Marker`；用途從第 8 章開始出現。
