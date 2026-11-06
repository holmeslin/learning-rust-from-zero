# data race 為什麼編譯不過

## 本集目標

知道 data race 是什麼，看懂同樣一個計數器，在 Go 要靠 `-race` 在執行時期才抓得到、在 Rust 則是編譯不過，並認得 Rust 擋下它的三道關卡。

## 正文

### data race 是什麼

同時符合三件事，就是 data race：

- 兩條以上的執行緒存取同一塊記憶體；
- 其中至少一條在寫；
- 沒有任何同步（鎖、atomic、channel）。

結果不可預期：數字算少了、資料壞掉、甚至 crash，而且常常「十次有九次看起來是對的」。注意它跟 race condition 不一樣：data race 是記憶體層面的問題；race condition 是「結果取決於誰先誰後」的邏輯問題，就算全部加了鎖，也可能發生（第 9 集的「兩次 `lock()`」）。

### 把 Go 的計數器照翻成 Rust

四位店員各把共用的 `count` 加 1。Go 寫起來毫不費力（下面的 Go 程式），照翻成 Rust：

```rust,compile_fail=E0499
use std::thread;

fn main() {
    let mut count = 0;
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| count += 1);
        }
    });
    println!("count = {count}");
}
```

編譯錯誤：

```text
error[E0499]: cannot borrow `count` as mutable more than once at a time
 --> src/main.rs:7:21
  |
5 |     thread::scope(|s| {
  |                    - has type `&'1 Scope<'1, '_>`
6 |         for _ in 0..4 {
7 |             s.spawn(|| count += 1);
  |             --------^^------------
  |             |       |  |
  |             |       |  borrows occur due to use of `count` in closure
  |             |       `count` was mutably borrowed here in the previous iteration of the loop
  |             argument requires that `count` is borrowed for `'1`
```

這是第 4 章第 4 集的 E0499：迴圈第二圈又要一個 `&mut count`，上一圈那條執行緒還借著。只貼前半段，後面的 `note` 指向標準函式庫內部的位置。

仔細看，data race 的三個條件，剛好就是「同時有兩個存取者，其中一個在寫」，而借用規則「多個 `&` 或一個 `&mut`」禁止的正是這件事。Rust 沒有為了 data race 另外發明一套機制，它是從所有權與借用順便得到的。完整的防線有三道：

- 借用規則（第 4 章）：同時兩個 `&mut`、或 `&` 加 `&mut`，是 E0499、E0502。
- `move` 與 `'static`（第 3 集）：`thread::spawn` 不能借區域變數，是 E0373。
- `Send` 與 `Sync`（第 11 集）：想靠 `Rc`、`Cell`、`RefCell` 偷渡共用的可變狀態，是 E0277。

要共用又要改，只剩下型別本身就保證執行緒安全的工具：`Mutex`、`RwLock`、atomic、channel。

### 編譯成功，不代表答案是對的

如果照 E0373 的 `help`，加上 `move`，程式就編譯成功了：

```rust
use std::thread;

fn main() {
    let mut count = 0;
    let handle = thread::spawn(move || {
        count += 1;
        count
    });
    println!("店員那邊 {}", handle.join().expect("店員出事了"));
    println!("count = {count}");
}
```

執行結果：

```text
店員那邊 1
count = 0
```

`count` 是整數，`move` 搬進去的是複本（第 3 集），執行緒改的是自己的那一份，沒有任何共用，當然沒有 data race；但 `main` 的 `count` 還是 0。Rust 保證沒有 data race，不保證你的邏輯正確。

真的要共用同一個 `count`，就把它放進 `Mutex`（第 7 集，四條執行緒各 `lock()` 加 1，最後就是 4），或改用 `AtomicU32`（第 12 集）。

### 跟 Go 比一比

這是 Go 版，存成 `main.go`：

```go
package main

import (
	"fmt"
	"sync"
)

func main() {
	count := 0
	var wg sync.WaitGroup
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			count++
		}()
	}
	wg.Wait()
	fmt.Println("count =", count)
}
```

`go run main.go` 編譯得過，也常常印出 `count = 4`，看起來沒事。加上 `-race`（Go 內建的 data race 偵測器），才會在執行時期抓出來。下面是節錄的輸出，位址與 goroutine 編號每次不同：

`go run -race main.go` 的輸出（節錄）：

```text
==================
WARNING: DATA RACE
Read at 0x00c000012178 by goroutine 9:
  main.main.func1()
      ./main.go:15 +0x68

Previous write at 0x00c000012178 by goroutine 6:
  main.main.func1()
      ./main.go:15 +0x78
...
count = 4
Found 2 data race(s)
exit status 66
```

兩種做法的差別：

- Rust 在編譯時期擋下，程式根本跑不起來；Go 要等程式執行，而且要記得加 `-race`。
- `-race` 只抓得到「這次執行真的跑到、而且真的撞上」的 race；沒被測試跑到的路徑，抓不到。偵測本身也很貴，官方文件估計記憶體約多 5～10 倍、執行時間約多 2～20 倍，所以通常只在測試與 CI 開。
- Rust 的檢查屬於型別系統，每一次編譯都做，沒有開關，也不會漏看。代價是你得學會它的規則，並接受很多「在 Go 看起來沒事」的寫法會被拒絕。

## 重點整理

- data race：兩條以上執行緒存取同一塊記憶體、至少一條在寫、沒有同步。它不同於 race condition（邏輯上的先後問題），後者編譯器擋不住。
- Rust 靠三道關卡在編譯時期擋下 data race：借用規則（E0499、E0502）、`move` 與 `'static`（E0373）、`Send` 與 `Sync`（E0277）。
- 想共用又要改，只有 `Mutex`、`RwLock`、atomic、channel 這些型別本身就保證執行緒安全的工具。
- 加 `move` 把整數搬進執行緒，是複本，編譯成功卻沒有共用，答案是錯的：沒有 data race，不等於程式正確。
- 跟 Go 比：Go 的 `-race` 在執行時期偵測，只抓得到跑到的路徑，而且很貴；Rust 在每次編譯的時候檢查。
