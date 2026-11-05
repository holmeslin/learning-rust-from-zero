# `#[test]` 與 `cargo test`

## 本集目標

寫出第一個 `#[test]` 函式，用 `cargo test` 執行，並看懂測試通過與失敗時的輸出。

## 正文

### 測試就是會 panic 才算失敗的函式

到目前為止，我們都是靠 `println!` 印出結果、用眼睛檢查。測試讓這件事自動化：在函式上面標 `#[test]`，`cargo test` 就會執行它，**沒有 panic 就算通過，panic 了就算失敗**。

寫測試要先有可以測的東西，所以測試通常寫在 library 裡（第 2 集的 `lib.rs`）。檢查用 `assert!(條件)`：條件是 `false` 就 panic。

`src/lib.rs`：

```rust,test,noplayground
pub fn total_for(price: u32, cups: u32) -> u32 {
    price * cups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_cups() {
        assert!(total_for(30, 2) == 60);
    }

    #[test]
    fn zero_cups() {
        assert!(total_for(30, 0) == 0);
    }
}
```

`#[...]` 叫**屬性**（attribute），第 5 章的 `#[derive(Debug)]` 就是一種。`#[test]` 標記「這是測試」，測試函式不能有參數。

`#[cfg(test)]` 請先照抄：它的意思是「只有 `cargo test` 的時候才編譯這個模組」，平常 `cargo build` 不會把測試放進成品。`cfg` 第 16 章第 10 集才正式教。

`mod tests` 裡的 `use super::*;` 把上一層模組的所有東西引進來（第 5 集的 `*`），測試才叫得到 `total_for`。因為 `tests` 是子模組，第 4 集說過子模組看得到外面私有的東西，所以**測試連沒有 `pub` 的函式都測得到**。

### `cargo test`

`cargo test` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-32c5571f53fea115)

running 2 tests
test tests::two_cups ... ok
test tests::zero_cups ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests teashop

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- `Running unittests src/lib.rs (...)` 是在跑 `src/lib.rs` 裡的測試。括號裡 `teashop-` 後面那串雜湊，每台機器都不一樣。
- `test tests::two_cups ... ok`：一個測試一行，名字是模組路徑加函式名稱。測試是平行執行的，所以行的順序不一定。
- `test result: ok. 2 passed` 是總結。最後的 `finished in 0.00s` 是花了多久，每次略有不同。
- 後面的 `Doc-tests` 是另一種測試，第 13 集才會用到，現在 0 個。

測試也能寫在 `main.rs` 裡，只是別的 crate 和第 12 集的整合測試，沒辦法用到 `main.rs` 裡的函式，所以多半寫在 library。

### 失敗的測試

把預期值寫錯，看看失敗長什麼樣子：

`src/lib.rs`：

```rust,test,noplayground,exit=101
pub fn total_for(price: u32, cups: u32) -> u32 {
    price * cups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_cups() {
        assert!(total_for(30, 3) == 100);
    }
}
```

`cargo test` 的輸出（省略編譯進度，某一次）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-32c5571f53fea115)

running 1 test
test tests::three_cups ... FAILED

failures:

---- tests::three_cups stdout ----

thread 'tests::three_cups' (158852105) panicked at src/lib.rs:11:9:
assertion failed: total_for(30, 3) == 100
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::three_cups

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

失敗的測試會多一段 `failures:`：`---- tests::three_cups stdout ----` 底下是那個測試的 panic 訊息，第一行 `thread 'tests::three_cups' (數字) panicked at ...`，括號裡的數字每次不同，執行緒的名字就是測試的名字。`assertion failed: total_for(30, 3) == 100` 只告訴你哪個條件不成立，不知道 `total_for` 實際回傳多少，下一集的 `assert_eq!` 會補上這個資訊。

最後一行是 cargo 在說有測試失敗了（`--lib` 指的是 library 裡的測試）。只要有測試失敗，`cargo test` 的結束碼就不是 0，CI 才能靠它擋下壞掉的程式。

### 只跑一部分

測試多了，可以在 `cargo test` 後面加名字，只跑名字裡含有那段文字的測試：`cargo test two` 只跑 `tests::two_cups`，輸出會多一句 `1 filtered out`（有一個測試被略過）。

### 跟 Go 比一比

Go 的測試放在另一個 `_test.go` 檔，靠函式名稱 `TestXxx` 辨認，失敗是呼叫 `t.Errorf`，**失敗後函式會繼續往下跑**（要 `t.Fatalf` 才停）。Rust 的單元測試，通常跟被測的程式寫在同一個檔案，靠 `#[test]` 辨認，失敗就是 panic，這個測試馬上結束，其他測試照跑。

```go
func TestTotalFor(t *testing.T) {
	if got := TotalFor(30, 2); got != 60 {
		t.Errorf("TotalFor(30, 2) = %d, want 60", got)
	}
}
```

## 重點整理

- 在函式上標 `#[test]`，`cargo test` 就會執行；沒有 panic 算通過，panic 算失敗。
- `assert!(條件)` 在條件是 `false` 時 panic，訊息是 `assertion failed: 條件`。
- 測試放在 `#[cfg(test)] mod tests { use super::*; ... }`：`#[cfg(test)]` 先照抄，意思是只有測試時才編譯；`use super::*;` 讓測試用到外層的東西，子模組連私有函式都看得到。
- `cargo test` 逐一列出每個測試的結果與總結，失敗的會多一段 `failures:` 顯示 panic 訊息，結束碼不是 0。
- `cargo test 名稱` 只跑名字符合的測試。
