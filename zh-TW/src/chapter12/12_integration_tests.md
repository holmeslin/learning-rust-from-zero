# `tests/` 整合測試

## 本集目標

在 `tests/` 資料夾寫整合測試，站在使用者的角度，只透過公開的 API 測試 library。

## 正文

### 單元測試與整合測試

第 10 集的測試寫在程式旁邊的 `mod tests` 裡，叫**單元測試**，看得到私有的函式。**整合測試**是另一種：放在專案根目錄的 `tests/` 資料夾，每個檔案都是獨立的 crate，**把你的 library 當成外部的 crate 使用**，就像 `main.rs` 引用 `teashop` 那樣。所以它只能用 `pub` 的東西，測的是「使用者實際會碰到的功能」。

先準備一個 library，裡面有一個私有函式、一個公開函式，還有一個單元測試：

`src/lib.rs`：

```rust,test,noplayground
fn price_of(drink: &str) -> u32 {
    match drink {
        "紅茶" => 30,
        "奶茶" => 38,
        _ => 45,
    }
}

pub fn total_for(drink: &str, cups: u32) -> u32 {
    price_of(drink) * cups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_tea_costs_30() {
        assert_eq!(price_of("紅茶"), 30);
    }
}
```

### 寫整合測試

專案目錄多了一個 `tests/`：

```text
teashop/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── shop.rs
```

`tests/shop.rs` 跟一般程式一樣，用 `use teashop::...` 引用 library。不需要 `#[cfg(test)]`，整個檔案只有 `cargo test` 才會編譯：

`tests/shop.rs`：

```rust,ignore
use teashop::total_for;

#[test]
fn two_milk_teas() {
    assert_eq!(total_for("奶茶", 2), 76);
}
```

`cargo test` 的輸出（省略編譯進度）：

```text
     Running unittests src/lib.rs (target/debug/deps/teashop-32c5571f53fea115)

running 1 test
test tests::black_tea_costs_30 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/shop.rs (target/debug/deps/shop-2a30695097a4da2e)

running 1 test
test two_milk_teas ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests teashop

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

輸出分成三段，每一段開頭都有 `Running` 或 `Doc-tests`：

1. `unittests src/lib.rs`：`lib.rs` 裡的單元測試。
2. `tests/shop.rs`：這個整合測試檔，每個檔案各自一段。
3. `Doc-tests`：第 13 集的主題。

只想跑某個整合測試檔，用 `cargo test --test shop`（`shop` 是檔名，不含 `.rs`）。

### 只看得到 `pub` 的東西

整合測試跟 `main.rs` 一樣，是 library 外面的 crate。試著在 `tests/shop.rs` 引用私有的 `price_of`，就會得到跟第 4 集一樣的 E0603：`function price_of is private`。想測它，就寫在 `lib.rs` 的單元測試裡。

如果好幾個測試檔要共用一些輔助函式，放在 `tests/common/mod.rs`，再用 `mod common;` 引進來。`tests/` 底下的每個 `.rs` 檔都會被當成獨立的測試 crate（輸出裡各有一段），但子資料夾裡的 `mod.rs` 不會，所以輔助函式放在那裡，不會多出一段空的測試輸出。

### 為什麼要把邏輯放進 library

第 2 集說「邏輯放 `lib.rs`」，原因就在這裡：**只有 binary crate 的專案沒辦法寫整合測試**，因為 `main.rs` 編譯出來是可執行檔，別的 crate 沒辦法 `use` 它，`use 套件名::...` 會是 E0432（`unresolved import`）。想測 `main.rs` 裡的邏輯，就把邏輯搬進 `lib.rs`，`main.rs` 只留下呼叫它的幾行。

## 重點整理

- 單元測試寫在 `src` 的 `mod tests` 裡，看得到私有的東西；整合測試放在專案根目錄的 `tests/`，每個檔案是獨立的 crate，只能用 `pub` 的東西。
- 整合測試用 `use 套件名::...` 引用 library，不需要 `#[cfg(test)]`。
- `cargo test` 依序跑單元測試、每個 `tests/*.rs`、doc test；`cargo test --test 檔名` 只跑一個整合測試檔。
- 共用的輔助程式放 `tests/common/mod.rs`，不會被當成測試。
- 只有 binary crate 沒辦法寫整合測試，所以邏輯放 `lib.rs`、`main.rs` 保持很薄。
