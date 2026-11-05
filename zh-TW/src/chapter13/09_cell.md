# `Cell<T>`

## 本集目標

知道 `Cell<T>` 是更輕量的內部可變性：用 `get`、`set` 整個取出、放入值，不借用；會分辨什麼時候用 `Cell`、什麼時候用 `RefCell`。

## 正文

### 取出、放進，不用借

點餐機要數出了幾杯飲料。計數只是一個整數，不需要 `RefCell` 的 `borrow_mut()`：

```rust
use std::cell::Cell;

struct Kiosk {
    served: Cell<u32>,
}

impl Kiosk {
    fn serve(&self, drink: &str) {
        self.served.set(self.served.get() + 1);
        println!("第 {} 杯：{drink}", self.served.get());
    }
}

fn main() {
    let kiosk = Kiosk {
        served: Cell::new(0),
    };
    kiosk.serve("紅茶");
    kiosk.serve("咖啡");
    println!("共 {} 杯", kiosk.served.get());
}
```

執行結果：

```text
第 1 杯：紅茶
第 2 杯：咖啡
共 2 杯
```

- `Cell::new(0)` 把值包起來。`get()` 回傳裡面值的**複本**，`set(新值)` 把新值放進去，舊的丟掉。
- `serve` 只拿 `&self`，卻改得動 `served`，這就是內部可變性。
- 整個過程沒有借出任何參照：`get`、`set` 一下子就做完，不會有「還沒還」的借用，所以不會像 `RefCell` 那樣 panic，也沒有借用計數的成本。

### `get` 只給 `Copy` 的值

`get()` 要複製一份出來，所以裡面的型別得是 `Copy`（第 3 章第 8 集）。`String` 不是：

```rust,compile_fail=E0599
use std::cell::Cell;

fn main() {
    let name = Cell::new(String::from("紅茶"));
    println!("{}", name.get());
}
```

編譯錯誤：

```text
error[E0599]: the method `get` exists for struct `Cell<String>`, but its trait bounds were not satisfied
 --> src/main.rs:5:25
  |
5 |     println!("{}", name.get());
  |                         ^^^
  |
  = note: the following trait bounds were not satisfied:
          `String: Copy`
```

錯誤說 `String: Copy` 沒有滿足。不是 `Copy` 的值，`Cell` 提供另外兩個方法：`replace(新值)` 放進新值、拿回舊值；`take()` 拿走裡面的值，換成預設值（`String` 的預設值是空字串）：

```rust
use std::cell::Cell;

fn main() {
    let name = Cell::new(String::from("紅茶"));
    let old = name.replace(String::from("咖啡"));
    println!("換掉了 {old}");
    let current = name.take();
    println!("拿走了 {current}");
    let rest = name.take();
    println!("剩下 [{rest}]");
}
```

執行結果：

```text
換掉了 紅茶
拿走了 咖啡
剩下 []
```

### `Cell` 還是 `RefCell`

|  | `Cell<T>` | `RefCell<T>` |
| --- | --- | --- |
| 怎麼用 | `get`、`set`、`replace`、`take`：整個值進出 | `borrow`、`borrow_mut`：借出參照 |
| 能借出裡面的參照嗎 | 不能 | 能（`Ref`、`RefMut`） |
| 違反規則 | 不可能：根本沒有借用 | 執行時 panic |
| 適合 | 計數、旗標、編號這種小的 `Copy` 值 | `Vec`、`String`、struct 這種要就地修改的值 |

只是要替一個數字加 1，用 `Cell`；要對 `Vec` 做 `push`，用 `RefCell`。兩個都只能在單一執行緒裡用（原因是第 14 章第 11 集的 `Sync`）；跨執行緒的計數器要用第 14 章第 12 集的 atomic。

## 重點整理

- `Cell<T>`：用 `set` 放入、`get` 取出複本，只拿 `&self` 就能改，不借用，所以不會 panic。
- `get` 要求 `T: Copy`（E0599）；不是 `Copy` 的值用 `replace`（放新拿舊）或 `take`（拿走、留預設值）。
- `Cell` 不能借出裡面的參照；要就地修改 `Vec`、`String`、struct，用 `RefCell`。
- 計數、旗標這種小的 `Copy` 值用 `Cell`，其他情況用 `RefCell`；兩個都只能單一執行緒。
