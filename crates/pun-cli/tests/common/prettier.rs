//! Where the Blade tests find prettier: crates/pun-fixers/node_modules of this checkout, or, in a git worktree that
//! has none, that of the main checkout (the parent of `git rev-parse --git-common-dir`).

use std::path::{Path, PathBuf};
use std::process::Command;

/// The canonical node_modules holding prettier, or `None` when neither checkout has one.
pub fn node_modules() -> Option<PathBuf> {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let local = crate_dir.join("../pun-fixers/node_modules");
    let main = Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .current_dir(crate_dir)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| PathBuf::from(String::from_utf8_lossy(&o.stdout).trim()))
        .and_then(|git| Some(git.parent()?.join("crates/pun-fixers/node_modules")));
    [Some(local), main]
        .into_iter()
        .flatten()
        .find(|m| m.join("prettier").exists())
        .map(|m| m.canonicalize().unwrap())
}
