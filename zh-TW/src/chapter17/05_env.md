# `std::env`

## 本集目標

用 `std::env::var` 讀環境變數並處理「沒設定」，知道為什麼 `set_var` 在 edition 2024 是 `unsafe`，也會用 `current_dir`、`temp_dir` 取得工作目錄與暫存目錄。

## 正文

### 讀環境變數：`env::var`

環境變數是行程（process）啟動時，由啟動它的程式傳進來的一組「名稱＝文字」，常拿來放不想寫死在程式裡的設定。`env::var(名稱)` 回傳 `Result<String, VarError>`：

```rust
use std::env;

fn main() {
    match env::var("TEASHOP_NO_SUCH_VAR") {
        Ok(value) => println!("值是 {value}"),
        Err(e) => println!("{e}"),
    }
    println!("{:?}", env::var("TEASHOP_NO_SUCH_VAR"));

    unsafe {
        env::set_var("TEASHOP_DISCOUNT", "15");
    }
    let discount: u32 = env::var("TEASHOP_DISCOUNT")
        .expect("沒有設定")
        .parse()
        .expect("不是整數");
    println!("折扣 {discount}%");
    println!("{:?}", env::var_os("TEASHOP_DISCOUNT"));

    unsafe {
        env::remove_var("TEASHOP_DISCOUNT");
    }
    let rate = env::var("TEASHOP_DISCOUNT").unwrap_or(String::from("0"));
    println!("沒設定就用預設：{rate}");
}
```

執行結果：

```text
environment variable not found
Err(NotPresent)
折扣 15%
Some("15")
沒設定就用預設：0
```

- 沒設定是 `Err(VarError::NotPresent)`，所以變數可能不存在是正常情況，用 `match` 或 `unwrap_or`（第 11 章第 18 集）給預設值；`VarError` 還有一種 `NotUnicode`，變數的值不是合法的 UTF-8。
- 環境變數永遠是文字，要數字就自己 `parse`（第 1 章第 10 集），而且要處理 `parse` 失敗。
- `env::var_os` 回傳 `Option<OsString>`，連不是 UTF-8 的值也拿得到，用法跟上一集的路徑一樣。
- 空字串與沒設定不同：`TEASHOP_EMPTY=` 讀到的是 `Ok("")`。

### 為什麼 `set_var` 是 `unsafe`

上面的 `set_var`、`remove_var` 都包在 `unsafe` 裡。從 edition 2024 起，不包是 E0133：

```rust,compile_fail=E0133
use std::env;

fn main() {
    env::set_var("TEASHOP_DISCOUNT", "15");
    println!("{:?}", env::var("TEASHOP_DISCOUNT"));
}
```

編譯錯誤：

```text
error[E0133]: call to unsafe function `set_var` is unsafe and requires unsafe block
 --> src/main.rs:4:5
  |
4 |     env::set_var("TEASHOP_DISCOUNT", "15");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
  |
  = note: consult the function's documentation for information on how to avoid undefined behavior
```

原因在作業系統：環境變數放在整個行程共用的一塊記憶體，C 函式庫讀寫它時並不保證執行緒安全。一條執行緒在改、另一條在讀，就是未定義行為（第 16 章第 5 集），而編譯器管不到其他執行緒（甚至其他語言寫的程式庫）。所以 `unsafe` 區塊裡的承諾是：**同一時間沒有別的執行緒在讀寫環境變數**。實務上的做法：

- 設定只在 `main` 一開始、還沒開執行緒之前寫；
- 更好的是根本不改自己的環境變數，要給子行程設定，用下一集的 `Command::env`，那是安全的；
- 測試程式的執行緒是平行的，在測試裡 `set_var` 特別容易出事。

### 走訪全部：`env::vars`

`env::vars()` 回傳所有 `(名稱, 值)`，順序不固定，拿來找特定字首的設定：

```rust
use std::env;

fn main() {
    unsafe {
        env::set_var("TEASHOP_MODE", "dev");
        env::set_var("TEASHOP_DISCOUNT", "15");
    }
    let mut mine: Vec<(String, String)> = Vec::new();
    for (key, value) in env::vars() {
        if key.starts_with("TEASHOP_") {
            mine.push((key, value));
        }
    }
    mine.sort();
    println!("{mine:?}");
}
```

執行結果：

```text
[("TEASHOP_DISCOUNT", "15"), ("TEASHOP_MODE", "dev")]
```

整份環境變數每台機器都不同，所以只挑自己設定的字首，排序後才印。

### 目前目錄與暫存目錄

`env::current_dir()` 是程式目前所在的工作目錄，也是上一集相對路徑的起點，`env::temp_dir()` 是作業系統的暫存目錄，兩個都回傳絕對路徑。需要暫存檔就放在 `temp_dir()` 底下，用完自己刪：

```rust
use std::env;
use std::fs;

fn main() {
    println!("{}", env::current_dir().expect("讀不到目錄").is_absolute());
    let work = env::temp_dir().join("teashop-env-demo");
    fs::create_dir_all(&work).expect("建立失敗");
    fs::write(work.join("a.txt"), "紅茶").expect("寫入失敗");
    let text = fs::read_to_string(work.join("a.txt")).expect("讀取失敗");
    println!("{text:?}");
    fs::remove_dir_all(&work).expect("刪除失敗");
}
```

執行結果：

```text
true
"紅茶"
```

具體是哪個目錄依系統與設定而不同（Linux 通常是 `/tmp`，macOS 是 `/var/folders/…`），所以書上不印出路徑。命令列引數 `env::args()` 留到第 18 章第 1 集；`build.rs`（第 16 章第 11 集）裡讀 `OUT_DIR` 與 `CARGO_*`，用的就是這集的 `env::var`。跟編譯時期讀環境變數的 `env!` 巨集不同：`env::var` 是程式執行時才讀。

### 跟 Go 比一比

- Go 的 `os.Getenv("X")` 在「沒設定」和「設成空字串」都回傳 `""`，分不出來；要分得開得用 `os.LookupEnv`。Rust 的 `env::var` 本來就分成 `Err(NotPresent)` 與 `Ok("")`。
- Go 的 `os.Setenv` 是普通函式，內部有鎖；Rust 2024 的 `set_var` 要 `unsafe`，因為它直接動 C 函式庫那份沒有鎖的環境變數。

## 重點整理

- `env::var(名稱)` 回傳 `Result<String, VarError>`，沒設定是 `Err(NotPresent)`；`var_os` 回傳 `Option<OsString>`；值永遠是文字，要數字自己 `parse`。
- `env::set_var`、`env::remove_var` 是 `unsafe`（edition 2024，不包是 E0133），因為環境變數是全行程共用、讀寫不是執行緒安全的；只在還沒開執行緒時設定，給子行程設定改用 `Command::env`。
- `env::vars()` 走訪全部，順序不固定。
- `env::current_dir()` 是工作目錄，`env::temp_dir()` 是暫存目錄，都回傳絕對路徑的 `PathBuf`；暫存檔放在 `temp_dir()` 底下、用完刪掉。
- 跟 Go 比：Go 的 `Getenv` 分不出沒設定與空字串。
