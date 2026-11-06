# `std::time`

## 本集目標

認識 `std::time` 的三個型別：`Duration`（一段時間）、`Instant`（量經過時間用的時鐘）、`SystemTime`（牆上時鐘）；知道什麼時候用哪一個，也知道標準函式庫沒有日期格式化。

## 正文

### `Duration`：一段時間

第 14 章第 1 集一直照抄的 `thread::sleep(Duration::from_millis(50))`，裡面的 `Duration` 就是這一集的主角。它代表一段時間，內部是「秒」加「奈秒」，**不會是負的**：

```rust
use std::time::Duration;

fn main() {
    let a = Duration::from_millis(1500);
    let b = Duration::from_secs(2);
    println!("{a:?} {b:?}");
    println!("{} {} {}", a.as_secs(), a.as_millis(), a.subsec_millis());
    println!("{}", a.as_secs_f64());
    println!("{:?} {:?} {:?}", a + b, a * 3, b / 4);
    println!("{}", a < b);
    println!("{:?}", Duration::from_secs_f64(0.25));
    println!("{:?}", Duration::from_micros(2500));
    println!("{:?}", Duration::from_secs(90));
}
```

執行結果：

```text
1.5s 2s
1 1500 500
1.5
3.5s 4.5s 500ms
true
250ms
2.5ms
90s
```

- 建立：`from_secs`、`from_millis`、`from_micros`、`from_nanos`，或用小數秒 `from_secs_f64`。
- 取出：`as_secs()` 是整數秒（1.5 秒取 1，小數丟掉）、`as_millis()` 是總毫秒數、`subsec_millis()` 只取不足一秒的部分、`as_secs_f64()` 是小數秒。注意 `as_millis()` 的型別是 `u128`，`as_secs()` 是 `u64`。
- 可以加、減、乘以整數、除以整數，也能比較大小。
- 用 `{:?}` 印出來會自動挑單位：`1.5s`、`500ms`、`2.5ms`；90 秒就是 `90s`，不會換成「1 分 30 秒」。要自己換算分、秒，用 `as_secs() / 60` 與 `as_secs() % 60`。

### 不能減成負的

`Duration` 沒有負數，所以小的減大的沒有答案。`a - b` 在 `b` 比較大時會 panic（`overflow when subtracting durations`），不確定誰大就用會回傳結果的版本：

```rust
use std::time::Duration;

fn main() {
    let a = Duration::from_secs(1);
    let b = Duration::from_secs(2);
    println!("{:?}", a.checked_sub(b));
    println!("{:?}", a.saturating_sub(b));
    println!("{:?}", b - a);
}
```

執行結果：

```text
None
0ns
1s
```

`checked_sub` 減不出來就回傳 `None`，`saturating_sub` 減不出來就停在 0（整數也有 `checked_add`、`saturating_add` 這組，跟第 1 章第 16 集的 `wrapping_add` 同一套命名）。

### `Instant`：量經過了多久

要量程式跑了多久，用 `Instant`：`Instant::now()` 記下現在，之後呼叫 `elapsed()` 得到經過的 `Duration`。`Instant` 是單調的時鐘：只會往前走，不會因為系統校時而倒退，量時間差一定用它：

```rust
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    thread::sleep(Duration::from_millis(50));
    let elapsed = start.elapsed();
    println!("{}", elapsed >= Duration::from_millis(50));
    println!("{}", elapsed < Duration::from_secs(5));
    let deadline = start + Duration::from_secs(60);
    println!("{}", Instant::now() < deadline);
}
```

執行結果：

```text
true
true
true
```

實際花了幾毫秒每次都不同，所以這裡只比大小，不印數字；平常你會寫 `println!("花了 {:?}", start.elapsed());`。`sleep` 保證至少睡這麼久，所以第一行一定是 `true`。`Instant` 本身的值沒有意義（不是幾點幾分），只能拿來相減：`later.duration_since(start)`，或加上一段 `Duration` 當期限（上面的 `deadline`）。

### `SystemTime`：牆上時鐘

要知道「現在是什麼時候」用 `SystemTime`，它跟系統時鐘走，可以被使用者或校時調回去。常見的用法是跟 `UNIX_EPOCH`（1970 年 1 月 1 日）相減，得到「自 1970 年以來經過幾秒」：

```rust
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    let since = now.duration_since(UNIX_EPOCH).expect("時鐘在 1970 年之前");
    println!("{}", since.as_secs() > 1_700_000_000);

    let tomorrow = UNIX_EPOCH + Duration::from_secs(86_400);
    println!("{:?}", tomorrow.duration_since(UNIX_EPOCH));
    println!("{}", UNIX_EPOCH.duration_since(tomorrow).is_err());
}
```

執行結果：

```text
true
Ok(86400s)
true
```

`duration_since` 回傳 `Result`：「後面的比較晚」就是 `Err`，因為牆上時鐘可能倒退，標準函式庫不假裝這件事不會發生。1_700_000_000 秒大約是 2023 年底，所以現在的時間一定比它大。

標準函式庫**沒有**把時間格式化成「2026-11-06 14:30」、也沒有時區與日期運算，這些要靠別的 crate（`chrono`、`time`），本書不展開。

### 跟 Go 比一比

- Go 的 `time.Duration` 是 `int64` 奈秒，可以是負的，`2 * time.Second` 這種常數運算很方便；Rust 的 `Duration` 不能是負的，要用 `Duration::from_secs(2)` 建立。
- Go 的 `time.Now()` 同時帶著牆上時間與單調時鐘讀數，`time.Since(start)` 會自動用單調的那個。Rust 把兩件事分成兩個型別：量間隔用 `Instant`，查日期時間用 `SystemTime`，不會用錯。
- Go 的 `time.Time` 內建 `Format`、時區；Rust 的標準函式庫沒有。

## 重點整理

- `Duration` 是一段不會為負的時間：`from_secs`／`from_millis`／`from_secs_f64` 建立，`as_secs`、`as_millis`（`u128`）、`subsec_millis`、`as_secs_f64` 取出，`{:?}` 印成 `1.5s`、`500ms`。
- `Duration` 可以加減乘除與比較；小的減大的會 panic，用 `checked_sub`（`None`）或 `saturating_sub`（0）。
- `Instant::now()` 加 `elapsed()` 量間隔，單調、不會倒退；值本身沒有意義，只拿來相減。經過時間每次不同，不要期待固定的輸出。
- `SystemTime` 是牆上時鐘，跟 `UNIX_EPOCH` 用 `duration_since` 相減，回傳 `Result`；標準函式庫沒有日期格式化。
- 跟 Go 比：Go 的 `Duration` 可以是負的，`time.Now()` 兼具兩種時鐘；Rust 分成 `Instant` 與 `SystemTime`。
