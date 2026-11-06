# `unsafe`

## 本集目標

知道 `unsafe` 讓你多做哪幾件事、又有哪些規則照舊；會呼叫 `unsafe` 函式、自己寫 `unsafe fn`、使用 `static mut`，並學會「把 `unsafe` 包在小範圍、對外提供安全的函式」這個慣例。

## 正文

### 編譯器看不出來，不代表不對

Rust 的安全保證，建立在編譯器能檢查的規則上。有些事編譯器沒辦法證明是對的，卻可能真的是對的，例如「這個索引我確定沒有越界」。`unsafe` 就是留給這種情況的出口：在 `unsafe { }` 區塊裡，你多了五種能力，後果由你負責：

1. 解參照原始指標（第 6 集）。
2. 呼叫 `unsafe` 函式或方法（包括呼叫 C 的函式，第 7 集）。
3. 讀寫 `static mut`。
4. 實作 `unsafe trait`。
5. 讀 `union` 的欄位（這本書不展開）。

除了這五件事，其他規則一條都沒有放鬆。

### 呼叫 `unsafe` 函式，對外包成安全的

陣列和 `Vec` 的 `get_unchecked(i)` 跟 `[i]` 一樣取第 `i` 個元素，只是省掉越界檢查。它是 `unsafe` 方法，因為索引越界時結果是**未定義行為**（undefined behavior）：不是 panic，而是程式想做什麼都可能，包括當機、讀到別人的資料，或看起來一切正常。呼叫它而不寫 `unsafe`，是 E0133：``call to unsafe function `...get_unchecked` is unsafe and requires unsafe block``。

函式有編譯器看不出的前提、要呼叫端自己保證時，就宣告成 `unsafe fn`。慣例是把前提寫在 `# Safety` 小節裡：

```rust
/// # Safety
/// 呼叫的人要保證 `index < prices.len()`。
unsafe fn price_at(prices: &[u32], index: usize) -> u32 {
    unsafe { *prices.get_unchecked(index) }
}

fn price_or_zero(prices: &[u32], index: usize) -> u32 {
    if index < prices.len() {
        unsafe { price_at(prices, index) }
    } else {
        0
    }
}

fn main() {
    let prices = [30, 45, 38];
    println!("{}", price_or_zero(&prices, 1));
    println!("{}", price_or_zero(&prices, 9));
}
```

執行結果：

```text
45
0
```

- `unsafe fn` 的意思是「呼叫我之前，你得自己保證前提成立」。clippy 的 `missing_safety_doc` 會檢查有沒有 `# Safety` 小節。
- edition 2024 起，`unsafe fn` 的本體不再自動算 `unsafe`，裡面做不安全的事還是要寫 `unsafe { }`，不寫會有警告。
- `price_or_zero` 先檢查索引再呼叫 `price_at`，它自己是安全的函式，呼叫端不必寫 `unsafe`。這是最重要的慣例：`unsafe` 縮在小地方，在那裡把前提檢查清楚，對外只提供安全的介面。標準函式庫的 `Vec` 內部有不少 `unsafe`，你用起來卻一個都不用寫。

### `static mut`

第 14 章第 12 集說過，全域的 `static` 預設不可變。`static mut` 是可以修改的全域變數，整個程式只有一份，任何執行緒都能改，編譯器沒辦法保證不會同時改，所以每次讀寫都要 `unsafe`：

```rust
static mut CUPS: u32 = 0;

fn main() {
    unsafe {
        CUPS += 1;
        CUPS += 1;
    }
    let total = unsafe { CUPS };
    println!("{total}");
}
```

執行結果：

```text
2
```

沒寫 `unsafe` 就用它是 E0133，訊息說 ``use of mutable static is unsafe and requires unsafe block``。把整個值複製出來（`let total = unsafe { CUPS };`）沒問題。edition 2024 另外規定：不能對 `static mut` 取參照。`println!("{CUPS}")` 其實會替 `CUPS` 取一個參照，所以被擋下：

```rust,compile_fail
static mut CUPS: u32 = 0;

fn main() {
    unsafe {
        println!("{CUPS}");
    }
}
```

編譯錯誤：

```text
error: creating a shared reference to mutable static
 --> src/main.rs:5:20
  |
5 |         println!("{CUPS}");
  |                    ^^^^ shared reference to mutable static
  |
  = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives
```

後面還有幾則 `note` 說明同一件事。要在多條執行緒之間共用的計數器，請用第 14 章第 12 集的 atomic 或第 7 集的 `Mutex`，不要用 `static mut`。

### `unsafe` 不會關掉借用檢查

`unsafe` 只多給你上面五種能力，借用規則照樣檢查。下面在 `unsafe` 區塊裡同時借兩個 `&mut`，還是 E0499：

```rust,compile_fail=E0499
fn main() {
    let mut cups = [3, 4];
    unsafe {
        let a = cups.get_unchecked_mut(0);
        let b = cups.get_unchecked_mut(1);
        *a += 1;
        *b += 1;
    }
    println!("{cups:?}");
}
```

編譯錯誤：

```text
error[E0499]: cannot borrow `cups` as mutable more than once at a time
 --> src/main.rs:5:17
  |
4 |         let a = cups.get_unchecked_mut(0);
  |                 ---- first mutable borrow occurs here
5 |         let b = cups.get_unchecked_mut(1);
  |                 ^^^^ second mutable borrow occurs here
6 |         *a += 1;
  |         ------- first borrow later used here
```

### `unsafe trait`

有些 trait 要求實作的人保證編譯器看不出來的性質，要宣告成 `unsafe trait`，實作時也得寫 `unsafe impl`，少寫是 E0200。第 14 章第 11 集的 `Send` 和 `Sync` 就是：編譯器依欄位自動推斷，推斷不出來、而你確定沒問題時，才需要自己寫 `unsafe impl`。下一集會看到實際的例子。

### 跟 Go 比一比

Go 的 `unsafe` 是一個套件：`unsafe.Pointer`、`unsafe.Sizeof`，`import` 進來就能用，程式裡沒有哪一段被明確圈出來。Rust 的 `unsafe` 是關鍵字，每個用到的地方都被圈成區塊或標在函式上。想審查一份程式碼有哪些地方要特別小心，搜尋 `unsafe` 就找得到；不想有任何 `unsafe` 的專案，可以在 crate 最上面寫 `#![forbid(unsafe_code)]`（附錄一 j 的屬性），出現就是編譯錯誤。

## 重點整理

- `unsafe { }` 只多給五種能力：解參照原始指標、呼叫 `unsafe` 函式、讀寫 `static mut`、實作 `unsafe trait`、讀 `union` 欄位。其他規則（借用檢查、型別檢查）照舊，E0499 在 `unsafe` 裡一樣出現。
- 呼叫 `unsafe` 函式而不寫 `unsafe` 是 E0133。違反前提的後果是未定義行為，不是 panic。
- `unsafe fn` 把前提交給呼叫端，前提寫在 `# Safety` 小節；edition 2024 起，`unsafe fn` 裡不安全的操作也要包 `unsafe { }`。
- 慣例：`unsafe` 縮在小範圍，先檢查前提，對外提供安全的函式。
- `static mut` 的讀寫要 `unsafe`（E0133）；edition 2024 不能對它取參照（`static_mut_refs`，例如 `println!("{CUPS}")`）；共用計數器請用 atomic 或 `Mutex`。
