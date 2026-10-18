//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocTagCasingFixer` (an AbstractProxyFixer over general_phpdoc_tag_rename).
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::general_phpdoc_tag_rename::GeneralPhpdocTagRenameFixer;
use anyhow::{Result, anyhow};
use pun_format::Fixer;
use pun_tokens::Tokens;
use serde_json::{Map, Value, json};
use std::path::Path;

#[derive(Default)]
struct PhpdocTagCasingFixer {
    proxy: GeneralPhpdocTagRenameFixer,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocTagCasingFixer::default())
}

impl Fixer for PhpdocTagCasingFixer {
    fn name(&self) -> &'static str {
        "phpdoc_tag_casing"
    }
    fn priority(&self) -> i32 {
        // AbstractProxyFixer: the proxied general_phpdoc_tag_rename's priority.
        self.proxy.priority()
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["tags"])?;
        let tags = r.strings("tags", &["inheritDoc"], &[])?;
        // configurePostNormalisation: `$replacements[$tag] = $tag`.
        let mut replacements = Map::new();
        for tag in tags {
            replacements.insert(tag.clone(), Value::String(tag));
        }
        self.proxy
            .configure(&json!({
                "case_sensitive": false,
                "fix_annotation": true,
                "fix_inline": true,
                "replacements": replacements,
            }))
            .map_err(|e| {
                let message = e.to_string();
                let message = message
                    .strip_prefix("[general_phpdoc_tag_rename] ")
                    .unwrap_or(&message);
                anyhow!("[{}] {message}", self.name())
            })
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        self.proxy.is_candidate(tokens)
    }
    fn fix(&self, file: &Path, tokens: &mut Tokens) {
        self.proxy.fix(file, tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// PhpdocTagCasingFixerTest::provideInvalidConfigurationCases: the messages the real fixer's configure() throws.
    #[test]
    fn invalid_configuration() {
        let raw = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../pun-harness/fixtures/phpdoc_tag_casing.json"
        ))
        .unwrap();
        let doc: Value = serde_json::from_slice(&raw).unwrap();
        let test = doc["tests"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["method"] == "testInvalidConfiguration")
            .unwrap();
        let cases = test["cases"].as_array().unwrap();
        assert!(!cases.is_empty());
        for case in cases {
            let (config, message) = (&case["args"][0], case["args"][1].as_str().unwrap());
            let error = fixer().configure(config).unwrap_err().to_string();
            assert!(error.contains(message), "{config}: {error} lacks {message}");
        }
    }
}
