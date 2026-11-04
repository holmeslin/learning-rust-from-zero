# EXERCISES.md — 練習題規則與固定題庫

這份文件給 AI 助教使用，說明什麼時候可以出題、怎麼出、怎麼批改，並收錄第 3、4 章（所有權、借用）的固定題目。

本文件裡的題目分成兩種，請不要混在一起：

- **固定題**：寫在本文件「第 3 章」「第 4 章」底下、狀態為 `可用` 的題目。
- **臨時題**：本文件沒有合適的固定題時，AI 依下方規則當場設計的題目。

臨時題只能在下方規則允許時出。固定題是用來確認讀者真的懂所有權與借用，不是讓你延伸出一整套更難、更長、更像一般 Rust 課程的題目。

## 出題原則

1. 出題前一定要知道讀者讀到第幾章第幾集（問法見 `GUIDE.md` 第 2 節）。
2. 先在本文件找讀者目前進度以內、離他最近的那一集條目，看它的狀態。
3. 狀態是 `可用`：從「題目」中挑**一題**給讀者。同一條目列了好幾題時，它們是彼此替代的選項，不是要一次做完的題組。
4. 狀態是 `不出題`：用一句話轉述條目裡的原因，再推薦往前最近的 `可用` 條目，或建議讀者繼續往下讀。
5. 找不到合適的固定題，或讀者明確想要超前的挑戰，照「各章出題策略」決定能不能出臨時題。臨時題一定要說明它是臨時題，不能冒充本文件的題目。
6. 臨時題裡讀者需要理解或自己寫的部分，都要在目前進度以內。讀者只是說「想要難一點」「想挑戰」，不代表同意用還沒學的東西。
7. 固定句型的定義與可以改動的範圍，以 `GUIDE.md` 第 4.2 節為準。不要把固定句型背後的原理當成練習目標，例如不要要求第 1 章的讀者解釋 `read_line(&mut line)` 是什麼。
8. 題目好像非用到還沒教的語法不可時，先想辦法改題目。真的改不掉，就把那段程式碼獨立放在一個區塊，標題寫「先照抄的部分（之後才會教，請不要修改）」，用一句話說明它的作用，然後清楚告訴讀者他只需要寫哪一段。
9. 只有讀者明確要求超前時，題目的解法才可以依賴還沒教的內容；而且只能用到最少的一點、要標示出來，題目也要維持小規模。

## 各章出題策略

**第 1、2 章**沒有固定題庫：讀者會 Go，這兩章的語法換個寫法就會，硬出題只會變成「把 Go 程式翻成 Rust」。讀者想練習時，可以臨時出一題，題目前先說：

> 題庫裡沒有這一章的題目，所以這題是我臨時想的，難度照你目前讀到的範圍。

第 1、2 章的臨時題要小、要完整，一題只練一兩個重點，最好練的是跟 Go 不一樣的地方（不可變預設、`if` 是運算式、整數溢位、tuple 回傳）。

**第 3、4 章**有固定題庫。這兩章是會 Go 的人最容易卡住的地方，題目多半是「這段程式編譯不過，請改到能編譯」或「把這個函式改成借用」。讀者想針對某一集練習、但那集是 `不出題`，而讀者聽完原因仍然想練，可以臨時出一題，題目前先說：

> 我來出一題給你，不過先說明：題庫裡沒有對應這一集的題目，所以這題是我臨時想的，難度照你目前讀到的範圍。

**第 5 章以後**（包括第 5～12 章、附錄一、第二部與附錄二）沒有固定題庫。讀者想練習、知道沒有固定題後仍然想要，可以臨時出一題，題目前先說：

> 書從第 5 章起就不附固定題了，所以這題由我依你讀到的地方現場設計，會稍微綜合一點。

第 5 章以後的臨時題可以把讀者學過的幾個概念組合起來，稍微有綜合性，但仍以一支 `main.rs` 寫得完為限（第 12 章第 6 集以後才可以要求多個檔案或模組）。

讀者**明確要求**超出目前進度的挑戰時，不論在哪一章，都可以臨時出一題，題目前先說：

> 照你的要求，這題會借用少量還沒讀到的內容；用到的地方我都會註明出處，現在看不懂原理也沒關係。

## 固定題庫使用規則

