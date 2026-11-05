# 為什麼需要生命週期

## 本集目標

知道每個參照都有一段「能安全使用的範圍」，這段範圍叫生命週期；也知道函式有兩個以上的參照參數、又要回傳參照時，編譯器為什麼會報 E0106，要你把關係寫出來。

## 正文

### 參照有效的範圍

第 4 章第 6 集的規則是：**參照不能比它借的東西活得久**。第 4 章第 5 集又說，參照從借出開始、用到最後一次為止才有效。這一段範圍，就是這個參照的**生命週期**（lifetime）。

```rust
fn main() {
    let order = String::from("珍珠奶茶");
    let favorite = &order;
    println!("{favorite}");
}
```

執行結果：

```text
珍珠奶茶
```

把兩段範圍畫出來：

```text
order      [===========================]   建立到 main 結束
favorite            [=========]             借出到最後一次使用
```

`favorite` 的範圍完全落在 `order` 裡面，沒有問題。只要它伸出去、超過 `order`，就是第 4 章看過的 E0597。

到目前為止，你沒寫過任何生命週期，因為編譯器推斷得出來，就像推斷型別一樣。這一章要學的，是推斷不出來、需要你開口的時候。

### 編譯器只看簽名

呼叫函式的時候，編譯器不會鑽進函式本體，只看簽名：名稱、參數型別、回傳型別。如果參照參數只有一個，簽名就夠清楚了，例如第 4 章第 8 集的 `prefix(s: &String, n: usize) -> &str`，回傳的參照只可能借自 `s`。

有兩個參照參數，就不夠了。看這個函式：訂單上的名字如果是空的，就改用預設稱呼。

```rust,compile_fail=E0106
fn name_or(name: &str, fallback: &str) -> &str {
    if name.is_empty() { fallback } else { name }
}

fn main() {
    println!("{}", name_or("", "貴賓"));
}
```

編譯錯誤：

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:43
  |
1 | fn name_or(name: &str, fallback: &str) -> &str {
  |                  ----            ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `name` or `fallback`
help: consider introducing a named lifetime parameter
  |
1 | fn name_or<'a>(name: &'a str, fallback: &'a str) -> &'a str {
  |           ++++        ++                 ++          ++
```

`help` 的第一行把問題講得很清楚：回傳型別裡有借來的值，但簽名沒有說，它借自 `name` 還是 `fallback`。下面建議的 `<'a>` 就是下一集的內容，現在先看編譯器為什麼非問不可。

假設 `name` 活得久、`fallback` 在內層區塊就結束了：

```text
name       [===========================]
fallback            [=========]
結果借自 name：      能用到最後
結果借自 fallback：  只能用到 fallback 結束
```

結果能用多久，要看它借自哪一個；借自哪一個，又要看執行時 `name` 是不是空的。這種事編譯時期不知道，所以編譯器要你把關係寫在簽名裡，然後兩邊都檢查：呼叫的人要遵守，函式本體也不能違反。

### 標註只存在編譯時期

生命週期標註是寫給編譯器看的資訊，不會變成機器碼，執行時一點成本也沒有。它描述的是「這些參照之間的關係」，不是在管理記憶體。釋放記憶體的，還是擁有者離開作用域時的 drop。

### 跟 Go 比一比

Go 版的 `nameOr` 不會有這個問題：

```go
func nameOr(name, fallback string) string {
	if name == "" {
		return fallback
	}
	return name
}
```

Go 有垃圾回收，回傳的字串只要還有人在用，就不會被回收，所以沒有「借自誰」這個問題，代價是執行時要付出垃圾回收的成本。Rust 沒有垃圾回收，改成在編譯時期證明每個參照都有效，代價是有時候要自己在簽名裡寫出關係。

## 重點整理

- 參照的生命週期，是它從借出到最後一次使用的範圍；這段範圍不能超過它借的值。
- 大多數時候編譯器自己推斷，不用寫；推斷不出來才需要標註。
- 呼叫函式時，編譯器只看簽名。有兩個以上的參照參數、又回傳參照時，簽名沒說結果借自哪一個，就是 E0106。
- 生命週期標註只存在編譯時期，不影響執行。
- Go 靠垃圾回收讓參照一直有效；Rust 沒有，所以要在簽名裡寫出參照之間的關係。
