# `where` 子句

## 本集目標

用 `+` 要求一個型別同時實作多個 trait，並把條件挪到 `where` 子句，讓簽名比較好讀。

## 正文

### 用 `+` 要求好幾個 trait

有時候一個函式需要型別同時會好幾件事。下面除了 `Priced`，多了一個 `Named`，函式要用到名字和價錢。條件之間用 `+` 連起來：

```rust
struct Tea {
    toppings: u32,
}

struct Cookie {
    pieces: u32,
}

trait Priced {
    fn price(&self) -> u32;
}

trait Named {
    fn name(&self) -> String;
}

impl Priced for Tea {
    fn price(&self) -> u32 {
        35 + self.toppings * 5
    }
}

impl Named for Tea {
    fn name(&self) -> String {
        String::from("奶茶")
    }
}

impl Priced for Cookie {
    fn price(&self) -> u32 {
        20 * self.pieces
    }
}

fn label<T: Priced + Named>(item: &T) -> String {
    format!("{}：{} 元", item.name(), item.price())
}

fn combo<T, U>(drink: &T, snack: &U) -> String
where
    T: Priced + Named,
    U: Priced,
{
    let total = drink.price() + snack.price();
    format!("{}加點心，共 {} 元", drink.name(), total)
}

fn main() {
    let tea = Tea { toppings: 2 };
    let cookie = Cookie { pieces: 2 };
    println!("{}", label(&tea));
    println!("{}", combo(&tea, &cookie));
}
```

執行結果：

```text
奶茶：45 元
奶茶加點心，共 85 元
```

- `label` 的 `<T: Priced + Named>`：`T` 兩個 trait 都要實作。`Tea` 都有，所以能傳；`Cookie` 只實作了 `Priced`，`label(&cookie)` 會是 E0277，訊息是 `` the trait bound `Cookie: Named` is not satisfied ``。少一個都不行。
- 函式裡面可以用兩個 trait 的所有方法：`item.name()` 來自 `Named`，`item.price()` 來自 `Priced`。

### 條件多了，改用 `where`

`combo` 有兩個型別參數，每個的條件都不一樣。條件全部擠在 `<>` 裡，簽名會又長又擠，參數反而不好找。這時候把條件移到回傳型別的後面，用 `where` 逐條列出來：

- `where` 寫在回傳型別之後、`{` 之前。
- 每個型別參數一行：`T: Priced + Named,`，行尾要逗號。
- `<T, U>` 只留名字。條件改寫在 `where` 裡，意思跟寫在 `<>` 裡完全相同，編譯結果也一樣。

`T` 是飲料，要有名字和價錢；`U` 是點心，只要有價錢，所以 `combo` 才能收 `Cookie`。兩個型別參數可以各自是不同的型別，也可以相同，因為 `T` 和 `U` 之間沒有限制。

什麼時候改用 `where`？條件只有一兩個，寫在 `<>` 裡就好（像 `label`）；條件多、型別參數多，或是簽名超過一行，就用 `where`。這是排版上的選擇，沒有對錯。

## 重點整理

- `T: Priced + Named` 用 `+` 要求同時實作多個 trait，少一個是 E0277。
- 條件裡列出的每個 trait，方法在函式裡都能用。
- `where` 寫在回傳型別之後、`{` 之前，每個型別參數一行、行尾加逗號；`<T, U>` 只留名字。
- `where` 跟寫在 `<>` 裡意思完全相同，只是比較好讀；條件多的時候才改用。
