//! Shared by the tests that run `pun format` next to Pint. Human output is Pun's own (output.md §1), so a human run
//! that ends in a summary (Pint's Termwind box, Pun's summary line) is compared by the files each tool names; abort
//! boxes, `-q`, stdin and every `--format` stay byte for byte.
//!
//! Also the scratch-project helpers every end-to-end test uses, and `pty` for the interactive commands.
#![allow(dead_code, reason = "each test binary uses some of the helpers")]
pub(crate) mod prettier;
pub(crate) mod pty;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) const PUN: &str = env!("CARGO_BIN_EXE_pun");

/// The pinned Pint (`scripts/oracle.sh`), canonical so paths in its output match; `None` skips the Pint cases.
pub(crate) fn oracle() -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.oracle/pint/builds/pint");
    p.exists().then(|| p.canonicalize().unwrap())
}

/// A fresh, empty directory `pun-<prefix>-<name>-<pid>-<n>` under the temp dir, canonical.
pub(crate) fn scratch(prefix: &str, name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!(
        "pun-{prefix}-{name}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d.canonicalize().unwrap()
}

/// Writes each `(path, content)` under `dir`, creating parent directories.
pub(crate) fn write_files(dir: &Path, files: &[(&str, &str)]) {
    for (f, c) in files {
        let p = dir.join(f);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
}

/// A scratch project holding `files`.
pub(crate) fn project(prefix: &str, name: &str, files: &[(&str, &str)]) -> PathBuf {
    let d = scratch(prefix, name);
    write_files(&d, files);
    d
}

/// The text of `f` under `d`.
pub(crate) fn read(d: &Path, f: &str) -> String {
    std::fs::read_to_string(d.join(f)).unwrap()
}

/// Every file under `dir` by its `/`-separated relative path, except symlinks and the paths `skip` names (checked
/// for directories too). An unreadable file reads as `<unreadable>`.
pub(crate) fn snapshot(dir: &Path, skip: impl Fn(&str) -> bool) -> BTreeMap<String, Vec<u8>> {
    fn walk(
        dir: &Path,
        rel: &str,
        skip: &dyn Fn(&str) -> bool,
        out: &mut BTreeMap<String, Vec<u8>>,
    ) {
        for e in std::fs::read_dir(dir.join(rel)).unwrap().flatten() {
            let r = if rel.is_empty() {
                e.file_name().to_string_lossy().into_owned()
            } else {
                format!("{rel}/{}", e.file_name().to_string_lossy())
            };
            if skip(&r) || e.path().is_symlink() {
                continue;
            }
            if e.path().is_dir() {
                walk(dir, &r, skip, out);
            } else {
                let content = std::fs::read(e.path()).unwrap_or_else(|_| b"<unreadable>".to_vec());
                out.insert(r, content);
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(dir, "", &skip, &mut out);
    out
}

/// Clears `c`'s environment but for the variables in `keep` and LLVM_PROFILE_FILE (so coverage runs don't drop
/// .profraw files into the project).
pub(crate) fn clean_env<'c>(c: &'c mut Command, keep: &[&str]) -> &'c mut Command {
    c.env_clear();
    for k in keep.iter().chain(&["LLVM_PROFILE_FILE"]) {
        if let Ok(v) = std::env::var(k) {
            c.env(k, v);
        }
    }
    c
}

/// `pun migrate` in `dir`, which must succeed.
pub(crate) fn migrate(dir: &Path) {
    let out = clean_env(&mut Command::new(PUN), &["PATH", "HOME"])
        .arg("migrate")
        .env("PUN_AGENT", "0")
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Whether `args` print human output: no `--format`, no `-q` (alone or in a short-option group), no stdin.
pub(crate) fn human(args: &[&str]) -> bool {
    !args.iter().any(|a| {
        a.starts_with("--format")
            || a.starts_with("--stdin")
            || *a == "--quiet"
            || *a == "-"
            || (a.starts_with('-') && !a.starts_with("--") && a.contains('q'))
    })
}

/// Whether `line` is Pun's summary line, which always ends with the run's time (`0.12s`).
fn pun_summary(line: &str) -> bool {
    let time = line.strip_suffix('s').and_then(|l| l.rsplit(' ').next());
    time.is_some_and(|t| {
        t.len() >= 4 && t.as_bytes()[t.len() - 3] == b'.' && t.parse::<f64>().is_ok()
    })
}

/// Pint's and Pun's stdout of one human run, made comparable: when Pint printed its summary and Pun its summary line,
/// each becomes the files it names (Pint's issue lines `⨯`, `✓`, `!`; Pun's file lines and error frames, but not its
/// dim `-v` lines for unchanged files, which Pint doesn't list), followed by what Pun printed after its summary (an
/// abort box) when Pint's output ends with it too. Anything else (an abort box alone) stays as it is.
pub(crate) fn human_pair<'a>(
    pint: &str,
    pun: &str,
    pint_files: impl IntoIterator<Item = &'a String>,
    pun_files: impl IntoIterator<Item = &'a String>,
) -> (String, String) {
    let plain = strip_ansi(pun);
    let Some(end) = plain
        .lines()
        .collect::<Vec<_>>()
        .into_iter()
        .rposition(pun_summary)
    else {
        return (pint.to_string(), pun.to_string());
    };
    if !pint.contains("────") {
        return (pint.to_string(), pun.to_string());
    }
    let tail: String = pun.split_inclusive('\n').skip(end + 1).collect();
    let pint_tail = if pint.ends_with(&tail) {
        tail.clone()
    } else {
        "(Pint's output ends differently)".to_string()
    };
    let head: Vec<&str> = plain.lines().take(end).collect();
    (
        format!("{}\n{pint_tail}", named(&strip_ansi(pint), pint_files)),
        format!("{}\n{tail}", named(&head.join("\n"), pun_files)),
    )
}

/// The files of `files` that `text` names after a space, before a space, a line end or a `:`.
fn named<'a>(text: &str, files: impl IntoIterator<Item = &'a String>) -> String {
    let lines: Vec<&str> = (text.lines())
        .filter(|l| {
            let bare = l.strip_prefix(" ✔ ").or_else(|| l.strip_prefix(" ok "));
            bare.is_none_or(|rest| rest.contains("  "))
        })
        .collect();
    let named: Vec<&str> = (files.into_iter())
        .filter(|rel| {
            lines.iter().any(|l| {
                [" ", "\n", ":"]
                    .iter()
                    .any(|end| format!("{l}\n").contains(&format!(" {rel}{end}")))
            })
        })
        .map(String::as_str)
        .collect();
    format!("named: {}", named.join(", "))
}

/// `out` without ANSI CSI sequences (`ESC [`, parameters, one final byte in `@`..=`~`).
pub(crate) fn strip_ansi(out: &str) -> String {
    let mut plain = String::new();
    let mut chars = out.chars();
    while let Some(c) = chars.next() {
        if c != '\x1b' {
            plain.push(c);
        } else if chars.next() == Some('[') {
            for c in chars.by_ref() {
                if ('@'..='~').contains(&c) {
                    break;
                }
            }
        }
    }
    plain
}
