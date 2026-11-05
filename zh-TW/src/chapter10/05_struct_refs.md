# struct 持有參照

## 本集目標

會替 struct 加上 `<'a>`，讓欄位可以放參照；知道這種 struct 不能比它借的資料活得久，也認得函式簽名裡的 `Receipt<'_>`。

## 正文

### 欄位放參照，要標生命週期

第 5 章第 1 集，欄位寫 `name: &str` 得到 E0106，`help` 建議 `<'a>`，當時說先不用管。現在可以用了：

```rust
struct Receipt<'a> {
    customer: &'a str,
    drink: &'a str,
}

fn main() {
    let customer = String::from("小美");
    let drink = String::from("咖啡");
    let receipt = Receipt {
        customer: &customer,
        drink: &drink,
    };
    println!("{} 點了 {}", receipt.customer, receipt.drink);
}
```

執行結果：

```text
小美 點了 咖啡
```

寫法跟函式一樣：`struct Receipt<'a>` 在型別名稱後面宣告 `'a`，欄位裡的參照標上 `'a`。讀成：`Receipt` 裡的參照，都借自某段 `'a` 內有效的資料，這個 `Receipt` 不能活得比 `'a` 久。函式有省略規則可以偷懶，struct 欄位沒有，一定要寫。

`Receipt` 現在是「借東西」的 struct，借用規則（第 4 章）照樣管它：`receipt` 還要用的時候，不能改動 `customer`，也不能讓它被 drop。

### struct 不能比借來的資料活得久

規則還是那一條：參照不能比它借的東西活得久，只是現在參照藏在 struct 裡，struct 就繼承了這個限制：

```rust,compile_fail=E0597
struct Receipt<'a> {
    customer: &'a str,
    drink: &'a str,
}

fn main() {
    let receipt;
    {
        let customer = String::from("小美");
        receipt = Receipt {
            customer: &customer,
            drink: "咖啡",
        };
    }
    println!("{} 點了 {}", receipt.customer, receipt.drink);
}
```

編譯錯誤：

```text
error[E0597]: `customer` does not live long enough
  --> src/main.rs:11:23
   |
 9 |         let customer = String::from("小美");
   |             -------- binding `customer` declared here
10 |         receipt = Receipt {
11 |             customer: &customer,
   |                       ^^^^^^^^^ borrowed value does not live long enough
...
14 |     }
   |     - `customer` dropped here while still borrowed
15 |     println!("{} 點了 {}", receipt.customer, receipt.drink);
   |                            ---------------- borrow later used here
```

`customer` 在內層 `}` 就 drop，`receipt` 之後還要讀 `receipt.customer`，所以不行。被點名的只有 `customer`：`drink` 是字面值，整個程式執行期間都有效（第 7 集會講），不會有問題。

### 函式簽名裡的 `Receipt<'_>`

struct 可以直接指向別人給的文字，不用複製。下面的函式把 `小美,咖啡` 這種一行文字，拆成 `Receipt`：

```rust
struct Receipt<'a> {
    customer: &'a str,
    drink: &'a str,
}

fn parse_receipt(line: &str) -> Option<Receipt<'_>> {
    let comma = line.find(',')?;
    Some(Receipt {
        customer: &line[..comma],
        drink: &line[comma + 1..],
    })
}

fn main() {
    for text in ["小美,咖啡", "小明"] {
        match parse_receipt(text) {
            Some(r) => println!("{} 點了 {}", r.customer, r.drink),
            None => println!("格式錯誤：{text}"),
        }
    }
}
```

執行結果：

```text
小美 點了 咖啡
格式錯誤：小明
```

回傳型別裡的 `'_` 叫**匿名生命週期**：這裡有一個生命週期，但不取名字，交給省略規則決定。輸入只有 `line` 一個參照，規則 2 成立，所以回傳的 `Receipt` 借自 `line`，沒有複製任何文字。

回傳型別寫成 `Option<Receipt>`，不寫 `<'_>` 也能編譯，但 rustc 會警告 `hiding a lifetime that's elided elsewhere is confusing`（把省略掉的生命週期藏起來，容易看錯），建議補上 `'_`。有生命週期的型別出現在簽名裡，就寫出 `<'_>`，讀的人一眼就看得出這裡有借用。

### 跟 Go 比一比

Go 的 struct 欄位放 `string` 是家常便飯：

```go
type Receipt struct {
	Customer string
	Drink    string
}

func parseReceipt(line string) (Receipt, bool) {
	customer, drink, ok := strings.Cut(line, ",")
	return Receipt{Customer: customer, Drink: drink}, ok
}
```

`strings.Cut` 切出來的字串，跟 `line` 共用同一段記憶體，沒有複製；有垃圾回收，不管 `Receipt` 傳到哪裡，那段記憶體都不會被收走，所以什麼都不用標。

Rust 的 `Receipt<'_>` 做的事一樣，沒有複製，但它要靠生命週期證明資料夠久，結果就是這個 struct 不能活得比 `line` 久。所以 Go 的習慣搬到 Rust 要小心：struct 欄位預設放擁有的 `String`（第 5 章），只有在 struct 只是短暫借來看一眼資料的時候，才放參照。

## 重點整理

- struct 欄位放參照，要在型別名稱後面宣告生命週期，並標在欄位上：`struct Receipt<'a> { customer: &'a str }`；欄位沒有省略規則可用。
- 這種 struct 不能活得比它借的資料久，違反了是 E0597；借用規則也適用，struct 還要用的時候，不能改動或 drop 被借的資料。
- 函式簽名裡出現有生命週期的型別，寫成 `Receipt<'_>`：`'_` 是匿名生命週期，交給省略規則決定。不寫 `<'_>` 會有 `mismatched_lifetime_syntaxes` 警告。
- Go 的 struct 放 `string` 欄位不需要想這些；Rust 預設放擁有的 `String`，確定只是短暫借看資料時才放參照。
