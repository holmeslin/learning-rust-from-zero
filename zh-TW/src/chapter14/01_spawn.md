# `thread::spawn`

## 本集目標

用 `thread::spawn` 開出新的執行緒，知道它跟 `main` 同時執行、順序不固定，而且 `main` 一結束，還沒做完的執行緒就會被直接結束。

## 正文

### 開一條新的執行緒

飲料店有三位店員同時開工。Go 用 `go func() { ... }()` 開 goroutine，Rust 用 `thread::spawn`，參數是閉包（第 11 章），閉包裡的程式就在新的執行緒（thread）裡執行：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    for id in 1..=3 {
        thread::spawn(move || {
            println!("店員 {id} 開始泡茶");
            thread::sleep(Duration::from_millis(50));
            println!("店員 {id} 泡好了");
        });
    }
    thread::sleep(Duration::from_millis(300));
    println!("店長：打烊");
}
```

有兩個地方先照抄：

- `thread::sleep(Duration::from_millis(50))` 是「睡 50 毫秒」，這裡用來假裝正在泡茶，也讓 `main` 多等一會兒。`Duration` 是 `std::time` 的型別，第 17 章第 6 集才完整介紹。
- `move ||` 是第 11 章第 3 集的 `move` 閉包，`spawn` 為什麼一定要它，第 3 集說明。

執行它，會看到三個「開始泡茶」，接著是三個「泡好了」，最後是「打烊」。但同一支程式每次跑，「泡好了」的順序都可能不同：

某一次的輸出（順序每次可能不同）：

```text
店員 1 開始泡茶
店員 2 開始泡茶
店員 3 開始泡茶
店員 1 泡好了
店員 3 泡好了
店員 2 泡好了
店長：打烊
```

三位店員是各自跑在自己的執行緒上，作業系統想先讓哪一條跑，由它決定，我們管不到。所以只要有好幾條執行緒各自印東西，輸出的順序就是「某一次」。本章的範例會盡量讓輸出固定（等執行緒做完再印、先收集再排序），這一集是刻意讓你看見順序會變。

### 主執行緒結束，其他執行緒就跟著結束

`main` 跑的那條叫主執行緒。它一回傳，整個程式就結束，其他執行緒做到一半也會被當場中止，不會等它們：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(200));
        println!("店員：泡好了");
    });
    println!("店長：打烊");
}
```

執行結果：

```text
店長：打烊
```

店員要 200 毫秒才泡好，但 `main` 印完「打烊」就結束了，「泡好了」永遠不會出現。`thread::spawn` 開了執行緒就不管了，想等它做完，得靠下一集的 `join`；上面第一個範例用 `sleep` 硬等，只是為了讓你看見輸出。

### 跟 Go 比一比

`go` 開的 goroutine 由 Go 的 runtime 排程，起始的堆疊只有幾 KB，用多少長多少，所以隨手開個幾萬條也沒事：

```go
for id := 1; id <= 3; id++ {
	go func() {
		fmt.Printf("店員 %d 開始泡茶\n", id)
	}()
}
time.Sleep(300 * time.Millisecond)
```

`main` 結束就一起結束，這點跟 Rust 一樣。不一樣的是成本：`thread::spawn` 每次都向作業系統要一條真正的執行緒，作業系統要替它配置堆疊（Rust 預設 2 MiB）、負責排程，開一萬條就很吃力，開十萬條多半會先撞到作業系統的上限。需要大量輕量的並行工作，是第 15 章 async 的事；這一章的執行緒，適合開幾條到幾十條。

## 重點整理

- `thread::spawn(閉包)` 開一條新的執行緒，閉包裡的程式在新的執行緒上跑，跟 `main` 同時進行。
- 好幾條執行緒各自印東西，順序由作業系統決定，每次可能不同。
- `main` 結束，整個程式就結束，還沒做完的執行緒當場中止；要等它做完，用下一集的 `join`。
- `thread::sleep(Duration::from_millis(n))` 讓目前的執行緒睡 `n` 毫秒，先照抄，`Duration` 第 17 章第 6 集解釋。
- 跟 Go 比：goroutine 很輕，隨手開；`thread::spawn` 是真的作業系統執行緒，成本高，輕量任務留給第 15 章的 async。
