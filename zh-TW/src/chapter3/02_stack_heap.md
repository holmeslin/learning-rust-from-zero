# stack 與 heap

## 本集目標

分辨堆疊（stack）和堆積（heap），知道哪些型別的資料放在哪裡，以及為什麼 `String` 的內容得放在 heap。

## 正文

### 兩種放資料的地方

程式執行時，有兩個地方可以放資料。

**堆疊**（stack）：每呼叫一個函式，就在 stack 上疊一層，放這個函式的區域變數；函式結束，整層一起拿掉。後進先出，速度很快。條件是：每個值的大小，在編譯時期就要知道。

**堆積**（heap）：一大塊比較自由的空間。要放資料時，向系統要一塊指定大小的空間，系統告訴你它的位置，用完要還。比 stack 慢，但大小可以到執行時才決定，也可以變大。

### 大小固定的放 stack

整數、浮點數、`bool`、`char`，以及由它們組成的 tuple 和陣列，大小都在編譯時期就確定了，所以直接放在 stack 上：

```rust
fn main() {
    let n = 42;
    let point = (3, 4);
    let scores = [85, 92, 78];
    println!("{n} {point:?} {scores:?}");
}
```

執行結果：

```text
42 (3, 4) [85, 92, 78]
```

`n` 是 4 個位元組（`i32`），`scores` 是三個 `i32`，加起來 12 個位元組。這些大小寫在型別裡，不會變。

### 會變長的放 heap

`String` 不一樣。同一個 `String`，可以先放兩個字，再接上更多：

```rust
fn main() {
    let mut s = String::from("Hi");
    println!("{}", s.len());
    s.push_str(", Rust");
    println!("{}", s.len());
}
```

執行結果：

```text
2
8
```

編譯時期沒辦法知道 `s` 最後會多長，所以文字內容不能直接放在 stack 的固定位置，而是放在 heap：需要更多空間時，向系統要一塊更大的。

那 stack 上放什麼？一個 `String` 在 stack 上只有三個小欄位：指向 heap 的位置（pointer）、目前的長度（length），以及 heap 上預留了多少空間（capacity）。以 `String::from("Hello")` 為例：

```text
stack                      heap
s: [ ptr | len 5 | cap 5 ] ---> H e l l o
```

三個欄位的大小固定，放在 stack；文字本身放在 heap。第 1 集說的「誰負責還記憶體」，要還的就是 heap 上這塊。stack 那一層會跟著函式結束自動拿掉，不用另外管。

### 放哪裡，由型別決定

Go 也有 stack 和 heap，但你看不到：編譯器用逃逸分析自己決定每個變數放哪，寫程式時不用管。Rust 是由型別決定：整數、tuple、陣列放 stack；`String` 的內容放 heap。想把別的東西放進 heap，要自己用 `Box`（第 13 章第 1 集）。

這一章裡的 move、`Copy`、`clone`，都是在處理同一件事：heap 上那塊資料，到底歸誰。

## 重點整理

- stack 放大小在編譯時期就固定的資料，後進先出，函式結束時整層拿掉，速度快。
- heap 放大小到執行時才決定、或會變大的資料；要向系統要空間、用完要還，比較慢。
- 整數、浮點數、`bool`、`char`，以及它們組成的 tuple 和陣列，直接放 stack。
- `String` 分成兩半：指標、長度、容量三個欄位放 stack，文字內容放 heap。
- 所有權要管的，主要是 heap 上那塊資料誰負責還。
