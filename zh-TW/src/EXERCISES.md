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

這一章開始有借用了，所以「改到能編譯」的題目，能用的改法變多，但這些題要練的是借用本身，不是讓程式編譯成功就好。用 `.clone()` 複製一份、或是退回第 3 章的做法（用回傳值或 tuple 把所有權還回來），雖然能編譯，卻避開了借用，**不算**練到，除非題目明講要 `clone`。每題的「批改重點」會說明這題要練哪一種寫法。

這一章的題目一律不用 `struct`、`Vec`、`match`；會回傳參照的函式，只有一個參照參數。

### 第 4 章第 1 集：參照 `&`

狀態：不出題

不出題原因：
- 這集在認識 `&` 的寫法、`&String` 型別，以及「參照不擁有資料」；單看 `let r = &s;` 沒有可以改的錯誤，硬出題只會變成「照抄一個 `&`」。
- 「把函式改成借用」從第 2 集開始有題目可練，建議讀者繼續往下讀。

### 第 4 章第 2 集：借用不轉移所有權

狀態：可用

練習目標：
- 把「拿走所有權、再還回來」的函式，改成只借用：參數改成 `&String`，呼叫端傳 `&`。
- 看出借用之後，呼叫端不必再接回所有權，原本的變數可以一直用。

題目：
1. 起始程式 1 能編譯，但要靠 tuple 把兩個 `String` 還回來。請把 `longer_len` 改成借用，回傳值只剩長度，輸出要跟下方一樣。
2. 起始程式 2 編譯不過。請用借用改到能編譯，輸出要跟下方一樣。

執行範例：

題 1 的起始程式：

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

題 1 改好之後的輸出：

```text
哈囉 和 Rust 世界 比較，較長的有 11 個位元組
```

題 2 的起始程式（編譯不過）：

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

題 2 改好之後的輸出：

```text
加油！
加油！
```

批改重點：
- 題 1：參數改成 `&String`，函式回傳型別只剩 `usize`；`main` 呼叫 `longer_len(&a, &b)`，用 `let n = ...` 接住就好，不再需要 tuple 和重新接回 `a`、`b`。仍然回傳 tuple、或在呼叫時 `clone`，都沒有練到借用。
- 題 2：參數改成 `&String`，兩次呼叫都寫 `shout(&word)`。第 3 章這題是用 `clone` 解的，這一章的重點是不用複製、只借用；用 `clone` 不算。
- 兩題都要說得出為什麼呼叫之後 `a`、`word` 還能用：傳進去的是參照，所有權沒有搬家。

提示方向：
1. 函式只是想讀這些 `String`，真的需要把所有權拿走嗎？參數型別換成什麼，就變成「借來看」？
2. 參數寫了 `&String`，呼叫的地方也要跟著寫 `&`。
3. 題 1：函式不再把 `a`、`b` 交還，回傳值還需要放什麼？

參考答案：

題 1：

```rust
fn longer_len(a: &String, b: &String) -> usize {
    if a.len() > b.len() { a.len() } else { b.len() }
}

fn main() {
    let a = String::from("哈囉");
    let b = String::from("Rust 世界");
    let n = longer_len(&a, &b);
    println!("{a} 和 {b} 比較，較長的有 {n} 個位元組");
}
```

題 2：

```rust
fn shout(text: &String) {
    println!("{text}！");
}

fn main() {
    let word = String::from("加油");
    shout(&word);
    shout(&word);
}
```

### 第 4 章第 3 集：可變參照 `&mut`

狀態：可用

練習目標：
- 用 `&mut` 讓函式修改傳進來的 `String`，並同時寫對三個地方：變數用 `let mut`、參數型別 `&mut String`、呼叫端 `&mut`。
- 分辨 `&`（只能讀）和 `&mut`（可以改）。

題目：
1. 起始程式編譯不過。請改到能編譯，輸出要跟下方一樣。
2. 寫一個函式 `shout(s: &mut String)`，在 `String` 的尾端加上 `！！！`。`main` 裡建立 `String::from("加油")`，呼叫 `shout` 兩次，最後印出結果。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0596
fn add_suffix(text: &String) {
    text.push_str("，謝謝");
}

