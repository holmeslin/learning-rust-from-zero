# struct 更新語法 `..`（會 move）

## 本集目標

用 `..舊值` 以現有的 struct 為底建立新的 struct，並看懂哪些欄位會被 move、哪些只是複製。

## 正文

### 用 `..` 沿用其他欄位

只想改一兩個欄位、其他照舊時，不用把每個欄位重寫一遍。在最後面加上 `..舊值`，沒寫的欄位就從舊值拿：

```rust
struct Member {
    name: String,
    city: String,
    points: u32,
}

fn main() {
    let m1 = Member {
        name: String::from("小美"),
        city: String::from("台北"),
        points: 120,
    };
    let m2 = Member {
        name: String::from("小華"),
        ..m1
    };
    println!("{} 住在 {}，有 {} 點", m2.name, m2.city, m2.points);
}
```

執行結果：

```text
小華 住在 台北，有 120 點
```

`m2` 的 `name` 是新寫的，`city` 和 `points` 從 `m1` 拿。`..m1` 一定要放在最後，而且後面不能再加逗號。

### 拿走的欄位會 move

「從 `m1` 拿」的意思，跟 `let t = s;` 一樣：`city` 是 `String`，所以是 move；`points` 是 `u32`，只是複製。`m1.city` 的所有權已經給了 `m2`，再讀它就是 E0382：

```rust,compile_fail=E0382
struct Member {
    name: String,
    city: String,
    points: u32,
}

fn main() {
    let m1 = Member {
        name: String::from("小美"),
        city: String::from("台北"),
        points: 120,
    };
    let m2 = Member {
        name: String::from("小華"),
        ..m1
    };
    println!("{} 住在 {}", m2.name, m2.city);
    println!("{} 住在 {}", m1.name, m1.city);
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `m1.city`
  --> src/main.rs:18:35
   |
13 |       let m2 = Member {
   |  ______________-
14 | |         name: String::from("小華"),
15 | |         ..m1
16 | |     };
   | |_____- value moved here
17 |       println!("{} 住在 {}", m2.name, m2.city);
18 |       println!("{} 住在 {}", m1.name, m1.city);
   |                                       ^^^^^^^ value borrowed here after move
   |
   = note: move occurs because `m1.city` has type `String`, which does not implement the `Copy` trait
```

訊息說 `m1.city` 已經被 move：`value moved here` 指的是那一整段 `Member { ... ..m1 }`。這叫**部分 move**（partial move）：`m1` 只有 `city` 被拿走。**沒被拿走的欄位還能讀**：`m1.name`（`m2` 的 `name` 是新寫的，沒動到舊的）和 `m1.points`（複製）都沒問題，只有 `m1.city` 不行。

整個 `m1` 少了一塊，就不能再當成整體用：不能傳給函式，也不能寫 `&m1`，編譯器會說 `borrow of partially moved value: m1`。

### 不拿 `String`，舊值就不受影響

如果新值把所有 `String` 欄位都自己寫了（`name` 和 `city` 都寫），從舊值拿的就只剩 `Copy` 的 `points`。沒有任何東西被 move，`m1` 完好如初，整個傳給函式、借出去都可以。

### 跟 Go 比一比

Go 的 `b := a` 會把整個 struct 複製一份，兩邊各改各的，`a` 還能用：

```go
a := Member{Name: "小美", City: "台北", Points: 120}
b := a // 複製
b.Name = "小華"
fmt.Println(a.Name) // 小美
```

Rust 的 struct 預設**不是** `Copy`。就算欄位全部都是整數，`let b = a;` 也是 move：

```rust,compile_fail=E0382
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;
    println!("{} {}", a.x, b.y);
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `a`
 --> src/main.rs:9:23
  |
7 |     let a = Point { x: 1, y: 2 };
  |         - move occurs because `a` has type `Point`, which does not implement the `Copy` trait
8 |     let b = a;
  |             - value moved here
9 |     println!("{} {}", a.x, b.y);
  |                       ^^^ value borrowed here after move
  |
```

這裡只貼前半段，後面還有一則 `note`，說的是 `Clone`：想讓自己的 struct 能複製，要另外加上 `derive`，第 8 章才教。在那之前，想要另一份，就得像上面那樣逐一寫出欄位，或是用 `..` 沿用。

## 重點整理

- `Member { name: ..., ..m1 }` 以 `m1` 為底，沒寫的欄位從 `m1` 拿；`..m1` 要放在最後，後面不加逗號。
- 從舊值拿的欄位照一般規則：`String` 等非 `Copy` 的欄位會 move，`Copy` 的欄位是複製。
- 欄位被 move 之後（partial move），沒被拿走的欄位還能讀，但被拿走的欄位和整個舊值不能再用（E0382）。
- 新值把所有非 `Copy` 的欄位都自己寫了，舊值就完全不受影響。
- struct 預設不是 `Copy`，欄位全是整數也一樣：`let b = a;` 會 move，跟 Go 的 `b := a` 複製不同。
