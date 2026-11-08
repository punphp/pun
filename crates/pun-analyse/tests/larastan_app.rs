//! What Larastan learns by booting the app, without booting it: tests/fixtures/larastan-app is a Laravel 13.23 app
//! with Larastan's own stubs (`stubs/common` and `stubs/12.20.0`, plus a `stubs/99.0.0` one for a later Laravel that
//! must not load) and the framework's declarations the probe reaches (laravel.php). The probe dumps what Larastan's
//! extensions type: the `auth()`, `now()`, `Storage::disk()` and `Auth::user()` helpers from the app's auth config
//! and `Date::use()`, ContractsMethodsExtension's methods, `collect()`, the `filter()`/`reject()` narrowing, and the
//! helper, container and request extensions (`app()`, `make()`, `value()`, `validator()`, `str()`, `tap()`, `file()`,
//! `route()`, the stubs' benevolent `query()`), and the ConsoleCommand extensions on app/Console/Commands' command
//! (an entry's `file`, app/Probe.php when absent).
//!
//! Expected: tests/fixtures/larastan-app/expected-phpstan.json, PHPStan 2.3.1 + Larastan 3.10.0 at level max on the
//! same app/Probe.php inside a copy of Pinkary (Laravel 13.23, `Date::use(CarbonImmutable::class)`, the
//! framework's auth config).

use std::path::Path;

use pun_analyse::{Project, run};
use pun_phpstan::{Array, Key, Value as Parameter};
use serde_json::Value;

#[test]
fn larastan_app_extensions() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/larastan-app");
    let oracle: Value =
        serde_json::from_str(include_str!("fixtures/larastan-app/expected-phpstan.json")).unwrap();
    let mut parameters = Array::new();
    parameters.set(Key::from("level"), Parameter::Str("max".into()));
    // extension.neon's parameter, which says Larastan is installed
    parameters.set(Key::from("noModelMake"), Parameter::Bool(true));
    let mut project = Project::from_config(&root, &parameters);
    project.analysed = vec![
        root.join("app/Probe.php"),
        root.join("app/Console/Commands/ProbeCommand.php"),
    ];
    project.scanned = vec![
        root.join("laravel.php"),
        root.join("app/Models/User.php"),
        root.join("vendor/laravel/framework/src/Illuminate/Foundation/Application.php"),
    ];
    let prefix = format!("{}/", root.display());
    let mut got: Vec<_> = run(&project)
        .unwrap()
        .into_iter()
        .map(|e| {
            (
                e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string(),
                e.line,
                e.identifier,
                e.message,
            )
        })
        .collect();
    let mut want: Vec<_> = oracle
        .as_array()
        .unwrap()
        .iter()
        .map(|e| {
            (
                e["file"].as_str().unwrap_or("app/Probe.php").to_string(),
                e["line"].as_i64().unwrap(),
                e["identifier"].as_str().unwrap().to_string(),
                e["message"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    got.sort();
    want.sort();
    assert_eq!(got, want);
}
