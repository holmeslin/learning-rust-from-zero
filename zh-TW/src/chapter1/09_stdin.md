# 讀取輸入

## 本集目標

讓程式在執行時停下來，讀取使用者打的一行文字。

## 正文

### 讀一行輸入的固定句型

讀一行輸入，要寫這三行。**現在先照抄**，原理之後才會解釋：

```rust,stdin=Andy,noplayground
fn main() {
    println!("請輸入你的名字：");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    let line = line.trim();
    println!("你好，{line}！");
}
```

用 `cargo run` 執行，程式印出提示之後會停下來等你打字。打完按 Enter，程式才繼續。

輸入 `Andy` 的執行結果：

```text
請輸入你的名字：
你好，Andy！
```

你自己打的 `Andy` 也會顯示在畫面上，在兩行之間，這裡只列出程式印出來的部分。這類要鍵盤輸入的範例沒有 Playground 的執行按鈕，請複製到自己的專案執行。

### 拆開來看

每次要讀一行，就把這三行再寫一次，變數名稱可以自己取：

```rust,ignore
let mut line = String::new();
std::io::stdin().read_line(&mut line).expect("讀取失敗");
let line = line.trim();
```

- 第一行：準備一個空的文字容器 `line`，讀進來的內容要放進去。因為內容會改變，所以要 `let mut`。`String` 是一種文字型別，第 2 章第 15 集再介紹。
- 第二行：`read_line(&mut line)` 讀一行，放進 `line`。`&mut` 的意思第 4 章解釋，`expect("讀取失敗")` 是「讀取出錯就印這句話並停止程式」，第 9 章解釋。
- 第三行：`line.trim()` 把前後的空白和換行去掉。這裡用了第 6 集學的 shadowing，新的 `line` 蓋住舊的。

為什麼需要 `trim`？因為 `read_line` 會把你按下的 Enter 一起讀進來。不去掉的話，換行會跟著被印出來：

```rust,stdin=Andy,noplayground
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    println!("[{line}]");
    let line = line.trim();
    println!("[{line}]");
}
```

執行結果：

```text
[Andy
]
[Andy]
```

### 讀好幾行

要讀第二行，就再寫一次三行，並換個變數名稱：

```rust,stdin=小明\n台北,noplayground
fn main() {
    println!("名字：");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("讀取失敗");
    let name = name.trim();

    println!("住在哪裡：");
    let mut city = String::new();
    std::io::stdin().read_line(&mut city).expect("讀取失敗");
    let city = city.trim();

    println!("{name} 住在 {city}");
}
```

輸入 `小明` 和 `台北` 的執行結果：

```text
名字：
住在哪裡：
小明 住在 台北
```

讀進來的永遠是文字。就算使用者打的是 `18`，拿到的也是文字 `"18"`，不能直接拿來加減。下一集教怎麼轉成數字。

### 跟 Go 比一比

Go 的 `scanner.Text()` 已經幫你去掉換行，Rust 的 `read_line` 不會，所以才需要 `trim`。

另外，`read_line` 是把內容**接在**變數後面，不是覆蓋。同一個 `String` 連讀兩次，兩行會黏在一起。這就是前面每讀一行都重新建立一個新 `String` 的原因。

如果輸入已經結束（例如沒有下一行可讀），`line` 會是空的，程式也不會報錯。現在的句型沒有處理這種情況，先不用管。

## 重點整理

- 讀一行輸入的固定句型：`let mut line = String::new();`、`std::io::stdin().read_line(&mut line).expect("讀取失敗");`、`let line = line.trim();`，先照抄，原理之後解釋。
- `read_line` 會把換行一起讀進來，所以要 `trim()`。
- 每讀一行就重新寫一次這三行，變數名稱可以自己取。
- 讀進來的永遠是文字。
- 要讀鍵盤輸入的範例沒有執行按鈕，要在自己的電腦上用 `cargo run`。
