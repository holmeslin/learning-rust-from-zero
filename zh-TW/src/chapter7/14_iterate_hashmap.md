# 走訪 `HashMap`（順序不固定）

## 本集目標

用 `for (鍵, 值) in &map` 走訪 `HashMap`，用 `keys()`、`values()` 只取一半，用 `&mut map` 修改值，並記住：走訪的順序每次都可能不同。

## 正文

### `for (鍵, 值) in &map`

走訪 `HashMap` 跟走訪 `Vec` 一樣，加 `&` 借用就好。每一圈拿到的是一組 `(鍵的參照, 值的參照)`，用模式拆開，跟上一集 `for (name, city) in members` 是同一種寫法：

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert(String::from("紅茶"), 10);
    stock.insert(String::from("咖啡"), 4);
    stock.insert(String::from("奶茶"), 7);
    for (name, count) in &stock {
        println!("{name}：{count} 杯");
    }
}
```

某一次的輸出（順序每次可能不同）：

```text
咖啡：4 杯
紅茶：10 杯
奶茶：7 杯
```

這個程式連跑幾次，三行出現的順序有可能不一樣，所以上面只貼其中一次的輸出。

### 順序不固定

`HashMap` 不保證走訪的順序，不同次執行、甚至同一個程式裡的兩個 `HashMap`，順序都可能不同。標準函式庫刻意在每次執行時換一個隨機的種子，避免有人刻意挑選會讓 `HashMap` 變慢的鍵。Go 的 map 走訪順序也是不固定的。

所以程式不能依賴順序。做法有兩種：

- 結果跟順序無關（加總、計數、找最大），照樣走訪。
- 真的要固定的順序，用第 16 集的 `BTreeMap`，或是第 17 集把鍵排序之後再處理。

### `keys()` 與 `values()`

只需要鍵或只需要值，用 `keys()`、`values()`，一樣直接放在 `for` 後面。下面的結果都跟順序無關：

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert(String::from("紅茶"), 10);
    stock.insert(String::from("咖啡"), 4);
    stock.insert(String::from("奶茶"), 7);

    let mut total = 0;
    for count in stock.values() {
        total += count;
    }
    println!("共 {total} 杯");

    let mut tea = 0;
    for name in stock.keys() {
        if name.contains("茶") {
            tea += 1;
        }
    }
    println!("{tea} 種茶");
}
```

執行結果：

```text
共 21 杯
2 種茶
```

### `for (鍵, 值) in &mut map`

要改值，改成借出 `&mut`，`map` 要 `let mut`，值變成 `&mut u32`，改它要用 `*`。鍵永遠只能讀，不能改：

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert(String::from("紅茶"), 10);
    stock.insert(String::from("咖啡"), 4);
    for (name, count) in &mut stock {
        if name.contains("茶") {
            *count += 5;
        }
    }
    println!("{:?}", stock.get("紅茶"));
    println!("{:?}", stock.get("咖啡"));
}
```

執行結果：

```text
Some(15)
Some(4)
```

`for x in map` 不加 `&` 的話，跟 `Vec` 一樣會把整個 `HashMap` move 進迴圈，之後不能再用。

## 重點整理

- `for (鍵, 值) in &map` 走訪 `HashMap`，每一圈是 `(&K, &V)`，用模式拆開；`&mut map` 時值是 `&mut V`，用 `*` 改。
- `map.keys()`、`map.values()` 只走訪鍵或值，可以直接放在 `for` 後面。
- 走訪順序不固定，每次執行都可能不同（Go 的 map 也是）；結果要跟順序無關，或改用 `BTreeMap` 或排序。
- `for x in map` 會 move 整個 `HashMap`。
