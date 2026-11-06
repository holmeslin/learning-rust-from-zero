# derive `Serialize` / `Deserialize`

## 本集目標

用 `#[derive(Serialize, Deserialize)]` 讓自己的 struct 與 enum 能轉成 JSON、再轉回來；知道欄位怎麼對應、解析失敗的錯誤長什麼樣，並會用 `#[serde(...)]` 調整對應規則。

## 正文

### 一行 derive，雙向轉換

第 16 章第 4 集說過，`#[derive(Serialize)]` 是程序巨集，替型別產生實作 trait 的程式碼。這次要把 `serde` 的 `derive` 功能打開：

```toml
[dependencies]
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.154"
```

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Size {
    S,
    M,
    L,
}

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    id: u32,
    drink: String,
    size: Size,
    toppings: Vec<String>,
    note: Option<String>,
}

fn main() {
    let order = Order {
        id: 1,
        drink: String::from("紅茶"),
        size: Size::M,
        toppings: vec![String::from("珍珠"), String::from("椰果")],
        note: None,
    };
    let text = serde_json::to_string(&order).expect("轉換失敗");
    println!("{text}");

    let back: Order = serde_json::from_str(&text).expect("解析失敗");
    println!("{back:?}");
}
```

執行結果：

```text
{"id":1,"drink":"紅茶","size":"M","toppings":["珍珠","椰果"],"note":null}
Order { id: 1, drink: "紅茶", size: M, toppings: ["珍珠", "椰果"], note: None }
```

- 型別裡**每一個**欄位的型別都要能轉換：`u32`、`String`、`Vec<String>`、`Option<String>`、`Size` 都行；有一個沒實作就是編譯錯誤。
- struct 變成 JSON 物件，鍵的順序就是欄位宣告的順序（`Value` 才是依字母排序）；`Vec` 變陣列；`None` 變 `null`；沒有資料的 variant（`Size::M`）變成字串 `"M"`。帶資料的 variant 多一層，`Percent(10)` 變成 `{"Percent":10}`，鍵是 variant 的名字。
- 轉回型別時，用型別標註 `let back: Order` 告訴 `from_str` 目標；上一集的 `Value` 是同一個函式的另一個目標。

### 解析失敗：欄位對不上

型別是固定的，JSON 對不上就是 `Err`，訊息會指出哪個欄位、位置在哪：

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Item {
    drink: String,
    price: u32,
    note: Option<String>,
}

fn main() {
    let inputs = [
        r#"{"drink": "紅茶", "price": 30}"#,
        r#"{"drink": "紅茶"}"#,
        r#"{"drink": "紅茶", "price": "三十"}"#,
    ];
    for text in inputs {
        match serde_json::from_str::<Item>(text) {
            Ok(item) => println!("{} {} {:?}", item.drink, item.price, item.note),
            Err(e) => println!("錯誤：{e}"),
        }
    }
}
```

執行結果：

```text
紅茶 30 None
錯誤：missing field `price` at line 1 column 19
錯誤：invalid type: string "三十", expected u32 at line 1 column 37
```

- 第一筆沒有 `note` 也成功：`Option` 欄位缺了就是 `None`。其他型別的欄位缺了是錯誤（第二筆 `missing field`）。
- 第三筆型別不對，是錯誤，不會悄悄轉換（`-5` 放不進 `u32` 也一樣）。
- 另外，JSON 裡多出型別沒有的欄位（例如 `"cups": 2`），**預設會直接忽略**，不是錯誤。

### `#[serde(...)]` 調整對應規則

欄位名稱、預設值這類規則，用 `#[serde(...)]` 屬性調整：

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Order {
    drink_name: String,
    #[serde(default)]
    cup_count: u32,
    #[serde(rename = "type")]
    kind: String,
}

fn main() {
    let order = Order {
        drink_name: String::from("紅茶"),
        cup_count: 2,
        kind: String::from("外帶"),
    };
    println!("{}", serde_json::to_string(&order).expect("轉換失敗"));

    let text = r#"{"drinkName": "咖啡", "type": "內用"}"#;
    let back: Order = serde_json::from_str(text).expect("解析失敗");
    println!("{back:?}");
}
```

執行結果：

```text
{"drinkName":"紅茶","cupCount":2,"type":"外帶"}
Order { drink_name: "咖啡", cup_count: 0, kind: "內用" }
```

- `rename_all = "camelCase"` 把所有欄位換成 JSON 慣用的駝峰命名，Rust 這邊仍然寫 `snake_case`。
- `rename = "type"` 替單一欄位改名，因為 `type` 是 Rust 的關鍵字，不能當欄位名稱。
- `default` 讓缺少的欄位用型別的預設值（`u32` 是 0），不再是錯誤。
- `skip_serializing_if = "Option::is_none"` 讓 `None` 的欄位不輸出，而不是印 `null`；`deny_unknown_fields` 讓多餘的欄位也變成錯誤（``unknown field `cups`, expected `drink` ``）。兩個都是加在欄位或型別上的 `#[serde(...)]`，寫法跟上面一樣。

### 跟 Go 比一比

Go 用 struct tag（`` `json:"drink"` ``）對應名稱，行為卻寬鬆得多（Go 1.27 實測）：

- JSON 少了 `price`，`Unmarshal` 沒有錯誤，`Price` 悄悄是 0；Rust 缺欄位是錯誤，除非是 `Option` 或加了 `default`。
- Go 鍵名比對不分大小寫（`"DRINK"` 也對得上 `drink`），Rust 一定要完全相同；Go 小寫開頭的欄位不是公開的，轉換時悄悄略過，Rust 的欄位可見性不影響 derive。

## 重點整理

- `serde` 要開 `derive` 功能：`serde = { version = "1.0.229", features = ["derive"] }`；`#[derive(Serialize, Deserialize)]` 讓 struct、enum 與 JSON 互轉，欄位型別都要能轉換。
- struct 變物件（鍵依欄位宣告順序）、`Vec` 變陣列、`None` 變 `null`、沒資料的 variant 變字串、帶資料的 variant 變 `{"名字": 資料}`。
- 反序列化對不上就是 `Err`：缺欄位（`Option` 例外）、型別不符、放不進的數字；預設忽略多餘的欄位。
- `#[serde(rename_all)]`、`rename`、`default`、`skip_serializing_if`、`deny_unknown_fields` 調整對應規則。
- 跟 Go 比：Go 缺欄位悄悄是零值、鍵名不分大小寫、小寫欄位被略過；Rust 都明確得多。
