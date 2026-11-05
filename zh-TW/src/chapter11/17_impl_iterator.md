# 自己實作 `Iterator`

## 本集目標

替自己的型別實作 `Iterator`：只要寫 `type Item` 和 `next`，就能用 `for`，也能接上 `filter`、`sum`、`zip` 這一整排方法。

## 正文

### 只寫 `next`

第 8 章第 17 集的 `Tickets` 取餐號碼機，取號的方法叫 `take`。把名字換成標準函式庫的 `next`、換成實作 `Iterator`，它就成了迭代器：

```rust
struct Tickets {
    current: u32,
    last: u32,
}

impl Iterator for Tickets {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current > self.last {
            None
        } else {
            let number = self.current;
            self.current += 1;
            Some(number)
        }
    }
}

fn main() {
    let tickets = Tickets {
        current: 1,
        last: 3,
    };
    for t in tickets {
        println!("取餐號碼 {t}");
    }
}
```

執行結果：

```text
取餐號碼 1
取餐號碼 2
取餐號碼 3
```

- `type Item = u32;` 指定每次取出的型別；`next` 的回傳型別寫實際的 `Option<u32>`。跟第 8 章第 17 集一樣，`impl` 裡不寫 `Self::Item`。
- `next` 拿 `&mut self`，把「走到哪裡」記在欄位 `current`：還沒超過 `last` 就取出目前的號碼、再往前一格，超過了就回傳 `None`。
- 這樣就能用 `for`：每個迭代器自己也是 `IntoIterator`（第 8 集），不用另外實作。

忘了寫 `type Item`，或忘了寫 `next`，都是 E0046（`not all trait items implemented`，第 8 章第 1 集看過），訊息會告訴你缺的是哪一個。

### 其餘的方法免費送

`Iterator` 要求的只有 `Item` 和 `next` 兩樣，其他幾十個方法，包括這一章學過的 `map`、`filter`、`sum`、`zip`，都是 trait 的預設方法（第 8 章第 3 集），寫好 `next` 就全部自動擁有：

```rust
struct Tickets {
    current: u32,
    last: u32,
}

impl Iterator for Tickets {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current > self.last {
            None
        } else {
            let number = self.current;
            self.current += 1;
            Some(number)
        }
    }
}

fn main() {
    let odd_total: u32 = Tickets {
        current: 1,
        last: 10,
    }
    .filter(|t| t % 2 == 1)
    .sum();
    println!("奇數號碼共 {odd_total}");

    let names = ["小明", "小華", "小美"];
    let queue = Tickets {
        current: 7,
        last: 99,
    };
    for (name, ticket) in names.iter().zip(queue) {
        println!("{name} 領到 {ticket} 號");
    }
}
```

執行結果：

```text
奇數號碼共 25
小明 領到 7 號
小華 領到 8 號
小美 領到 9 號
```

第二段的 `Tickets` 可以一直領到 99 號，但 `zip` 在 `names` 走完就停了，所以只取到三個。自製迭代器與標準函式庫迭代器地位相同，可以混著串。

### 不定義型別，也能回傳迭代器

多數時候不需要為了回傳迭代器特地寫 struct。第 8 章第 7 集說過，寫不出名字的型別可以用 `impl Trait` 回傳；迭代器串接出來的型別又長又寫不出名字，剛好用得上。`Iterator<Item = u32>` 的寫法跟第 8 章第 17 集的 `Dispenser<Item = u32>` 一樣：

```rust
fn odd_tickets(last: u32) -> impl Iterator<Item = u32> {
    (1..=last).filter(|t| t % 2 == 1)
}

fn main() {
    for t in odd_tickets(7) {
        println!("奇數號碼 {t}");
    }
}
```

執行結果：

```text
奇數號碼 1
奇數號碼 3
奇數號碼 5
奇數號碼 7
```

回傳的是還沒走的迭代器：呼叫的人想怎麼接、怎麼 `for`，都由他決定。

## 重點整理

- 實作 `Iterator` 只需要：`type Item = 型別;` 與 `fn next(&mut self) -> Option<型別>`（`impl` 裡寫實際型別，不寫 `Self::Item`）；少了任何一個是 E0046。
- 迭代器自己把「走到哪裡」記在欄位裡，`next` 每次改動它，回傳 `Some(元素)`，沒有了就回傳 `None`。
- 實作之後可以直接用 `for`，並自動擁有 `map`、`filter`、`sum`、`zip`、`collect` 等全部預設方法（第 8 章第 3 集的預設方法）。
- 第 8 章第 17 集 `Dispenser` 的 `take` 就是 `next`，`Iterator` 是標準函式庫裡那個版本。
- 不想為了回傳迭代器另外定義型別，就回傳 `impl Iterator<Item = u32>`，把整串 adapter 直接交出去。
