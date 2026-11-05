//! 多檔案專案：一集（一個 .md 檔）就是一個專案的時間線，由上往下重播。
//!
//! - 「`路徑`：」加程式區塊：專案裡這個檔案現在的內容（同一路徑再出現就覆蓋，像讀者一路在改它）。
//! - 「`cargo …` 的輸出：」加 `text` 區塊：檢查點。這一刻把所有檔案寫進暫時專案，執行那條指令，
//!   檢查成功或失敗跟書上貼的相符；cargo 輸出的文字不逐字比對（雜湊、秒數、路徑每次都不同）。
//! - 「`cargo fmt` 之後的 `路徑`：」加 `rust` 區塊：前一個版本排版亂掉的示範，檢查 rustfmt 真的排成這樣。
//!
//! 規則全文與理由見 `docs/style-guide.md` 的「多檔案專案」。

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::sync::atomic::Ordering;
use std::sync::{Mutex, PoisonError};
use std::time::Duration;

use crate::{
    CARGO_LOCK, DEPS_LOCK, DEPS_TARGET, DEPS_TOML, Failure, NEXT_DIR, RUN_TIMEOUT, clippy_allow,
    par_each, parse_opts, run_limited, toml_keys,
};

/// 書中沒寫 `Cargo.toml` 時用的預設設定。
const DEFAULT_MANIFEST: &str =
    "[package]\nname = \"teashop\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n";

pub struct Project {
    file: PathBuf,
    steps: Vec<Step>,
}

enum Step {
    File {
        path: String,
        code: String,
        line: usize,
        warn: bool,
        /// `rust,ignore`：單檔檢查會跳過，改在這裡檢查排版。
        ignored: bool,
        /// 後面緊接著「`cargo fmt` 之後」的標籤：這個區塊刻意排版亂掉，不檢查排版。
        exempt: bool,
    },
    FmtAfter {
        path: String,
        code: String,
        line: usize,
    },
    Check {
        cmd: Vec<String>,
        shown: String,
        line: usize,
    },
}

enum Label {
    File(String),
    FmtAfter(String),
    Out(Vec<String>),
}

struct Entry {
    code: String,
    line: usize,
    warn: bool,
}

impl Project {
    /// 專案裡有幾個不同路徑的檔案。
    pub fn files(&self) -> usize {
        let paths: BTreeSet<&str> = self
            .steps
            .iter()
            .filter_map(|step| match step {
                Step::File { path, .. } => Some(path.as_str()),
                _ => None,
            })
            .collect();
        paths.len()
    }

    /// 會實際執行的檢查點數量。
    pub fn checkpoints(&self) -> usize {
        self.steps
            .iter()
            .filter(|step| matches!(step, Step::Check { cmd, .. } if supported(cmd)))
            .count()
    }
}

/// 驗證的指令。`new`、`add`、`fmt` 這類改檔案、不編譯的指令不驗證。
fn supported(cmd: &[String]) -> bool {
    cmd.get(1).is_some_and(|sub| {
        matches!(
            sub.as_str(),
            "run" | "test" | "check" | "build" | "doc" | "clippy"
        )
    })
}

/// 路徑：只有英數與 `_ . / -`，不能是絕對路徑或含 `..`，檔名要有英文字母的副檔名。
fn valid_path(path: &str) -> bool {
    path.chars()
        .all(|c| c.is_ascii_alphanumeric() || "_./-".contains(c))
        && !path.starts_with('/')
        && path.split('/').all(|part| part != "..")
        && path
            .rsplit('/')
            .next()
            .and_then(|name| name.rsplit_once('.'))
            .is_some_and(|(_, ext)| !ext.is_empty() && ext.chars().all(|c| c.is_ascii_alphabetic()))
}

