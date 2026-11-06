//! 驗證書中所有 ```rust 區塊：rustfmt、clippy（警告視為錯誤）、實際執行。
//! 用法：cargo run --manifest-path tools/checkcode/Cargo.toml -- <目錄或 .md 檔>...
//!
//! 只用 std 的區塊各自用 clippy-driver 編譯。用到 crate 的區塊（見 `uses_crates`）放進
//! `deps/` 的共用依賴 package，每個區塊一個 bin，一次批次編譯與 lint；`compile_fail` 與 `test`
//! 區塊另外單獨編譯。編譯完成後，兩種區塊都走同一套判定與執行流程。
//!
//! 多檔案專案（第 12 章起，一集一個專案，見 `project.rs`）：依路徑標籤把區塊組成暫時的 Cargo
//! 專案，在每個 `cargo …` 的輸出區塊那一刻執行該指令，檢查成功或失敗跟書上貼的一致。
//!
//! shortcut: 共用的 target 目錄固定在 `deps/target`（跟 CI 快取一致），所以同一時間只能有
//! 一個 checkcode 行程編譯 crate 區塊；有需要平行跑多個行程時，再改成各自的 target 目錄。

mod project;

use std::env;
use std::fs::{self, File};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

use project::{Project, check_projects, extract_project};

const CLIPPY_ALLOW: &str = include_str!("../clippy-allow.txt");
const DEPS_TOML: &str = include_str!("../deps/Cargo.toml");
const DEPS_LOCK: &str = include_str!("../deps/Cargo.lock");
const DEPS_TARGET: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/deps/target");
const RUN_TIMEOUT: Duration = Duration::from_secs(30);
static NEXT_DIR: AtomicUsize = AtomicUsize::new(0);
/// 所有 crate 區塊共用同一個 target 目錄與 package 目錄，編譯到執行期間一次只讓一組進行。
static CARGO_LOCK: Mutex<()> = Mutex::new(());

/// 失敗的位置（檔案、行號）與原因。
type Failure = (PathBuf, usize, String);

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

/// 通過 rustfmt 檢查、等著編譯的區塊。程式存在 `dir/main.rs`。
struct Job<'a> {
    block: &'a Block,
    opts: Opts,
    dir: PathBuf,
}

/// 編譯結果：是否成功、錯誤訊息、可執行檔的位置。
struct Built {
    ok: bool,
    stderr: String,
    exe: PathBuf,
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

/// 放行的 clippy lint（不含 `clippy::` 前綴）。
fn clippy_allow() -> impl Iterator<Item = &'static str> {
    CLIPPY_ALLOW
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
}

/// Cargo.toml 中 `[section]` 底下的鍵（每行 `key = ...`）。
fn toml_keys<'a>(toml: &'a str, section: &str) -> Vec<&'a str> {
    let header = format!("[{section}]");
    toml.lines()
        .skip_while(|l| l.trim() != header)
        .skip(1)
        .take_while(|l| !l.trim_start().starts_with('['))
        .filter_map(|l| l.split_once('=').map(|(key, _)| key.trim()))
        .filter(|key| !key.is_empty() && !key.starts_with('#'))
        .collect()
}

/// 區塊是否用到共用依賴裡的 crate：程式裡出現 `<crate>::`（連字號換成底線），而且前面
/// 不是識別字或 `::`。誤判（例如註解提到 `rand::`）只會多走一次批次編譯，不影響結果；
/// 清單外的 crate 不會被認出來，交給 clippy-driver 報「找不到 crate」而失敗。
fn uses_crates(code: &str) -> bool {
    toml_keys(DEPS_TOML, "dependencies").iter().any(|name| {
        let root = format!("{}::", name.replace('-', "_"));
        code.match_indices(&root).any(|(i, _)| {
            !code[..i].ends_with(|c: char| c.is_alphanumeric() || c == '_' || c == ':')
        })
    })
}

