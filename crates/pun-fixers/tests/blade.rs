//! Pint's own Blade golden files (tests/blade, copied from Pint's tests/Fixtures/blade-formatting): every input
//! formats to its `.expected`, and every `.expected` stays as is (Pint's Blade tests: `pint --blade`, preset laravel).
//! The ignorables are left untouched by the fixer itself. Needs `bun install` in crates/pun-fixers (prettier and
//! its plugins at the versions in package.json): the worker resolves them from the working directory, like Pint.
//! The preset's other fixers run too, so a PUN_FIXERS_ONLY build must include them.
extern crate punphp_fixers as pun_fixers;
use std::path::{Path, PathBuf};

/// `pint --blade` with preset laravel: the preset's rules plus Pint/laravel_blade, sorted like the runner.
fn pipeline() -> pun_format::Pipeline {
    let (_, json) = pun_config::PRESETS
        .iter()
        .find(|(n, _)| *n == "laravel")
        .unwrap();
    let preset: serde_json::Value = serde_json::from_str(json).unwrap();
    let mut rules: Vec<pun_config::Rule> = preset["rules_as_configured"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, v)| **v != serde_json::Value::Bool(false))
        .map(|(name, v)| pun_config::Rule {
            name: name.clone(),
            options: if v.is_boolean() {
                serde_json::Value::Null
            } else {
                v.clone()
            },
        })
        .collect();
    rules.push(pun_config::Rule {
        name: "Pint/laravel_blade".into(),
        options: serde_json::Value::Null,
    });
    pun_format::Pipeline::new(&rules, &pun_fixers::all).unwrap()
}

fn files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files(&path, out);
        } else if path.to_string_lossy().ends_with(".blade.php") {
            out.push(path);
        }
    }
}

/// Runner::fixFile + AbstractFixer::fix: every fixer in order, skipping empty or non-candidate tokens. Like the
/// Runner (and Pint's tests), only the resulting code counts.
fn run(pipeline: &pun_format::Pipeline, file: &Path, code: &[u8]) -> Vec<u8> {
    let mut tokens = pun_tokens::Tokens::from_code(code);
    for f in pipeline.fixers() {
        if tokens.count() > 0 && f.is_candidate(&tokens) {
            f.fix(file, &mut tokens);
            tokens.clear_empty_tokens();
            tokens.clear_changed();
        }
    }
    tokens.generate_code()
}

#[test]
fn oracle_blade_fixtures() {
    if !pun_fixers::all()
        .iter()
        .any(|f| f.name() == "Pint/laravel_blade")
    {
        return; // PUN_FIXERS_ONLY build without this fixer
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(
        root.join("node_modules/prettier-plugin-blade").exists(),
        "run `npm install` in {}",
        root.display()
    );
    let pipeline = pipeline();
    // Prettier would re-indent this; untouched means the fixer bailed out.
    let html: &[u8] = b"<div>\n<p>x</p></div>\n";
    // Without prettier installed the worker dies: the fixer fails the file with the worker's error output (a panic,
    // the runner's per-file exception, as Pint throws PrettierException), leaves it untouched, and the next file
    // starts a fresh worker.
    let bare = std::env::temp_dir().join(format!("pun-blade-test-{}", std::process::id()));
    std::fs::create_dir_all(&bare).unwrap();
    std::env::set_current_dir(&bare).unwrap();
    let blade = pipeline
        .fixers()
        .find(|f| f.name() == "Pint/laravel_blade")
        .unwrap();
    let mut tokens = pun_tokens::Tokens::from_code(html);
    let error = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        blade.fix(Path::new("a.blade.php"), &mut tokens)
    }))
    .unwrap_err();
    let error = error.downcast_ref::<String>().unwrap();
    assert!(
        error.contains("Error: Cannot find module 'prettier'"),
        "{error}"
    );
    assert_eq!(tokens.generate_code(), html);
    std::env::set_current_dir(root).unwrap();
    // Not a Blade file.
    assert_eq!(run(&pipeline, Path::new("a.php"), html), html);
    let mut all = Vec::new();
    files(&root.join("tests/blade"), &mut all);
    all.sort();
    let mut fail = Vec::new();
    for input in &all {
        let rel = input.strip_prefix(root).unwrap();
        let code = std::fs::read(input).unwrap();
        let expected = std::fs::read(format!("{}.expected", input.display())).unwrap();
        let got = run(&pipeline, rel, &code);
        if got != expected {
            fail.push(format!(
                "{} (input)\n{}",
                rel.display(),
                String::from_utf8_lossy(&got)
            ));
        }
        let again = run(&pipeline, rel, &expected);
        if again != expected {
            fail.push(format!(
                "{} (expected changed)\n{}",
                rel.display(),
                String::from_utf8_lossy(&again)
            ));
        }
    }
    assert!(
        fail.is_empty(),
        "{} of {} failed:\n{}",
        fail.len(),
        all.len() * 2,
        fail.join("\n")
    );
}
