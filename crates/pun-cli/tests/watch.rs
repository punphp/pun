//! `pun format --watch` on a live directory: the first run and every re-run report the files the real Pint reports
//! for the same files (`php .oracle/pint/builds/pint <files>` on a copy; the output itself is Pun's own, compared as
//! in tests/common), files end up as Pint leaves them, excluded files are left alone, Pun's own writes don't trigger
//! runs, errors don't stop the watch, and removing the project ends it with exit 0. Skipped without the oracle.
mod common;

use common::{PUN, migrate, oracle, scratch, write_files};

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Pint's stdout on `files` (on top of `before`, all fresh in a scratch copy; the whole project unless `explicit`),
/// and the resulting `files`' contents.
fn pint(before: &[(&str, &str)], files: &[(&str, &str)], explicit: bool) -> (String, Vec<String>) {
    let dir = scratch("watch", "oracle");
    write_files(&dir, before);
    write_files(&dir, files);
    let mut args = vec![oracle().unwrap().to_str().unwrap().to_string()];
    args.extend(files.iter().filter(|_| explicit).map(|f| f.0.to_string()));
    let out = Command::new("php")
        .args(&args)
        .current_dir(&dir)
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap())
        .env("HOME", &dir)
        .env("COLUMNS", "80")
        .output()
        .unwrap();
    let after = files
        .iter()
        .map(|f| std::fs::read_to_string(dir.join(f.0)).unwrap())
        .collect();
    (String::from_utf8(out.stdout).unwrap(), after)
}

/// Kills (and reaps) the watcher however the test ends, so a failed assertion can't leave it running.
struct Reap(std::process::Child);

