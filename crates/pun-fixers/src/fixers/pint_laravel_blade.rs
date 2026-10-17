//! Port of Pint's `App\Fixers\LaravelBlade\Fixer` (+ `App\BladeFormatter`, `App\Support\Prettier`):
//! formats `*.blade.php` files with prettier (prettier-plugin-blade, resolved from the project's
//! node_modules like Pint does) wrapped in Pint's pre/post formatters.
#[path = "pint_laravel_blade/collapse_single_attribute.rs"]
mod collapse_single_attribute;
#[path = "pint_laravel_blade/directive_trailing_commas.rs"]
mod directive_trailing_commas;
#[path = "pint_laravel_blade/embedded_blade_masker.rs"]
mod embedded_blade_masker;
#[path = "pint_laravel_blade/line_formatters.rs"]
mod line_formatters;
#[path = "pint_laravel_blade/not_operator_spacing.rs"]
mod not_operator_spacing;
#[path = "pint_laravel_blade/php.rs"]
mod php;
#[path = "pint_laravel_blade/php_block_formatting.rs"]
mod php_block_formatting;

use anyhow::{Result, bail};
use line_formatters::*;
use pun_format::Fixer;
use pun_tokens::Tokens;
use std::cell::RefCell;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::time::Duration;

const WORKER: &str = include_str!("pint_laravel_blade/worker.cjs");
const PRETTIERRC: &str = include_str!("pint_laravel_blade/prettierrc.json");
const WORKER_IDLE_TIMEOUT: Duration = Duration::from_secs(30);

struct LaravelBladeFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(LaravelBladeFixer)
}

impl Fixer for LaravelBladeFixer {
    fn name(&self) -> &'static str {
        "Pint/laravel_blade"
    }
    fn priority(&self) -> i32 {
        -2
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn fix(&self, file: &Path, tokens: &mut Tokens) {
        // SplFileInfo::getRealPath()
        let path = std::fs::canonicalize(file).unwrap_or_else(|_| file.to_path_buf());
        let path_str = path.to_string_lossy().into_owned();
        if !path_str.ends_with(".blade.php") {
            return;
        }
        let content = tokens.generate_code();
        if is_ignored(&path_str) {
            return;
        }
        let content = match format(&path_str, &content) {
            Ok(c) => c,
            Err(e) => {
                // PrettierException: the runner reports the file as errored and leaves it untouched.
                eprintln!("Pint/laravel_blade: {}: {e}", path.display());
                return;
            }
        };
        // Tokens::setCode() rejects an empty string, so clear the tokens directly.
        if content.is_empty() {
            for index in 0..tokens.count() {
                tokens.clear_at(index);
            }
            tokens.clear_empty_tokens();
            return;
        }
        tokens.set_code(&content);
    }
}

/// Ignorables: Envoy, BoostGuidelines, EmailView.
fn is_ignored(path: &str) -> bool {
    let basename = path.rsplit('/').next().unwrap_or(path);
    let path = path.replace('\\', "/");
    matches!(basename, "envoy.blade.php" | "Envoy.blade.php")
        || path.contains("resources/boost/guidelines/")
        || path.contains("resources/views/emails/")
        || path.contains("resources/views/mail/")
}

/// `BladeFormatter::format`: pre-formatters, prettier, then post-formatters in declaration order.
fn format(path: &str, content: &[u8]) -> Result<Vec<u8>> {
    let mut masker = embedded_blade_masker::EmbeddedBladeMasker::default();
    let content = masker.pre_format(content);
    let formatted = prettier_format(path, &content)?;
    let formatted = strip_sensitive_leading_blank_lines::post_format(&formatted);
    let formatted = directive_trailing_commas::post_format(&formatted);
    let formatted = not_operator_spacing::post_format(&formatted);
    let formatted = collapse_short_slots::post_format(&formatted);
    let formatted = collapse_single_attribute::post_format(&formatted);
    let formatted = join_dangling_close_bracket::post_format(&formatted);
    let formatted = join_dangling_open_bracket::post_format(&formatted);
    let formatted = dedent_hugged_terminator::post_format(&formatted);
    let formatted = masker.post_format(&formatted);
    let blocks = php_block_formatting::PhpBlockFormatting::new(
        php_block_formatting::PhpFragmentFormatter::new(crate::analyzers::active_preset::get()),
    );
    Ok(blocks.post_format(&formatted))
}

enum Chunk {
    Out(Vec<u8>),
    Err(Vec<u8>),
    Closed,
}

/// `App\Support\Prettier`: one long-lived `node worker.cjs <projectRoot> <prettierrc.json>` process.
struct Worker {
    child: Child,
    stdin: ChildStdin,
    events: Receiver<Chunk>,
}

