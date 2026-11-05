# `RwLock`

## 本集目標

用 `RwLock<T>` 讓多個讀者同時讀、寫的時候只留一個人；會用 `read()` 與 `write()`，並知道什麼情況值得換掉 `Mutex`。

## 正文

### 很多人讀，一個人寫

`Mutex` 不分讀寫，一次只放一個人進去，就算大家都只是看一眼，也得排隊。價目表這類資料，天天有人看、很少有人改，可以用 `RwLock<T>`（read-write lock）：

- `read()` 拿讀鎖，得到的 guard 當 `&T` 用。同時可以有很多個讀者。
- `write()` 拿寫鎖，得到的 guard 當 `&mut T` 用。寫的時候，不能有別的讀者或寫者。

這就是第 4 章第 4 集「多個 `&` 或一個 `&mut`」的規則，只是變成執行時期在鎖上輪流遵守。guard 離開作用域鎖就放開，跟 `Mutex` 一樣。

```rust
use std::sync::RwLock;

fn main() {
    let menu = RwLock::new(vec![30, 45]);
    let first = menu.read().expect("價目表壞了");
    let second = menu.read().expect("價目表壞了");
    println!("兩位讀者同時看：{} 項、{} 項", first.len(), second.len());
    println!("有人在讀，寫得進去嗎？{}", menu.try_write().is_ok());
    drop(first);
    drop(second);
    println!("讀者都離開了，寫得進去嗎？{}", menu.try_write().is_ok());
}
```

執行結果：

```text
兩位讀者同時看：2 項、2 項
有人在讀，寫得進去嗎？false
讀者都離開了，寫得進去嗎？true
```

- 兩個 `read()` 都拿得到，兩個 guard 同時活著。
- `try_write()` 不等，拿不到就回傳 `Err`；有讀者的時候是 `Err`，所以 `is_ok()` 是 `false`。把兩個讀者 `drop` 掉，才寫得進去。
- `read()`、`write()` 跟 `lock()` 一樣回傳 `Result`（寫鎖的執行緒 panic，也會讓鎖中毒），先照 `.expect(...)`。

### 多條執行緒一起讀

三條讀者執行緒同時看價目表，全部結束之後，主執行緒再寫入：

```rust
use std::sync::RwLock;
use std::thread;

fn main() {
    let board = RwLock::new(vec![30u32, 45, 38]);
    thread::scope(|s| {
        let mut readers = Vec::new();
        for _ in 0..3 {
            readers.push(s.spawn(|| board.read().expect("價目表壞了").iter().sum::<u32>()));
        }
        for reader in readers {
            println!("讀到合計 {}", reader.join().expect("讀者出事了"));
        }
    });
    board.write().expect("價目表壞了").push(55);
    println!("改完共 {} 項", board.read().expect("價目表壞了").len());
}
```

執行結果：

```text
讀到合計 113
讀到合計 113
讀到合計 113
改完共 4 項
```

- 跟 `Mutex` 一樣，`RwLock` 只需要 `&`，所以能搭配 `thread::scope` 直接借；要交給 `thread::spawn`，就包成 `Arc<RwLock<T>>`。
- 讀者拿的是共用借用，三條執行緒不必互相等待。換成 `Mutex` 的話，三條會排隊。
- `board.write()` 回傳的 guard 是暫時的，所以那一行結束鎖就放開，下一行才讀得到。

### 什麼時候值得用

- 讀遠多於寫，而且讀的時候要花一點時間（掃一整份資料）：用 `RwLock`，讀者才不會互相擋住。
- 讀寫差不多，或每次只拿鎖一瞬間：用 `Mutex`。`RwLock` 要多記帳（現在有幾位讀者），單次操作通常比 `Mutex` 慢，省不了什麼。
- 有人在等寫鎖的時候，新來的讀者要不要先讓他，各作業系統的做法不同，不要靠這個行為。
- 同一條執行緒拿著讀鎖，又去 `write()`，會把自己卡死（或 panic），這是第 14 集的死結。

## 重點整理

- `RwLock<T>`：`read()` 拿讀鎖（當 `&T`，可同時有多個）、`write()` 拿寫鎖（當 `&mut T`，同時只有一個，而且不能有讀者）。
- `try_read()`、`try_write()` 不等待，拿不到回傳 `Err`。三個方法都回傳 `Result`（中毒時是 `Err`）。
- 只需要 `&` 就能用，搭配 `thread::scope` 直接借，或用 `Arc<RwLock<T>>` 交給 `thread::spawn`。
- 讀遠多於寫才值得；否則 `Mutex` 更簡單、也不慢。
- 同一條執行緒持有讀鎖時去拿寫鎖，會死結。
