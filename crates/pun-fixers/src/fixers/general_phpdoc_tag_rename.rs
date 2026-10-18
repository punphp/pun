//! Port of `PhpCsFixer\Fixer\Phpdoc\GeneralPhpdocTagRenameFixer` (its code is in analyzers/general_phpdoc_tag_rename.rs).
use crate::analyzers::general_phpdoc_tag_rename::GeneralPhpdocTagRenameFixer;
use pun_format::Fixer;

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(GeneralPhpdocTagRenameFixer::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use serde_json::json;

    // Accept/reject and messages from the real fixer's configure() (PHP-CS-Fixer 3.95).
    #[test]
    fn configure_validation() {
        let err = |c: Value| fixer().configure(&c).unwrap_err().to_string();
        let p = "[general_phpdoc_tag_rename] Invalid configuration: ";
        assert_eq!(
            err(json!({"replacements": ["b"]})),
            format!("{p}Tag to replace must be a string.")
        );
        assert_eq!(
            err(json!({"replacements": {"a": "b c"}})),
            format!("{p}Tag \"a\" cannot be replaced by invalid tag \"b c\".")
        );
        assert_eq!(
            err(json!({"replacements": {"a": "b*/"}})),
            format!("{p}Tag \"a\" cannot be replaced by invalid tag \"b*/\".")
        );
        assert_eq!(
            err(json!({"replacements": {"A": "b", "a": "c"}})),
            format!(
                "{p}Tag \"a\" cannot be configured to be replaced with several different tags when case sensitivity is off."
            )
        );
        assert_eq!(
            err(json!({"replacements": {"a": "b", "b": "c"}})),
            format!(
                "{p}Cannot change tag \"a\" to tag \"b\", as the tag \"b\" is configured to be replaced to \"c\"."
            )
        );
        for ok in [
            json!({"replacements": {"A": "b", "a": "b"}}),
            json!({"replacements": {"a": "b", "b": "b"}}),
            json!({"replacements": []}),
        ] {
            assert!(fixer().configure(&ok).is_ok(), "{ok}");
        }
    }
}
