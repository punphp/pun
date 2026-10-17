//! `mago_fixtures`: Mago's analyser on every case of PHPStan's rule fixtures (tests/fixtures/rules), one JSON line
//! per case (`test`, `method`, `issues`), for scripts/analyse-spike.py.

use std::path::{Path, PathBuf};

use pun_analyse::{Project, run_mago};

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

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/rules");
    let data = root.join("data");
    let prefix = format!("{}/", data.display());
    let mut files = Vec::new();
    fixtures(&root, &mut files);
    files.sort();
    for f in files {
        let test: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&f).unwrap()).unwrap();
        let name = f
            .strip_prefix(&root)
            .unwrap()
            .with_extension("")
            .display()
            .to_string();
        for case in test["cases"].as_array().unwrap() {
            let paths = |k: &str| -> Vec<PathBuf> {
                case[k]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|p| data.join(p.as_str().unwrap()))
                    .collect()
            };
            let run = run_mago(&Project::new(paths("files"), paths("scan")), None);
            let issues: Vec<_> = run
                .issues
                .iter()
                .map(|i| {
                    let file = i.file.strip_prefix(&prefix).unwrap_or(&i.file);
                    serde_json::json!({"file": file, "line": i.line, "code": i.code, "message": i.message})
                })
                .collect();
            println!(
                "{}",
                serde_json::json!({"test": name, "method": case["method"], "issues": issues})
            );
        }
    }
}
