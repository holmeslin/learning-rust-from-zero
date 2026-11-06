# worker pool

## 本集目標

把這一章學的工具組起來，做出固定人數的 worker pool：幾位工人從同一條佇列領工作、做完把結果送回來，最後乾淨地收工。

## 正文

### 設計：兩條通道，三位工人

飲料店有 8 張訂單，只請 3 位店員（worker），誰有空誰就領下一張。需要兩條通道：

```text
main --job_tx--> [ 工作佇列 ] ---> 工人 1、工人 2、工人 3 --done_tx--> [ 結果佇列 ] ---> main
```

- 工作佇列：`main` 送訂單，三位工人領。但 `Receiver` 只有一個擁有者（single consumer），三位工人都要從它領，所以包成 `Arc<Mutex<Receiver<u32>>>`：`Arc` 讓三個人共有，`Mutex` 讓他們輪流領。
- 結果佇列：每位工人拿一個 `done_tx` 的複本，做完把（訂單、價格）送回 `main`。

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn make_drink(order: u32) -> u32 {
    order * 10 + 5
}

fn main() {
    let (job_tx, job_rx) = mpsc::channel::<u32>();
    let (done_tx, done_rx) = mpsc::channel();
    let job_rx = Arc::new(Mutex::new(job_rx));

    let mut workers = Vec::new();
    for _ in 0..3 {
        let job_rx = Arc::clone(&job_rx);
        let done_tx = done_tx.clone();
        workers.push(thread::spawn(move || {
            loop {
                let job = job_rx.lock().expect("工作佇列壞了").recv();
                match job {
                    Ok(order) => {
                        let price = make_drink(order);
                        done_tx.send((order, price)).expect("櫃台不見了");
                    }
                    Err(_) => break,
                }
            }
        }));
    }
    drop(done_tx);

    for order in 1..=8 {
        job_tx.send(order).expect("工人都下班了");
    }
    drop(job_tx);

    let mut results: Vec<(u32, u32)> = done_rx.iter().collect();
    results.sort();
    println!("共做完 {} 杯", results.len());
    println!("{results:?}");
    for worker in workers {
        worker.join().expect("工人出事了");
    }
}
```

執行結果：

```text
共做完 8 杯
[(1, 15), (2, 25), (3, 35), (4, 45), (5, 55), (6, 65), (7, 75), (8, 85)]
```

- 每位工人跑一個 `loop`：領一張、做一張，直到領不到。`let job = job_rx.lock()....recv();` 這一行，guard 在行尾就 drop，鎖只在「領工作」那一瞬間被拿著，做飲料（`make_drink`）的時候鎖是放開的。
- 收工靠通道關閉（第 5 集）：`main` 送完 8 張就 `drop(job_tx)`，工作佇列的 `Sender` 沒了。工人把剩下的訂單領完，再 `recv()` 就是 `Err`，於是 `break` 下班。
- `done_rx.iter()` 是 `Receiver` 的走訪方法，意思跟 `for x in done_rx` 一樣，一直收到通道關閉，接上第 11 章的 `collect` 收進 `Vec`。
- `drop(done_tx)` 是第 6 集的老問題：`main` 手上那個原始的 `done_tx` 不丟，結果佇列永遠不會關，`done_rx.iter()` 就收不完。
- 每張訂單由哪位工人做、誰先做完，每次都不同，所以先收集、`sort`，再印，輸出才固定。最後 `join` 每位工人，確認沒人出事。

### 領工作時，不要握著鎖

工人領工作的寫法有一個很容易踩的坑。看起來更簡潔的寫法是這樣：

```text
while let Ok(order) = job_rx.lock().expect("工作佇列壞了").recv() {
    ...做飲料...
}
```

這裡的 `job_rx.lock()` 拿到的 guard 是 `while let` 條件裡的暫時值，會一直活到迴圈本體結束。結果是：一位工人領到工作之後，一路握著鎖做飲料，另外兩位工人連「領」都領不到，三位工人變成輪流做事。實測每張訂單 100 毫秒、6 張訂單，上面第一種寫法約 200 毫秒完成，`while let` 這種約 600 毫秒，跟沒有 pool 一樣慢，程式還完全沒有錯誤訊息。

原則跟第 9 集一樣：鎖只拿在真的要碰共用資料的那一行，把 `recv()` 的結果存進 `let` 之後，鎖就放開了。

### 還能怎麼延伸

- 工作不只是數字：把佇列的訊息型別換成 `Box<dyn FnOnce() + Send>`，工人收到之後直接呼叫，就是能執行任何閉包的 pool。`Send` 是第 11 集的限制，因為閉包要搬到工人的執行緒。
- 工作佇列有上限：把 `mpsc::channel` 換成第 16 集的 `mpsc::sync_channel(n)`，`main` 送太快就會被擋住，不會堆出無限長的佇列。
- 需要現成的、更有效率的工具：第 15 章的 tokio（第 4 集起）與 `spawn_blocking`（第 12 集）就是這件事的延伸。

### 跟 Go 比一比

Go 的 channel 可以讓好幾個 goroutine 同時 `range` 同一條，直接分工，不用包 `Mutex`，也不用 `Arc`：

```go
for w := 0; w < 3; w++ {
	wg.Add(1)
	go func() {
		defer wg.Done()
		for order := range jobs {
			results <- [2]int{order, order*10 + 5}
		}
	}()
}
```

Rust 的 `mpsc` 是單一消費者（multiple producer, single consumer），一條通道只有一個 `Receiver`，所以多位工人共用要自己包 `Arc<Mutex<Receiver<T>>>`，多寫兩層，換來的是型別系統替你保證：每張訂單只會被一位工人領走。另一個差別在收工：Go 要自己 `close(jobs)` 並用 `WaitGroup` 等工人；Rust 是 `drop(job_tx)`，通道自己關。

## 重點整理

- worker pool：一條工作佇列加幾位工人加一條結果佇列；工人 `loop` 領工作，直到佇列關閉。
- `Receiver` 只有一個擁有者；多位工人共用同一條佇列，要包成 `Arc<Mutex<Receiver<T>>>`。
- `let job = rx.lock().expect(...).recv();` 鎖在行尾放開；`while let Ok(x) = rx.lock().expect(...).recv() { ... }` 會握著鎖做事，工人變成輪流做，效能等於單執行緒。
- 收工：送完工作就 `drop` 工作佇列的 `Sender`，工人領完剩下的工作後 `recv()` 回傳 `Err` 就 `break`；結果佇列的原始 `done_tx` 也要 `drop`。
- 完成順序每次不同，要先收集、排序再印；跟 Go 比：Go 的 channel 能多個消費者直接共用，Rust 的 `mpsc` 要自己包 `Arc<Mutex<_>>`。
