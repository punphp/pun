//! Port of `PhpCsFixer\Fixer\AttributeNotation\GeneralAttributeRemoveFixer`.
use crate::analyzers::attribute_analyzer::AttributeAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use pun_format::Fixer;
use pun_tokens::{Kind::*, NamespacesAnalyzer, Tokens};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

struct GeneralAttributeRemoveFixer {
    attributes: Vec<String>,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(GeneralAttributeRemoveFixer { attributes: vec![] })
}

impl Fixer for GeneralAttributeRemoveFixer {
    fn name(&self) -> &'static str {
        "general_attribute_remove"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["attributes"])?;
        // `class-string[]`, which upstream's resolver checks as `string[]`.
        self.attributes = c.strings("attributes", &[], &[])?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_ATTRIBUTE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        while let Some(found) = tokens.get_next_token_of_kind(index, &[T_ATTRIBUTE.into()], true) {
            let attribute_analysis = AttributeAnalyzer::collect_one(tokens, found);
            let end_index = attribute_analysis.end_index;
            let mut removed_count = 0;
            for element in &attribute_analysis.attributes {
                let fullname =
                    determine_attribute_fully_qualified_name(tokens, &element.name, element.start);
                if !self.attributes.iter().any(|a| a.as_bytes() == fullname) {
                    continue;
                }
                tokens.clear_range(element.start, element.end);
                removed_count += 1;
                let sibling_index = tokens.get_non_empty_sibling(element.end, 1).unwrap();
                // Clear element comma
                if tokens[sibling_index].get_content() == b"," {
                    tokens.clear_at(sibling_index);
                }
            }
            // Clear whole attribute if all are removed (multiline attribute case)
            if attribute_analysis.attributes.len() == removed_count {
                tokens.clear_range(attribute_analysis.start_index, attribute_analysis.end_index);
            }
            // Clear trailing comma
            let token_index = tokens
                .get_meaningful_token_sibling(attribute_analysis.closing_bracket_index, -1)
                .unwrap();
            if tokens[token_index].get_content() == b"," {
                tokens.clear_at(token_index);
            }
            index = end_index;
        }
    }
}

/// `AttributeAnalyzer::determineAttributeFullyQualifiedName`.
fn determine_attribute_fully_qualified_name(
    tokens: &Tokens,
    name: &[u8],
    mut index: usize,
) -> Vec<u8> {
    if name[0] == b'\\' {
        return name.to_vec();
    }
    if !tokens[index].is_given_kind(&[T_STRING, T_NS_SEPARATOR]) {
        index = tokens
            .get_next_token_of_kind(index, &[T_STRING.into(), T_NS_SEPARATOR.into()], true)
            .unwrap();
    }
    // `NamespacesAnalyzer::collectNamespaceAnalysis`: the namespace at `index` and its class imports by short name.
    let namespace_analysis = NamespacesAnalyzer.get_namespace_at(tokens, index);
    let uses: HashMap<Vec<u8>, Vec<u8>> = NamespaceUsesAnalyzer
        .get_declarations_in_namespace(tokens, &namespace_analysis, false)
        .into_iter()
        .filter(|u| u.is_class())
        .map(|u| (u.short_name, u.full_name))
        .collect();
    let mut namespace = namespace_analysis.full_name;
    let first_token_of_name = tokens[index].get_content();
    let mut name = name.to_vec();
    if let Some(full_name) = uses.get(first_token_of_name) {
        namespace = full_name.clone();
        if name == first_token_of_name {
            return namespace;
        }
        // `substr((string) strstr($name, '\\'), 1)`: empty when there is no `\`.
        let after = name
            .iter()
            .position(|&b| b == b'\\')
            .map_or(name.len(), |at| at + 1);
        name = name[after..].to_vec();
    }
    [namespace, b"\\".to_vec(), name].concat()
}
