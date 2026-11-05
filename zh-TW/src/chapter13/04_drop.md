# `Drop`

## 本集目標

替型別實作 `Drop`，讓值被 drop 時執行自己寫的程式；親眼看 drop 的順序，並知道提早丟棄要用 `drop(x)`，不能寫 `x.drop()`。

## 正文

### drop 的時候，印一行字

第 3 章第 4 集說，擁有者離開作用域，值就被 drop。那時候看不見，現在實作 `Drop` trait，讓它在 drop 時印出訊息：

```rust
struct Cup {
    name: String,
}

impl Cup {
    fn new(name: &str) -> Cup {
        Cup {
            name: String::from(name),
        }
    }
}

impl Drop for Cup {
    fn drop(&mut self) {
        println!("洗杯子：{}", self.name);
    }
}

fn main() {
    let a = Cup::new("紅茶");
    {
        let b = Cup::new("咖啡");
        println!("內層：{}", b.name);
    }
    let mut c = Cup::new("奶茶");
    println!("外層：{} {}", a.name, c.name);
    c = Cup::new("拿鐵");
    println!("換成 {}", c.name);
    let c = Cup::new("抹茶");
    println!("遮住之後 {}", c.name);
    drop(a);
    println!("提早丟掉紅茶");
}
```

執行結果：

```text
內層：咖啡
洗杯子：咖啡
外層：紅茶 奶茶
洗杯子：奶茶
換成 拿鐵
遮住之後 抹茶
洗杯子：紅茶
提早丟掉紅茶
洗杯子：抹茶
洗杯子：拿鐵
```

`Drop` 只有一個方法 `drop(&mut self)`，值要被 drop 時，編譯器自動呼叫它，不需要 `use`。逐行對照輸出：

- 內層的 `}`：`b` 離開作用域，印出「洗杯子：咖啡」。
- `c = Cup::new("拿鐵")`：新值建好之後，舊的「奶茶」當場被 drop。
- `let c = ...` 是 shadowing（第 1 章第 6 集）：「拿鐵」只是被蓋住，沒有被 drop。
- `drop(a)`：提早丟掉紅茶，下一節說明。
- `main` 結束：還活著的變數依建立順序反過來 drop，先「抹茶」，再「拿鐵」。

這正是第 3 章第 4 集預告的：重新指定值，舊的先 drop；shadowing 的舊值要等到作用域結束；同一個作用域裡，後建立的先 drop。

### 提早丟棄：`drop(x)`

上面的 `drop(a)` 是標準函式庫的 `drop` 函式（不用 `use`）。它拿走 `a` 的所有權，函式一結束，`a` 就被 drop，跟第 3 章第 9 集把值傳進函式是同一件事。丟掉之後 `a` 已經 move，再用就是 E0382。

想直接呼叫 `Drop` 的 `drop` 方法不行：

```rust,compile_fail=E0040
struct Cup;

impl Drop for Cup {
    fn drop(&mut self) {
        println!("洗杯子");
    }
}

fn main() {
    let cup = Cup;
    cup.drop();
}
```

編譯錯誤：

```text
error[E0040]: explicit use of destructor method
  --> src/main.rs:11:9
   |
11 |     cup.drop();
   |         ^^^^ explicit destructor calls not allowed
   |
help: consider using `drop` function
   |
11 -     cup.drop();
11 +     drop(cup);
   |
```

編譯器不准你呼叫它，因為離開作用域時還會再呼叫一次，同一個值就被 drop 兩次。要提早丟棄，一律用 `drop(cup)`，`help` 也是這樣建議。

另外兩個規則：

- 有 `Drop` 的型別不能同時是 `Copy`（E0184）：複製出來的每一份都要各自 drop，`Copy` 做不到。
- struct 自己的 `drop` 先執行，然後欄位依宣告順序 drop；`Vec` 的元素從第一個到最後一個。這跟區域變數「後建立的先 drop」相反。

### 跟 Go 比一比

Go 用 `defer` 釋放資源，但 `defer` 屬於**函式**，不是區塊：

```go
for _, name := range names {
	f, err := os.Open(name)
	if err != nil {
		return err
	}
	defer f.Close() // 每圈都排隊，整個函式結束才一起關
}
```

迴圈跑一萬圈，就有一萬個檔案開著，直到函式結束。Rust 的 drop 跟著區塊走：`for` 的每一圈，在圈尾的 `}` 就 drop 該圈的變數，不會越堆越多。

## 重點整理

- `impl Drop for 型別 { fn drop(&mut self) { ... } }`：值被 drop 的那一刻自動呼叫，`Drop` 不用 `use`。
- 區域變數依建立順序的相反順序 drop；重新指定值時舊值當場 drop；shadowing 的舊值活到作用域結束。
- 提早丟棄用 `drop(x)`（拿走所有權）；呼叫 `x.drop()` 是 E0040；丟掉之後再用是 E0382。
- struct 先跑自己的 `drop`，再依欄位宣告順序 drop 欄位；`Vec` 從第一個元素 drop 到最後一個；有 `Drop` 的型別不能 `Copy`（E0184）。
- 跟 Go 比：`defer` 屬於函式，要等函式結束；Rust 的 drop 屬於區塊，離開那個 `}` 就發生。
