# 加入 crate 與 `Cargo.lock`

## 本集目標

把別人寫好的 crate 加進專案，看懂版本號碼的寫法，並知道 `Cargo.lock` 記了什麼、為什麼要提交。

## 正文

### 加進 `[dependencies]`

[crates.io](https://crates.io) 是 Rust 的官方套件庫，別人寫好的函式庫都發布在上面，它們也是 crate。要用它們，在 `Cargo.toml` 的 `[dependencies]` 底下加一行，名稱加版本號碼。用 `cargo add` 讓 cargo 幫你寫：

```bash
cargo add rand
```

`rand` 是產生亂數的 crate，這集只看怎麼加進專案，怎麼用第 17 章第 11 集才教。執行之後 `Cargo.toml` 變成：

`Cargo.toml`：

```toml
[package]
name = "teashop"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10.3"
```

你看到的版本號碼可能比這裡新，那是 crates.io 上的最新版。下次執行 `cargo build` 或 `cargo run` 時，cargo 會自動下載 `rand` 和它用到的其他 crate，一起編譯。

本書第一部的範例只用標準函式庫，第二部才真的寫程式去用 crate。

### 版本號碼的意思

`"0.10.3"` 看起來像「就是 0.10.3」，其實意思是「相容 0.10.3 的版本都可以」。版本號碼依 SemVer 規則，由主版號、次版號、修訂號三段數字組成，主版號一變就表示有不相容的改動。常見寫法：

| 寫法 | 可以用的版本 |
| --- | --- |
| `"1.4.2"` | 1.4.2 以上、2.0.0 以前 |
| `"0.10.3"` | 0.10.3 以上、0.11.0 以前（主版號是 0 時，次版號變了就算不相容） |
| `"=1.4.2"` | 只有 1.4.2 |
| `">=1.2, <1.5"` | 這個範圍內 |

所以 `Cargo.toml` 裡寫的是**範圍**。那實際用的是哪個版本？記在 `Cargo.lock`。

### `Cargo.lock`：記下實際用的版本

第一次 `cargo build` 之後，專案根目錄會出現 `Cargo.lock`，cargo 自動產生與維護，不要手改。它記錄**每一個**用到的 crate 的確切版本、來源與雜湊，包括相依套件自己用到的套件。只加了 `rand` 一個，裡面就有 9 個 package（含你自己的）。`rand` 與你自己的 package 的部分：

`Cargo.lock` 裡的片段：

```toml
[[package]]
name = "rand"
version = "0.10.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "65c9fb96cbc91e3478eaae79a69fcd3f1ae4ad052e471fe6732fff548984b4af"
dependencies = [
 "chacha20",
 "getrandom",
 "rand_core",
]

[[package]]
name = "teashop"
version = "0.1.0"
dependencies = [
 "rand",
]
```

有了 `Cargo.lock`，之後的 `cargo build` 就照裡面記的版本來編，不會自己升級。這樣你、同事、CI 編出來的是同一組版本。所以：

- **給別人執行的程式（binary）要把 `Cargo.lock` 提交進版本控制**。
- 想升級，執行 `cargo update`：在 `Cargo.toml` 允許的範圍內，把 `Cargo.lock` 更新成最新版。
- 在 `Cargo.toml` 改了版本範圍，或加了新的 crate，cargo 也會自動更新 `Cargo.lock`。

### `[dev-dependencies]` 與其他來源

只有測試（第 10 集）、範例才會用到的 crate，放在 `[dev-dependencies]`，編譯正式成品時不會包含它：

```toml
[dev-dependencies]
rand = "0.10.3"
```

相依套件也可以不來自 crates.io，例如本機另一個專案（第 16 集會用到），或 git 儲存庫：

```toml
[dependencies]
teashop = { path = "../teashop" }
menu = { git = "https://github.com/example/menu" }
```

### 跟 Go 比一比

Go 的 `go.mod` 裡 `require` 寫的已經是確切的版本，Go 會選出「符合所有需求的最低版本」，`go.sum` 只放雜湊。Rust 的 `Cargo.toml` 寫的是版本**範圍**，cargo 會選範圍內的最新版，所以才需要 `Cargo.lock` 把實際選到的版本記下來，作用接近 Go 的 `go.mod` 加 `go.sum`。

## 重點整理

- 用 `cargo add 名稱`，或手動在 `[dependencies]` 加 `名稱 = "版本"`，就能加入 crates.io 上的 crate；下次編譯時 cargo 會自動下載。
- `"0.10.3"` 是範圍：主版號沒變就算相容（主版號是 0 時，連次版號都不能變）；`"=1.4.2"` 才是鎖死單一版本。
- `Cargo.lock` 由 cargo 自動產生，記錄每個 crate（含相依套件用到的套件）的確切版本；binary 專案要提交它，才能讓每個人編出同一組版本。
- `cargo update` 在範圍內升級 `Cargo.lock`；`[dev-dependencies]` 放只有測試用的 crate；相依套件也可以來自本機路徑（`path`）或 git 儲存庫（`git`）。
- 跟 Go 比：Go 的 `go.mod` 本身就寫確切版本；Cargo 的 `Cargo.toml` 寫範圍，確切版本在 `Cargo.lock`。