/// 標籤整行只有一個行內程式碼加全形冒號，句尾可以多一段（說明）。
fn parse_label(line: &str) -> Option<Label> {
    let line = line.strip_suffix('：')?;
    let line = match line.strip_suffix('）') {
        Some(rest) => rest.rsplit_once('（')?.0,
        None => line,
    };
    if let Some(path) = line
        .strip_prefix("`cargo fmt` 之後的 `")
        .and_then(|rest| rest.strip_suffix('`'))
    {
        return valid_path(path).then(|| Label::FmtAfter(path.to_string()));
    }
    if let Some(cmd) = line
        .strip_suffix("` 的輸出")
        .and_then(|rest| rest.strip_prefix('`'))
    {
        return cmd
            .starts_with("cargo ")
            .then(|| Label::Out(cmd.split_whitespace().map(str::to_string).collect()));
    }
    let path = line.strip_prefix('`')?.strip_suffix('`')?;
    valid_path(path).then(|| Label::File(path.to_string()))
}

/// 圍欄開頭：(反引號數, info string)。四個反引號的外層區塊裡面可以有三個反引號。
fn fence_open(raw: &str) -> Option<(usize, &str)> {
    let text = raw.trim_start();
    let n = text.len() - text.trim_start_matches('`').len();
    (n >= 3).then(|| (n, text[n..].trim()))
}

fn fence_close(raw: &str, n: usize) -> bool {
    let text = raw.trim();
    text.len() >= n && text.chars().all(|c| c == '`')
}

fn step_of(label: Label, info: &str, code: String, line: usize) -> Option<Step> {
    let lang = info.split(',').next().unwrap_or_default();
    match label {
        Label::File(path) => {
            // 標註拼錯由單檔檢查報告，這裡只讀 warn 與 ignore。
            let opts = parse_opts(info).ok().filter(|_| lang == "rust");
            let opts = opts.unwrap_or_default();
            Some(Step::File {
                ignored: lang == "rust" && opts.ignore && path.ends_with(".rs"),
                warn: opts.warn,
                exempt: false,
                path,
                code,
                line,
            })
        }
        Label::FmtAfter(path) => (lang == "rust").then_some(Step::FmtAfter { path, code, line }),
        Label::Out(cmd) => (lang == "text").then_some(Step::Check {
            cmd,
            shown: code,
            line,
        }),
    }
}

/// 掃出一集的專案時間線；沒有 `.rs` 檔也沒有檢查點的集不算專案。
pub fn extract_project(file: &Path, text: &str) -> Option<Project> {
    let mut steps = Vec::new();
    let mut label = None;
    let mut lines = text.lines().enumerate();
    while let Some((i, raw)) = lines.next() {
        if let Some((n, info)) = fence_open(raw) {
            let mut code = String::new();
            for (_, body) in lines.by_ref() {
                if fence_close(body, n) {
                    break;
                }
                code.push_str(body);
                code.push('\n');
            }
            steps.extend(label.take().and_then(|l| step_of(l, info, code, i + 1)));
        } else if let Some(l) = parse_label(raw.trim()) {
            label = Some(l);
        } else if !raw.trim().is_empty() {
            label = None;
        }
    }
    let exempted: Vec<usize> = steps
        .windows(2)
        .enumerate()
        .filter_map(|(i, pair)| match pair {
            [Step::File { path, .. }, Step::FmtAfter { path: after, .. }] if path == after => {
                Some(i)
            }
            _ => None,
        })
        .collect();
    for i in exempted {
        if let Step::File { exempt, .. } = &mut steps[i] {
            *exempt = true;
        }
    }
    let is_project = steps.iter().any(|step| match step {
        Step::File { path, .. } => path.ends_with(".rs"),
        _ => true,
    });
    is_project.then_some(Project {
        file: file.to_path_buf(),
        steps,
    })
}

