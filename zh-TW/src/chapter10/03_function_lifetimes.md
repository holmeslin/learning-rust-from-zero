# 函式的生命週期

## 本集目標

看懂呼叫端怎麼套用 `'a`：結果的有效範圍，是幾個輸入重疊的那一段；並會用不同的名字，讓回傳值只綁定真正會回傳的參數。

## 正文

### 同一個 `'a`：取重疊的那一段

上一集的 `name_or`，兩個輸入和回傳值共用 `'a`。如果呼叫端的兩個輸入活得不一樣久，會怎麼樣？

```rust,compile_fail=E0597
fn name_or<'a>(name: &'a str, fallback: &'a str) -> &'a str {
    if name.is_empty() { fallback } else { name }
}

fn main() {
    let name = String::from("小美");
    let result;
    {
        let fallback = String::from("貴賓");
        result = name_or(&name, &fallback);
    }
    println!("{result}");
}
```

編譯錯誤：

```text
error[E0597]: `fallback` does not live long enough
  --> src/main.rs:10:33
   |
 9 |         let fallback = String::from("貴賓");
   |             -------- binding `fallback` declared here
10 |         result = name_or(&name, &fallback);
   |                                 ^^^^^^^^^ borrowed value does not live long enough
11 |     }
   |     - `fallback` dropped here while still borrowed
12 |     println!("{result}");
   |                ------ borrow later used here
```

`name` 活到 `main` 結束，`fallback` 只活到內層的 `}`。簽名把兩個輸入和結果綁在同一個 `'a`，`'a` 就只能取兩段範圍重疊的部分，也就是活得比較短的那個：

```text
name       [==============================]
fallback            [=========]
'a                  [=========]               兩段範圍重疊的部分
result                  [==============]      用到 println!，超出 'a
```

`result` 在內層 `}` 之後還要用，超出了 `'a`，所以編譯器說 `fallback` 活得不夠久（`does not live long enough`）。

這段程式實際跑起來，`name` 不是空的，回傳的就是 `name`，照理沒有問題。可是編譯器不看值，只看簽名；簽名說「結果可能是任何一個輸入」，它就照最壞的情況算。**共用同一個 `'a`，結果的壽命就是輸入裡最短的那個。**

### 只綁定真正會回傳的那個

如果函式只會回傳其中一個參數，就只替那一個標 `'a`，其他參照參數不用。例如從 `小美,台北` 這種文字裡，取逗號前面那一段：

```rust
fn first_part<'a>(line: &'a str, sep: &str) -> &'a str {
    match line.find(sep) {
        Some(i) => &line[..i],
        None => line,
    }
}

fn main() {
    let line = String::from("小美,台北");
    let part;
    {
        let sep = String::from(",");
        part = first_part(&line, &sep);
    }
    println!("{part}");
}
```

執行結果：

```text
小美
```

回傳值只可能借自 `line`，所以只有 `line` 與回傳值標 `'a`。`sep` 沒有名字，它有自己不具名的生命週期（省略規則補上的，第 4 集再講），跟結果無關。因此 `sep` 在內層 `}` 就結束了，`part` 照樣能在外面用。

你也可以替 `sep` 取名字，寫成 `<'a, 'b>` 和 `sep: &'b str`，意思完全一樣：不同的名字，代表兩段彼此無關的範圍。只用到一次的名字沒有必要取，clippy 也會叫你省掉，所以直接寫 `sep: &str`。

### 簽名說的，函式本體要做到

簽名說回傳值只借自 `line`，可是函式本體卻回傳了 `sep` 呢？

```rust,compile_fail=E0621
fn first_part<'a>(line: &'a str, sep: &str) -> &'a str {
    if line.is_empty() { sep } else { line }
}

fn main() {
    println!("{}", first_part("小美,台北", ","));
}
```

編譯錯誤：

```text
error[E0621]: explicit lifetime required in the type of `sep`
 --> src/main.rs:2:26
  |
2 |     if line.is_empty() { sep } else { line }
  |                          ^^^ lifetime `'a` required
  |
help: add explicit lifetime `'a` to the type of `sep`
  |
1 | fn first_part<'a>(line: &'a str, sep: &'a str) -> &'a str {
  |                                        ++
```

編譯器發現：回傳值必須在 `'a` 內有效，但 `sep` 沒有標 `'a`，沒人保證它夠久，所以說 `` explicit lifetime required in the type of `sep` ``，並建議把 `sep` 也標成 `&'a str`。照它改，就變回這一集第一個例子的樣子，兩個輸入都綁進 `'a`。

遇到這種錯誤，先問自己：回傳值到底會不會是這個參數？會，就把它綁進 `'a`；不會，就改函式本體。

## 重點整理

- 同一個 `'a` 標在好幾個參數與回傳值上，結果的有效範圍，就是這些輸入重疊的那一段，也就是活得最短的那個；在那之後還使用結果，是 E0597。
- 編譯器不看執行時的值，只看簽名：簽名說可能是哪一個，就照每一個都有可能來算。
- 只有可能被回傳的參數，才需要標進回傳值的生命週期；其他參照參數不必取名，跟結果無關，活多久都可以。
- 不同的名字代表彼此無關的範圍：`<'a, 'b>`。
- 函式本體回傳了簽名沒有綁定的參數，是 E0621，`help` 會建議替它標上 `'a`。
