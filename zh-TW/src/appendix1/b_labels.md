# 標籤 `'outer`

## 本集目標

用標籤（label）讓 `break` 和 `continue` 直接對外層迴圈生效，也能用標籤把值從內層帶出去。

## 正文

### 只跳出最內層的問題

第 1 章第 14 集說過，`break` 和 `continue` 只影響最內層的迴圈。下面用兩層迴圈找「兩杯加起來剛好 83 元」的組合，找到就想結束：

```rust
fn main() {
    let prices = [30, 45, 38];
    for a in 0..3 {
        for b in 0..3 {
            if prices[a] + prices[b] == 83 {
                println!("{} 加 {} 剛好 83", prices[a], prices[b]);
                break;
            }
        }
    }
}
```

執行結果：

```text
45 加 38 剛好 83
38 加 45 剛好 83
```

內層的 `break` 結束之後，外層還繼續跑，所以同一組又找了一次，只是順序反過來。

### `break 'outer`

在迴圈前面寫 `'名字:`，這個迴圈就有了標籤；`break '名字` 會直接結束有這個標籤的迴圈：

```rust
fn main() {
    let prices = [30, 45, 38];
    'outer: for a in 0..3 {
        for b in 0..3 {
            if prices[a] + prices[b] == 83 {
                println!("{} 加 {} 剛好 83", prices[a], prices[b]);
                break 'outer;
            }
        }
    }
}
```

執行結果：

```text
45 加 38 剛好 83
```

`continue '名字` 同理，直接開始那個迴圈的下一圈。

標籤是單引號加名字，跟第 10 章的生命週期 `'a` 長得一樣，但兩者無關：標籤只能搭配 `break` 和 `continue`。名字自己取，常見的是 `'outer`。`for`、`while`、`loop` 都能加標籤；`break` 用了沒宣告的標籤是 E0426。

### 用標籤把值帶出去

第 1 章第 12 集說過，`loop` 的 `break` 能帶值，但 `for` 的 `break` 不行。內層是 `for` 的時候，值要靠外層有標籤的 `loop` 帶出來：

```rust
fn main() {
    let mut total = 0;
    let count = 'search: loop {
        for price in [30, 45, 38] {
            total += price;
            if total > 100 {
                break 'search price;
            }
        }
    };
    println!("{count} {total}");
}
```

執行結果：

```text
38 113
```

反覆加 30、45、38，加到超過 100 就停，`count` 拿到的是讓總額超標的那一杯的價錢。

標籤也能加在一般的 `{ }` 區塊上：`let msg = 'check: { ... break 'check "缺貨"; ... "足夠" };`。`break '名字 值` 會提早離開區塊，區塊的值就是 `break` 帶的值，像是離開區塊的 early `return`。

### 跟 Go 比一比

Go 也有標籤，寫法是 `outer:` 放在迴圈前一行，`break outer` 的意思相同。差別在 `switch` 與 `match`。Go 的 `break` 寫在 `switch` 裡，只會跳出 `switch`：

```go
for i := 0; i < 5; i++ {
	switch i {
	case 3:
		break // 只跳出 switch，迴圈繼續
	default:
		fmt.Println(i)
	}
}
// 印出 0 1 2 4
```

Rust 的 `match` 不是迴圈，`break` 會穿過它，結束外面的迴圈：

```rust
fn main() {
    for i in 0..5 {
        match i {
            3 => break,
            _ => println!("{i}"),
        }
    }
    println!("done");
}
```

執行結果：

```text
0
1
2
done
```

迴圈在 3 就結束了，所以沒有 4。

## 重點整理

- `break`、`continue` 預設只對最內層的迴圈生效；在迴圈前面寫 `'名字:` 加標籤，`break '名字`、`continue '名字` 就能指定要結束或繼續哪一層。
- 標籤是單引號加名字，跟生命週期無關；用了沒宣告的標籤是 E0426。
- `loop` 加標籤後，`break '名字 值` 能把值從內層迴圈帶出來；標籤也能加在區塊上。
- Go 的 `break` 在 `switch` 裡只跳出 `switch`；Rust 的 `break` 在 `match` 裡會結束外面的迴圈。
