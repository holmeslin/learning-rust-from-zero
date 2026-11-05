# `thread::scope`

## 本集目標

用 `thread::scope` 開「範圍執行緒」：直接借用外面的變數，不必 `move`、不必 `'static`，範圍一結束所有執行緒就都做完了；並知道借用規則在執行緒之間照樣有效。

## 正文

### 借得動的原因：範圍結束前一定做完

上一集的 `thread::spawn` 不能借區域變數，是因為編譯器不知道新執行緒什麼時候結束。`thread::scope` 補上這個保證：在 `scope` 裡開的執行緒，一定在 `scope` 回傳之前全部結束。借用的資料活得比執行緒久，借用就安全，不必 `'static`，也就不必 `move`：

```rust
use std::thread;

fn main() {
    let sales = [30, 45, 38, 55, 42, 60];
    let first = &sales[..3];
    let second = &sales[3..];
    let (a, b) = thread::scope(|s| {
        let a = s.spawn(|| first.iter().sum::<u32>());
        let b = s.spawn(|| second.iter().sum::<u32>());
        (
            a.join().expect("前半的店員出事了"),
            b.join().expect("後半的店員出事了"),
        )
    });
    println!("{a} + {b} = {}", a + b);
}
```

執行結果：

```text
113 + 157 = 270
```

- `thread::scope(|s| { ... })` 的參數是閉包，`s` 代表這個範圍，用 `s.spawn(閉包)` 在範圍裡開執行緒。
- 兩條執行緒各自借 `sales` 的一半（`&[u32]`，第 4 章第 11 集），沒有 `move`，也沒有 `.clone()`。
- `s.spawn` 也回傳 handle，`join` 的用法跟第 2 集一樣；`scope` 本身會回傳閉包最後的值，所以 `(a, b)` 直接接得到。
- 不 `join` 也可以：`scope` 結束時，沒 `join` 的執行緒會被自動等完。如果其中有執行緒 panic，`scope` 也會 panic。

### 讓執行緒把結果寫回外面

範圍執行緒也能借 `&mut`，把結果直接寫進外面的變數，每條執行緒各寫自己的那一個：

```rust
use std::thread;

fn main() {
    let morning = [30, 45, 38];
    let evening = [55, 42, 60];
    let mut morning_total = 0;
    let mut evening_total = 0;
    thread::scope(|s| {
        s.spawn(|| morning_total = morning.iter().sum::<u32>());
        s.spawn(|| evening_total = evening.iter().sum::<u32>());
    });
    println!("早班 {morning_total} 元，晚班 {evening_total} 元");
    println!("共 {} 元", morning_total + evening_total);
}
```

執行結果：

```text
早班 113 元，晚班 157 元
共 270 元
```

- `morning_total` 與 `evening_total` 是兩個不同的變數，各被一條執行緒用 `&mut` 借走，不衝突。
- `scope` 一結束，借用就還回來，主執行緒又能讀它們了，不需要任何鎖。

### 借用規則在執行緒之間照樣有效

第 4 章第 4 集的規則是「多個 `&` 或一個 `&mut`」。兩條執行緒，一條要改 `menu`、另一條要讀，就是同時有 `&mut` 和 `&`：

```rust,compile_fail=E0502
use std::thread;

fn main() {
    let mut menu = vec![String::from("紅茶")];
    thread::scope(|s| {
        s.spawn(|| menu.push(String::from("咖啡")));
        s.spawn(|| println!("{}", menu.len()));
    });
    println!("{menu:?}");
}
```

編譯錯誤：

```text
error[E0502]: cannot borrow `menu` as immutable because it is also borrowed as mutable
 --> src/main.rs:7:17
  |
5 |     thread::scope(|s| {
  |                    - has type `&'1 Scope<'1, '_>`
6 |         s.spawn(|| menu.push(String::from("咖啡")));
  |         -------------------------------------------
  |         |       |  |
  |         |       |  first borrow occurs due to use of `menu` in closure
  |         |       mutable borrow occurs here
  |         argument requires that `menu` is borrowed for `'1`
7 |         s.spawn(|| println!("{}", menu.len()));
  |                 ^^                ---- second borrow occurs due to use of `menu` in closure
  |                 |
  |                 immutable borrow occurs here
```

這是 E0502，跟單一執行緒裡的借用衝突一模一樣。只貼前半段，後面的 `note` 指向標準函式庫裡 `scope` 內部的位置，不用管。編譯器不是為了執行緒另外訂了一套規則，而是同一套規則，在「兩條執行緒同時動同一份資料」這件事上也嚴格執行；第 15 集會看到，這就是 data race 編譯不過的原因。

`scope` 裡的執行緒仍然可以借，但迴圈裡每圈剛建立的變數（像 `for i in 0..3` 的 `i`）活不到範圍結束，要借它還是得 `move`，第 7 集會遇到。

## 重點整理

- `thread::scope(|s| { s.spawn(|| ...); })` 開範圍執行緒：保證在 `scope` 回傳前全部結束，所以可以直接借用外面的變數，不必 `move`、不必 `'static`。
- `s.spawn` 回傳 handle，可以 `join` 取結果；沒 `join` 的會在範圍結束時自動等完，有人 panic，`scope` 也會 panic。
- 每條執行緒可以各自 `&mut` 借不同的變數；範圍結束後借用歸還，主執行緒可以直接讀。
- 借用規則照舊：兩條執行緒同時要改、或一改一讀同一份資料，是 E0499／E0502。
- 要借的東西活得比執行緒久時用 `scope`；執行緒要活過目前的函式（`thread::spawn`），就得擁有資料。