impl Drop for Reap {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn watcher(dir: &Path, home: &Path) -> Command {
    let mut c = Command::new(PUN);
    common::clean_env(&mut c, &["PATH"])
        .args(["format", "--watch"])
        .current_dir(dir)
        .env("HOME", home)
        .env("PUN_AGENT", "0")
        .env("LANG", "en_US.UTF-8")
        .env("COLUMNS", "80");
    c
}

fn wait_for(what: &str, buf: &Mutex<String>, cond: impl Fn(&str) -> bool) {
    let start = Instant::now();
    while !cond(&buf.lock().unwrap()) {
        assert!(
            start.elapsed() < Duration::from_secs(60),
            "timed out waiting for {what}; got {:?}",
            buf.lock().unwrap()
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn watch_reformats_like_pint() {
    if oracle().is_none() {
        return;
    }
    const CFG: &str = r#"{"preset":"laravel","notName":["Skip.php"]}"#;
    let start = [
        ("pint.json", CFG),
        ("app/A.php", "<?php\n$a = array(1,2);\n"),
        ("app/Clean.php", "<?php\n\n$c = 1;\n"),
    ];
    let dir = scratch("watch", "live");
    write_files(&dir, &start);
    migrate(&dir);
    let home = scratch("watch", "home");
    let mut child = Reap(
        watcher(&dir, &home)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    let child_out = child.0.stdout.take().unwrap();
    let child_err = child.0.stderr.take().unwrap();
    let collect = |mut r: Box<dyn Read + Send>| {
        let buf = Arc::new(Mutex::new(String::new()));
        let b = buf.clone();
        std::thread::spawn(move || {
            let mut chunk = [0; 4096];
            while let Ok(n @ 1..) = r.read(&mut chunk) {
                b.lock()
                    .unwrap()
                    .push_str(&String::from_utf8_lossy(&chunk[..n]));
            }
        });
        buf
    };
    let out = collect(Box::new(child_out));
    let err = collect(Box::new(child_err));

    // The first run is a normal run. Each run ends with Pun's summary line, the only line ending in its time.
    let runs = |o: &str| -> Vec<String> {
        let mut runs = vec![String::new()];
        for line in o.split_inclusive('\n') {
            runs.last_mut().unwrap().push_str(line);
            if line.ends_with("s\n") && line.contains(" · ") {
                runs.push(String::new());
            }
        }
        runs.pop();
        runs
    };
    let names =
        |files: &[(&str, &str)]| -> Vec<String> { files.iter().map(|f| f.0.to_string()).collect() };
    let (first, _) = pint(&[("pint.json", CFG)], &start[1..], false);
    wait_for("the first run", &out, |o| !runs(o).is_empty());
    let (want, got) = common::human_pair(
        &first,
        &runs(&out.lock().unwrap())[0],
        &names(&start[1..]),
        &names(&start[1..]),
    );
    assert_eq!(got, want);
    std::thread::sleep(Duration::from_millis(500));

    // A new file and an edited one are re-run together; excluded files (notName, vendor) are not touched.
    let fixed = [
        ("pint.json", CFG),
        ("app/A.php", "<?php\n\n$a = [1, 2];\n"),
        ("app/Clean.php", "<?php\n\n$c = 1;\n"),
    ];
    let edits = [
        ("app/A.php", "<?php\n\n$a = [1, 2];\n$b = array( 3 );\n"),
        ("app/B.php", "<?php\nif($x){echo 1;}\n"),
    ];
    let excluded = [
        ("app/Skip.php", "<?php\n$s = array(1);\n"),
        ("vendor/V.php", "<?php\n$v = array(1);\n"),
    ];
    write_files(&dir, &excluded);
    write_files(&dir, &edits);
    let (second, after) = pint(&fixed, &edits, true);
    assert!(second.contains("2 files"), "{second}");
    wait_for("the re-run", &out, |o| runs(o).len() >= 2);
    let rerun = runs(&out.lock().unwrap())[1].clone();
    let (want, got) = common::human_pair(&second, &rerun, &names(&edits), &names(&edits));
    assert_eq!(got, want);
    assert!(
        rerun.contains("Formatted 2 files, 0 unchanged · 2 files in "),
        "{rerun}"
    );
    for ((f, _), want) in edits.iter().zip(&after) {
        assert_eq!(std::fs::read_to_string(dir.join(f)).unwrap(), *want, "{f}");
    }
    for (f, c) in excluded {
        assert_eq!(std::fs::read_to_string(dir.join(f)).unwrap(), c, "{f}");
    }
    // Pun's own writes are not changes: nothing more runs.
    std::thread::sleep(Duration::from_millis(1500));
    let printed = out.lock().unwrap().len();
    assert_eq!(runs(&out.lock().unwrap()).len(), 2);

    // A broken pun.toml, then an unreadable file: reported on stderr, and the watch goes on.
    let toml = std::fs::read_to_string(dir.join("pun.toml")).unwrap();
    write_files(&dir, &[("pun.toml", "[format")]);
    wait_for("the config error", &err, |e| {
        e.contains(&format!("invalid {}/pun.toml", dir.display()))
    });
    write_files(&dir, &[("pun.toml", &toml), ("app/U.php", "<?php\n")]);
    std::fs::set_permissions(
        dir.join("app/U.php"),
        std::os::unix::fs::PermissionsExt::from_mode(0o000),
    )
    .unwrap();
    // Root reads a mode-000 file: there no read error comes.
    if std::fs::read(dir.join("app/U.php")).is_err() {
        wait_for("the read error", &err, |e| {
            e.contains(&format!(
                "Failed to read content from \"{}/app/U.php\".",
                dir.display()
            ))
        });
        assert_eq!(out.lock().unwrap().len(), printed);
    }

    // Removing the project ends the watch.
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(child.0.wait().unwrap().code(), Some(0));
}

/// The watch ends when the process that started it does (here a shell that exits right after starting it).
#[test]
fn watch_ends_with_its_parent() {
    let dir = scratch("watch", "orphan");
    write_files(&dir, &[("pint.json", "{}"), ("a.php", "<?php\n")]);
    migrate(&dir);
    let home = scratch("watch", "orphan-home");
    let w = watcher(&dir, &home);
    let program = w.get_program().to_str().unwrap().to_string();
    let args: Vec<String> = w
        .get_args()
        .map(|a| format!("'{}'", a.to_str().unwrap()))
        .collect();
    let mut sh = Command::new("sh");
    // The shell exits once the first run has printed, so the watcher has seen its parent.
    let log = home.join("out");
    let script = format!(
        "'{program}' {} >'{}' 2>&1 </dev/null & echo $!; while [ ! -s '{}' ]; do sleep 0.05; done",
        args.join(" "),
        log.display(),
        log.display()
    );
    sh.args(["-c", &script]).current_dir(&dir);
    sh.env_clear()
        .envs(w.get_envs().filter_map(|(k, v)| v.map(|v| (k, v))));
    let pid = String::from_utf8(sh.output().unwrap().stdout)
        .unwrap()
        .trim()
        .to_string();
    let alive =
        || std::fs::read_to_string(format!("/proc/{pid}/stat")).is_ok_and(|s| !s.contains(") Z "));
    let start = Instant::now();
    while alive() {
        if start.elapsed() > Duration::from_secs(20) {
            let _ = Command::new("kill").arg(&pid).status();
            panic!("pun format --watch (pid {pid}) outlived its parent");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