/// 寫入暫存檔並檢查 rustfmt。回傳 `None` 表示 `ignore`。
fn prepare<'a>(block: &'a Block, tmp: &Path) -> Result<Option<Job<'a>>, String> {
    let opts = parse_opts(&block.info)?;
    if opts.ignore {
        return Ok(None);
    }
    let dir = tmp.join(NEXT_DIR.fetch_add(1, Ordering::Relaxed).to_string());
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let src = dir.join("main.rs");
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
    Ok(Some(Job { block, opts, dir }))
}

/// 只用 std 的區塊：單檔編譯。
fn compile_std(job: &Job) -> Result<Built, String> {
    let exe = job.dir.join("main");
    let mut cmd = Command::new("clippy-driver");
    cmd.args(["--edition", "2024", "-o"])
        .arg(&exe)
        .arg(job.dir.join("main.rs"));
    if !job.opts.warn {
        cmd.args(["-D", "warnings"]);
    }
    if job.opts.test {
        cmd.arg("--test");
    }
    for lint in clippy_allow() {
        cmd.arg("-A").arg(format!("clippy::{lint}"));
    }
    let out = cmd
        .output()
        .map_err(|e| format!("無法執行 clippy-driver：{e}"))?;
    Ok(Built {
        ok: out.status.success(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        exe,
    })
}

/// 依標註判定編譯結果，通過就執行程式並比對結束碼。
fn finish(job: &Job, built: &Built) -> Result<(), String> {
    let opts = &job.opts;
    let stderr = &built.stderr;
    if let Some(code) = &opts.compile_fail {
        if built.ok {
            return Err("標了 compile_fail，卻編譯成功".into());
        }
        return match code.as_deref() {
            Some(code) if stderr.contains(&format!("error[{code}]")) => Ok(()),
            Some(code) => Err(format!("預期錯誤碼 {code}，實際錯誤：\n{stderr}")),
            None if stderr.contains("error[E") => Err(format!(
                "錯誤有錯誤碼，請寫成 compile_fail=E....：\n{stderr}"
            )),
            None => Ok(()),
        };
    }
    if !built.ok {
        return Err(format!("編譯失敗：\n{stderr}"));
    }
    if opts.no_run {
        return Ok(());
    }

    let (code, output) = run_limited(
        &mut Command::new(&built.exe),
        &job.dir,
        opts.stdin.as_deref().unwrap_or_default(),
        RUN_TIMEOUT,
    )?;
    if code != opts.exit {
        return Err(format!(
            "結束碼是 {code}，預期 {}。輸出：\n{output}",
            opts.exit
        ));
    }
    Ok(())
}

/// 執行 `cmd`（stdin 餵 `input`），超過 `limit` 就結束它。stdout 與 stderr 先收進 `scratch` 底下的
/// 檔案；回傳 (結束碼, stdout 加 stderr)。命令自成一個行程群組，逾時時整組結束，所以 `cargo run`
/// 啟動的程式也不會留下來。
///
/// shortcut: 行程群組只支援 unix（用 `kill` 指令結束），要支援 Windows 時再換做法。
fn run_limited(
    cmd: &mut Command,
    scratch: &Path,
    input: &str,
    limit: Duration,
) -> Result<(i32, String), String> {
    let (out_path, err_path) = (scratch.join("stdout"), scratch.join("stderr"));
    let mut child = cmd
        .process_group(0)
        .stdin(Stdio::piped())
        .stdout(File::create(&out_path).map_err(|e| e.to_string())?)
        .stderr(File::create(&err_path).map_err(|e| e.to_string())?)
        .spawn()
        .map_err(|e| format!("無法執行 {:?}：{e}", cmd.get_program()))?;
    // 先寫完 stdin 再關掉，程式讀到 EOF 才不會卡住；不讀 stdin 的程式可能已結束，忽略寫入錯誤。
    if let Some(mut stdin) = child.stdin.take() {
        let _ = std::io::Write::write_all(&mut stdin, input.as_bytes());
    }
    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            break status;
        }
        if start.elapsed() > limit {
            let group = format!("-{}", child.id());
            let _ = Command::new("kill")
                .args(["-s", "KILL", "--", &group])
                .status();
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!("執行超過 {} 秒", limit.as_secs()));
        }
        thread::sleep(Duration::from_millis(20));
    };
    let output = fs::read_to_string(&out_path).unwrap_or_default()
        + &fs::read_to_string(&err_path).unwrap_or_default();
    Ok((status.code().unwrap_or(-1), output))
}

