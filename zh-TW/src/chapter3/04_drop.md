# 離開作用域就 drop

## 本集目標

知道值會在擁有者離開作用域的那個 `}` 被 drop，包括區塊、函式結束，以及重新指定值的時候。

## 正文

### drop 發生在 `}`

擁有者離開作用域，值就被 drop（丟棄，記憶體釋放）。先故意寫錯，看作用域的邊界在哪裡：

```rust,compile_fail=E0425
fn main() {
    {
        let s = String::from("哈囉");
        println!("{s}");
    }
    println!("{s}");
}
```

編譯錯誤：

```text
error[E0425]: cannot find value `s` in this scope
 --> src/main.rs:6:16
  |
6 |     println!("{s}");
  |                ^
  |
help: the binding `s` is available in a different scope in the same function
 --> src/main.rs:3:13
  |
3 |         let s = String::from("哈囉");
  |             ^
```

內層的 `}` 一到，`s` 離開作用域：名字不能再用，它擁有的 `String` 也在同一個位置被 drop。編譯器就是在這個 `}` 放進釋放記憶體的動作，所以你永遠知道釋放發生在哪一行，不是某個不知道什麼時候才來的回收。

### 函式結束也一樣

函式的大括號也是區塊。函式裡建立的變數，函式結束時就被 drop：

```rust
fn greet() {
    let s = String::from("哈囉");
    println!("{s}");
}

fn main() {
    greet();
    greet();
}
```

執行結果：

```text
哈囉
哈囉
```

`greet` 呼叫兩次，就建立兩個 `String`、drop 兩次，每次都在 `greet` 的 `}`。

### 重新指定值，舊的先 drop

`let mut` 的變數被指定成新的值時，舊的值已經沒有擁有者，會當場被 drop：

```rust
fn main() {
    let mut s = String::from("舊的");
    println!("{s}");
    s = String::from("新的");
    println!("{s}");
}
```

執行結果：

```text
舊的
新的
```

`s = String::from("新的")` 那一行執行時，「舊的」就被 drop 了。

shadowing 不一樣：再寫一次 `let s = ...`，是建立新的變數，舊的變數只是被蓋住、叫不到而已，它擁有的值要等到作用域結束才 drop。

同一個作用域裡有好幾個變數時，後建立的先 drop。這些動作平常看不到，第 13 章學到 `Drop` 之後，可以讓它們在 drop 時印出訊息，親眼看順序。

### 跟 Go 比一比

Go 的記憶體由 GC 回收，什麼時候回收你不知道，也不用管。但檔案、連線、鎖這類資源，要自己記得釋放，慣用的做法是 `defer`：

```go
f, err := os.Open("data.txt")
if err != nil {
    return err
}
defer f.Close() // 忘了寫，檔案就一直開著
```

Rust 的 drop 時間確定，所以這類資源也跟著擁有者走：開啟的檔案離開作用域，就在那個 `}` 自動關閉。不用另外寫 `defer`，也就不會忘。檔案要到第 17 章才會學，但它靠的就是這一集的 drop。

## 重點整理

- 擁有者離開作用域的那個 `}`，值就被 drop，記憶體在那一刻釋放。
- 函式的 `}` 也一樣：函式裡建立的變數，函式結束時被 drop。
- 對 `let mut` 變數重新指定值，舊的值當場被 drop；shadowing 不會，舊的值要等到作用域結束。
- 同一個作用域裡，後建立的變數先 drop。
- Go 的記憶體靠 GC，資源要用 `defer` 釋放；Rust 的 drop 時間確定，資源隨擁有者自動釋放。
