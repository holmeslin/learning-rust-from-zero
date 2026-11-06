# `uuid`

## 本集目標

用 `uuid` 產生與解析 UUID：`Uuid::new_v4()`、`parse_str`、各種輸出格式，並知道 `Uuid` 能直接拿來當 `HashMap` 的鍵。

## 正文

UUID 是 128 位元的識別碼，隨機產生時幾乎不會重複，所以不需要中央資料庫發號碼，訂單編號、檔案名稱、請求編號常常用它。標準函式庫沒有，加 `uuid` crate，並打開產生隨機 UUID 的 feature `v4`（第 12 章第 15 集的 feature）：

```toml
[dependencies]
uuid = { version = "1.29.0", features = ["v4"] }
```

### 產生隨機的 UUID

`Uuid::new_v4()` 產生第 4 版（完全隨機）的 UUID。每次的值都不同，所以這裡只看它不變的性質：

```rust
use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();
    let text = id.to_string();
    println!("{}", text.len());
    let parts: Vec<usize> = text.split('-').map(|p| p.len()).collect();
    println!("{parts:?}");
    println!("{:?} {}", id.get_version(), id.is_nil());
    println!("{}", id != Uuid::new_v4());
    println!("{} {}", id.simple().to_string().len(), id.as_bytes().len());
}
```

執行結果：

```text
36
[8, 4, 4, 4, 12]
Some(Random) false
true
32 16
```

- 印出來是 36 個字元，分成 8-4-4-4-12 五段十六進位數字，像 `67e55044-10b1-426f-9247-bb680e5fe0c8`。
- `get_version()` 是 `Some(Random)`，也就是第 4 版；`is_nil()` 檢查是不是全 0 的特殊值。
- `simple()` 是不含連字號的 32 字元寫法，`as_bytes()` 是原本的 16 個位元組。
- 兩次產生的值幾乎一定不同，但「幾乎」不是「一定」：128 位元裡有 122 位元是隨機的，實務上不用擔心撞號。

### 解析與固定的值

收到別人傳來的 UUID（網址、JSON、資料庫）要用 `Uuid::parse_str`，它也接受不含連字號的寫法，大小寫都行，輸出一律是小寫加連字號：

```rust
use uuid::Uuid;

fn main() {
    let id = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").expect("不是合法的 UUID");
    println!("{id}");
    println!("{}", id.simple());
    println!("{}", id.as_u128());
    println!("{}", Uuid::from_u128(1));
    println!("{}", Uuid::nil());

    for text in [
        "67E5504410B1426F9247BB680E5FE0C8",
        "不是 uuid",
        "67e55044-10b1",
    ] {
        match Uuid::parse_str(text) {
            Ok(id) => println!("{id}"),
            Err(e) => println!("{e}"),
        }
    }
}
```

執行結果：

```text
67e55044-10b1-426f-9247-bb680e5fe0c8
67e5504410b1426f9247bb680e5fe0c8
138101147531900207301164854559698313416
00000000-0000-0000-0000-000000000001
00000000-0000-0000-0000-000000000000
67e55044-10b1-426f-9247-bb680e5fe0c8
invalid character: found `不` at 0
invalid group count: expected 5, found 2
```

- `from_u128` 與 `as_u128` 在 `Uuid` 與一個 128 位元整數之間轉換，寫測試時用 `Uuid::from_u128(1)` 就能得到固定的 UUID。`Uuid::nil()` 是全 0，`Uuid::max()` 是全 f。
- 大小寫、有沒有連字號都能解析（輸出一律小寫加連字號）。`parse_str` 回傳 `Result`，不合法時訊息會指出哪裡不對（英文）。使用者傳來的字串一定要這樣驗證，不要直接相信。

### 當作鍵

`Uuid` 實作了 `Copy`、`Eq`、`Hash`、`Ord`（第 8 章第 10 集），所以能放進 `HashMap`、`HashSet`、`BTreeMap`，也能直接比較：

```rust
use std::collections::HashMap;
use uuid::Uuid;

fn main() {
    let a = Uuid::from_u128(1);
    let b = Uuid::from_u128(2);
    let mut orders: HashMap<Uuid, &str> = HashMap::new();
    orders.insert(a, "紅茶");
    orders.insert(b, "咖啡");
    println!("{:?} {:?}", orders.get(&a), orders.get(&Uuid::from_u128(3)));
    println!("{}", a < b);
}
```

執行結果：

```text
Some("紅茶") None
true
```

因為 `Copy`，`insert(a, …)` 之後還能繼續用 `a`。第 4 版是隨機的，大小順序沒有意義，**不能用來推測誰先建立**；要按時間排序的 UUID 是第 7 版（需要 feature `v7`），本書不展開。

## 重點整理

- `uuid = { version = "1.29.0", features = ["v4"] }`；`Uuid::new_v4()` 產生隨機的第 4 版 UUID，輸出 36 個字元（8-4-4-4-12），`simple()` 是不含連字號的 32 字元。
- `Uuid::parse_str(文字)` 回傳 `Result`，大小寫、有無連字號都接受；`from_u128`、`as_u128`、`nil()` 提供固定的值，適合測試。
- `Uuid` 是 `Copy`、`Eq`、`Hash`、`Ord`，可以當 `HashMap` 的鍵；第 4 版的順序沒有意義。
