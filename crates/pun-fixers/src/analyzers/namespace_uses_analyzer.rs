//! Port of `PhpCsFixer\Tokenizer\Analyzer\NamespaceUsesAnalyzer`.
use crate::analyzers::analysis::{NamespaceAnalysis, NamespaceUseAnalysis, NamespaceUseType};
use pun_tokens::Kind::*;
use pun_tokens::{Tokens, TokensAnalyzer};

pub struct NamespaceUsesAnalyzer;

struct QualifiedName {
    full_name: Vec<u8>,
    short_name: Vec<u8>,
    aliased: bool,
    after_index: Option<usize>,
}

impl NamespaceUsesAnalyzer {
    pub fn get_declarations_from_tokens(&self, tokens: &Tokens, allow_multi_uses: bool) -> Vec<NamespaceUseAnalysis> {
        let use_indices = TokensAnalyzer::new(tokens).get_import_use_indexes(false).remove(&0).unwrap_or_default();
        self.get_declarations(tokens, &use_indices, allow_multi_uses)
    }

    pub fn get_declarations_in_namespace(
        &self,
        tokens: &Tokens,
        namespace: &NamespaceAnalysis,
        allow_multi_uses: bool,
    ) -> Vec<NamespaceUseAnalysis> {
        self.get_declarations_from_tokens(tokens, allow_multi_uses)
            .into_iter()
            .filter(|u| u.start_index >= namespace.scope_start_index && u.start_index <= namespace.scope_end_index)
            .collect()
    }

    fn get_declarations(&self, tokens: &Tokens, use_indices: &[usize], allow_multi_uses: bool) -> Vec<NamespaceUseAnalysis> {
        let mut uses = Vec::new();
        for &index in use_indices {
            let end_index = tokens.get_next_token_of_kind(index, &[";".into(), T_CLOSE_TAG.into()], true).unwrap();
            let mut declarations = self.parse_declarations(index, end_index, tokens);
            if !allow_multi_uses {
                declarations.retain(|d| !d.is_in_multi);
            }
            uses.extend(declarations);
        }
        uses
    }

    fn parse_declarations(&self, start_index: usize, end_index: usize, tokens: &Tokens) -> Vec<NamespaceUseAnalysis> {
        let type_ = self.determine_import_type(tokens, start_index);
        let potential_multi = tokens.get_next_token_of_kind(start_index, &[",".into(), CT_GROUP_IMPORT_BRACE_OPEN.into()], true);
        let multi = potential_multi.is_some_and(|p| p < end_index);
        let mut index = tokens.get_next_token_of_kind(start_index, &[T_STRING.into(), T_NS_SEPARATOR.into()], true);
        let mut imports = Vec::new();
        while let Some(i) = index.filter(|&i| i <= end_index) {
            let mut i = i;
            let qualified_name = self.get_nearest_qualified_name(tokens, i);
            let after = qualified_name.after_index.unwrap();
            let token = &tokens[after];
            if token.is_kind(CT_GROUP_IMPORT_BRACE_OPEN) {
                let group_start = after;
                let mut group_index = after;
                let group_end = tokens.find_block_end(Tokens::BLOCK_TYPE_GROUP_IMPORT_BRACE, group_start);
                while group_index < group_end {
                    let chunk_start = tokens.get_next_meaningful_token(group_index).unwrap();
                    // Finish parsing on trailing comma (no more chunks there)
                    if tokens[chunk_start].is_kind(CT_GROUP_IMPORT_BRACE_CLOSE) {
                        break;
                    }
                    let group_qualified_name = self.get_nearest_qualified_name(tokens, chunk_start);
                    let group_after = group_qualified_name.after_index.unwrap();
                    imports.push(NamespaceUseAnalysis {
                        type_,
                        full_name: [&qualified_name.full_name[..], &group_qualified_name.full_name[..]].concat(),
                        short_name: group_qualified_name.short_name,
                        is_aliased: group_qualified_name.aliased,
                        is_in_multi: true,
                        start_index,
                        end_index,
                        chunk_start_index: Some(chunk_start),
                        chunk_end_index: tokens.get_prev_meaningful_token(group_after),
                    });
                    group_index = group_after;
                }
                i = group_index;
            } else if token.equals_any(&[",".into(), ";".into(), T_CLOSE_TAG.into()], true) {
                let previous_token = tokens.get_prev_meaningful_token(after).unwrap();
                if !tokens[previous_token].is_kind(CT_GROUP_IMPORT_BRACE_CLOSE) {
                    imports.push(NamespaceUseAnalysis {
                        type_,
                        full_name: qualified_name.full_name,
                        short_name: qualified_name.short_name,
                        is_aliased: qualified_name.aliased,
                        is_in_multi: multi,
                        start_index,
                        end_index,
                        chunk_start_index: multi.then_some(i),
                        chunk_end_index: multi.then_some(previous_token),
                    });
                }
                i = after;
            }
            index = tokens.get_next_meaningful_token(i);
        }
        imports
    }

