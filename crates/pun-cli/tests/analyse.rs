//! `pun analyse` against PHPStan 2.2.7 (`.oracle/phpstan`, scripts/oracle.sh): every `--format` (PHPStan's
//! `--error-format`) byte for byte and the exit code, on a project with errors and on a clean one. Without the
//! oracle (or `php`) the comparisons are skipped. The baseline and ignore cases are derived by hand from
//! context/plan/analyse.md Build 2 and rules.md §4.
mod common;

use common::{PUN, clean_env, project};
use std::path::{Path, PathBuf};
use std::process::Command;

const FORMATS: [&str; 9] = [
    "raw",
    "json",
    "prettyJson",
    "checkstyle",
    "junit",
    "gitlab",
    "github",
    "teamcity",
    "table",
];

const ERRORS: &str = "<?php\n\nnamespace App;\n\nfunction f(): void\n{\n    undefined_fn();\n    new Missing();\n}\n\nclass B\n{\n    public function g(): void\n    {\n        $this->nope();\n    }\n}\n";
const CLEAN: &str = "<?php\n\nnamespace App;\n\nfunction f(): int\n{\n    return 1;\n}\n";

fn phpstan() -> Option<PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../.oracle/phpstan/vendor/phpstan/phpstan/phpstan.phar");
    p.exists().then(|| p.canonicalize().unwrap())
}

/// A project PHPStan (level 0, `src`) and Pun (`[analyse] paths`, the basic preset) analyse alike.
fn sample(name: &str, source: &str) -> PathBuf {
    project(
        "analyse",
        name,
        &[
            (
                "phpstan.neon",
                "parameters:\n    level: 0\n    paths:\n        - src\n",
            ),
            ("pun.toml", "[analyse]\npaths = [\"src\"]\n"),
            ("src/a.php", source),
        ],
    )
}

/// (stdout, exit code) of `program args` in `dir`, 80 columns wide, no agent detected.
fn run(dir: &Path, program: &[&str], args: &[&str]) -> (String, Option<i32>) {
    let out = clean_env(&mut Command::new(program[0]), &["PATH", "HOME"])
        .current_dir(dir)
        .env("COLUMNS", "80")
        .env("PUN_AGENT", "0")
        .args(&program[1..])
        .args(args)
        .output()
        .unwrap();
    (String::from_utf8(out.stdout).unwrap(), out.status.code())
}

#[test]
fn formats_match_phpstan() {
    let Some(phar) = phpstan() else { return };
    let phar = phar.to_str().unwrap();
    for (name, source) in [("errors", ERRORS), ("clean", CLEAN)] {
        let d = sample(name, source);
        for f in FORMATS {
            let format = format!("--error-format={f}");
            let want = run(
                &d,
                &[
                    "php",
                    phar,
                    "analyse",
                    "--no-progress",
                    "-c",
                    "phpstan.neon",
                ],
                &[&format],
            );
            let got = run(&d, &[PUN, "analyse"], &[&format]);
            assert_eq!(got, want, "{name} {f}");
        }
        // `-v`: raw adds each identifier.
        let want = run(
            &d,
            &[
                "php",
                phar,
                "analyse",
                "--no-progress",
                "-c",
                "phpstan.neon",
                "-v",
            ],
            &["--error-format=raw"],
        );
        assert_eq!(
            run(&d, &[PUN, "analyse"], &["--error-format=raw", "-v"]),
            want,
            "{name} raw -v"
        );
    }
}

#[test]
fn unknown_format_is_a_run_error() {
    let d = sample("unknown", CLEAN);
    let (out, code) = run(&d, &[PUN, "analyse"], &["--format=nope"]);
    assert_eq!((out.as_str(), code), ("", Some(2)));
}

#[test]
fn baseline_covers_the_errors_it_lists() {
    let d = sample("baseline", ERRORS);
    let (out, code) = run(
        &d,
        &[PUN, "analyse"],
        &["--generate-baseline", "--color=never"],
    );
    assert_eq!(code, Some(0), "{out}");
    assert!(
        out.contains("Baseline generated with 3 errors in pun-baseline.toml"),
        "{out}"
    );
    let baseline = std::fs::read_to_string(d.join("pun-baseline.toml")).unwrap();
    assert!(baseline.contains("variant = \"loose\""), "{baseline}");
    // A message with a backslash is a TOML literal string.
    assert!(
        baseline.contains("message = 'Call to an undefined method App\\B::nope().'"),
        "{baseline}"
    );
    // Not configured: the run still reports everything.
    assert_eq!(run(&d, &[PUN, "analyse"], &["--format=raw"]).1, Some(1));
    std::fs::write(
        d.join("pun.toml"),
        "[analyse]\npaths = [\"src\"]\nbaseline = \"pun-baseline.toml\"\n",
    )
    .unwrap();
    assert_eq!(
        run(&d, &[PUN, "analyse"], &["--format=raw"]),
        (String::new(), Some(0))
    );
    // An entry that no longer matches is noted; a new error is reported.
    std::fs::write(
        d.join("src/a.php"),
        ERRORS.replace("$this->nope();", "nope2();"),
    )
    .unwrap();
    let (out, code) = run(&d, &[PUN, "analyse"], &["--color=never"]);
    assert!(out.contains("nope2 not found."), "{out}");
    assert!(out.contains("1 baseline entry no longer match"), "{out}");
    assert_eq!(code, Some(1));
}

#[test]
fn ignore_comments_drop_errors() {
    let source = ERRORS
        .replace(
            "    undefined_fn();",
            "    undefined_fn(); // @pun-ignore analyse/undefined/function",
        )
        .replace(
            "    new Missing();",
            "    // @pun-ignore\n    new Missing();",
        );
    let d = sample("ignores", &source);
    let (out, code) = run(&d, &[PUN, "analyse"], &["--format=raw"]);
    let lines: Vec<&str> = out
        .lines()
        .map(|l| l.rsplit_once("/src/").unwrap().1)
        .collect();
    // The bare `@pun-ignore` is malformed and covers nothing; raw lists only the analyse errors.
    assert_eq!(
        lines,
        [
            "a.php:9:Instantiated class App\\Missing not found.",
            "a.php:16:Call to an undefined method App\\B::nope().",
        ]
    );
    assert_eq!(code, Some(1));
}