fn main() {
    let mut text = String::from("你好");
    add_suffix(&text);
    println!("{text}");
}
```

題 1 改好之後的輸出：

```text
你好，謝謝
```

題 2 的輸出：

```text
加油！！！！！！
```

批改重點：
- 題 1：函式參數 `&String` 改成 `&mut String`，修好之後會冒出第二個錯誤，呼叫端也要改成 `add_suffix(&mut text)`（`let mut text` 本來就有）。要說得出：`&` 只能讀，要改就得借出 `&mut`。用回傳 `String` 的做法，或 `clone` 一份來改，都沒有練到 `&mut`。
- 題 2：函式簽名是 `&mut String`，裡面 `s.push_str("！！！")`；`main` 的 `String` 要用 `let mut`，兩次呼叫都寫 `shout(&mut word)`。函式不需要回傳值。
- 題 2 的 `push_str` 參數至少要兩個字元，所以固定寫 `！！！`。

提示方向：
1. 第一個錯誤訊息說 `text` 是 `&` 參照，不能當成可變的來借。`help` 建議把型別改成什麼？
2. 改了函式簽名之後，呼叫端傳 `&text` 就不對了，要傳哪一種參照？
3. 題 2：`let mut`、`&mut String`、`&mut word`，三個地方是不是都寫了？

參考答案：

題 1：

```rust
fn add_suffix(text: &mut String) {
    text.push_str("，謝謝");
}

fn main() {
    let mut text = String::from("你好");
    add_suffix(&mut text);
    println!("{text}");
}
```

題 2：

```rust
fn shout(s: &mut String) {
    s.push_str("！！！");
}

fn main() {
    let mut word = String::from("加油");
    shout(&mut word);
    shout(&mut word);
    println!("{word}");
}
```

### 第 4 章第 4 集：借用規則：多個 `&` 或一個 `&mut`

狀態：可用

練習目標：
- 認得 E0499（同時兩個 `&mut`）、E0502（`&mut` 還活著時，擁有者又去讀）。
- 不用 `clone`，靠調整順序或減少參照的數量，讓借用不重疊。

題目：
1. 起始程式 1 編譯不過。請改到能編譯，輸出要跟下方一樣，不要用 `clone`。
2. 起始程式 2 編譯不過。請改到能編譯，輸出要跟下方一樣，不要用 `clone`。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0499
fn main() {
    let mut s = String::from("Rust");
    let a = &mut s;
    let b = &mut s;
    a.push_str(" 很");
    b.push_str("好玩");
    println!("{s}");
}
```

題 1 改好之後的輸出：

```text
Rust 很好玩
```

題 2 的起始程式（編譯不過）：

```rust,compile_fail=E0502
fn main() {
    let mut cart = String::from("牛奶");
    let editor = &mut cart;
    println!("改之前：{cart}");
    editor.push_str("、雞蛋");
    println!("改之後：{cart}");
}
```

題 2 改好之後的輸出：

```text
改之前：牛奶
改之後：牛奶、雞蛋
```

批改重點：
- 題 1：兩個 `&mut` 不能同時活著。改法有兩種都算對：讓 `a` 在 `let b = &mut s;` 之前先用完（兩行 `push_str` 各接在各自的 `let` 後面），或是只留一個 `&mut`、用它呼叫兩次 `push_str`。要說得出 E0499 的意思：同一時間只能有一個 `&mut`。
- 題 2：把「改之前」那行 `println!` 移到 `let editor = &mut cart;` 之前。要說得出原因：`editor` 這個 `&mut` 還要用，在它活著的期間，擁有者 `cart` 連讀都不行。
- 兩題都不該出現 `clone`；題 2 用 `clone` 的話，會得到一份跟 `cart` 無關的複本，沒有練到借用規則。

提示方向：
1. 錯誤訊息裡 `first mutable borrow occurs here` 和 `later used here` 各指著哪一行？兩個借用為什麼重疊？
2. 題 1：第一個 `&mut` 最後一次用到是哪一行？第二個借用能不能等它用完才開始？
3. 題 2：`editor` 在哪一行之後才開始用？想印 `cart`，應該排在 `editor` 建立之前還是之後？

參考答案：

題 1：

```rust
fn main() {
    let mut s = String::from("Rust");
    let a = &mut s;
    a.push_str(" 很");
    let b = &mut s;
    b.push_str("好玩");
    println!("{s}");
}
```

題 2：

```rust
fn main() {
    let mut cart = String::from("牛奶");
    println!("改之前：{cart}");
    let editor = &mut cart;
    editor.push_str("、雞蛋");
    println!("改之後：{cart}");
}
```

### 第 4 章第 5 集：參照用完就結束（NLL）

