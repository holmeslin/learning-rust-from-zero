# `assert_eq!` 與 `#[should_panic]`

## 本集目標

用 `assert_eq!` 讓失敗訊息顯示兩邊的值，用 `#[should_panic]` 測試「應該 panic」的情況。

## 正文

### `assert_eq!`：失敗時看得到兩邊的值

`assert!(a == b)` 失敗時只說條件不成立。`assert_eq!(a, b)` 檢查兩個值相等，失敗時把兩邊都印出來，後面還可以加一段自己的說明（格式跟 `println!` 一樣）。故意把預期值寫錯：

`src/lib.rs`：

```rust,test,noplayground,exit=101
pub fn pay(total: u32) -> u32 {
    if total >= 100 { total - 10 } else { total }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discount_over_100() {
        assert_eq!(pay(120), 120, "滿 100 折 10，結果是 {}", pay(120));
    }
}
```

`cargo test` 的輸出（省略編譯進度，某一次）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-32c5571f53fea115)

running 1 test
test tests::discount_over_100 ... FAILED

failures:

---- tests::discount_over_100 stdout ----

thread 'tests::discount_over_100' (158855061) panicked at src/lib.rs:11:9:
assertion `left == right` failed: 滿 100 折 10，結果是 110
  left: 110
 right: 120
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::discount_over_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

`left` 是第一個引數的值，`right` 是第二個，冒號後面是自己加的說明。一看就知道 `pay(120)` 是 110，預期寫成了 120。

預期值改成 110 就會通過。另外還有 `assert_ne!(a, b)`：兩個值**不相等**才通過。一個測試裡可以放好幾個 `assert`，但第一個失敗就會 panic，後面的不會執行。

### 比較的型別要能 `==` 也能印出來

`assert_eq!` 用 `==` 比較，失敗時用 `{:?}` 印出兩邊，所以型別要實作 `PartialEq` 與 `Debug`，也就是第 8 章第 10 集的 `derive`。只有 `Debug` 的 struct 會是 E0369：

```rust,compile_fail=E0369
#[derive(Debug)]
struct Cup {
    ml: u32,
}

fn main() {
    let cup = Cup { ml: 500 };
    assert_eq!(cup, Cup { ml: 500 });
    println!("{}", cup.ml);
}
```

編譯錯誤：

```text
error[E0369]: binary operation `==` cannot be applied to type `Cup`
 --> src/main.rs:8:5
  |
8 |     assert_eq!(cup, Cup { ml: 500 });
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     Cup
  |     Cup
  |
note: an implementation of `PartialEq` might be missing for `Cup`
 --> src/main.rs:2:1
  |
2 | struct Cup {
  | ^^^^^^^^^^ must implement `PartialEq`
```

訊息說 `Cup` 不能用 `==` 比較，`note` 提示可能少了 `PartialEq`；後面還有一則 `help`，直接建議 `#[derive(PartialEq)]`。如果只有 `PartialEq` 卻沒有 `Debug`，就是 E0277，一樣用 `derive` 解決。

### `#[should_panic]`：應該 panic 才算通過

有些函式在輸入不合理時**本來就該 panic**，這也要測。在 `#[test]` 底下再標 `#[should_panic]`，這個測試 panic 才算通過，沒 panic 反而失敗：

`src/lib.rs`：

```rust,test,noplayground
pub fn split_bill(total: u32, people: u32) -> u32 {
    if people == 0 {
        panic!("人數不能是 0");
    }
    total / people
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "人數")]
    fn zero_people() {
        split_bill(100, 0);
    }
}
```

`cargo test` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-32c5571f53fea115)

running 1 test
test tests::zero_people - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests teashop

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

結果那一行多了 `- should panic`。`expected = "人數"` 的意思是：panic 的訊息要**含有**這段文字才算通過。只寫 `#[should_panic]` 的話，任何 panic 都通過，哪怕是別的 bug 造成的，所以建議都寫 `expected`。如果函式其實沒 panic，測試會失敗，訊息是 `note: test did not panic as expected`，後面接著測試的位置。

用 `Result` 回傳錯誤、不 panic 的函式（第 9 章），就用 `assert_eq!` 比較它回傳的 `Err(...)`；`#[should_panic]` 留給本來就用 `panic!` 或 `expect` 當作失敗方式的函式。

## 重點整理

- `assert_eq!(a, b)` 檢查相等，失敗時印出 `left` 與 `right`；後面可以加 `println!` 格式的說明。
- 兩邊的型別要有 `PartialEq`（比較）與 `Debug`（印出來），用 `derive` 補上；只有 `Debug` 會是 E0369。
- `assert_ne!(a, b)` 檢查不相等；一個測試裡第一個失敗的 `assert` 就 panic，後面不會執行。
- `#[should_panic]` 讓「panic 才通過」；`expected = "文字"` 要求 panic 訊息含有那段文字，建議都寫。
- 回傳 `Result` 的函式用 `assert_eq!` 比較 `Err`，不用 `#[should_panic]`。
