# `Display` 與 `to_string`

## 本集目標

替自己的型別實作 `Display`，讓 `{}` 能印出它；並知道實作之後，型別會自動多一個 `.to_string()`。

## 正文

### `{}` 要有 `Display`

第 2 章第 7 集說過，`()` 不能用 `{}` 印，因為它沒有「定義好的顯示方式」。這個「顯示方式」就是 `Display` trait：`{}` 只能印實作了 `Display` 的型別，`{:?}` 則是要 `Debug`（上一集的 `derive`）。第 7 章用 `{:?}` 印 `Vec`，就是因為 `Vec` 沒有 `Display`。

自己的 struct 沒有實作的話，用 `{}` 印會是 E0277：

```rust,compile_fail=E0277
struct Tea {
    toppings: u32,
}

fn main() {
    let tea = Tea { toppings: 2 };
    println!("{}", tea);
}
```

編譯錯誤：

```text
error[E0277]: `Tea` doesn't implement `std::fmt::Display`
 --> src/main.rs:7:20
  |
7 |     println!("{}", tea);
  |               --   ^^^ `Tea` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
```

`` `Tea` doesn't implement `std::fmt::Display` ``。訊息後面還有 `help` 和 `note`，`note` 建議改用 `{:?}`，但 `Tea` 也沒有 `derive(Debug)`，所以那條路也要先 `derive`。

### 自己實作 `Display`

`Debug` 是給寫程式的人看的，格式固定，所以能 `derive`。`Display` 是給使用者看的，要怎麼顯示得由你決定，所以編譯器不能代寫，要自己實作：

```rust
struct Tea {
    toppings: u32,
}

enum Status {
    Pending,
    Paid,
}

impl std::fmt::Display for Tea {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "奶茶（{} 種料）", self.toppings)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Pending => write!(f, "待付款"),
            Status::Paid => write!(f, "已付款"),
        }
    }
}

fn main() {
    let tea = Tea { toppings: 2 };
    println!("{}", tea);
    println!("{tea}");
    for status in [Status::Pending, Status::Paid] {
        println!("{status}");
    }

    let text = tea.to_string();
    let cups = 3.to_string();
    let name: String = "哈囉".to_string();
    println!("{} {cups} {name}", text.len());
}
```

執行結果：

```text
奶茶（2 種料）
奶茶（2 種料）
待付款
已付款
20 3 哈囉
```

`impl Display` 的骨架是**固定句型**，先照抄，只改型別名稱和 `write!` 裡的內容：

- `impl std::fmt::Display for Tea { ... }`：`Display` 住在 `std::fmt` 裡，這裡直接寫出完整路徑。
- `fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result` 整行照抄。`&self`、`&mut` 都學過；回傳的 `Result` 第 9 章才會解釋。
- `write!(f, "奶茶（{} 種料）", self.toppings)`：格式字串的寫法跟 `println!` 一樣，只是內容寫進 `f`，不是印到螢幕。巨集的原理第 16 章才說。
- `write!` 那一行沒有分號，它的值就是整個函式的回傳值（第 2 章第 4 集）。
- enum 的 `Display` 是每個分支各寫一個 `write!`，搭配 `match self`（第 6 章第 16 集）。

寫好之後，`{}` 跟 `{tea}` 都能印。

### 免費的 `to_string`

只要型別實作了 `Display`，就自動多了一個 `.to_string()`，回傳 `String`。上面的 `tea.to_string()` 就是這樣來的，`text.len()` 是 20 個位元組（中文字一個三位元組，第 7 章第 8 集）。

整數和 `&str` 本來就有 `Display`，所以 `3.to_string()`、`"哈囉".to_string()` 也能用。第 2 章第 15 集的錯誤訊息建議過 `.to_string()`，就是這個方法：它能把 `&str` 變成 `String`。

不要自己替型別寫一個叫 `to_string` 的方法，實作 `Display` 就夠了。

Go 的 `String() string` 方法對應的就是 `Display`：`fmt.Println` 看到有 `String()` 的型別就會呼叫它。差別在於 Go 是隱式滿足（介面 `fmt.Stringer`），Rust 要明確寫 `impl Display for`（第 2 集）。

## 重點整理

- `{}` 要型別實作 `Display`，`{:?}` 要 `Debug`；沒有 `Display` 的型別用 `{}` 是 E0277。
- `Display` 不能 `derive`，要自己寫：`impl std::fmt::Display for 型別 { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "...", ...) } }`，這個骨架先照抄。
- `write!(f, ...)` 的格式字串跟 `println!` 一樣，最後一行不加分號；enum 搭配 `match self`，每個分支各寫一個 `write!`。
- 實作 `Display` 之後自動有 `.to_string()`，回傳 `String`；不要自己寫 `to_string` 方法。
