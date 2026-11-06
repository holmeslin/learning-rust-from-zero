# 多個生產者

## 本集目標

用 `tx.clone()` 讓好幾條執行緒往同一條 channel 送訊息，並記得把 `main` 手上那個原始的 `tx` drop 掉，不然 `for x in rx` 永遠不會結束。

## 正文

### 每位生產者拿一個 `tx.clone()`

`mpsc` 的 `m` 是 multiple producer：`Sender` 可以 `clone`，每個複本都通到同一條 channel。三位店員各做兩杯，各自送出（店員編號、第幾杯）：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for id in 1..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for cup in 1..=2 {
                tx.send((id, cup)).expect("櫃台不見了");
            }
        });
    }
    drop(tx);
    let mut done: Vec<(u32, u32)> = Vec::new();
    for order in rx {
        done.push(order);
    }
    done.sort();
    println!("共 {} 杯", done.len());
    println!("{done:?}");
}
```

執行結果：

```text
共 6 杯
[(1, 1), (1, 2), (2, 1), (2, 2), (3, 1), (3, 2)]
```

- 每一圈用 `let tx = tx.clone();` 把外面的 `tx` 遮蔽成複本，再 `move` 進那一圈的執行緒。這是開多條執行緒時很常見的寫法。
- 這次沒有 `join`：每條執行緒做完，它的 `tx` 複本就 drop；等到全部的 `Sender` 都不在了，channel 關閉，`for order in rx` 自然結束。channel 關閉本身就是「大家都做完了」的訊號。
- 同一位店員送的訊息保持順序（上一集），但三位店員的訊息怎麼交錯，每次都可能不同。所以先收進 `Vec`，`sort` 之後再印，輸出才固定。

### 忘了 `drop(tx)`：永遠等下去

注意上面那行 `drop(tx)`。`main` 一開始建立的 `tx` 也是一個 `Sender`，三個複本都用完了，它還活著，channel 就不會關。拿掉那一行：

```rust,no_run
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    for id in 1..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for cup in 1..=2 {
                tx.send((id, cup)).expect("櫃台不見了");
            }
        });
    }
    let mut count = 0;
    for _order in rx {
        count += 1;
        println!("收到第 {count} 杯");
    }
    println!("全部收完了");
}
```

這支程式只編譯、不執行（`no_run`），因為它不會結束：印完「收到第 6 杯」就不動了，「全部收完了」永遠出不來。沒有錯誤訊息，也不會 panic，`main` 只是靜靜等著下一則，因為它自己手上的 `tx` 還能送。

遇到「`for x in rx` 收完了卻不結束」，先找有沒有哪個 `Sender` 還活著。修法有兩種：

- 開完執行緒之後，立刻 `drop(tx)`（上面的寫法）。
- 把原始的 `tx` 本身 `move` 進最後一條執行緒，不再另外留著。

### 跟 Go 比一比

Go 的 channel 由大家共用，誰都能送，也就沒有人知道「最後一個送的人是誰」。所以 `close` 要有人負責，常見的寫法是 `WaitGroup` 加一個專門等待、再關閉的 goroutine：

```go
var wg sync.WaitGroup
for id := 1; id <= 3; id++ {
	wg.Add(1)
	go func() {
		defer wg.Done()
		for cup := 1; cup <= 2; cup++ {
			orders <- [2]int{id, cup}
		}
	}()
}
go func() {
	wg.Wait()
	close(orders) // 誰來關、什麼時候關，要自己安排
}()
```

提早關掉，還在送的 goroutine 會 panic；忘了關，收的人永遠等下去。Rust 把「還有幾個送的人」記在型別裡：每個 `Sender` 複本代表一位送的人，最後一個 drop channel 就關，不用自己數。要小心的反而是 `main` 手上多留的那一個。

## 重點整理

- `tx.clone()` 得到另一個通向同一條 channel 的 `Sender`；每條生產者執行緒各拿一個，用 `move` 搬進去。
- 全部的 `Sender`（含 `main` 手上的原始 `tx`）都被 drop，channel 才關，`for x in rx` 才會結束。
- 忘了 `drop(tx)`，`for x in rx` 會永遠等下去，沒有任何錯誤訊息。
- 同一個 `Sender` 送的訊息保持順序，不同 `Sender` 之間的交錯順序每次可能不同；要固定輸出就先收集、排序再印。
- 跟 Go 比：Go 要自己決定誰來 `close`；Rust 靠最後一個 `Sender` drop 自動關閉。
