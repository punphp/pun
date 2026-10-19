//! Larastan 3.10.0's own rule tests, run through PHPStan 2.2.7 with Larastan by
//! scripts/export-larastan-fixtures.php: every case's errors with an identifier of a ported Larastan rule must be
//! exactly PHPStan's. Pun knows Laravel through tests/fixtures/larastan/stubs.php. Cases that still differ are
//! listed in tests/fixtures/larastan/gaps.txt, and the list must be exact (a fixed case has to leave it). The app root
//! is tests/fixtures/larastan/application, with the migrations of Larastan's test application.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use pun_analyse::{Larastan, Project, run};
use serde_json::Value;

type Key = (String, u32, String, String, Option<String>);

/// The identifiers of the ported Larastan rules.
const PORTED: &[&str] = &[
    "larastan.uselessConstructs.with",
    "larastan.uselessConstructs.value",
    "larastan.noEnvCallsOutsideOfConfig",
    "larastan.noModelMake",
    "larastan.deferrableServiceProvider.missingProvides",
    "larastan.noAuthFacadeInRequestScope",
    "larastan.noAuthHelperInRequestScope",
    "larastan.noPublicModelScopeMethod",
    "larastan.noPublicModelAccessorMethod",
    "larastan.octaneCompatibility",
    "rules.modelAppends",
];

/// Laravel's stubs, and the migrations' Schema facade (autoloadable in Larastan's test application).
fn scanned(root: &Path) -> Vec<PathBuf> {
    vec![
        root.join("stubs.php"),
        root.join("application/database/migrations/Schema.php"),
    ]
}

fn fixtures(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() && p.file_name().unwrap() != "data" {
            fixtures(&p, out);
        } else if p.extension().is_some_and(|x| x == "json") {
            out.push(p);
        }
    }
}

#[test]
fn larastan_rule_fixtures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/larastan");
    let data = root.join("data");
    let mut files = Vec::new();
    fixtures(&root, &mut files);
    files.sort();
    let gaps: BTreeSet<String> = std::fs::read_to_string(root.join("gaps.txt"))
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect();
    let prefix = format!("{}/", data.display());
    let mut larastan = Larastan::new(&root.join("application"));
    // what export-larastan-fixtures.php switches on, and NoEnvCallsOutsideOfConfigRuleTest's directories
    larastan.config_directories = ["Rules/data/config", "Rules/data/module/*/config"]
        .iter()
        .map(|d| data.join(d).to_string_lossy().into_owned())
        .collect();
    larastan.check_octane_compatibility = true;
    larastan.check_model_method_visibility = true;
    larastan.check_auth_calls_when_in_request_scope = true;
    let mut failing = BTreeSet::new();
    let mut details = Vec::new();
    let mut compared = 0;
    for f in files {
        let test: Value = serde_json::from_str(&std::fs::read_to_string(&f).unwrap()).unwrap();
        for case in test["cases"].as_array().unwrap() {
            let ported = |id: &str| PORTED.contains(&id);
            let want: BTreeSet<Key> = case["errors"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|e| e["identifier"].as_str().is_some_and(ported))
                .map(|e| {
                    (
                        e["file"].as_str().unwrap().to_string(),
                        e["line"].as_u64().unwrap() as u32,
                        e["identifier"].as_str().unwrap().to_string(),
                        e["message"].as_str().unwrap().to_string(),
                        e["tip"].as_str().map(str::to_string),
                    )
                })
                .collect();
            let analysed = case["files"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| data.join(p.as_str().unwrap()))
                .collect();
            let mut project = Project::new(analysed, scanned(&root));
            project.larastan = Some(larastan.clone());
            let got: BTreeSet<Key> = run(&project)
                .unwrap()
                .into_iter()
                .filter(|e| ported(&e.identifier))
                .map(|e| {
                    let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
                    (file, e.line, e.identifier, e.message, e.tip)
                })
                .collect();
            let name = format!(
                "{}::{}",
                test["test"].as_str().unwrap(),
                case["method"].as_str().unwrap()
            );
            compared += want.len();
            if want != got {
                details.push(format!(
                    "{name}\n  missing {:#?}\n  extra   {:#?}",
                    want.difference(&got).collect::<Vec<_>>(),
                    got.difference(&want).collect::<Vec<_>>()
                ));
                failing.insert(name);
            }
        }
    }
    // PHPStan's own errors of the ported rules on the fixtures
    assert!(compared >= 30, "only {compared} expected errors");
    let fixed: Vec<_> = gaps.difference(&failing).collect();
    let new: Vec<_> = details
        .iter()
        .filter(|d| !gaps.contains(d.lines().next().unwrap()))
        .collect();
    assert!(
        fixed.is_empty() && new.is_empty(),
        "fixed (remove from gaps.txt): {fixed:#?}\nnew failures:\n{}",
        new.iter()
            .map(|d| d.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// The opt-in rules stay off with Larastan's defaults, and no Larastan rule runs without Larastan.
#[test]
fn larastan_switches_gate_the_rules() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/larastan");
    let data = root.join("data");
    let test: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("Pun/Cases.json")).unwrap())
            .unwrap();
    const OPT_IN: &[&str] = &[
        "larastan.noAuthFacadeInRequestScope",
        "larastan.noAuthHelperInRequestScope",
        "larastan.noPublicModelScopeMethod",
        "larastan.noPublicModelAccessorMethod",
        "larastan.octaneCompatibility",
    ];
    for case in test["cases"].as_array().unwrap() {
        let want: BTreeSet<(u32, String)> = case["errors"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|e| {
                Some((
                    e["line"].as_u64()? as u32,
                    e["identifier"].as_str()?.to_string(),
                ))
            })
            .filter(|(_, id)| PORTED.contains(&id.as_str()) && !OPT_IN.contains(&id.as_str()))
            .collect();
        let files: Vec<PathBuf> = case["files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| data.join(p.as_str().unwrap()))
            .collect();
        let got = |larastan: Option<Larastan>| {
            let mut project = Project::new(files.clone(), scanned(&root));
            project.larastan = larastan;
            run(&project)
                .unwrap()
                .into_iter()
                .filter(|e| PORTED.contains(&e.identifier.as_str()))
                .map(|e| (e.line, e.identifier))
                .collect::<BTreeSet<_>>()
        };
        assert_eq!(
            got(Some(Larastan::new(&root.join("application")))),
            want,
            "{}",
            case["method"]
        );
        assert!(got(None).is_empty(), "{}", case["method"]);
    }
}

/// Larastan's switches come from the resolved PHPStan config, and only when it includes Larastan.
#[test]
fn larastan_switches_from_the_config() {
    let root = Path::new("/app");
    let switches = |neon: &str| {
        let config = pun_phpstan::neon::decode(neon).unwrap();
        let params = config.as_array().unwrap().get_str("parameters").unwrap();
        Larastan::from_params(root, params.as_array().unwrap())
    };
    assert!(switches("parameters:\n\tlevel: 0\n").is_none());
    let l = switches(
        "parameters:\n\tnoModelMake: false\n\tcheckOctaneCompatibility: true\n\tconfigDirectories:\n\t\t- modules/*/config\n",
    )
    .unwrap();
    assert!(!l.no_model_make);
    assert!(l.check_octane_compatibility);
    assert!(l.no_env_calls_outside_of_config);
    assert!(!l.check_model_method_visibility);
    assert_eq!(l.config_directories, ["/app/modules/*/config"]);
    let defaults = switches("parameters:\n\tnoModelMake: true\n").unwrap();
    assert_eq!(defaults.config_directories, ["/app/config"]);
}
