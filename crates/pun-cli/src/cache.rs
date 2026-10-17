//! Content-hash cache, PHP-CS-Fixer's `.php-cs-fixer.cache` idea: a JSON file with a signature (Pun build, target PHP,
//! whitespace config, preset, engine, resolved rules + options, Blade toolchain lockfile) and, per file path, the
//! xxh3-128 of content that was clean under exactly that signature. Such a file is skipped without lexing.
//!
//! Pint itself never caches (its presets call `setUsingCache(false)`, so the `--cache-file` it computes is unused).
//! Pun keeps Pint's `--cache-file` flag for the location and otherwise writes to
//! `${XDG_CACHE_HOME:-~/.cache}/pun/<xxh3 of the project root>.json`, so nothing lands in the project. `--no-cache`
//! turns it off. Only files that came out clean are stored: never fixed (single-pass fixers need not be idempotent),
//! failing or skipped ones. A missing, unreadable, corrupt or differently-signed file means an empty cache.
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use xxhash_rust::xxh3::xxh3_128;

pub struct Cache {
    path: PathBuf,
    signature: Value,
    hashes: BTreeMap<String, String>,
}

pub fn hash(code: &[u8]) -> String {
    format!("{:032x}", xxh3_128(code))
}

/// Default location for `root`, or `None` without a home directory.
pub fn default_path(root: &Path) -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .filter(|v| !v.is_empty())
                .map(|h| PathBuf::from(h).join(".cache"))
        })?;
    Some(base.join("pun").join(format!(
        "{:016x}.json",
        xxhash_rust::xxh3::xxh3_64(root.as_os_str().as_encoded_bytes())
    )))
}

/// What a clean result depends on besides the file itself. The binary's size and mtime stand in for the code, so a
/// rebuild with the same version number still invalidates.
pub fn signature(cfg: &pun_config::Config) -> Value {
    let exe = std::env::current_exe().and_then(std::fs::metadata).ok();
    let build = exe.map(|m| {
        (
            m.len(),
            m.modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_nanos()),
        )
    });
    let rules: Vec<Value> = cfg
        .rules
        .iter()
        .map(|r| json!([r.name, r.options]))
        .collect();
    // Pint/laravel_blade output follows the project's prettier install: any lockfile change invalidates.
    let blade_toolchain: Vec<Value> = [
        "node_modules/.package-lock.json",
        "package-lock.json",
        "yarn.lock",
        "pnpm-lock.yaml",
        "bun.lock",
    ]
    .iter()
    .filter(|_| cfg.rules.iter().any(|r| r.name == "Pint/laravel_blade"))
    .filter_map(|f| {
        std::fs::read(cfg.root.join(f))
            .ok()
            .map(|c| json!([f, hash(&c)]))
    })
    .collect();
    json!({
        "version": format!("{} {build:?}", env!("CARGO_PKG_VERSION")),
        "php": cfg.php,
        "indent": "    ",
        "lineEnding": "\n",
        "preset": cfg.preset,
        "engine": format!("{:?}", cfg.engine),
        "rules": rules,
        "bladeToolchain": blade_toolchain,
    })
}

impl Cache {
    pub fn load(path: PathBuf, signature: Value) -> Cache {
        let mut hashes = BTreeMap::new();
        if let Some(mut v) = std::fs::read(&path)
            .ok()
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        {
            let stored = v.as_object_mut().and_then(|o| o.remove("hashes"));
            if v == signature {
                hashes = stored
                    .and_then(|h| serde_json::from_value(h).ok())
                    .unwrap_or_default();
            }
        }
        Cache {
            path,
            signature,
            hashes,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// `rel` had exactly this content (`hash`) the last time it was found clean.
    pub fn is_clean(&self, rel: &str, hash: &str) -> bool {
        self.hashes.get(rel).is_some_and(|h| h == hash)
    }

    /// Records this run's outcome per file (`Some(hash)` = clean) and writes the file (best effort, atomic rename).
    /// Entries for files outside this run (e.g. `--dirty`) are kept.
    pub fn save(mut self, outcomes: impl IntoIterator<Item = (String, Option<String>)>) {
        for (rel, clean) in outcomes {
            match clean {
                Some(h) => self.hashes.insert(rel, h),
                None => self.hashes.remove(&rel),
            };
        }
        let mut v = self.signature;
        v["hashes"] = json!(self.hashes);
        let tmp = self
            .path
            .with_extension(format!("tmp{}", std::process::id()));
        let written = self
            .path
            .parent()
            .is_some_and(|d| std::fs::create_dir_all(d).is_ok())
            && std::fs::write(&tmp, v.to_string()).is_ok()
            && std::fs::rename(&tmp, &self.path).is_ok();
        if !written {
            let _ = std::fs::remove_file(&tmp);
        }
    }
}
