# 零成本抽象

## 本集目標

知道「零成本抽象」在 Rust 裡指什麼：泛型、迭代器、閉包、`Option`、newtype 這些抽象，執行時不會比手寫慢；也知道成本轉移到哪裡、哪些東西不算零成本。

## 正文

### 兩個承諾

零成本抽象有兩半：用不到的功能，不必付費；用到的功能，自己手寫也不會更快。這裡的「成本」是指執行時期要多付的東西：多出來的資料、額外的間接呼叫、heap 配置。

### 抽象沒有多出資料

附錄二 a 學會了 `size_of`，正好拿來檢查：

```rust
struct Money(u32);

struct Tea;

fn main() {
    let money = Money(30);
    println!("{} {}", money.0, size_of::<Money>());
    println!("{} {}", size_of::<Tea>(), size_of::<[Tea; 1000]>());

    let rate = 5u32;
    let range = 0..10u32;
    let doubled = (0..10u32).map(|x| x * 2);
    let chain = (0..10u32).filter(|x| x % 2 == 0).map(|x| x * 2);
    let with_rate = (0..10u32).map(move |x| x + rate);
    println!("{}", size_of_val(&range));
    println!("{} {}", size_of_val(&doubled), size_of_val(&chain));
    println!("{}", size_of_val(&with_rate));
}
```

執行結果：

```text
30 4
0 0
8
8 8
12
```

- 包了一層的 `Money(u32)`（第 5 章第 5 集的 tuple struct）跟 `u32` 一樣大，包裝本身沒有成本。`Tea` 沒有欄位，一千個 `Tea` 也是 0。
- `map`、`filter` 串起來的迭代器，大小跟原本的範圍（8）一樣：沒有帶額外的資料，也沒有中間的 `Vec`。沒有擷取變數的閉包是 0；擷取了 `rate`（4）就多 4，剛好是 12。
- 附錄二 a 的 `Option<&T>` 也是同一類：多了一層 `Option` 的安全，大小跟指標一樣。

### 迭代器與手寫迴圈

```rust
fn total_loop(prices: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 0..prices.len() {
        if prices[i] >= 40 {
            sum += prices[i] * 2;
        }
    }
    sum
}

fn total_iter(prices: &[u32]) -> u32 {
    prices.iter().filter(|p| **p >= 40).map(|p| p * 2).sum()
}

fn main() {
    let prices = [30, 45, 38, 50];
    println!("{}", total_loop(&prices));
    println!("{}", total_iter(&prices));
}
```

執行結果：

```text
190
190
```

結果一樣，不稀奇；零成本說的是：用 `--release` 編譯的時候，編譯器通常把 `total_iter` 編成跟 `total_loop` 幾乎相同的機器碼，閉包被展開、沒有每個元素一次的函式呼叫。這要讀組合語言才看得到，而組合語言每個平台都不同，這本書不展開。但有個前提：不開最佳化的 debug 編譯，這些層層函式呼叫都還在，會慢很多（附錄二 e 會量一次）。

### 泛型：每個型別各編一份

```rust
fn double<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
    println!("{}", std::any::type_name::<T>());
    x + x
}

fn main() {
    println!("{}", double(3_i32));
    println!("{}", double(2.5_f64));
}
```

執行結果：

```text
i32
6
f64
5
```

`type_name` 回傳型別的名稱。`double` 只寫了一次，編譯器卻替用到的每個 `T` 各生一份專用的版本，叫**單態化**（monomorphization）：`double::<i32>` 用整數加法，`double::<f64>` 用浮點數加法，呼叫哪一份在編譯時期就決定了，還能把函式內嵌（inline）進呼叫的地方。

成本沒有消失，只是搬了家：每個型別一份，編譯變慢，執行檔也變大。`Vec<u8>` 與 `Vec<String>` 的 `push` 各有一份專用的程式碼。第 8 章第 15 集的 `dyn Trait` 反過來：只有一份，換成執行時期查表（附錄二 c 比較兩者）。

### 零成本不等於免費

「自己手寫也不會更快」的另一面是：你用了功能，就得付它該付的。`RefCell<u32>` 是 16 位元組，`Cell<u32>` 只有 4，多的是借用計數，`borrow_mut()` 時還要檢查；`Rc` 要維護參照計數；`Box<dyn Trait>` 多一次 heap 配置與間接呼叫；`v[i]` 要檢查範圍；debug 編譯的整數運算要檢查溢位。這些你不要就不用付，要用就算手寫也一樣貴。

### 跟 Go 比一比

Go 的編譯器同樣會最佳化，差別在成本寫在哪裡。把值放進會逃出函式的介面，Go 通常會在 heap 配置一份，寫出來的程式看不出來，要用 `go build -gcflags=-m` 才看得到「escapes to heap」；介面的方法呼叫預設是動態的，編譯器有時才能最佳化掉。Rust 把這些寫在型別上：看到 `Box`、`Rc`、`dyn` 才有 heap 或動態呼叫的成本，沒看到就沒有。

## 重點整理

- 零成本抽象：用不到的不付費，用到的手寫也不會更快；指的是執行時期的成本。
- newtype、沒有欄位的 struct、不擷取變數的閉包、`map` 與 `filter` 串成的迭代器，都不多佔空間；`Option<&T>` 跟指標一樣大。
- 泛型靠單態化：每個用到的型別各編一份，沒有執行時期成本，成本轉到編譯時間與執行檔大小。
- 前提是開最佳化（`--release`）。`RefCell`、`Rc`、`Box<dyn Trait>`、索引檢查不是零成本，是「你用了才付」。
- 跟 Go 比：Go 的 heap 配置與動態呼叫由編譯器決定、看不出來；Rust 寫在型別上。