狀態：可用

練習目標：
- 看懂借用在「最後一次使用」就結束，會用調整順序讓 `&` 先用完、再去改。

題目：
1. 起始程式編譯不過。請只調整程式的順序（不要新增變數、不要用 `clone`），讓程式能編譯，輸出要跟下方一樣。

執行範例：

起始程式（編譯不過）：

```rust,compile_fail=E0502
fn main() {
    let mut s = String::from("Rust");
    let r = &s;
    s.push_str(" 書");
    println!("借來看：{r}");
    println!("改完之後：{s}");
}
```

改好之後的輸出：

```text
借來看：Rust
改完之後：Rust 書
```

批改重點：
- 把 `println!("借來看：{r}");` 移到 `s.push_str(" 書");` 之前。要說得出原因：`r` 最後一次使用的那一行，就是借用結束的地方；它必須排在 `push_str` 之前。
- 把 `let r = &s;` 改成 `let r = s.clone();` 也能編譯，但輸出裡 `r` 是複本，沒有練到 NLL，不算對。
- 不用把 `println!("改完之後：{s}");` 移動，它在 `r` 用完之後，本來就沒有問題。

提示方向：
1. 錯誤訊息的 `immutable borrow later used here`，指的是哪一行？
2. `r` 的借用，到哪一行就不需要了？`push_str` 應該排在那一行的前面還是後面？

參考答案：

```rust
fn main() {
    let mut s = String::from("Rust");
    let r = &s;
    println!("借來看：{r}");
    s.push_str(" 書");
    println!("改完之後：{s}");
}
```

### 第 4 章第 6 集：懸置參照

狀態：可用

練習目標：
- 認得 E0106（回傳區域變數的參照）和 E0597（參照活得比被借的值久）。
- 知道改法：回傳擁有的值，或把值建立在夠外層的地方。

題目：
1. 起始程式 1 編譯不過。請改到能編譯，輸出要跟下方一樣。不要加 `'static`（那不是這題要的）。
2. 起始程式 2 編譯不過。請改到能編譯，輸出要跟下方一樣，`String::from` 只能出現一次。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0106
fn make_greeting() -> &String {
    let mut s = String::from("哈囉");
    s.push_str("，世界");
    &s
}

fn main() {
    let greeting = make_greeting();
    println!("{greeting}");
}
```

題 1 改好之後的輸出：

```text
哈囉，世界
```

題 2 的起始程式（編譯不過）：

```rust,compile_fail=E0597
fn main() {
    let view = {
        let name = String::from("小明");
        &name
    };
    println!("{view}");
}
```

題 2 改好之後的輸出：

```text
小明
```

批改重點：
- 題 1：回傳型別改成 `String`，最後一行回傳 `s`（沒有 `&`、沒有分號），所有權交給呼叫端。要說得出原因：`s` 是函式裡的區域變數，`}` 就 drop 了，不能把指著它的參照交出去。加 `'static` 對區域變數仍然會失敗，不算改好。
- 題 2：把 `let name = String::from("小明");` 搬到區塊的外面，讓 `name` 活得比 `view` 久；區塊本身可以拿掉。要說得出：`name` 在區塊的 `}` 就被 drop，`view` 卻還要用，所以不行。
- 兩題都不要靠 `clone` 繞過去。

提示方向：
1. 題 1：函式裡建立的 `String`，要怎麼交給呼叫端？第 3 章第 10 集用過的辦法是什麼？
2. 題 2：`name` 在哪一個 `}` 被 drop？`view` 最後一次用到是哪一行？
3. 題 2：`name` 應該建立在哪一層，才會比 `view` 活得久？

參考答案：

題 1：

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

題 2：

```rust
fn main() {
    let name = String::from("小明");
    let view = &name;
    println!("{view}");
}
```

### 第 4 章第 7 集：解參照 `*`

狀態：可用

練習目標：
- 知道對 `&mut` 的數字做運算、對參照的值做比較，要先用 `*` 取出指向的值。

題目：
1. 起始程式 1 編譯不過。請改到能編譯，輸出要跟下方一樣。
2. 起始程式 2 編譯不過。請改到能編譯，輸出要跟下方一樣。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0368
fn add_ten(n: &mut i32) {
    n += 10;
}

fn main() {
    let mut score = 5;
    add_ten(&mut score);
    println!("{score}");
}
```

題 1 改好之後的輸出：

```text
15
```

題 2 的起始程式（編譯不過）：

```rust,compile_fail=E0308
fn is_big(n: &i32) -> bool {
    n > 100
}

