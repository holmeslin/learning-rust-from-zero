# `Cow` 初探

## 本集目標

知道 `Cow<'_, str>`（clone-on-write，寫入時才複製）能讓函式「不用改就借用、需要改才配置新字串」，會用 `Cow::Borrowed`、`Cow::Owned`、`to_mut` 與 `into_owned`。

## 正文

### 有時要改，有時不用

訂單代碼要統一成小寫。大多數代碼本來就是小寫，直接借用原本的 `&str` 最省事；只有含大寫字母的，才需要建立新的 `String`。但函式的回傳型別只能寫一種：`&str` 裝不了新字串，`String` 又逼每次都配置。標準函式庫的 `Cow` 是 enum，兩種都裝得下：

```text
Cow<'a, str>
  Borrowed(&'a str)   借來的，沒有配置
  Owned(String)       自己擁有的新字串
```

```rust
use std::borrow::Cow;

fn normalize(code: &str) -> Cow<'_, str> {
    if code.chars().any(|c| c.is_uppercase()) {
        Cow::Owned(code.to_lowercase())
    } else {
        Cow::Borrowed(code)
    }
}

fn main() {
    for code in ["tea-01", "TEA-02"] {
        let fixed = normalize(code);
        match &fixed {
            Cow::Borrowed(_) => println!("{code}：沒改，直接借用"),
            Cow::Owned(_) => println!("{code}：改過，配置了新字串"),
        }
        println!("{fixed}（{} 位元組）", fixed.len());
    }
}
```

執行結果：

```text
tea-01：沒改，直接借用
tea-01（6 位元組）
TEA-02：改過，配置了新字串
tea-02（6 位元組）
```

- `"tea-01"` 沒有大寫，回傳 `Cow::Borrowed(code)`，沒有配置任何新記憶體；`"TEA-02"` 有大寫，回傳 `Cow::Owned(...)`，裝著新字串。
- 回傳型別寫 `Cow<'_, str>`：`Borrowed` 借的是參數，所以帶著生命週期（第 10 章第 5 集的 `'_`）。寫成 `Cow<str>` 會被 rustc 警告 `hiding a lifetime that's elided elsewhere is confusing`。
- 呼叫端不必管是哪一種：`Cow` 實作了 `Deref`（第 3 集），所以 `fixed.len()` 能用，也能直接用 `{fixed}` 印出來。想知道是哪一種，用 `match` 看 variant。

### 要改才複製：`to_mut`

名字 clone-on-write 的由來，是修改的那一刻：

```rust
use std::borrow::Cow;

fn main() {
    let mut code = Cow::Borrowed("tea-01");
    println!("{}", matches!(code, Cow::Borrowed(_)));
    println!("{code}");
    code.to_mut().push_str("-big");
    println!("{}", matches!(code, Cow::Owned(_)));
    println!("{code}");
    let owned: String = code.into_owned();
    println!("{}", owned.len());
}
```

執行結果：

```text
true
tea-01
true
tea-01-big
10
```

- `to_mut()` 回傳 `&mut String`。裡面如果還是 `Borrowed`，會先複製成 `Owned` 再給你；已經是 `Owned` 就直接給。所以沒有修改的時候，一次複製都不會發生。
- `into_owned()` 取出 `String`：`Borrowed` 就複製一份，`Owned` 直接交出來。

### 什麼時候用

- 函式「大多數時候不用改，偶爾才要改」，不想每次都配置新字串。
- 標準函式庫裡就有：`String::from_utf8_lossy(&bytes)` 回傳 `Cow<str>`，位元組合法就借用，遇到壞掉的才產生新字串，把壞掉的部分換成 `�`。
- 不只字串：`Cow<'_, [i32]>` 可以借用或擁有一段 slice。

`Cow` 不是預設選擇：簽名多了生命週期，呼叫端也多一層要想。先用 `&str` 或 `String`，確定有「常常不用複製」的熱點，再換成 `Cow`。

## 重點整理

- `Cow<'a, str>` 是 enum：`Borrowed(&'a str)` 借來的、`Owned(String)` 自己擁有的；回傳型別寫成 `Cow<'_, str>`。
- 不用改就回傳 `Cow::Borrowed(原本的參照)`，要改才建立新值，回傳 `Cow::Owned(...)`。
- `Cow` 實作 `Deref`，可以直接當 `&str` 用；`match` 可以看是哪個 variant。
- `to_mut()` 在需要修改時才把 `Borrowed` 複製成 `Owned`（clone-on-write）；`into_owned()` 取出 `String`。
- 常見來源是 `String::from_utf8_lossy`；先用 `&str` 或 `String`，有明確的需求再用 `Cow`。
