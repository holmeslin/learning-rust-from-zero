# 原始指標

## 本集目標

認識 `*const T` 與 `*mut T`：怎麼建立、為什麼建立是安全的而解參照要 `unsafe`，跟參照差在哪裡，並看一個 `unsafe impl Send` 的實際用法。

## 正文

### 原始指標就是一個位址

**原始指標**（raw pointer）有兩種：`*const T` 與 `*mut T`，前者用來讀，後者可以寫。它們跟參照 `&T`、`&mut T` 長得像，保證卻少得多：

| | 參照 | 原始指標 |
| --- | --- | --- |
| 借用規則 | 編譯器檢查 | 不檢查，可以同時有好幾個 `*mut` |
| 會不會是 null | 不會 | 會 |
| 指向的東西一定還活著 | 是（生命週期保證） | 不保證，可能已經被釋放 |

建立原始指標是安全的，沒有 `unsafe`。用 `&raw const 變數` 或 `&raw mut 變數` 直接取得位址，也可以把參照用 `as` 轉成指標（`r as *mut i32`）。**解參照**才需要 `unsafe`：

```rust
fn main() {
    let mut cups = 3;
    let a = &raw mut cups;
    let b = &raw mut cups;
    unsafe {
        *a += 1;
        *b += 1;
    }
    println!("{cups}");
}
```

執行結果：

```text
5
```

`a` 和 `b` 同時指向 `cups`，兩個都能寫。換成 `&mut cups` 的話，這是 E0499；原始指標不受借用規則管，守規則的責任落到你身上。

沒寫 `unsafe` 就解參照，是 E0133：``dereference of raw pointer is unsafe and requires unsafe block``。`note` 把原因列得很清楚：原始指標可能是 null、可能已經失效、可能沒對齊，可能違反借用規則、可能造成 data race，這些都是未定義行為，編譯器沒辦法替你檢查。

### null 與指標算術

原始指標可以是 null，用 `std::ptr::null()` 建立、`is_null()` 檢查。解參照之前先檢查，是最基本的習慣：

```rust
fn read_price(p: *const u32) -> Option<u32> {
    if p.is_null() {
        None
    } else {
        Some(unsafe { *p })
    }
}

fn main() {
    let price = 30;
    println!("{:?}", read_price(&price));
    println!("{:?}", read_price(std::ptr::null()));
}
```

執行結果：

```text
Some(30)
None
```

`&price` 是 `&u32`，傳給 `*const u32` 的參數時會自動轉成指標。clippy 不讓你寫 `0 as *const u32`（`zero_ptr`），要用 `null()`。

指標也能往前走：`p.add(n)` 是往後移 `n` 個**元素**（不是 `n` 個位元組），這是 C 語言走訪陣列的方式。它同樣是 `unsafe`，移出原本那塊記憶體的範圍就是未定義行為。Rust 平常用索引或迭代器，不會自己這樣走。

### 把指標交給別的執行緒

第 14 章第 11 集說過，`Send` 與 `Sync` 是編譯器依欄位自動推斷的。原始指標一律不是 `Send`，因為編譯器不知道它指向的東西能不能被別的執行緒碰。包了指標的 struct 也因此不是 `Send`：

```rust,compile_fail=E0277
use std::thread;

struct CupsPtr(*mut u32);

impl CupsPtr {
    fn add_five(&self) {
        unsafe {
            *self.0 += 5;
        }
    }
}

fn main() {
    let mut cups = 0;
    let ptr = CupsPtr(&raw mut cups);
    let handle = thread::spawn(move || {
        ptr.add_five();
    });
    handle.join().expect("店員出事了");
    println!("{cups}");
}
```

編譯錯誤：

```text
error[E0277]: `*mut u32` cannot be sent between threads safely
  --> src/main.rs:16:32
   |
16 |       let handle = thread::spawn(move || {
   |                    ------------- ^------
   |                    |             |
   |  __________________|_____________within this `{closure@src/main.rs:16:32: 16:39}`
   | |                  |
   | |                  required by a bound introduced by this call
17 | |         ptr.add_five();
18 | |     });
   | |_____^ `*mut u32` cannot be sent between threads safely
   |
   = help: within `{closure@src/main.rs:16:32: 16:39}`, the trait `Send` is not implemented for `*mut u32`
```

這裡只貼前半段，後面還有幾則 `note`：`CupsPtr` 裡面有 `*mut u32`、閉包用到了它、`thread::spawn` 要求 `Send`。如果你確定這樣用沒問題（這個程式裡，只有新執行緒碰 `cups`，主執行緒 `join` 之後才讀），就替 `CupsPtr` 加上一行 `unsafe impl`：

```rust,ignore
struct CupsPtr(*mut u32);

unsafe impl Send for CupsPtr {}
```

這樣整支程式就能編譯，`cups` 最後是 5。`unsafe impl Send for CupsPtr {}` 的意思是「編譯器看不出來，但我保證它能安全交給別的執行緒」；保證錯了，就是 data race。

閉包裡要透過方法 `ptr.add_five()` 用它，不能直接寫 `*ptr.0 += 5`：閉包只會擷取它用到的欄位，直接碰 `ptr.0` 的話，被搬進去的是欄位本身（一個 `*mut u32`），不是整個 `CupsPtr`，`Send` 就白宣告了，還是 E0277。

### 跟 Go 比一比

Go 的指標 `*T` 一直都很安全：不能算術，GC 保證指向的東西還活著，頂多是 `nil`。所以 Go 裡寫 `*T` 的地方，在 Rust 要對應的是參照 `&T`／`&mut T`（或 `Box<T>`），不是原始指標。Rust 的原始指標比較像 C 的指標，用在 FFI（下一集）和少數要自己實作資料結構的地方，日常程式不會碰到。

## 重點整理

- `*const T`、`*mut T` 是原始指標：不受借用規則管、可能是 null、不保證指向的東西還活著。用 `&raw const`、`&raw mut`，或把參照用 `as` 轉換來建立（安全）。
- 解參照要 `unsafe`，不寫是 E0133；`is_null()` 檢查 null，`std::ptr::null()` 建立 null，`p.add(n)` 以元素為單位移動（也是 `unsafe`）。
- 原始指標不是 `Send`，包了它的 struct 也不是（E0277）；確定安全時用 `unsafe impl Send for 型別 {}` 宣告，保證錯了就是 data race。
- 閉包只擷取用到的欄位，所以要透過方法使用整個包裝型別，`unsafe impl Send` 才派得上用場。
- Go 的 `*T` 對應 Rust 的參照或 `Box<T>`，不是原始指標。
