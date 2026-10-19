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

/// (stdout, stderr, exit code) of `pun analyse args` in `dir` with `env` on top of [`run`]'s environment.
fn pun(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (String, String, Option<i32>) {
    let out = clean_env(&mut Command::new(PUN), &["PATH", "HOME"])
        .current_dir(dir)
        .env("COLUMNS", "80")
        .env("PUN_AGENT", "0")
        .envs(env.iter().copied())
        .arg("analyse")
        .args(args)
        .output()
        .unwrap();
    let s = |b: Vec<u8>| String::from_utf8(b).unwrap();
    (s(out.stdout), s(out.stderr), out.status.code())
}

/// One analyse error and a malformed `@pun-ignore` on line 8.
const MALFORMED: &str = "<?php\n\nnamespace App;\n\nfunction f(): void\n{\n    undefined_fn();\n    new Missing(); // @pun-ignore\n}\n";

#[test]
fn malformed_ignores_show_in_every_output() {
    let d = sample("malformed", MALFORMED);
    // Human: the malformed comment is a problem of its own, at the comment (context/plan/output.md §1); no UTF-8
    // locale here, so ASCII glyphs.
    let (out, _, code) = pun(&d, &["--color=never"], &[]);
    assert!(
        out.contains(" x invalid-ignore-comment  `@pun-ignore` needs one or more rule IDs\n  ,- src/a.php:8:23\n"),
        "{out}"
    );
    assert_eq!(code, Some(1));
    // GITHUB_ACTIONS: one `::error` workflow command per problem after the human output, at its line and column.
    let (out, _, _) = pun(&d, &["--color=never"], &[("GITHUB_ACTIONS", "true")]);
    let annotations: Vec<&str> = out.lines().filter(|l| l.starts_with("::")).collect();
    assert_eq!(
        annotations,
        [
            "::error file=src/a.php,line=7,col=5,title=analyse/undefined/function::Function undefined_fn not found.",
            "::error file=src/a.php,line=8,col=5,title=analyse/classes/invalid-new::Instantiated class App\\Missing not found.",
            "::error file=src/a.php,line=8,col=23,title=invalid-ignore-comment::`@pun-ignore` needs one or more rule IDs",
        ]
    );
    // -q: nothing printed, the exit code still says there are errors.
    assert_eq!(
        pun(&d, &["-q"], &[]),
        (String::new(), String::new(), Some(1))
    );
}

#[test]
fn generate_baseline_to_a_named_file() {
    let d = sample("baseline-path", ERRORS);
    let (out, _, code) = pun(&d, &["--generate-baseline", "b.toml", "--color=never"], &[]);
    assert!(
        out.contains("Baseline generated with 3 errors in b.toml"),
        "{out}"
    );
    assert_eq!(code, Some(0));
    assert!(d.join("b.toml").exists());
    assert!(!d.join("pun-baseline.toml").exists());
    // -q writes it without a word.
    assert_eq!(
        pun(&d, &["--generate-baseline", "-q"], &[]),
        (String::new(), String::new(), Some(0))
    );
    assert!(d.join("pun-baseline.toml").exists());
}

#[test]
fn run_errors_exit_2() {
    // An invalid pun.toml.
    let d = sample("bad-config", CLEAN);
    std::fs::write(d.join("pun.toml"), "[analyse]\nnope = 1\n").unwrap();
    let (out, err, code) = pun(&d, &[], &[]);
    assert_eq!((out.as_str(), code), ("", Some(2)));
    assert!(err.contains("unknown field `nope`"), "{err}");
    // A configured baseline that is missing or not TOML, and a baseline path that can't be written.
    let d = sample("bad-baseline", ERRORS);
    std::fs::write(
        d.join("pun.toml"),
        "[analyse]\npaths = [\"src\"]\nbaseline = \"b.toml\"\n",
    )
    .unwrap();
    let b = d.join("b.toml");
    let hint = "  run `pun analyse --generate-baseline` to write it again\n";
    let (out, err, code) = pun(&d, &["--color=never"], &[]);
    assert_eq!(
        (out.as_str(), err, code),
        (
            "",
            format!(
                "x {}  can't read the baseline: No such file or directory (os error 2)\n{hint}",
                b.display()
            ),
            Some(2)
        )
    );
    std::fs::write(&b, "x = \n").unwrap();
    let (_, err, code) = pun(&d, &["--color=never"], &[]);
    assert_eq!(
        (err, code),
        (
            format!(
                "x {}  invalid baseline: string values must be quoted, expected literal string\n{hint}",
                b.display()
            ),
            Some(2)
        )
    );
    std::fs::create_dir(d.join("dir.toml")).unwrap();
    let (_, err, code) = pun(
        &d,
        &["--generate-baseline", "dir.toml", "--color=never"],
        &[],
    );
    assert_eq!(
        (err, code),
        (
            format!(
                "x can't write the baseline to {}: Is a directory (os error 21)\n",
                d.join("dir.toml").display()
            ),
            Some(2)
        )
    );
    // A file that exists but can't be read stops the run, as it stops PHPStan's (CouldNotReadFileException).
    use std::os::unix::fs::PermissionsExt;
    let d = sample("unreadable", CLEAN);
    let file = d.join("src/a.php");
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o000)).unwrap();
    if std::fs::read(&file).is_err() {
        let (out, _, code) = pun(&d, &[], &[]);
        assert_eq!((out.as_str(), code), ("", Some(2)));
    }
}

