# 參照用完就結束（NLL）

## 本集目標

知道參照的借用從建立開始，到「最後一次被用到」就結束，不是等到 `}`，所以只要前一個借用用完了，就能接著借下一個。

## 正文

### 借用到最後一次使用為止

上一集的 E0502，只要把 `println!("{r}")` 排到 `s.push_str(...)` 前面，程式就能編譯。原因是：參照的借用，從建立的那一行開始，到**最後一次被用到**為止。之後 `r` 這個名字雖然還在作用域裡，但借用已經結束，不會再擋住別人。

這個規則的名字叫 NLL（non-lexical lifetimes），意思是：借用的長短不是看 `}` 的位置，而是看最後一次使用在哪裡。

```rust
fn main() {
    let mut s = String::from("哈囉");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");

    let r3 = &mut s;
    r3.push_str("，世界");
    println!("{r3}");
}
```

執行結果：

```text
哈囉 哈囉
哈囉，世界
```

`r1`、`r2` 在 `println!` 之後就沒再用了，借用到那裡結束；接下來借出 `&mut s` 給 `r3`，這時沒有任何 `&` 還活著，符合規則。

### 只要後面還有用，借用就沒結束

把 `r1` 留到最後才用，同一段程式就會失敗：

```rust,compile_fail=E0502
fn main() {
    let mut s = String::from("哈囉");

    let r1 = &s;
    let r2 = &s;
    println!("{r2}");

    let r3 = &mut s;
    r3.push_str("，世界");
    println!("{r3} {r1}");
}
```

編譯錯誤：

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:8:14
   |
 4 |     let r1 = &s;
   |              -- immutable borrow occurs here
...
 8 |     let r3 = &mut s;
   |              ^^^^^^ mutable borrow occurs here
 9 |     r3.push_str("，世界");
10 |     println!("{r3} {r1}");
   |                     -- immutable borrow later used here
```

`immutable borrow later used here` 指向最後那行的 `r1`：因為它後面還要用，`r1` 的借用一直撐到那裡，跟中間的 `&mut s` 重疊了。看到 `later used here`，就去找那個「還在用」的地方，把它挪到前面，或是想想看那個參照還需不需要。

### 函式呼叫也是一樣

把參照傳給函式，借用只撐到函式回來為止：

```rust
fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let mut s = String::from("哈囉");
    let n = length(&s);
    s.push_str("，世界");
    println!("{s} 原本長 {n} 個位元組");
}
```

執行結果：

```text
哈囉，世界 原本長 6 個位元組
```

`length(&s)` 回來之後，借用就結束了，`n` 只是單純的數字，跟 `s` 沒有關係，所以接著 `push_str` 沒問題。

### 兩個 `&mut` 也可以，只要不同時

上一集的 E0499，是兩個 `&mut` 同時活著。如果第一個先用完，第二個就能借：

```rust
fn main() {
    let mut s = String::from("哈囉");

    let r1 = &mut s;
    r1.push_str("，世界");

    let r2 = &mut s;
    r2.push_str("，加油");

    println!("{s}");
}
```

執行結果：

```text
哈囉，世界，加油
```

`r1` 在 `push_str` 之後不再使用，`r2` 才開始借。最後的 `println!("{s}")` 也是：`r2` 已經用完，擁有者可以直接讀。

## 重點整理

- 借用從建立參照開始，到最後一次使用為止，不是到 `}`；這個規則叫 NLL。
- 前一個借用用完了，就能接著借衝突的下一個：先 `&` 後 `&mut`、連續兩個 `&mut` 都行。
- 後面還要用的參照，借用就還沒結束；錯誤訊息的 `later used here` 指的就是那個地方。
- 把參照傳進函式，借用只到函式回來為止；函式回傳的若是普通的數字，跟 `s` 就沒有關係了。
