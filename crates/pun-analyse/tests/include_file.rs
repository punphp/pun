//! RequireFileExistsRule and IncludedFilePathResolver, pinned PHPStan 2.3.1. The JSON expectations come from
//! fresh CLI runs with the fixture root as cwd and `php -d phpstan.restarted=1 -d include_path=.:/usr/share/php
//! <oracle>/vendor/bin/phpstan analyse --debug -c <config>.neon --error-format=json --no-progress`.
//! `--debug` disables the result cache and workers; phpstan.restarted prevents TurboProcessRestarter from dropping
//! the explicit PHP ini setting. Default-runtime and no-restart runs produce identical diagnostics for the four
//! feature-toggle configs. Environment runs change cwd/include_path exactly as the public Project fields do.

use std::path::PathBuf;

use pun_analyse::{Project, run};
use pun_phpstan::config::{Options, resolve};
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

/// These expectations were regenerated from the CLI, not exported from RuleTestCase, which mutates cwd and
/// include_path in setUp(). There are intentionally empty, duplicate, relative and normalized entries here.
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
