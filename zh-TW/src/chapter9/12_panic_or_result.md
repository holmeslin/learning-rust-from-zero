# 什麼時候 panic、什麼時候回傳 `Result`

## 本集目標

學會判斷一個失敗該 `panic!` 還是回傳 `Result`，並知道 Rust 的 `Result` 加 `?` 跟 Go 的 `if err != nil` 怎麼對應。

## 正文

### 先問：這是誰的錯？

一個失敗該用哪一種，看一個問題：**出錯是預料之內，還是程式寫錯了？**

- 預料之內：使用者輸入打錯字、商品查不到、外部資料格式不對。呼叫的人有辦法處理（提醒重填、換預設值），所以回傳 `Result` 或 `Option`，把決定權交給他。
- 程式寫錯：索引越界、不可能發生的狀態、呼叫的人違反了函式事先講好的前提。繼續下去只會更糟，所以 `panic!`。

標準函式庫就是這樣分的：`v[10]` 越界會 panic，因為你自己寫的索引不該超出範圍；`v.get(10)` 回傳 `Option`，用在你不確定有沒有的時候。`parse` 回傳 `Result`，因為文字常常來自使用者。

```rust
fn split_bill(total: u32, people: u32) -> Result<u32, String> {
    if people == 0 {
        return Err(String::from("人數不能是 0"));
    }
    Ok(total / people)
}

fn main() {
    let total: u32 = "120".parse().expect("120 一定是整數");
    for text in ["4", "0", "abc"] {
        match text.parse::<u32>() {
            Ok(people) => match split_bill(total, people) {
                Ok(each) => println!("{people} 人，每人 {each} 元"),
                Err(msg) => println!("{people} 人：{msg}"),
            },
            Err(e) => println!("{text:?} 不是整數：{e}"),
        }
    }
}
```

執行結果：

```text
4 人，每人 30 元
0 人：人數不能是 0
"abc" 不是整數：invalid digit found in string
```

- `total` 是寫死的 `"120"`，不可能轉換失敗；萬一失敗，是程式寫錯了，所以用 `expect`，訊息寫「為什麼我認為不會失敗」（第 4 集）。
- `people` 來自「使用者輸入」的文字，出錯是預料之內，所以 `match`：不是整數、是 0，各自印出提醒，程式繼續跑完。
- `split_bill` 回傳 `Result`，因為人數從外面來。第 1 集的 `panic!` 版本，只適合別處已經保證人數不是 0 的情況。

### 實務上的判斷

- 寫給別人呼叫的函式（程式庫）：盡量回傳 `Result`。替呼叫的人決定「整支程式結束」，對他來說太霸道。
- 程式自己的前提被破壞：用 `panic!`，訊息寫清楚。這是在說「這裡有 bug」，不是在處理錯誤。
- 範例、小工具、測試：`unwrap`、`expect` 沒問題。
- 一路往上傳：中間的函式用 `?`，到最外層的 `main` 才決定怎麼辦：印訊息，或讓 `main` 回傳 `Result`（第 11 集）。

不確定時，先回傳 `Result`：呼叫的人想讓它 panic，自己加一個 `expect` 就行；函式如果一開始就 panic，呼叫的人就沒有機會處理了。

### 跟 Go 比一比

Go 的標準寫法，每一步都要自己檢查：

```go
func orderTotal(qtyText, priceText string) (int, error) {
	qty, err := strconv.Atoi(qtyText)
	if err != nil {
		return 0, err
	}
	price, err := strconv.Atoi(priceText)
	if err != nil {
		return 0, err
	}
	return qty * price, nil
}
```

Rust 版就是第 5 集的 `order_total`：兩個 `?`，加最後的 `Ok(qty * price)`。把兩邊的概念對起來：

| Go | Rust |
| --- | --- |
| `(值, error)`，失敗時值是零值 | `Result<T, E>`，不是 `Ok(值)` 就是 `Err(原因)` |
| `if err != nil { return err }` | `?` |
| 忘了檢查 `err`，編譯照過 | 不處理 `Result` 會被警告（第 2 集）；要取值就一定得面對 `Err` |
| `errors.New("...")` | `Err(String::from("..."))`，或自己的錯誤 enum（第 7 集） |
| `fmt.Errorf("...: %w", err)` | `impl From<別人的錯誤>` 加 `?`（第 9 集） |
| `errors.Is`、`errors.As` | 對錯誤 enum 用 `match`（第 7 集）、`downcast_ref`（第 10 集） |
| `error` 介面，什麼都收 | 具體的錯誤型別，或 `Box<dyn Error>`（第 10 集） |
| `log.Fatal(err)` | `main` 回傳 `Err`（第 11 集） |
| `panic` / `recover` | `panic!`，一般程式不接 panic |

最大的差別不是語法，是把關的位置：Go 靠程式設計師記得檢查，漏了也不會被擋；Rust 把「可能失敗」寫進型別，不處理就拿不到值，編譯器替你把關。

## 重點整理

- 預料之內的失敗（使用者輸入、查不到、外部資料）回傳 `Result` 或 `Option`；程式寫錯、前提被破壞才 `panic!`。
- 程式庫盡量回傳 `Result`，不替呼叫的人決定整支程式結束；範例、原型、測試可以 `unwrap` / `expect`。
- 有把握不會失敗的值用 `expect`，訊息寫「為什麼我認為不會失敗」。
- 中間的函式用 `?` 往上傳，到最外層的 `main` 才決定怎麼處理。
- 跟 Go 比：`if err != nil { return err }` 對應 `?`；Go 的 `error` 介面對應 `Box<dyn Error>` 或自己的錯誤 enum；Go 靠自覺檢查，Rust 由型別強制處理。