/// 在共用依賴 package 裡跑 cargo：clippy-driver 當工作區 wrapper，所以只有區塊本身會被 lint，
/// 依賴照一般方式編譯（才能快取）。回傳 (是否成功, stderr)。
fn cargo(pkg: &Path, args: &[&str]) -> Result<(bool, String), String> {
    let out = Command::new("cargo")
        .args(args)
        .args(["--locked", "--color", "never"])
        .current_dir(pkg)
        .env("CARGO_TARGET_DIR", DEPS_TARGET)
        .env("RUSTC_WORKSPACE_WRAPPER", "clippy-driver")
        .output()
        .map_err(|e| format!("無法執行 cargo：{e}"))?;
    Ok((
        out.status.success(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    ))
}

/// 產生 package：共用的 Cargo.toml 與 Cargo.lock，加上 `src/bin/<名稱>.rs`。
fn write_package(dir: &Path, bins: &[(String, String)]) -> Result<(), String> {
    let io = |e: std::io::Error| e.to_string();
    let _ = fs::remove_dir_all(dir.join("src"));
    fs::create_dir_all(dir.join("src/bin")).map_err(io)?;
    fs::write(dir.join("Cargo.toml"), DEPS_TOML).map_err(io)?;
    fs::write(dir.join("Cargo.lock"), DEPS_LOCK).map_err(io)?;
    for (name, code) in bins {
        fs::write(dir.join("src/bin").join(format!("{name}.rs")), code).map_err(io)?;
    }
    Ok(())
}

/// 放進 package 的程式。`warn` 區塊在第一行前面加上 allow，行號不變。
fn bin_source(job: &Job) -> String {
    if job.opts.warn {
        format!("#![allow(warnings)] {}", job.block.code)
    } else {
        job.block.code.clone()
    }
}

/// cargo 的錯誤輸出中，屬於這個 bin 的段落；找不到就整份回傳（例如依賴或網路出問題）。
fn diagnostics_for(stderr: &str, bin: &str) -> String {
    let (src, tag) = (format!("src/bin/{bin}.rs"), format!("(bin \"{bin}\""));
    // 一則診斷從 `error` 或 `warning` 開頭的行開始，後面縮排與 help、note 的行都算同一則。
    let mut chunks: Vec<String> = Vec::new();
    for line in stderr.lines() {
        if chunks.is_empty() || line.starts_with("error") || line.starts_with("warning") {
            chunks.push(String::new());
        }
        let chunk = chunks.last_mut().expect("上面保證至少有一段");
        chunk.push_str(line);
        chunk.push('\n');
    }
    let own: String = chunks
        .iter()
        .filter(|chunk| chunk.contains(&src) || chunk.contains(&tag))
        .map(String::as_str)
        .collect();
    if own.is_empty() {
        stderr.to_string()
    } else {
        own
    }
}

/// 一次批次編譯所有一般區塊。`--keep-going`：一個區塊失敗，其他區塊照常編完。
fn build_batch(jobs: &[Job]) -> Result<Vec<Built>, String> {
    if jobs.is_empty() {
        return Ok(Vec::new());
    }
    let pkg = Path::new(DEPS_TARGET).join("pkg");
    let debug = Path::new(DEPS_TARGET).join("debug");
    let names: Vec<String> = (0..jobs.len()).map(|i| format!("b{i}")).collect();
    let bins: Vec<(String, String)> = names
        .iter()
        .zip(jobs)
        .map(|(name, job)| (name.clone(), bin_source(job)))
        .collect();
    write_package(&pkg, &bins)?;
    // 先刪掉上次留下的可執行檔，之後有檔案才代表這次編譯成功。
    for name in &names {
        let _ = fs::remove_file(debug.join(name));
    }
    let mut args = vec!["build", "--keep-going"];
    for name in &names {
        args.extend(["--bin", name]);
    }
    let (_, stderr) = cargo(&pkg, &args)?;
    Ok(names
        .iter()
        .map(|name| {
            let exe = debug.join(name);
            let ok = exe.exists();
            let stderr = if ok {
                String::new()
            } else {
                diagnostics_for(&stderr, name)
            };
            Built { ok, stderr, exe }
        })
        .collect())
}

/// `compile_fail` 與 `test` 區塊：各自放進獨立的 package 單獨編譯，不影響批次。
fn build_solo(job: &Job, index: usize) -> Result<Built, String> {
    let pkg = Path::new(DEPS_TARGET).join("solo");
    let name = format!("s{index}");
    write_package(&pkg, &[(name.clone(), bin_source(job))])?;
    let (ok, stderr) = if job.opts.test {
        cargo(&pkg, &["test", "--no-run", "--bin", &name])?
    } else {
        cargo(&pkg, &["build", "--bin", &name])?
    };
    let mut exe = Path::new(DEPS_TARGET).join("debug").join(&name);
    if ok && job.opts.test {
        // cargo 會印出「Executable unittests src/bin/s0.rs (路徑)」。
        let line = stderr
            .lines()
            .find(|l| l.contains("Executable"))
            .ok_or("找不到測試的可執行檔")?;
        let path = line
            .rsplit_once('(')
            .and_then(|(_, rest)| rest.strip_suffix(')'));
        exe = pkg.join(path.ok_or("看不懂測試可執行檔的路徑")?);
    }
    Ok(Built { ok, stderr, exe })
}

/// 用到 crate 的區塊：一般區塊批次編譯、`compile_fail` 與 `test` 區塊逐一編譯，再逐一執行。
fn check_crates<'a>(jobs: Vec<Job<'a>>, fail: &(impl Fn(&'a Block, String) + Sync)) {
    if jobs.is_empty() {
        return;
    }
    let _guard = CARGO_LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    let (solo, batch): (Vec<_>, Vec<_>) = jobs
        .into_iter()
        .partition(|job| job.opts.compile_fail.is_some() || job.opts.test);
    let mut ready = Vec::new();
    match build_batch(&batch) {
        Ok(built) => ready.extend(batch.into_iter().zip(built)),
        Err(msg) => batch.iter().for_each(|job| fail(job.block, msg.clone())),
    }
    for (i, job) in solo.into_iter().enumerate() {
        match build_solo(&job, i) {
            Ok(built) => ready.push((job, built)),
            Err(msg) => fail(job.block, msg),
        }
    }
    par_each(&ready, |(job, built)| {
        if let Err(msg) = finish(job, built) {
            fail(job.block, msg);
        }
    });
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

/// 依 CPU 核心數平行處理每個項目。
fn par_each<'a, T: Sync>(items: &'a [T], f: impl Fn(&'a T) + Sync) {
    let next = AtomicUsize::new(0);
    let workers = thread::available_parallelism().map_or(4, |n| n.get());
    thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| {
                while let Some(item) = items.get(next.fetch_add(1, Ordering::Relaxed)) {
                    f(item);
                }
            });
        }
    });
}

