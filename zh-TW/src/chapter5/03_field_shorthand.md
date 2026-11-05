# 欄位初始化簡寫

## 本集目標

變數名稱跟欄位名稱一樣時，建立 struct 可以只寫名稱；也要知道這個寫法一樣會把變數 move 進 struct。

## 正文

### 名字重複的問題

用函式建立 struct 時，參數的名字常常就是欄位的名字。照上一集的寫法，會變成 `Member { name: name, city: city, points: 0 }`，每個名字都寫兩遍。

### 簡寫

變數名稱跟欄位名稱相同時，`欄位: 變數` 可以只寫一次名字：

```rust
struct Member {
    name: String,
    city: String,
    points: u32,
}

fn make_member(name: String, city: String) -> Member {
    Member {
        name,
        city,
        points: 0,
    }
}

fn main() {
    let member = make_member(String::from("小美"), String::from("台北"));
    println!("{} 住在 {}", member.name, member.city);
    println!("點數：{}", member.points);
}
```

執行結果：

```text
小美 住在 台北
點數：0
```

`name,` 等於 `name: name`：欄位 `name` 的值，就是同名變數 `name`。`make_member` 的最後一個運算式是 `Member { ... }`，所以它就是回傳值（第 2 章第 4 集）。

簡寫跟完整寫法可以混用：名字相同的用簡寫，其他欄位照樣寫 `欄位: 值`，像上面的 `points: 0`。名字不同時就不能簡寫，必須寫完整。

### 簡寫也會 move

`name,` 就是把變數 `name` 放進 struct，所以 `String` 會 move 進去（第 3 章第 5 集）。之後再用原本的變數，就是 E0382：

```rust,compile_fail=E0382
struct Member {
    name: String,
    points: u32,
}

fn main() {
    let name = String::from("小美");
    let member = Member { name, points: 0 };
    println!("{name} 有 {} 點", member.points);
}
```

編譯錯誤：

```text
error[E0382]: borrow of moved value: `name`
 --> src/main.rs:9:16
  |
7 |     let name = String::from("小美");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
8 |     let member = Member { name, points: 0 };
  |                           ---- value moved here
9 |     println!("{name} 有 {} 點", member.points);
  |                ^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
8 |     let member = Member { name: name.clone(), points: 0 };
  |                               ++++++++++++++
```

`name` 已經 move 進 `member`，要用名字就從 `member.name` 讀。像 `points` 這種 `Copy` 型別的欄位，簡寫之後原本的變數還能用，因為被複製了。

## 重點整理

- 變數名稱跟欄位名稱相同時，可以寫 `Member { name, city, points: 0 }`，`name,` 等於 `name: name`。
- 簡寫可以跟 `欄位: 值` 混用；名字不同就要寫完整。
- 簡寫是把變數放進 struct：`String` 之類的值會 move（之後再用變數是 E0382），`Copy` 型別則是複製。
