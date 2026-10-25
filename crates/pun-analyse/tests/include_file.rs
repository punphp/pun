//! RequireFileExistsRule and IncludedFilePathResolver, pinned PHPStan 2.3.1. The JSON expectations come from
//! fresh CLI runs with the fixture root as cwd and `php -d phpstan.restarted=1 -d include_path=.:/usr/share/php
//! <oracle>/vendor/bin/phpstan analyse --debug -c <config>.neon --error-format=json --no-progress`.
//! `--debug` disables the result cache and workers; phpstan.restarted prevents TurboProcessRestarter from dropping
//! the explicit PHP ini setting. Default-runtime and no-restart runs produce identical diagnostics for the four
//! feature-toggle configs. Environment runs change cwd/include_path exactly as the public Project fields do.
//!
//! Exported rule fixtures instead use the repository cwd and default CLI settings, ignoring RuleTestCase flags
//! and method environment changes. Reproduce those expectations from revision cb3b7e99489b2e3789ee2eec10501cb7a72580ea:
//! `php <oracle>/phpstan.phar analyse --debug --level=0 --error-format=json --no-progress <source>/tests/PHPStan/Rules/<file>`.
//! Copy existing include targets from that source tree to the same relative paths under `fixtures/rules/data/`
//! with `cp`, including both trait-directory and class-directory controls. Do not create intentionally missing
//! targets. The exported fixture test below checks every include-bearing rules/levels case without gap exemptions.

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use pun_analyse::{Project, run};
use pun_phpstan::config::{Options, resolve};
use pun_tokens::{Kind, tokenize};
use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/include-file")
}

fn check(config: &str) {
    let root = root();
    let resolved = resolve(&Options {
        config: Some(
            root.join(format!("{config}.neon"))
                .to_string_lossy()
                .into_owned(),
        ),
        ..Options::for_project(&root.to_string_lossy())
    })
    .unwrap();
    let project = Project::from_config(&root, &resolved.parameters);
    assert_errors(project, config);
}

fn assert_errors(project: Project, config: &str) {
    let root = root();
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .filter(|e| e.identifier.ends_with(".fileNotFound"))
        .map(|e| {
            (
                e.file.replace(&root.to_string_lossy().to_string(), "$ROOT"),
                e.line,
                e.identifier,
                e.message
                    .replace(&root.to_string_lossy().to_string(), "$ROOT"),
                e.tip,
                e.can_be_ignored,
            )
        })
        .collect();
    let expected: Value = serde_json::from_slice(
        &std::fs::read(root.join(format!("expected-{config}.json"))).unwrap(),
    )
    .unwrap();
    let mut want: Vec<_> = expected
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["file"].as_str().unwrap().to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
                e["tip"].as_str().map(str::to_string),
                e["ignorable"].as_bool().unwrap(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want, "{config}");
}

#[test]
fn default_level_zero() {
    check("phpstan");
}

#[test]
fn magic_dir_fallback() {
    check("phpstan-magic");
}

#[test]
fn path_constants() {
    check("phpstan-constants");
}

#[test]
fn magic_dir_with_path_constants() {
    check("phpstan-both");
}

/// These dedicated environments were run through the CLI with explicit cwd/include_path settings, separately
/// from the generic exported cases. There are intentionally empty, duplicate, relative and normalized entries.
#[test]
fn cwd_include_path_and_script_directory() {
    let root = root();
    for (mode, cwd, include_paths) in [
        (
            "cwd",
            root.join("cwd"),
            vec![PathBuf::from("."), PathBuf::from("/usr/share/php")],
        ),
        (
            "relative",
            root.clone(),
            vec![
                PathBuf::from("inc"),
                PathBuf::from("."),
                PathBuf::from("/usr/share/php"),
            ],
        ),
        (
            "absolute",
            root.join("cwd"),
            vec![
                root.join("inc/../inc"),
                root.join("inc"),
                PathBuf::new(),
                root.join("inc"),
            ],
        ),
    ] {
        let mut project = Project::new(vec![root.join("paths.php")], Vec::new());
        project.current_working_directory = cwd;
        project.include_paths = include_paths;
        project.use_path_constants_as_constant_string = true;
        assert_errors(project, &format!("environment-{mode}"));
    }
}

#[test]
fn exported_include_cases() {
    fn exports(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() && path.file_name().unwrap() != "data" {
                exports(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "json") {
                out.push(path);
            }
        }
    }

    for identifier in [
        "include.fileNotFound",
        "includeOnce.fileNotFound",
        "require.fileNotFound",
        "requireOnce.fileNotFound",
    ] {
        assert!(pun_analyse::IDENTIFIERS.contains(&identifier));
    }
    let fixtures = root().parent().unwrap().to_path_buf();
    let data = fixtures.join("rules/data");
    let cwd = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap();
    let mut files = Vec::new();
    exports(&fixtures.join("rules"), &mut files);
    exports(&fixtures.join("levels"), &mut files);
    files.sort();
    let mut checked = 0;
    let mut failures = Vec::new();
    for file in files {
        let test: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
        for case in test["cases"].as_array().unwrap() {
            let paths = |key: &str| -> Vec<PathBuf> {
                case[key]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|p| data.join(p.as_str().unwrap()))
                    .collect()
            };
            let files = paths("files");
            if !files.iter().any(|file| {
                tokenize(&std::fs::read(file).unwrap()).iter().any(|token| {
                    matches!(
                        token.kind,
                        Some(
                            Kind::T_INCLUDE
                                | Kind::T_INCLUDE_ONCE
                                | Kind::T_REQUIRE
                                | Kind::T_REQUIRE_ONCE
                        )
                    )
                })
            }) {
                continue;
            }
            checked += 1;
            let mut project = Project::new(files, paths("scan"));
            project.current_working_directory = cwd.clone();
            if let Ok(path) = file.strip_prefix(fixtures.join("levels")) {
                project.level = path
                    .components()
                    .next()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            let prefix = format!("{}/", data.display());
            let got: BTreeSet<_> = run(&project)
                .unwrap()
                .into_iter()
                .filter(|e| e.identifier.ends_with(".fileNotFound"))
                .map(|e| {
                    assert!(e.can_be_ignored);
                    (
                        e.file.strip_prefix(&prefix).unwrap().to_string(),
                        e.line,
                        e.identifier,
                        e.message,
                        e.tip,
                    )
                })
                .collect();
            let want: BTreeSet<_> = case["errors"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|e| {
                    e["identifier"]
                        .as_str()
                        .is_some_and(|id| id.ends_with(".fileNotFound"))
                })
                .map(|e| {
                    (
                        e["file"].as_str().unwrap().to_string(),
                        e["line"].as_i64().unwrap(),
                        e["identifier"].as_str().unwrap().to_string(),
                        e["message"].as_str().unwrap().to_string(),
                        e["tip"].as_str().map(str::to_string),
                    )
                })
                .collect();
            if got != want {
                failures.push(format!(
                    "{}::{} ({})\nmissing: {:?}\nextra: {:?}",
                    test["test"].as_str().unwrap(),
                    case["method"].as_str().unwrap(),
                    file.display(),
                    want.difference(&got).collect::<Vec<_>>(),
                    got.difference(&want).collect::<Vec<_>>()
                ));
            }
        }
    }
    assert!(checked > 0);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    eprintln!("{checked} exported include-bearing cases match, with all four identifiers active");
}