fn main() {
    let x = 150;
    let y = 30;
    println!("{} {}", is_big(&x), is_big(&y));
}
```

題 2 改好之後的輸出：

```text
true false
```

批改重點：
- 題 1：`*n += 10;`。要說得出 `n` 是參照，不是數字，要改它指向的值，得先 `*`。改成回傳新的值不算，這題要練的是 `&mut` 加 `*`。
- 題 2：`*n > 100`。要說得出 `&i32` 跟 `i32` 是不同的型別，要比較就得先解參照；函式簽名維持 `&i32`，不要改成 `i32`。
- 用 `.clone()` 或改成回傳新的值蒙混過關，不算練到這一集的借用。

提示方向：
1. 錯誤訊息的 `help` 建議在哪個變數前面補上什麼符號？
2. `n` 的型別是 `&mut i32`（或 `&i32`），而你要運算、比較的是它指向的 `i32`。從參照拿出值的符號是哪一個？

參考答案：

題 1：

```rust
fn add_ten(n: &mut i32) {
    *n += 10;
}

fn main() {
    let mut score = 5;
    add_ten(&mut score);
    println!("{score}");
}
```

題 2：

```rust
fn is_big(n: &i32) -> bool {
    *n > 100
}

fn main() {
    let x = 150;
    let y = 30;
    println!("{} {}", is_big(&x), is_big(&y));
}
```

### 第 4 章第 8 集：字串 slice `&str`

狀態：可用

練習目標：
- 會用 `&s[起..迄]` 取出字串的一段，並寫出回傳 `&str` 的函式。
- 知道 slice 還要用的時候，不能改原本的 `String`。

題目：
1. 寫一個函式 `inner(s: &String) -> &str`，去掉第一個和最後一個位元組，回傳中間的部分（只考慮 ASCII 字元）。`main` 裡用 `String::from("[hello]")` 呼叫，印出結果。
2. 起始程式編譯不過。請只調整程式的順序，讓程式能編譯，輸出要跟下方一樣，不要用 `clone`。

執行範例：

題 1 的輸出：

```text
hello
```

題 2 的起始程式（編譯不過）：

```rust,compile_fail=E0502
fn main() {
    let mut title = String::from("hello world");
    let first = &title[0..5];
    title.push_str("!!");
    println!("第一個字：{first}");
    println!("全部：{title}");
}
```

題 2 改好之後的輸出：

```text
第一個字：hello
全部：hello world!!
```

批改重點：
- 題 1：回傳型別 `&str`，函式裡是 `&s[1..s.len() - 1]`；只有一個參照參數 `s`，不需要任何額外標註。`main` 要傳 `&s`。不能先建立新的 `String` 再回傳參照（那是第 6 集的懸置參照）。
- 題 2：把「第一個字」那行 `println!` 移到 `title.push_str("!!");` 之前。要說得出：`first` 是對 `title` 的借用，還要用的時候，不能去改 `title`。`first` 改成 `title.clone()` 的複本，沒有練到 slice 是借用。

提示方向：
1. 題 1：想取中間的一段，範圍的起點和終點各是多少？終點可以用 `s.len()` 減掉什麼來算？
2. 題 2：`first` 最後一次用在哪一行？`push_str` 應該排在它前面還是後面？

參考答案：

題 1：

```rust
fn inner(s: &String) -> &str {
    &s[1..s.len() - 1]
}

