# `Path` 與 `PathBuf`

## 本集目標

用 `Path` 與 `PathBuf` 拆解、組合檔案路徑，分得清楚兩者的關係（像 `&str` 與 `String`），看懂 `impl AsRef<Path>`，並避開 `join` 遇到絕對路徑的陷阱。

## 正文

### 路徑不是字串

作業系統的路徑不一定是合法的 UTF-8，分隔符號也依平台而不同，所以標準函式庫給路徑自己的型別，放在 `std::path`：

| 型別 | 像誰 | 說明 |
| --- | --- | --- |
| `Path` | `str` | 借來的路徑，通常以 `&Path` 出現，不能增減 |
| `PathBuf` | `String` | 擁有的路徑，能 `push`、`pop` |

`Path::new("shop/menu.txt")` 不會複製，直接把字串看成 `&Path`；`PathBuf::from("shop")` 才會建立擁有的版本。`PathBuf` 實作了 `Deref<Target = Path>`（第 13 章第 3 集），所以 `Path` 的方法對 `PathBuf` 也能呼叫。`&Path` 不能當 `&str` 用：

```rust,compile_fail=E0308
use std::path::Path;

fn main() {
    let p = Path::new("shop/menu.txt");
    let s: &str = p;
    println!("{s}");
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
 --> src/main.rs:5:19
  |
5 |     let s: &str = p;
  |            ----   ^ expected `&str`, found `&Path`
  |            |
  |            expected due to this
  |
  = note: expected reference `&str`
             found reference `&Path`
```

要拿到文字，用 `p.to_str()`（回傳 `Option<&str>`，不是合法的 UTF-8 就是 `None`）、`p.to_string_lossy()`，或印出來用的 `p.display()`。本書的輸出都是 Linux、macOS 的樣子，分隔符號是 `/`；Windows 是 `\`。

### 拆解與組合

```rust
use std::path::{Path, PathBuf};

fn main() {
    let menu = Path::new("shop/menu/tea.txt");
    println!("{:?}", menu.parent());
    println!("{:?}", menu.file_name());
    println!("{:?} {:?}", menu.file_stem(), menu.extension());

    let mut dir = PathBuf::from("shop");
    dir.push("menu");
    dir.push("coffee.txt");
    println!("{}", dir.display());
    dir.pop();
    let json = dir.join("tea.txt").with_extension("json");
    println!("{}", json.display());
    println!("{}", json == Path::new("shop/menu/tea.json"));
    println!("{:?}", json.components().collect::<Vec<_>>());
}
```

執行結果：

```text
Some("shop/menu")
Some("tea.txt")
Some("tea") Some("txt")
shop/menu/coffee.txt
shop/menu/tea.json
true
[Normal("shop"), Normal("menu"), Normal("tea.json")]
```

- 拆解：`parent()` 是上一層、`file_name()` 是最後一段、`file_stem()` 是不含副檔名的檔名、`extension()` 是副檔名（**不含點**）。這些都回傳 `Option`，因為路徑可能沒有那一段，例如 `Path::new("tea").extension()` 是 `None`。
- 組合：`PathBuf` 的 `push` 加一段、`pop` 拿掉最後一段（回傳 `bool`），`join` 不改原本的、回傳新的 `PathBuf`，`with_extension` 換副檔名。
- 兩個路徑用 `==` 比較的是逐段（`components`），不是逐字比字串，所以不必擔心多餘的 `/`。

### `join` 遇到絕對路徑會整個換掉

```rust
use std::path::Path;

fn main() {
    let base = Path::new("shop");
    let absolute = Path::new("/tmp/tea.txt");
    println!("{}", absolute.is_absolute());
    println!("{}", base.join(absolute).display());
}
```

執行結果：

```text
true
/tmp/tea.txt
```

接上去的如果是絕對路徑，`join` 的結果就是那個絕對路徑，前面的 `shop` 整個不見。路徑來自使用者輸入時，這是安全問題（有人傳 `/etc/passwd`）：先檢查 `is_absolute()`，或確認它不含 `..`。如果參數是寫死的字串字面值，clippy 會直接警告（`join_absolute_paths`）。

### `impl AsRef<Path>`：字串、`String`、`Path` 都能傳

上一集的 `fs::write("teashop-demo/menu.txt", ...)` 直接傳字串，是因為路徑參數寫成「任何能當成 `&Path` 看的型別」：`impl AsRef<Path>`。`&str`、`String`、`&Path`、`PathBuf` 都實作了 `AsRef<Path>`，函式裡呼叫 `.as_ref()` 拿到 `&Path`：

```rust
use std::path::{Path, PathBuf};

fn describe(path: impl AsRef<Path>) {
    let path = path.as_ref();
    println!("{}：{}", path.display(), path.exists());
}

fn main() {
    describe("no-such-file.txt");
    describe(String::from("no-such-dir"));
    describe(Path::new("."));
    describe(PathBuf::from("."));
}
```

執行結果：

```text
no-such-file.txt：false
no-such-dir：false
.：true
.：true
```

`exists()` 查路徑存不存在，另外有 `is_file()`、`is_dir()`。自己寫要收路徑的函式，也照這樣寫，呼叫的人就不必先轉換型別。

### 跟 Go 比一比

```go
filepath.Join("shop", "/tmp/tea.txt") // "shop/tmp/tea.txt"
filepath.Ext("archive.tar.gz")        // ".gz"
```

- Go 的路徑就是 `string`，`filepath.Join` 把每一段接起來，遇到 `/tmp/tea.txt` 也只是接在後面。Rust 的 `join` 遇到絕對路徑會整個取代，這是兩邊最容易搞混的地方。
- Go 的 `filepath.Ext` 回傳帶點的 `.gz`，沒有副檔名是空字串；Rust 的 `extension()` 回傳不帶點的 `Option`，沒有就是 `None`。
- Go 用 `string` 就不會有「不是合法 UTF-8」的型別問題，Rust 把這件事放進型別裡，換來轉文字時要多一步。

## 重點整理

- `Path` 是借來的路徑（像 `&str`），`PathBuf` 是擁有的路徑（像 `String`）；`Path::new` 不複製，`PathBuf` 有 `push`、`pop`；`&Path` 不是 `&str`，轉文字用 `to_str()`、`to_string_lossy()`、`display()`。
- `parent`、`file_name`、`file_stem`、`extension` 拆解路徑，都回傳 `Option`；副檔名不含點；`join`、`with_extension` 回傳新的 `PathBuf`；`==` 逐段比較。
- `join` 遇到絕對路徑會整個取代前面的，路徑來自使用者時要先檢查 `is_absolute()`。
- 函式收 `impl AsRef<Path>`，`&str`、`String`、`Path`、`PathBuf` 都能傳；`exists()`、`is_file()`、`is_dir()` 查路徑。
