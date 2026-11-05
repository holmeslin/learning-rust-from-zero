# 省略規則

## 本集目標

知道編譯器在函式簽名裡自動補上生命週期的三條省略規則，所以大多數函式不必寫 `'a`；也知道規則不夠用的時候，編譯器會用 E0106 要你自己補。

## 正文

### 為什麼之前都不用寫

第 4 章第 8 集的 `prefix(s: &String, n: usize) -> &str`，回傳的 `&str` 沒有任何標註，卻能編譯。因為有**省略規則**（lifetime elision）：編譯器看簽名，照三條固定的規則把生命週期補上去。它只看簽名，不看函式本體。

三條規則，依序套用：

1. 每個參照參數，各自得到一個生命週期。
2. 如果輸入的生命週期只有一個，回傳值裡所有的參照，都用這一個。
3. 如果參數裡有 `&self` 或 `&mut self`，回傳值裡所有的參照，都用 `self` 的生命週期（第 6 集）。

三條套用完，回傳的參照還是沒有生命週期，就是 E0106。

以 `tail` 為例，它取一段文字從第幾個位元組開始的後半：

```text
fn tail(line: &str, from: usize) -> &str
規則 1：fn tail<'a>(line: &'a str, from: usize) -> &str
規則 2：fn tail<'a>(line: &'a str, from: usize) -> &'a str
```

`from` 不是參照，沒有生命週期，所以輸入的生命週期只有 `line` 那一個，規則 2 成立：

```rust
fn tail(line: &str, from: usize) -> &str {
    &line[from..]
}

fn main() {
    let line = String::from("小美,咖啡");
    println!("{}", tail(&line, 7));
}
```

執行結果：

```text
咖啡
```

我們一個標註也沒寫，編譯器仍然知道回傳值借自 `line`。把 `line` 放進內層區塊就看得出來：

```rust,compile_fail=E0597
fn tail(line: &str, from: usize) -> &str {
    &line[from..]
}

fn main() {
    let rest = {
        let line = String::from("小美,咖啡");
        tail(&line, 7)
    };
    println!("{rest}");
}
```

編譯錯誤：

```text
error[E0597]: `line` does not live long enough
 --> src/main.rs:8:14
  |
6 |     let rest = {
  |         ---- borrow later stored here
7 |         let line = String::from("小美,咖啡");
  |             ---- binding `line` declared here
8 |         tail(&line, 7)
  |              ^^^^^ borrowed value does not live long enough
9 |     };
  |     - `line` dropped here while still borrowed
```

`rest` 借自 `line`，`line` 在內層 `}` 就 drop，`rest` 卻還要用。這跟上一集手寫標註的結果一樣，只是這次是規則 2 替我們寫的。

### 規則不夠用的時候

兩個參照參數的 `name_or`，三條規則會卡在哪：

```text
fn name_or(name: &str, fallback: &str) -> &str
規則 1：fn name_or<'a, 'b>(name: &'a str, fallback: &'b str) -> &str
規則 2：輸入有兩個生命週期，不適用
規則 3：沒有 self，不適用
```

回傳的參照沒有生命週期，所以是第 1 集的 E0106。沒有任何參照參數的函式也一樣：沒有輸入的生命週期可以用，規則 2 不成立（第 7 集會看到）。

第 4、5 章規定「回傳參照的函式只放一個參照參數」，就是為了讓規則 2 永遠成立，這樣我們才能先不談生命週期。從這一章起沒有這個限制了，規則不夠用的地方，自己標就行。

### 能省略的地方就省略

標註跟省略的意思完全一樣的時候，慣例就是省略，不寫 `<'a>`。clippy 有一條 `needless_lifetimes` 會提醒你：把 `tail` 寫成 `fn tail<'a>(line: &'a str, from: usize) -> &'a str`，clippy 會建議拿掉。所以後面幾集的函式，規則補得出來的都不寫。

省略規則只管函式與方法的簽名。struct 的欄位沒有這個待遇，欄位裡的參照一定要自己寫生命週期，下一集就會用到。

## 重點整理

- 省略規則有三條：每個參照參數各有一個生命週期；輸入的生命週期只有一個，回傳值就用它；有 `&self` 或 `&mut self`，回傳值就用 `self` 的（第 6 集）。
- 規則只看簽名；套用完回傳的參照還是沒有生命週期，就是 E0106。
- 只有一個參照參數（其他參數不是參照）時，回傳的參照借自它，不必標註。
- 兩個以上的參照參數、或沒有參照參數時，回傳參照要自己標，或改成回傳擁有的值。
- 能省略就省略（clippy `needless_lifetimes`）；省略只用在函式與方法簽名，struct 欄位一定要寫。
