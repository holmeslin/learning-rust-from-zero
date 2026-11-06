# FFI 與 `unsafe extern "C"`

## 本集目標

會宣告並呼叫 C 語言的函式（`unsafe extern "C"`、`safe fn`、`c"..."` 字串），也知道怎麼寫出能讓 C 呼叫的 Rust 函式（`#[unsafe(no_mangle)]`、`extern "C" fn`）。

## 正文

### FFI 是什麼

**FFI**（foreign function interface）是呼叫別種語言寫的函式。作業系統與大多數底層函式庫都提供 C 的介面，所以 FFI 幾乎就是在跟 C 打交道。Rust 程式本來就連結了 C 標準函式庫，下面的 `strlen`、`atoi` 都不用另外安裝。

### 宣告要呼叫的 C 函式

C 的函式沒有 Rust 的簽名可以讀，要自己用 `unsafe extern "C"` 區塊把簽名寫出來：

```rust
use std::ffi::{c_char, c_int};

unsafe extern "C" {
    safe fn toupper(c: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn atoi(s: *const c_char) -> c_int;
}

fn main() {
    let upper = toupper(b'a' as c_int);
    println!("{}", upper as u8 as char);
    let len = unsafe { strlen(c"紅茶".as_ptr()) };
    println!("{len}");
    let n = unsafe { atoi(c"42".as_ptr()) };
    println!("{}", n + 1);
}
```

執行結果：

```text
A
6
43
```

- `extern "C"` 指定呼叫慣例是 C 的。大括號裡只寫簽名，沒有本體；函式由系統的 C 函式庫提供。
- edition 2024 的區塊一定要寫成 `unsafe extern`，省略 `unsafe` 是編譯錯誤：``extern blocks must be unsafe``。因為簽名是你寫的，編譯器看不到 C 的標頭檔，簽名寫錯就是未定義行為，這份責任由你簽名承擔。
- 區塊裡的函式預設是 `unsafe`，呼叫要寫 `unsafe { }`，不寫是 E0133。如果某個函式不管收到什麼值都不會出事，可以標成 `safe fn`，呼叫時就不用 `unsafe`，例如 `toupper`，它只是收一個整數。
- `std::ffi` 有對應 C 型別的名字：`c_int` 是 C 的 `int`，`c_char` 是 `char`，`*const c_char` 就是 `const char *`，`usize` 對應 `size_t`。

### C 的字串

C 字串是一串位元組，結尾有一個 `\0`（NUL），沒有長度欄位。Rust 的 `str` 沒有結尾的 `\0`，所以不能直接交給 C。

- `c"紅茶"` 是 **C 字串字面值**，編譯器自動在結尾補上 `\0`，型別是 `&CStr`；`.as_ptr()` 取出 `*const c_char`。`strlen` 數的是位元組，所以「紅茶」是 6。
- 執行時才得到的字串，用 `CString::new(字串)` 轉換。字串中間如果有 `\0`，C 會以為在那裡結束，所以它回傳 `Result`，遇到 `\0` 是 `Err`，要先處理。

### 讓 C 呼叫 Rust

反過來，也可以把 Rust 函式交給 C 呼叫：

```rust
#[unsafe(no_mangle)]
pub extern "C" fn price_of(cups: u32) -> u32 {
    cups * 30
}

fn main() {
    println!("{}", price_of(3));
}
```

執行結果：

```text
90
```

- `extern "C" fn` 讓這個函式使用 C 的呼叫慣例。
- `#[unsafe(no_mangle)]` 要編譯器保留 `price_of` 這個名字。平常編譯器會把函式名稱改成另一個編碼過的長名字（name mangling），C 就找不到了。edition 2024 要寫成 `unsafe(no_mangle)`，因為名字跟別的符號撞到是未定義行為；舊寫法 `#[no_mangle]` 會得到 ``unsafe attribute used without unsafe``。
- 要讓 C 真的呼叫到，得把 crate 編成 C 能連結的格式（`Cargo.toml` 的 `crate-type` 設成 `staticlib` 或 `cdylib`），這本書不展開。
- 跨過 FFI 的型別只能是 C 認得的：整數、浮點數、指標。`String`、`Vec` 不行。要傳 struct 的話，欄位的排列要跟 C 一樣，在 struct 上加 `#[repr(C)]`。

### 跟 Go 比一比

Go 呼叫 C 靠 cgo：程式最上面的註解寫 `#include`，再 `import "C"`，之後用 `C.strlen`、`C.CString`：

```go
text := C.CString("紅茶")
defer C.free(unsafe.Pointer(text))
fmt.Println(C.strlen(text))
```

這要求機器上有 C 編譯器，而且 `C.CString` 配置的記憶體要自己 `free`。cgo 的呼叫比一般函式貴，交叉編譯也麻煩，所以許多 Go 專案會盡量避開 cgo。Rust 的 FFI 沒有額外的工具：宣告簽名就能呼叫，成本跟一般函式呼叫差不多。代價是 Rust 不會替你讀 C 的標頭檔，簽名完全靠你手寫（大型的函式庫通常用 `bindgen` 這類工具自動產生，這本書不展開）。

## 重點整理

- FFI 是呼叫其他語言（主要是 C）的函式。用 `unsafe extern "C" { fn 名字(參數) -> 型別; }` 宣告，edition 2024 一定要寫 `unsafe extern`。
- 區塊裡的函式預設是 `unsafe`（呼叫要 `unsafe { }`，否則 E0133）；沒有任何前提的才標 `safe fn`。簽名寫錯是未定義行為，由你負責。
- C 型別用 `std::ffi` 的 `c_int`、`c_char` 等；C 字串字面值寫 `c"..."`（`&CStr`），`.as_ptr()` 取指標；執行時的字串用 `CString::new`，中間有 `\0` 會回傳 `Err`。
- 讓 C 呼叫 Rust：`#[unsafe(no_mangle)]` 加 `pub extern "C" fn`；跨界的型別只能是 C 認得的，struct 加 `#[repr(C)]`；編成 `staticlib` 或 `cdylib` 才能連結（本書不展開）。
- cgo 要 C 編譯器、字串要手動 `free`；Rust 的 FFI 只要宣告簽名，呼叫成本跟一般函式差不多。
