# 遞迴型別需要 `Box`

## 本集目標

知道為什麼「裡面又包含自己」的型別一定要經過 `Box`，並用 `Box` 寫出遞迴的 enum 與 struct。

## 正文

### 自己包含自己

套餐的價格可以是單一品項，也可以是「兩個價格相加」，或是「某個價格再折掉一些」。後兩種的內容裡又有 `Price`，這種型別叫遞迴型別（recursive type）。直接寫寫看：

```rust,compile_fail=E0072
enum Price {
    Item(i32),
    Add(Price, Price),
}

fn main() {
    let tea = Price::Item(35);
    println!("{}", matches!(tea, Price::Item(_)));
}
```

編譯錯誤：

```text
error[E0072]: recursive type `Price` has infinite size
 --> src/main.rs:1:1
  |
1 | enum Price {
  | ^^^^^^^^^^
2 |     Item(i32),
3 |     Add(Price, Price),
  |         ----- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
3 |     Add(Box<Price>, Price),
  |         ++++     +
```

編譯器要在編譯時期就知道每個型別佔多大。`Price` 的大小由最大的 variant 決定，`Add` 裡面有兩個完整的 `Price`，每個又有兩個，永遠算不完，所以是 E0072（infinite size）。Go 也一樣：struct 不能直接包含自己，一定要寫成 `*Node`。

```text
Price = 最大的 variant
Add(Price, Price) = 兩個 Price = 四個 Price = ...   算不完
```

`help` 說得很清楚：插入 `Box`。`Box<Price>` 在 `Add` 裡面只佔一個指標，大小固定；真正的 `Price` 放在 heap 上，想要多深都行：

```text
Add(Box<Price>, Box<Price>)
    [ ptr | ptr ]
       |     +---> Item(45)    (heap)
       +---------> Item(35)    (heap)
```

### 用 `Box` 改好

```rust
enum Price {
    Item(i32),
    Add(Box<Price>, Box<Price>),
    Off(Box<Price>, i32),
}

fn total(price: &Price) -> i32 {
    match price {
        Price::Item(n) => *n,
        Price::Add(a, b) => total(a) + total(b),
        Price::Off(p, off) => total(p) - off,
    }
}

fn main() {
    let combo = Price::Off(
        Box::new(Price::Add(
            Box::new(Price::Item(35)),
            Box::new(Price::Item(45)),
        )),
        10,
    );
    println!("{}", total(&combo));
}
```

執行結果：

```text
70
```

- `combo` 是「紅茶 35 加咖啡 45，再折 10 元」。每一層子價格都用 `Box::new(...)` 包起來。
- `total` 照著資料的形狀遞迴（第 2 章第 14 集），遇到 `Item` 就停。`match` 裡綁到的 `a`、`b` 是 `&Box<Price>`，函式要的卻是 `&Price`：編譯器會自動轉換，跟第 4 章 `&String` 變 `&str` 是同一件事，原理下一集解釋。
- 整棵價格樹只有一個擁有者 `combo`；它離開作用域時，外層的 `Box` 被 drop，裡面的 `Box` 也跟著逐層 drop，heap 上的每一塊都會釋放。

### struct 也一樣

struct 想包含自己，要靠 `Option` 讓它有個終點，再用 `Box` 讓大小固定：

```rust
struct Step {
    name: String,
    next: Option<Box<Step>>,
}

fn print_steps(step: &Step) {
    println!("{}", step.name);
    if let Some(next) = &step.next {
        print_steps(next);
    }
}

fn main() {
    let steps = Step {
        name: String::from("裝杯"),
        next: Some(Box::new(Step {
            name: String::from("加料"),
            next: Some(Box::new(Step {
                name: String::from("封口"),
                next: None,
            })),
        })),
    };
    print_steps(&steps);
}
```

執行結果：

```text
裝杯
加料
封口
```

`next: Option<Step>`（沒有 `Box`）同樣是 E0072，`help` 也一樣建議 `Option<Box<Step>>`。`None` 就是鏈子的尾巴。

不是所有「裡面有自己」的型別都需要 `Box`：`Vec` 本身就是 heap 上的一塊記憶體，變數裡只有一個指標，大小固定，所以 enum 的 variant 寫成 `Group(String, Vec<Menu>)`（裡面又裝著 `Menu`）不需要 `Box`。

## 重點整理

- 型別裡面直接包含自己（遞迴型別），大小算不完，編譯器報 E0072：`recursive type has infinite size`。
- 解法是在遞迴的位置放 `Box<自己>`：指標大小固定，真正的資料在 heap，可以任意深。
- enum 用 `Add(Box<Price>, Box<Price>)`，struct 用 `next: Option<Box<Step>>`（`None` 當終點）；建立時每層都要 `Box::new(...)`。
- 遞迴型別搭配遞迴函式處理；`&Box<Price>` 傳給要 `&Price` 的參數，編譯器會自動轉換（下一集解釋）。
- `Vec<T>` 本身就在 heap，所以裝著自己的 `Vec` 不需要 `Box`。
