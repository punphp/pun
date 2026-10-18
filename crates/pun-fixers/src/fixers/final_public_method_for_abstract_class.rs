//! Port of `PhpCsFixer\Fixer\ClassNotation\FinalPublicMethodForAbstractClassFixer`.
use pun_format::Fixer;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

const MAGIC_METHODS: &[&[u8]] = &[
    b"__construct",
    b"__destruct",
    b"__call",
    b"__callstatic",
    b"__get",
    b"__set",
    b"__isset",
    b"__unset",
    b"__sleep",
    b"__wakeup",
    b"__tostring",
    b"__invoke",
    b"__set_state",
    b"__clone",
    b"__debuginfo",
];

struct FinalPublicMethodForAbstractClassFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(FinalPublicMethodForAbstractClassFixer)
}

impl Fixer for FinalPublicMethodForAbstractClassFixer {
    fn name(&self) -> &'static str {
        "final_public_method_for_abstract_class"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[Kind::T_ABSTRACT, Kind::T_PUBLIC, Kind::T_FUNCTION])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let abstracts = tokens.find_given_kind(Kind::T_ABSTRACT, 0, None);
        for &abstract_index in abstracts.iter().rev() {
            let class_index = tokens
                .get_next_token_of_kind(
                    abstract_index,
                    &[Kind::T_CLASS.into(), Kind::T_FUNCTION.into()],
                    true,
                )
                .unwrap();
            if !tokens[class_index].is_kind(Kind::T_CLASS) {
                continue;
            }
            let class_open = tokens
                .get_next_token_of_kind(class_index, &["{".into()], true)
                .unwrap();
            let class_close = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, class_open);
            fix_class(tokens, class_open, class_close);
        }
    }
}

fn fix_class(tokens: &mut Tokens, class_open_index: usize, class_close_index: usize) {
    let mut index = class_close_index - 1;
    while index > class_open_index {
        if tokens[index].equals("}", true) {
            index = tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, index) - 1;
            continue;
        }
        if !tokens[index].is_kind(Kind::T_PUBLIC) {
            index -= 1;
            continue;
        }
        let mut next_index = tokens.get_next_meaningful_token(index).unwrap();
        if tokens[next_index].is_kind(Kind::T_STATIC) {
            next_index = tokens.get_next_meaningful_token(next_index).unwrap();
        }
        if !tokens[next_index].is_kind(Kind::T_FUNCTION) {
            index -= 1;
            continue;
        }
        next_index = tokens.get_next_meaningful_token(next_index).unwrap();
        if MAGIC_METHODS.contains(&&strtolower(tokens[next_index].get_content())[..]) {
            index -= 1;
            continue;
        }
        let mut prev_index = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[prev_index].is_kind(Kind::T_STATIC) {
            index = prev_index;
            prev_index = tokens.get_prev_meaningful_token(index).unwrap();
        }
        if tokens[prev_index].is_given_kind(&[Kind::T_ABSTRACT, Kind::T_FINAL]) {
            index = prev_index - 1;
            continue;
        }
        tokens.insert_at(
            index,
            vec![
                Token::new(Kind::T_FINAL, b"final".to_vec()),
                Token::new(Kind::T_WHITESPACE, b" ".to_vec()),
            ],
        );
        index -= 1;
    }
}