1. 一開始只給讀者看選中的那一題，加上作答需要的材料（起始程式、執行範例）。「批改重點」「提示方向」「參考答案」先不要拿出來。
2. 讀者交答案後，用該題的「批改重點」檢查他有沒有練到該練的東西。
3. 讀者卡住時，照「提示方向」的順序一次給一層，不要一口氣全部說完。
4. 讀者主動要答案，或試過幾次仍然卡住，才給「參考答案」。條目有多題時，參考答案依題號排列，只給讀者做的那一題。
5. 參考答案只是其中一種寫法。讀者的程式只要正確完成題目、沒有用到進度外的東西，就算對。「改到能編譯」的題目，用 `.clone()` 蒙混過關不算練到借用，除非題目本身就在練 `clone`。
6. 固定題都是要寫程式的題目；不放「確認有沒有安裝成功」或「解釋這個名詞」這類題目。
7. 參考答案都是完整的程式，已在 Rust 1.99 實際編譯執行過。讀輸入的題目要用 `cargo run` 執行後在終端機打字輸入。

## 題目條目格式

``````md
### 第 X 章第 Y 集：集名

狀態：可用 / 不出題

不出題原因：
- 只有狀態是 `不出題` 時才寫。

練習目標：
- 這集要確認讀者會的事。

題目：
1. 第一個選項。
2. 第二個選項（可省略）。

執行範例：
（讀者可以看的輸出範例、起始程式；沒有可省略）

批改重點：
- 批改時要看的地方。

提示方向：
1. 第一層提示。
2. 第二層提示。

參考答案：
（每一題一個完整的 Rust 程式）
``````

## 第 3 章

這一章還沒學到借用（第 4 章才教），所以「改到能編譯」的題目，合理的改法只有四種：調整程式的順序、用 `clone` 複製一份、換成 `Copy` 型別、用回傳值把所有權交還。每題的「批改重點」都會寫明指定哪一種；指定的不是 `clone` 時，用 `clone` 蒙混過關不算練到。

### 第 3 章第 1 集：為什麼需要所有權

狀態：不出題

不出題原因：
- 這集在說明 Rust 沒有 GC、靠所有權在編譯時期決定記憶體何時釋放，屬於背景觀念，沒有可以寫的程式。
- 硬出題會變成「GC 跟所有權差在哪」的問答。建議讀者繼續往下讀，第 4 集起有題目可練。

### 第 3 章第 2 集：stack 與 heap

狀態：不出題

不出題原因：
- 這集在分辨 stack 與 heap，資料放哪裡是編譯器的安排，程式裡看不出來，硬出題只會變成名詞解釋。
- 建議讀者繼續往下讀，第 4 集起有題目可練。

### 第 3 章第 3 集：所有權三條規則

狀態：不出題

不出題原因：
- 三條規則是觀念整理，沒有新語法；規則被違反（編譯不過）要從第 6 集的 move 才看得到。
- 建議讀者繼續往下讀，第 4 集起有題目可練。

### 第 3 章第 4 集：離開作用域就 drop

狀態：可用

練習目標：
- 看出變數的作用域，到它所在區塊的 `}` 為止。
- 會把變數建立在夠外層的地方，讓用到它的程式都在它的作用域裡。

題目：
1. 起始程式想在內層區塊和外層各印一次 `message`，但編譯不過。請改到能編譯，輸出要跟下方一樣，而且 `String::from` 只能出現一次。

執行範例：

起始程式（編譯不過）：

```rust,compile_fail=E0425
fn main() {
    {
        let message = String::from("哈囉");
        println!("內層：{message}");
    }
    println!("外層：{message}");
}
```

改好之後的輸出：

```text
內層：哈囉
外層：哈囉
```

批改重點：
- `String::from` 只有一個，`let message` 搬到外層（區塊的外面）。
- 要說得出為什麼原本會錯：`message` 在內層的 `}` 就離開作用域了，外層看不到。
- 保留內層區塊，或把區塊拿掉，只要輸出一樣都算對。

提示方向：
1. 錯誤訊息說 `cannot find value message in this scope`，也就是外層找不到 `message` 這個名字。
2. 外層和內層都要用的變數，應該建立在哪一層？

參考答案：

```rust
fn main() {
    let message = String::from("哈囉");
    {
        println!("內層：{message}");
    }
    println!("外層：{message}");
}
```

### 第 3 章第 5 集：move

狀態：不出題

不出題原因：
- 這集在觀察 `let s2 = s1;` 把所有權轉走，沒有新語法也沒有錯誤可改；真正能出的題目，是「move 之後又用到舊變數」，那是下一集的內容。
- 建議讀者繼續讀第 6 集，那裡有題目可練。

### 第 3 章第 6 集：move 之後不能再用（E0382）

狀態：可用

練習目標：
- 認得 E0382，找得到所有權是在哪一行被拿走。
- 不用 `clone`，靠調整順序或改變 `String` 建立的位置，讓程式能編譯。

