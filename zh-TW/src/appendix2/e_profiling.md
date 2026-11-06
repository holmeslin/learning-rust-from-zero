# 效能分析工具簡介

## 本集目標

學會用標準函式庫量出程式跑多快：知道為什麼要用 `--release`、`Instant` 怎麼計時、`black_box` 在做什麼，以及量的時候常掉的陷阱。其他分析工具只認識名字。

## 正文

### 先量，再改

覺得慢的地方，常常不是真正慢的地方。順序是：先量出哪裡慢，改了，再量一次確認真的變快。量一小段程式有多快，叫基準測試（benchmark）；找出整支程式的時間花在哪些函式上，叫效能剖析（profiling）。這一集只用標準函式庫做前者；後者的工具（`perf`、`cargo flamegraph`、`criterion` 之類）這本書不展開，也不要求安裝。

### 要量，先用 `--release`

`cargo run` 是 debug 編譯：不最佳化、檢查整數溢位（第 1 章第 16 集），還保留除錯資訊，目標是好除錯，不是跑得快。要量速度，一律用最佳化編譯：

```bash
cargo run --release
cargo build --release
```

`cargo build --release` 的執行檔在 `target/release/`。`Cargo.toml` 的 `[profile.release]` 還能調整最佳化的設定（`opt-level`、`lto` 等），這本書不展開。

### 用 `Instant` 計時

`Instant` 與 `elapsed()` 在第 17 章第 6 集教過。下面量兩種寫法的速度，順便避開兩個常見的陷阱：

```rust
use std::hint::black_box;
use std::time::{Duration, Instant};

fn best_of(runs: u32, mut work: impl FnMut() -> u64) -> Duration {
    let mut best = Duration::MAX;
    for _ in 0..runs {
        let start = Instant::now();
        black_box(work());
        best = best.min(start.elapsed());
    }
    best
}

fn by_loop(n: u64) -> u64 {
    let mut total = 0;
    for i in 0..n {
        total += i * i % 7;
    }
    total
}

fn by_iter(n: u64) -> u64 {
    (0..n).map(|i| i * i % 7).sum()
}

fn main() {
    let n = black_box(5_000_000);
    println!("{} {}", by_loop(n), by_iter(n));
    println!("迴圈 {:?}", best_of(5, || by_loop(n)));
    println!("迭代器 {:?}", best_of(5, || by_iter(n)));
}
```

- 第一行 `9999999 9999999` 每次都一樣，確認兩種寫法算出同樣的結果；後面兩行的時間每次都不同，所以這一集不貼「執行結果」。
- **重複量，取最小值。** 一次計時會被作業系統、別的程式、CPU 降頻攪和，這些雜訊只會讓時間變長、不會變短，所以 `best_of` 跑 5 次，留下最小的（`Duration::MAX` 是最大的 `Duration`，`min` 取較小的）。
- **`black_box`。** 它不改變值，原樣傳回來，但編譯器看不穿它。開了最佳化，編譯器會把算得出來的東西在編譯時期先算好，或發現結果沒人用就整段刪掉，量到的就成了 0。`black_box(n)` 讓它不知道 `n` 是多少，`black_box(work())` 讓它不能丟掉結果。這只是盡力的提示，不是保證。

在作者的電腦上，各跑一次的結果如下（你的數字會不同，只看比例）：

```text
$ cargo run
9999999 9999999
迴圈 28.646458ms
迭代器 45.529333ms
$ cargo run --release
9999999 9999999
迴圈 2.146375ms
迭代器 2.1385ms
```

debug 比 release 慢十倍以上；而且 debug 下迭代器比迴圈慢了不少，release 下兩者幾乎一樣。這就是附錄二 b 說的：零成本抽象要開了最佳化才成立。量速度卻用 `cargo run`，得到的是不具參考價值的數字。

### 還有什麼工具

標準函式庫只提供 `Instant` 與 `black_box`；自帶的 `#[bench]` 到 1.99 還在 nightly，stable 用不了。想要統計與圖表，要靠外部的 crate 或工具，這本書不展開。量的時候還要記得：輸入要有代表性（別拿空陣列量），數字只能跟同一台電腦、同一種編譯方式的結果比。

### 跟 Go 比一比

Go 沒有 debug 與 release 之分：`go run`、`go build` 本來就會最佳化（要除錯才用 `-gcflags="all=-N -l"` 關掉）。Go 的基準測試與剖析也是內建的：

```go
func BenchmarkCount(b *testing.B) {
	for b.Loop() {
		countMultiples(1_000_000)
	}
}
```

寫在 `_test.go` 檔裡，用 `go test -bench .` 執行；`pprof` 也在標準函式庫。Go 的直覺會讓人用 `cargo run` 隨手量一下，就以為 Rust 比 Go 慢：量之前先加 `--release`。

## 重點整理

- 先量再改。`cargo run` 是 debug 編譯，不最佳化；要量速度一律用 `--release`，差距可以到十倍以上。
- 用 `Instant::now()` 與 `elapsed()` 計時；重複量、取最小值，因為雜訊只會讓時間變長。
- `std::hint::black_box` 原樣傳回值，但讓編譯器看不穿：用在輸入與結果上，避免被編譯時期算掉或整段刪掉。
- 計時的數字每次不同、跟機器有關，只比相對的大小；零成本抽象的比較，也要在 `--release` 下量。
- 其他剖析與基準測試工具這本書不展開；Go 內建 `testing.B` 與 `pprof`，Rust 的標準函式庫只有 `Instant` 與 `black_box`。
