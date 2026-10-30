//! `pun analyse` against PHPStan 2.3.1 (`.oracle/phpstan`, scripts/oracle.sh): every `--format` (PHPStan's
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
const NON_IGNORABLE: &str = "<?php\nfunction duplicate($value, $value): void {}\nundefined_fn();\n";

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

fn formats_match_phpstan(name: &str, source: &str) {
    let Some(phar) = phpstan() else { return };
    let phar = phar.to_str().unwrap();
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

#[test]
fn formats_match_phpstan_errors() {
    formats_match_phpstan("errors", ERRORS);
}

#[test]
fn formats_match_phpstan_clean() {
    formats_match_phpstan("clean", CLEAN);
}

#[test]
fn formats_match_phpstan_non_ignorable() {
    formats_match_phpstan("non-ignorable", NON_IGNORABLE);
}

#[test]
fn formats_match_phpstan_dumps() {
    formats_match_phpstan(
        "dumps",
        "<?php\n\\PHPStan\\dumpType(1, true); // @phpstan-ignore phpstan.dumpType\n\\PHPStan\\dumpPhpDocType(['value' => 2]);\n",
    );
}

#[test]
fn unknown_format_is_a_run_error() {
    let d = sample("unknown", CLEAN);
    let (out, code) = run(&d, &[PUN, "analyse"], &["--format=nope"]);
    assert_eq!((out.as_str(), code), ("", Some(2)));
}

#[test]
fn parser_failures_are_severe_not_configurable_rule_errors() {
    let Some(phar) = phpstan() else { return };
    let phar = phar.to_str().unwrap();
    let d = sample("parser-severe", "<?php\n#[\\self] class InvalidA {}\n");
    common::write_files(&d, &[
        ("src/b.php", "<?php\n#[\\self] class InvalidB {}\n"),
        ("src/c.php", "<?php\nnew \\parent;\n"),
        ("src/control.php", ERRORS),
        ("phpstan.neon", "parameters:\n    level: 0\n    paths: [src]\n    ignoreErrors:\n        - '#^never matches$#'\n        -\n            identifier: phpstan.parse\n"),
        ("pun.toml", "[analyse]\npaths = [\"src\"]\n[[analyse.ignore]]\nmessage = \"#^never matches$#\"\n[[analyse.ignore]]\nidentifier = \"phpstan.parse\"\n[analyse.rules]\n\"analyse/undefined/function\" = \"off\"\n"),
    ]);
    for format in FORMATS {
        let arg = format!("--error-format={format}");
        let want = run(&d, &["php", phar, "analyse", "--no-progress", "-c", "phpstan.neon"], &[&arg]);
        assert_eq!(want.1, Some(1));
        assert_eq!(run(&d, &[PUN, "analyse"], &[&arg]), want, "{format}");
    }
    std::fs::write(d.join("src/a.php"), "<?php\n#[\\self] // @pun-ignore\nclass InvalidA {}\n").unwrap();
    let (out, code) = run(&d, &[PUN, "analyse"], &["--format=json"]);
    let json: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(code, Some(1));
    assert_eq!(json["totals"]["file_errors"], 2, "{out}");
    assert_eq!(json["totals"]["errors"], 0, "{out}");
    let (_, code) = run(&d, &[PUN, "analyse"], &["--generate-baseline", "--quiet"]);
    assert_eq!(code, Some(1));
    assert!(!d.join("pun-baseline.toml").exists());
    std::fs::remove_dir_all(d).unwrap();
}

#[test]
fn non_ignorable_errors_survive_comments_and_baselines() {
    let d = sample("non-ignorable-baseline", NON_IGNORABLE);
    let source = NON_IGNORABLE
        .replace(
            "void {}",
            "void {} // @pun-ignore analyse/functions/duplicate-parameter",
        )
        .replace(
            "undefined_fn();",
            "undefined_fn(); // @pun-ignore analyse/undefined/function",
        );
    std::fs::write(d.join("src/a.php"), source).unwrap();
    let (out, code) = run(&d, &[PUN, "analyse"], &["--format=json"]);
    let json: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(code, Some(1), "{out}");
    assert_eq!(json["totals"]["file_errors"], 1, "{out}");
    let file = d.join("src/a.php").to_string_lossy().into_owned();
    assert_eq!(
        json["files"][&file]["messages"][0]["identifier"], "parameter.duplicate",
        "{out}"
    );
    assert_eq!(
        json["files"][&file]["messages"][0]["ignorable"], false,
        "{out}"
    );

    std::fs::write(d.join("src/a.php"), NON_IGNORABLE).unwrap();
    let (out, code) = run(
        &d,
        &[PUN, "analyse"],
        &["--generate-baseline", "--color=never"],
    );
    assert_eq!(code, Some(0), "{out}");
    assert!(out.contains("Baseline generated with 1 error in"), "{out}");
    let mut baseline = std::fs::read_to_string(d.join("pun-baseline.toml")).unwrap();
    assert!(
        baseline.contains("Function undefined_fn not found."),
        "{baseline}"
    );
    assert!(!baseline.contains("Redefinition"), "{baseline}");
    // A manually added entry cannot hide a non-ignorable error either.
    baseline.push_str("\n[[issues]]\nfile = 'src/a.php'\ncode = 'analyse/functions/duplicate-parameter'\nmessage = 'Redefinition of parameter $value.'\ncount = 1\n");
    std::fs::write(d.join("pun-baseline.toml"), baseline).unwrap();
    std::fs::write(
        d.join("pun.toml"),
        "[analyse]\npaths = [\"src\"]\nbaseline = \"pun-baseline.toml\"\n",
    )
    .unwrap();
    let (out, code) = run(&d, &[PUN, "analyse"], &["--format=json"]);
    let json: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(code, Some(1), "{out}");
    assert_eq!(json["totals"]["file_errors"], 1, "{out}");
    assert_eq!(
        json["files"][&file]["messages"][0]["identifier"], "parameter.duplicate",
        "{out}"
    );
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
    assert!(
        out.contains(
            "1 baseline entry no longer matches: run `pun analyse --generate-baseline` to drop it\n"
        ),
        "{out}"
    );
    assert_eq!(code, Some(1));
    // Every error fixed: all three entries are stale.
    std::fs::write(d.join("src/a.php"), CLEAN).unwrap();
    let (out, _) = run(&d, &[PUN, "analyse"], &["--color=never"]);
    assert!(
        out.contains("! 3 baseline entries no longer match: run `pun analyse --generate-baseline` to drop them\n"),
        "{out}"
    );
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
    let cases: [(&str, &[(&str, &str)], &str, &str); 5] = [
        (
            "escapes",
            &[("src/it's [a|b]/x.php", "<?php\nnope();\n")],
            "",
            "src",
        ),
        (
            "file",
            &[("src/a.php", long), ("src/b.php", "<?php\nnope();\n")],
            "",
            "src/a.php",
        ),
        (
            "outside",
            &[
                ("src/a.php", "<?php\nnope();\n"),
                ("lib/b.php", "<?php\nnope3();\n"),
            ],
            "",
            &format!("src\", \"lib\", \"{outside}"),
        ),
        (
            "outside-first",
            &[("src/a.php", "<?php\nnope();\n")],
            "",
            &format!("{outside}\", \"src"),
        ),
        // A path through `..` (PHPStan normalises it) to wide characters (mb_strwidth 2: U+20000, U+30000).
        (
            "parent",
            &[
                ("proj/src/a.php", "<?php\nnope();\n"),
                ("x/o.php", "<?php\n\u{20000}();\n\u{30000}();\n"),
            ],
            "proj",
            "../x\", \"src",
        ),
    ];
    for (name, files, cwd, paths) in cases {
        let d = project("analyse", name, files).join(cwd);
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

/// `pun check` reports analyse's findings with the format and lint ones: in files format and lint skip too, a malformed
/// `@pun-ignore` once (lint and analyse both see it), and the baseline note in human output.
#[test]
fn check_reports_analyse_findings() {
    let d = project(
        "analyse",
        "check",
        &[
            (
                "pun.toml",
                "[format]\npreset = \"empty\"\nexclude = [\"lib\"]\n\n[lint]\nexclude = [\"lib\"]\n\n[analyse]\npaths = [\"src\", \"lib\"]\nbaseline = \"b.toml\"\n",
            ),
            ("src/a.php", "<?php\n\nnope(); // @pun-ignore\n"),
            ("lib/b.php", "<?php\n\nnope2();\n"),
        ],
    );
    assert_eq!(
        pun(&d, &["--generate-baseline", "b.toml", "-q"], &[]).2,
        Some(0)
    );
    std::fs::write(d.join("lib/b.php"), "<?php\n\nnope3();\n").unwrap();
    let check = |env: &[(&str, &str)]| {
        let out = clean_env(&mut Command::new(PUN), &["PATH", "HOME"])
            .current_dir(&d)
            .env("PUN_AGENT", "0")
            .envs(env.iter().copied())
            .args(["check", "--no-cache", "--color=never"])
            .output()
            .unwrap();
        (String::from_utf8(out.stdout).unwrap(), out.status.code())
    };
    let (out, code) = check(&[("PUN_AGENT", "1")]);
    let lines: Vec<&str> = out.lines().skip(1).collect();
    assert_eq!(
        lines,
        [
            "lib/b.php:3:1 analyse/undefined/function Function nope3 not found.",
            "src/a.php:3:12 invalid-ignore-comment `@pun-ignore` needs one or more rule IDs",
        ]
    );
    assert_eq!(code, Some(1));
    // nope2's entry no longer matches.
    let (out, _) = check(&[]);
    assert!(
        out.contains("! 1 baseline entry no longer matches: run `pun analyse --generate-baseline` to drop it\n"),
        "{out}"
    );
}

/// `[[analyse.ignore]]` against PHPStan's `ignoreErrors` in every format: a message pattern that matches, an
/// identifier with a `count` it misses (`ignore.count` at the first match), a pattern nothing matches in a file
/// (`ignore.unmatched`, no line, not ignorable) and a raw message with several paths nothing matches (a
/// not-file-specific error). The baseline leaves out the errors about ignores, as PHPStan's does.
#[test]
fn ignores_match_phpstan() {
    let neon = "parameters:\n    level: 0\n    paths:\n        - src\n    ignoreErrors:\n        - message: '#^Function undefined_fn not found\\.$#'\n          path: src/a.php\n        - identifier: method.notFound\n          path: src/a.php\n          count: 2\n        - message: '#^Never$#'\n          path: src/a.php\n        - rawMessage: 'Nope'\n          paths: [src/*, lib/*]\n";
    let toml = "[analyse]\npaths = [\"src\"]\npreset = \"basic\"\n\n[[analyse.ignore]]\nmessage = '#^Function undefined_fn not found\\.$#'\npaths = [\"src/a.php\"]\n\n[[analyse.ignore]]\nidentifier = \"method.notFound\"\npaths = [\"src/a.php\"]\ncount = 2\n\n[[analyse.ignore]]\nmessage = \"#^Never$#\"\npaths = [\"src/a.php\"]\n\n[[analyse.ignore]]\nraw-message = \"Nope\"\npaths = [\"src/*\", \"lib/*\"]\n";
    let d = project(
        "analyse",
        "ignores",
        &[
            ("phpstan.neon", neon),
            ("pun.toml", toml),
            ("src/a.php", ERRORS),
        ],
    );
    // Human, agent and baseline output need no oracle.
    let (out, _, code) = pun(&d, &["--color=never"], &[]);
    assert_eq!(code, Some(1));
    let general = format!(
        "Ignored error \"Nope\" in paths: {0}/src/*, {0}/lib/* was not matched in reported errors.",
        d.display()
    );
    assert!(out.contains(&format!("x {general}\n")), "{out}");
    assert!(out.contains("4 errors"), "{out}");
    let (out, _, _) = pun(&d, &[], &[("PUN_AGENT", "1")]);
    assert!(
        out.contains(&format!("unmatched-ignore {general}")),
        "{out}"
    );
    let (out, _, code) = pun(&d, &["--generate-baseline", "--color=never"], &[]);
    assert!(out.contains("Baseline generated with 1 error in"), "{out}");
    assert_eq!(code, Some(0));
    std::fs::remove_file(d.join("pun-baseline.toml")).unwrap();
    let Some(phar) = phpstan() else { return };
    let phar = phar.to_str().unwrap();
    for f in FORMATS {
        let format = format!("--error-format={f}");
        let phpstan = [
            "php",
            phar,
            "analyse",
            "--no-progress",
            "-c",
            "phpstan.neon",
        ];
        let mut want = run(&d, &phpstan, &[&format, "-v"]);
        // teamcity names the config file
        want.0 = want.0.replace("file='phpstan.neon'", "file='pun.toml'");
        let got = run(&d, &[PUN, "analyse"], &[&format, "-v"]);
        assert_eq!(got, want, "{f}");
    }
}

/// No analyse rule on: PHPStan stops with "No rules detected" (no level, CommandHelper); Pun stops the same, with
/// its run-error exit code.
#[test]
fn no_rules_detected() {
    let d = sample("no-rules", ERRORS);
    // rules/mapping.toml's analyse groups
    let groups = "arrays attributes casing casts classes comments control-flow dead-code debug deprecated functions imports laravel operators phpdoc statements strings types undefined";
    let off: String = groups
        .split(' ')
        .map(|g| format!("{g} = \"off\"\n"))
        .collect();
    std::fs::write(
        d.join("pun.toml"),
        format!("[analyse]\npaths = [\"src\"]\n\n[analyse.rules]\n{off}"),
    )
    .unwrap();
    let (out, err, code) = pun(&d, &["--color=never"], &[]);
    assert_eq!((out.as_str(), code), ("", Some(2)));
    assert!(err.starts_with("x No rules detected\n"), "{err}");
}
