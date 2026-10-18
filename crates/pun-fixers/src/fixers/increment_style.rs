//! Port of `PhpCsFixer\Fixer\Operator\IncrementStyleFixer`.
use crate::analyzers::abstract_increment_operator_fixer::find_start;
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct IncrementStyleFixer {
    pre: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(IncrementStyleFixer::default())
}

const STATEMENT_START: [Proto<'static>; 5] = [
    Proto::Char(b";"),
    Proto::Char(b"{"),
    Proto::Char(b"}"),
    Proto::Kind(T_OPEN_TAG),
    Proto::Char(b")"),
];
const STATEMENT_END: [Proto<'static>; 2] = [Proto::Char(b";"), Proto::Char(b")")];

impl Fixer for IncrementStyleFixer {
    fn name(&self) -> &'static str {
        "increment_style"
    }
    fn priority(&self) -> i32 {
        15
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["style"])?;
        self.pre = c.string("style", "pre", &["pre", "post"])? == "pre";
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_INC, T_DEC])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_given_kind(&[T_INC, T_DEC]) {
                continue;
            }
            let token = tokens[index].clone();
            if self.pre && TokensAnalyzer::new(tokens).is_unary_successor_operator(index) {
                let next_token = &tokens[tokens.get_next_meaningful_token(index).unwrap()];
                if !next_token.equals_any(&STATEMENT_END, true) {
                    continue;
                }
                let start_index = find_start(tokens, index);
                let prev_token = &tokens[tokens.get_prev_meaningful_token(start_index).unwrap()];
                if prev_token.equals_any(&STATEMENT_START, true) {
                    tokens.clear_at(index);
                    tokens.insert_at(start_index, vec![token]);
                }
            } else if !self.pre && TokensAnalyzer::new(tokens).is_unary_predecessor_operator(index)
            {
                let prev_token = &tokens[tokens.get_prev_meaningful_token(index).unwrap()];
                if !prev_token.equals_any(&STATEMENT_START, true) {
                    continue;
                }
                let end_index = find_end(tokens, index);
                let next_token = &tokens[tokens.get_next_meaningful_token(end_index).unwrap()];
                if next_token.equals_any(&STATEMENT_END, true) {
                    tokens.clear_at(index);
                    let at = tokens.get_next_non_whitespace(end_index, None).unwrap();
                    tokens.insert_at(at, vec![token]);
                }
            }
        }
    }
}

fn find_end(tokens: &Tokens, mut index: usize) -> usize {
    let mut next_index = tokens.get_next_meaningful_token(index).unwrap();
    let mut next_token = &tokens[next_index];
    while next_token.equals_any(
        &[
            "$".into(),
            "(".into(),
            "[".into(),
            CT_DYNAMIC_PROP_BRACE_OPEN.into(),
            CT_DYNAMIC_VAR_BRACE_OPEN.into(),
            CT_ARRAY_INDEX_BRACE_OPEN.into(),
            T_NS_SEPARATOR.into(),
            T_STATIC.into(),
            T_STRING.into(),
            T_VARIABLE.into(),
        ],
        true,
    ) {
        if let Some((block_type, _)) = Tokens::detect_block_type(next_token) {
            next_index = tokens.find_block_end(block_type, next_index);
        }
        index = next_index;
        next_index = tokens.get_next_meaningful_token(next_index).unwrap();
        next_token = &tokens[next_index];
    }
    if next_token.is_object_operator() {
        return find_end(tokens, next_index);
    }
    if next_token.is_kind(T_DOUBLE_COLON) {
        return find_end(
            tokens,
            tokens.get_next_meaningful_token(next_index).unwrap(),
        );
    }
    index
}
