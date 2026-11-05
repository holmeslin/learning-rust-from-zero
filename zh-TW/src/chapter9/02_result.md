# `Result<T, E>`

## 本集目標

認得 `Result<T, E>` 的兩個 variant `Ok` 與 `Err`，會寫回傳 `Result` 的函式，並知道忽略 `Result` 會被編譯器警告。

## 正文

### 成功或失敗，寫進型別

上一集的 `split_bill` 碰到 0 人就 panic。但「人數填錯」是預料得到的事，應該讓呼叫的人決定怎麼辦。Rust 用標準函式庫的另一個 enum 表示「可能失敗」，叫 `Result<T, E>`：

```rust,ignore
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

跟第 6 章的 `Option` 同一個設計：`Ok` 帶著成功的值，`Err` 帶著失敗的原因。`<T, E>` 是兩個型別參數（第 8 章第 9 集），`Result<u32, String>` 讀成「成功的話是 `u32`，失敗的話是 `String`」。`Ok`、`Err` 跟 `Some`、`None` 一樣，直接寫就行。

```rust
fn split_bill(total: u32, people: u32) -> Result<u32, String> {
    if people == 0 {
        return Err(String::from("人數不能是 0"));
    }
    Ok(total / people)
}

fn main() {
    println!("{:?}", split_bill(120, 4));
    println!("{:?}", split_bill(120, 0));
}
```

執行結果：

```text
Ok(30)
Err("人數不能是 0")
```

- 失敗時 `return Err(...)`，沒有 `panic!`，程式照常往下走。成功時最後一個運算式是 `Ok(...)`。
- 失敗的原因是 `String`，用 `String::from` 建立；原因的型別可以換成別的，第 7 集會換成自己的 enum。
- `Result` 能用 `{:?}` 印，印出 `Ok(30)` 或 `Err("人數不能是 0")`。

### 不能直接當成功的值用

跟 `Option<u32>` 不是 `u32` 一樣，`Result<u32, String>` 也不是 `u32`：

```rust,compile_fail=E0369
fn split_bill(total: u32, people: u32) -> Result<u32, String> {
    if people == 0 {
        return Err(String::from("人數不能是 0"));
    }
    Ok(total / people)
}

fn main() {
    let total = split_bill(120, 4) + 10;
    println!("{total}");
}
```

編譯錯誤：

```text
error[E0369]: cannot add `{integer}` to `Result<u32, String>`
 --> src/main.rs:9:36
  |
9 |     let total = split_bill(120, 4) + 10;
  |                 ------------------ ^ -- {integer}
  |                 |
  |                 Result<u32, String>
```

`` cannot add `{integer}` to `Result<u32, String>` ``：想用裡面的 `u32`，就得先面對「其實失敗了」的情況，怎麼處理是下一集的事。訊息後面還有一則 `note`，講的是 `Result` 沒有實作 `Add`，這裡省略。

### 忘了處理會被警告

呼叫了回傳 `Result` 的函式，卻不看結果，編譯器會警告：

```rust,warn
fn split_bill(total: u32, people: u32) -> Result<u32, String> {
    if people == 0 {
        return Err(String::from("人數不能是 0"));
    }
    Ok(total / people)
}

fn main() {
    split_bill(120, 0);
    println!("結帳完成");
}
```

編譯警告：

```text
warning: unused `Result` that must be used
 --> src/main.rs:9:5
  |
9 |     split_bill(120, 0);
  |     ^^^^^^^^^^^^^^^^^^
  |
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
9 |     let _ = split_bill(120, 0);
  |     +++++++
```

`` unused `Result` that must be used ``：這個 `Result` 可能是 `Err`，應該處理。程式照樣能跑，但編譯器已經提醒你「失敗被默默吞掉了」。真的想丟掉，要像 `help` 建議的，明確寫 `let _ = ...`，讓讀程式的人知道這是故意的。

### 跟 Go 比一比

Go 用兩個回傳值表示：

```go
func splitBill(total, people int) (int, error) {
	if people == 0 {
		return 0, errors.New("人數不能是 0")
	}
	return total / people, nil
}

func main() {
	n, _ := splitBill(120, 0)
	fmt.Println(n)
}
```

- Go 的結果永遠是「值加 `error`」兩個東西，失敗時值是零值。`n, _ :=` 把 `error` 丟掉，編譯器不吭聲，`n` 就悄悄變成 0；連 `splitBill(120, 0)` 單獨一行也能編譯。
- Rust 的 `Result` 是二選一：不是 `Ok(值)` 就是 `Err(原因)`，沒有「失敗但還有個零值」。要拿到值，就得先面對 `Err`；不處理會有警告。

## 重點整理

- `Result<T, E>` 是 enum：`Ok(T)` 表示成功並帶值，`Err(E)` 表示失敗並帶原因；`Ok`、`Err` 直接寫，不用 `Result::`。
- 回傳 `Result<u32, String>` 的函式：失敗時 `return Err(...)`，成功時以 `Ok(...)` 結尾；`{:?}` 能印出 `Ok(30)`、`Err("...")`。
- `Result<u32, String>` 不是 `u32`，不能直接運算（E0369），要先處理 `Err` 的情況。
- 呼叫回傳 `Result` 的函式卻不處理，編譯器會警告 `unused Result that must be used`；故意丟掉要寫 `let _ = ...`。
- 跟 Go 比：Go 回傳「值加 error」兩個東西，error 可以悄悄丟掉；Rust 是二選一，不處理就拿不到值。
