# 必須明確實作

## 本集目標

知道型別剛好有同名、同簽名的方法並不算實作 trait，一定要寫 `impl 某trait for 型別`；並能對照 Go 的隱式介面。

## 正文

### 方法對得上，也不算數

`Coffee` 自己有一個 `price` 方法，簽名跟 `Priced` 的 `price` 一模一樣。但它沒有寫 `impl Priced for Coffee`，所以編譯器不承認它是 `Priced`：

```rust,compile_fail=E0277
struct Coffee {
    shots: u32,
}

impl Coffee {
    fn price(&self) -> u32 {
        45 + self.shots * 10
    }
}

trait Priced {
    fn price(&self) -> u32;
}

fn show(item: &impl Priced) {
    println!("{} 元", item.price());
}

fn main() {
    let coffee = Coffee { shots: 1 };
    println!("{}", coffee.price());
    show(&coffee);
}
```

編譯錯誤：

```text
error[E0277]: the trait bound `Coffee: Priced` is not satisfied
  --> src/main.rs:22:10
   |
22 |     show(&coffee);
   |     ---- ^^^^^^^ unsatisfied trait bound
   |     |
   |     required by a bound introduced by this call
   |
```

`show` 的參數 `&impl Priced` 先照抄，第 4 集才正式解釋：現在讀成「任何實作了 `Priced` 的型別的參照」就好。

- `coffee.price()` 沒問題，那是 `impl Coffee` 裡自己的方法。
- `show(&coffee)` 才出錯：`show` 只收實作了 `Priced` 的型別，而 `Coffee` 沒有。訊息的第一行已經說了：`` the trait bound `Coffee: Priced` is not satisfied ``。
- 這裡只貼前半段，後面還有 `help` 和 `note`，大意是建議你替 `Coffee` 補上實作。

### 補上 `impl`

改成明確實作，`show` 就收得下了：

```rust
struct Coffee {
    shots: u32,
}

trait Priced {
    fn price(&self) -> u32;
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        45 + self.shots * 10
    }
}

fn show(item: &impl Priced) {
    println!("{} 元", item.price());
}

fn main() {
    let coffee = Coffee { shots: 1 };
    show(&coffee);
}
```

執行結果：

```text
55 元
```

為什麼 Rust 要多寫這一行？因為名字一樣不代表意思一樣：`Coffee` 的 `price` 可能是含稅價，`Priced` 要的卻是未稅價。`impl Priced for Coffee` 是型別明確表態「我願意當 `Priced` 用」。這一行同時也是搜尋的線索：想知道哪些型別實作了 `Priced`，找 `impl Priced for` 就行。

### 跟 Go 比一比

Go 的介面是隱式的：型別有對的方法，就自動滿足介面，不需要宣告。

```go
type Pricer interface {
	Price() int
}

type Coffee struct{ Shots int }

func (c Coffee) Price() int { return 45 + c.Shots*10 }

func show(p Pricer) { fmt.Println(p.Price()) }

func main() {
	show(Coffee{Shots: 1}) // 沒有任何地方寫「Coffee 實作 Pricer」，照樣編譯成功
}
```

Go 的直覺在 Rust 會踩到兩個地方：

- 在 Rust，方法簽名對得上沒有用，一定要寫 `impl 某trait for 型別`。沒寫，就是 E0277。
- Go 裡碰巧同名的方法會悄悄滿足介面，沒有人宣告過意圖。Rust 把這個意圖寫在程式裡，所以不會「意外」實作了某個 trait。

## 重點整理

- 型別有同名、同簽名的方法，不算實作 trait；一定要寫 `impl 某trait for 型別`。
- 型別沒實作 trait，卻傳給要求該 trait 的函式，是 E0277（`the trait bound ... is not satisfied`）。
- `&impl Priced` 先讀成「任何實作了 `Priced` 的型別的參照」，第 4 集正式解釋。
- 跟 Go 比：Go 的介面是隱式滿足，Rust 一定要明確寫 `impl`；找哪些型別實作了某 trait，搜 `impl 某trait for` 即可。
