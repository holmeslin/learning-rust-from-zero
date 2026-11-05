# 專案：每個檢查點都應該失敗

整份檔案是一個專案；每個檢查點各自判定，失敗了後面照樣檢查。測試靠標籤分辨：標籤是「輸出」或寫了（故意…）的區塊，就是應該失敗的位置。

## 拿掉 `pub`

`main.rs` 叫不到 `total_for`，書上卻說成功。

`src/lib.rs`：

```rust,ignore
fn total_for(price: u32, cups: u32) -> u32 {
    price * cups
}

pub fn tea_total(cups: u32) -> u32 {
    total_for(30, cups)
}
```

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::total_for(30, 2));
    println!("{}", teashop::tea_total(2));
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
60
60
```

## 沒排版

`src/lib.rs`（故意排版亂掉，後面沒有「`cargo fmt` 之後」）：

```rust,ignore
pub fn total_for(price:u32,cups:u32)->u32{price*cups}
```

## 警告都算失敗

clippy 的 `len_zero`（不在放行清單）：

`src/lib.rs`：

```rust,ignore
pub fn no_orders(orders: &[u32]) -> bool {
    orders.len() == 0
}
```

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::no_orders(&[]));
}
```

`cargo check` 的輸出：

```text
    Checking teashop v0.1.0
    Finished `dev` profile
```

rustc 的 `unused_variables`：

`src/lib.rs`：

```rust,ignore
pub fn total_for(price: u32, cups: u32) -> u32 {
    let unused = 1;
    price * cups
}
```

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::total_for(30, 2));
}
```

`cargo build` 的輸出：

```text
   Compiling teashop v0.1.0
    Finished `dev` profile
```

## 測試的結果跟書上不同

測試其實會失敗，書上卻貼成功：

`src/lib.rs`：

```rust,test,noplayground
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

`cargo test` 的輸出（省略編譯進度）：

```text
running 1 test
test tests::three_cups ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

測試其實會通過，書上卻貼失敗：

`src/lib.rs`：

```rust,test,noplayground
pub fn total_for(price: u32, cups: u32) -> u32 {
    price * cups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_cups() {
        assert_eq!(total_for(30, 3), 90);
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

## 錯誤的結果跟書上不同

書上貼錯誤，實際卻成功：

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::total_for(30, 2));
}
```

`cargo build` 的輸出：

```text
error[E0603]: function `total_for` is private
```

書上的錯誤碼跟實際的不同（實際是 E0425）：

`src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::no_such_function(2));
}
```

`cargo build` 的輸出：

```text
error[E0603]: function `no_such_function` is private
```

## 「`cargo fmt` 之後」跟 rustfmt 的結果不同

`src/main.rs`：

```rust,ignore
fn main(){println!("hi");}
```

`cargo fmt` 之後的 `src/main.rs`（故意跟 rustfmt 的結果不同）：

```rust
fn main() { println!("hi"); }
```

## 執行超過 30 秒

`src/main.rs`：

```rust,ignore
fn main() {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
```

## 最後一個檔案沒有任何檢查點

`src/extra.rs`（故意放在最後，之後沒有任何輸出區塊）：

```rust,ignore
pub fn extra() {}
```