thread_local! {
    /// One worker per formatting thread, so Blade files are formatted in parallel (Pint's parallel runner likewise
    /// gives every PHP process its own worker). A worker exits when its stdin closes with the process.
    static WORKER_PROCESS: RefCell<Option<Worker>> = const { RefCell::new(None) };
}

/// The worker script and prettier config, in a temp dir named by their content so every run shares it
/// (a per-process dir would be left behind by each run). Written via rename, so a concurrent run never sees a
/// partial file.
fn resources() -> Result<(PathBuf, PathBuf)> {
    let mut hasher = std::hash::DefaultHasher::new();
    std::hash::Hash::hash(&(WORKER, PRETTIERRC), &mut hasher);
    let hash = std::hash::Hasher::finish(&hasher);
    resources_in(&std::env::temp_dir().join(format!("pun-blade-{hash:016x}")))
}

fn resources_in(dir: &Path) -> Result<(PathBuf, PathBuf)> {
    let (worker, config) = (dir.join("worker.cjs"), dir.join("prettierrc.json"));
    if !worker.exists() {
        std::fs::create_dir_all(dir)?;
        for (path, body) in [(&config, PRETTIERRC), (&worker, WORKER)] {
            let tmp = path.with_extension(std::process::id().to_string());
            std::fs::write(&tmp, body)?;
            std::fs::rename(&tmp, path)?;
        }
    }
    Ok((worker, config))
}

fn start() -> Result<Worker> {
    let project_root = std::env::current_dir()?; // Project::path() is getcwd()
    let (worker, config) = resources()?;
    let mut node = Command::new("node");
    node.arg(&worker)
        .arg(&project_root)
        .arg(&config)
        .current_dir(&project_root);
    spawn(node)
}

fn spawn(mut command: Command) -> Result<Worker> {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let stdin = child.stdin.take().unwrap();
    let (tx, events) = channel();
    for (mut pipe, is_err) in [
        (
            Box::new(child.stdout.take().unwrap()) as Box<dyn Read + Send>,
            false,
        ),
        (Box::new(child.stderr.take().unwrap()), true),
    ] {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let mut buf = [0u8; 65536];
            loop {
                match pipe.read(&mut buf) {
                    Ok(0) | Err(_) => {
                        let _ = tx.send(Chunk::Closed);
                        return;
                    }
                    Ok(n) => {
                        let chunk = buf[..n].to_vec();
                        let _ = tx.send(if is_err {
                            Chunk::Err(chunk)
                        } else {
                            Chunk::Out(chunk)
                        });
                    }
                }
            }
        });
    }
    Ok(Worker {
        child,
        stdin,
        events,
    })
}

const START: &[u8] = b"[PINT_PRETTIER_WORKER_START]";
const END: &[u8] = b"[PINT_PRETTIER_WORKER_END]";

fn prettier_format(path: &str, content: &[u8]) -> Result<Vec<u8>> {
    WORKER_PROCESS.with(|w| {
        let mut guard = w.borrow_mut();
        if guard.is_none() {
            *guard = Some(start()?);
        }
        prettier_format_with(&mut guard, path, content, WORKER_IDLE_TIMEOUT)
    })
}

/// One request to the started worker in `guard`; a failure tears it down (the next file starts a fresh one).
fn prettier_format_with(
    guard: &mut Option<Worker>,
    path: &str,
    content: &[u8],
    idle_timeout: Duration,
) -> Result<Vec<u8>> {
    let worker = guard.as_mut().unwrap();
    // json_encode(..., JSON_INVALID_UTF8_SUBSTITUTE)
    let request = serde_json::json!({ "path": path, "content": String::from_utf8_lossy(content) });
    // A dead worker fails the write and then reports `Closed` below.
    let _ = writeln!(worker.stdin, "{request}").and_then(|_| worker.stdin.flush());

    let mut formatted: Vec<u8> = Vec::new();
    let failure = loop {
        if php::contains(&formatted, END) {
            let body = &formatted[php::strpos(&formatted, START, 0).unwrap() + START.len()..];
            return Ok(body[..php::strpos(body, END, 0).unwrap()].to_vec());
        }
        match worker.events.recv_timeout(idle_timeout) {
            Ok(Chunk::Out(c)) => formatted.extend(c),
            event => break event,
        }
    };
    // Error output, the worker exiting, or silence: collect the rest of stderr (getErrorOutput()), tear the
    // worker down and report the file.
    std::thread::sleep(Duration::from_millis(100));
    let timed_out = matches!(failure, Err(RecvTimeoutError::Timeout));
    let error: Vec<u8> = std::iter::once(failure.ok())
        .flatten()
        .chain(worker.events.try_iter())
        .flat_map(|e| if let Chunk::Err(c) = e { c } else { Vec::new() })
        .collect();
    let mut dead = guard.take().unwrap();
    let _ = dead.child.kill();
    let _ = dead.child.wait();
    let message = if timed_out {
        format!("Laravel Pint's Prettier worker timed out while formatting [{path}].")
    } else if error.is_empty() {
        "Laravel Pint's Prettier worker terminated unexpectedly.".into()
    } else {
        String::from_utf8_lossy(&error).into_owned()
    };
    bail!("{message}")
}

