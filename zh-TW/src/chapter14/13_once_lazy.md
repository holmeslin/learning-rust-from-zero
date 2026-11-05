# `OnceLock` 與 `LazyLock`

## 本集目標

用 `LazyLock` 與 `OnceLock` 建立「執行時期才算得出來、只算一次」的全域值，也會把 `Mutex` 放進 `static` 當全域的可變狀態。

## 正文

### `static` 的初始值要在編譯時期算得出來

第 2 章第 1 集說過，`const` 與 `static` 的值要在編譯時期算得出來。想放一張價目表（`HashMap`）在全域，寫成 `static MENU: HashMap<&str, u32> = HashMap::new();`，就會碰到這道牆：E0015，`cannot call non-const associated function`。`HashMap::new` 不是編譯時期能算的函式（它要建立隨機種子），`String::from("你好")` 也一樣。編譯器的 `note` 已經指了路：`consider wrapping this expression in std::sync::LazyLock::new(|| ...)`。

### `LazyLock`：第一次用到才初始化

`LazyLock<T>` 在建立時收一個閉包，等到有人第一次存取，才執行閉包算出值，之後永遠是同一個：

```rust
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

static LOADS: AtomicU32 = AtomicU32::new(0);

static MENU: LazyLock<HashMap<&str, u32>> = LazyLock::new(|| {
    LOADS.fetch_add(1, Ordering::SeqCst);
    let mut menu = HashMap::new();
    menu.insert("紅茶", 30);
    menu.insert("咖啡", 45);
    menu
});

fn main() {
    println!("還沒有人查價目表，載入 {} 次", LOADS.load(Ordering::SeqCst));
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| MENU.get("紅茶").copied());
        }
    });
    println!("紅茶 {} 元", MENU["紅茶"]);
    println!("四條執行緒都查過，載入 {} 次", LOADS.load(Ordering::SeqCst));
}
```

執行結果：

```text
還沒有人查價目表，載入 0 次
紅茶 30 元
四條執行緒都查過，載入 1 次
```

- `MENU` 當成 `HashMap` 用（`.get`、`["紅茶"]`），因為 `LazyLock` 實作了 `Deref`（第 13 章第 3 集）。
- 四條執行緒同時第一次存取，閉包還是只跑一次：其他執行緒會等它算完。`LOADS` 就是用來數閉包跑了幾次。
- 沒人查的時候，閉包不會跑，所以第一行印的是 0 次。

### `OnceLock`：只能寫一次的格子

初始值要等程式執行到某個地方才知道（讀設定、命令列參數），就用 `OnceLock<T>`：先放空的，之後由你決定什麼時候填進去，只能填一次：

```rust
use std::sync::OnceLock;

static SHOP_NAME: OnceLock<String> = OnceLock::new();

fn main() {
    println!("設定之前：{:?}", SHOP_NAME.get());
    SHOP_NAME
        .set(String::from("小美茶坊"))
        .expect("已經設定過了");
    println!("設定之後：{:?}", SHOP_NAME.get());
    let second = SHOP_NAME.set(String::from("大美茶坊"));
    println!("再設定一次：{second:?}");
    let name = SHOP_NAME.get_or_init(|| String::from("不會用到"));
    println!("店名還是 {name}");
}
```

執行結果：

```text
設定之前：None
設定之後：Some("小美茶坊")
再設定一次：Err("大美茶坊")
店名還是 小美茶坊
```

- `get()` 回傳 `Option<&T>`：還沒填是 `None`。
- `set(值)` 回傳 `Result`：第一次成功；第二次是 `Err`，並把沒用上的值還給你。
- `get_or_init(閉包)` 有值就直接回傳，沒值才用閉包填，是「第一個到的人負責初始化」。
- `LazyLock` 的閉包寫在宣告的地方；`OnceLock` 由你在別處決定填什麼，所以兩個各有用處。

### `static` 放 `Mutex`：全域的可變狀態

`Mutex::new` 與 `Vec::new` 都是編譯時期能算的函式，所以 `Mutex` 可以直接放進 `static`，不必 `LazyLock`。第 2 章第 1 集留下一個問題：可以隨時修改的全域變數怎麼寫？這是另一個答案（上一集是 atomic）：

```rust
use std::sync::Mutex;
use std::thread;

static ORDERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    thread::scope(|s| {
        for id in 1..=3 {
            s.spawn(move || ORDERS.lock().expect("訂單簿壞了").push(id));
        }
    });
    let mut orders = ORDERS.lock().expect("訂單簿壞了");
    orders.sort();
    println!("{orders:?}");
}
```

執行結果：

```text
[1, 2, 3]
```

`static` 是全域的，不需要 `Arc`，每條執行緒直接用名字就能碰到。不過全域變數讓程式的哪個角落都能改它，一般還是優先把資料當參數傳。

以前的教學常用 `lazy_static` 或 `once_cell` 這兩個 crate 做同樣的事，現在標準函式庫已經內建，不需要它們。

### 跟 Go 比一比

Go 的套件層級變數在程式啟動時就依序初始化，也可以寫 `init()` 函式；需要「用到才算、只算一次」時，用 `sync.Once` 或 `sync.OnceValue`：

```go
var menu = sync.OnceValue(func() map[string]int {
	return map[string]int{"紅茶": 30, "咖啡": 45}
})

// menu()["紅茶"]
```

Rust 沒有 `init()`，也沒有「`main` 之前先跑一段程式」這回事：`static` 只能是編譯時期算好的值。要在執行時期才算，一定是明寫的 `LazyLock` 或 `OnceLock`，什麼時候初始化、有沒有初始化，從程式碼就看得出來。

## 重點整理

- `static` 的初始值必須是編譯時期能算的運算式（`HashMap::new()`、`String::from` 都不行，E0015）。
- `LazyLock<T>`：`static X: LazyLock<T> = LazyLock::new(|| ...)`，第一次存取才跑閉包，只跑一次；多條執行緒同時第一次存取，其他的會等。
- `OnceLock<T>`：先放空，`set` 只能成功一次（第二次回傳 `Err`），`get` 回傳 `Option<&T>`，`get_or_init` 沒值才初始化。
- `Mutex::new(Vec::new())` 可以直接放進 `static`，做全域的可變狀態；不需要 `Arc`。
- 這兩個取代以前的 `lazy_static`、`once_cell`。跟 Go 比：Rust 沒有 `init()`，全域的執行時期初始化都是明寫的 `LazyLock`／`OnceLock`。
