# `#[derive(Debug)]` 與 `{:?}` `{:#?}`

## 本集目標

在 struct 前面加上 `#[derive(Debug)]`，就能用 `{:?}` 或 `{:#?}` 一次印出整個 struct。這是固定句型，先照抄，原理第 8 章解釋。

## 正文

### 印不出來的 struct

第 2 章教過 `{:?}` 可以印陣列和 tuple。拿去印自己定義的 struct，會被擋下來：

```rust,compile_fail=E0277
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        points: 120,
    };
    println!("{:?}", member);
}
```

編譯錯誤：

```text
error[E0277]: `Member` doesn't implement `Debug`
  --> src/main.rs:11:22
   |
11 |     println!("{:?}", member);
   |               ----   ^^^^^^ `Member` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |               |
   |               required by this formatting parameter
   |
   = help: the trait `Debug` is not implemented for `Member`
help: consider annotating `Member` with `#[derive(Debug)]`
   |
 1 + #[derive(Debug)]
 2 | struct Member {
   |
```

`Member` doesn't implement `Debug`：編譯器不知道要怎麼印這個型別。`help` 已經給了解法：在 struct 前面加 `#[derive(Debug)]`。

### 加上 `#[derive(Debug)]`

```rust
#[derive(Debug)]
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        points: 120,
    };
    println!("{:?}", member);
    println!("{:#?}", member);
    println!("{} 加 30 點是 {}", member.name, member.points + 30);
}
```

執行結果：

```text
Member { name: "小美", points: 120 }
Member {
    name: "小美",
    points: 120,
}
小美 加 30 點是 150
```

- `#[derive(Debug)]` 寫在 `struct` 那一行的正上方，整行照抄。原理第 8 章才解釋，現在只要知道：加了它，這個型別就能用 `{:?}` 印。
- `{:?}` 印成一行，欄位用逗號隔開；字串欄位會連引號一起印出來。
- `{:#?}` 是排版過的版本，每個欄位獨立一行，欄位多、或 struct 裡面還有 struct 時比較好讀。
- 變數也可以直接寫在大括號裡：`{member:?}`、`{member:#?}`。

### 欄位也要有 `Debug`

struct 的欄位如果也是自訂型別，那個型別也要加 `#[derive(Debug)]`，否則同樣是 E0277。印出來的時候，`{:#?}` 會依巢狀的深度一層層縮排。

tuple struct 和 unit struct 一樣能加：`#[derive(Debug)] struct Meters(f64);` 的值印出來是 `Meters(2.5)`，unit struct `Marker` 印出來就是 `Marker`。

### 印出來不算「讀過」

編譯器檢查「哪些欄位沒用到」時，**不把 derive 出來的 `Debug` 算進去**。如果程式只用 `{:?}` 印整個 struct，從來沒有用 `.` 讀過某個欄位，rustc 還是會警告：

```rust,warn
#[derive(Debug)]
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let member = Member {
        name: String::from("小美"),
        points: 120,
    };
    println!("{:?}", member);
}
```

編譯警告：

```text
warning: fields `name` and `points` are never read
 --> src/main.rs:3:5
  |
2 | struct Member {
  |        ------ fields in this struct
3 |     name: String,
  |     ^^^^
4 |     points: u32,
  |     ^^^^^^
  |
  = note: `Member` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default
```

警告不會擋住程式，但書裡的範例都要求沒有警告。所以有 `#[derive(Debug)]` 的 struct，每個欄位都會在程式裡用 `.` 讀到。真實的程式本來就會用到欄位，通常不會遇到這個警告。

## 重點整理

- 自訂型別要先加 `#[derive(Debug)]`（寫在 `struct` 上一行，整行照抄），才能用 `{:?}`、`{:#?}` 印；沒加是 E0277。原理第 8 章解釋。
- `{:?}` 印成一行，`{:#?}` 分行排版；也能寫成 `{member:?}`。
- 欄位是自訂型別時，那個型別也要 `#[derive(Debug)]`；tuple struct 印成 `Meters(2.5)`，unit struct 印成名字。
- 用 `{:?}` 印整個 struct 不算讀過欄位；從來沒用 `.` 讀過的欄位，rustc 會警告 `never read`。本書範例的每個欄位都會被讀到。
