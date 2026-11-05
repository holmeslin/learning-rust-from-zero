# 方法的生命週期

## 本集目標

會替帶生命週期的 struct 寫 `impl`（`impl Receipt<'_>`、`impl<'a> Receipt<'a>`），並分辨方法回傳 `&str`（借自 `&self`）與 `&'a str`（借自原本的資料）有什麼不同。

## 正文

### `impl` 要把生命週期帶上

struct 有 `<'a>`，`impl` 也要帶上。只寫 `impl Receipt` 會得到 E0726（`implicit elided lifetime not allowed here`），`help` 會建議 `impl Receipt<'_>`。有兩種寫法：

- `impl Receipt<'_>`：方法裡用不到 `'a` 這個名字的時候。
- `impl<'a> Receipt<'a>`：方法簽名裡要寫出 `'a` 的時候。`impl<'a>` 負責宣告，道理跟第 8 章第 9 集的 `impl<T> Shelf<T>` 一樣。

先看不需要名字的：

```rust
struct Receipt<'a> {
    customer: &'a str,
    drink: &'a str,
}

impl Receipt<'_> {
    fn drink(&self) -> &str {
        self.drink
    }
}

impl std::fmt::Display for Receipt<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} 點了 {}", self.customer, self.drink)
    }
}

fn main() {
    let customer = String::from("小美");
    let receipt = Receipt {
        customer: &customer,
        drink: "咖啡",
    };
    println!("{receipt}");
    println!("飲料：{}", receipt.drink());
}
```

執行結果：

```text
小美 點了 咖啡
飲料：咖啡
```

`Display` 的 `impl` 也是同樣的道理，`Receipt<'_>` 要帶上。順便回收第 8 章第 11 集那個骨架：`std::fmt::Formatter` 其實也是帶生命週期的型別，完整寫法是 `Formatter<'_>`。以前省略也能編譯，現在你知道省略掉的是什麼了。

### 方法回傳 `&str`：借自 `&self`

`drink(&self) -> &str` 沒有標註，靠的是省略規則 3：參數有 `&self`，回傳值就用 `self` 的生命週期。也就是說，回傳的 `&str` 借自「`receipt` 這個變數被借用的這段」。

規則 3 優先於其他參數：就算方法還有別的參照參數，沒標註的回傳值也只跟 `&self` 綁在一起。

這會有個副作用。`receipt.customer` 其實借自外面的 `customer`，活得比 `receipt` 久，但方法簽名把結果綁在 `&self` 上，編譯器就只能保證結果活得跟 `receipt` 一樣久：

```rust,compile_fail=E0597
struct Receipt<'a> {
    customer: &'a str,
}

impl Receipt<'_> {
    fn customer(&self) -> &str {
        self.customer
    }
}

fn main() {
    let customer = String::from("小美");
    let name;
    {
        let receipt = Receipt {
            customer: &customer,
        };
        name = receipt.customer();
    }
    println!("{name}");
}
```

編譯錯誤：

```text
error[E0597]: `receipt` does not live long enough
  --> src/main.rs:18:16
   |
15 |         let receipt = Receipt {
   |             ------- binding `receipt` declared here
...
18 |         name = receipt.customer();
   |                ^^^^^^^ borrowed value does not live long enough
19 |     }
   |     - `receipt` dropped here while still borrowed
20 |     println!("{name}");
   |                ---- borrow later used here
```

`name` 在 `receipt` 的 `}` 之後還要用，超出了「`receipt` 被借用的那段」，所以編譯器說 `receipt` 活得不夠久。可是實際上 `name` 指的是 `customer`，根本沒有問題，只是簽名沒把這件事說出來。

### 方法回傳 `&'a str`：借自原本的資料

要說出來，就在回傳型別寫 `'a`：結果的有效範圍，是 struct 欄位裡那個參照的範圍，不是這次借用 `&self` 的範圍。這時候 `impl` 要寫成 `impl<'a>`，才有 `'a` 可以用：

```rust
struct Receipt<'a> {
    customer: &'a str,
}

impl<'a> Receipt<'a> {
    fn customer(&self) -> &'a str {
        self.customer
    }
}

fn main() {
    let customer = String::from("小美");
    let name;
    {
        let receipt = Receipt {
            customer: &customer,
        };
        name = receipt.customer();
    }
    println!("{name}");
}
```

執行結果：

```text
小美
```

兩種寫法怎麼選：

- 回傳的是 struct 裡存著的參照（欄位裡的 `&'a str`）：標 `'a`，結果可以活得比 struct 久。
- 回傳的內容借自 struct 本身，或你不確定：省略就好，結果跟 `&self` 綁在一起，最保險。

## 重點整理

- 帶生命週期的 struct，`impl` 也要帶：不需要名字時寫 `impl Receipt<'_>`；方法簽名裡要寫 `'a` 時寫 `impl<'a> Receipt<'a>`。只寫 `impl Receipt` 是 E0726。
- 實作 trait 也一樣（`impl std::fmt::Display for Receipt<'_>`）；`Formatter` 的完整寫法是 `Formatter<'_>`。
- 方法回傳的參照沒標註，省略規則 3 讓它借自 `&self`，優先於其他參數。
- 回傳值要比 struct 活得久，就標 `-> &'a str`，借的是欄位裡存的參照，不是 `&self`。
- 編譯器說某個值「活得不夠久」，但你確定資料夠久，通常就是簽名少說了一句：把關係寫出來。
