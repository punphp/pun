// Registers every src/fixers/<name>.rs. Each file exposes `pub fn fixer() -> Box<dyn pun_format::Fixer>`.
// PUN_FIXERS_ONLY=a,b compiles only those fixers, so one agent's half-done file can't break another's build.
use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=src/fixers");
    println!("cargo:rerun-if-env-changed=PUN_FIXERS_ONLY");
    let only: Option<Vec<String>> = env::var("PUN_FIXERS_ONLY")
        .ok()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(',').map(|x| x.trim().to_string()).collect());
    let mut names: Vec<String> = fs::read_dir("src/fixers")
        .unwrap()
        .filter_map(|e| {
            e.ok()?
                .file_name()
                .into_string()
                .ok()?
                .strip_suffix(".rs")
                .map(String::from)
        })
        .filter(|n| only.as_ref().is_none_or(|o| o.contains(n)))
        .collect();
    names.sort();
    let mut out = String::new();
    let dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/fixers");
    for n in &names {
        // `{:?}` writes a Rust string literal, so a checkout path holding `"` or `\` still compiles.
        let path = dir.join(format!("{n}.rs")).display().to_string();
        out += &format!("#[path = {path:?}] mod {n};\n");
    }
    out += "pub fn all() -> Vec<Box<dyn pun_format::Fixer>> { vec![";
    for n in &names {
        out += &format!("{n}::fixer(),");
    }
    out += "] }\n";
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("fixers.rs"),
        out,
    )
    .unwrap();
}
