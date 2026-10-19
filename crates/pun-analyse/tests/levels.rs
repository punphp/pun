//! PHPStan's own rule tests of levels 1 to 9, run through `phpstan analyse --level N` by
//! `scripts/export-rule-fixtures.php --level=N` into tests/fixtures/levels/N: every case's errors with an
//! identifier Pun reports at that level must be exactly PHPStan's. Cases that still differ are listed in
//! tests/fixtures/levels/gaps.txt, and the list must be exact (a fixed case has to leave it).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use pun_analyse::{Project, run};
use serde_json::Value;

type Key = (String, i64, String, String, Option<String>);

/// The identifiers of the rules ported above level 0, and of level-0 rules whose errors the level fixtures hold.
const PORTED: &[&str] = &[
    "argument.unused",
    "callable.inVoidCast",
    "callable.resultDiscarded",
    "closure.unusedUse",
    "constructor.unusedParameter",
    "finally.exitPoint",
    "function.inVoidCast",
    "function.resultDiscarded",
    "function.uselessReturnValue",
    "method.inVoidCast",
    "method.resultDiscarded",
    "staticMethod.inVoidCast",
    "staticMethod.resultDiscarded",
    "missingType.parameter",
    "missingType.return",
    "missingType.property",
    "missingType.iterableValue",
    "missingType.generics",
    "generics.existingClass",
    "generics.existingTypeAlias",
    "generics.traitBound",
    "generics.notSupportedBound",
    "generics.requiredTypeAfterOptional",
    "generics.templateDefaultOutOfBounds",
    "generics.notGeneric",
    "generics.lessTypes",
    "generics.moreTypes",
    "generics.notSubtype",
    "generics.notCompatible",
    "generics.noParent",
    "generics.wrongParent",
    "generics.trait",
    "method.shadowTemplate",
    "methodTag.shadowTemplate",
    "enum.generic",
    "method.parentMethodFinal",
    "method.parentMethodFinalByPhpDoc",
    "method.abstractOverridingNonAbstract",
    "method.override",
    "method.nonStatic",
    "method.static",
    "method.tentativeReturnType",
    "method.childReturnType",
    "method.childParameterType",
    "parameter.missing",
    "parameter.byRef",
    "parameter.notByRef",
    "parameter.notOptional",
    "parameter.notVariadic",
    "property.override",
    "property.nonStatic",
    "property.static",
    "property.readWrite",
    "property.readOnly",
    "property.notReadable",
    "property.notWritable",
    "property.abstractOverridingNonAbstractHook",
    "property.visibility",
    "property.parentPropertyFinal",
    "property.parentPropertyFinalByPhpDoc",
    "property.missingNativeType",
    "property.nativeType",
    "property.extraNativeType",
    "propertySetHook.nativeParameterType",
];

/// The `class.notFound` messages of the Generics rules (other rules report the identifier at every level).
const GENERICS_NOT_FOUND: &[&str] = &[
    "PHPDoc tag @template ",
    "PHPDoc tag @method template ",
    "PHPDoc tag @extends has invalid type",
    "PHPDoc tag @implements has invalid type",
    "PHPDoc tag @use has invalid type",
];

fn fixtures(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            fixtures(&p, out);
        } else if p.extension().is_some_and(|x| x == "json") {
            out.push(p);
        }
    }
}

fn key(e: &Value) -> Key {
    (
        e["file"].as_str().unwrap().to_string(),
        e["line"].as_i64().unwrap(),
        e["identifier"].as_str().unwrap().to_string(),
        e["message"].as_str().unwrap().to_string(),
        e["tip"].as_str().map(str::to_string),
    )
}

#[test]
fn phpstan_level_fixtures() {
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let root = tests.join("levels");
    let data = tests.join("rules/data");
    let gaps: BTreeSet<String> = std::fs::read_to_string(root.join("gaps.txt"))
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect();
    let prefix = format!("{}/", data.display());
    let mut failing = BTreeSet::new();
    let mut details = Vec::new();
    let mut levels: Vec<_> = std::fs::read_dir(&root)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    levels.sort();
    for dir in levels {
        let level: u8 = dir.file_name().unwrap().to_str().unwrap().parse().unwrap();
        let mut files = Vec::new();
        fixtures(&dir, &mut files);
        files.sort();
        for f in files {
            let test: Value = serde_json::from_str(&std::fs::read_to_string(&f).unwrap()).unwrap();
            for case in test["cases"].as_array().unwrap() {
                let paths = |k: &str| -> Vec<PathBuf> {
                    case[k]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|p| data.join(p.as_str().unwrap()))
                        .collect()
                };
                let ported = |id: &str, message: &str| {
                    PORTED.contains(&id)
                        || (id == "class.notFound"
                            && GENERICS_NOT_FOUND.iter().any(|p| message.starts_with(p)))
                };
                let want: BTreeSet<Key> = case["errors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|e| {
                        e["identifier"]
                            .as_str()
                            .is_some_and(|id| ported(id, e["message"].as_str().unwrap()))
                    })
                    .map(key)
                    .collect();
                let mut project = Project::new(paths("files"), paths("scan"));
                project.level = level;
                project.check_class_case_sensitivity = level >= 2;
                let got: BTreeSet<Key> = run(&project)
                    .unwrap()
                    .into_iter()
                    .filter(|e| ported(&e.identifier, &e.message))
                    .map(|e| {
                        let file = e.file.strip_prefix(&prefix).unwrap_or(&e.file).to_string();
                        (file, i64::from(e.line), e.identifier, e.message, e.tip)
                    })
                    .collect();
                let name = format!(
                    "{level} {}::{}",
                    test["test"].as_str().unwrap(),
                    case["method"].as_str().unwrap()
                );
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
    }
    let fixed: Vec<_> = gaps.difference(&failing).collect();
    let new: Vec<_> = details
        .iter()
        .filter(|d| !gaps.contains(d.lines().next().unwrap()))
        .collect();
    assert!(
        fixed.is_empty() && new.is_empty(),
        "gaps.txt lists {} passing case(s): {fixed:?}\n{} new failure(s):\n{}",
        fixed.len(),
        new.len(),
        new.iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
