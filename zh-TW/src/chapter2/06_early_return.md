# early `return`

## 本集目標

用 `return` 在函式中途提早離開，並知道它跟「最後一個運算式」怎麼分工。

## 正文

### 提早離開

函式跑到一半就知道答案時，可以用 `return 值;` 立刻結束函式，把值交出去，後面的程式碼不會再執行：

```rust
fn member_price(price: i32, is_member: bool) -> i32 {
    if !is_member {
        return price;
    }
    price * 9 / 10
}

fn main() {
    println!("{}", member_price(200, true));
    println!("{}", member_price(200, false));
}
```

執行結果：

```text
180
200
```

不是會員就直接回傳原價；會員才繼續往下，最後一行 `price * 9 / 10`（九折）沒有分號，照舊是函式的回傳值。`return` 負責「中途離開」，最後一個運算式負責「正常走到底」。

### 迴圈裡的 `return`

`return` 可以放在迴圈裡面，一找到答案就整個函式結束。下面用 `for` 判斷質數：

```rust
fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    for n in 1..=12 {
        if is_prime(n) {
            println!("{n}");
        }
    }
}
```

執行結果：

```text
2
3
5
7
11
```

只要找到一個能整除的 `i`，就知道 `n` 不是質數，直接 `return false;`。整個迴圈跑完都沒有被整除，才會走到最後的 `true`。這跟 Go 的寫法一樣。

### 沒有回傳值的 `return;`

不回傳值的函式也可以提早離開，只寫 `return;`：

```rust
fn show_score(score: i32) {
    if score < 0 || score > 100 {
        println!("分數不合法");
        return;
    }
    println!("分數是 {score}");
}

fn main() {
    show_score(85);
    show_score(-5);
}
```

執行結果：

```text
分數是 85
分數不合法
```

### 結尾的 `return` 不必要

在函式的最後一行寫 `return a + b;` 是合法的，但不是 Rust 習慣的寫法。最後一個運算式本來就會被回傳，習慣上只有「中途離開」才用 `return`。讀 Rust 程式時，看到 `return` 就可以知道：這裡是個提早離開的出口。

## 重點整理

- `return 值;` 立刻結束函式並交出值；不回傳值的函式寫 `return;`。
- `return` 可以放在 `if` 或迴圈裡，找到答案就提早離開。
- 正常走到底的值，用最後一個沒有分號的運算式；結尾不需要寫 `return`。
- 習慣上，Rust 的 `return` 只用在中途離開。
