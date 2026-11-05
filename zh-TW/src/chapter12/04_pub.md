# `pub` 與可見性

## 本集目標

知道 Rust 的東西預設是私有的，會用 `pub` 開放函式、`struct` 與欄位，並說得出「私有」到底是對誰私有。

## 正文

### 預設是私有的

上一集說過，模組裡的函式前面要加 `pub`。拿掉看看：

```rust,compile_fail=E0603
mod menu {
    fn price(drink: &str) -> u32 {
        if drink == "紅茶" { 30 } else { 45 }
    }
}

fn main() {
    println!("{}", menu::price("紅茶"));
}
```

編譯錯誤：

```text
error[E0603]: function `price` is private
 --> src/main.rs:8:26
  |
8 |     println!("{}", menu::price("紅茶"));
  |                          ^^^^^ private function
```

訊息說 `price` 是私有的（private）；後面還有一則 `note`，指出它定義在哪裡。Rust 跟 Go 的預設相反：**什麼都不寫就是私有，要開放得明確寫 `pub`**。函式、`struct`、`enum`、常數、`mod` 都一樣。

### 私有是對「外面」私有

私有的東西不是誰都不能用：定義它的模組自己，還有那個模組**裡面**的子模組，都看得到。看不到的是外面的人。

```rust
mod shop {
    fn secret_discount() -> u32 {
        5
    }

    pub mod bill {
        pub fn total(price: u32) -> u32 {
            price - crate::shop::secret_discount()
        }
    }
}

fn main() {
    println!("{}", shop::bill::total(30));
}
```

執行結果：

```text
25
```

`secret_discount` 沒有 `pub`，`main` 呼叫不到；但 `shop` 裡面的 `bill` 可以。所以規則是一句話：**沒有 `pub` 的東西，只有定義它的模組，以及這個模組裡面的子模組看得到。**

### `struct` 的欄位要各自開放

`pub struct` 只開放型別本身，欄位還是各自預設私有，要逐一決定：

```rust,compile_fail=E0616
mod club {
    pub struct Member {
        pub name: String,
        points: u32,
    }

    impl Member {
        pub fn new(name: String) -> Member {
            Member { name, points: 0 }
        }
    }
}

fn main() {
    let m = club::Member::new(String::from("小美"));
    println!("{} 有 {} 點", m.name, m.points);
}
```

編譯錯誤：

```text
error[E0616]: field `points` of struct `Member` is private
  --> src/main.rs:16:37
   |
16 |     println!("{} 有 {} 點", m.name, m.points);
   |                                       ^^^^^^ private field
```

`name` 加了 `pub`，外面讀得到；`points` 沒有，讀它是 E0616。這不是麻煩，是好處：外面既然碰不到 `points`，就不會有人把它亂改成不合理的值。要讓外面存取，由你提供方法，第 5 章第 7 集學過的 `impl` 就派上用場：

```rust
mod club {
    pub struct Member {
        pub name: String,
        points: u32,
    }

    impl Member {
        pub fn new(name: String) -> Member {
            Member { name, points: 0 }
        }

        pub fn add_points(&mut self, n: u32) {
            self.points += n;
        }

        pub fn points(&self) -> u32 {
            self.points
        }
    }
}

fn main() {
    let mut m = club::Member::new(String::from("小美"));
    m.add_points(30);
    println!("{} 有 {} 點", m.name, m.points());
}
```

執行結果：

```text
小美 有 30 點
```

方法也要 `pub`，不然外面一樣叫不到。注意 `new` 也得是 `pub`：有私有欄位的 `struct`，外面沒辦法用 `club::Member { ... }` 直接建立（那是 E0451），只能靠你開放的 `new`。這就是第 5 章第 9 集 `new` 慣例的另一層意義：由建立的人決定欄位的初始值。

### `enum` 的 variant 跟著開放

`pub enum` 的每個 variant 自動就是公開的，不用逐一加 `pub`，外面可以直接寫 `shop::Size::M`。跟 `struct` 的欄位不同：少了一個 variant，`match` 就不完整，沒有理由只開放一部分。

### 跟 Go 比一比

Go 用名字的大小寫，決定別的 package 能不能用，而且「別人」是以 package 為單位：同一個 package 裡的檔案，不管大小寫都互通。Rust 的單位是模組，比較細；而且要不要開放是明寫的 `pub`，改名字不會意外改變可見性。

## 重點整理

- 預設私有；加 `pub` 才開放。函式、型別、常數、`mod` 都一樣，外面用私有的東西是 E0603。
- 沒有 `pub` 的東西，只有定義它的模組和裡面的子模組看得到。
- `pub struct` 只開放型別，欄位要各自加 `pub`；外面讀私有欄位是 E0616，用大括號直接建立是 E0451，要靠 `pub fn new` 和 `pub` 方法。
- `pub enum` 的 variant 自動公開。
- 跟 Go 比：Go 以大小寫決定、以 package 為單位；Rust 以 `pub` 決定、以模組為單位。