/// 平行驗證每一集的專案（用到 pinned crate 的檢查點會輪流使用共用的 target）。
pub fn check_projects(projects: &[Project]) -> Vec<Failure> {
    let tmp = env::temp_dir().join(format!(
        "checkcode-proj-{}-{}",
        process::id(),
        NEXT_DIR.fetch_add(1, Ordering::Relaxed)
    ));
    let failures = Mutex::new(Vec::new());
    par_each(projects, |project| {
        let dir = tmp.join(NEXT_DIR.fetch_add(1, Ordering::Relaxed).to_string());
        for (line, msg) in project.replay(&dir) {
            failures
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push((project.file.clone(), line, msg));
        }
    });
    let _ = fs::remove_dir_all(&tmp);
    failures
        .into_inner()
        .unwrap_or_else(PoisonError::into_inner)
}

impl Project {
    /// 由上往下重播，回傳 (行號, 原因)。
    fn replay(&self, dir: &Path) -> Vec<(usize, String)> {
        let mut bad = Vec::new();
        let mut files: BTreeMap<String, Entry> = BTreeMap::new();
        // 最後一個還沒被任何檢查點編譯過的 `rust,ignore` 檔案。
        let mut never_built = None;
        for step in &self.steps {
            match step {
                Step::File {
                    path,
                    code,
                    line,
                    warn,
                    ignored,
                    exempt,
                } => {
                    if *ignored {
                        never_built = Some((path, *line));
                        if !exempt && let Err(msg) = fmt_check(path, code) {
                            bad.push((*line, msg));
                        }
                    }
                    let entry = Entry {
                        code: code.clone(),
                        line: *line,
                        warn: *warn,
                    };
                    files.insert(path.clone(), entry);
                }
                Step::FmtAfter { path, code, line } => {
                    match files.get(path).map(|before| rustfmt(&before.code)) {
                        None => {
                            bad.push((*line, format!("{path} 之前沒有出現過，沒有東西可以排版")))
                        }
                        Some(Err(msg)) => bad.push((*line, msg)),
                        Some(Ok(formatted)) if formatted != *code => bad.push((
                            *line,
                            format!("rustfmt 把 {path} 排成這樣，跟書上不同：\n{formatted}"),
                        )),
                        Some(Ok(_)) => {}
                    }
                    let entry = Entry {
                        code: code.clone(),
                        line: *line,
                        warn: false,
                    };
                    files.insert(path.clone(), entry);
                }
                Step::Check { cmd, shown, line } if supported(cmd) => {
                    never_built = None;
                    if let Err(msg) = checkpoint(dir, &files, cmd, shown) {
                        bad.push((*line, msg));
                    }
                }
                Step::Check { .. } => {}
            }
        }
        if let Some((path, line)) = never_built {
            bad.push((
                line,
                format!(
                    "{path} 之後沒有任何 `cargo …` 的輸出區塊，專案沒有被編譯過，這個檔案沒被驗證"
                ),
            ));
        }
        bad
    }
}

/// 用 stdin 排版（不會去找 `mod x;` 對應的檔案）。不用 `--check`：stdin 模式有差異時它也回傳 0。
fn rustfmt(code: &str) -> Result<String, String> {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout"])
        .current_dir(env::temp_dir())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("無法執行 rustfmt：{e}"))?;
    // 程式很短，寫完就關；rustfmt 讀完整份才會開始輸出，不會互相卡住。
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(code.as_bytes());
    }
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(format!(
            "rustfmt 失敗：\n{}",
            String::from_utf8_lossy(&out.stderr)
        ))
    }
}

fn fmt_check(path: &str, code: &str) -> Result<(), String> {
    let formatted = rustfmt(code)?;
    if formatted == code {
        Ok(())
    } else {
        Err(format!(
            "{path} 沒有通過 rustfmt，排版後應該是：\n{formatted}"
        ))
    }
}

