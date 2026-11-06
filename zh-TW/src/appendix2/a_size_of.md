# 記憶體配置與 `size_of`

## 本集目標

用 `size_of` 與 `align_of` 量出型別在記憶體裡佔多少、怎麼排；知道哪些資料在 stack、哪些在 heap，以及指標與 `Option` 為什麼是現在這個大小。

## 正文

### 量大小

`size_of::<型別>()` 回傳型別的大小，單位是位元組，編譯時期就決定了；`size_of_val(&值)` 量一個值。它們與 `align_of` 都在 prelude 裡，不必 `use`。這一集的數字都是 Rust 1.99 在 64 位元電腦上量到的（`usize` 是 8 位元組）：

```rust
fn main() {
    println!("{} {}", size_of::<u8>(), size_of::<i32>());
    println!("{} {}", size_of::<f64>(), size_of::<usize>());
    println!("{} {}", size_of::<bool>(), size_of::<char>());
    println!("{} {}", size_of::<()>(), size_of::<(i32, u8)>());
    println!("{} {}", size_of::<[u32; 4]>(), size_of::<[u8; 0]>());
}
```

執行結果：

```text
1 4
8 8
1 4
0 8
16 0
```

數字型別的大小是位元數除以 8，陣列是元素大小乘以個數。`()` 與 `[u8; 0]` 是 0，叫零大小型別（zero-sized type），完全不佔空間。`(i32, u8)` 是 8，不是 4 + 1，原因在下一節。

### 對齊與填充

CPU 讀 `i32` 時，位址是 4 的倍數最有效率，所以每個型別都有**對齊**（alignment，`align_of` 量得到）：值必須放在對齊的倍數的位置。編譯器因此會在欄位之間補空位，叫**填充**（padding），整個型別的大小也要補成對齊的倍數：

```rust
#[allow(dead_code)]
struct Mixed {
    small: u8,
    big: u32,
    tiny: u8,
}

#[allow(dead_code)]
#[repr(C)]
struct InOrder {
    small: u8,
    big: u32,
    tiny: u8,
}

fn main() {
    println!("{} {}", size_of::<Mixed>(), align_of::<Mixed>());
    println!("{} {}", size_of::<InOrder>(), align_of::<InOrder>());
}
```

執行結果：

```text
8 4
12 4
```

這裡只量大小、不讀欄位，所以用附錄一 j 的 `#[allow(dead_code)]` 關掉警告。兩個 struct 的欄位一模一樣，大小卻差了 4：

```text
InOrder： [small][.填充.][  big  ][tiny][.填充.]   共 12
Mixed：   [  big  ][small][tiny][填充]             共 8
```

- `#[repr(C)]` 要求欄位照宣告的順序排（第 16 章第 7 集跟 C 交換資料時用到）。`big` 要放在 4 的倍數，所以 `small` 後面補 3 位元組；結尾再補 3 位元組，大小才是 4 的倍數。
- 預設的排法，編譯器可以自己重排欄位。它把 `big` 放最前面，省掉中間的填充，所以是 8。
- 預設排法是 1.99 目前的做法，**語言沒有保證**。別假設欄位在記憶體裡的順序，也別把 `size_of` 的數字寫進檔案格式。

### 指標有胖有瘦，heap 不算在內

```rust
fn main() {
    println!("{} {}", size_of::<&u8>(), size_of::<&[u8; 100]>());
    println!("{} {}", size_of::<&str>(), size_of::<&[u32]>());
    println!("{} {}", size_of::<String>(), size_of::<Vec<u8>>());
    println!("{} {}", size_of::<&str>(), size_of_val("哈囉"));

    let mut prices: Vec<u32> = Vec::new();
    for i in 0..1000 {
        prices.push(i);
    }
    let in_heap = prices.len() * size_of::<u32>();
    println!("{} {}", size_of_val(&prices), in_heap);
}
```

執行結果：

```text
8 8
16 16
24 24
16 6
24 4000
```

- 指向固定大小資料的參照與 `Box` 只有一個位址，8 位元組；指到 1 位元組還是 100 位元組都一樣。
- `&str` 與 `&[T]` 是 16：位址加長度。這種多帶一份資訊的指標叫**胖指標**（fat pointer）；`&dyn Trait` 也是，額外的資訊是 vtable 的位址（附錄二 c）。
- `String` 與 `Vec` 是 24：位址、長度、容量，就是第 3 章第 2 集畫的三個欄位。
- `size_of_val("哈囉")` 量的是字串本身，兩個字各 3 位元組，共 6；`size_of::<&str>()` 才是參照的 16。
- `size_of` 只量值本身，不追到 heap：`prices` 放了 1000 個元素，stack 上還是 24；4000 位元組的內容在 heap，要自己用長度乘以元素大小。

### `Option` 的空位

```rust
fn main() {
    println!("{} {}", size_of::<u32>(), size_of::<Option<u32>>());
    println!("{} {}", size_of::<&u32>(), size_of::<Option<&u32>>());
    println!("{} {}", size_of::<Box<u8>>(), size_of::<Option<Box<u8>>>());
    println!("{} {}", size_of::<String>(), size_of::<Option<String>>());
    println!("{} {}", size_of::<bool>(), size_of::<Option<bool>>());
}
```

執行結果：

```text
4 8
8 8
8 8
24 24
1 1
```

`Option` 是 enum，要記住現在是 `Some` 還是 `None`，所以 `Option<u32>` 多一個標記（tag），補上對齊就是 8；enum 的大小都是標記加上最大的 variant。

但 `Option<&u32>` 一樣是 8，沒有多出標記：參照永遠不是 0，編譯器就拿 0 代表 `None`。這個借來用的「不可能的值」叫空位（niche）；`Box` 不會是 0，`bool` 只用 0 與 1，所以都不必多出標記。標準函式庫只保證 `Option<&T>` 與 `Option<Box<T>>` 跟指標一樣大，`Option<String>`、`Option<bool>` 是 1.99 目前的結果。

### 跟 Go 比一比

Go 不重排 struct 的欄位，順序決定大小：`struct { small uint8; big uint32; tiny uint8 }` 是 12，把 `big` 排到最前面才是 8，所以 Go 程式常為了省空間手動排欄位順序。Rust 的預設排法由編譯器代勞，欄位照好讀的順序排就好。

## 重點整理

- `size_of::<T>()`、`align_of::<T>()` 量型別的大小與對齊，`size_of_val(&x)` 量值；都在 prelude 裡。本集的數字，是 1.99 在 64 位元電腦上量到的。
- 欄位要放在對齊的倍數上，中間補填充，結尾補到對齊的倍數。預設排法會重排欄位省填充（沒有保證），`#[repr(C)]` 照宣告順序排。
- `&T`、`Box<T>` 是 8；`&str`、`&[T]`、`&dyn Trait` 是胖指標，16；`String`、`Vec` 是 24。`size_of` 只量 stack 上的值，heap 上的內容要自己算。
- enum 是標記加最大的 variant；`Option<&T>`、`Option<Box<T>>` 靠空位不多佔空間。
- 跟 Go 比：Go 不重排 struct 欄位，順序影響大小；Rust 預設由編譯器重排。