題目：
1. 起始程式 1 編譯不過。請改到能編譯，輸出不變，不要用 `clone`（這一集還沒教）。
2. 起始程式 2 編譯不過。請改到能編譯，輸出不變，不要用 `clone`。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0382
fn main() {
    let first = String::from("Rust");
    let second = first;
    println!("第一個：{first}");
    println!("第二個：{second}");
}
```

題 1 改好之後的輸出：

```text
第一個：Rust
第二個：Rust
```

題 2 的起始程式（編譯不過）：

```rust,compile_fail=E0382
fn main() {
    let name = String::from("小明");
    for round in 1..=3 {
        let owner = name;
        println!("第 {round} 圈：{owner}");
    }
}
```

題 2 改好之後的輸出：

```text
第 1 圈：小明
第 2 圈：小明
第 3 圈：小明
```

批改重點：
- 題 1：把印 `first` 的那一行移到 `let second = first;` 之前，是最直接的改法；兩行都改印 `second` 也能編譯，同樣算對。重點是說得出 move 發生在 `let second = first;`。
- 題 2：把 `String::from` 搬進迴圈，讓每一圈各有自己的 `String`。要說得出原因：`let owner = name;` 在第一圈就把 `name` 拿走了，第二圈沒有東西可以 move。
- 兩題都不該出現 `clone`；用了 `clone` 能編譯，但沒有練到這一集要練的順序與位置。

提示方向：
1. 先看錯誤訊息裡的 `value moved here`，找出所有權在哪一行被拿走。
2. 題 1：move 之後 `first` 就不能用了。要用 `first` 的那一行，應該排在哪裡？
3. 題 2：`let owner = name;` 在迴圈裡會執行三次。每一圈要用的 `String`，應該在哪裡建立？

參考答案：

題 1：

```rust
fn main() {
    let first = String::from("Rust");
    println!("第一個：{first}");
    let second = first;
    println!("第二個：{second}");
}
```

題 2：

```rust
fn main() {
    for round in 1..=3 {
        let owner = String::from("小明");
        println!("第 {round} 圈：{owner}");
    }
}
```

### 第 3 章第 7 集：`clone`

狀態：可用

練習目標：
- 需要兩份獨立的資料時，會用 `.clone()` 明確複製。
- 看出 `clone` 之後，兩個變數各自擁有自己的資料，改其中一個不影響另一個。

題目：
1. 起始程式 1 編譯不過。請改到能編譯，輸出要跟下方一樣：`original` 保持原樣，`edited` 是加長之後的版本。
2. 起始程式 2 編譯不過。請改到能編譯，輸出要跟下方一樣：`base` 保持原樣，`a`、`b` 各自接上不同的字尾。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0382
fn main() {
    let original = String::from("hello");
    let mut edited = original;
    edited.push_str(" world");
    println!("{original}");
    println!("{edited}");
}
```

題 1 改好之後的輸出：

```text
hello
hello world
```

題 2 的起始程式（編譯不過）：

```rust,compile_fail=E0382
fn main() {
    let base = String::from("log");
    let mut a = base;
    let mut b = base;
    a.push_str("-a");
    b.push_str("-b");
    println!("{base} {a} {b}");
}
```

題 2 改好之後的輸出：

```text
log log-a log-b
```

批改重點：
- 這兩題本身就是要三份、兩份獨立的資料，所以 `clone` 是正解。
- 題 1：`let mut edited = original.clone();`，`push_str` 作用在複本上，`original` 才會維持 `hello`。
- 題 2：兩個變數都要從 `base.clone()` 來；`println!` 裡的 `base` 不用改。
- 如果讀者改成只印 `edited`，輸出就不一樣了，不算對。

提示方向：
1. `let mut edited = original;` 把所有權整個拿走了。如果兩邊都要有一份，其中一份要從哪裡來？
2. 有一個方法會在 heap 上另外複製一份資料，回傳新的 `String`。

參考答案：

題 1：

```rust
fn main() {
    let original = String::from("hello");
    let mut edited = original.clone();
    edited.push_str(" world");
    println!("{original}");
    println!("{edited}");
}
```

題 2：

```rust
fn main() {
    let base = String::from("log");
    let mut a = base.clone();
    let mut b = base.clone();
    a.push_str("-a");
    b.push_str("-b");
    println!("{base} {a} {b}");
}
```

### 第 3 章第 8 集：`Copy` 型別

狀態：可用

