# 把資料 `move` 進執行緒

## 本集目標

知道 `thread::spawn` 的閉包為什麼一定要 `move`（E0373、`'static`），並會把資料交給執行緒、用 `.clone()` 多備一份。

## 正文

### 不加 `move`：E0373

店員要用外面的 `name`。照閉包的預設，閉包只會「借」變數：

```rust,compile_fail=E0373
use std::thread;

fn main() {
    let name = String::from("小美");
    let handle = thread::spawn(|| {
        println!("{name} 開始泡茶");
    });
    handle.join().expect("店員出事了");
}
```

編譯錯誤：

```text
error[E0373]: closure may outlive the current function, but it borrows `name`, which is owned by the current function
 --> src/main.rs:5:32
  |
5 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `name`
6 |         println!("{name} 開始泡茶");
  |                    ---- `name` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:5:18
  |
5 |       let handle = thread::spawn(|| {
  |  __________________^
6 | |         println!("{name} 開始泡茶");
7 | |     });
  | |______^
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword
  |
5 |     let handle = thread::spawn(move || {
  |                                ++++
```

這跟第 11 章第 3 集的 E0373 是同一種錯。`note` 那一句是關鍵：`function requires argument type to outlive 'static`。就算我們馬上 `join`，編譯器也不會放行：它只看 `spawn` 的簽名，不去推敲 `join` 會不會等。

`thread::spawn` 的簽名（簡化後）是：

```text
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

- `F: FnOnce() -> T` 是第 11 章第 4 集的 bound：閉包只會被呼叫一次。
- `'static` 是第 10 章第 7 集教過的生命週期，意思是整個程式都有效。放在 bound 上（`F: 'static`），意思是：這個型別裡面不能有會過期的借用。閉包借了區域變數 `name`，`main` 的 `}` 一到 `name` 就沒了，但新執行緒可能還在跑，所以編譯器不讓你這樣借。
- `T: 'static` 同理：閉包的回傳值也不能借著區域變數，否則 `join` 拿回來的東西會懸在半空。
- `Send` 先跳過，第 11 集再說明。

所以交給 `spawn` 的閉包，擷取的東西一律要「擁有」，不能「借」。第 11 章第 3 集的 `move` 就是這個用途。

### 加上 `move`：把資料搬進去

```rust
use std::thread;

fn main() {
    let mut handles = Vec::new();
    for i in 1..=3 {
        let order = format!("第 {i} 杯");
        handles.push(thread::spawn(move || format!("{order} 完成")));
    }
    for handle in handles {
        println!("{}", handle.join().expect("店員出事了"));
    }
}
```

執行結果：

```text
第 1 杯 完成
第 2 杯 完成
第 3 杯 完成
```

- 每一圈都建一個新的 `order`，用 `move` 交給那一圈開的執行緒，各拿各的，彼此不相干。
- `handles` 收集每一個 `JoinHandle`，第二個 `for` 依序 `join`、印出結果，所以輸出順序固定。
- 迴圈變數 `i` 是整數，`move` 搬進去的是複本（第 11 章第 3 集）。

`move` 之後，原本的變數就不能再用了，規則跟第 3 章一樣（E0382）。一份資料如果兩邊都要用，先 `.clone()` 一份交給執行緒：

```rust
use std::thread;

fn main() {
    let orders = vec![30, 45, 38];
    let for_barista = orders.clone();
    let handle = thread::spawn(move || {
        let mut total = 0;
        for price in for_barista {
            total += price;
        }
        total
    });
    println!("店員算出 {} 元", handle.join().expect("店員出事了"));
    println!("我這邊還留著 {orders:?}");
}
```

執行結果：

```text
店員算出 113 元
我這邊還留著 [30, 45, 38]
```

要共用「同一份」而不是複製一份，就是第 8 集的 `Arc`。

### 跟 Go 比一比

Go 的閉包擷取的是變數本身，`go func() { ... name ... }()` 想用就用，編譯器會自動把 `name` 搬到 heap，讓它活得夠久。方便，代價是 `main` 和 goroutine 手上是同一個變數，兩邊同時改就是 data race，要等 `-race` 在執行時才抓得到。Rust 逼你在開執行緒的當下就決定：資料交給它（`move`），或各拿一份（`.clone()`）。

## 重點整理

- 閉包預設借用外面的變數；`thread::spawn` 要求閉包 `'static`（不含會過期的借用），所以借區域變數是 E0373。
- `F: 'static` 的意思是「型別裡沒有會過期的借用」，不是「活到程式結束」。
- 用 `move` 把變數的所有權搬進執行緒；`Copy` 型別搬進去的是複本，其他型別搬走後原變數失效（E0382）。
- 兩邊都要用同一份資料，先 `.clone()`；要共用同一份，用第 8 集的 `Arc`。
- 跟 Go 比：Go 的閉包直接共用變數，錯誤要等執行時的 `-race`；Rust 在開執行緒時就要決定搬走或複製。
