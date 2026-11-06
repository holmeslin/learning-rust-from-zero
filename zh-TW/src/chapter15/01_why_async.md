# 為什麼需要 async

## 本集目標

看懂 async 想解決什麼問題：很多工作大部分時間在「等」，卻各佔一條執行緒。也先知道 Rust 的 async 跟 Go 的 goroutine 差在哪裡，後面十三集會逐一拆開。

## 正文

### 一杯飲料一條執行緒

飲料店收到三張訂單，每杯要等一段時間才好（這裡用 `thread::sleep` 假裝在等）。用上一章的工具，最直接的寫法是每杯開一條執行緒，好了之後用 channel 送回櫃台：

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    for (drink, wait) in [("紅茶", 300), ("咖啡", 100), ("奶茶", 200)] {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(wait));
            tx.send(drink).expect("櫃台不見了");
        });
    }
    drop(tx);
    for drink in rx {
        println!("{drink} 好了");
    }
}
```

執行結果：

```text
咖啡 好了
奶茶 好了
紅茶 好了
```

- 三條執行緒各睡 300、100、200 毫秒，先好的先送，所以順序是咖啡、奶茶、紅茶，不是下單的順序。
- 這支程式沒有錯。三杯一起等，總共只等最久的那杯。
- 問題在睡覺的時候：那條執行緒什麼事都不做，卻一直佔著作業系統配給它的資源（堆疊、排程名額）。

### 等得多的時候

三杯無所謂。換成網路服務就不同了：一台伺服器同時有一萬個連線，每個都在等資料庫、等對方的回應，真正在算東西的時間很短，絕大部分時間在等。照上面的寫法就是一萬條執行緒：每條都要配置堆疊（第 14 章第 1 集說過，Rust 預設 2 MiB），作業系統還得在它們之間來回切換。開得出來也很浪費，開到系統的上限就開不出來。

這類「大部分時間在等網路、等磁碟、等計時器」的工作，正是 async 要處理的對象。反過來，一直在算的工作（壓縮、加密、大量計算）沒有東西可等，用 async 沒有好處，要靠更多執行緒，第 12 集會再看到。

### async 的做法：等的時候讓出去

async 的想法是：把程式裡「要等」的地方標出來，走到那裡就把執行緒讓給別的工作，等的東西好了，再回來接著做。一件這樣的獨立工作叫任務（task），同一條執行緒可以輪流照顧大量的任務：

```text
一件事一條執行緒：
  thread 1  [A work]~~~~wait~~~~[A work]
  thread 2  [B work]~~~~wait~~~~[B work]
  thread 3  [C work]~~~~wait~~~~[C work]

一條執行緒輪流照顧：
  thread 1  [A][B][C]~~~~wait~~~~[A][B][C]
```

`~~~~wait~~~~` 是三件事都在等的時間：左邊三條執行緒各自閒著，右邊只有一條執行緒，而且它在這段時間可以去接別的任務。

### Rust 的 async 長什麼樣子

- `async fn` 與 `.await` 是語言本身的功能（第 2 集）；編譯器把每個 async 函式變成一個狀態機，每個 `.await` 是可以暫停、再繼續的地方。
- `Future` 是標準函式庫的 trait（第 3 集），代表「還沒做完、將來會有結果」的工作。
- 標準函式庫沒有附「負責執行任務的引擎」，也就是 runtime。要自己選一個函式庫，這本書用 tokio（第 4 集起）。

### 跟 Go 比一比

同樣的三杯飲料，Go 寫成這樣：

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	done := make(chan string)
	for _, order := range []struct {
		drink string
		wait  int
	}{{"紅茶", 300}, {"咖啡", 100}, {"奶茶", 200}} {
		go func() {
			time.Sleep(time.Duration(order.wait) * time.Millisecond)
			done <- order.drink
		}()
	}
	for range 3 {
		fmt.Println(<-done, "好了")
	}
}
```

goroutine 本來就是「很輕、可以開很多」的任務，行為很接近 async：等的時候 runtime 自動換別人跑。但寫法與底層有幾個地方不一樣，從 Go 轉過來要先有心理準備：

- Go 的函式沒有任何標記，`time.Sleep`、讀 channel 的時候 runtime 自動把 goroutine 停下來；Rust 要自己標 `async`，等待的地方也要明明白白寫 `.await`。
- Go 的 runtime 內建，程式一啟動就在；Rust 要自己選 runtime，並且啟動它（第 4 集）。
- Go 的 runtime 能搶佔（preemption）：一個一直算、不等任何東西的 goroutine，runtime 會中途把它換下來，別的 goroutine 照樣有機會跑。Rust 的任務只在 `.await` 的地方讓出；長時間運算如果沒有 `.await`，同一條執行緒上其他任務就全部被卡住（第 12 集）。
- `go f()` 立刻開跑；Rust 呼叫 async 函式只會得到一個 `Future`，不 `.await` 它就什麼都不會發生（第 3 集）。

## 重點整理

- async 處理「大部分時間在等」的工作：等的時候把執行緒讓給別的任務，一條執行緒可以輪流照顧大量的任務。
- 每件事一條執行緒也做得到，但每條執行緒都有堆疊與排程的成本，同時等上萬件事會很吃力。
- 一直在算的工作沒有東西可等，async 沒有好處。
- Rust 的 `async`/`.await` 與 `Future` 在語言與標準函式庫裡，runtime 不在，這本書用 tokio。
- 跟 Go 比：goroutine 的 runtime 內建、自動換手；Rust 要標 `async`、`.await`，要選 runtime，任務只在 `.await` 讓出，而且 `Future` 不會自己開跑。
