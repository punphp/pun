//! Runs `pun` the way the end-to-end tests of interactive commands need: agent mode off, a clean environment, and
//! optionally on a pseudo-terminal. Included with `#[path = "common/pty.rs"] mod common;`.
use std::path::{Path, PathBuf};
use std::process::Command;

/// `pun args` in `dir`; stdin is not a terminal, so nothing is asked. (stdout, stderr, exit code).
pub fn pun(dir: &Path, args: &[&str]) -> (String, String, Option<i32>) {
    pun_with(dir, args, &[])
}

/// `pun args` with `env` set as well.
#[allow(dead_code)]
pub fn pun_with(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (String, String, Option<i32>) {
    run(Command::new(env!("CARGO_BIN_EXE_pun")).args(args), dir, env)
}

/// `cmd` in `dir` with only PATH and the coverage profile kept, `PUN_AGENT=0`, and `env`.
fn run(cmd: &mut Command, dir: &Path, env: &[(&str, &str)]) -> (String, String, Option<i32>) {
    let out = cmd
        .current_dir(dir)
        .env_clear()
        .envs(std::env::vars().filter(|(k, _)| k == "PATH" || k == "LLVM_PROFILE_FILE"))
        .env("PUN_AGENT", "0")
        .envs(env.iter().copied())
        .output()
        .unwrap();
    let s = |b: Vec<u8>| String::from_utf8(b).unwrap();
    (s(out.stdout), s(out.stderr), out.status.code())
}

/// `pun args` (shell words) on a pseudo-terminal, typing `keys` (printf escapes) once it is up, with `env` set. The
/// output with `\r` removed, and the exit code. util-linux `script` runs a command string (`-c`); BSD `script` (macOS)
/// takes the command as its arguments.
pub fn pty(dir: &Path, args: &str, keys: &str, env: &[(&str, &str)]) -> (String, Option<i32>) {
    let pun = format!("{} {args}", env!("CARGO_BIN_EXE_pun"));
    let script = if cfg!(target_os = "linux") {
        format!("script -qec '{pun}' /dev/null")
    } else {
        format!("script -q /dev/null {pun}")
    };
    let sh = format!("(sleep 1; printf '{keys}'; sleep 2) | {script}");
    let (out, _, code) = run(Command::new("sh").args(["-c", &sh]), dir, env);
    (out.replace('\r', ""), code)
}

/// A scratch project `pun-<prefix>-<name>-<pid>` under the temp dir holding `files`.
pub fn project(prefix: &str, name: &str, files: &[(&str, &str)]) -> PathBuf {
    let d = std::env::temp_dir().join(format!("pun-{prefix}-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    for (f, c) in files {
        std::fs::create_dir_all(d.join(f).parent().unwrap()).unwrap();
        std::fs::write(d.join(f), c).unwrap();
    }
    d.canonicalize().unwrap()
}

pub fn read(d: &Path, f: &str) -> String {
    std::fs::read_to_string(d.join(f)).unwrap()
}
