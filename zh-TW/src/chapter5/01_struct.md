# `struct`

## 本集目標

用 `struct` 定義一個欄位有名字的型別，知道欄位要標型別，以及這一章的欄位為什麼都用 `String` 而不是 `&str`。

## 正文

### tuple 的問題

第 2 章用 tuple 表示一筆訂單：

```rust
fn main() {
    let order = (3, 12.5, true);
    println!("數量：{}", order.0);
    println!("單價：{}", order.1);
    println!("已付款：{}", order.2);
}
```

執行結果：

```text
數量：3
單價：12.5
已付款：true
```

程式能跑，但 `.0`、`.1`、`.2` 各是什麼意思，只能靠你自己記得。欄位一多，或是隔一個月再回來看，就得用猜的。

### 定義 struct

`struct` 讓每個欄位都有名字：

```rust
struct Order {
    quantity: u32,
    price: f64,
    paid: bool,
}

fn main() {
    let order = Order {
        quantity: 3,
        price: 12.5,
        paid: true,
    };
    println!("數量：{}", order.quantity);
    println!("單價：{}", order.price);
    println!("已付款：{}", order.paid);
}
```

執行結果：

```text
數量：3
單價：12.5
已付款：true
```

這段程式分成兩部分：定義，以及在 `main` 裡建立與讀取。建立與讀取下一集再細講，這一集先看定義：

- `struct Order { ... }` 定義一個新的型別，叫 `Order`，寫在函式外面。
- 每個欄位寫成 `名稱: 型別`，欄位之間用逗號隔開。最後一個欄位後面的逗號習慣也寫上。
- 型別名稱用大寫開頭（`Order`），欄位名稱跟變數一樣用小寫加底線（`quantity`）。

定義只是說明「訂單長什麼樣子」，不會產生任何資料。對照一下 tuple：`order.0` 變成 `order.quantity`，不用記，一看就知道是數量。

Go 靠欄位名稱的大小寫決定別的 package 看不看得到；Rust 的欄位預設是私有的，要用 `pub` 才開放，第 12 章才教。現在程式都在同一個檔案裡，完全不受影響。

### 欄位的型別用 `String`

要存名字，欄位型別該寫什麼？第 4 章說字串字面值是 `&str`，直覺會想寫 `&str`：

```rust,compile_fail=E0106
struct Member {
    name: &str,
}

fn main() {
    let member = Member { name: "小美" };
    println!("{}", member.name);
}
```

編譯錯誤：

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:11
  |
2 |     name: &str,
  |           ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Member<'a> {
2 ~     name: &'a str,
  |
```

`missing lifetime specifier` 的意思是：`&str` 是借來的，struct 要說清楚它借自誰、能活多久，這牽涉到第 10 章的生命週期。`help` 建議的 `<'a>` 是第 10 章的語法，現在先不用管。

這一章先避開這件事：**欄位一律放自己擁有資料的型別**，字串用 `String`。欄位歸 struct 所有，struct 離開作用域時，欄位跟著 drop（第 3 章第 4 集），不必牽涉別人的資料：

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

## 重點整理

- tuple 靠 `.0`、`.1` 取值，意思要自己記；`struct` 讓每個欄位有名字。
- `struct 型別名稱 { 欄位: 型別, ... }` 定義新型別，寫在函式外面；型別名稱大寫開頭，欄位名稱小寫加底線。
- 定義只是描述長相，不會產生資料；建立與讀取欄位下一集講。
- 欄位型別用 `&str` 會得到 E0106（缺少生命週期標註），第 10 章才解決；這一章的欄位都放擁有的型別，字串用 `String`。
- Rust 欄位預設私有，`pub` 第 12 章才教。
