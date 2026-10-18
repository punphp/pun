//! Token parity against real PHP.
//!
//!   cargo run --release -p punphp-tokens --example parity -- [--mode raw|parse|fixer] [-v] <path>...
//!
//! raw:   our lexer vs `token_get_all($code)` (kind, text, line)
//! parse: our lexer + TOKEN_PARSE keyword demotion vs `token_get_all($code, TOKEN_PARSE)`
//! fixer: our `Tokens::from_code` (transformers applied) vs PHP-CS-Fixer's `Tokens::fromCode`
//!        (kind incl. `CT::` names, content). Uses .oracle/php-cs-fixer unless PUN_PHP_CS_FIXER is set.
//! classy: TokensAnalyzer::get_classy_elements vs getClassyElements (type, classIndex, index).
//! Files PHP refuses to parse (parse/fixer modes) are reported and skipped.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

type Stream = Vec<(String, Vec<u8>, u32)>;

fn collect(path: &Path, out: &mut Vec<PathBuf>) {
    if path.is_dir() {
        let mut entries: Vec<_> = std::fs::read_dir(path)
            .unwrap()
            .flatten()
            .map(|e| e.path())
            .collect();
        entries.sort();
        for p in entries {
            let name = p.file_name().unwrap().to_string_lossy();
            if name == "vendor" || name == "node_modules" || name == ".git" {
                continue;
            }
            collect(&p, out);
        }
    } else if path.extension().is_some_and(|e| e == "php") {
        out.push(path.to_path_buf());
    }
}

/// Parse dump.php output into (path -> stream | error).
fn parse_dump(data: &[u8]) -> Vec<(String, Result<Stream, String>)> {
    let mut files = Vec::new();
    let mut i = 0;
    let line = |i: &mut usize| -> &[u8] {
        let start = *i;
        let end = start + data[start..].iter().position(|&c| c == b'\n').unwrap();
        *i = end + 1;
        &data[start..end]
    };
    while i < data.len() {
        let l = line(&mut i);
        if let Some(path) = l.strip_prefix(b"=== ") {
            files.push((String::from_utf8_lossy(path).into_owned(), Ok(Vec::new())));
        } else if let Some(err) = l.strip_prefix(b"!!! ") {
            files.last_mut().unwrap().1 = Err(String::from_utf8_lossy(err).into_owned());
        } else {
            let header = String::from_utf8_lossy(l).into_owned();
            let mut parts = header.split(' ');
            let name = parts.next().unwrap().to_string();
            let ln: u32 = parts.next().unwrap().parse().unwrap();
            let len: usize = parts.next().unwrap().parse().unwrap();
            let text = data[i..i + len].to_vec();
            i += len + 1;
            if let Ok(s) = &mut files.last_mut().unwrap().1 {
                s.push((name, text, ln));
            }
        }
    }
    files
}

/// Our stream, plus the time spent producing it (excluding conversion for comparison).
fn ours(mode: &str, code: &[u8]) -> (Stream, Duration) {
    let name = |k: Option<pun_tokens::Kind>| k.map_or("-".to_string(), |k| k.name().to_string());
    match mode {
        "raw" | "parse" => {
            let t = Instant::now();
            let mut toks = pun_tokens::tokenize(code);
            if mode == "parse" {
                pun_tokens::apply_parse_mode(&mut toks);
            }
            let d = t.elapsed();
            let s = toks
                .iter()
                .map(|t| {
                    (
                        name(t.kind),
                        t.text.to_vec(),
                        if t.kind.is_some() { t.line } else { 0 },
                    )
                })
                .collect();
            (s, d)
        }
        "classy" => {
            let tokens = pun_tokens::Tokens::from_code(code);
            let t = Instant::now();
            let el = pun_tokens::TokensAnalyzer::new(&tokens).get_classy_elements();
            let d = t.elapsed();
            let s = el
                .iter()
                .map(|(i, e)| {
                    (
                        e.kind.as_str().to_string(),
                        i.to_string().into_bytes(),
                        e.class_index as u32,
                    )
                })
                .collect();
            (s, d)
        }
        _ => {
            let t = Instant::now();
            let tokens = pun_tokens::Tokens::from_code(code);
            let d = t.elapsed();
            (
                tokens
                    .iter()
                    .map(|t| (name(t.id), t.content.clone(), 0))
                    .collect(),
                d,
            )
        }
    }
}

fn show(t: Option<&(String, Vec<u8>, u32)>) -> String {
    match t {
        Some((n, c, l)) => format!("{n} line {l} {:?}", String::from_utf8_lossy(c)),
        None => "<end>".into(),
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut mode = "raw".to_string();
    if let Some(i) = args.iter().position(|a| a == "--mode") {
        mode = args.remove(i + 1);
        args.remove(i);
    }
    let verbose = args.iter().any(|a| a == "-v");
    args.retain(|a| a != "-v");
    let php = std::env::var_os("PUN_PHP").unwrap_or_else(|| "php".into());
    if run(Path::new(&php), &mode, &args, verbose) > 0 {
        std::process::exit(1);
    }
}

/// Compare every .php file under `paths` with the dump of `php`; prints a report, returns the number of mismatched
/// files.
pub(crate) fn run(php: &Path, mode: &str, paths: &[String], verbose: bool) -> usize {
    let mut files = Vec::new();
    for a in paths {
        collect(Path::new(a), &mut files);
    }

    let dumper = Path::new(env!("CARGO_MANIFEST_DIR")).join("tools/dump.php");
    let t0 = Instant::now();
    let mut child = Command::new(php)
        .args(["-d", "short_open_tag=0", "-d", "memory_limit=-1"])
        .arg(&dumper)
        .arg(mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("php");
    let list: String = files.iter().map(|f| format!("{}\n", f.display())).collect();
    let mut stdin = child.stdin.take().unwrap();
    std::thread::spawn(move || stdin.write_all(list.as_bytes()).unwrap());
    let output = child.wait_with_output().unwrap();
    let php_time = t0.elapsed();
    let dump = parse_dump(&output.stdout);

    let (mut same, mut diff, mut skipped, mut tokens, mut bad_tokens) = (0, 0, 0, 0usize, 0usize);
    let mut rust_time = Duration::ZERO;
    for (path, expected) in &dump {
        let expected = match expected {
            Ok(s) => s,
            Err(e) => {
                skipped += 1;
                if verbose {
                    println!("SKIP {path}: {e}");
                }
                continue;
            }
        };
        // Only for code PHP accepts: like upstream, Tokens::fromCode isn't defined on unparsable input.
        let code = std::fs::read(path).unwrap();
        let (got, d) = ours(mode, &code);
        rust_time += d;
        tokens += expected.len();
        let first = (0..expected.len().max(got.len())).find(|&i| expected.get(i) != got.get(i));
        match first {
            None => same += 1,
            Some(i) => {
                diff += 1;
                bad_tokens += (i..expected.len())
                    .filter(|&j| expected.get(j) != got.get(j))
                    .count()
                    .max(1);
                println!("MISMATCH {path} at token {i}");
                println!("  php:  {}", show(expected.get(i)));
                println!("  ours: {}", show(got.get(i)));
                if i > 0 {
                    println!("  prev: {}", show(expected.get(i - 1)));
                }
            }
        }
    }
    println!(
        "mode={mode} files={} identical={same} mismatched={diff} skipped={skipped} tokens={tokens} mismatched_tokens={bad_tokens}",
        dump.len()
    );
    println!(
        "time: php {:.2?} (incl. startup), ours {:.2?}",
        php_time, rust_time
    );
    diff
}