/// Cargo.lock 裡來自 registry 的 package，每個寫成 `名稱 版本`。
fn registry_packages(lock: &str) -> Vec<String> {
    lock.split("[[package]]")
        .skip(1)
        .filter_map(|stanza| {
            let field = |key: &str| {
                stanza
                    .lines()
                    .find_map(|l| l.strip_prefix(key))
                    .map(|v| v.trim_matches('"'))
            };
            field("source = ")?;
            Some(format!("{} {}", field("name = ")?, field("version = ")?))
        })
        .collect()
}

/// 專案的 Cargo.lock 裡，不在 `deps/Cargo.lock` 的 crate 或版本。
pub fn unpinned_packages(lock: &str) -> Vec<String> {
    let pinned = registry_packages(DEPS_LOCK);
    registry_packages(lock)
        .into_iter()
        .filter(|package| !pinned.contains(package))
        .collect()
}

/// 有沒有哪份 `Cargo.toml` 用到共用依賴清單裡的 crate。
fn uses_pinned_crates(files: &BTreeMap<String, Entry>) -> bool {
    let pinned = toml_keys(DEPS_TOML, "dependencies");
    files
        .iter()
        .filter(|(path, _)| path.ends_with("Cargo.toml"))
        .any(|(_, entry)| {
            toml_keys(&entry.code, "dependencies")
                .iter()
                .any(|name| pinned.contains(name))
        })
}

fn write_project(
    proj: &Path,
    files: &BTreeMap<String, Entry>,
    seed_lock: bool,
) -> Result<(), String> {
    let io = |e: std::io::Error| e.to_string();
    let _ = fs::remove_dir_all(proj);
    fs::create_dir_all(proj).map_err(io)?;
    if seed_lock {
        fs::write(proj.join("Cargo.lock"), DEPS_LOCK).map_err(io)?;
    }
    if !files.contains_key("Cargo.toml") {
        fs::write(proj.join("Cargo.toml"), DEFAULT_MANIFEST).map_err(io)?;
    }
    for (path, entry) in files {
        let full = proj.join(path);
        fs::create_dir_all(full.parent().unwrap_or(proj)).map_err(io)?;
        fs::write(full, &entry.code).map_err(io)?;
    }
    Ok(())
}

/// 在專案裡執行 cargo。clippy-driver 當工作區 wrapper，所以只有專案自己的 crate 被 lint
/// （跟批次編譯一樣），lint 設定走 `CLIPPY_ARGS`：書中的 `Cargo.toml` 不必（也不該）寫 `[lints]`。
fn cargo(
    args: &[&str],
    proj: &Path,
    target: &Path,
    warn: bool,
    limit: Duration,
) -> Result<(bool, String), String> {
    let mut lints = Vec::new();
    if !warn {
        lints.extend(["-D".to_string(), "warnings".to_string()]);
    }
    for lint in clippy_allow() {
        lints.extend(["-A".to_string(), format!("clippy::{lint}")]);
    }
    let mut cmd = Command::new("cargo");
    cmd.args(args)
        .current_dir(proj)
        .env("CARGO_TARGET_DIR", target)
        .env("CARGO_TERM_COLOR", "never")
        .env("RUSTC_WORKSPACE_WRAPPER", "clippy-driver")
        .env("CLIPPY_ARGS", lints.join("__CLIPPY_HACKERY__"))
        // 跟 deps/Cargo.toml 的 [profile.dev] 一樣，共用的 target 裡的 crate 才不必重編。
        .env("CARGO_PROFILE_DEV_DEBUG", "0")
        .env("CARGO_PROFILE_DEV_STRIP", "none");
    let scratch = proj.parent().unwrap_or(proj);
    let (code, text) = run_limited(&mut cmd, scratch, "", limit)?;
    Ok((code == 0, text))
}