/// 平行檢查所有區塊，回傳失敗的 (區塊, 原因)，依檔名與行號排序。
fn check_all(blocks: &[Block]) -> Vec<(&Block, String)> {
    // 每次呼叫各用一個目錄：測試會同時呼叫好幾次，結束時各自刪掉自己的目錄。
    let tmp = env::temp_dir().join(format!(
        "checkcode-{}-{}",
        std::process::id(),
        NEXT_DIR.fetch_add(1, Ordering::Relaxed)
    ));
    let failures = Mutex::new(Vec::new());
    let fail = |block, msg| failures.lock().unwrap().push((block, msg));
    let crate_jobs = Mutex::new(Vec::new());
    par_each(blocks, |block| {
        let result = prepare(block, &tmp).and_then(|job| match job {
            None => Ok(()),
            Some(job) if uses_crates(&block.code) => {
                crate_jobs.lock().unwrap().push(job);
                Ok(())
            }
            Some(job) => finish(&job, &compile_std(&job)?),
        });
        if let Err(msg) = result {
            fail(block, msg);
        }
    });
    check_crates(crate_jobs.into_inner().unwrap(), &fail);
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
    let mut projects = Vec::new();
    for file in &files {
        match fs::read_to_string(file) {
            Ok(text) => {
                blocks.extend(extract_blocks(file, &text));
                projects.extend(extract_project(file, &text));
            }
            Err(e) => {
                eprintln!("{}: {e}", file.display());
                return ExitCode::from(2);
            }
        }
    }

    let mut failures: Vec<Failure> = check_all(&blocks)
        .into_iter()
        .map(|(block, msg)| (block.file.clone(), block.line, msg))
        .collect();
    failures.extend(check_projects(&projects));
    failures.sort_by(|a, b| (&a.0, a.1).cmp(&(&b.0, b.1)));
    for (file, line, msg) in &failures {
        println!("{}:{line}: {msg}", file.display());
    }
    println!(
        "{} 個檔案、{} 個 rust 區塊、{} 個專案（{} 個檔案、{} 個檢查點），{} 個失敗",
        files.len(),
        blocks.len(),
        projects.len(),
        projects.iter().map(Project::files).sum::<usize>(),
        projects.iter().map(Project::checkpoints).sum::<usize>(),
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
    use crate::project::unpinned_packages;
    use std::collections::BTreeSet;

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
        let failures = check_all(&blocks);
        for block in &blocks {
            assert!(
                failures.iter().any(|f| f.0.line == block.line),
                "第 {} 行應該失敗卻通過",
                block.line
            );
        }
        // 用到清單內 crate 的區塊如果報「找不到 crate」，代表被誤放進單檔編譯，而不是真的有問題。
        // （刻意混進 reqwest 的區塊除外。）
        for (block, msg) in &failures {
            if uses_crates(&block.code) && !block.code.contains("reqwest") {
                assert!(
                    !msg.contains("unlinked crate"),
                    "第 {} 行失敗的原因不對：{msg}",
                    block.line
                );
            }
        }
    }

    /// 通過與失敗的區塊（含 crate 區塊）放在同一次檢查，各自的判定不能互相影響。
    #[test]
    fn mixed_run_keeps_verdicts_separate() {
        let all: Vec<Block> = blocks_of("pass.md")
            .into_iter()
            .chain(blocks_of("fail.md"))
            .collect();
        let failed: Vec<(&Path, usize)> = check_all(&all)
            .iter()
            .map(|f| (f.0.file.as_path(), f.0.line))
            .collect();
        for block in &all {
            let should_fail = block.file.ends_with("fail.md");
            assert_eq!(
                failed.contains(&(block.file.as_path(), block.line)),
                should_fail,
                "{}:{} 的判定不對",
                block.file.display(),
                block.line
            );
        }
    }

    fn testdata(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("testdata")
            .join(name)
    }

    fn project_of(name: &str) -> Project {
        let path = testdata(name);
        extract_project(&path, &fs::read_to_string(&path).unwrap()).expect("應該認得出專案")
    }

    /// 檔案 `name` 裡，路徑標籤 `label` 後面那個程式區塊的行號（標籤、空行、區塊開頭）。
    fn block_line_after(name: &str, label: &str) -> usize {
        let text = fs::read_to_string(testdata(name)).unwrap();
        let at = text.lines().position(|l| l == label).expect("找不到標籤");
        at + 3
    }

    /// 失敗專案裡應該失敗的位置：標籤是「輸出」（檢查點），或寫了（故意…）的區塊，後面那個圍欄的行號。
    fn marked_lines(name: &str) -> BTreeSet<usize> {
        let text = fs::read_to_string(testdata(name)).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        (0..lines.len())
            .filter(|&i| {
                let l = lines[i];
                l.starts_with('`')
                    && l.ends_with('：')
                    && (l.contains("的輸出") || l.contains("（故意"))
            })
            .map(|i| {
                i + lines[i + 1..]
                    .iter()
                    .position(|l| l.starts_with("```"))
                    .unwrap()
                    + 2
            })
            .collect()
    }

    #[test]
    fn project_pass_all_pass() {
        let projects: Vec<Project> = [
            "project-pass.md",
            "project-workspace.md",
            "project-crate.md",
        ]
        .map(project_of)
        .into();
        assert!(projects.iter().all(|p| p.checkpoints() >= 1));
        let failures = check_projects(&projects);
        assert!(failures.is_empty(), "{failures:#?}");
    }

    #[test]
    fn project_fail_each_marked_case_fails() {
        let failures = check_projects(&[project_of("project-fail.md")]);
        let failed: BTreeSet<usize> = failures.iter().map(|f| f.1).collect();
        assert_eq!(failed, marked_lines("project-fail.md"), "{failures:#?}");
    }

    /// 拿掉一個 `pub`：失敗要落在檢查點，並指出是哪一集、哪些檔案（連同區塊在該集的行號）。
    #[test]
    fn project_removed_pub_names_episode_and_files() {
        let mut failures = check_projects(&[project_of("project-fail.md")]);
        failures.sort_by_key(|f| f.1);
        let (file, line, msg) = failures.first().expect("應該失敗");
        assert!(file.ends_with("project-fail.md"));
        assert_eq!(*line, *marked_lines("project-fail.md").first().unwrap());
        for label in ["`src/lib.rs`：", "`src/main.rs`："] {
            let name = label.trim_matches(|c| c == '`' || c == '：');
            let line = block_line_after("project-fail.md", label);
            assert!(msg.contains(&format!("{name}（第 {line} 行）")), "{msg}");
        }
    }

    /// 結束碼的失敗要說對原因：編譯失敗不是程式的 101，標籤寫錯要指出標籤。
    #[test]
    fn exit_label_failures_say_why() {
        let failures = check_projects(&[project_of("project-fail.md")]);
        let reason = |label: &str| {
            let line = block_line_after("project-fail.md", label);
            &failures.iter().find(|f| f.1 == line).expect("應該失敗").2
        };
        assert!(reason("`cargo run` 的輸出（省略編譯進度，結束碼 101）：").contains("編譯失敗"));
        for label in [
            "`cargo run` 的輸出（省略編譯進度，結束碼二）：",
            "`cargo run` 的輸出（省略編譯進度，結束碼：0）：",
        ] {
            assert!(reason(label).contains("結束碼 N"), "{label}");
        }
    }

    /// 通過與失敗的專案放在同一次檢查，各自的判定不能互相影響。
    #[test]
    fn mixed_projects_keep_verdicts_separate() {
        let projects = ["project-fail.md", "project-crate.md", "project-pass.md"].map(project_of);
        let failed: Vec<(PathBuf, usize)> = check_projects(&projects)
            .into_iter()
            .map(|f| (f.0, f.1))
            .collect();
        let expected: Vec<(PathBuf, usize)> = marked_lines("project-fail.md")
            .into_iter()
            .map(|line| (testdata("project-fail.md"), line))
            .collect();
        assert_eq!(
            failed.iter().collect::<BTreeSet<_>>(),
            expected.iter().collect::<BTreeSet<_>>()
        );
    }

    #[test]
    fn packages_outside_deps_lock_are_unpinned() {
        let registry = "source = \"registry+https://github.com/rust-lang/crates.io-index\"";
        let lock = format!(
            "[[package]]\nname = \"serde\"\nversion = \"0.0.1\"\n{registry}\n\n[[package]]\nname = \"teashop\"\nversion = \"0.1.0\"\n"
        );
        assert_eq!(unpinned_packages(&lock), ["serde 0.0.1"]);
        assert!(unpinned_packages(DEPS_LOCK).is_empty());
    }

    #[test]
    fn deps_clippy_lints_match_allow_list() {
        assert_eq!(
            toml_keys(DEPS_TOML, "lints.clippy"),
            clippy_allow().collect::<Vec<_>>()
        );
    }
}
