# 泛型函式

## 本集目標

用 `<T>` 寫出能處理多種型別的泛型函式，認得 `Option<T>`、`Vec<T>` 裡的 `<T>` 就是它；並知道沒有 trait bound 的 `T` 什麼都不能做。

## 正文

### 一個函式，很多種型別

第 5 集的 `T: Priced` 已經用過型別參數了。這一集的 `T` 沒有任何條件，一個函式就能處理各種型別：

```rust
fn first<T>(items: &[T]) -> &T {
    &items[0]
}

fn swap_pair<T, U>(pair: (T, U)) -> (U, T) {
    let (a, b) = pair;
    (b, a)
}

fn main() {
    let prices = [30, 45, 38];
    let names = [String::from("紅茶"), String::from("咖啡")];
    println!("{}", first(&prices));
    println!("{}", first(&names));
    println!("{:?}", swap_pair(("紅茶", 30)));
    println!("{}", first::<i32>(&prices));
}
```

執行結果：

```text
30
紅茶
(30, "紅茶")
30
```

- `first<T>` 的 `<T>` 宣告型別參數。`&[T]` 是第 4 章第 11 集的 slice，元素的型別是 `T`；回傳 `&T`，指向其中一個元素。
- 呼叫的時候不用寫 `T`：編譯器從引數推出來。`first(&prices)` 的 `T` 是 `i32`，`first(&names)` 的 `T` 是 `String`。
- `swap_pair<T, U>` 有兩個型別參數，名字隨便取，只要彼此不同。`T` 和 `U` 可以是不同的型別，也可以剛好相同。
- `first::<i32>(&prices)` 把型別明明白白寫出來，用 `::<型別>`。平常不需要，編譯器推不出來的時候才要用。第 14 集會看到 `parse::<i32>()`，是同一種寫法。
- 編譯器會替每一種實際用到的型別，各編出一份專用的版本，所以泛型不會讓執行變慢。

### `Option<T>`、`Vec<T>` 的 `<T>` 就是它

第 6 章第 6 集和第 7 章第 1 集一直叫你「先把 `Vec<i32>` 整個當成型別名字」。現在可以拆開看了：

- `Option<T>` 就是 `enum Option<T> { None, Some(T) }`，`T` 是型別參數。`Option<u32>` 是 `T` 換成 `u32`，`Option<String>` 是換成 `String`。
- `Vec<T>` 也一樣，`T` 是元素的型別。
- `HashMap<K, V>` 有兩個型別參數，鍵 `K` 跟值 `V`，就像上面的 `<T, U>`。

下一集再來自己寫泛型 struct。

### 沒有條件的 `T`，什麼都不能做

`first` 和 `swap_pair` 只是把值搬來搬去，不需要知道 `T` 是什麼。但是只要想「對它做點什麼」，就會被擋下。例如想比較兩個 `T` 是不是相等：

```rust,compile_fail=E0369
fn has_item<T>(items: &[T], target: &T) -> bool {
    for item in items {
        if item == target {
            return true;
        }
    }
    false
}

fn main() {
    let prices = [30, 45, 38];
    println!("{}", has_item(&prices, &45));
}
```

編譯錯誤：

```text
error[E0369]: binary operation `==` cannot be applied to type `&T`
 --> src/main.rs:3:17
  |
3 |         if item == target {
  |            ---- ^^ ------ &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialEq`
  |
1 | fn has_item<T: std::cmp::PartialEq>(items: &[T], target: &T) -> bool {
  |              +++++++++++++++++++++
```

`T` 可能是任何型別，編譯器不知道它能不能 `==`。`help` 說得很直接：替 `T` 加上 `PartialEq` 這個 trait。`==` 背後就是 `PartialEq`（第 4 章第 7 集、第 6 章第 3 集提過），怎麼讓自己的型別也能 `==`，是第 10 集的事。現在只要知道：想用 `==`，就要求 `T: PartialEq`。`help` 寫的是 `std::cmp::PartialEq`，那是完整路徑，直接寫 `PartialEq` 就行。

```rust
fn has_item<T: PartialEq>(items: &[T], target: &T) -> bool {
    for item in items {
        if item == target {
            return true;
        }
    }
    false
}

fn main() {
    let prices = [30, 45, 38];
    let names = [String::from("紅茶"), String::from("咖啡")];
    println!("{}", has_item(&prices, &45));
    println!("{}", has_item(&names, &String::from("奶茶")));
}
```

執行結果：

```text
true
false
```

整數和 `String` 都實作了 `PartialEq`，所以兩個都能傳。Go 的泛型也是這樣：`[T any]` 什麼都不能做，要 `==` 得寫成 `[T comparable]`。Rust 用 trait bound 做同樣的事。

## 重點整理

- `fn first<T>(items: &[T]) -> &T`：`<T>` 宣告型別參數，一個函式處理多種型別；呼叫時編譯器推出 `T`，要明寫用 `first::<i32>(...)`。
- 可以有多個型別參數：`<T, U>`；編譯器為每種實際用到的型別各產生一份，沒有執行時期的額外成本。
- `Option<T>`、`Vec<T>`、`HashMap<K, V>` 的尖括號，就是這種型別參數。
- 沒有 trait bound 的 `T` 只能搬動、不能操作；用 `==` 會是 E0369，要寫成 `T: PartialEq`。
