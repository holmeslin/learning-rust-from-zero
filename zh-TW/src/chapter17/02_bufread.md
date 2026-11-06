# `BufRead` 與 `BufReader`

## 本集目標

用 `BufReader` 把任何 `Read` 變成能逐行讀的 `BufRead`，會用 `lines()` 與 `read_line`，逐行處理標準輸入；也認識寫的一方：`BufWriter`。

## 正文

### 為什麼要多包一層

上一集的 `read` 每次都直接跟來源要資料。來源如果是檔案或網路，每次都是一次系統呼叫，逐位元組地要太慢。`BufReader` 的做法是一次先多讀一大塊（預設 8 KiB）放在記憶體裡，之後的讀取先從這塊拿，用完再去要。

包了緩衝之後，就多出「讀一行」這類方法，它們屬於 `std::io::BufRead` 這個 trait。

### `lines()`：逐行走訪

```rust
use std::io::{BufRead, BufReader};

fn main() {
    let data = "紅茶,30\n咖啡,45\n\n奶茶,38";
    let reader = BufReader::new(data.as_bytes());
    for line in reader.lines() {
        let line = line.expect("讀取失敗");
        println!("[{line}]");
    }
}
```

執行結果：

```text
[紅茶,30]
[咖啡,45]
[]
[奶茶,38]
```

- `BufReader::new` 收任何 `Read`，這裡是記憶體裡的 `&[u8]`；用 `BufRead` 的方法前，要 `use std::io::BufRead;`。
- `lines()` 回傳迭代器，每個元素是 `io::Result<String>`，每讀一行都可能出錯，所以要 `.expect()` 或 `?`。
- 每一行的換行（`\n`，Windows 的 `\r\n` 也是）已經去掉；空行是空字串；最後一行沒有換行也讀得到。

迭代器的元素是 `Result`，就能用第 11 章第 11 集的辦法收成 `Result<Vec<_>, _>`：`reader.lines().collect::<Result<Vec<String>, _>>()`，一出錯就停。

### `read_line`：回收第 1 章的固定句型

`lines()` 底下用的是 `read_line`：把下一行**連同換行**附加到你給的 `String` 後面，回傳讀到的位元組數，**0 代表沒有資料了**：

```rust
use std::io::BufRead;

fn main() {
    let mut reader = "紅茶,30\n咖啡,45\n".as_bytes();
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line).expect("讀取失敗");
        if n == 0 {
            break;
        }
        println!("{n} 個位元組：{line:?}");
    }
}
```

執行結果：

```text
10 個位元組："紅茶,30\n"
10 個位元組："咖啡,45\n"
```

第 1 章第 9 集一直照抄的 `std::io::stdin().read_line(&mut line)` 就是它：「附加」所以每次要 `clear()`（固定句型每次都重新 `String::new()`，剛好避開這件事），換行還留著，所以才要 `trim()`。`Stdin` 自己已經帶緩衝，不必再包 `BufReader`。

### 逐行讀標準輸入

要處理很多行輸入，用 `stdin().lock().lines()`。`lock()` 先鎖住標準輸入（它被所有執行緒共用，第 14 章第 7 集的鎖），拿到實作了 `BufRead` 的 `StdinLock`：

```rust,stdin=30\n45\n38,noplayground
use std::io::BufRead;

fn main() {
    let mut total = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.expect("讀取失敗");
        let price: u32 = line.trim().parse().expect("請輸入整數");
        total += price;
        println!("加上 {price}，累計 {total}");
    }
    println!("共 {total} 元");
}
```

輸入 `30`、`45` 和 `38` 的執行結果：

```text
加上 30，累計 30
加上 45，累計 75
加上 38，累計 113
共 113 元
```

迴圈在輸入結束時自己停下（鍵盤輸入按 Ctrl-D）。`std::io::stdin().lines()` 是同一件事的簡寫，不需要 `use BufRead`。

### 寫的一方：`BufWriter`

`BufWriter` 把要寫的位元組先存在記憶體，攢夠了才一次寫出去，大量小量寫入時少很多次系統呼叫：

```rust
use std::io::{BufWriter, Write};

fn main() {
    let mut buf: Vec<u8> = Vec::new();
    {
        let mut out = BufWriter::new(&mut buf);
        writeln!(out, "紅茶").expect("寫入失敗");
        println!("寫入之後，buf 有 {} 個位元組", out.get_ref().len());
    }
    println!("out 離開作用域之後，buf 有 {} 個位元組", buf.len());
}
```

執行結果：

```text
寫入之後，buf 有 0 個位元組
out 離開作用域之後，buf 有 7 個位元組
```

寫進 `out` 的「紅茶」（加換行是 7 個位元組）還在 `BufWriter` 的緩衝裡，`buf` 是空的；`out` 離開作用域被 drop（第 3 章第 4 集），才把緩衝倒進 `buf`。drop 時如果出錯是沒人看得到的，所以重要的資料要自己呼叫 `flush()` 並檢查結果。

### 跟 Go 比一比

```go
scanner := bufio.NewScanner(os.Stdin)
for scanner.Scan() {
	line := scanner.Text()
	_ = line
}
// 一行超過 64 KiB，Scan() 就停了，要看 scanner.Err() 才知道
```

- Go 的 `Scanner` 預設一行最多 64 KiB，更長的行讓 `Scan()` 回傳 `false`、迴圈靜靜結束（實測，輸入一行 70000 個字元後面再接一行，迴圈跑 0 次，`Err()` 是 `bufio.Scanner: token too long`）。Rust 的 `lines()` 沒有長度限制，出錯一定回傳 `Err`，不會悄悄停下。
- Go 的 `bufio.Writer` 不 `Flush()` 資料就丟了；Rust 的 `BufWriter` drop 時會自動寫出，只是錯誤被吞掉，所以還是建議自己 `flush()`。

## 重點整理

- `BufReader::new(reader)` 替任何 `Read` 加上緩衝，得到實作了 `BufRead` 的讀取器；用 `lines()`、`read_line` 前要 `use std::io::BufRead;`。
- `lines()` 每個元素是 `io::Result<String>`，已去掉換行；`read_line` 把一行（含換行）附加到 `String`，回傳位元組數，0 代表結束，所以每次要 `clear()`。
- 第 1 章的 `stdin().read_line(&mut line)` 就是這個 `read_line`；逐行讀標準輸入用 `stdin().lock().lines()`，或簡寫的 `stdin().lines()`。
- `BufWriter` 先把寫入存在緩衝，drop 或 `flush()` 才真正寫出；drop 時的錯誤看不到，重要的資料自己 `flush()`。
- 跟 Go 比：`Scanner` 超過 64 KiB 的行會靜靜停止，`lines()` 沒有這個限制。
