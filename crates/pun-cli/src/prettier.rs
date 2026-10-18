//! Pint's EnsurePrettierIsConfigured: the Node and Prettier checks `Pint/laravel_blade` needs before a run.
use crate::report::Abort;
use std::path::Path;

/// EnsurePrettierIsConfigured, run before anything else when pint.json sets `"Pint/laravel_blade": true` (or
/// `--blade`): Node must exist and prettier, prettier-plugin-blade and prettier-plugin-tailwindcss must resolve
/// from the project at the required versions. Returns the abort (Pint's message and box) when it would abort.
/// ponytail: Pint offers to install missing packages when interactive; Pun always takes the non-interactive "no".
pub fn ensure(root: &Path, enabled: bool) -> Option<Abort> {
    use std::process::{Command, Stdio};
    if !enabled {
        return None;
    }
    const REQUIRED: [(&str, &str); 3] = [
        ("prettier", "^3.8.4"),
        ("prettier-plugin-blade", "^3.2.2"),
        ("prettier-plugin-tailwindcss", "^0.8.0"),
    ];
    // Pint's `node -v` plus resources/js/version-probe.cjs per package, in one Node start: a JSON list with, per
    // package, null when it doesn't resolve, else its version ("" when no manifest names it).
    const PROBE: &str = r#"const path = require('path'); const fs = require('fs');
const probe = (pkg) => { let entry;
  try { entry = require.resolve(pkg, { paths: [process.cwd()] }); } catch (e) { return null; }
  let dir = path.dirname(entry);
  while (dir !== path.dirname(dir)) { const manifest = path.join(dir, 'package.json');
    if (fs.existsSync(manifest)) { try { const json = JSON.parse(fs.readFileSync(manifest, 'utf8'));
      if (json.name === pkg && json.version) { return String(json.version).trim(); } } catch (e) {} }
    dir = path.dirname(dir); }
  return ''; };
process.stdout.write(JSON.stringify(process.argv.slice(1).map(probe)));"#;
    let out = Command::new("node")
        .args(["-e", PROBE])
        .args(REQUIRED.map(|r| r.0))
        .current_dir(root)
        .stderr(Stdio::null())
        .output();
    let probes = out
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| serde_json::from_slice::<Vec<Option<String>>>(&o.stdout).ok());
    let Some(probes) = probes else {
        let mut abort = Abort::pint(
            "The rules enabled in your pint configuration require Node.js to be installed.",
            Some("install Node.js, or turn `format/laravel/blade` off in pun.toml".into()),
        );
        abort.error.message =
            "`format/laravel/blade` needs Node.js, and there is none on PATH".into();
        return Some(abort);
    };
    let manager = if root.join("bun.lock").exists() || root.join("bun.lockb").exists() {
        ("bun", "add -d")
    } else if root.join("pnpm-lock.yaml").exists() {
        ("pnpm", "add -D")
    } else if root.join("yarn.lock").exists() {
        ("yarn", "add -D")
    } else {
        ("npm", "install -D")
    };
    let mut missing = vec![];
    let mut outdated = vec![];
    for ((pkg, constraint), probe) in REQUIRED.into_iter().zip(probes) {
        match probe {
            None => missing.push(pkg),
            Some(v) if !v.is_empty() && !caret_satisfies(&v, &constraint[1..]) => {
                outdated.push((pkg, v, constraint))
            }
            Some(_) => {}
        }
    }
    if !missing.is_empty() {
        let warning = if crate::agent::enabled() {
            String::new()
        } else {
            format!(
                "\n\x1b[33m The rules enabled in your pint configuration require the following prettier dependencies to be installed using [{}]: {}.\x1b[39m\n\n",
                manager.0,
                missing.join(", ")
            )
        };
        let msg = format!(
            "The rules enabled in your pint configuration require the following prettier dependencies to be installed: {}.",
            missing.join(", ")
        );
        let install = format!(
            "install them: {} {} {}",
            manager.0,
            manager.1,
            missing.join(" ")
        );
        let mut abort = Abort::pint(msg, Some(install));
        abort.before = warning;
        abort.error.message = format!(
            "`format/laravel/blade` needs {} in the project",
            missing.join(", ")
        );
        return Some(abort);
    }
    if !outdated.is_empty() {
        let list: Vec<String> = outdated
            .iter()
            .map(|(p, v, c)| format!("  - {p} (installed: {v}, required: {c})"))
            .collect();
        let specs: Vec<String> = outdated
            .iter()
            .map(|(p, _, c)| format!("{p}@{c}"))
            .collect();
        let msg = format!(
            "The following prettier dependencies do not satisfy the versions required by your pint configuration:\n{}\n\nUpdate them using [{}]: {} {} {}",
            list.join("\n"),
            manager.0,
            manager.0,
            manager.1,
            specs.join(" ")
        );
        let have: Vec<String> = (outdated.iter())
            .map(|(p, v, c)| format!("{p} {v} (needs {c})"))
            .collect();
        let mut abort = Abort::pint(
            msg,
            Some(format!(
                "update them: {} {} {}",
                manager.0,
                manager.1,
                specs.join(" ")
            )),
        );
        abort.error.message = format!(
            "`format/laravel/blade` needs newer prettier packages: {}",
            have.join(", ")
        );
        return Some(abort);
    }
    None
}

/// Composer Semver `^x.y.z` for plain `a.b.c` versions.
/// ponytail: pre-release/build suffixes are ignored (Composer treats them by stability); real installs are tagged releases.
fn caret_satisfies(version: &str, base: &str) -> bool {
    // Three parts each, missing ones 0 (`"1"` is 1.0.0), so the prefix compare below can't run past the end.
    let parse = |s: &str| -> Vec<u64> {
        let mut parts: Vec<u64> = (s.trim_start_matches('v').split(['.', '-', '+']))
            .take(3)
            .map(|p| p.parse().unwrap_or(0))
            .collect();
        parts.resize(3, 0);
        parts
    };
    let (v, b) = (parse(version), parse(base));
    let first = b.iter().position(|&n| n != 0).unwrap_or(2);
    v >= b && v[..=first] == b[..=first]
}

#[cfg(test)]
mod tests {
    /// npm's caret ranges, derived by hand from node-semver's rules: `^0.8.0` is `>=0.8.0 <0.9.0`, `^3.0.0` is
    /// `>=3.0.0 <4.0.0`; a short version is padded with zeros instead of panicking (AUDIT item 10).
    #[test]
    fn caret_ranges() {
        use super::caret_satisfies as ok;
        assert!(ok("0.8.3", "0.8.0"));
        assert!(!ok("0.9.0", "0.8.0"));
        assert!(!ok("1", "0.8.0"));
        assert!(ok("3.4", "3.0.0"));
        assert!(!ok("4", "3.0.0"));
        assert!(!ok("2.9.9", "3.0.0"));
    }
}
