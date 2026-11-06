# `mpsc` channel

## 本集目標

用 `mpsc::channel` 在執行緒之間傳訊息：會 `send`、`recv`，會用 `for` 一路收到 channel 關閉，並知道 `send` 會把值的所有權交出去。

## 正文

### 一條 channel，兩個端點

不想共用變數，可以讓執行緒互相傳訊息。標準函式庫的 `std::sync::mpsc`（multiple producer, single consumer：多個生產者、單一消費者）提供 channel：`mpsc::channel()` 回傳一對 `(tx, rx)`，`tx` 是 `Sender`，只能送；`rx` 是 `Receiver`，只能收。訊息的型別由第一次 `send` 的值推斷。

廚房做好飲料，從 `tx` 送給櫃台：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let kitchen = thread::spawn(move || {
        for drink in ["紅茶", "咖啡", "奶茶"] {
            tx.send(String::from(drink)).expect("櫃台不見了");
        }
    });
    for drink in rx {
        println!("櫃台收到 {drink}");
    }
    kitchen.join().expect("廚房出事了");
}
```

執行結果：

```text
櫃台收到 紅茶
櫃台收到 咖啡
櫃台收到 奶茶
```

- `tx` 用 `move` 搬進廚房的執行緒（第 3 集的規則），`rx` 留在 `main`。
- `tx.send(值)` 把值送進 channel，回傳 `Result`；`.expect(...)` 照舊。
- `for drink in rx` 逐則收：channel 裡沒東西就等，有就進迴圈跑一圈；channel 關閉就結束迴圈。這是第 11 章第 8 集 `for` 的又一種對象，`Receiver` 也能走訪。
- 同一個 `tx` 送出的訊息，會依送的順序抵達，所以這支程式的輸出固定。

### channel 什麼時候關：所有 `Sender` 都被 drop

上面的 `for` 為什麼會結束？廚房的閉包跑完，搬進去的 `tx` 跟著 drop，`main` 沒有別的 `tx` 了，channel 就關了。把這件事在單一執行緒裡慢慢看：

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    println!("{:?}", rx.try_recv());
    tx.send(String::from("紅茶")).expect("櫃台不見了");
    println!("{:?}", rx.recv());
    drop(tx);
    println!("{:?}", rx.recv());
}
```

執行結果：

```text
Err(Empty)
Ok("紅茶")
Err(RecvError)
```

- `recv()` 會卡住等下一則；`try_recv()` 不等，沒東西就立刻回傳 `Err(Empty)`。
- 所有 `Sender` 都 drop 之後，已經在 channel 裡的訊息還是收得到，收完了 `recv()` 回傳 `Err(RecvError)`，意思是「channel 關了，不會再有訊息」。`for` 就是靠這個結束。
- 反過來，`Receiver` 被 drop 了，`send` 會回傳 `Err`，並把沒送出去的值還給你。送的人不會 panic，也不會卡住。

### `send` 之後，值就是對方的了

`send` 的參數是值，不是參照，所以會 move（第 3 章）。送出去之後再用，編譯器擋下來：

```rust,compile_fail=E0382
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let order = String::from("紅茶");
        tx.send(order).expect("櫃台不見了");
        println!("送出 {order}");
    });
    println!("收到 {}", rx.recv().expect("channel 關閉了"));
    handle.join().expect("店員出事了");
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `order`
 --> src/main.rs:9:23
  |
7 |         let order = String::from("紅茶");
  |             ----- move occurs because `order` has type `String`, which does not implement the `Copy` trait
8 |         tx.send(order).expect("櫃台不見了");
  |                 ----- value moved here
9 |         println!("送出 {order}");
  |                         ^^^^^ value borrowed here after move
```

收的人手上已經有那杯紅茶，送的人再碰它，就是兩個執行緒碰同一份資料。所有權規則把這件事擋在編譯時期，所以用 channel 傳東西，不用擔心送出去之後又被改。要兩邊都留，就 `.clone()` 一份再送。

### 跟 Go 比一比

Go 要由送的一方 `close` channel，收的一方用 `range` 收到關閉為止：

```go
orders := make(chan string)
go func() {
	for _, drink := range []string{"紅茶", "咖啡", "奶茶"} {
		orders <- drink
	}
	close(orders) // 忘了關，收的人的 range 就永遠等下去
}()
for drink := range orders {
	fmt.Println("櫃台收到", drink)
}
```

Rust 沒有 `close`：最後一個 `Sender` 被 drop，channel 就關了，忘不掉；對關掉的 channel 送值，Go 會 panic，Rust 的 `send` 回傳 `Err`。另外 Go 的 `chan string` 兩端是同一個值，誰拿到都能收能送；Rust 拆成 `Sender` 與 `Receiver` 兩個型別。最大的差別是所有權：Go 送指標過去，兩邊還是共用同一份資料，送出去之後繼續改，就是 data race；Rust 的 `send` 把值整個搬走。

## 重點整理

- `mpsc::channel()` 回傳 `(tx, rx)`；`tx.send(值)` 送、`rx.recv()` 收（會等）、`rx.try_recv()` 不等。`send`、`recv` 都回傳 `Result`。
- `for x in rx` 一直收到 channel 關閉為止；所有 `Sender` 都被 drop，channel 才關。
- channel 關閉後，已排隊的訊息還是收得到；收完才是 `Err(RecvError)`。`Receiver` 被 drop，`send` 回傳 `Err`。
- `send` 會 move 值，送出去之後再用是 E0382；同一個 `tx` 送出的訊息保持順序。
- 跟 Go 比：Go 由送的人 `close`，Rust 靠 drop；Go 送的指標兩邊共用，Rust 送出去就是對方的。
