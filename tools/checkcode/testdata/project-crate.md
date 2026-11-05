# 專案：用 deps 裡固定版本的 crate，應該通過

`Cargo.toml` 的 `[dependencies]` 寫了共用依賴清單裡的 crate：checkcode 用 `deps/Cargo.lock` 當專案的鎖定檔，所以版本跟 `deps/Cargo.toml` 一致，編譯也共用 `deps/target` 的快取。

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1.0.104"
```

`src/main.rs`：

```rust,ignore
fn main() -> anyhow::Result<()> {
    let cups: u32 = "42".parse()?;
    println!("{cups}");
    Ok(())
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop`
42
```
