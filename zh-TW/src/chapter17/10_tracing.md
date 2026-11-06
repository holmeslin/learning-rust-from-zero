# `tracing`

## 本集目標

用 `tracing` 記錄程式的事件與工作範圍：`info!`、`warn!` 等巨集、帶欄位的事件、`span` 與 `#[instrument]`；並知道 `tracing` 只負責「發出」，要有 subscriber 才看得到。

## 正文

### 為什麼不只用 `println!`

`println!` 沒辦法依層級篩選（正式上線只留警告）、沒辦法附上結構化的欄位、也不知道這行是在哪個請求裡印的。`tracing` 把「記錄」拆成兩邊：程式碼用巨集**發出**事件，另一個叫 subscriber 的東西決定要**留下**哪些、**怎麼印**。這是跟第 8 集 `serde` 一樣的分工：

```toml
[dependencies]
tracing = "0.1.44"
```

最容易讓人以為壞掉的地方：**沒有 subscriber，什麼都不會印**。

```rust
use tracing::info;

fn main() {
    info!("沒有 subscriber，這行不會印出來");
    println!("結束");
}
```

執行結果：

```text
結束
```

### 事件、層級、欄位、span

下面是完整的程式，最上面是你要學的使用端，最下面的 subscriber 先照抄（實務上用 `tracing-subscriber` 這個 crate 一行搞定，本書的 crate 清單沒有它，所以這裡自己寫一個很短的）：

```rust
use std::fmt::Debug;
use std::sync::Mutex;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Level, Metadata, Subscriber};
use tracing::{debug, info, info_span, instrument, warn};

#[instrument]
fn brew(drink: &str, cups: u32) {
    info!("開始沖泡 {drink}");
    debug!(step = "燒水", "這行會被濾掉");
    if cups > 5 {
        warn!(cups, "一次超過 5 杯");
    }
}

fn main() {
    tracing::subscriber::set_global_default(Printer::default()).expect("只能設定一次");

    info!(drink = "紅茶", price = 30, "收到訂單");
    let span = info_span!("closing", shop = "信義店");
    let _guard = span.enter();
    brew("紅茶", 2);
    brew("咖啡", 8);
}

// 以下是 subscriber：決定留下哪些事件、怎麼印。先照抄，不用看懂。
#[derive(Default)]
struct Fields(String);

impl Visit for Fields {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        if !self.0.is_empty() {
            self.0.push(' ');
        }
        if field.name() == "message" {
            self.0.push_str(&format!("{value:?}"));
        } else {
            self.0.push_str(&format!("{}={value:?}", field.name()));
        }
    }
}

#[derive(Default)]
struct Printer {
    spans: Mutex<Vec<String>>,
    stack: Mutex<Vec<String>>,
}

impl Subscriber for Printer {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        *metadata.level() <= Level::INFO
    }

    fn new_span(&self, attrs: &Attributes<'_>) -> Id {
        let mut fields = Fields::default();
        attrs.record(&mut fields);
        let mut spans = self.spans.lock().expect("鎖中毒");
        spans.push(format!("{}({})", attrs.metadata().name(), fields.0));
        Id::from_u64(spans.len() as u64)
    }

    fn record(&self, _: &Id, _: &Record<'_>) {}

    fn record_follows_from(&self, _: &Id, _: &Id) {}

    fn event(&self, event: &Event<'_>) {
        let mut fields = Fields::default();
        event.record(&mut fields);
        let path = self.stack.lock().expect("鎖中毒").join(" > ");
        println!("{:5} [{path}] {}", event.metadata().level(), fields.0);
    }

    fn enter(&self, id: &Id) {
        let span = self.spans.lock().expect("鎖中毒")[id.into_u64() as usize - 1].clone();
        self.stack.lock().expect("鎖中毒").push(span);
    }

    fn exit(&self, _: &Id) {
        self.stack.lock().expect("鎖中毒").pop();
    }
}
```

執行結果：

```text
INFO  [] 收到訂單 drink="紅茶" price=30
INFO  [closing(shop="信義店") > brew(drink="紅茶" cups=2)] 開始沖泡 紅茶
INFO  [closing(shop="信義店") > brew(drink="咖啡" cups=8)] 開始沖泡 咖啡
WARN  [closing(shop="信義店") > brew(drink="咖啡" cups=8)] 一次超過 5 杯 cups=8
```

- **事件**：`info!`、`warn!`，還有 `error!`、`debug!`、`trace!`，用法像 `println!`，最後一個字串是訊息（跟 `println!` 一樣能帶 `{drink}` 這種格式引數），前面可以放 `名稱 = 值` 的欄位（結構化資料）。`warn!(cups, "…")` 是 `cups = cups` 的簡寫。層級由嚴重到瑣碎是 `error`、`warn`、`info`、`debug`、`trace`。
- **篩選**：subscriber 的 `enabled` 說「INFO 以上才留」，所以 `debug!` 那一行沒有出現。換一個 subscriber，不必改任何 `info!`。
- **span**：一段有開始有結束的工作。`info_span!("closing", shop = …)` 建立，`span.enter()` 進入，回傳的 guard 離開作用域就離開（第 13 章第 4 集的 `Drop`）。在 span 裡發出的事件會帶著這個 span 的上下文：輸出中括號裡就是目前所在的 span，最外層沒有 span 所以是 `[]`。
- **`#[instrument]`**：加在函式上，每次呼叫自動建立一個以函式名稱命名的 span，並把參數記成欄位（參數要能用 `{:?}` 印），所以不用手動寫 `info_span!`。
- `set_global_default` 只能呼叫一次，通常放在 `main` 的第一行。

### 跟 Go 比一比

```go
slog.Info("收到訂單", "drink", "紅茶", "price", 30)
// 2026/11/06 10:28:01 INFO 收到訂單 drink=紅茶 price=30
```

- Go 的 `log/slog` 沒有設定也會印（預設印到 stderr、INFO 以上、帶時間）；Rust 的 `tracing` 沒有 subscriber 就是靜默，這是最常見的「怎麼沒有 log」。
- Go 的 `slog` 沒有 span：同一個請求的多行 log 要自己帶 request id；`tracing` 用 span，範圍內每個事件都會自動帶上下文。
- Go 的 `context` 要一路傳進函式；`tracing` 的目前 span 記在執行緒裡，函式簽名不用多一個參數。

## 重點整理

- `tracing` 負責發出事件與 span，subscriber 決定留下哪些、怎麼印；沒有 subscriber 什麼都不會印；`set_global_default` 只能設定一次。
- `info!`、`warn!`、`error!`、`debug!`、`trace!` 發出事件，用 `名稱 = 值` 附欄位；層級由 subscriber 篩選。
- `info_span!` 加 `span.enter()` 建立並進入 span，guard 離開作用域就離開；`#[instrument]` 讓函式每次呼叫自動成為一個 span，參數記成欄位。
- 實務上用 `tracing-subscriber` 當 subscriber，本書不展開，這集的 subscriber 是為了讓範例能執行。
