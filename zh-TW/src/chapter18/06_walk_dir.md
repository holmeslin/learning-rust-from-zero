# 走訪目錄

## 本集目標

寫出走訪資料夾的遞迴函式，處理 `read_dir` 的順序不固定、符號連結與錯誤，讓 `teashop` 掃過一整個資料夾的訂單檔。

## 正文

第 17 章第 3 集用過 `fs::read_dir`，它只列出**一層**。要走進子資料夾，標準函式庫沒有現成的函式，得自己遞迴。這一集的 `teashop` 接一個資料夾名稱，列出裡面所有 `.txt` 訂單檔（每行一筆訂單），數出各有幾筆。先準備一份資料，放在專案裡：

`orders/mon.txt`：

```text
紅茶:2
咖啡:1
```

`orders/tue.txt`：

```text
奶茶:3
```

`orders/archive/old.txt`：

```text
紅茶:10
```

`orders/notes.csv`：

```text
這不是訂單檔
```

### 遞迴走訪

`walk` 讀一個資料夾：遇到子資料夾就遞迴進去，遇到 `.txt` 就數行數，最後回傳這個資料夾底下的總筆數。`depth` 是目前的深度，用來縮排：

`src/main.rs`：

```rust,ignore
use std::fs;
use std::io;
use std::path::Path;
use std::process::ExitCode;

fn count_orders(path: &Path) -> io::Result<usize> {
    Ok(fs::read_to_string(path)?.lines().count())
}

fn walk(dir: &Path, depth: usize) -> io::Result<usize> {
    let mut entries = fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    let indent = "  ".repeat(depth);
    let mut total = 0;
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            println!("{indent}{name}/");
            total += walk(&path, depth + 1)?;
        } else if path.extension().is_some_and(|ext| ext == "txt") {
            let count = count_orders(&path)?;
            println!("{indent}{name} {count} 筆");
            total += count;
        }
    }
    Ok(total)
}

fn main() -> ExitCode {
    let Some(dir) = std::env::args().nth(1) else {
        eprintln!("用法：teashop <資料夾>");
        return ExitCode::from(2);
    };
    match walk(Path::new(&dir), 0) {
        Ok(total) => {
            println!("共 {total} 筆");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("掃描失敗：{:?}", e.kind());
            ExitCode::FAILURE
        }
    }
}
```

`cargo run -- orders` 的輸出（省略編譯進度）：

```text
     Running `target/debug/teashop orders`
archive/
  old.txt 1 筆
mon.txt 2 筆
tue.txt 1 筆
共 4 筆
```

`cargo run -- no-such-dir` 的輸出（省略編譯進度，結束碼 1）：

```text
     Running `target/debug/teashop no-such-dir`
掃描失敗：NotFound
```

幾個重點：

- **`read_dir` 的順序不保證**（第 17 章第 3 集），每個作業系統、每次都可能不同。工具的輸出要固定，所以先 `collect` 成 `Vec`，用 `sort_by_key` 依檔名排好再走。`collect::<Result<Vec<_>, _>>()?` 在任何一筆讀取失敗時整個回傳錯誤。
- 判斷是不是資料夾，用 `entry.file_type()?.is_dir()`，不要用 `path.is_dir()`。`DirEntry::file_type` 不會跟著符號連結走：連到資料夾的符號連結，`is_dir()` 是 `false`、`is_symlink()` 是 `true`。`path.is_dir()` 會跟過去：指回上層的符號連結會讓同一批檔案被重複走很多層（直到作業系統的連結層數上限才停），筆數就全錯了（實測：上面的資料加一個指回上層的連結，總數從 4 變成 164）。
- `"  ".repeat(depth)` 把字串重複 `depth` 次，拿來縮排；字串的 `lines()` 跟第 17 章的 `BufRead::lines` 一樣一行一行切，`count()` 數有幾行；`is_some_and(|ext| ext == "txt")` 是「有副檔名，而且是 `txt`」。
- 檔案名稱是 `OsString`（第 17 章第 4 集），顯示用 `to_string_lossy()`；只看路徑的最後一段，輸出就不會夾著各作業系統不同的路徑分隔符號。
- 錯誤用 `?` 往上傳，`main` 只印 `kind()`（`NotFound`），不印作業系統的原文，輸出才跟平台無關。

`?` 的代價是一個資料夾讀不了，整個掃描就中斷。實際的工具常常想要「印一則警告到 stderr、跳過它、繼續掃」，只要把 `read_dir` 失敗的分支改成 `eprintln!` 加 `return Ok(0)`，並在最後依有沒有警告決定結束碼。

標準函式庫沒有走訪目錄的函式，社群的 `walkdir` crate 就是做這件事的（這本書不展開，清單外的 crate 不用）。遞迴函式不長，先自己寫過一次，看過 `file_type` 與排序的細節，之後換 crate 也看得懂它在做什麼。

### 跟 Go 比一比

```go
filepath.WalkDir(root, func(path string, d fs.DirEntry, err error) error {
	if err != nil {
		return err
	}
	if !d.IsDir() && filepath.Ext(path) == ".txt" {
		fmt.Println(path)
	}
	return nil
})
```

- Go 的標準函式庫有 `filepath.WalkDir`：遞迴、依檔名排序、不跟符號連結，每個項目呼叫一次你的函式，錯誤也交給它。Rust 的標準函式庫只有一層的 `read_dir`，遞迴、排序、符號連結都得自己處理。
- `WalkDir` 的 callback 傳進來的是完整路徑，`Ext` 帶點（`.txt`）；Rust 的 `extension()` 不帶點（`txt`），而且回傳 `Option`。

## 重點整理

- 標準函式庫只有一層的 `fs::read_dir`；走訪整棵樹要自己遞迴，函式回傳 `io::Result`，用 `?` 往上傳錯誤。
- `read_dir` 的順序不保證：先 `collect` 成 `Vec` 再用 `sort_by_key` 排序，輸出才固定。
- 判斷資料夾用 `entry.file_type()?.is_dir()`，它不跟著符號連結走，不會被指回上層的連結帶進無窮遞迴；`path.is_dir()` 會跟。
- 檔名是 `OsString`，印出用 `to_string_lossy()`；錯誤印 `kind()`，不印作業系統的原文。
- 跟 Go 比：Go 有 `filepath.WalkDir`，Rust 標準函式庫沒有（社群有 `walkdir` crate）；`Ext` 帶點，`extension()` 不帶。
