//! The core of `PhpCsFixer\Fixer\ClassNotation\FinalInternalClassFixer`, shared by the `final_internal_class`
//! fixer and `final_class` (upstream's proxy to it), so each fixer builds on its own under `PUN_FIXERS_ONLY`.
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use crate::analyzers::utils::natural_language_join;
use anyhow::bail;
use pun_tokens::Kind::{self, *};
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::BTreeMap;

const DEFAULT_INCLUDE: &[&str] = &["internal"];
const DEFAULT_EXCLUDE: &[&str] = &[
    "final",
    "Entity",
    "ORM\\Entity",
    "ORM\\Mapping\\Entity",
    "Mapping\\Entity",
    "Document",
    "ODM\\Document",
];

const CLASS_CANDIDATE_ACCEPT_TYPES: &[Kind] =
    &[CT_ATTRIBUTE_CLOSE, T_DOC_COMMENT, T_COMMENT, T_READONLY];

/// A normalized annotation list: lowercased keys (leading `@` dropped) in PHP array insertion order.
type Keys = Vec<Vec<u8>>;

#[derive(Default)]
pub(crate) struct FinalInternalClassFixer {
    include: Keys,
    exclude: Keys,
    consider_absent_docblock_as_internal_class: bool,
    check_attributes: bool,
}

fn normalize<S: AsRef<[u8]>>(values: &[S]) -> Keys {
    let mut out: Keys = Vec::new();
    for v in values {
        let v = v.as_ref();
        let key = v.strip_prefix(b"@").unwrap_or(v).to_ascii_lowercase();
        if !out.contains(&key) {
            out.push(key);
        }
    }
    out
}

fn intersects(a: &Keys, b: &BTreeMap<Vec<u8>, bool>) -> bool {
    a.iter().any(|k| b.contains_key(k))
}

impl FinalInternalClassFixer {
    pub(crate) fn configure_options(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(
            "final_internal_class",
            options,
            &[
                "annotation_include",
                "annotation_exclude",
                "include",
                "exclude",
                "consider_absent_docblock_as_internal_class",
            ],
        )?;
        let list = |key: &str, default: &[&str], at: bool| -> anyhow::Result<Keys> {
            let defaults: Vec<String> = default
                .iter()
                .map(|s| if at { format!("@{s}") } else { s.to_string() })
                .collect();
            let defaults: Vec<&str> = defaults.iter().map(String::as_str).collect();
            let values = c.strings(key, &defaults, &[])?;
            if values.iter().any(|v| v.is_empty()) {
                return Err(c.invalid(key, c.get(key).unwrap(), &[]));
            }
            Ok(normalize(&values))
        };
        let mut include = list("include", DEFAULT_INCLUDE, false)?;
        let mut exclude = list("exclude", DEFAULT_EXCLUDE, false)?;
        let annotation_include = list("annotation_include", DEFAULT_INCLUDE, true)?;
        let annotation_exclude = list("annotation_exclude", DEFAULT_EXCLUDE, true)?;
        self.consider_absent_docblock_as_internal_class =
            c.bool("consider_absent_docblock_as_internal_class", false)?;
        self.check_attributes = true;
        // assertConfigHasNoConflicts
        for (new, old, new_key, old_key, defaults) in [
            (
                &mut include,
                annotation_include,
                "include",
                "annotation_include",
                DEFAULT_INCLUDE,
            ),
            (
                &mut exclude,
                annotation_exclude,
                "exclude",
                "annotation_exclude",
                DEFAULT_EXCLUDE,
            ),
        ] {
            let defaults = normalize(defaults);
            let new_config_is_set = *new != defaults;
            let old_config_is_set = old != defaults;
            if new_config_is_set && old_config_is_set {
                bail!(
                    "[final_internal_class] Configuration cannot contain deprecated option \"{old_key}\" and new option \"{new_key}\"."
                );
            }
            if old_config_is_set {
                *new = old;
                self.check_attributes = false; // run in old mode
            }
        }
        let dup: Vec<String> = include
            .iter()
            .filter(|k| exclude.contains(k))
            .map(|k| String::from_utf8_lossy(k).into_owned())
            .collect();
        if !dup.is_empty() {
            let joined = natural_language_join(&dup);
            bail!(
                "[final_internal_class] Annotation cannot be used in both \"include\" and \"exclude\" list, got duplicates: {joined}."
            );
        }
        self.include = include;
        self.exclude = exclude;
        Ok(())
    }

