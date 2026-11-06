# const 泛型

## 本集目標

會用 `const N: usize` 這種「值的泛型參數」，寫出對任意長度的陣列都能用的函式與型別，並知道它目前的限制，以及什麼時候該用切片就好。

## 正文

### 長度也能是泛型參數

第 8 章教的泛型參數都是**型別**：`T` 代表某個型別。陣列的型別是 `[u32; 3]`，長度 `3` 也是型別的一部分，所以 `[u32; 3]` 和 `[u32; 4]` 是兩個不同的型別。想寫一個「不管長度是多少」的函式，就需要讓長度也能當參數，這就是 **const 泛型**：

```rust
fn total<const N: usize>(prices: [u32; N]) -> u32 {
    prices.iter().sum()
}

fn main() {
    println!("{}", total([30, 45, 38]));
    println!("{}", total([30, 45]));
    println!("{}", total::<1>([30]));
}
```

執行結果：

```text
113
75
30
```

- `<const N: usize>` 宣告一個**值**的參數：名字叫 `N`，型別是 `usize`，傳進來的是編譯時期就確定的數字。
- 參數 `[u32; N]` 的長度用到了 `N`，所以呼叫 `total([30, 45, 38])` 時，編譯器從陣列的長度推論出 `N` 是 3，不用寫。要明寫的話，是 `total::<1>([30])`，跟第 8 章第 8 集的 `::<型別>` 一樣。
- 每個不同的 `N`，編譯器各產生一份專屬的函式，跟型別泛型一樣，沒有執行時期的成本。

### 型別也可以帶 const 參數

struct 和 `impl` 也能用。下面是一個固定格數的杯架，格數寫在型別上：

```rust
struct Tray<const N: usize> {
    cups: [Option<u32>; N],
}

impl<const N: usize> Tray<N> {
    fn new() -> Tray<N> {
        Tray { cups: [None; N] }
    }

    fn capacity(&self) -> usize {
        N
    }

    fn put(&mut self, price: u32) -> bool {
        for slot in &mut self.cups {
            if slot.is_none() {
                *slot = Some(price);
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut tray: Tray<2> = Tray::new();
    println!("{}", tray.capacity());
    println!("{}", tray.put(30));
    println!("{}", tray.put(45));
    println!("{}", tray.put(38));
}
```

執行結果：

```text
2
true
true
false
```

- `impl<const N: usize> Tray<N>` 的寫法跟 `impl<T> Shelf<T>`（第 8 章第 9 集）一樣，只是 `T` 換成 `const N: usize`。
- `N` 在函式裡可以當一般的 `usize` 值使用（`capacity` 直接回傳 `N`，`[None; N]` 用它當長度）。
- `Tray<2>` 只有兩格，第三杯放不進去，`put` 回傳 `false`。

`Tray<2>` 與 `Tray<3>` 是不同的型別，把一個傳給要求另一個的函式，是 E0308：

```rust,compile_fail=E0308
struct Tray<const N: usize> {
    cups: [Option<u32>; N],
}

fn show(tray: &Tray<3>) {
    println!("{}", tray.cups.len());
}

fn main() {
    let tray = Tray::<2> { cups: [None; 2] };
    show(&tray);
}
```

編譯錯誤：

```text
error[E0308]: mismatched types
  --> src/main.rs:11:10
   |
11 |     show(&tray);
   |     ---- ^^^^^ expected `3`, found `2`
   |     |
   |     arguments to this function are incorrect
   |
   = note: expected reference `&Tray<3>`
              found reference `&Tray<2>`
```

### 目前的限制

- 參數的型別只能是整數、`bool`、`char`。`const NAME: &str` 這種字串，編譯器會直接說只支援這三類。
- 傳進去的必須是編譯時期就確定的值：字面值、`const`，或另一個 const 泛型參數。拿 `let n = 2;` 的 `n` 寫成 `total::<n>(...)` 是 E0435（`attempt to use a non-constant value in a constant`）；執行時才知道長度的資料（例如 `Vec`），傳給要求 `[u32; N]` 的函式也不行，是 E0308。
- 型別裡不能對 `N` 做運算，`[u32; N * 2]` 會得到 ``generic parameters may not be used in const operations``；這個功能還沒穩定。
- 可以替 `N` 設預設值：`struct Tray<const N: usize = 4>`，寫成 `Tray` 就是 `Tray<4>`。

### 什麼時候用，什麼時候不用

如果只是要「讀一排價格」，用切片就好：`fn total(prices: &[u32]) -> u32` 任何長度都收，`total(&[30, 45, 38])` 和 `total(&[30, 45])` 都能呼叫，呼叫端不用在意長度。

const 泛型是為了「長度本身是型別的一部分」的情況：要把陣列**整個**（不是借用）傳進、傳出，要讓編譯器分辨兩份不同長度的資料不能混用（`Tray<2>` 與 `Tray<3>`），或是要在編譯時期就保證大小，例如固定大小的緩衝區、向量與矩陣。

### 跟 Go 比一比

Go 的泛型參數只能是型別，寫不出「長度是 `N` 的陣列」。`[3]int` 和 `[4]int` 在 Go 裡也是不同型別，但沒辦法替它們寫同一個泛型函式，所以 Go 幾乎都用切片 `[]int`。Rust 同樣可以用切片；const 泛型是多出來的選擇，讓你需要「長度是型別的一部分」的時候有辦法表達。

## 重點整理

- `fn f<const N: usize>(a: [T; N])`、`struct Tray<const N: usize>`、`impl<const N: usize> Tray<N>`：把數字當泛型參數，`N` 在本體裡可以當 `usize` 值用。
- 呼叫時 `N` 通常從陣列長度推論；要明寫用 `f::<3>(...)`；不同的 `N` 是不同型別，混用是 E0308。
- 限制：型別只能是整數、`bool`、`char`；傳入的值要在編譯時期確定（E0435）；型別裡不能做 `N * 2` 這類運算。
- 只是要讀任意長度的資料，用切片 `&[T]` 就好；const 泛型留給長度本身是型別一部分的情況。
- Go 的泛型參數只有型別，沒有對應的東西，通常改用切片。
