# trait 物件 vs 泛型的取捨

## 本集目標

比較泛型（`<T: Trait>`）、trait 物件（`dyn Trait`）與 enum 三種寫法的成本和限制，能依情況選一種；知道 trait 要「dyn 相容」才能做成 trait 物件。

## 正文

### 同一件事，三種寫法

第 8 章第 15 集說過，泛型在編譯時期決定型別，`dyn` 在執行時期決定。這一集把成本攤開來比。用一個題目：把一排飲料的價錢加總。

```rust
trait Priced {
    fn price(&self) -> u32;
}

struct Tea;
struct Coffee;

impl Priced for Tea {
    fn price(&self) -> u32 {
        35
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        45
    }
}

enum Drink {
    Tea,
    Coffee,
}

impl Drink {
    fn price(&self) -> u32 {
        match self {
            Drink::Tea => 35,
            Drink::Coffee => 45,
        }
    }
}

fn total_generic<T: Priced>(items: &[T]) -> u32 {
    items.iter().map(|item| item.price()).sum()
}

fn total_dyn(items: &[Box<dyn Priced>]) -> u32 {
    items.iter().map(|item| item.price()).sum()
}

fn total_enum(items: &[Drink]) -> u32 {
    items.iter().map(|item| item.price()).sum()
}

fn main() {
    println!("{}", total_generic(&[Tea, Tea]));
    println!("{}", total_dyn(&[Box::new(Tea), Box::new(Coffee)]));
    println!("{}", total_enum(&[Drink::Tea, Drink::Coffee]));
    println!("{} {}", size_of::<Drink>(), size_of::<Box<dyn Priced>>());
}
```

執行結果：

```text
70
80
80
1 16
```

`total_generic(&[Tea, Coffee])` 編不過，是 E0308：`T` 一次只能是一種型別，混放就要換成 `dyn` 或 enum。三種寫法的差別：

| | 泛型 `<T: Priced>` | `dyn Priced` | enum |
| --- | --- | --- | --- |
| 能放的型別 | 一次一種 | 任何實作了 trait 的型別，別人的 crate 日後新增的也行 | 只有列在 enum 裡的 |
| 呼叫怎麼決定 | 編譯時期，可以內嵌（inline） | 執行時期查 vtable | `match` 的分支 |
| 資料放哪裡 | 就地放 | 通常要 `Box`，每個元素一次 heap 配置 | 就地放，不用 heap |
| 機器碼 | 每個型別一份 | 一份 | 一份 |
| 新增一種 | 多一個 `impl` | 多一個 `impl` | 改 enum，每個 `match` 都要跟著改 |

### `dyn` 靠 vtable

`&dyn Priced` 是胖指標（附錄二 a）：資料的位址，加上 vtable 的位址。vtable 是編譯器替每一組「型別加 trait」做好的表，記著這個型別的大小、對齊、怎麼 drop，還有每個方法的位址。呼叫 `item.price()` 要先讀表、再跳過去，這是間接呼叫，編譯器看不出會跳到哪裡，也就無法內嵌；一排 `Box<dyn Priced>` 還各自指到 heap 的不同位置。`size_of_val` 用在 `&dyn Priced` 上，量到的是背後真正的型別的大小，也是從 vtable 讀出來的。

enum 的成本最直接：`Drink` 只有 1 位元組的標記，`match` 就是比較標記，沒有 heap、沒有間接呼叫。代價是型別集合是封閉的。

### 不是每個 trait 都能做成 `dyn`

要做成 `dyn`，編譯器必須能替 trait 造一張 vtable。有些方法放不進去，這樣的 trait 就不是 dyn 相容（dyn compatible，舊稱 object safe）：

```rust,compile_fail=E0038
trait Priced {
    fn doubled(&self) -> Self;
}

struct Tea;

impl Priced for Tea {
    fn doubled(&self) -> Tea {
        Tea
    }
}

fn main() {
    let _item: Box<dyn Priced> = Box::new(Tea);
}
```

編譯錯誤：

```text
error[E0038]: the trait `Priced` is not dyn compatible
  --> src/main.rs:14:24
   |
14 |     let _item: Box<dyn Priced> = Box::new(Tea);
   |                        ^^^^^^ `Priced` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
  --> src/main.rs:2:26
   |
 1 | trait Priced {
   |       ------ this trait is not dyn compatible...
 2 |     fn doubled(&self) -> Self;
   |                          ^^^^ ...because method `doubled` references the `Self` type in its return type
   = help: consider moving `doubled` to another trait
   = help: only type `Tea` implements `Priced`; consider using it directly instead.
```

`doubled` 回傳 `Self`，但 `dyn Priced` 背後是哪個型別、多大，要到執行時期才知道，回傳值的大小不固定，vtable 放不進去。最後一則 `help` 只是因為這個例子只有一個型別實作了 `Priced`，不用管。其他常見的情況：方法有泛型型別參數（`fn f<T>(&self, x: T)`，每個 `T` 一個版本，表放不下）、沒有 `self` 的關聯函式（沒有物件可以查表）。

不想放棄整個 trait，可以在那個方法後面加 `where Self: Sized`：這個方法就不進 vtable，只能在具體型別上呼叫（`Tea.doubled()`），其他方法照樣能用在 `dyn` 上。

### 怎麼選

- 型別的集合在編譯時期就知道，而且不會再增加：enum，最直接，編譯器還會檢查 `match` 有沒有漏。
- 一個位置永遠只有一種型別，或是小而熱的函式（迴圈裡呼叫的，需要內嵌）：泛型。
- 要把不同型別放進同一個容器、型別到執行時期才知道（依設定檔載入）、或公開的介面要讓別人加新型別：`dyn`。
- 擔心編譯時間或執行檔大小：`dyn` 只有一份程式碼。也可以只讓外層是泛型，裡面立刻轉成具體型別，再呼叫另一個不泛型的函式，這樣被複製的只有薄薄一層，標準函式庫的 `fs` 函式就常這樣寫。
- 拿不定主意：先用泛型或 enum，真的需要混放才改 `dyn`，改動通常不大；成本差距要量過才算數（附錄二 e）。

## 重點整理

- 泛型：每個型別一份程式碼，編譯時期決定、可內嵌，一次只能一種型別；`dyn Trait`：一份程式碼，執行時期查 vtable，可以混放，通常要 `Box`；enum：型別集合封閉，沒有 heap 與間接呼叫。
- `&dyn Trait` 是胖指標，16 位元組：資料位址加 vtable 位址；vtable 記著大小、對齊、drop 與各方法。
- trait 要 dyn 相容才能做成 `dyn`：方法不能回傳 `Self`、不能有泛型型別參數、不能是沒有 `self` 的關聯函式，否則是 E0038；方法加 `where Self: Sized` 就會被排除在 vtable 之外。
- 選擇：封閉集合用 enum，單一型別或熱路徑用泛型，開放集合或混放用 `dyn`。
