# `sort_by_key` 與 `sort_by`

## 本集目標

用 `sort_by_key` 依某個欄位排序、用 `sort_by` 自己決定怎麼比，兩個都靠閉包；並用 `sort_by` 排浮點數，兌現第 7、8 章的伏筆。

## 正文

### `sort_by_key`：依某個值排

第 7 章第 17 集的 `sort()` 只能照元素本身的大小排。想依 struct 的某個欄位排，用 `sort_by_key`，閉包拿到一個元素，回傳要拿來比的「鍵」：

```rust
struct Member {
    name: String,
    points: u32,
}

impl Member {
    fn new(name: &str, points: u32) -> Member {
        Member {
            name: String::from(name),
            points,
        }
    }
}

fn main() {
    let mut members = Vec::new();
    members.push(Member::new("小明", 120));
    members.push(Member::new("小華", 80));
    members.push(Member::new("小美", 200));

    members.sort_by_key(|m| m.points);
    for m in &members {
        println!("{} {} 點", m.name, m.points);
    }

    members.sort_by_key(|m| std::cmp::Reverse(m.points));
    println!("{}", members[0].name);
}
```

執行結果：

```text
小華 80 點
小明 120 點
小美 200 點
小美
```

- 鍵必須能完整比大小，也就是 `Ord`（第 8 章第 12 集）：整數、字串、tuple 都可以。
- `std::cmp::Reverse(...)` 是把順序反過來的包裝，所以第二次是由大到小，最高點的小美排第一。
- 排序是穩定的：鍵相同的元素，維持原本的先後。

鍵不能是借用元素的參照。想依姓名排，直覺會寫 `|m| &m.name`：

```rust,compile_fail
struct Member {
    name: String,
}

fn main() {
    let mut members = Vec::new();
    members.push(Member {
        name: String::from("小明"),
    });
    members.sort_by_key(|m| &m.name);
}
```

編譯錯誤：

```text
error: lifetime may not live long enough
  --> src/main.rs:10:29
   |
10 |     members.sort_by_key(|m| &m.name);
   |                          -- ^^^^^^^ returning this value requires that `'1` must outlive `'2`
   |                          ||
   |                          |return type of closure is &'2 String
   |                          has type `&'1 Member`
```

閉包的參數 `m` 是 `&Member`（生命週期 `'1`），回傳的鍵卻是 `&String`（生命週期 `'2`），編譯器無法保證鍵比 `m` 活得久。這個錯誤沒有錯誤碼。改成 `m.name.clone()` 可以，但每個元素都複製一份字串，下面的 `sort_by` 更省。

### `sort_by`：自己說怎麼比

`sort_by` 的閉包拿到兩個元素的參照 `a`、`b`，回傳 `Ordering`（第 8 章第 12 集：`Less`、`Equal`、`Greater`）。字串欄位直接 `cmp`，不用複製：`members.sort_by(|a, b| a.name.cmp(&b.name))`，把會員依姓名排好。

### 排浮點數

第 7 章第 17 集與第 8 章第 12 集都說過：`f64` 不是 `Ord`，不能當鍵，`sort()` 也排不了。用 `sort_by`，自己決定怎麼比：`partial_cmp` 回傳 `Option<Ordering>`（第 8 章第 12 集），用 `expect` 說明「我保證沒有 NaN」。由大到小，把 `a`、`b` 對調：

```rust
fn main() {
    let mut scores: Vec<f64> = vec![4.5, 3.0, 4.8];
    scores.push(3.9);
    scores.sort_by(|a, b| a.partial_cmp(b).expect("分數不是 NaN"));
    println!("{scores:?}");
    scores.sort_by(|a, b| b.partial_cmp(a).expect("分數不是 NaN"));
    println!("{scores:?}");
}
```

執行結果：

```text
[3.0, 3.9, 4.5, 4.8]
[4.8, 4.5, 3.9, 3.0]
```

遇到 NaN 就 panic。想讓 NaN 也有固定的位置、不 panic，`f64` 還有 `total_cmp`：`scores.sort_by(|a, b| a.total_cmp(b))`。

### 跟 Go 比一比

Go 的 `sort.Slice` 閉包拿到的是索引，回傳 `bool`（`i` 要不要排在 `j` 前面），而且不保證穩定：

```go
sort.Slice(members, func(i, j int) bool { return members[i].Points < members[j].Points })
```

Rust 的閉包直接拿到元素，不用再去查陣列；`sort_by_key` 連「比較」都不用寫，只要給出鍵。`sort`、`sort_by`、`sort_by_key` 都是穩定排序。

## 重點整理

- `sort_by_key(|x| 鍵)` 依鍵排序，鍵要是 `Ord`；由大到小用 `std::cmp::Reverse(鍵)`。
- 鍵不能借用元素：`|m| &m.name` 是沒有錯誤碼的生命週期錯誤，要 `clone()`，或改用 `sort_by`。
- `sort_by(|a, b| ...)` 的閉包拿到兩個參照，回傳 `Ordering`；字串欄位用 `a.name.cmp(&b.name)`。
- 浮點數：`sort_by(|a, b| a.partial_cmp(b).expect("..."))`；由大到小把 `a`、`b` 對調；`total_cmp` 不用處理 NaN。
- 跟 Go 比：Go 的閉包拿索引、回傳 `bool`，且不穩定；Rust 拿元素、回傳 `Ordering`，且穩定。
