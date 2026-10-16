//! Port of `PhpCsFixer\Fixer\ClassNotation\SingleTraitInsertPerStatementFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

struct SingleTraitInsertPerStatementFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SingleTraitInsertPerStatementFixer)
}

impl Fixer for SingleTraitInsertPerStatementFixer {
    fn name(&self) -> &'static str {
        "single_trait_insert_per_statement"
    }
    /// Must run before BracesFixer, SpaceAfterSemicolonFixer.
    fn priority(&self) -> i32 {
        36
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(CT_USE_TRAIT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.len() - 1;
        while 1 < index {
            if tokens[index].is_kind(CT_USE_TRAIT) {
                let candidates = get_candidates(tokens, index);
                fix_trait_use(tokens, index, &candidates);
            }
            index -= 1;
        }
    }
}

/// `candidates`: ',' indices to fix, last first.
fn fix_trait_use(tokens: &mut Tokens, use_trait_index: usize, candidates: &[usize]) {
    for &comma_index in candidates {
        let mut inserts = vec![Token::new(CT_USE_TRAIT, "use"), Token::new(T_WHITESPACE, " ")];
        let next_import_start_index = tokens.get_next_meaningful_token(comma_index).unwrap();
        if tokens[next_import_start_index - 1].is_whitespace(None) {
            if Preg::r#match("/\\R/", tokens[next_import_start_index - 1].get_content()) {
                inserts.insert(0, tokens[use_trait_index - 1].clone());
            }
            tokens.clear_at(next_import_start_index - 1);
        }
        tokens.set(comma_index, Token::char(";"));
        tokens.insert_at(next_import_start_index, inserts);
    }
}

fn get_candidates(tokens: &Tokens, index: usize) -> Vec<usize> {
    let kinds = [",".into(), ";".into(), "{".into()];
    let mut indices = vec![];
    let mut index = tokens.get_next_token_of_kind(index, &kinds, true).unwrap();
    while !tokens[index].equals(";", true) {
        if tokens[index].equals("{", true) {
            return vec![]; // do not fix use cases with grouping
        }
        indices.push(index);
        index = tokens.get_next_token_of_kind(index, &kinds, true).unwrap();
    }
    indices.reverse();
    indices
}
