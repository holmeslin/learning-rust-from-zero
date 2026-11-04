//! 驗證書中所有 ```rust 區塊：rustfmt、clippy-driver（警告視為錯誤）、實際執行。
//! 用法：cargo run --manifest-path tools/checkcode/Cargo.toml -- <目錄或 .md 檔>...
//!
//! shortcut: 目前每個區塊各自用 clippy-driver 編譯，只支援 std；
//! 第二部（第 13 章起）用到 crate 時，再加上共用 Cargo.toml + 多 bin package 的批次編譯。

use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const CLIPPY_ALLOW: &str = include_str!("../clippy-allow.txt");
const RUN_TIMEOUT: Duration = Duration::from_secs(30);
static NEXT_DIR: AtomicUsize = AtomicUsize::new(0);

struct Block {
    file: PathBuf,
    line: usize,
    info: String,
    code: String,
}

#[derive(Default, Debug)]
struct Opts {
    /// `Some(None)` 是不帶錯誤碼的 `compile_fail`。
    compile_fail: Option<Option<String>>,
    stdin: Option<String>,
    exit: i32,
    no_run: bool,
    ignore: bool,
    warn: bool,
    test: bool,
    noplayground: bool,
}

fn parse_opts(info: &str) -> Result<Opts, String> {
    let mut o = Opts::default();
    for tok in info.split(',').skip(1) {
        match tok.split_once('=') {
            Some(("compile_fail", code)) => o.compile_fail = Some(Some(code.to_string())),
            Some(("stdin", v)) => o.stdin = Some(v.replace("\\n", "\n") + "\n"),
            Some(("exit", v)) => {
                o.exit = v.parse().map_err(|_| format!("exit 的值不是整數：{v}"))?;
            }
            None if tok == "compile_fail" => o.compile_fail = Some(None),
            None if tok == "no_run" => o.no_run = true,
            None if tok == "ignore" => o.ignore = true,
            None if tok == "warn" => o.warn = true,
            None if tok == "test" => o.test = true,
            None if tok == "noplayground" => o.noplayground = true,
            _ => return Err(format!("不認得的標註：{tok}")),
        }
    }
    if o.stdin.is_some() && !o.noplayground {
        return Err("用了 stdin= 就要同時加 noplayground".into());
    }
    Ok(o)
}

fn extract_blocks(file: &Path, text: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut current: Option<Block> = None;
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim_start();
        match current.as_mut() {
            None => {
                if let Some(info) = line.strip_prefix("```")
                    && (info == "rust" || info.starts_with("rust,"))
                {
                    current = Some(Block {
                        file: file.to_path_buf(),
                        line: i + 1,
                        info: info.to_string(),
                        code: String::new(),
                    });
                }
            }
            Some(block) => {
                if line.trim_end() == "```" {
                    blocks.extend(current.take());
                } else {
                    block.code.push_str(raw);
                    block.code.push('\n');
                }
            }
        }
    }
    blocks
}

fn check(block: &Block, tmp: &Path) -> Result<(), String> {
    let opts = parse_opts(&block.info)?;
    if opts.ignore {
        return Ok(());
    }
    let dir = tmp.join(NEXT_DIR.fetch_add(1, Ordering::Relaxed).to_string());
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let src = dir.join("main.rs");
    let bin = dir.join("main");
    fs::write(&src, &block.code).map_err(|e| e.to_string())?;

    // compile_fail 的程式可能連語法都不對，rustfmt 無法處理，所以跳過。
    if opts.compile_fail.is_none() {
        let out = Command::new("rustfmt")
            .args(["--edition", "2024", "--check"])
            .arg(&src)
            .output()
            .map_err(|e| format!("無法執行 rustfmt：{e}"))?;
        if !out.status.success() {
            return Err(format!(
                "沒有通過 rustfmt --check：\n{}",
                String::from_utf8_lossy(&out.stdout)
            ));
        }
    }

    let mut cmd = Command::new("clippy-driver");
    cmd.args(["--edition", "2024", "-o"]).arg(&bin).arg(&src);
    if !opts.warn {
        cmd.args(["-D", "warnings"]);
    }
    if opts.test {
        cmd.arg("--test");
    }
    for lint in CLIPPY_ALLOW
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        cmd.arg("-A").arg(format!("clippy::{lint}"));
    }
    let out = cmd
        .output()
        .map_err(|e| format!("無法執行 clippy-driver：{e}"))?;
    let stderr = String::from_utf8_lossy(&out.stderr);

    if let Some(code) = opts.compile_fail {
        if out.status.success() {
            return Err("標了 compile_fail，卻編譯成功".into());
        }
        return match code {
            Some(code) if stderr.contains(&format!("error[{code}]")) => Ok(()),
            Some(code) => Err(format!("預期錯誤碼 {code}，實際錯誤：\n{stderr}")),
            None if stderr.contains("error[E") => Err(format!(
                "錯誤有錯誤碼，請寫成 compile_fail=E....：\n{stderr}"
            )),
            None => Ok(()),
        };
    }
    if !out.status.success() {
        return Err(format!("編譯失敗：\n{stderr}"));
    }
    if opts.no_run {
        return Ok(());
    }

    let stdout_path = dir.join("stdout");
    let stderr_path = dir.join("stderr");
    let mut child = Command::new(&bin)
        .stdin(Stdio::piped())
        .stdout(File::create(&stdout_path).map_err(|e| e.to_string())?)
        .stderr(File::create(&stderr_path).map_err(|e| e.to_string())?)
        .spawn()
        .map_err(|e| format!("無法執行程式：{e}"))?;
    // 先寫完 stdin 再關掉，程式讀到 EOF 才不會卡住；不讀 stdin 的程式可能已結束，忽略寫入錯誤。
    if let Some(mut input) = child.stdin.take() {
        let _ = std::io::Write::write_all(&mut input, opts.stdin.unwrap_or_default().as_bytes());
    }
    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            break status;
        }
        if start.elapsed() > RUN_TIMEOUT {
            let _ = child.kill();
            return Err(format!("執行超過 {} 秒", RUN_TIMEOUT.as_secs()));
        }
        thread::sleep(Duration::from_millis(20));
    };
    let code = status.code().unwrap_or(-1);
    if code != opts.exit {
        let output = fs::read_to_string(&stdout_path).unwrap_or_default()
            + &fs::read_to_string(&stderr_path).unwrap_or_default();
        return Err(format!(
            "結束碼是 {code}，預期 {}。輸出：\n{output}",
            opts.exit
        ));
    }
    Ok(())
}

