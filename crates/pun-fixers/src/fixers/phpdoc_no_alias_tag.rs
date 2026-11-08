//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocNoAliasTagFixer` (an AbstractProxyFixer over general_phpdoc_tag_rename).
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::general_phpdoc_tag_rename::GeneralPhpdocTagRenameFixer;
use anyhow::{Result, anyhow};
use pun_format::Fixer;
use pun_tokens::Tokens;
use serde_json::{Value, json};
use std::path::Path;

#[derive(Default)]
struct PhpdocNoAliasTagFixer {
    proxy: GeneralPhpdocTagRenameFixer,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocNoAliasTagFixer::default())
}

impl Fixer for PhpdocNoAliasTagFixer {
    fn name(&self) -> &'static str {
        "phpdoc_no_alias_tag"
    }
    fn priority(&self) -> i32 {
        // AbstractProxyFixer: the proxied general_phpdoc_tag_rename's priority.
        self.proxy.priority()
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["replacements"])?;
        // `array<string, string>`, which the resolver checks as `string[]`.
        r.check_types("replacements", &["string[]"])?;
        // Future::getV4OrV3(['const' => 'var'], []) + [...]: Pint runs without future mode.
        let replacements = r.get("replacements").cloned().unwrap_or_else(|| {
            json!({
                "property-read": "property",
                "property-write": "property",
                "type": "var",
                "link": "see",
            })
        });
        // configurePostNormalisation: re-label the proxied fixer's exception (`Preg::replace('/^\[.+?\] /', '', ...)`).
        self.proxy
            .configure(&json!({
                "fix_annotation": true,
                "fix_inline": false,
                "replacements": replacements,
                "case_sensitive": true,
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

    /// PhpdocNoAliasTagFixerTest::provideInvalidConfigurationCases: the messages the real fixer's configure() throws.
    #[test]
    fn invalid_configuration() {
        let raw = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/phpdoc_no_alias_tag.json"
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
        assert_eq!(cases.len(), 7);
        for case in cases {
            let (config, pattern) = (&case["args"][0], case["args"][1].as_str().unwrap());
            let message = fixer().configure(config).unwrap_err().to_string();
            assert!(
                crate::analyzers::preg::Preg::r#match(pattern, &message),
                "{config}: {message} !~ {pattern}"
            );
        }
    }
}
