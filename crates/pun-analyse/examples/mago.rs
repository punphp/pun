//! `mago <project dir> <phpstan.neon> [--laravel]`: Mago's analyser on the files the config analyses, as JSON
//! (`issues`, `dumped`), for the engine spike (context/reports/analyse-spike.md). `--laravel` adds Pun's Laravel
//! plugin. Paths are relative to the project dir.

use std::path::Path;

use pun_analyse::{Laravel, Project, run_mago};
use pun_phpstan::config::{Options, resolve};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = Path::new(&args[1]);
    let options = Options {
        config: Some(args[2].clone()),
        ..Options::for_project(&args[1])
    };
    let config = resolve(&options).unwrap_or_else(|e| panic!("{e}"));
    let project = Project::from_config(root, &config.parameters);
    let laravel = args.iter().any(|a| a == "--laravel");
    let app = Laravel {
        root,
        parse_casts_method: false,
    };
    let run = run_mago(&project, laravel.then_some(app));
    let prefix = format!("{}/", root.display());
    let rel = |f: &str| f.strip_prefix(&prefix).unwrap_or(f).to_string();
    let issues: Vec<_> = run
        .issues
        .iter()
        .map(|i| serde_json::json!({"file": rel(&i.file), "line": i.line, "code": i.code, "message": i.message}))
        .collect();
    let dumped: Vec<_> = run
        .dumped
        .iter()
        .map(|d| serde_json::json!({"file": rel(&d.file), "line": d.line, "type": d.type_id}))
        .collect();
    println!(
        "{}",
        serde_json::json!({ "issues": issues, "dumped": dumped })
    );
}
