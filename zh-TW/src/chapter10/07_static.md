# `'static`

## 本集目標

知道 `'static` 是「整個程式執行期間都有效」的生命週期，字串字面值就是 `&'static str`；會用 `-> &'static str` 回傳字面值，也知道不能拿 `'static` 蓋掉懸置參照的錯誤。

## 正文

### 字面值的完整型別

第 4 章第 9 集說過，字串字面值的型別是 `&str`，錯誤訊息裡會看到 `&'static str`，當時說先不管。現在可以讀懂了：`'static` 是特別的生命週期名字，意思是從程式開始到結束都有效。字面值的文字寫在程式本身裡面，程式在跑，它就在，所以字面值的完整型別就是 `&'static str`。

第 4 章第 6 集那個 `help` 也要重新看。想寫一個回傳字面值的函式，會碰到 E0106：

```rust,compile_fail=E0106
fn welcome() -> &str {
    "歡迎光臨"
}

fn main() {
    println!("{}", welcome());
}
```

編譯錯誤：

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:17
  |
1 | fn welcome() -> &str {
  |                 ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
1 | fn welcome() -> &'static str {
  |                  +++++++
help: instead, you are more likely to want to return an owned value
  |
1 - fn welcome() -> &str {
1 + fn welcome() -> String {
  |
```

沒有參照參數，規則 2 沒有輸入的生命週期可以用，所以是 E0106。這次 `help` 的第一個建議 `&'static str` 是對的：這個參照指向字面值，確實整個程式都有效。第二個建議改回傳 `String` 也行，但為了一句固定的文字，不必多配置一份 heap。

### 回傳字面值：`-> &'static str`

第 6 章把 enum 轉成文字，都是在 `match` 分支裡直接印，或在 `main` 裡寫 `let text = match ...`，因為當時還不能寫「回傳字面值的函式」。現在可以包成函式了：

```rust
enum Status {
    Pending,
    Paid,
    Shipped,
}

fn status_text(status: &Status) -> &'static str {
    match status {
        Status::Pending => "待付款",
        Status::Paid => "已付款",
        Status::Shipped => "已出貨",
    }
}

fn main() {
    for status in [Status::Pending, Status::Paid, Status::Shipped] {
        println!("{}", status_text(&status));
    }
    let text = {
        let status = Status::Paid;
        status_text(&status)
    };
    println!("{text}");
}
```

執行結果：

```text
待付款
已付款
已出貨
已付款
```

`status_text` 有一個參照參數，寫成 `-> &str` 也能編譯，省略規則會讓結果借自 `status`。可是這樣一來，`main` 最後那段內層區塊就會變成 E0597：編譯器以為 `text` 借自 `status`，`status` 在 `}` 就 drop 了。標成 `'static`，等於告訴編譯器「結果跟參數無關，永遠有效」，`text` 才能活到外面。

### struct 欄位放字面值

欄位只會放字面值的話，標 `'static` 就好，不用 `<'a>`：

```rust
struct Drink {
    name: &'static str,
    price: u32,
}

fn main() {
    let tea = Drink {
        name: "紅茶",
        price: 30,
    };
    println!("{} {} 元", tea.name, tea.price);
}
```

執行結果：

```text
紅茶 30 元
```

`'static` 是固定的名字，不是需要宣告的參數，所以 `Drink` 沒有 `<'a>`，也就不用跟著傳播。第 2 章第 1 集的 `const` 與 `static` 裡，參照預設就是 `'static`，寫成 `const GREETING: &str = "…";` 就好，多寫 `'static` 反而會被 clippy 提醒（`redundant_static_lifetimes`）。

### `'static` 不是萬靈丹

第 4 章第 6 集的 `dangle`，這次照 `help` 加上 `'static`：

```rust,compile_fail=E0515
fn dangle() -> &'static String {
    let s = String::from("哈囉");
    &s
}

fn main() {
    let r = dangle();
    println!("{r}");
}
```

編譯錯誤：

```text
error[E0515]: cannot return reference to local variable `s`
 --> src/main.rs:3:5
  |
3 |     &s
  |     ^^ returns a reference to data owned by the current function
```

結果跟第 2 集的 `make<'a>` 一樣是 E0515。`'static` 是一個承諾：「這個參照整個程式都有效」。`s` 是區域變數，`}` 就 drop 了，承諾是假的，編譯器照樣抓出來。只有真的活整個程式的資料（字面值、`static`），才配得上 `'static`；編譯器叫你加 `'static` 時，先想想資料到底活多久。第 14 章第 3 集的 `thread::spawn` 還會再遇到它：它要求閉包是 `'static`。

## 重點整理

- `'static` 是整個程式執行期間都有效的生命週期；字串字面值的完整型別是 `&'static str`。
- 沒有參照參數、回傳字面值的函式寫成 `-> &'static str`；回傳 `-> &str` 會是 E0106。
- 函式有參照參數時，`-> &'static str` 讓結果不受參數影響，可以活得比參數久。
- struct 欄位只放字面值，可以寫 `&'static str`，不必有 `<'a>`；`const`、`static` 裡的參照預設就是 `'static`，不用寫。
- `'static` 是承諾，不是修正：對區域變數的參照標 `'static`，仍然是 E0515。
