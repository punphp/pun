//! `[files]` and each command's `exclude` (context/plan/config.md §3, §7). Expected sets are derived by hand from
//! the plan's rules: the `[files]` set walks `paths` like PHP-CS-Fixer's `Finder::create()` and drops pun.toml's
//! excludes (globs and plain prefixes matched Mago's way, relative to the project root); `format` is Pint's finder.
extern crate punphp_workspace as pun_workspace;
mod common;

use pun_workspace::{Command, Exclusions, FinderError, files, is_path_excluded};
use std::path::{Path, PathBuf};

fn tree(files: &[&str]) -> PathBuf {
    common::tree("files", files, "<?php\n")
}

fn list(
    root: &Path,
    toml: &str,
    command: Option<Command>,
    paths: &[&str],
) -> Result<Vec<String>, String> {
    let cfg = pun_config::from_toml(root, toml).map_err(|e| e.to_string())?;
    let paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    let out = files(&cfg, command, &paths).map_err(|e| e.to_string())?;
    Ok(out
        .iter()
        .map(|p| p.strip_prefix(root).unwrap().to_string_lossy().into_owned())
        .collect())
}

const TREE: &[&str] = &[
    "app/A.php",
    "app/Models/User.php",
    "app/storage/Kept.php",
    "app/view.blade.php",
    "app/_ide_helper.php",
    "app/vendor/V.php",
    "app/.hidden/H.php",
    "app/notes.txt",
    "tests/Feature/ExampleTest.php",
    "tests/Pest.php",
    "storage/framework/S.php",
    "src/Other.php",
    "vendor/x/V.php",
    "Root.php",
];

#[test]
fn plan_example_gives_the_listed_sets() {
    let root = tree(TREE);
    let toml = r#"
[files]
paths = ["app", "tests"]
exclude = ["storage/**"]

[analyse]
exclude = ["tests/**"]
"#;
    let app = [
        "app/A.php",
        "app/Models/User.php",
        "app/_ide_helper.php",
        "app/storage/Kept.php",
    ];
    let tests = ["tests/Feature/ExampleTest.php", "tests/Pest.php"];
    // `pun list-files`: app and tests. `storage/**` is root-relative, so app/storage stays.
    assert_eq!(
        list(&root, toml, None, &[]).unwrap(),
        [
            &app[..2],
            &[
                "app/_ide_helper.php",
                "app/storage/Kept.php",
                "app/view.blade.php"
            ],
            &tests
        ]
        .concat()
    );
    // `pun list-files --command analyse`: only app, without the Blade template.
    assert_eq!(list(&root, toml, Some(Command::Analyse), &[]).unwrap(), app);
    // Lint has no exclude of its own here: the [files] set without the Blade template.
    assert_eq!(
        list(&root, toml, Some(Command::Lint), &[]).unwrap(),
        [&app[..], &tests].concat()
    );
    // Format is Pint's finder: no `storage` dir at any depth, no IDE helpers, no Blade without the Blade rule.
    assert_eq!(
        list(&root, toml, Some(Command::Format), &[]).unwrap(),
        [
            "app/A.php",
            "app/Models/User.php",
            "tests/Feature/ExampleTest.php",
            "tests/Pest.php"
        ]
    );
}

#[test]
fn each_command_adds_its_own_exclude() {
    let root = tree(TREE);
    let toml = r#"
[files]
exclude = ["tests"]

[format]
exclude = ["app/Models/**"]

[lint]
exclude = ["app/*.php", "Root.php"]

[analyse]
paths = ["app"]
exclude = ["app/Models"]
"#;
    // No [files] paths: the project root. Plain `tests` is a root-relative prefix; vendor/ is never listed.
    let all = [
        "Root.php",
        "app/A.php",
        "app/Models/User.php",
        "app/_ide_helper.php",
        "app/storage/Kept.php",
        "app/view.blade.php",
        "src/Other.php",
        "storage/framework/S.php",
    ];
    assert_eq!(list(&root, toml, None, &[]).unwrap(), all);
    // Mago's globs let `*` cross `/`: `app/*.php` drops every PHP file under app. Lint and analyse skip the root's
    // `storage` (Pint's default excludes, root-relative) and Blade templates.
    assert_eq!(
        list(&root, toml, Some(Command::Lint), &[]).unwrap(),
        ["src/Other.php"]
    );
    assert_eq!(
        list(&root, toml, Some(Command::Analyse), &[]).unwrap(),
        ["app/A.php", "app/_ide_helper.php", "app/storage/Kept.php"]
    );
    // Format: Pint's `exclude: ["tests"]` (a dir name at any depth) plus the root-relative glob.
    assert_eq!(
        list(&root, toml, Some(Command::Format), &[]).unwrap(),
        ["Root.php", "app/A.php", "src/Other.php"]
    );
}

