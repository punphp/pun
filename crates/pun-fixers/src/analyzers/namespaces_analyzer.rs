//! Port of `PhpCsFixer\Tokenizer\Analyzer\NamespacesAnalyzer`.
use crate::analyzers::analysis::NamespaceAnalysis;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use pun_tokens::php_compat::trim;
use std::sync::Arc;

pub struct NamespacesAnalyzer;

impl NamespacesAnalyzer {
    pub fn get_declarations(&self, tokens: &Tokens) -> Vec<NamespaceAnalysis> {
        self.declarations(tokens).to_vec()
    }

    /// `Tokens::getNamespaceDeclarations()`: computed once per token-list version (see `Tokens::namespace_declarations`).
    pub fn declarations(&self, tokens: &Tokens) -> Arc<Vec<NamespaceAnalysis>> {
        tokens.namespace_declarations(Self::compute)
    }

    fn compute(tokens: &Tokens) -> Vec<NamespaceAnalysis> {
        let mut namespaces = Vec::new();
        let count = tokens.count();
        let mut index = 1;
        while index < count {
            if !tokens[index].is_kind(T_NAMESPACE) {
                index += 1;
                continue;
            }
            let declaration_end_index = tokens.get_next_token_of_kind(index, &[";".into(), "{".into()], true).unwrap();
            let namespace = trim(&tokens.generate_partial_code(index + 1, declaration_end_index - 1)).to_vec();
            let short_name = namespace.rsplit(|&c| c == b'\\').next().unwrap_or(&[]).to_vec();
            let scope_end_index = if tokens[declaration_end_index].equals("{", true) {
                tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, declaration_end_index)
            } else {
                tokens.get_next_token_of_kind(declaration_end_index, &[T_NAMESPACE.into()], true).unwrap_or(count) - 1
            };
            namespaces.push(NamespaceAnalysis {
                full_name: namespace,
                short_name,
                start_index: index,
                end_index: declaration_end_index,
                scope_start_index: index,
                scope_end_index,
            });
            // Continue the analysis after the end of this namespace to find the next one
            index = scope_end_index + 1;
        }
        if namespaces.is_empty() && tokens.is_token_kind_found(T_OPEN_TAG) {
            let open_tag_index = if tokens[0].is_kind(T_INLINE_HTML) { 1 } else { 0 };
            namespaces.push(NamespaceAnalysis {
                full_name: Vec::new(),
                short_name: Vec::new(),
                start_index: open_tag_index,
                end_index: open_tag_index,
                scope_start_index: open_tag_index,
                scope_end_index: count - 1,
            });
        }
        namespaces
    }

    pub fn get_namespace_at(&self, tokens: &Tokens, index: usize) -> NamespaceAnalysis {
        assert!(index < tokens.len(), "Token index {index} does not exist.");
        self.get_declarations(tokens)
            .into_iter()
            .find(|ns| ns.scope_start_index <= index && ns.scope_end_index >= index)
            .unwrap_or_else(|| panic!("Unable to get the namespace at index {index}."))
    }
}
