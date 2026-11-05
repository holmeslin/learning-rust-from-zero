# workspace

## 本集目標

用 workspace 把好幾個 package 放在同一個專案裡，共用編譯結果與 `Cargo.lock`，並學會用 `-p` 指定要編譯或執行哪一個。

## 正文

### 為什麼需要 workspace

專案長大之後，常常想拆成好幾個 package：一個 library、一個命令列工具、一個伺服器……各自有 `Cargo.toml`，但又希望一起編譯、版本一致。**workspace** 就是把幾個 package 放在同一個資料夾底下管理的方式：它們共用一份 `Cargo.lock` 和一個 `target/`。

### 建立 workspace

在最外層建一個只有 `[workspace]` 的 `Cargo.toml`，沒有 `[package]`，列出成員的資料夾：

`Cargo.toml`：

```toml
[workspace]
members = ["teashop", "teashop-cli"]
resolver = "3"
```

`resolver = "3"` 是 cargo 決定相依套件版本的方式（edition 2024 預設用 3），這種沒有 `[package]` 的根需要明寫，先照抄；漏了 cargo 會警告。兩個成員各是普通的 package。`teashop` 是 library：

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
pub fn total_for(drink: &str, cups: u32) -> u32 {
    let price = match drink {
        "紅茶" => 30,
        "奶茶" => 38,
        _ => 45,
    };
    price * cups
}
```

`teashop-cli` 是 binary，要用到同一個 workspace 裡的 `teashop`。它不在 crates.io 上，所以用第 14 集的 `path`：

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
    println!("共 {} 元", teashop::total_for("奶茶", 3));
}
```

整個專案的樣子：

```text
teashop-workspace/
├── Cargo.toml
├── Cargo.lock
├── target/
├── teashop/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── teashop-cli/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

`Cargo.lock` 與 `target/` 都在最外層，只有一份：兩個成員用到的 crate，版本一致，也只編譯一次。

### 指令：在根目錄下，用 `-p` 指定成員

在根目錄執行 `cargo build`，會編譯**所有**成員；`cargo test` 也會跑每個成員的測試。只想處理其中一個，用 `-p`（package）加名稱：

`cargo run -p teashop-cli` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop-cli`
共 114 元
```

`cargo build -p teashop`、`cargo test -p teashop` 也是一樣的寫法。在成員自己的資料夾裡執行 cargo，則預設只對那個成員。

在 workspace 根目錄執行 `cargo new 名稱`，cargo 會順便把新的 package 加進 `members`。想讓各成員共用同一個相依套件版本，可以用 `[workspace.dependencies]`，需要時再查文件。

## 重點整理

- workspace 是同一個資料夾底下的多個 package，共用一份 `Cargo.lock` 與一個 `target/`。
- 根目錄的 `Cargo.toml` 只放 `[workspace]`（`members` 列出成員的資料夾，`resolver = "3"` 先照抄）。
- 成員之間用 `path` 互相引用：`teashop = { path = "../teashop" }`。
- 在根目錄 `cargo build`、`cargo test` 處理全部成員；`-p 名稱` 指定其中一個。
- 在 workspace 裡 `cargo new`，會自動加入 `members`。
