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
