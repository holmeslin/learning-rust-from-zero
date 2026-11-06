# `#[cfg]` 條件編譯

## 本集目標

會用 `#[cfg(...)]` 讓程式碼只在某些條件下才編譯，會寫 `not`、`all`、`any`，會用 `cfg!(...)`，並真正看懂第 12 章先照抄的 `#[cfg(test)]` 與 `#[cfg(feature = "名字")]`。

## 正文

### 條件不成立，程式碼就像沒寫過

`#[cfg(條件)]` 是屬性（附錄一 j），加在項目（函式、struct、`mod`、`use`……）前面。條件成立，項目照常編譯；不成立，編譯器直接把它丟掉，就像沒寫過：

```rust
#[cfg(debug_assertions)]
fn mode() -> &'static str {
    "debug"
}

#[cfg(not(debug_assertions))]
fn mode() -> &'static str {
    "release"
}

fn main() {
    println!("{}", mode());
}
```

執行結果：

```text
debug
```

`debug_assertions` 在 `cargo run`、`cargo build` 的預設（debug）編譯下成立，用 `--release` 就不成立，所以 `cargo run --release` 印出 `release`。兩個函式同名卻不衝突，因為編譯器只留下其中一個。

被丟掉的程式碼不會被編譯，也就不會做名稱解析和型別檢查。它仍然要能被**解析**（語法要正確），但裡面可以呼叫只有某個平台才有的函式，在別的平台上編譯器根本不看。

### 常見的條件

| 條件 | 什麼時候成立 |
| --- | --- |
| `test` | 用 `cargo test` 編譯的時候 |
| `debug_assertions` | debug 編譯（不是 `--release`） |
| `feature = "名字"` | `Cargo.toml` 宣告的 feature 被打開了 |
| `unix`、`windows` | 作業系統家族（Linux 與 macOS 都是 `unix`） |
| `target_os = "linux"` | 具體的作業系統（`"macos"`、`"windows"` 等） |
| `target_pointer_width = "64"` | 目標平台是 64 位元 |

條件可以組合：`not(條件)` 反過來，`all(甲, 乙)` 要兩個都成立，`any(甲, 乙)` 有一個成立就好，例如 `#[cfg(all(unix, target_pointer_width = "64"))]`。

### `cfg!`：條件變成 `bool`

`cfg!(條件)` 是巨集，直接展開成 `true` 或 `false`，可以寫在運算式裡：

```rust
fn main() {
    if cfg!(debug_assertions) {
        println!("檢查全開");
    } else {
        println!("檢查全關");
    }
    println!("{}", cfg!(target_pointer_width = "64"));
}
```

執行結果：

```text
檢查全開
true
```

跟 `#[cfg]` 不同，`cfg!` 的兩個分支**都會**被編譯，只是其中一邊永遠不會執行。所以兩邊都必須通過型別檢查：

```rust,compile_fail=E0308
fn main() {
    let x: u32 = if cfg!(debug_assertions) { 1 } else { "no" };
    println!("{x}");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:2:57
  |
2 |     let x: u32 = if cfg!(debug_assertions) { 1 } else { "no" };
  |                                                         ^^^^ expected `u32`, found `&str`
```

即使 debug 編譯下 `else` 永遠不會跑，`"no"` 是字串、不是 `u32`，照樣是 E0308。要讓某一段程式碼連編譯都不要，只能用屬性 `#[cfg]`。

### `test` 與 `feature` 的真面目

第 12 章先照抄的兩種寫法，其實都是 `#[cfg]`，只是條件由 cargo 決定：

- `cargo test` 編譯時，cargo 會替 rustc 加上 `--cfg test`，所以 `#[cfg(test)] mod tests { ... }` 只有測試的時候存在，`cargo build`、`cargo run` 的成品裡沒有它。
- `cargo run --features member` 會加上 `--cfg 'feature="member"'`。沒打開的時候，`#[cfg(feature = "member")]` 的項目被丟掉，用到它就是第 12 章第 15 集看到的 E0425，`note` 說它是 `configured out`。

### 打錯字的警告

`cfg` 的條件寫錯字，不會編譯失敗，只是條件永遠不成立，程式碼靜靜地消失，很難察覺。所以在 cargo 專案裡，rustc 會檢查條件的名字和值是不是認得的，不認得就警告：

`src/main.rs`：

```rust,warn
#[cfg(debug_assertion)]
fn extra() -> u32 {
    1
}

fn main() {
    println!("{}", 2);
}
```

`cargo run` 的輸出（省略編譯進度）：

```text
warning: unexpected `cfg` condition name: `debug_assertion`
 --> src/main.rs:1:7
  |
1 | #[cfg(debug_assertion)]
  |       ^^^^^^^^^^^^^^^ help: there is a config with a similar name: `debug_assertions`
  |
  = help: consider using a Cargo feature instead
  = help: or consider adding in `Cargo.toml` the `check-cfg` lint config for the lint:
           [lints.rust]
           unexpected_cfgs = { level = "warn", check-cfg = ['cfg(debug_assertion)'] }
  = help: or consider adding `println!("cargo::rustc-check-cfg=cfg(debug_assertion)");` to the top of the `build.rs`
  = note: see <https://doc.rust-lang.org/nightly/rustc/check-cfg/cargo-specifics.html> for more information about checking conditional configuration
  = note: `#[warn(unexpected_cfgs)]` on by default

warning: `teashop` (bin "teashop") generated 1 warning
     Running `target/debug/teashop`
2
```

`debug_assertion` 少了一個 `s`，警告直接指出名字不認得，並建議相近的 `debug_assertions`。`feature = "member"` 如果沒有在 `Cargo.toml` 的 `[features]` 宣告，也一樣會警告（`unexpected cfg condition value`）。`help` 還列了幾種登記自訂條件的辦法，其中一種寫在 `build.rs`，下一集會看到。

### 跟 Go 比一比

Go 也有條件編譯，叫 build constraint，單位是**整個檔案**：檔案最上面寫 `//go:build linux && amd64`，或把檔名取成 `store_windows.go`，不符合條件的檔案整份不編譯。`runtime.GOOS` 則是執行時的值，兩個平台的程式碼都會編譯進去。Rust 的 `#[cfg]` 可以加在檔案裡任何一個項目、陳述式，同一個檔案就能放不同平台的版本；`--features` 的角色，對應 Go 的 `-tags`。

## 重點整理

- `#[cfg(條件)]` 加在項目前面：條件不成立，編譯器把它丟掉，不編譯、不做型別檢查（但語法要能解析）。組合用 `not(…)`、`all(…)`、`any(…)`。
- 常見條件：`test`、`debug_assertions`、`feature = "名字"`、`unix`／`windows`、`target_os = "…"`、`target_pointer_width = "64"`。
- `cfg!(條件)` 展開成 `true`／`false`，兩個分支都會被編譯，都要通過型別檢查。
- `#[cfg(test)]` 與 `#[cfg(feature = "…")]` 的條件由 cargo 傳給 rustc（`--cfg test`、`--cfg 'feature="…"'`）。
- 在 cargo 專案裡，不認得的條件名稱或值（打錯字、沒宣告的 feature）是 `unexpected_cfgs` 警告。
- Go 的 build constraint 以整個檔案為單位；Rust 的 `#[cfg]` 可以加在任何項目上。