fn main() {
    let s = String::from("[hello]");
    println!("{}", inner(&s));
}
```

題 2：

```rust
fn main() {
    let mut title = String::from("hello world");
    let first = &title[0..5];
    println!("第一個字：{first}");
    title.push_str("!!");
    println!("全部：{title}");
}
```

### 第 4 章第 9 集：字串字面值就是 `&str`

狀態：可用

練習目標：
- 知道字串字面值（`&str`）不能增長，要增長就得有自己擁有的 `String`。

題目：
1. 起始程式編譯不過。請改到能編譯，輸出要跟下方一樣。

執行範例：

起始程式（編譯不過）：

```rust,compile_fail=E0599
fn main() {
    let mut note = "借用";
    note.push_str("與切片");
    println!("{note}");
}
```

改好之後的輸出：

```text
借用與切片
```

批改重點：
- 把 `"借用"` 換成 `String::from("借用")`，其他不用動；`let mut` 要保留，因為 `push_str` 會改動這個 `String`。要說得出：字串字面值是 `&str`，沒有 `push_str`，要能增長就得用 `String`。
- 這題沒有借用可以閃躲，也不該出現 `clone`。

提示方向：
1. 錯誤訊息說 `&str` 上找不到 `push_str`。字面值是哪一種型別？
2. 哪一種型別有 `push_str`，而且是自己擁有文字的？第 2 章第 15 集用什麼寫法建立它？

參考答案：

```rust
fn main() {
    let mut note = String::from("借用");
    note.push_str("與切片");
    println!("{note}");
}
```

### 第 4 章第 10 集：參數用 `&str` 而不是 `&String`

狀態：可用

練習目標：
- 把只需要讀的字串參數從 `&String` 改成 `&str`，讓字面值、`String` 都能傳進去。
- 會寫參數是 `&str` 的函式，並用不同來源的字串呼叫它。

題目：
1. 起始程式編譯不過。請把函式改成借用 `&str`，讓 `main` 兩種呼叫都能編譯，輸出要跟下方一樣。
2. 寫一個函式 `is_long(text: &str) -> bool`，字串超過 6 個位元組就回傳 `true`。`main` 裡用 `String::from("哈囉世界")` 和字面值 `"Rust"` 各呼叫一次，印出兩個結果。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0308
fn count_bytes(s: &String) -> usize {
    s.len()
}

fn main() {
    let owned = String::from("哈囉");
    println!("{}", count_bytes(&owned));
    println!("{}", count_bytes("Rust"));
}
```

題 1 改好之後的輸出：

```text
6
4
```

題 2 的輸出：

```text
true
false
```

批改重點：
- 題 1：只改函式的參數型別，`&String` 換成 `&str`；`main` 一行都不用動。不要改成 `String`（那會拿走所有權）或在呼叫端 `String::from("Rust")`（那是繞過去，不是改函式）。要說得出：`&str` 能收字面值、`&String` 和 slice，`&String` 只能收 `String` 的參照。
- 題 2：參數型別是 `&str`，`main` 裡傳 `&owned`（`&String` 會自動轉成 `&str`）和 `"Rust"`；不需要 `clone`。

提示方向：
1. 錯誤訊息說要的是 `&String`、給的是 `&str`。哪一個型別比較通用，連字面值都收？
2. 題 1：函式裡面 `s.len()` 要不要改？

參考答案：

題 1：

```rust
fn count_bytes(s: &str) -> usize {
    s.len()
}

fn main() {
    let owned = String::from("哈囉");
    println!("{}", count_bytes(&owned));
    println!("{}", count_bytes("Rust"));
}
```

題 2：

```rust
fn is_long(text: &str) -> bool {
    text.len() > 6
}

fn main() {
    let owned = String::from("哈囉世界");
    println!("{}", is_long(&owned));
    println!("{}", is_long("Rust"));
}
```

### 第 4 章第 11 集：陣列 slice `&[T]`

狀態：可用

練習目標：
- 讓函式用 `&[T]` 接收任意長度的陣列，也能只傳其中一段。

題目：
1. 起始程式編譯不過。請改到能編譯，輸出要跟下方一樣。
2. 寫一個函式 `average(nums: &[i32]) -> i32`，回傳平均值（整數除法，直接捨去小數）。`main` 裡用 `[80, 90, 70, 60]` 呼叫一次，再只傳前兩個元素呼叫一次，印出兩個結果。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0308
fn largest(nums: &[i32; 3]) -> i32 {
    let mut best = nums[0];
    for i in 1..3 {
        if nums[i] > best {
            best = nums[i];
        }
    }
    best
}

