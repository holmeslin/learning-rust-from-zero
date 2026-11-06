# `rand`

## 本集目標

用 `rand` 產生亂數：`rand::rng()`、`random_range`、`random_bool`；用固定的種子讓亂數可以重現，並用 `shuffle` 與 `choose` 處理序列。

## 正文

標準函式庫沒有亂數，要加 crate（第 12 章第 14 集教過怎麼加）：

```toml
[dependencies]
rand = "0.10.3"
```

這一集的範例都有 `noplayground` 標註，網頁上沒有執行按鈕，請在自己的專案裡跑。

### 擲骰子

`rand::rng()` 回傳這條執行緒專用的亂數產生器，種子來自作業系統。方法定義在 `RngExt` 這個 trait 上，要先 `use`（第 1 集說過，trait 的方法要先 `use`）：

```rust,noplayground
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    for _ in 0..3 {
        let dice: u32 = rng.random_range(1..=6);
        println!("擲出 {dice}");
    }
    let lucky = rng.random_bool(0.3);
    let ratio: f64 = rng.random();
    println!("{lucky} {ratio:.2}");
}
```

某一次的輸出（每次都不同）：

```text
擲出 6
擲出 3
擲出 3
false 0.23
```

- `random_range(1..=6)` 在範圍裡挑一個，範圍的寫法跟第 1 章第 14 集的 `for` 一樣：`1..=6` 含 6，`1..6` 不含。空的範圍（`5..5`）會 panic（`cannot sample empty range`）。
- `random_bool(0.3)` 有 30% 的機率是 `true`。
- `random()` 要靠型別決定產生什麼：`f64` 是 0.0（含）到 1.0（不含）；整數是整個範圍；`bool` 是五五波。
- 隨手一次的用途，可以不建 `rng`，直接呼叫 `rand::random::<u8>()` 或 `rand::random_range(10..20)`。

網路上很多範例寫 `rand::thread_rng()`、`rng.gen_range(..)`、`use rand::Rng;`，那是舊版的名字。0.10 版全部改了：`rand::rng()`、`random_range`、`RngExt`，用舊的會得到 E0425 或 E0599。

### 固定種子：同樣的種子，同樣的亂數

測試、除錯、遊戲存檔常常需要「看起來隨機、但可以重現」。亂數產生器其實是一個演算法，從種子開始算出一串數字；種子相同，數字就一樣：

```rust,noplayground
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..5 {
        let dice: u32 = rng.random_range(1..=6);
        print!("{dice} ");
    }
    println!();
    let mut again = StdRng::seed_from_u64(42);
    let first: u32 = again.random_range(1..=6);
    println!("{first}");
}
```

執行結果：

```text
1 4 2 4 6 
1
```

- `StdRng::seed_from_u64(42)` 用固定的種子建立產生器（`SeedableRng` 這個 trait 提供）。同一個種子再建立一次，第一個數字又是 1。
- 這個序列對「這個版本的 `rand`」成立；升級 `rand` 之後演算法可能不同，所以不要把種子當成永久的存檔格式。
- `rand::rng()` 不能指定種子。需要重現的程式，一開始就用 `StdRng`，並把產生器當參數傳給需要亂數的函式。

### 洗牌與挑選

`rand::seq` 替切片加了兩個常用的方法：`shuffle`（原地洗牌，`SliceRandom`）與 `choose`（挑一個，`IndexedRandom`），一樣要先 `use`：

```rust,noplayground
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::{IndexedRandom, SliceRandom};

fn main() {
    let mut rng = StdRng::seed_from_u64(1);
    let mut drinks = vec!["紅茶", "咖啡", "奶茶", "烏龍茶", "綠茶"];
    drinks.shuffle(&mut rng);
    println!("{drinks:?}");
    println!("{:?}", drinks.choose(&mut rng));

    let empty: Vec<u32> = Vec::new();
    println!("{:?}", empty.choose(&mut rng));
}
```

執行結果：

```text
["紅茶", "烏龍茶", "奶茶", "咖啡", "綠茶"]
Some("綠茶")
None
```

`choose` 回傳 `Option<&T>`：空的切片沒東西可挑，是 `None`。`shuffle` 需要 `&mut` 切片，所以 `drinks` 要 `mut`。這兩個方法吃的是 `&mut rng`，換成 `rand::rng()` 的結果就是每次都不同。

### 跟 Go 比一比

```go
n := rand.IntN(6) + 1 // math/rand/v2：先取 0 到 5，再加 1
```

- Go 的 `rand.IntN(6)` 是「0 到 5」，要自己加 1；Rust 直接寫範圍 `1..=6`，邊界一目了然。
- Go 1.20 起，`math/rand` 的全域函式預設用隨機種子，早年的 `rand.Seed(time.Now().UnixNano())` 已經不需要；要重現就自己建立帶種子的產生器（`rand.New(rand.NewSource(42))`），做法跟 `StdRng::seed_from_u64` 相同。
- Go 的 `math/rand` 不是給密碼學用的，產生金鑰、token 要用 `crypto/rand`；Rust 做安全用途前，先讀 `rand` 文件對安全性的說明，本書不展開。

## 重點整理

- `rand = "0.10.3"`；`rand::rng()` 是這條執行緒的亂數產生器，方法在 `RngExt` 上，要 `use rand::RngExt;`；舊名字 `thread_rng`、`gen_range`、`Rng` 已不存在。
- `random_range(1..=6)` 挑範圍內的數（空範圍會 panic）、`random_bool(p)` 依機率、`random()` 依型別決定；`rand::random()`、`rand::random_range()` 是不建產生器的簡寫。
- `StdRng::seed_from_u64(種子)` 建立可重現的產生器，同種子同序列，只在同一版本的 `rand` 成立。
- `shuffle`（`SliceRandom`）原地洗牌，`choose`（`IndexedRandom`）回傳 `Option<&T>`。
- 金鑰、token 這類安全用途，本書不展開，先讀 `rand` 文件對安全性的說明。
