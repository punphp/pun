//! `pun format --test` on a scratch copy of Pinkary: right file set, unported fixers reported, no crash.

use std::path::Path;
use std::process::Command;

const PINKARY: &str = "/home/forge/projects/pinkary.com";

#[test]
fn pinkary_format_test() {
    if !Path::new(PINKARY).join("pint.json").exists() {
        eprintln!("skipping: {PINKARY} not present");
        return;
    }
    let copy = std::env::temp_dir().join(format!("pun-pinkary-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&copy);
    assert!(Command::new("cp").arg("-a").arg(PINKARY).arg(&copy).status().unwrap().success());

    let pun = |args: &[&str]| {
        // env_clear: no coding-agent variables, so Pint's human summary is used.
        Command::new(env!("CARGO_BIN_EXE_pun")).args(args).current_dir(&copy).env_clear().env("PATH", std::env::var("PATH").unwrap()).output().unwrap()
    };
    let listed = pun(&["format", "--list-files"]);
    let files: Vec<String> = String::from_utf8(listed.stdout).unwrap().lines().map(String::from).collect();
    // Pint 1.29.x on Pinkary (blade rule on): app, config, database, routes, tests, resources/views; never vendor/storage/hidden.
    assert!(files.len() > 400, "{} files", files.len());
    assert!(files.iter().any(|f| f.ends_with(".blade.php")));
    assert!(files.iter().all(|f| !f.starts_with("vendor/") && !f.starts_with("storage/") && !f.starts_with('.') && !f.starts_with("bootstrap/cache/")));

    let out = pun(&["format", "--test"]);
    let (stdout, stderr) = (String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(stdout.contains(&format!(" {} files  ", files.len())), "{stdout}");
    assert!(stdout.contains("PASS"), "{stdout}"); // zero native fixers: nothing changes
    assert!(stderr.contains("not ported yet"), "{stderr}");
    assert_eq!(out.status.code(), Some(1), "unported rules must not look green");

    let explain = pun(&["format", "--test", "--explain"]);
    let explain = String::from_utf8_lossy(&explain.stderr);
    assert!(explain.contains("not ported        Pint/laravel_blade") && explain.contains("not ported        final_class"), "{explain}");
    let _ = std::fs::remove_dir_all(&copy);
}