/// 這個檢查點的結果：書上貼了 `error…` 或 `test result: FAILED` 就必須失敗（而且書上的錯誤碼都要出現），
/// 否則必須成功。
fn checkpoint(
    dir: &Path,
    files: &BTreeMap<String, Entry>,
    cmd: &[String],
    shown: &str,
) -> Result<(), String> {
    let crates = uses_pinned_crates(files);
    // 共用的 target 同一時間只讓一個 cargo 用（跟批次編譯一樣）。
    let _guard = crates.then(|| CARGO_LOCK.lock().unwrap_or_else(PoisonError::into_inner));
    let proj = dir.join("proj");
    write_project(&proj, files, crates)?;
    let target = if crates {
        PathBuf::from(DEPS_TARGET)
    } else {
        dir.join("target")
    };
    let warn = files.values().any(|entry| entry.warn);

    // 先編譯（不設時限，編譯可能很久；`doc` 本身不編譯程式，先 `check` 才有 lint），`run`、`test`、
    // `doc` 再用書上的指令執行（30 秒內）。
    let args: Vec<&str> = cmd[1..].iter().map(String::as_str).collect();
    let flags_end = args.iter().position(|a| *a == "--").unwrap_or(args.len());
    let flags = &args[1..flags_end];
    let sub = args[0];
    let mut build = match sub {
        "run" => vec!["build"],
        "test" => vec!["test", "--no-run"],
        "clippy" | "doc" => vec!["check"],
        other => vec![other],
    };
    build.extend(flags);
    let (mut ok, mut text) = cargo(&build, &proj, &target, warn, Duration::MAX)?;
    if ok && matches!(sub, "run" | "test" | "doc") {
        let (run_ok, run_text) = cargo(&args, &proj, &target, warn, RUN_TIMEOUT)?;
        ok = run_ok;
        text.push_str(&run_text);
    }

    let command = cmd.join(" ");
    if let Ok(lock) = fs::read_to_string(proj.join("Cargo.lock")) {
        let unpinned = unpinned_packages(&lock);
        if !unpinned.is_empty() {
            return Err(format!(
                "`{command}` 用到 deps/Cargo.lock 沒有的 crate 或版本：{}",
                unpinned.join("、")
            ));
        }
    }
    let expect_fail = shown.lines().any(|l| l.trim_start().starts_with("error"))
        || shown.contains("test result: FAILED");
    match (expect_fail, ok) {
        (false, true) => Ok(()),
        (false, false) => Err(format!(
            "`{command}` 書上貼的是成功，實際卻失敗。{}\n{text}",
            blamed_files(&text, files)
        )),
        (true, true) => Err(format!("`{command}` 書上貼的是錯誤，實際卻成功")),
        (true, false) => {
            let missing = error_codes(shown)
                .into_iter()
                .find(|code| !text.contains(&format!("error[{code}]")));
            match missing {
                None => Ok(()),
                Some(code) => Err(format!(
                    "`{command}` 書上貼的錯誤碼是 {code}，實際沒有。{}\n{text}",
                    blamed_files(&text, files)
                )),
            }
        }
    }
}

/// 文字裡所有 `error[E….]` 的錯誤碼。
fn error_codes(text: &str) -> Vec<&str> {
    text.match_indices("error[")
        .filter_map(|(i, _)| text[i + 6..].split_once(']').map(|(code, _)| code))
        .collect()
}

/// cargo 的診斷（`--> 路徑:行:欄`）指到哪些專案檔案，連同該檔案的區塊在這一集的行號。
fn blamed_files(output: &str, files: &BTreeMap<String, Entry>) -> String {
    let mut seen: Vec<&str> = Vec::new();
    for line in output.lines() {
        if let Some(rest) = line.trim_start().strip_prefix("--> ")
            && let Some((path, _)) = rest.split_once(':')
            && files.contains_key(path)
            && !seen.contains(&path)
        {
            seen.push(path);
        }
    }
    if seen.is_empty() {
        return String::new();
    }
    let named: Vec<String> = seen
        .iter()
        .map(|path| format!("{path}（第 {} 行）", files[*path].line))
        .collect();
    format!("有問題的檔案：{}。", named.join("、"))
}
