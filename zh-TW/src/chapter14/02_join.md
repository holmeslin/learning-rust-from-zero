# `join`

## 本集目標

用 `JoinHandle` 的 `join` 等執行緒做完並拿回它的結果，也知道子執行緒 panic 時，`join` 會回傳 `Err`，而不是讓整個程式一起倒。

## 正文

### `join`：等它做完，拿回結果

`thread::spawn` 的回傳值是 `JoinHandle<T>`，相當於這條執行緒的「取貨單」。呼叫 `handle.join()` 會卡在那裡，一直等到那條執行緒結束；閉包最後一個運算式的值，就是執行緒的結果，從 `join` 拿回來：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let tea = thread::spawn(|| {
        thread::sleep(Duration::from_millis(200));
        30
    });
    let coffee = thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        45
    });
    let tea_price = tea.join().expect("泡茶的店員出事了");
    let coffee_price = coffee.join().expect("煮咖啡的店員出事了");
    println!("紅茶 {tea_price} 元，咖啡 {coffee_price} 元");
    println!("共 {} 元", tea_price + coffee_price);
}
```

執行結果：

```text
紅茶 30 元，咖啡 45 元
共 75 元
```

- 兩位店員同時開工，總共只等了約 200 毫秒（較慢的那位），不是 300 毫秒。
- 咖啡其實比較早做好，但我們先 `join` 紅茶，所以先拿到紅茶的結果。結果出現的順序由我們 `join` 的順序決定，跟誰先做完無關，輸出因此是固定的。
- `join` 回傳 `Result`，先照第 9 章第 4 集的固定句型 `.expect(...)`，下一節說明什麼時候會是 `Err`。
- `join` 會拿走 `handle`（它的參數是 `self`，第 5 章第 8 集），所以一條執行緒只能 `join` 一次，第二次是 E0382。
- 開了執行緒卻不 `join`（上一集的做法），執行緒會繼續跑，但 `main` 結束時就被中止，你也拿不到它的結果。

### 子執行緒 panic：`join` 回傳 `Err`

一條執行緒 panic，不會讓整個程式倒下，只有那條執行緒結束。結果會從 `join` 以 `Err` 傳回來，由等它的人決定怎麼辦：

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let stock: Vec<u32> = Vec::new();
        stock[0]
    });
    match handle.join() {
        Ok(cups) => println!("庫存 {cups} 杯"),
        Err(_) => println!("盤點的店員出事了"),
    }
    println!("店長：繼續營業");
}
```

執行結果：

```text
盤點的店員出事了
店長：繼續營業
```

- 空的 `Vec` 取 `stock[0]`，在子執行緒裡 panic（第 7 章第 3 集）。`main` 用第 6 章的 `match` 接住 `Err`，程式照常跑完，結束碼是 0。
- 子執行緒的 panic 訊息照樣會印在終端機的 stderr，格式跟第 9 章第 1 集一樣：`thread '<unnamed>' (數字) panicked at src/main.rs:6:14:`，下一行是 `index out of bounds: the len is 0 but the index is 0`。括號裡的數字每次不同，執行緒沒取名字時顯示 `<unnamed>`。上面的「執行結果」只貼 stdout。
- `Err` 裡裝的是 panic 傳出來的內容，型別是 `Box<dyn Any + Send>`，這本書不展開，只用 `Ok`、`Err` 判斷。
- 如果寫成 `handle.join().expect("盤點的店員出事了")`，`Err` 會讓 `main` 也 panic，訊息是 `盤點的店員出事了: Any { .. }`。要不要讓整個程式跟著倒，是 `join` 的人決定。

### 跟 Go 比一比

Go 的 goroutine 沒有 `recover` 就 panic，整個程式一起結束，不會有人替它收屍：

```go
go func() {
	panic("店員出事了") // 沒有 recover，整個程式倒掉
}()
time.Sleep(100 * time.Millisecond)
fmt.Println("店長：繼續營業") // 印不出來
```

程式會印出 `panic: 店員出事了` 加上 goroutine 的追蹤資訊，結束碼是 2。Rust 的執行緒是各自獨立的：子執行緒 panic，只有它自己結束，`join` 把結果交給你。另外，Go 的 `sync.WaitGroup` 只能等「做完了」，要拿結果得另外靠 channel 或共用變數；Rust 的 `join` 等待與取回結果一次完成。

## 重點整理

- `thread::spawn` 回傳 `JoinHandle<T>`；`handle.join()` 等執行緒結束，回傳 `Result<T, _>`，`Ok` 裡是閉包的回傳值。
- `join` 拿走 handle，一條執行緒只能 `join` 一次；結果的順序由你 `join` 的順序決定。
- 子執行緒 panic 只結束那條執行緒，`join` 回傳 `Err`；`match` 處理它，程式可以繼續。
- 子執行緒的 panic 訊息仍會印在 stderr，帶 `thread '<unnamed>' (數字)`。
- 跟 Go 比：goroutine 沒 `recover` 的 panic 會讓整個程式倒下；Rust 的 `join` 同時負責等待與取回結果（或 `Err`）。
