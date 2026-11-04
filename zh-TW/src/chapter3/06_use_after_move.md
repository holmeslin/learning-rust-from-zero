# move 之後不能再用（E0382）

## 本集目標

看到 E0382（用了已經 move 的變數）時，知道訊息在說什麼，也知道怎麼改。

## 正文

### 用已經 move 的變數

上一集說，move 之後 `s1` 不再是擁有者。如果還去用它，編譯器會擋下來：

```rust,compile_fail=E0382
fn main() {
    let s1 = String::from("哈囉");
    let s2 = s1;
    println!("{s1}");
    println!("{s2}");
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:4:16
  |
2 |     let s1 = String::from("哈囉");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{s1}");
  |                ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++
```

這是所有權最常見的錯誤。訊息可以這樣讀：

- 第一行 `borrow of moved value: s1`：`s1` 的值已經被 move 走了。`borrow`（借用）第 4 章才會講，現在把它當成「用到」就好，`println!` 要看 `s1` 的內容，算用到。
- `move occurs because s1 has type String, which does not implement the Copy trait`：move 的原因，是 `String` 沒有實作 `Copy`。`Copy` 第 8 集會講，現在先記得：`String` 一指定就 move。
- `value moved here` 指向 `let s2 = s1;`，是 move 發生的地方；`value borrowed here after move` 指向後面又用到它的地方。
- 最後的 `help` 建議用 `clone`，下一集再說。

### 怎麼改

最直接的改法，是之後都改用新的擁有者 `s2`，也就是上一集那支程式。如果真的要印 `s1`，就趁它還是擁有者的時候先用完：

```rust
fn main() {
    let s1 = String::from("哈囉");
    println!("{s1}");
    let s2 = s1;
    println!("{s2}");
}
```

執行結果：

```text
哈囉
哈囉
```

重點是順序：用 `s1` 的程式，要排在 move 之前。

### 迴圈裡也會中

把外面的 `String` move 進迴圈，第一圈就把它帶走了，第二圈再 move 一次，就是用到已經 move 的值：

```rust,compile_fail=E0382
fn main() {
    let s = String::from("哈囉");
    for i in 0..2 {
        let t = s;
        println!("{i} {t}");
    }
}
```

編譯錯誤：

```text
error[E0382]: use of moved value: `s`
 --> src/main.rs:4:17
  |
2 |     let s = String::from("哈囉");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     for i in 0..2 {
  |     ------------- inside of this loop
4 |         let t = s;
  |                 ^ value moved here, in previous iteration of loop
  |
help: consider cloning the value if the performance cost is acceptable
  |
4 |         let t = s.clone();
  |                  ++++++++
```

訊息裡的 `value moved here, in previous iteration of loop`，意思是「上一圈已經 move 走了」。如果每一圈都需要一個 `String`，就在迴圈裡面建立，每一圈各用各的：

```rust
fn main() {
    for i in 0..2 {
        let t = String::from("哈囉");
        println!("{i} {t}");
    }
}
```

執行結果：

```text
0 哈囉
1 哈囉
```

## 重點整理

- move 之後，原本的變數不能再用，否則編譯錯誤 E0382。
- 訊息裡 `value moved here` 是 move 發生的地方，`value borrowed here after move` 是之後又用到的地方；`borrow` 第 4 章才講，先當成「用到」。
- 修法之一：之後改用新的擁有者，或把用到原變數的程式移到 move 之前。想留兩份，要用 `clone`（下一集）。
- 把外面的 `String` move 進迴圈，第二圈就會 E0382；每一圈要用，就在迴圈裡建立。
