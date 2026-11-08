//! Real Node/Prettier process lifetime through the CLI, in addition to the oracle output tests.
mod common;

use std::io::{Read, Write};
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

struct Running(Child);

impl Running {
    fn finish(mut self, input: &[u8]) -> Output {
        self.0.stdin.take().unwrap().write_all(input).unwrap();
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        self.0
            .stdout
            .take()
            .unwrap()
            .read_to_end(&mut stdout)
            .unwrap();
        self.0
            .stderr
            .take()
            .unwrap()
            .read_to_end(&mut stderr)
            .unwrap();
        Output {
            status: self.0.wait().unwrap(),
            stdout,
            stderr,
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn project(name: &str) -> Option<std::path::PathBuf> {
    let modules = common::prettier::node_modules()?;
    let dir = common::project(
        "prettier-worker",
        name,
        &[
            (
                "pun.toml",
                "[format]\npreset = \"empty\"\n[format.rules]\n\"laravel/blade\" = true\n",
            ),
            (
                "track.cjs",
                "require('fs').appendFileSync('node-pids', process.pid + '\\n');",
            ),
        ],
    );
    std::os::unix::fs::symlink(modules, dir.join("node_modules")).unwrap();
    Some(dir)
}

fn start(dir: &Path, args: &[&str]) -> Running {
    let mut cmd = Command::new(common::PUN);
    common::clean_env(&mut cmd, &["HOME", "PATH"]);
    Running(
        cmd.arg("format")
            .args(args)
            .env("PUN_AGENT", "0")
            .env(
                "NODE_OPTIONS",
                format!("--require={}", dir.join("track.cjs").display()),
            )
            .current_dir(dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap(),
    )
}

fn wait_until(check: impl Fn() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !check() {
        assert!(
            Instant::now() < deadline,
            "worker lifetime condition timed out"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn pids(dir: &Path) -> Vec<u32> {
    std::fs::read_to_string(dir.join("node-pids"))
        .unwrap_or_default()
        .lines()
        .map(|p| p.parse().unwrap())
        .collect()
}

fn alive(pid: u32) -> bool {
    Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "pid="])
        .output()
        .unwrap()
        .status
        .success()
}

#[test]
fn blade_probe_is_reused_for_stdin_and_files() {
    let Some(oracle) = common::oracle() else {
        return;
    };
    let Some(dir) = project("reuse") else { return };
    let input = b"<div>\n<p>x</p></div>\n";
    // Pint's stdin uses a temporary .php path even with --stdin-filename; use its real Blade file path as the
    // formatting oracle for Pun's existing named-Blade stdin support.
    std::fs::write(dir.join("a.blade.php"), input).unwrap();
    let mut pint = Command::new("php");
    common::clean_env(&mut pint, &["HOME", "PATH"]);
    let want = pint
        .arg(oracle)
        .args(["--blade", "--preset=empty", "a.blade.php"])
        .env("PUN_AGENT", "0")
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(
        want.status.success(),
        "{}",
        String::from_utf8_lossy(&want.stderr)
    );
    let expected = std::fs::read(dir.join("a.blade.php")).unwrap();
    std::fs::remove_file(dir.join("a.blade.php")).unwrap();
    let got = start(&dir, &["--stdin-filename=a.blade.php", "-"]).finish(input);
    assert!(
        got.status.success(),
        "{}",
        String::from_utf8_lossy(&got.stderr)
    );
    assert_eq!(got.stdout, expected);
    assert_eq!(
        pids(&dir).len(),
        1,
        "probe and format must share one real Node process"
    );
    // A normal file goes through Rayon rather than the main thread that ran the probe.
    std::fs::remove_file(dir.join("node-pids")).unwrap();
    std::fs::write(dir.join("a.blade.php"), input).unwrap();
    let got = start(&dir, &["--max-processes=2", "--format=json", "a.blade.php"]).finish(b"");
    assert!(
        got.status.success(),
        "{}",
        String::from_utf8_lossy(&got.stderr)
    );
    assert_eq!(std::fs::read(dir.join("a.blade.php")).unwrap(), expected);
    assert_eq!(
        pids(&dir).len(),
        1,
        "the probe worker must transfer to a formatting thread"
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn failed_node_preload_aborts_and_reaps_the_probe() {
    for (name, preload) in [
        ("closed", "process.exit(1);"),
        (
            "malformed",
            r"process.stdout.write('not a probe\n'); setInterval(() => {}, 1000);",
        ),
    ] {
        let Some(dir) = project(name) else { return };
        let tracker = dir.join("track.cjs");
        let script = std::fs::read_to_string(&tracker).unwrap();
        std::fs::write(tracker, format!("{script}\n{preload}\n")).unwrap();
        let input = b"<div>\n<p>x</p></div>\n";
        std::fs::write(dir.join("a.blade.php"), input).unwrap();
        let out = start(&dir, &[]).finish(b"");
        // A failed Node start is a run error, not a missing-package diagnosis or a silently skipped Blade file.
        assert_eq!(out.status.code(), Some(2));
        let message = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(message.contains("needs a JavaScript runtime"), "{message}");
        assert_eq!(std::fs::read(dir.join("a.blade.php")).unwrap(), input);
        assert_eq!(pids(&dir).len(), 1);
        wait_until(|| pids(&dir).iter().all(|&pid| !alive(pid)));
        std::fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn unused_probe_stops_before_reading_stdin() {
    for args in [
        vec!["-"],
        vec!["--stdin-filename=a.php", "-"],
        vec!["--stdin-filename=a.blade.php", "--test", "-"],
        vec!["--stdin-filename=a.blade.php", "--bail", "-"],
    ] {
        let Some(dir) = project("stdin") else { return };
        let mut running = start(&dir, &args);
        wait_until(|| !pids(&dir).is_empty());
        wait_until(|| pids(&dir).iter().all(|&pid| !alive(pid)));
        assert!(
            running.0.try_wait().unwrap().is_none(),
            "Pun should still be waiting for stdin"
        );
        let input = b"<div>\n<p>x</p></div>\n";
        let out = running.finish(input);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(out.stdout, input);
        assert_eq!(pids(&dir).len(), 1);
        std::fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn unused_probe_stops_for_unselected_blade() {
    let Some(dir) = project("watch") else { return };
    common::write_files(
        &dir,
        &[
            ("app/a.php", "<?php\n"),
            ("resources/v.blade.php", "<div>\n    <p>x</p>\n</div>\n"),
        ],
    );
    let finished = start(&dir, &["--format=json", "app"]).finish(b"");
    assert!(finished.status.success());
    let result: serde_json::Value = serde_json::from_slice(&finished.stdout).unwrap();
    assert_eq!(result["files"], serde_json::json!([]));
    assert_eq!(pids(&dir).len(), 1);
    wait_until(|| pids(&dir).iter().all(|&pid| !alive(pid)));
    std::fs::remove_file(dir.join("node-pids")).unwrap();
    let mut running = start(&dir, &["--watch", "--format=json", "app"]);
    let stdout = running.0.stdout.take().unwrap();
    let result = serde_json::Deserializer::from_reader(stdout)
        .into_iter::<serde_json::Value>()
        .next()
        .unwrap()
        .unwrap();
    assert_eq!(result["files"], serde_json::json!([]));
    wait_until(|| !pids(&dir).is_empty());
    wait_until(|| pids(&dir).iter().all(|&pid| !alive(pid)));
    assert!(
        running.0.try_wait().unwrap().is_none(),
        "watch should stay alive"
    );
    assert_eq!(pids(&dir).len(), 1);
    drop(running);
    std::fs::remove_dir_all(dir).unwrap();
}
