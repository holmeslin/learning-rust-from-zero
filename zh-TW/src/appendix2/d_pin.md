# `Pin` 初探

## 本集目標

知道 `Pin` 要解決什麼問題：有些值開始執行之後就不能搬動；認識 `Unpin`；能自己 `poll` 一個 `Future`，並看懂第 15 章第 3 集先跳過的 `self: Pin<&mut Self>` 與 `Context`。

## 正文

### 有些值不能搬

第 3 章說過，move 就是把位元組複製到新的位置，多半沒有影響。但如果值裡面有一個參照，指向它自己的另一個欄位，搬動之後，參照還指著舊位置：

```text
搬動之前：[ buf | ref ]    ref 指向同一個值裡的 buf
搬動之後：[ buf | ref ]    ref 還指著舊位置，那裡的 buf 已經不是它的了
```

`async` 區塊正是這樣：區域變數與指向它的參照，同時活過一個 `.await`，編譯器把它們放進同一個 `Future` 裡，這個 `Future` 就指向自己。所以 `Future` 被 `poll` 過之後，不能再搬動。

光靠 `&mut` 擋不住：拿到 `&mut T`，就能用 `std::mem::swap` 把整個值換走。`Pin<&mut T>` 是在 `&mut T` 外面包一層，承諾「這個值不會再被搬動」：安全的程式碼拿不到裡面的 `&mut T`，也就換不走它。`Box<T>` 一樣能包成 `Pin<Box<T>>`。

### `Unpin`：搬了也沒關係

多數型別搬動沒有任何問題：整數、`String`、`Vec` 都是。它們自動實作了 `Unpin`（跟第 14 章第 11 集的 `Send`、`Sync` 一樣，由編譯器自動判斷）。對 `Unpin` 的型別，`Pin<&mut T>` 等於 `&mut T`：想包就 `Pin::new(&mut x)`，想拿 `&mut` 也拿得到。

真正不是 `Unpin` 的只有少數：編譯器替 `async` 區塊與 `async fn` 產生的 `Future`，以及特地加了 `std::marker::PhantomPinned` 欄位的型別。

### 親手 `poll` 一個 `Future`

第 15 章第 3 集說，runtime 會一直 `poll`，直到 `Ready`。自己動手做一次：

```rust
use std::future::Future;
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

struct Countdown(u32);

impl Future for Countdown {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Countdown>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if self.0 == 0 {
            Poll::Ready("泡好了")
        } else {
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    let mut cx = Context::from_waker(Waker::noop());
    let mut countdown = pin!(Countdown(2));
    for round in 1..=4 {
        match countdown.as_mut().poll(&mut cx) {
            Poll::Ready(text) => {
                println!("第 {round} 次：{text}");
                break;
            }
            Poll::Pending => println!("第 {round} 次：還沒好"),
        }
    }
}
```

執行結果：

```text
第 1 次：還沒好
第 2 次：還沒好
第 3 次：泡好了
```

- `poll` 的第一個參數 `self: Pin<&mut Countdown>`，就是 `Pin` 出現在 `Future` 定義裡的地方。`Countdown` 只有一個 `u32`，是 `Unpin`，所以 `self.0 -= 1` 這種要 `&mut` 的操作可以直接寫。
- `Context` 帶著 `Waker`，也就是第 15 章第 3 集說的「好了再叫我」。回傳 `Pending` 之前要安排好喚醒，不然 runtime 不會再來問；`wake_by_ref()` 就是請它馬上再 `poll`。這裡沒有 runtime 可以喚醒，所以用什麼都不做的 `Waker::noop()`。
- `pin!` 把值固定在目前的作用域裡，得到 `Pin<&mut Countdown>`；`poll` 會用掉它，所以每一輪用 `as_mut()` 重新借一份。

### `async` 區塊不是 `Unpin`

`Pin::new` 只接受 `Unpin` 的型別，拿它包 `async` 區塊：

```rust,compile_fail=E0277
use std::pin::Pin;

fn main() {
    let mut order = async { 1 + 1 };
    let _pinned = Pin::new(&mut order);
}
```

編譯錯誤：

```text
error[E0277]: `{async block@src/main.rs:4:21: 4:26}` cannot be unpinned
 --> src/main.rs:5:28
  |
5 |     let _pinned = Pin::new(&mut order);
  |                   -------- ^^^^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:4:21: 4:26}`
  |                   |
  |                   required by a bound introduced by this call
  |
  = note: consider using the `pin!` macro
          consider using `Box::pin` if you need to access the pinned value outside of the current scope
```

後面還有一則 `note` 指出 `Pin::new` 的限制，夾帶 `/rustc/` 路徑，這裡省略。訊息已經說出路：`pin!` 把它固定在目前的作用域；`Box::pin` 把它放進 heap，之後想傳來傳去、從函式回傳都行，因為搬動的只是 `Box` 的指標，裡面的 `Future` 沒動。

### `Pin<Box<dyn Future>>`

不同的 `async` 區塊型別都不一樣，想放進同一個變數或回傳值，要用 `dyn`（附錄二 c）。要 pin、要 `dyn`，合起來就是 async 程式裡最常見的 `Pin` 寫法：

```rust
use std::future::Future;
use std::pin::Pin;

fn make(n: u32) -> Pin<Box<dyn Future<Output = u32>>> {
    if n > 1 {
        Box::pin(async move { n * 2 })
    } else {
        Box::pin(std::future::ready(0))
    }
}

fn main() {
    println!("{}", size_of_val(&make(5)));
    println!("{}", size_of::<Pin<Box<u32>>>());
}
```

執行結果：

```text
16
8
```

`Pin<Box<dyn Future<Output = u32>>>` 是胖指標，16；`Pin<Box<u32>>` 跟 `Box<u32>` 一樣是 8：`Pin` 只是型別上的承諾，沒有多佔空間（附錄二 b）。

另外兩個會遇到的地方：遞迴的 `async fn` 要 `Box::pin` 自己；`tokio::select!` 想反覆等同一個 `Future`，要先 `pin!`，這本書不展開。

### 跟 Go 比一比

Go 沒有 `Pin`。goroutine 的 stack 不夠大時，Go 的 runtime 會把整個 stack 複製到新的空間，同時把指向 stack 的指標改成新位置。程式碼從來不用管「值被搬了」。Rust 沒有這層 runtime，編譯好的 `Future` 裡的參照沒人替你改，只能用型別系統保證它不會被搬。

## 重點整理

- 搬動（move）會讓指向自己的參照失效；`async` 產生的 `Future` 就是這種值，被 `poll` 之後不能再搬。`Pin<P>` 包住指標（`&mut T`、`Box<T>`）並承諾值不再被搬動：安全的程式碼拿不到裡面的 `&mut T`，也沒有執行時期成本。
- `Unpin` 是編譯器自動實作的 trait，表示搬了也沒關係；大部分型別都是，這時 `Pin<&mut T>` 跟 `&mut T` 一樣好用。`async` 區塊、`async fn` 與 `PhantomPinned` 不是。
- 要 pin 一個 `!Unpin` 的值：`std::pin::pin!`（固定在目前的作用域）或 `Box::pin`（放進 heap）；`Pin::new` 只接受 `Unpin`，否則是 E0277。
- `Future::poll` 的簽名是 `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`；`Context` 帶著 `Waker`，回傳 `Pending` 前要安排好喚醒。
- 跟 Go 比：Go 的 runtime 搬動 stack 時會改寫指標，所以不需要 `Pin`。