/// `table` and `teamcity` against PHPStan beyond [`formats_match_phpstan`]'s sample: Teamcity's escapes (in a path),
/// one error, several tables, a 40-column terminal that wraps and cuts a long message, and the fuzzy relative path
/// of an analysed file path and of paths outside the project.
#[test]
fn table_and_teamcity_match_phpstan() {
    let Some(phar) = phpstan() else { return };
    let phar = phar.to_str().unwrap();
    // Outside the project and outside its top-level directory (the temp dir), so they share no directory.
    let outside = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("analyse-outside-{}", std::process::id()));
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("o.php"), "<?php\nnope2();\n").unwrap();
    let outside = outside.display();
    let long = "<?php\nthis_is_an_extremely_long_function_name_to_wrap();\n";
    let cases: [(&str, &[(&str, &str)], &str); 4] = [
        (
            "escapes",
            &[("src/it's [a|b]/x.php", "<?php\nnope();\n")],
            "src",
        ),
        (
            "file",
            &[("src/a.php", long), ("src/b.php", "<?php\nnope();\n")],
            "src/a.php",
        ),
        (
            "outside",
            &[
                ("src/a.php", "<?php\nnope();\n"),
                ("lib/b.php", "<?php\nnope3();\n"),
            ],
            &format!("src\", \"lib\", \"{outside}"),
        ),
        (
            "outside-first",
            &[("src/a.php", "<?php\nnope();\n")],
            &format!("{outside}\", \"src"),
        ),
    ];
    for (name, files, paths) in cases {
        let d = project("analyse", name, files);
        let list: Vec<&str> = paths.split("\", \"").collect();
        let neon: String = list.iter().map(|p| format!("        - {p}\n")).collect();
        std::fs::write(
            d.join("phpstan.neon"),
            format!("parameters:\n    level: 0\n    paths:\n{neon}"),
        )
        .unwrap();
        std::fs::write(
            d.join("pun.toml"),
            format!("[analyse]\npaths = [\"{paths}\"]\n"),
        )
        .unwrap();
        for f in ["table", "teamcity"] {
            for columns in ["80", "40"] {
                let format = format!("--error-format={f}");
                let with = |program: &[&str]| {
                    let out = clean_env(&mut Command::new(program[0]), &["PATH", "HOME"])
                        .current_dir(&d)
                        .env("COLUMNS", columns)
                        .env("PUN_AGENT", "0")
                        .args(&program[1..])
                        .arg(&format)
                        .output()
                        .unwrap();
                    (String::from_utf8(out.stdout).unwrap(), out.status.code())
                };
                let want = with(&[
                    "php",
                    phar,
                    "analyse",
                    "--no-progress",
                    "-c",
                    "phpstan.neon",
                ]);
                assert_eq!(with(&[PUN, "analyse"]), want, "{name} {f} {columns}");
            }
        }
    }
}