練習目標：
- 分辨哪些型別指定時是複製（`Copy`），哪些是 move。
- 知道 tuple 裡只要有 `String`，整個 tuple 就不是 `Copy`。

題目：
1. 起始程式 1 編譯不過。請不要用 `clone`，只改 `order` 那一行，讓程式能編譯，輸出要跟下方一樣。
2. 寫一個程式：建立陣列 `a`，裡面是 `10`、`20`、`30`；用 `let mut b = a;` 複製一份，把 `b` 的第一個元素改成 `99`，最後印出 `a` 和 `b`，確認 `a` 沒有被改到。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0382
fn main() {
    let order = (3, String::from("珍珠奶茶"));
    let backup = order;
    println!("{} {}", order.0, backup.0);
    println!("{} {}", order.1, backup.1);
}
```

題 1 改好之後的輸出：

```text
3 3
珍珠奶茶 珍珠奶茶
```

題 2 的輸出：

```text
[10, 20, 30] [99, 20, 30]
```

批改重點：
- 題 1：改的是型別，不是加 `clone`：把 `String::from("珍珠奶茶")` 換成字串字面值 `"珍珠奶茶"`，tuple 的每個元素都是 `Copy`，整個 tuple 就是 `Copy`。要說得出是 `String` 讓整個 tuple 不能 `Copy`。
- 題 2：`let mut b = a;` 之後 `a` 還能用，`b` 改了 `a` 不變，因為 `[i32; 3]` 是 `Copy`。

提示方向：
1. 錯誤訊息說 `(i32, String)` 沒有實作 `Copy`。是哪個元素讓整個 tuple 不能複製？
2. 第 2 章第 15 集看過，寫死的文字不一定要用 `String::from`。
3. 題 2：`b` 要改值，所以要用 `let mut`。

參考答案：

題 1：

```rust
fn main() {
    let order = (3, "珍珠奶茶");
    let backup = order;
    println!("{} {}", order.0, backup.0);
    println!("{} {}", order.1, backup.1);
}
```

題 2：

```rust
fn main() {
    let a = [10, 20, 30];
    let mut b = a;
    b[0] = 99;
    println!("{a:?} {b:?}");
}
```

### 第 3 章第 9 集：傳參數也會 move

狀態：可用

練習目標：
- 認出把 `String` 傳進函式會 move，呼叫之後原本的變數不能再用。
- 分清楚同一個呼叫裡，`String` 參數會 move、整數參數只是複製。

題目：
1. 起始程式 1 編譯不過。請改到能編譯，輸出要跟下方一樣。
2. 起始程式 2 編譯不過。請改到能編譯，輸出要跟下方一樣；只改一個地方。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0382
fn shout(text: String) {
    println!("{text}！");
}

fn main() {
    let word = String::from("加油");
    shout(word);
    shout(word);
}
```

題 1 改好之後的輸出：

```text
加油！
加油！
```

題 2 的起始程式（編譯不過）：

```rust,compile_fail=E0382
fn describe(name: String, age: i32) {
    println!("{name}，{age} 歲");
}

fn main() {
    let name = String::from("小明");
    let age = 20;
    describe(name, age);
    println!("剛剛介紹的是 {name}，{age} 歲");
}
```

題 2 改好之後的輸出：

```text
小明，20 歲
剛剛介紹的是 小明，20 歲
```

批改重點：
- 這一章還沒學借用，所以這兩題用 `clone` 是合理的解法（也可以每次都重新 `String::from`）。
- 題 1：第一次呼叫傳 `word.clone()` 就夠了，最後一次呼叫可以直接把 `word` 交出去；兩次都 `clone` 也能編譯，不扣分，但可以提一下最後一次不需要。
- 題 2：只有 `name` 要 `clone`，`age` 是 `i32`，傳進去本來就是複製。如果讀者連 `age` 也寫 `.clone()`，能編譯，但要提醒他整數不需要。

提示方向：
1. 第一次呼叫 `shout(word)` 之後，`word` 的所有權在誰手上？
2. 題 2：兩個參數裡，哪一個被 move，哪一個只是複製？
3. 想在呼叫之後繼續用原本的變數，要傳什麼進去？

參考答案：

題 1：

```rust
fn shout(text: String) {
    println!("{text}！");
}

fn main() {
    let word = String::from("加油");
    shout(word.clone());
    shout(word);
}
```

題 2：

```rust
fn describe(name: String, age: i32) {
    println!("{name}，{age} 歲");
}

fn main() {
    let name = String::from("小明");
    let age = 20;
    describe(name.clone(), age);
    println!("剛剛介紹的是 {name}，{age} 歲");
}
```

