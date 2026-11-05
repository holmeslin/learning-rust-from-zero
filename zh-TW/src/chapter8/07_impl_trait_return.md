# 回傳 `impl Trait`

## 本集目標

用 `-> impl Priced` 讓函式回傳「某個實作了 `Priced` 的型別」，而不寫出具體型別；知道這個寫法只能回傳同一種型別。

## 正文

### 只說能做什麼，不說是什麼

`impl Trait` 也能寫在回傳型別的位置。下面的 `daily_special` 回傳「某個實作了 `Priced` 的東西」：

```rust
struct Tea {
    toppings: u32,
}

trait Priced {
    fn price(&self) -> u32;
}

impl Priced for Tea {
    fn price(&self) -> u32 {
        35 + self.toppings * 5
    }
}

fn daily_special() -> impl Priced {
    Tea { toppings: 3 }
}

fn main() {
    let special = daily_special();
    println!("今日特餐 {} 元", special.price());
}
```

執行結果：

```text
今日特餐 50 元
```

- `-> impl Priced`：呼叫的人只知道拿到的東西實作了 `Priced`，不知道它是 `Tea`。所以 `special.price()` 可以用，`special.toppings` 不行（E0609，訊息是 `` no field `toppings` on type `impl Priced` ``），跟第 4 集的參數是同一個道理。
- 函式裡面回傳的是真正的 `Tea`。換成別的型別也行，只要有 `Priced`，呼叫的人不用改。

### 只能回傳同一種型別

`impl Priced` 不是「`Priced` 的任何型別都可以」，而是「某一個固定的型別，只是不說名字」。函式的每一條回傳路徑必須是同一種型別：

```rust,compile_fail=E0308
struct Tea {
    toppings: u32,
}

struct Coffee {
    shots: u32,
}

trait Priced {
    fn price(&self) -> u32;
}

impl Priced for Tea {
    fn price(&self) -> u32 {
        35 + self.toppings * 5
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        45 + self.shots * 10
    }
}

fn pick(want_tea: bool) -> impl Priced {
    if want_tea {
        Tea { toppings: 0 }
    } else {
        Coffee { shots: 1 }
    }
}

fn main() {
    let item = pick(true);
    println!("{}", item.price());
}
```

編譯錯誤：

```text
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:29:9
   |
26 | /     if want_tea {
27 | |         Tea { toppings: 0 }
   | |         ------------------- expected because of this
28 | |     } else {
29 | |         Coffee { shots: 1 }
   | |         ^^^^^^^^^^^^^^^^^^^ expected `Tea`, found `Coffee`
30 | |     }
   | |_____- `if` and `else` have incompatible types
```

`if` 回傳 `Tea`、`else` 回傳 `Coffee`，兩邊的型別不同，就是 E0308。這裡只貼前半段，後面的 `help` 建議改成 `Box<dyn Priced>`，那是第 15 集的做法（`Box` 第 13 章第 1 集才正式介紹），現在不用管。

### 什麼時候用

- 想把具體型別藏起來：函式對外只承諾「有 `Priced`」，裡面要換成別的型別，呼叫端不受影響。
- 有些型別根本寫不出名字，例如第 11 章的閉包與迭代器，那時候回傳 `impl Trait` 幾乎是唯一的寫法。

### 跟 Go 比一比

Go 的函式回傳介面型別，每次呼叫可以回傳不同的實際型別：

```go
func pick(wantTea bool) Pricer {
	if wantTea {
		return Tea{Toppings: 0}
	}
	return Coffee{Shots: 1}
}
```

這在 Rust 的 `-> impl Priced` 是 E0308。Go 的介面值在執行時才記下裡面是什麼型別；`impl Priced` 則是編譯時期就要定死是哪一個型別。要像 Go 一樣隨條件換型別，Rust 得用第 15 集的 `dyn Trait`。

## 重點整理

- `-> impl Priced`：回傳「某個實作了 `Priced` 的型別」，呼叫的人只能用 `Priced` 的方法，不能讀欄位（E0609）。
- 回傳的必須是同一種具體型別；不同路徑回傳不同型別是 E0308，要換型別得用 `dyn Trait`（第 15 集）。
- 用途：藏起具體型別；閉包、迭代器這類寫不出名字的型別（第 11 章）。
- 跟 Go 比：Go 回傳介面型別可以每次回傳不同的實際型別；Rust 的 `-> impl Trait` 要定死一個型別。