fn collect_md(path: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            collect_md(&entry?.path(), out)?;
        }
    } else if path.extension().is_some_and(|e| e == "md") {
        out.push(path.to_path_buf());
    }
    Ok(())
}

/// 平行檢查所有區塊，回傳失敗的 (區塊, 原因)，依檔名與行號排序。
fn check_all(blocks: &[Block]) -> Vec<(&Block, String)> {
    // 每次呼叫各用一個目錄：測試會同時呼叫好幾次，結束時各自刪掉自己的目錄。
    let tmp = env::temp_dir().join(format!(
        "checkcode-{}-{}",
        std::process::id(),
        NEXT_DIR.fetch_add(1, Ordering::Relaxed)
    ));
    let next = AtomicUsize::new(0);
    let failures = Mutex::new(Vec::new());
    let workers = thread::available_parallelism().map_or(4, |n| n.get());
    thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| {
                while let Some(block) = blocks.get(next.fetch_add(1, Ordering::Relaxed)) {
                    if let Err(msg) = check(block, &tmp) {
                        failures.lock().unwrap().push((block, msg));
                    }
                }
            });
        }
    });
    let _ = fs::remove_dir_all(&tmp);
    let mut failures = failures.into_inner().unwrap();
    failures.sort_by(|a, b| (&a.0.file, a.0.line).cmp(&(&b.0.file, b.0.line)));
    failures
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("用法：checkcode <目錄或 .md 檔>...");
        return ExitCode::from(2);
    }
    let mut files = Vec::new();
    for arg in &args {
        if let Err(e) = collect_md(Path::new(arg), &mut files) {
            eprintln!("{arg}: {e}");
            return ExitCode::from(2);
        }
    }
    let mut blocks = Vec::new();
    for file in &files {
        match fs::read_to_string(file) {
            Ok(text) => blocks.extend(extract_blocks(file, &text)),
            Err(e) => {
                eprintln!("{}: {e}", file.display());
                return ExitCode::from(2);
            }
        }
    }

    let failures = check_all(&blocks);
    for (block, msg) in &failures {
        println!("{}:{}: {msg}", block.file.display(), block.line);
    }
    println!(
        "{} 個檔案、{} 個 rust 區塊，{} 個失敗",
        files.len(),
        blocks.len(),
        failures.len()
    );
    if failures.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blocks_of(name: &str) -> Vec<Block> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("testdata")
            .join(name);
        extract_blocks(&path, &fs::read_to_string(&path).unwrap())
    }

    #[test]
    fn pass_blocks_all_pass() {
        let blocks = blocks_of("pass.md");
        assert!(blocks.len() >= 8);
        let failures = check_all(&blocks);
        assert!(
            failures.is_empty(),
            "{:#?}",
            failures
                .iter()
                .map(|f| (f.0.line, &f.1))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn fail_blocks_each_fail() {
        let blocks = blocks_of("fail.md");
        assert!(blocks.len() >= 8);
        let failed: Vec<usize> = check_all(&blocks).iter().map(|f| f.0.line).collect();
        for block in &blocks {
            assert!(
                failed.contains(&block.line),
                "第 {} 行應該失敗卻通過",
                block.line
            );
        }
    }
}
