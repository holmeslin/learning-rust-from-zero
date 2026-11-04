# `loop` 與 `break` 帶值

## 本集目標

用 `loop` 寫一個重複執行的迴圈，並用 `break` 結束它，甚至讓 `break` 帶一個值出來。

## 正文

### `loop` 是無限迴圈

`loop` 會一直重複執行大括號裡的程式，直到遇到 `break` 為止：

```rust
fn main() {
    let mut count = 0;
    loop {
        count += 1;
        println!("第 {count} 圈");
        if count == 3 {
            break;
        }
    }
}
```

執行結果：

```text
第 1 圈
第 2 圈
第 3 圈
```

`break` 的作用跟 Go 一樣：立刻跳出迴圈。如果忘了寫 `break`，程式就會一直跑下去，在終端機按 `Ctrl-C` 可以強制停止。

### `break` 可以帶一個值

`loop` 跟 `if` 一樣是運算式，所以可以把它的結果存進變數。做法是在 `break` 後面接一個值，迴圈結束時，這個值就是整個 `loop` 的值：

```rust
fn main() {
    let mut n = 0;
    let found = loop {
        n += 1;
        if n % 7 == 0 && n % 5 == 0 {
            break n;
        }
    };
    println!("第一個同時是 7 和 5 的倍數的正整數是 {found}");
}
```

執行結果：

```text
第一個同時是 7 和 5 的倍數的正整數是 35
```

`break n;` 把 `n` 交給 `found`。注意整個 `loop { ... }` 後面要加分號，因為它是 `let` 陳述式的右邊。

### 搭配讀輸入

`loop` 很適合「一直讀，直到使用者說停」的情況。下面這支程式一直讀整數並累加，讀到 `0` 就結束：

```rust,stdin=3\n5\n0,noplayground
fn main() {
    println!("請輸入整數，輸入 0 結束：");
    let mut sum = 0;
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("讀取失敗");
        let line = line.trim();
        let n: i32 = line.parse().expect("請輸入整數");
        if n == 0 {
            break;
        }
        sum += n;
    }
    println!("總和是 {sum}");
}
```

依序輸入 `3`、`5`、`0`，程式印出：

```text
請輸入整數，輸入 0 結束：
總和是 8
```

每一圈都重新建立一個 `line`，所以讀輸入的三行就寫在迴圈裡面。

### 跟 Go 比一比

Go 的無限迴圈是 `for { ... }`，Rust 用 `loop { ... }`。寫成 `while true` 也能跑，但編譯器會警告你改用 `loop`。

而且 `break` 帶值是 Rust 才有的：Go 的 `for` 不是運算式，要把結果帶出迴圈，得先宣告一個變數，在迴圈裡賦值，再 `break`。

## 重點整理

- `loop { ... }` 是無限迴圈，用 `break` 結束；Go 對應的寫法是 `for { ... }`。
- `loop` 是運算式，`break 值;` 可以讓整個 `loop` 得到這個值。
- 要一直讀輸入直到某個條件，用 `loop` 加 `break` 很自然。
- 迴圈裡面建立的變數，每一圈都是新的。
