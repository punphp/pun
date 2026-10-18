//! Port of `PhpCsFixer\Fixer\LanguageConstruct\ExplicitIndirectVariableFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct ExplicitIndirectVariableFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(ExplicitIndirectVariableFixer)
}

impl Fixer for ExplicitIndirectVariableFixer {
    fn name(&self) -> &'static str {
        "explicit_indirect_variable"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_VARIABLE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() - 1;
        while index > 1 {
            if tokens[index].is_kind(T_VARIABLE) {
                fix_variable(tokens, index);
            }
            index -= 1;
        }
    }
}

fn fix_variable(tokens: &mut Tokens, index: usize) {
    let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
    let prev_token = &tokens[prev_index];
    if !prev_token.equals("$", true) && !prev_token.is_object_operator() {
        return;
    }
    let (opening_brace, closing_brace) = if prev_token.is_object_operator() {
        (CT_DYNAMIC_PROP_BRACE_OPEN, CT_DYNAMIC_PROP_BRACE_CLOSE)
    } else {
        (CT_DYNAMIC_VAR_BRACE_OPEN, CT_DYNAMIC_VAR_BRACE_CLOSE)
    };
    let content = tokens[index].content.clone();
    tokens.override_range(
        index,
        index,
        vec![
            Token::new(opening_brace, "{"),
            Token::new(T_VARIABLE, content),
            Token::new(closing_brace, "}"),
        ],
    );
}
