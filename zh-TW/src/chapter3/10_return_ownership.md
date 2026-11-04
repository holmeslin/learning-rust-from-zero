# 回傳值交還所有權

## 本集目標

知道函式回傳 `String` 時，所有權會交給呼叫端，所以可以用回傳值把傳進去的 `String` 拿回來。

## 正文

### 函式把剛建立的 `String` 交出去

函式裡建立的變數，函式結束時會被 drop。可是如果它是回傳值，所有權就交給呼叫端，不會在函式裡被 drop：

```rust
fn make_greeting() -> String {
    let mut s = String::from("哈囉");
    s.push_str("，世界");
    s
}

fn main() {
    let greeting = make_greeting();
    println!("{greeting}");
}
```

執行結果：

```text
哈囉，世界
```

`make_greeting` 最後一行的 `s` 沒有分號，是整個函式的回傳值（第 2 章第 4 集）。它 move 給呼叫端的 `greeting`，所以 `}` 一到，`s` 雖然離開作用域，卻已經不是擁有者，沒有東西可以 drop。`greeting` 是新的擁有者，要等 `main` 結束才會 drop。

### 傳進去，再傳回來

上一集看到，`String` 傳進函式就回不來了。要拿回來，就讓函式把它回傳：

```rust
fn cheer(mut s: String) -> String {
    s.push_str("，加油");
    s
}

fn main() {
    let msg = String::from("哈囉");
    let msg = cheer(msg);
    let msg = cheer(msg);
    println!("{msg}");
}
```

執行結果：

```text
哈囉，加油，加油
```

整個過程是一連串的 move：`msg` 傳進 `cheer`，`cheer` 改完再 move 回來；`let msg = cheer(msg);` 右邊用的是舊的 `msg`（已經 move 走了），左邊是新的 `msg`，接住回傳的值。第 1 章第 6 集的 shadowing 在這裡剛好派上用場，名字可以沿用，不用一直想新名字。

參數前面的 `mut` 也是第 2 章看過的：函式要改 `s`，就要寫 `mut s`。它只是函式裡的那份擁有者可以改，跟所有權怎麼走無關。

### 規則其實沒變

這一集沒有新規則。傳進函式是 move，回傳也是 move，只是方向相反：

- 傳進去：呼叫端 → 參數，呼叫端失去所有權。
- 回傳：函式 → 呼叫端，呼叫端得到所有權。

函式不想還，所有權就留在函式裡，在 `}` 被 drop；函式想還，就放在回傳值。

## 重點整理

- 函式的回傳值會 move 給呼叫端，所以函式裡建立的 `String` 可以交出去，不會在函式結束時被 drop。
- 想把傳進函式的 `String` 拿回來，就讓函式回傳它：`let msg = cheer(msg);`。
- 用 shadowing 沿用同一個名字，是這種寫法的常見做法。
- 傳進去和回傳都是 move，方向相反；沒回傳的，就在函式的 `}` 被 drop。
