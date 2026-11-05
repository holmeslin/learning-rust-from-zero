# 專案：workspace，應該通過

根目錄的 `Cargo.toml` 只有 `[workspace]`，兩個成員各有自己的 `Cargo.toml`；路徑標籤寫相對於根目錄的完整路徑，所以不會加預設的 `Cargo.toml`。

`Cargo.toml`：

```toml
[workspace]
members = ["teashop", "teashop-cli"]
resolver = "3"
```

`teashop/Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`teashop/src/lib.rs`：

```rust,ignore
pub fn total_for(price: u32, cups: u32) -> u32 {
    price * cups
}
```

`teashop-cli/Cargo.toml`：

```toml
[package]
name = "teashop-cli"
version = "0.1.0"
edition = "2024"

[dependencies]
teashop = { path = "../teashop" }
```

`teashop-cli/src/main.rs`：

```rust,ignore
fn main() {
    println!("{}", teashop::total_for(38, 3));
}
```

`cargo run -p teashop-cli` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop-cli`
114
```