    fn determine_import_type(&self, tokens: &Tokens, start_index: usize) -> NamespaceUseType {
        let potential_type = &tokens[tokens.get_next_meaningful_token(start_index).unwrap()];
        if potential_type.is_kind(CT_FUNCTION_IMPORT) {
            return NamespaceUseType::Function;
        }
        if potential_type.is_kind(CT_CONST_IMPORT) {
            return NamespaceUseType::Constant;
        }
        NamespaceUseType::Class
    }

    fn get_nearest_qualified_name(&self, tokens: &Tokens, index: usize) -> QualifiedName {
        let (mut full_name, mut short_name) = (Vec::new(), Vec::new());
        let mut aliased = false;
        let mut index = Some(index);
        while let Some(i) = index {
            let token = &tokens[i];
            if token.is_kind(T_STRING) {
                short_name = token.content.clone();
                if !aliased {
                    full_name.extend_from_slice(&short_name);
                }
            } else if token.is_kind(T_NS_SEPARATOR) {
                full_name.extend_from_slice(&token.content);
            } else if token.is_kind(T_AS) {
                aliased = true;
            } else if token.equals_any(
                &[",".into(), ";".into(), CT_GROUP_IMPORT_BRACE_OPEN.into(), CT_GROUP_IMPORT_BRACE_CLOSE.into(), T_CLOSE_TAG.into()],
                true,
            ) {
                break;
            }
            index = tokens.get_next_meaningful_token(i);
        }
        QualifiedName { full_name, short_name, aliased, after_index: index }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzers::namespaces_analyzer::NamespacesAnalyzer;

    #[test]
    fn declarations() {
        let t = Tokens::from_code(b"<?php namespace A\\B; use Foo\\Bar as Baz, Qux; use function f\\{g, h as i,}; class C { use T; }");
        let ns = NamespacesAnalyzer.get_declarations(&t);
        assert_eq!(ns.len(), 1);
        assert_eq!(ns[0].full_name, b"A\\B");
        assert_eq!(ns[0].short_name, b"B");
        let all = NamespaceUsesAnalyzer.get_declarations_from_tokens(&t, true);
        let names: Vec<(String, String, bool)> = all
            .iter()
            .map(|u| (String::from_utf8_lossy(&u.full_name).into(), String::from_utf8_lossy(&u.short_name).into(), u.is_aliased))
            .collect();
        assert_eq!(
            names,
            [
                ("Foo\\Bar".into(), "Baz".into(), true),
                ("Qux".into(), "Qux".into(), false),
                ("f\\g".into(), "g".into(), false),
                ("f\\h".into(), "i".into(), true)
            ]
        );
        assert!(all[2].is_function());
        assert!(NamespaceUsesAnalyzer.get_declarations_from_tokens(&t, false).is_empty());
    }
}
