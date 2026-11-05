# `Self`

## 本集目標

認得 `Self`：在 `impl` 與 trait 裡，它代表「正在替它寫程式的那個型別」，可以取代重複的型別名稱。

## 正文

### `impl` 裡的 `Self`

第 5 章以來，`impl` 裡的型別都寫完整名稱。其實在 `impl Account { ... }` 裡，`Self` 就是 `Account`，兩種寫法完全等價：

```rust
struct Account {
    owner: String,
    balance: u32,
}

impl Account {
    const OPENING: u32 = 100;

    fn new(owner: String) -> Self {
        Self {
            owner,
            balance: Self::OPENING,
        }
    }

    fn show(&self) {
        println!("{} 有 {} 點", self.owner, self.balance);
    }
}

fn main() {
    let account = Account::new(String::from("小美"));
    account.show();
}
```

執行結果：

```text
小美 有 100 點
```

- `-> Self` 等於 `-> Account`。
- `Self { owner, balance: ... }` 等於 `Account { owner, balance: ... }`，建立 struct 的寫法不變，只是名字換了。
- `Self::OPENING` 等於 `Account::OPENING`；呼叫關聯函式也能寫 `Self::new(...)`。

用 `Self` 的好處是型別改名時，`impl` 裡不用跟著改。tuple struct 的建構子也一樣：`struct Meters(f64);` 的 `impl` 裡寫 `Self(1.5)`。

### enum 也能用

`match self` 的分支與回傳值，都可以寫 `Self::variant`：

```rust
#[derive(Debug)]
enum Status {
    Pending,
    Paid,
    Shipped,
}

impl Status {
    fn next(&self) -> Self {
        match self {
            Self::Pending => Self::Paid,
            Self::Paid | Self::Shipped => Self::Shipped,
        }
    }
}

fn main() {
    let status = Status::Pending;
    println!("{:?}", status.next());
}
```

執行結果：

```text
Paid
```

型別帶著型別參數或生命週期時，`Self` 更省事：`impl<T> Shelf<T> { fn new() -> Self { ... } }` 的 `Self` 就是 `Shelf<T>`，`impl<'a> Receipt<'a> { fn new(customer: &'a str) -> Self { ... } }` 的 `Self` 是 `Receipt<'a>`，不用再把 `<T>`、`<'a>` 重寫一遍。

### 小寫 `self` 與大寫 `Self`

`self`（小寫）是值：方法的第一個參數，代表呼叫這個方法的那個值。`Self`（大寫）是型別：`self` 的型別就是 `Self`。所以第 5 章第 8 集的 `&self` 是 `self: &Self` 的簡寫，`&mut self` 是 `self: &mut Self`，`self` 是 `self: Self`。

### trait 裡的 `Self`

在 trait 定義裡，`Self` 代表「將來實作這個 trait 的型別」，那時還不知道是誰。標準函式庫的 `FromStr`（簡化過）是這樣寫的：

```text
trait FromStr {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```

`Result<Self, Self::Err>` 讀成「成功是實作的型別本身，失敗是它的 `Err`」。`Self::Err` 是關聯型別（第 8 章第 17 集）。第 8 章第 17 集的 `Option<Self::Item>`、第 11 章第 7 集 `Iterator` 的 `next`，都是同一種寫法。第 9 章第 7 集的 `Size` 實作 `FromStr`，用 `Self` 重寫就是（錯誤型別簡化成 unit struct）：

```rust
#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
struct SizeError;

impl std::str::FromStr for Size {
    type Err = SizeError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "S" => Ok(Self::Small),
            "M" => Ok(Self::Medium),
            "L" => Ok(Self::Large),
            _ => Err(SizeError),
        }
    }
}

fn main() {
    println!("{:?}", "M".parse::<Size>());
    println!("{:?}", "XL".parse::<Size>());
}
```

執行結果：

```text
Ok(Medium)
Err(SizeError)
```

在這個 `impl` 裡，`Self` 是 `Size`，`Self::Err` 是 `SizeError`。

`Self` 只在 `impl`、trait 和型別定義的內部才存在，在 `main` 這類普通函式裡寫 `Self`，編譯器會說找不到（E0411）。

## 重點整理

- 在 `impl 型別 { ... }` 裡，`Self` 就是那個型別：`-> Self`、`Self { ... }`、`Self::new(...)`、`Self::CONST` 都能用，跟寫完整名稱等價。
- enum 的 `impl` 裡可以寫 `Self::variant`；型別帶 `<T>` 或 `<'a>` 時，`Self` 不用重寫它們。
- `self` 是值，`Self` 是型別；`&self` 是 `self: &Self` 的簡寫。
- trait 定義裡的 `Self` 代表將來實作的型別，`Self::Item`、`Self::Err` 是它的關聯型別。
- `Self` 只能用在 `impl`、trait、型別定義裡，其他地方是 E0411。
- 這一集之後，`impl` 裡可以用 `Self`；之前章節寫完整型別名稱，是為了照進度，兩種寫法編譯結果相同。