fn main() {
    let a = [3, 9, 4];
    let b = [7, 2, 8, 5, 1];
    println!("{}", largest(&a));
    println!("{}", largest(&b));
}
```

題 1 改好之後的輸出：

```text
9
8
```

題 2 的輸出：

```text
75
85
```

批改重點：
- 題 1：參數改成 `&[i32]`，迴圈的終點也要從寫死的 `3` 改成 `nums.len()`，否則 `b` 的後兩個元素會被漏掉。要說得出：`[i32; 3]` 的長度是型別的一部分，`&[i32]` 才什麼長度都收。
- 題 2：參數 `&[i32]`，用 `nums.len()` 取長度，總和除以 `nums.len() as i32`；呼叫時傳 `&scores`、`&scores[0..2]`。
- 用 `.clone()` 蒙混過關不算練到借用（兩題都一樣）。

提示方向：
1. 錯誤訊息說 `expected an array with a size of 3, found one with a size of 5`，意思是要的是長度 3 的陣列，給的是長度 5。哪一種參數型別，不把長度寫進去？
2. 題 1：迴圈的終點如果還寫 `3`，換成長度是 5 的陣列會怎樣？函式裡可以用什麼拿到長度？
3. 題 2：`len()` 回傳的是 `usize`，要跟 `i32` 的總和相除，需要怎麼轉換？（第 1 章第 17 集）

參考答案：

題 1：

```rust
fn largest(nums: &[i32]) -> i32 {
    let mut best = nums[0];
    for i in 1..nums.len() {
        if nums[i] > best {
            best = nums[i];
        }
    }
    best
}

fn main() {
    let a = [3, 9, 4];
    let b = [7, 2, 8, 5, 1];
    println!("{}", largest(&a));
    println!("{}", largest(&b));
}
```

題 2：

```rust
fn average(nums: &[i32]) -> i32 {
    let mut total = 0;
    for i in 0..nums.len() {
        total += nums[i];
    }
    total / nums.len() as i32
}

fn main() {
    let scores = [80, 90, 70, 60];
    println!("{}", average(&scores));
    println!("{}", average(&scores[0..2]));
}
```

### 第 4 章第 12 集：`&mut [T]`

狀態：可用

練習目標：
- 用 `&mut [T]` 讓函式修改陣列的元素（整個陣列或其中一段）。
- 知道參數是 `&[T]` 就改不了，要改成 `&mut [T]`，呼叫端也要跟著改。

題目：
1. 起始程式編譯不過。請改到能編譯，輸出要跟下方一樣。
2. 寫一個函式 `add_to_all(nums: &mut [i32], delta: i32)`，把每個元素都加上 `delta`。`main` 裡建立 `[1, 2, 3, 4, 5]`，先對整個陣列加 `10`，再只對索引 3 起的部分加 `100`，最後印出陣列。

執行範例：

題 1 的起始程式（編譯不過）：

```rust,compile_fail=E0594
fn reset_negatives(nums: &[i32]) {
    for i in 0..nums.len() {
        if nums[i] < 0 {
            nums[i] = 0;
        }
    }
}

fn main() {
    let mut data = [5, -3, 8, -1];
    reset_negatives(&data);
    println!("{data:?}");
}
```

題 1 改好之後的輸出：

```text
[5, 0, 8, 0]
```

題 2 的輸出：

```text
[11, 12, 13, 114, 115]
```

批改重點：
- 題 1：函式參數改成 `&mut [i32]`，呼叫端改成 `reset_negatives(&mut data)`（`let mut data` 本來就有）。要說得出：`&[T]` 只能讀，要改元素就得借出 `&mut [T]`。改成回傳新陣列不算，這題要練的是 `&mut [T]`。
- 題 2：函式簽名 `&mut [i32]`，裡面用索引迴圈 `nums[i] += delta`；呼叫時 `add_to_all(&mut a, 10)` 和 `add_to_all(&mut a[3..], 100)`。要說得出：兩次呼叫是依序進行，前一個 `&mut` 用完才借下一個，所以不會有 E0499。
- 用 `.clone()` 或改成回傳新的值蒙混過關，不算練到這一集的借用。

提示方向：
1. 第一個錯誤訊息說 `nums[_]` 在 `&` 參照後面，不能指定新值。`help` 建議把型別改成什麼？
2. 改了函式簽名之後，呼叫端的 `&data` 還對嗎？
3. 題 2：只改一段，要借出的是整個陣列，還是它的一段？範圍的寫法在第 11 集看過。

參考答案：

題 1：

```rust
fn reset_negatives(nums: &mut [i32]) {
    for i in 0..nums.len() {
        if nums[i] < 0 {
            nums[i] = 0;
        }
    }
}

fn main() {
    let mut data = [5, -3, 8, -1];
    reset_negatives(&mut data);
    println!("{data:?}");
}
```

題 2：

```rust
fn add_to_all(nums: &mut [i32], delta: i32) {
    for i in 0..nums.len() {
        nums[i] += delta;
    }
}

fn main() {
    let mut a = [1, 2, 3, 4, 5];
    add_to_all(&mut a, 10);
    add_to_all(&mut a[3..], 100);
    println!("{a:?}");
}
```
