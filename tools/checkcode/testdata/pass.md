# 應該全部通過

```rust
fn main() {
    println!("哈囉");
}
```

```rust,stdin=Andy\n3,noplayground
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    let name = line.trim().to_string();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    let n: i32 = line.trim().parse().expect("請輸入整數");
    println!("{name} {n}");
}
```

```rust,compile_fail=E0382
fn main() {
    let s = String::from("hi");
    let t = s;
    println!("{s} {t}");
}
```

```rust,compile_fail
fn main() {
    let x = ;
}
```

```rust,exit=101
fn main() {
    let a = [1, 2, 3];
    let i = a.len();
    println!("{}", a[i]);
}
```

```rust,no_run
fn main() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
```

```rust,warn
fn main() {
    let x = 5;
}
```

```rust,test
#[test]
fn adds() {
    assert_eq!(1 + 1, 2);
}
```

```rust,ignore
這不是程式
```

```rust
fn main() {
    let x = 7;
    if let Some(v) = Some(x)
        && v > 3
    {
        println!("{v}");
    }
}
```

用到 crate 的區塊（tokio）：

```rust
#[tokio::main]
async fn main() {
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    println!("哈囉 tokio");
}
```

crate 區塊也能用標註。stdin：

```rust,stdin=Andy,noplayground
fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("讀取失敗");
    let name = line.trim();
    println!("{}", serde_json::json!({ "name": name }));
}
```

exit：

```rust,exit=101
fn parse(s: &str) -> anyhow::Result<i32> {
    let n: i32 = s.parse()?;
    Ok(n)
}

fn main() {
    println!("{}", parse("abc").expect("失敗"));
}
```

no_run：

```rust,no_run
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "哈囉" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("綁定失敗");
    axum::serve(listener, app).await.expect("伺服器錯誤");
}
```

warn：

```rust,warn
fn main() {
    let id = uuid::Uuid::new_v4();
}
```

test：

```rust,test
#[tokio::test]
async fn adds() {
    assert_eq!(tokio::spawn(async { 1 + 1 }).await.expect("失敗"), 2);
}
```

compile_fail 帶錯誤碼：

```rust,compile_fail=E0382
fn main() {
    let v = serde_json::json!({ "a": 1 });
    let w = v;
    println!("{v} {w}");
}
```

compile_fail 不帶錯誤碼：

```rust,compile_fail
fn main() {
    let x = tokio::;
}
```

放行清單的 lint（needless_return）在 crate 區塊也放行：

```rust
fn double(n: i32) -> i32 {
    return n * 2;
}

fn main() {
    println!("{}", serde_json::json!(double(2)));
}
```