#[test]
fn path_arguments_replace_paths_but_keep_excludes() {
    let root = tree(TREE);
    let toml = "[files]\npaths = [\"app\"]\nexclude = [\"src/**\", \"app/Models\"]\n";
    // An explicit file outside a skipped directory is listed; one a pun.toml exclude matches is not (Mago).
    assert_eq!(
        list(
            &root,
            toml,
            None,
            &[
                "src",
                "vendor/x/V.php",
                "app/Models/User.php",
                "tests/Pest.php"
            ]
        )
        .unwrap(),
        ["tests/Pest.php", "vendor/x/V.php"]
    );
    // Format keeps Pint's rule: with `in` set, path arguments replace the finder, excludes included.
    assert_eq!(
        list(
            &root,
            toml,
            Some(Command::Format),
            &["src", "app/Models/User.php"]
        )
        .unwrap(),
        ["app/Models/User.php", "src/Other.php"]
    );
    assert_eq!(
        list(&root, toml, None, &["nope"]).unwrap_err(),
        "The path \"nope\" is not readable."
    );
    assert_eq!(
        list(&root, "[files]\npaths = [\"missing\"]\n", None, &[]).unwrap_err(),
        "The \"missing\" directory does not exist."
    );
    // A [files] path may name a file.
    assert_eq!(
        list(
            &root,
            "[files]\npaths = [\"Root.php\", \"tests\"]\n",
            Some(Command::Lint),
            &[]
        )
        .unwrap(),
        [
            "Root.php",
            "tests/Feature/ExampleTest.php",
            "tests/Pest.php"
        ]
    );
    assert!(
        list(&root, "[files]\nexclude = [\"a/{b\"]\n", None, &[])
            .unwrap_err()
            .starts_with("invalid exclude: ")
    );
}

#[test]
fn stdin_filename_honours_glob_excludes() {
    let root = tree(&[]);
    let cfg = pun_config::from_toml(&root, "[format]\nexclude = [\"app/Gen*/**\"]\n").unwrap();
    assert!(is_path_excluded(&cfg, &root.join("app/Generated/X.php")));
    assert!(!is_path_excluded(&cfg, &root.join("app/Models/X.php")));
}

/// The finder's errors carry their kind, so pun-cli picks Pint's exception (class, file, line) without reading the
/// message: every site, through `files` and `discover`, with or without a command.
#[test]
fn errors_are_typed() {
    let root = tree(&["src/A.php"]);
    let kind = |toml: &str, command: Option<Command>, paths: &[&str]| {
        let cfg = pun_config::from_toml(&root, toml).unwrap();
        let paths: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
        let e = files(&cfg, command, &paths).unwrap_err();
        e.downcast_ref::<FinderError>().cloned()
    };
    let missing = "[files]\npaths = [\"missing\"]\n";
    for command in [None, Some(Command::Format)] {
        assert_eq!(
            kind("", command, &["nope"]),
            Some(FinderError::NotReadable("nope".into()))
        );
        assert_eq!(
            kind(missing, command, &[]),
            Some(FinderError::DirectoryNotFound("missing".into()))
        );
    }
}

/// `pun analyse` drops excluded files from the symbol scan too: the same patterns as its file set, on absolute
/// paths, with Pint's default directories as root-relative prefixes.
#[test]
fn exclusions_test_absolute_paths() {
    let root = tree(&[]);
    let cfg = pun_config::from_toml(
        &root,
        "[files]\nexclude = [\"src/Gen*\"]\n[analyse]\nexclude = [\"tests/**\"]\n",
    )
    .unwrap();
    let e = Exclusions::new(&cfg, Command::Analyse).unwrap();
    for excluded in [
        "tests/Unit/A.php",
        "src/Generated.php",
        "storage/framework/views/x.php",
        "node_modules/a/b.php",
    ] {
        assert!(e.is_excluded(&root.join(excluded)), "{excluded}");
    }
    for kept in [
        "app/storage/A.php",
        "vendor/laravel/framework/src/A.php",
        "src/Models/A.php",
    ] {
        assert!(!e.is_excluded(&root.join(kept)), "{kept}");
    }
    // Lint has its own excludes: tests stay.
    let e = Exclusions::new(&cfg, Command::Lint).unwrap();
    assert!(!e.is_excluded(&root.join("tests/Unit/A.php")));
    assert!(e.is_excluded(&root.join("src/Generated.php")));
}