/// Each formatter step against Pint's own class on the same input (tests/blade_formatters.json, from
/// scripts/oracle-blade-formatter.php), for shapes the golden files in tests/blade don't reach.
/// PhpBlockFormatting runs the laravel preset's fixers, so they must be compiled in.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_blade_formatters() {
        let cases: Vec<serde_json::Value> =
            serde_json::from_str(include_str!("../../tests/blade_formatters.json")).unwrap();
        for (i, case) in cases.iter().enumerate() {
            let formatter = case["formatter"].as_str().unwrap();
            let input = case["input"].as_str().unwrap().as_bytes();
            let mut masker = embedded_blade_masker::EmbeddedBladeMasker::default();
            let got = match formatter {
                "StripSensitiveLeadingBlankLines" => {
                    strip_sensitive_leading_blank_lines::post_format(input)
                }
                "DirectiveTrailingCommas" => directive_trailing_commas::post_format(input),
                "NotOperatorSpacing" => not_operator_spacing::post_format(input),
                "CollapseShortSlots" => collapse_short_slots::post_format(input),
                "CollapseSingleAttribute" => collapse_single_attribute::post_format(input),
                "JoinDanglingCloseBracket" => join_dangling_close_bracket::post_format(input),
                "JoinDanglingOpenBracket" => join_dangling_open_bracket::post_format(input),
                "DedentHuggedTerminator" => dedent_hugged_terminator::post_format(input),
                "EmbeddedBladeMasker" => masker.pre_format(input),
                _ => php_block_formatting::PhpBlockFormatting::new(
                    php_block_formatting::PhpFragmentFormatter::new(
                        case.get("preset").map_or("laravel", |p| p.as_str().unwrap()).into(),
                    ),
                )
                .post_format(input),
            };
            assert_eq!(
                String::from_utf8_lossy(&got),
                case["expected"].as_str().unwrap(),
                "case {i} {formatter}"
            );
            if let Some(post_input) = case.get("post_input") {
                let got = masker.post_format(post_input.as_str().unwrap().as_bytes());
                assert_eq!(
                    String::from_utf8_lossy(&got),
                    case["expected_post"].as_str().unwrap(),
                    "case {i} {formatter} postFormat"
                );
            }
        }
    }

    /// Pint's Prettier::format() failures, with stand-in workers: silence, exiting, and error output.
    #[test]
    fn worker_failures() {
        for (script, idle, message) in [
            (
                "cat >/dev/null",
                Duration::from_millis(50),
                "Laravel Pint's Prettier worker timed out while formatting [a.blade.php].",
            ),
            (
                "exit 0",
                WORKER_IDLE_TIMEOUT,
                "Laravel Pint's Prettier worker terminated unexpectedly.",
            ),
            ("echo boom >&2; sleep 5", WORKER_IDLE_TIMEOUT, "boom\n"),
        ] {
            let mut sh = Command::new("sh");
            sh.args(["-c", script]);
            let mut guard = Some(spawn(sh).unwrap());
            let err = prettier_format_with(&mut guard, "a.blade.php", b"x", idle).unwrap_err();
            assert_eq!(err.to_string(), message);
            assert!(guard.is_none());
        }
    }

    /// Every run shares one copy of the worker files instead of leaving a temp dir per process.
    #[test]
    fn resources_are_shared() {
        let (worker, _) = resources().unwrap();
        let dir = worker.parent().unwrap().file_name().unwrap();
        assert_ne!(dir.to_string_lossy(), format!("pun-blade-{}", std::process::id()));
        let dir = std::env::temp_dir().join(format!("pun-blade-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (worker, config) = resources_in(&dir).unwrap();
        assert_eq!(std::fs::read_to_string(&worker).unwrap(), WORKER);
        assert_eq!(std::fs::read_to_string(config).unwrap(), PRETTIERRC);
        // An existing copy is used as is.
        std::fs::write(&worker, "kept").unwrap();
        resources_in(&dir).unwrap();
        assert_eq!(std::fs::read_to_string(&worker).unwrap(), "kept");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