### 第 3 章第 10 集：回傳值交還所有權

狀態：可用

練習目標：
- 會把函式改成回傳 `String`，讓所有權交還給呼叫端，並用 `let name = f(name);` 接住。
- 會寫一個在函式裡建立 `String`、最後回傳它的函式。

題目：
1. 起始程式 1 編譯不過。請不要用 `clone`，改到能編譯，輸出要跟下方一樣。
2. 寫一個函式 `make_line(n: i32) -> String`：用迴圈把 `"=-"` 接在一個新的 `String` 後面 `n` 次，最後回傳它。`main` 裡呼叫 `make_line(5)` 並印出結果。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0382
fn add_title(mut name: String) {
    name.push_str(" 老師");
}

fn main() {
    let name = String::from("王");
    add_title(name);
    println!("{name}");
}
```

題 1 改好之後的輸出：

```text
王 老師
```

題 2 的輸出：

```text
=-=-=-=-=-
```

批改重點：
- 題 1：`add_title` 的回傳型別改成 `String`，最後一行回傳 `name`；`main` 用 `let name = add_title(name);` 接住。用 `clone` 不算對：函式改的會是複本，`main` 的 `name` 不會變成 `王 老師`。
- 題 2：函式裡自己 `let mut` 一個 `String::new()`，迴圈 `push_str`，最後一行（沒有分號）回傳；`main` 用 `let line = make_line(5);` 接住。迴圈裡用不到的計數變數，名字要用 `_` 開頭（例如 `_i`），否則會有未使用變數的警告。

提示方向：
1. 題 1：函式改了 `name`，卻沒有把它還給 `main`。要怎麼讓所有權回到呼叫的地方？
2. 題 1：函式的 `->` 後面要寫什麼型別？最後一行要怎麼寫，才會成為回傳值？
3. 題 2：回傳的 `String` 是在函式裡建立的，最後一行直接寫變數名稱，不加分號。

參考答案：

題 1：

```rust
fn add_title(mut name: String) -> String {
    name.push_str(" 老師");
    name
}

fn main() {
    let name = String::from("王");
    let name = add_title(name);
    println!("{name}");
}
```

題 2：

```rust
fn make_line(n: i32) -> String {
    let mut line = String::new();
    for _i in 0..n {
        line.push_str("=-");
    }
    line
}

fn main() {
    let line = make_line(5);
    println!("{line}");
}
```

### 第 3 章第 11 集：「交出去再拿回來」很麻煩

狀態：可用

練習目標：
- 會用 tuple 回傳，把傳進函式的多個 `String` 連同結果一起還回來，並在呼叫端解構接住。

題目：
1. 起始程式編譯不過。請不要用 `clone`，改成讓函式把兩個 `String` 和結果一起回傳，輸出要跟下方一樣。

執行範例：

起始程式（編譯不過）：

```rust,compile_fail=E0382
fn longer_len(a: String, b: String) -> usize {
    if a.len() > b.len() { a.len() } else { b.len() }
}

fn main() {
    let a = String::from("哈囉");
    let b = String::from("Rust 世界");
    let n = longer_len(a, b);
    println!("{a} 和 {b} 比較，較長的有 {n} 個位元組");
}
```

改好之後的輸出：

```text
哈囉 和 Rust 世界 比較，較長的有 11 個位元組
```

批改重點：
- 回傳型別改成 `(String, String, usize)`，函式裡先算出長度，最後回傳 `(a, b, n)`。
- `main` 用 `let (a, b, n) = longer_len(a, b);` 接住，用 shadowing 沿用同樣的名字。
- 不能用 `clone`：這題要練的是把所有權還回來。寫完可以請讀者感覺一下，兩個參數就這麼長，再多一個會怎樣；第 4 章的借用就是為了解決這件事。

提示方向：
1. 函式只是想讀兩個字串，卻把它們都拿走了。要怎麼讓它們回到 `main`？
2. 一次要回傳三個東西，第 2 章用過的 tuple 可以幫忙。
3. 呼叫端要用同樣形狀的 `let (...) = ...` 把三個值接住。

參考答案：

```rust
fn longer_len(a: String, b: String) -> (String, String, usize) {
    let n = if a.len() > b.len() { a.len() } else { b.len() };
    (a, b, n)
}

fn main() {
    let a = String::from("哈囉");
    let b = String::from("Rust 世界");
    let (a, b, n) = longer_len(a, b);
    println!("{a} 和 {b} 比較，較長的有 {n} 個位元組");
}
```

## 第 4 章

（第 4 章寫完後補上。）