    pub(crate) fn apply_fix(&self, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_kind(T_CLASS) || !self.is_class_candidate(tokens, index) {
                continue;
            }
            let mut slices = BTreeMap::new();
            slices.insert(
                index,
                vec![Token::new(T_FINAL, "final"), Token::new(T_WHITESPACE, " ")],
            );
            tokens.insert_slices(slices);
        }
    }

    fn is_class_candidate(&self, tokens: &Tokens, index: usize) -> bool {
        let tokens_analyzer = TokensAnalyzer::new(tokens);
        if tokens_analyzer.is_anonymous_class(index) {
            return false;
        }
        let modifiers = tokens_analyzer.get_classy_modifiers(index);
        if modifiers.final_.is_some() || modifiers.abstract_.is_some() {
            return false; // ignore class; it is abstract or already final
        }
        let mut decisions: Vec<Option<bool>> = Vec::new();
        let mut current_index = Some(index);
        while let Some(ci) = current_index {
            let mut ci = match tokens.get_prev_non_whitespace(ci, None) {
                Some(c) if tokens[c].is_given_kind(CLASS_CANDIDATE_ACCEPT_TYPES) => c,
                _ => break,
            };
            if self.check_attributes && tokens[ci].is_kind(CT_ATTRIBUTE_CLOSE) {
                let attribute_start_index =
                    tokens.find_block_start(Tokens::BLOCK_TYPE_ATTRIBUTE, ci);
                decisions.push(self.is_class_candidate_based_on_attribute(
                    tokens,
                    attribute_start_index,
                    ci,
                ));
                ci = attribute_start_index;
            }
            if tokens[ci].is_kind(T_DOC_COMMENT) {
                decisions.push(self.is_class_candidate_based_on_php_doc(tokens, ci));
            }
            current_index = Some(ci);
        }
        if decisions.contains(&Some(false)) {
            return false;
        }
        decisions.contains(&Some(true))
            || (decisions.is_empty() && self.consider_absent_docblock_as_internal_class)
    }

    fn is_class_candidate_based_on_php_doc(&self, tokens: &Tokens, index: usize) -> Option<bool> {
        let doc = DocBlock::new(&tokens[index].content);
        let mut tags = BTreeMap::new();
        for annotation in doc.get_annotations() {
            let content = annotation.get_content();
            let Some(m) = Preg::match_groups("/@([^\\(\\s]+)/", &content) else {
                continue;
            };
            tags.insert(m.str(0)[1..].to_ascii_lowercase(), true);
        }
        if intersects(&self.exclude, &tags) {
            return Some(false);
        }
        if self.is_configured_as_include(&tags) {
            return Some(true);
        }
        None
    }

    fn is_class_candidate_based_on_attribute(
        &self,
        tokens: &Tokens,
        start_index: usize,
        end_index: usize,
    ) -> Option<bool> {
        let mut attribute_candidates = BTreeMap::new();
        let mut attribute_string: Vec<u8> = Vec::new();
        let mut current_index = start_index;
        while current_index < end_index
            && let Some(ci) = tokens.get_next_meaningful_token(current_index)
        {
            current_index = ci;
            if !tokens[ci].is_given_kind(&[T_STRING, T_NS_SEPARATOR]) {
                if !attribute_string.is_empty() {
                    attribute_candidates.insert(std::mem::take(&mut attribute_string), true);
                }
                continue;
            }
            attribute_string.extend(tokens[ci].content.to_ascii_lowercase());
        }
        if intersects(&self.exclude, &attribute_candidates) {
            return Some(false);
        }
        if self.is_configured_as_include(&attribute_candidates) {
            return Some(true);
        }
        None
    }

    fn is_configured_as_include(&self, attributes: &BTreeMap<Vec<u8>, bool>) -> bool {
        self.include.is_empty() || intersects(&self.include, attributes)
    }
}
