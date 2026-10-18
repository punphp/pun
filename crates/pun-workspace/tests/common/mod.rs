//! Shared by the file-listing tests.
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A scratch tree `pun-<prefix>-<pid>-<n>` under the temp dir holding each of `files` with `content`, canonical.
pub fn tree(prefix: &str, files: &[&str], content: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let root = std::env::temp_dir().join(format!(
        "pun-{prefix}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    for f in files {
        let p = root.join(f);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, content).unwrap();
    }
    root.canonicalize().unwrap()
}
