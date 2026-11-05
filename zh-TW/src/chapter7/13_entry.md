# `entry` 與 `or_insert`

## 本集目標

用 `entry(鍵).or_insert(初始值)` 處理「沒有就放進去、有就用它」，寫出計數與分組；並用 `or_default()` 取代手寫的初始值。

## 正文

### 計數：`entry` 加 `or_insert`

數每種飲料各點了幾杯。每次要做的事是：這個鍵還沒有，就先放 0；然後把它加 1。

```rust
use std::collections::HashMap;

fn main() {
    let orders = ["紅茶", "咖啡", "紅茶", "紅茶", "咖啡"];
    let mut counts: HashMap<String, u32> = HashMap::new();
    for order in orders {
        *counts.entry(String::from(order)).or_insert(0) += 1;
    }
    println!("{:?}", counts.get("紅茶"));
    println!("{:?}", counts.get("咖啡"));
    println!("{}", counts.len());
}
```

執行結果：

```text
Some(3)
Some(2)
2
```

- `counts.entry(鍵)` 找到這個鍵在 `HashMap` 裡的位置，可能已經有值，也可能還沒有。`entry` 會拿走鍵，所以傳的是 `String::from(order)`。
- `.or_insert(0)`：這個鍵還沒有值就放進 0；已經有值就不動它。兩種情況都會回傳指向這個值的 `&mut u32`。
- 回傳的是 `&mut`，要改裡面的數字，前面加 `*`（第 4 章第 7 集），再用 `+= 1` 加 1。
- 不用 `entry` 的話，要先 `get` 看有沒有、再 `insert`，等於查兩次。

`entry` 還有帶閉包的版本（`or_insert_with`、`and_modify`），第 11 章第 19 集再學。

### `or_default`：用預設值

初始值如果是「這個型別的預設值」，可以寫 `or_default()`，不用自己填：數字的預設是 0，`Vec` 是空的，`String` 是空字串。下面把成員依城市分組，值是 `Vec<String>`：

```rust
use std::collections::HashMap;

fn main() {
    let members = [("小明", "台北"), ("小華", "高雄"), ("小美", "台北")];
    let mut by_city: HashMap<String, Vec<String>> = HashMap::new();
    for (name, city) in members {
        by_city
            .entry(String::from(city))
            .or_default()
            .push(String::from(name));
    }
    println!("{:?}", by_city.get("台北"));
    println!("{:?}", by_city.get("高雄"));
}
```

執行結果：

```text
Some(["小明", "小美"])
Some(["小華"])
```

- `for (name, city) in members` 的 `(name, city)` 是模式，把每個 tuple 拆開，跟第 2 章第 10 集的 `let (a, b) = ...` 是同一回事。
- `or_default()` 在鍵不存在時放進空的 `Vec`，然後回傳 `&mut Vec<String>`，所以後面可以直接呼叫 `push`。
- 上一個例子的 `or_insert(0)` 也能寫成 `or_default()`，效果一樣。

### 跟 Go 比一比

Go 的零值讓 `counts[word]++` 直接可用，鍵不存在就從 0 開始。Rust 沒有零值，也沒有這種寫法：

```rust,compile_fail=E0594
use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<String, u32> = HashMap::new();
    counts.insert(String::from("紅茶"), 1);
    counts["紅茶"] += 1;
    println!("{:?}", counts.get("紅茶"));
}
```

編譯錯誤：

```text
error[E0594]: cannot assign to data in an index of `HashMap<String, u32>`
 --> src/main.rs:6:5
  |
6 |     counts["紅茶"] += 1;
  |     ^^^^^^^^^^^^^^^^^^^ cannot assign
  |
  = help: trait `IndexMut` is required to modify indexed content, but it is not implemented for `HashMap<String, u32>`
  = help: to modify a `HashMap<String, u32>`, use `.get_mut()`, `.insert()` or the entry API
```

`HashMap` 的 `map[鍵]` 只能讀，不能拿來改值，所以寫 `counts["紅茶"] += 1` 是 E0594。Go 程式碼直接照搬會碰到這個，改用 `entry` 加 `or_insert`。

## 重點整理

- `map.entry(鍵).or_insert(初始值)`：鍵還沒有值就放進初始值，已有就不動；回傳指向值的 `&mut V`。
- 計數寫成 `*map.entry(鍵).or_insert(0) += 1`；`entry` 會拿走鍵。
- `or_default()` 用型別的預設值當初始值（數字 0、空的 `Vec`、空字串），適合分組：`map.entry(鍵).or_default().push(值)`。
- `or_insert_with`、`and_modify` 要用閉包，第 11 章第 19 集才學。
- 跟 Go 比：Go 靠零值寫 `m[k]++`；Rust 的 `map[鍵]` 只能讀，`counts["紅茶"] += 1` 是 E0594，要用 `entry`。
