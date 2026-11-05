# 標註語法 `'a`

## 本集目標

會讀、會寫生命週期標註 `<'a>` 與 `&'a str`，並知道它描述的是參照之間的關係，不會讓任何值活得更久。

## 正文

### 長相

`'a` 叫**生命週期參數**（lifetime parameter）：一個單引號加上小寫名稱，慣例從 `'a` 開始，接著 `'b`、`'c`。它不是值，也不是型別，只是一段範圍的名字。

寫它的地方有兩個：

- 宣告：寫在函式名稱後面的 `<>` 裡，位置跟第 8 章第 8 集泛型函式的 `<T>` 一樣。
- 使用：寫在參照的 `&` 後面、型別前面，例如 `&'a str`、`&'a mut String`。

用它修好上一集的 `name_or`：

```rust
fn name_or<'a>(name: &'a str, fallback: &'a str) -> &'a str {
    if name.is_empty() { fallback } else { name }
}

fn main() {
    let name = String::from("小美");
    let fallback = String::from("貴賓");
    println!("{}", name_or(&name, &fallback));
    println!("{}", name_or("", &fallback));
}
```

執行結果：

```text
小美
貴賓
```

### 怎麼讀這行簽名

```text
fn name_or<'a>(name: &'a str, fallback: &'a str) -> &'a str
```

- `<'a>`：宣告一段範圍，名字叫 `'a`。
- `name: &'a str`、`fallback: &'a str`：兩個參照，都至少在 `'a` 內有效。
- `-> &'a str`：回傳的參照，同樣只保證在 `'a` 內有效。

**同一個名字，就是同一段範圍。** 簽名等於對編譯器說：不管回傳的是 `name` 還是 `fallback`，呼叫的人都只能在 `'a` 之內使用結果。這樣編譯器就不必猜了，檢查的時候，兩個輸入和回傳值都按這段範圍算。

`'a` 實際上有多長，不是你決定的，由呼叫端的程式決定；兩個輸入活得不一樣久時怎麼算，下一集再看。

這裡兩個參照參數都要標，因為兩個都可能被回傳。第 4、5 章的函式沒標，是省略規則替我們補上了，第 4 集會講。

### 標註不會讓值活得更久

初學時容易有個誤會：編譯器說「缺少生命週期」，那把 `'a` 加上去，值就會活得夠久。試試看，回到第 4 章第 6 集的 `dangle`，這次加上 `'a`：

```rust,compile_fail=E0515
fn make<'a>() -> &'a String {
    let s = String::from("哈囉");
    &s
}

fn main() {
    let r = make();
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

第 4 章第 6 集的 `fn dangle() -> &String` 是 E0106，因為簽名沒說借自誰。現在我們說了，錯誤變成 E0515：`returns a reference to data owned by the current function`，意思是回傳的參照，指向函式自己擁有的資料。

`s` 是函式裡的區域變數，`}` 一到就被 drop，標註改變不了這件事。標註是一份承諾，編譯器負責檢查函式本體有沒有做到：

- 沒寫標註：E0106，「你沒說清楚」。
- 寫了標註、但本體做不到：E0515 這類錯誤，「你說的跟你做的對不上」。

## 重點整理

- 生命週期參數是單引號加小寫名稱，慣例用 `'a`、`'b`；宣告在函式名稱後面的 `<>`，使用寫成 `&'a str`、`&'a mut String`。
- 同一個名字代表同一段範圍：`name_or<'a>` 的兩個輸入與回傳值都在 `'a` 內有效。
- 回傳值可能來自哪幾個參數，那幾個參數就要標上同一個名字。
- 標註只是描述參照之間的關係，不會讓任何值活得更久。
- 簽名沒說是 E0106；說了、函式本體卻違反（例如回傳區域變數的參照）是 E0515。
