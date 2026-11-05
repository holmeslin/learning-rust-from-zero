# match guard

## 本集目標

在 `match` 分支的模式後面加上 `if 條件`，讓同一種模式依條件分成不同的分支，並知道編譯器不把 guard 算進窮盡檢查。

## 正文

### 模式之外再加條件

前面的 `Percent(p)`，如果有人傳進 `Percent(120)`，`100 - p` 會是負數，`u32` 減過頭就是第 1 章第 16 集的溢位 panic。想在 `p` 大於等於 100 時另外處理，可以在模式後面加 `if 條件`，這叫 **match guard**：

```rust
enum Discount {
    Regular,
    Percent(u32),
    Threshold { min: u32, off: u32 },
}

fn final_price(price: u32, discount: Discount) -> u32 {
    match discount {
        Discount::Regular => price,
        Discount::Percent(p) if p >= 100 => 0,
        Discount::Percent(p) => price * (100 - p) / 100,
        Discount::Threshold { min, off } if price >= min => price - off,
        Discount::Threshold { .. } => price,
    }
}

fn main() {
    println!("{}", final_price(1000, Discount::Percent(20)));
    println!("{}", final_price(1000, Discount::Percent(120)));
    let t = Discount::Threshold { min: 800, off: 150 };
    println!("{}", final_price(500, t));
    println!("{}", final_price(1000, Discount::Regular));
}
```

執行結果：

```text
800
0
500
1000
```

- `Discount::Percent(p) if p >= 100 => 0`：模式符合，**而且**條件為 `true`，這個分支才會執行。條件裡可以用模式綁定的變數（這裡的 `p`）。
- 模式符合、條件卻是 `false`，就當作這個分支沒比中，繼續往下一個分支比。`Percent(20)` 就是這樣掉到下一個 `Percent(p)`。
- 所以有 guard 的分支要排前面，同樣模式、沒有 guard 的分支排後面；反過來寫，guard 永遠輪不到。
- `Discount::Threshold { .. }` 的 `..` 表示「欄位全都不要」，第 12 集會細講。這裡是讓 `price` 不到門檻時，也有分支接住 `Threshold`。

### guard 不算窮盡

編譯器不會去推理 guard 的條件，所以有 guard 的分支，一律不算進窮盡檢查。下面兩個條件加起來明明涵蓋所有整數，編譯器還是不接受：

```rust,compile_fail=E0004
fn main() {
    let n: i32 = 5;
    match n {
        x if x > 0 => println!("正數 {x}"),
        x if x <= 0 => println!("零或負數 {x}"),
    }
}
```

編譯錯誤：

```text
error[E0004]: non-exhaustive patterns: `i32::MIN..=i32::MAX` not covered
 --> src/main.rs:3:11
  |
3 |     match n {
  |           ^ pattern `i32::MIN..=i32::MAX` not covered
  |
  = note: the matched value is of type `i32`
  = note: match arms with guards don't count towards exhaustivity
```

訊息裡有一行直接說明：`match arms with guards don't count towards exhaustivity`。最後一個分支改成沒有 guard 的（例如 `x => ...` 或 `_ => ...`），就能編譯成功。所以 guard 是用來「把已經窮盡的 `match` 分得更細」，不是用來取代窮盡檢查的工具。

## 重點整理

- match guard：`模式 if 條件 => 運算式`。模式符合而且條件為 `true`，分支才執行；條件可以用模式綁定的變數。
- 條件為 `false` 時，繼續往下一個分支比，所以有 guard 的分支要排在同樣模式、沒有 guard 的分支之前。
- 有 guard 的分支不算進窮盡檢查（E0004）。剩下的情況，一定要有沒 guard 的分支來接住。
- `Discount::Threshold { .. }` 的 `..` 表示忽略所有欄位，第 12 集再細講。
