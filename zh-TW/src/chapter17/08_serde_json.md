# `serde` 與 `serde_json`

## 本集目標

分清楚 `serde`（序列化的框架）與 `serde_json`（JSON 這個格式）的分工，用 `serde_json::Value` 與 `json!` 處理形狀不固定的 JSON，並看懂解析失敗的錯誤。

## 正文

### 兩個 crate，兩層分工

第 12 章第 14 集教過怎麼加入 crate，這一集起用它們：

```toml
[dependencies]
serde_json = "1.0.154"
```

序列化（serialize）是把記憶體裡的資料轉成能存檔、能傳送的文字或位元組，反過來叫反序列化（deserialize）。Rust 把這件事拆成兩層：

```text
你的型別 <--> serde 的資料模型 <--> 某種格式
              (Serialize / Deserialize)   (serde_json：JSON)
```

`serde` 定義 `Serialize`、`Deserialize` 兩個 trait 與資料模型；`serde_json` 負責 JSON 這個格式。換成別的格式（TOML、YAML）只換格式的 crate，型別不用動。下一集替自己的型別 derive 這兩個 trait，這一集先用 `serde_json` 提供的動態型別 `Value`。

### `Value`：不知道形狀的 JSON

`serde_json::from_str` 把文字解析成 `Value`，它是 enum，每個 variant 對應 JSON 的一種值：`Null`、`Bool`、`Number`、`String`、`Array`、`Object`：

```rust
use serde_json::Value;

fn main() {
    let text = r#"{"drink": "紅茶", "price": 30, "iced": true, "toppings": ["珍珠", "椰果"], "note": null}"#;
    let order: Value = serde_json::from_str(text).expect("不是合法的 JSON");

    println!("{}", order["drink"]);
    println!("{:?}", order["drink"].as_str());
    println!("{:?}", order["price"].as_u64());
    println!("{}", order["toppings"][1]);
    println!("{}", order["note"].is_null());
    println!("{}", order["no_such_key"]);
    println!("{:?}", order["drink"].as_u64());
}
```

執行結果：

```text
"紅茶"
Some("紅茶")
Some(30)
"椰果"
true
null
None
```

- `r#"…"#` 是第 1 章第 18 集的 raw string，裡面的雙引號不用跳脫。
- 用 `["鍵"]` 取物件的值、`[數字]` 取陣列的元素，可以一路串下去。**鍵不存在不會 panic**，得到 `Value::Null`（印出來是 `null`），跟第 7 章 `HashMap` 的 `[]` 不一樣。
- `as_str()`、`as_u64()`、`as_bool()`、`as_array()` 把 `Value` 轉成真正的型別，回傳 `Option`：型別不對就是 `None`（最後一行把 `"紅茶"` 當數字）。印出 `Value` 時字串帶雙引號，因為印的是 JSON。

### 用 `json!` 建立

`json!` 巨集（第 16 章第 1 集）讓你直接寫出 JSON 的樣子，值可以是變數：

```rust
use serde_json::json;

fn main() {
    let drink = "咖啡";
    let order = json!({
        "drink": drink,
        "price": 45,
        "sizes": ["M", "L"],
        "note": null,
    });
    println!("{order}");
    let pretty = serde_json::to_string_pretty(&order).expect("轉換失敗");
    println!("{pretty}");
}
```

執行結果：

```text
{"drink":"咖啡","note":null,"price":45,"sizes":["M","L"]}
{
  "drink": "咖啡",
  "note": null,
  "price": 45,
  "sizes": [
    "M",
    "L"
  ]
}
```

`{order}` 印成一行的緊湊 JSON，`to_string_pretty` 換行縮排。注意輸出的**鍵依字母排序**，不是你寫的順序（`Value` 的物件預設用 `BTreeMap`，第 7 章第 16 集），所以輸出是固定的。取出與修改都能用 `[]`：`order["price"] = json!(30);`。

### 解析失敗

文字不是合法的 JSON 時，`from_str` 回傳 `Err(serde_json::Error)`，訊息帶行與欄：

```rust
fn main() {
    for text in ["{\"drink\": \"紅茶\",}", "[1, 2"] {
        match serde_json::from_str::<serde_json::Value>(text) {
            Ok(v) => println!("{v}"),
            Err(e) => println!("{e}"),
        }
    }
}
```

執行結果：

```text
trailing comma at line 1 column 20
EOF while parsing a list at line 1 column 5
```

JSON 不允許最後多一個逗號。錯誤的 `Display` 是英文。`::<serde_json::Value>` 是第 8 章第 8 集的寫法，告訴 `from_str` 要解析成什麼型別。

`Value` 什麼形狀都收。`Vec`、陣列、tuple、`Option` 這些標準型別也實作了 `Serialize` 與 `Deserialize`，例如 `serde_json::to_string(&["紅茶", "咖啡"])` 得到 `["紅茶","咖啡"]`。形狀已知的資料，應該直接轉成自己的型別，這是下一集的主題；`Value` 留給真的看不出形狀的資料。

### 跟 Go 比一比

```go
var v map[string]any
json.Unmarshal([]byte(`{"price": 30}`), &v)
_, ok := v["price"].(int) // false：數字一律解成 float64
```

- Go 解到 `any` 時，所有數字都是 `float64`，要 `v["price"].(float64)` 再轉；Rust 的 `Value` 分得出整數與小數：`as_u64()`、`as_i64()`、`as_f64()` 各自回傳 `Option`，30 能當 `u64` 也能當 `f64`，`1.5` 當 `u64` 是 `None`。
- 兩邊讀不存在的鍵都不會 panic（Go 是 `nil`，Rust 是 `Value::Null`），輸出物件時鍵都依字母排序。

## 重點整理

- `serde` 定義 `Serialize`／`Deserialize` 與資料模型，`serde_json` 負責 JSON 格式；`Cargo.toml` 寫 `serde_json = "1.0.154"`。
- `serde_json::from_str` 解析成 `Value`（`Null`、`Bool`、`Number`、`String`、`Array`、`Object`）；用 `["鍵"]`、`[索引]` 取值，鍵不存在得到 `Null`；`as_str()`、`as_u64()` 等回傳 `Option`。
- `json!` 直接寫出 JSON；`to_string` 輸出一行，`to_string_pretty` 換行縮排；物件的鍵依字母排序。
- 解析失敗是 `Err(serde_json::Error)`，訊息帶行與欄；`Vec`、tuple、`Option` 等標準型別也能直接序列化；形狀已知的資料，下一集直接轉成自己的型別。
- 跟 Go 比：Go 解到 `any` 數字一律 `float64`，`Value` 分得出整數與小數。
