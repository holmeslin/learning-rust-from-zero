# `or_insert_with` 與 `and_modify`

## 本集目標

用 `and_modify(閉包)` 在鍵已經存在時修改它的值，用 `or_insert_with(閉包)` 在鍵不存在時才現場產生初始值，並分清楚什麼時候用它們、什麼時候 `or_insert` 與 `or_default` 就夠了。

## 正文

### `and_modify`：已經有，就改它

第 7 章第 13 集的計數寫成 `*counts.entry(鍵).or_insert(0) += 1`：先放 0，再加 1。`and_modify` 讓兩種情況分開寫：鍵已經有值，就用閉包修改；沒有，才用後面的 `or_insert` 放進初始值：

```rust
use std::collections::HashMap;

fn main() {
    let orders = ["紅茶", "咖啡", "紅茶", "紅茶", "咖啡"];
    let mut counts: HashMap<String, u32> = HashMap::new();
    for order in orders {
        counts
            .entry(String::from(order))
            .and_modify(|n| *n += 1)
            .or_insert(1);
    }
    println!("{:?}", counts.get("紅茶"));
    println!("{:?}", counts.get("咖啡"));
}
```

執行結果：

```text
Some(3)
Some(2)
```

- `and_modify` 的閉包拿到值的 `&mut`（第 4 章第 3 集），所以用 `*n += 1` 改；鍵不存在時，閉包不會執行。
- `and_modify` 回傳的還是 `entry`，所以後面可以接著 `or_insert(1)`：鍵不存在，放進 1。
- 這個例子用 `or_insert(0)` 加 1 結果一樣。`and_modify` 適合「已經有」和「第一次」要用不同做法的時候，例如已經有就取較大的，第一次直接放進去。

### `or_insert_with`：沒有才現場做

`or_insert(值)` 的引數，不管鍵存不存在，都會先算好（跟上一集的 `unwrap_or(default_price())` 同一個道理）。如果初始值要花功夫做，或是做的時候有副作用，用 `or_insert_with(閉包)`：只有鍵不存在時，才會呼叫閉包拿初始值。閉包也可以直接放函式名稱：

```rust
use std::collections::HashMap;

fn start_record() -> Vec<u32> {
    println!("建立新的紀錄");
    Vec::new()
}

fn main() {
    let sales = [("紅茶", 30), ("咖啡", 45), ("紅茶", 32)];
    let mut history: HashMap<String, Vec<u32>> = HashMap::new();
    for (name, price) in sales {
        history
            .entry(String::from(name))
            .or_insert_with(start_record)
            .push(price);
    }
    println!("{:?}", history.get("紅茶"));
}
```

執行結果：

```text
建立新的紀錄
建立新的紀錄
Some([30, 32])
```

紅茶出現兩次、咖啡一次，但「建立新的紀錄」只印了兩次：只有紅茶第一次、咖啡第一次，才呼叫 `start_record`。如果寫成 `or_insert(start_record())`，每一圈都會先呼叫，印三次。

### 什麼時候用哪一個

| 想要的初始值 | 寫法 |
| --- | --- |
| 寫死的簡單值（`0`、`1`） | `or_insert(0)` |
| 型別的預設值（`0`、空的 `Vec`、空字串） | `or_default()`（第 7 章第 13 集） |
| 要計算、要呼叫函式、有副作用 | `or_insert_with(閉包)` |
| 已經有和第一次做法不同 | `and_modify(閉包)` 接 `or_insert(...)` |

如果只是想要空的 `Vec`，`or_insert_with(Vec::new)` 能用，但 clippy 會建議直接寫 `or_default()`，更短。

## 重點整理

- `entry(鍵).and_modify(|v| ...)`：鍵已經有值時，用閉包改它（`&mut V`）；鍵不存在，閉包不執行。回傳的還是 `entry`，所以能接 `or_insert`。
- `entry(鍵).or_insert_with(閉包)`：只有鍵不存在才呼叫閉包產生初始值；`or_insert(值)` 的引數則一定會先算。
- 型別的預設值用 `or_default()`；寫死的簡單值用 `or_insert(值)`；要計算或有副作用才用 `or_insert_with`。
- `and_modify` 適合「已經有」和「第一次」做法不同的情況，例如已經有就取較大的、第一次直接放進去。
