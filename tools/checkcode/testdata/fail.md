# 每個區塊都應該失敗

沒有通過 rustfmt：

```rust
fn main() { println!("哈囉"); }
```

有警告（未使用的變數）：

```rust
fn main() {
    let x = 5;
}
```

clippy 警告（沒有放行的 lint）：

```rust
fn main() {
    let ok = true;
    if ok == true {
        println!("ok");
    }
}
```

錯誤碼不符：

```rust,compile_fail=E0499
fn main() {
    let s = String::from("hi");
    let t = s;
    println!("{s} {t}");
}
```

有錯誤碼卻沒寫：

```rust,compile_fail
fn main() {
    let s = String::from("hi");
    let t = s;
    println!("{s} {t}");
}
```

標了 compile_fail 卻編譯成功：

```rust,compile_fail=E0382
fn main() {
    println!("ok");
}
```

panic 但沒標 exit：

```rust
fn main() {
    let v: Option<i32> = None;
    println!("{}", v.unwrap());
}
```

stdin 沒加 noplayground：

```rust,stdin=3
fn main() {
    println!("ok");
}
```

拼錯標註：

```rust,norun
fn main() {
    println!("ok");
}
```

逾時：

```rust
fn main() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
```

測試失敗：

```rust,test
#[test]
fn adds() {
    assert_eq!(1 + 1, 3);
}
```

crate 區塊沒有通過 rustfmt：

```rust
#[tokio::main]
async fn main() { println!("ok"); }
```

crate 區塊有警告（未使用的變數）：

```rust
#[tokio::main]
async fn main() {
    let x = 5;
}
```

crate 區塊的 clippy 警告（沒有放行的 lint）：

```rust
fn main() {
    let ok = true;
    if ok == true {
        println!("{}", serde_json::json!(ok));
    }
}
```

清單外的 crate：

```rust
fn main() {
    let _client = reqwest::Client::new();
}
```

清單內外的 crate 混用：

```rust
fn main() {
    let _client = reqwest::Client::new();
    println!("{}", serde_json::json!(1));
}
```

crate 區塊錯誤碼不符：

```rust,compile_fail=E0499
fn main() {
    let v = serde_json::json!({ "a": 1 });
    let w = v;
    println!("{v} {w}");
}
```

crate 區塊標了 compile_fail 卻編譯成功：

```rust,compile_fail=E0382
#[tokio::main]
async fn main() {
    println!("ok");
}
```

crate 區塊有錯誤碼卻沒寫：

```rust,compile_fail
fn main() {
    let v = serde_json::json!({ "a": 1 });
    let w = v;
    println!("{v} {w}");
}
```

crate 區塊 panic 但沒標 exit：

```rust
fn parse(s: &str) -> anyhow::Result<i32> {
    let n: i32 = s.parse()?;
    Ok(n)
}

fn main() {
    println!("{}", parse("abc").expect("失敗"));
}
```

crate 區塊的結束碼跟標註不同：

```rust,exit=1
#[tokio::main]
async fn main() {
    println!("ok");
}
```

crate 區塊測試失敗：

```rust,test
#[tokio::test]
async fn adds() {
    assert_eq!(tokio::spawn(async { 1 + 1 }).await.expect("失敗"), 3);
}
```

crate 區塊 stdin 沒加 noplayground：

```rust,stdin=3
fn main() {
    println!("{}", serde_json::json!(1));
}
```
