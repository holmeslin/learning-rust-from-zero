# 用 `match` 處理 `Result`

## 本集目標

用 `match` 對 `Ok` 和 `Err` 各寫一個分支，取出成功的值、讀出失敗的原因；也能用 `if let` 和 `let else` 處理。

## 正文

### `Ok` 一個分支、`Err` 一個分支

`Result` 也只是 enum，處理的方法跟第 6 章的 `Option` 一樣。`parse` 就是回傳 `Result` 的函式（第 8 章第 14 集），拿它來試：

```rust
fn main() {
    for text in ["3", "0", "abc", "300"] {
        match text.parse::<u8>() {
            Ok(0) => println!("{text:?} → 至少要點一杯"),
            Ok(n) => println!("{text:?} → {n} 杯"),
            Err(e) => println!("{text:?} → 失敗：{e}"),
        }
    }
}
```

執行結果：

```text
"3" → 3 杯
"0" → 至少要點一杯
"abc" → 失敗：invalid digit found in string
"300" → 失敗：number too large to fit in target type
```

- `Ok(n)` 把成功的值放進 `n`；`Ok(0)` 是只符合成功值剛好是 0 的分支，要排在 `Ok(n)` 前面。
- `Err(e)` 把失敗的原因放進 `e`。這裡的 `e` 是 `ParseIntError`，用 `{e}` 印出它的說明文字；標準函式庫的訊息是英文。
- `{e:?}` 印的是 `Debug` 格式，也就是第 8 章第 14 集看過的 `ParseIntError { kind: InvalidDigit }`。給使用者看用 `{e}`，除錯用 `{e:?}`。
- 不需要原因的時候，寫成 `Err(_)`。

### 一定要處理 `Err`

`match` 要窮盡（第 6 章第 4 集），只處理成功的情況不行：

```rust,compile_fail=E0004
fn main() {
    match "abc".parse::<u32>() {
        Ok(n) => println!("{n}"),
    }
}
```

編譯錯誤：

```text
error[E0004]: non-exhaustive patterns: `Err(_)` not covered
 --> src/main.rs:2:11
  |
2 |     match "abc".parse::<u32>() {
  |           ^^^^^^^^^^^^^^^^^^^^ pattern `Err(_)` not covered
  |
```

`` non-exhaustive patterns: `Err(_)` not covered ``：沒寫 `Err` 就編譯失敗，所以失敗的情況不可能被漏掉。訊息後面的 `note` 帶有標準函式庫的路徑，`help` 建議補上 `Err(_) => todo!()`，記得換成真正的處理。

### `match` 的值

`match` 是運算式（第 6 章第 3 集），可以直接把結果放進變數。把第 1 章讀整數的固定句型改成這樣，輸入不是整數也不會 panic：

```rust,stdin=abc,noplayground
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    let line = line.trim();
    let qty = match line.parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("請輸入整數，這次當作 1 杯");
            1
        }
    };
    println!("{qty} 杯，共 {} 元", qty * 30);
}
```

輸入 `abc` 的執行結果：

```text
請輸入整數，這次當作 1 杯
1 杯，共 30 元
```

兩個分支都要給 `u32`：`Ok` 直接給 `n`，`Err` 先印提醒，再以 `1` 結束。輸入 `3` 的話走 `Ok` 分支，印出 `3 杯，共 90 元`。

### `if let` 與 `let else`

只在意其中一種結果的時候，第 6 章的 `if let` 和 `let else` 也能用在 `Result`：

```rust
fn main() {
    if let Err(e) = "12x".parse::<u32>() {
        println!("失敗：{e}");
    }
    let Ok(n) = "abc".parse::<u32>() else {
        println!("不是整數，結束");
        return;
    };
    println!("{n}");
}
```

執行結果：

```text
失敗：invalid digit found in string
不是整數，結束
```

- `if let Err(e) = ...`：只有失敗才進去，成功就跳過。
- `let Ok(n) = ... else { ... };`：成功的話 `n` 在後面可以用，失敗就走 `else`，而且 `else` 一定要離開（這裡是 `return`）。

## 重點整理

- `match` 處理 `Result`：`Ok(值) => ...`、`Err(原因) => ...`；`Ok(0)` 這種字面值模式要排在 `Ok(n)` 前面。
- `Err(e)` 的 `e` 是失敗的原因，`{e}` 印給使用者看的說明，`{e:?}` 是除錯用的格式；不需要就寫 `Err(_)`。
- 只寫 `Ok` 不寫 `Err` 是 E0004：失敗的情況一定要處理。
- `match` 是運算式，每個分支給出同一種型別；`Err` 分支可以先印訊息、再回傳預設值。
- `if let Err(e) = ...` 只處理失敗；`let Ok(n) = ... else { return; }` 只處理成功，失敗就離開。
