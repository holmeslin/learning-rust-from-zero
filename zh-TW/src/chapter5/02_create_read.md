# 建立與讀取欄位

## 本集目標

用 `型別名稱 { 欄位: 值 }` 建立 struct，用 `.` 讀取欄位，並知道每個欄位都要給值、修改欄位時整個變數都要 `mut`。

## 正文

### 建立與讀取

建立 struct 的值，寫成 `型別名稱 { 欄位: 值, ... }`；讀取欄位用 `.`：

```rust
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        points: 120,
    };
    println!("{} 有 {} 點", member.name, member.points);
}
```

執行結果：

```text
小美 有 120 點
```

欄位的順序不用跟定義時一樣，把 `points` 寫在 `name` 前面也行，只要每個欄位都給了值。

### 每個欄位都要給

少寫一個欄位，編譯器不收：

```rust,compile_fail=E0063
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
    };
    println!("{}", member.name);
}
```

編譯錯誤：

```text
error[E0063]: missing field `points` in initializer of `Member`
 --> src/main.rs:7:18
  |
7 |     let member = Member {
  |                  ^^^^^^ missing `points`
```

訊息說建立 `Member` 時少了欄位 `points`（missing field）。Rust 沒有「沒寫就補預設值」這件事，每個欄位都得自己決定。

### 修改欄位

欄位可以用 `.` 指定新值，前提是整個變數用 `let mut` 建立：

```rust
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let mut member = Member {
        name: String::from("小美"),
        points: 120,
    };
    member.points = 150;
    member.name = String::from("小華");
    println!("{} 有 {} 點", member.name, member.points);
}
```

執行結果：

```text
小華 有 150 點
```

`member.name = String::from("小華");` 會把舊的 `String` 當場 drop（第 3 章第 4 集），跟重新指定任何變數一樣。

`mut` 是整個 struct 一起的，**沒有「只有某個欄位可以改」的寫法**。變數不是 `mut`，所有欄位都不能改：

```rust,compile_fail=E0594
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        points: 120,
    };
    member.points = 150;
    println!("{} 有 {} 點", member.name, member.points);
}
```

編譯錯誤：

```text
error[E0594]: cannot assign to `member.points`, as `member` is not declared as mutable
  --> src/main.rs:11:5
   |
11 |     member.points = 150;
   |     ^^^^^^^^^^^^^^^^^^^ cannot assign
   |
help: consider changing this to be mutable
   |
 7 |     let mut member = Member {
   |         +++
```

訊息說 `member` 不是 `mut`，所以不能指定新值。照 `help` 在 `let` 後面加上 `mut` 就好。

### 傳給函式

struct 傳進函式的規則，跟前兩章的 `String` 一樣：直接傳會 move，整個 struct 連同裡面的欄位都交給函式；只想讀就借 `&Member`，要改就借 `&mut Member`。

### 跟 Go 比一比

Go 的 struct 有零值，欄位可以只寫幾個，其他自動補零：

```go
type Member struct {
    Name   string
    Points int
}

a := Member{Name: "小美"} // Points 是 0
var b Member              // 全部欄位都是零值
b.Points = 150            // 變數不用特別宣告成可變
```

前兩行在 Rust 都行不通：每個欄位都要給值，也沒有 `var b Member` 這種先宣告、再慢慢補的寫法。第三行要改欄位，變數得寫 `let mut`。沒有零值是 Rust 刻意的設計：不會有人把沒填的 `""` 或 `0` 當成真正的資料。

## 重點整理

- 建立 struct：`型別名稱 { 欄位: 值, ... }`，欄位順序不拘，但**每個欄位都要給值**，漏掉是 E0063。
- 讀取欄位用 `.`；修改欄位要把整個變數宣告成 `let mut`，沒有單一欄位的 `mut`（否則 E0594）。
- struct 傳進函式遵守第 3、4 章的規則：直接傳會 move，`&Member` 只能讀，`&mut Member` 可改。
- Go 的 struct 有零值，可以只寫部分欄位；Rust 沒有零值，每個欄位都要明確給值。
