//! Port of `PhpCsFixer\Fixer\Operator\NewWithParenthesesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct NewWithParenthesesFixer {
    named_class: bool,
    anonymous_class: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NewWithParenthesesFixer {
        named_class: true,
        anonymous_class: true,
    })
}

const NEXT_TOKEN_KINDS: [Proto<'static>; 44] = [
    Proto::Char(b"?"),
    Proto::Char(b";"),
    Proto::Char(b","),
    Proto::Char(b"("),
    Proto::Char(b")"),
    Proto::Char(b"["),
    Proto::Char(b"]"),
    Proto::Char(b":"),
    Proto::Char(b"<"),
    Proto::Char(b">"),
    Proto::Char(b"+"),
    Proto::Char(b"-"),
    Proto::Char(b"*"),
    Proto::Char(b"/"),
    Proto::Char(b"%"),
    Proto::Char(b"&"),
    Proto::Char(b"^"),
    Proto::Char(b"|"),
    Proto::Kind(T_CLASS),
    Proto::Kind(T_IS_SMALLER_OR_EQUAL),
    Proto::Kind(T_IS_GREATER_OR_EQUAL),
    Proto::Kind(T_IS_EQUAL),
    Proto::Kind(T_IS_NOT_EQUAL),
    Proto::Kind(T_IS_IDENTICAL),
    Proto::Kind(T_IS_NOT_IDENTICAL),
    Proto::Kind(T_CLOSE_TAG),
    Proto::Kind(T_LOGICAL_AND),
    Proto::Kind(T_LOGICAL_OR),
    Proto::Kind(T_LOGICAL_XOR),
    Proto::Kind(T_BOOLEAN_AND),
    Proto::Kind(T_BOOLEAN_OR),
    Proto::Kind(T_SL),
    Proto::Kind(T_SR),
    Proto::Kind(T_INSTANCEOF),
    Proto::Kind(T_AS),
    Proto::Kind(T_DOUBLE_ARROW),
    Proto::Kind(T_POW),
    Proto::Kind(T_SPACESHIP),
    Proto::Kind(CT_ARRAY_BRACKET_OPEN),
    Proto::Kind(CT_ARRAY_BRACKET_CLOSE),
    Proto::Kind(CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN),
    Proto::Kind(CT_CLASS_INSTANTIATION_PARENTHESIS_CLOSE),
    Proto::Kind(T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG),
    Proto::Kind(T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG),
];

impl Fixer for NewWithParenthesesFixer {
    fn name(&self) -> &'static str {
        "new_with_parentheses"
    }
    fn priority(&self) -> i32 {
        38
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["named_class", "anonymous_class"])?;
        self.named_class = c.bool("named_class", true)?;
        self.anonymous_class = c.bool("anonymous_class", true)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_NEW)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (1..tokens.count().saturating_sub(2)).rev() {
            if !tokens[index].is_kind(T_NEW) {
                continue;
            }
            let mut next_index = tokens
                .get_next_token_of_kind(index, &NEXT_TOKEN_KINDS, true)
                .unwrap();
            if tokens[next_index].is_kind(T_CLASS) {
                next_index = tokens.get_next_meaningful_token(next_index).unwrap();
                ensure(tokens, next_index, self.anonymous_class);
                continue;
            }
            while tokens[next_index].equals("[", true)
                || tokens[next_index].is_kind(CT_ARRAY_INDEX_BRACE_OPEN)
            {
                let (block, _) = Tokens::detect_block_type(&tokens[next_index]).unwrap();
                next_index = tokens.find_block_end(block, next_index);
                next_index = tokens.get_next_meaningful_token(next_index).unwrap();
            }
            ensure(tokens, next_index, self.named_class);
        }
    }
}

fn ensure(tokens: &mut Tokens, index: usize, parentheses: bool) {
    if parentheses {
        if !tokens[index].equals("(", true) && !tokens[index].is_object_operator() {
            let at = tokens.get_prev_meaningful_token(index).unwrap() + 1;
            tokens.insert_at(at, vec![Token::char("("), Token::char(")")]);
        }
        return;
    }
    if !tokens[index].equals("(", true) {
        return;
    }
    let closing_index = tokens.get_next_meaningful_token(index).unwrap();
    // constructor has arguments - parentheses can not be removed
    if !tokens[closing_index].equals(")", true) {
        return;
    }
    // `new A()->method()` (PHP 8.4) keeps its parentheses
    if tokens[tokens.get_next_meaningful_token(closing_index).unwrap()].is_object_operator() {
        return;
    }
    tokens.clear_token_and_merge_surrounding_whitespace(closing_index);
    tokens.clear_token_and_merge_surrounding_whitespace(index);
}
