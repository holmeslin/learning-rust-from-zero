# 專案：應該全部通過

整份檔案是一個專案：路徑標籤加程式區塊是檔案內容，`cargo …` 的輸出區塊是檢查點。沒有寫 `Cargo.toml` 時，預設是 package `teashop`。

## 一般的 library 加 binary

`cargo new teashop` 的輸出：

```text
    Creating binary (application) `teashop` package
```

`cargo new` 不驗證（它改檔案、不編譯），上面這個區塊不影響結果。

放行清單（`clippy-allow.txt`）裡的 lint 不算失敗，例如 `needless_return`：

`src/lib.rs`：

```rust,ignore
pub fn total_for(price: u32, cups: u32) -> u32 {
    return price * cups;
}
```

標籤後面可以加（說明），空行不限幾個：

`src/main.rs`（呼叫 library）：


```rust,ignore
fn main() {
    println!("{}", teashop::total_for(30, 2));
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
60
```

## 書上貼錯誤：必須失敗，而且錯誤碼要對

`src/lib.rs`：

```rust,ignore
pub fn total_for(price: u32, cups: u32) -> u32 {
    price_of(price) * cups
}

pub(crate) fn price_of(price: u32) -> u32 {
    price
}
```

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::price_of(30));
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
error[E0603]: function `price_of` is private
 --> src/main.rs:2:29
```

沒有錯誤碼的錯誤，只要求失敗：

`cargo build` 的輸出：

```text
error: could not compile `teashop` (bin "teashop") due to 1 previous error
```

## 排版亂掉的示範：後面接「`cargo fmt` 之後」

`src/main.rs`：

```rust,ignore
fn main(){
println!("{}",teashop::total_for(30,2));
}
```

`cargo fmt` 之後的 `src/main.rs`：

```rust
fn main() {
    println!("{}", teashop::total_for(30, 2));
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
60
```

## 測試：成功與失敗

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
        assert_eq!(total_for(30, 2), 60);
    }
}
```

`cargo test` 的輸出（省略編譯進度）：

```text
running 1 test
test tests::two_cups ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

故意寫錯的測試，`exit=101` 只是給單檔檢查看的，專案以檢查點為準：

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
        assert_eq!(total_for(30, 3), 100);
    }
}
```

`cargo test` 的輸出（省略編譯進度，某一次）：

```text
running 1 test
test tests::three_cups ... FAILED

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

## 文件測試：外層用四個反引號

`src/lib.rs`：

````rust,ignore
/// 計算總價。
///
/// ```
/// assert_eq!(teashop::total_for(30, 2), 60);
/// ```
pub fn total_for(price: u32, cups: u32) -> u32 {
    price * cups
}
````

`cargo test` 的輸出（省略編譯進度）：

```text
   Doc-tests teashop

running 1 test
test src/lib.rs - total_for (line 4) ... ok
```

## feature：沒打開時失敗，打開就成功

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[features]
member = []

[dependencies]
```

`src/lib.rs`：

```rust,ignore
pub fn price(drink: &str) -> u32 {
    if drink == "紅茶" { 30 } else { 45 }
}

#[cfg(feature = "member")]
pub fn member_price(drink: &str) -> u32 {
    price(drink) * 9 / 10
}
```

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::price("紅茶"));
    println!("{}", teashop::member_price("紅茶"));
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
error[E0425]: cannot find function `member_price` in crate `teashop`
```

`cargo run --features member` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
30
27
```

## `warn`：刻意示範警告的區塊，允許警告

`src/main.rs`：

```rust,warn
fn main() {
    let unused = 5;
}
```

`cargo check` 的輸出：

```text
warning: unused variable: `unused`
```
