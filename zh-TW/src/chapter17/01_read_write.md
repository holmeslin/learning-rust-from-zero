# `Read` 與 `Write`

## 本集目標

認識標準函式庫讀寫位元組用的兩個 trait：`std::io::Read` 與 `std::io::Write`。讀完你會用它們讀寫記憶體裡的資料，也會寫出一個函式，目的地是螢幕或記憶體都能用。

## 正文

### 位元組進、位元組出

`Read` 代表「讀得出位元組的東西」，`Write` 代表「寫得進位元組的東西」。標準輸入只能讀、標準輸出只能寫，檔案（下一集）兩個都行，網路連線、記憶體裡的 `Vec<u8>` 也是。這一集全部用記憶體裡的資料，不碰檔案，輸出才會固定。

### 寫：`write_all` 與 `writeln!`

`Vec<u8>` 實作了 `Write`，寫進去的位元組就存在 `Vec` 裡：

```rust
use std::io::Write;

fn main() {
    let mut buf: Vec<u8> = Vec::new();
    buf.write_all("紅茶 30 元\n".as_bytes()).expect("寫入失敗");
    writeln!(buf, "咖啡 {} 元", 45).expect("寫入失敗");
    println!("{} 個位元組", buf.len());
    let text = String::from_utf8(buf).expect("不是 UTF-8");
    print!("{text}");
}
```

執行結果：

```text
28 個位元組
紅茶 30 元
咖啡 45 元
```

- `"…".as_bytes()` 把 `&str` 看成位元組切片 `&[u8]`，因為 `write_all` 收的是位元組。
- `writeln!` 跟第 16 章第 1 集的 `write!` 同一類，第一個引數換成實作了 `Write` 的東西，就寫到那裡去。
- 兩個都回傳 `std::io::Result<()>`，不處理會有 `unused_must_use` 警告，所以用 `.expect()`。
- 寫完的位元組不一定是文字。`String::from_utf8(buf)` 檢查是合法的 UTF-8 才轉成 `String`，回傳 `Result`。

### 方法要先 `use` 這個 trait

`write_all` 是 `Write` 的方法。沒有 `use std::io::Write;`，編譯器就看不到它：

```rust,compile_fail=E0599
fn main() {
    let mut buf: Vec<u8> = Vec::new();
    buf.write_all("紅茶".as_bytes()).expect("寫入失敗");
    println!("{}", buf.len());
}
```

編譯錯誤：

```text
error[E0599]: no method named `write_all` found for struct `Vec<u8>` in the current scope
 --> src/main.rs:3:9
  |
3 |     buf.write_all("紅茶".as_bytes()).expect("寫入失敗");
  |         ^^^^^^^^^
```

後面還有一則 `help`，直接建議加上 `use std::io::Write;`。`Read` 也一樣。第 16 章第 1 集寫進 `String` 用的是 `std::fmt::Write`（寫文字），跟這裡的 `std::io::Write`（寫位元組）同名，同一個檔案要同時用，得有一個用 `as` 改名（第 12 章第 5 集）。

### 同一個函式，不同的目的地

`Vec<u8>` 與標準輸出都是 `Write`，所以參數可以寫成「任何 `Write`」，也就是第 8 章第 4 集的 `impl Trait`：

```rust
use std::io::Write;

fn print_receipt(out: &mut impl Write, drink: &str, price: u32) {
    writeln!(out, "{drink}：{price} 元").expect("寫入失敗");
}

fn main() {
    print_receipt(&mut std::io::stdout(), "紅茶", 30);
    let mut buf: Vec<u8> = Vec::new();
    print_receipt(&mut buf, "咖啡", 45);
    println!("{:?}", String::from_utf8(buf).expect("不是 UTF-8"));
}
```

執行結果：

```text
紅茶：30 元
"咖啡：45 元\n"
```

第一次寫到螢幕，第二次寫進記憶體再檢查。函式不直接 `println!`、改收一個 `Write`，測試時餵 `Vec<u8>` 就能比對內容。

### 讀：`read_to_string` 與 `read`

`&[u8]` 也實作了 `Read`，讀的時候會把切片往前縮，所以變數要 `mut`。最常用的是 `reader.read_to_string(&mut text)`：把剩下的全部讀進 `String`，讀完再讀只會得到空字串；要讀進 `Vec<u8>` 用 `read_to_end`。標準輸入也是 `Read`，`std::io::stdin().read_to_string(&mut text)` 會讀到輸入結束為止；逐行讀的寫法在下一集。

讀不完的資料（太大、還在慢慢送來）用最底層的 `read`：拿一個陣列當緩衝區，每次最多讀滿它，回傳這次讀到幾個位元組，**回傳 0 代表沒有資料了**：

```rust
use std::io::Read;

fn main() {
    let mut reader = "紅茶".as_bytes();
    let mut chunk = [0u8; 4];
    loop {
        let n = reader.read(&mut chunk).expect("讀取失敗");
        if n == 0 {
            break;
        }
        println!("讀到 {n} 個位元組：{:?}", &chunk[..n]);
    }
}
```

執行結果：

```text
讀到 4 個位元組：[231, 180, 133, 232]
讀到 2 個位元組：[140, 182]
```

「紅茶」是 6 個位元組，緩衝區只有 4 格，所以分兩次，第一次還切在「茶」的中間（第 7 章第 9 集）。`read` 不保證填滿緩衝區，所以**一定要看回傳的 `n`**；clippy 甚至擋下「呼叫 `read` 卻不用回傳值」（`unused_io_amount`）。平常用 `read_to_string`，或 `read_to_end`（讀進 `Vec<u8>`）就夠了。

### 跟 Go 比一比

Go 的 `io.Reader`、`io.Writer` 長得很像，差別在細節：

```go
n, err := r.Read(buf) // 結尾時 err == io.EOF，而且 n 可能同時大於 0
```

- Go 靠 `io.EOF` 這個特殊的錯誤表示結束，`n > 0` 與 `err != nil` 還能同時成立。Rust 的 `read` 結束就是 `Ok(0)`，`Err` 一定是真的出事。
- Go 的 `Write` 寫不完一定回傳錯誤；Rust 的 `write` 可能只寫一部分還是 `Ok(n)`，所以平常用 `write_all`，它會寫到全部完成。
- Go 的 interface 方法隨時能呼叫，也能直接忽略 `fmt.Fprintf` 的錯誤；Rust 的 trait 方法要先 `use`，`writeln!` 的 `Result` 不處理會有警告。

## 重點整理

- `std::io::Read`、`std::io::Write` 是讀寫位元組的 trait，`Vec<u8>`、`&[u8]`、標準輸入輸出、檔案、網路連線都實作了它們。
- `write_all`、`writeln!` 寫進任何 `Write`，回傳 `io::Result`；`as_bytes()` 把 `&str` 轉成 `&[u8]`，`String::from_utf8` 轉回 `String`。
- trait 的方法要先 `use`，沒有 `use std::io::Write;` 是 E0599；`io::Write`（位元組）與 `fmt::Write`（文字）同名，同時用就 `as` 改名。
- `read_to_string`、`read_to_end` 讀完全部；底層的 `read` 回傳讀到幾個位元組，0 代表結束。
- 函式收 `&mut impl Write`，同一份程式既能寫螢幕也能寫進 `Vec<u8>`，方便測試。
