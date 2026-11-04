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
