//! Pun's analyse baseline (context/plan/analyse.md Build 2): `pun analyse --generate-baseline` writes every error
//! into pun-baseline.toml, and a run with `[analyse] baseline` drops the errors it lists. The shape is Mago's loose
//! baseline (mago-reporting's `LooseBaseline`): one entry per file, rule and message, with a count, so errors that
//! move to other lines stay covered. A PHPStan baseline is never read (`pun migrate` asks for a new one).

use crate::analyse::Finding;
use crate::render::RunError;
use crate::report::Abort;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// Where `--generate-baseline` writes without `[analyse] baseline` or a path.
pub(crate) const DEFAULT: &str = "pun-baseline.toml";

const HEADER: &str = "# Pun's analyse baseline: errors `pun analyse` doesn't report. Regenerate it with\n# `pun analyse --generate-baseline`.\n\n";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Baseline {
    variant: Variant,
    #[serde(default)]
    issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Variant {
    Loose,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
struct Issue {
    /// Relative to the project root, with `/`.
    file: String,
    /// The Pun rule ID.
    code: String,
    message: String,
    count: u32,
}

impl Baseline {
    /// One entry per (file, rule, message), sorted, counting its errors.
    pub(crate) fn of(findings: &[Finding]) -> Baseline {
        let mut counts: BTreeMap<(String, String, String), u32> = BTreeMap::new();
        for f in findings {
            *counts
                .entry((f.rel.replace('\\', "/"), f.rule.clone(), f.message.clone()))
                .or_default() += 1;
        }
        Baseline {
            variant: Variant::Loose,
            issues: (counts.into_iter())
                .map(|((file, code, message), count)| Issue {
                    file,
                    code,
                    message,
                    count,
                })
                .collect(),
        }
    }

    pub(crate) fn read(path: &Path) -> Result<Baseline> {
        let fail = |message: String| -> anyhow::Error {
            let error = RunError {
                path: Some(path.to_path_buf()),
                line: None,
                message: message.clone(),
                hint: Some("run `pun analyse --generate-baseline` to write it again".into()),
            };
            Abort::pun(format!("{}: {message}", path.display()), error).into()
        };
        let text = std::fs::read_to_string(path)
            .map_err(|e| fail(format!("can't read the baseline: {e}")))?;
        toml::from_str(&text).map_err(|e| fail(format!("invalid baseline: {}", e.message())))
    }

    pub(crate) fn write(&self, path: &Path) -> Result<()> {
        let text = format!("{HEADER}{}", toml::to_string(self)?);
        std::fs::write(path, text).map_err(|e| {
            let message = format!("can't write the baseline to {}: {e}", path.display());
            let error = RunError {
                path: None,
                line: None,
                message: message.clone(),
                hint: None,
            };
            Abort::pun(message, error).into()
        })
    }

    /// Drop the findings the baseline covers, up to each entry's count; the count left over (entries that no
    /// longer match) comes back.
    pub(crate) fn filter(&self, findings: &mut Vec<Finding>) -> usize {
        let mut left: BTreeMap<(String, String, String), u32> = (self.issues.iter())
            .map(|i| ((i.file.clone(), i.code.clone(), i.message.clone()), i.count))
            .collect();
        findings.retain(|f| {
            let key = (f.rel.replace('\\', "/"), f.rule.clone(), f.message.clone());
            match left.get_mut(&key) {
                Some(n) if *n > 0 => {
                    *n -= 1;
                    false
                }
                _ => true,
            }
        });
        left.values().map(|&n| n as usize).sum()
    }
}
