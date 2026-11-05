# `HashSet`

## 本集目標

用 `HashSet<T>` 存「不重複的一堆值」：會 `insert`（回傳是不是新的）、`contains`、`remove`，並用它判斷有沒有重複。

## 正文

### 沒有重複的集合

`HashSet` 只存值，不存對應的值，而且同一個值只會有一份。它跟 `HashMap` 在同一個地方，也要先 `use`：

```rust
use std::collections::HashSet;

fn main() {
    let mut seen: HashSet<String> = HashSet::new();
    println!("{}", seen.insert(String::from("小明")));
    println!("{}", seen.insert(String::from("小華")));
    println!("{}", seen.insert(String::from("小明")));
    println!("{}", seen.len());
    println!("{}", seen.contains("小華"));
    println!("{}", seen.remove("小華"));
    println!("{}", seen.len());
}
```

執行結果：

```text
true
true
false
2
true
true
1
```

- `insert` 回傳 `bool`：`true` 表示這個值是新放進去的，`false` 表示本來就有，這次什麼都沒改。第三行的「小明」第二次放進去，所以是 `false`。
- `len()` 是不重複的值有幾個：放了三次、其中兩次是同一個值，所以是 2。
- `contains(&值)`、`remove(&值)` 的參數是參照，值是 `String` 時傳 `&str` 就行。`remove` 回傳 `bool`，表示有沒有刪到東西。
- 型別 `HashSet<String>` 的 `<String>` 一樣是泛型，先當成型別名稱的一部分（第 8 章）。能放進去的型別，條件跟 `HashMap` 的鍵一樣。

### 用 `insert` 的回傳值找重複

簽到表上有人簽了兩次，要知道有幾個人、重複了幾次。每個名字都 `insert` 一次，回傳 `false` 就是重複：

```rust
use std::collections::HashSet;

fn main() {
    let signins = ["小明", "小華", "小明", "小美", "小華"];
    let mut seen: HashSet<String> = HashSet::new();
    let mut repeats = 0;
    for name in signins {
        if !seen.insert(String::from(name)) {
            repeats += 1;
        }
    }
    println!("{} 人簽到，重複 {repeats} 次", seen.len());
}
```

執行結果：

```text
3 人簽到，重複 2 次
```

### 為什麼不用 `Vec`

這個問題用 `Vec` 也能解：每次先看 `v.contains(&值)`，沒有再 `push`。差別在速度：`Vec` 的 `contains` 要從頭逐一比對，資料越多越慢；`HashSet` 用雜湊值直接找位置，不管放了多少個，查有沒有都很快。

Go 沒有內建的集合型別，大家用 `map[string]bool` 或 `map[string]struct{}` 湊。Rust 有 `HashSet`，讀程式的人一看就知道意圖。

### 順序也不固定

`HashSet` 跟 `HashMap` 一樣不保證走訪順序（第 14 集），所以範例只印 `len()`、`contains` 這些跟順序無關的結果。需要依序走訪、而且不重複，之後可以用 `Vec` 加上第 17 集的 `sort` 與 `dedup`。

## 重點整理

- `HashSet<T>` 存不重複的值；使用前先 `use std::collections::HashSet;`。
- `insert(值)` 回傳 `bool`：`true` 是新的，`false` 是本來就有；`contains(&值)` 問有沒有；`remove(&值)` 刪除，回傳有沒有刪到。
- `len()` 是不重複的值的個數。
- 判斷有沒有看過，`HashSet` 比 `Vec` 的 `contains` 快；Go 沒有內建集合型別。
- 走訪順序不固定，所以只依賴跟順序無關的結果。
