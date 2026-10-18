//! Port of `PhpCsFixer\Fixer\ClassNotation\SelfStaticAccessorFixer`.
use pun_format::Fixer;
use pun_tokens::{Kind, Proto, Token, Tokens, TokensAnalyzer};
use std::path::Path;

const CLASSY_TYPES: &[Kind] = &[Kind::T_CLASS, Kind::T_ENUM];
const CLASSY_TOKENS_OF_INTEREST: &[Proto<'static>] =
    &[Proto::Kind(Kind::T_CLASS), Proto::Kind(Kind::T_ENUM)];

struct SelfStaticAccessorFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(SelfStaticAccessorFixer)
}

impl Fixer for SelfStaticAccessorFixer {
    fn name(&self) -> &'static str {
        "self_static_accessor"
    }
    fn priority(&self) -> i32 {
        -10
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_STATIC)
            && tokens.is_any_token_kinds_found(CLASSY_TYPES)
            && tokens.is_any_token_kinds_found(&[
                Kind::T_DOUBLE_COLON,
                Kind::T_NEW,
                Kind::T_INSTANCEOF,
            ])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut classy_index = tokens.get_next_token_of_kind(0, CLASSY_TOKENS_OF_INTEREST, true);
        while let Some(mut ci) = classy_index {
            if tokens[ci].is_kind(Kind::T_CLASS) {
                let a = TokensAnalyzer::new(tokens);
                if a.get_classy_modifiers(ci).final_.is_some() || a.is_anonymous_class(ci) {
                    ci = fix_classy(tokens, ci);
                }
            } else {
                ci = fix_classy(tokens, ci);
            }
            classy_index = tokens.get_next_token_of_kind(ci, CLASSY_TOKENS_OF_INTEREST, true);
        }
    }
}

fn fix_classy(tokens: &mut Tokens, index: usize) -> usize {
    let mut index = tokens
        .get_next_token_of_kind(index, &["{".into()], true)
        .unwrap();
    let mut class_open_count = 1;
    while class_open_count > 0 {
        index += 1;
        if tokens[index].equals("{", true) {
            class_open_count += 1;
            continue;
        }
        if tokens[index].equals("}", true) {
            class_open_count -= 1;
            continue;
        }
        if tokens[index].is_kind(Kind::T_FUNCTION) {
            if TokensAnalyzer::new(tokens).is_lambda(index) {
                index = tokens
                    .get_next_token_of_kind(index, &["{".into()], true)
                    .unwrap();
                let mut open_count = 1;
                loop {
                    index = tokens
                        .get_next_token_of_kind(
                            index,
                            &["}".into(), "{".into(), Kind::T_CLASS.into()],
                            true,
                        )
                        .unwrap();
                    if tokens[index].equals("}", true) {
                        open_count -= 1;
                    } else if tokens[index].equals("{", true) {
                        open_count += 1;
                    } else {
                        index = fix_classy(tokens, index);
                    }
                    if open_count <= 0 {
                        break;
                    }
                }
            }
            continue;
        }
        if tokens[index].is_given_kind(&[Kind::T_NEW, Kind::T_INSTANCEOF]) {
            index = tokens.get_next_meaningful_token(index).unwrap();
            if tokens[index].is_kind(Kind::T_STATIC) {
                tokens.set(index, Token::new(Kind::T_STRING, b"self".to_vec()));
            }
            continue;
        }
        if !tokens[index].is_kind(Kind::T_STATIC) {
            continue;
        }
        let static_index = index;
        index = tokens.get_next_meaningful_token(index).unwrap();
        if !tokens[index].is_kind(Kind::T_DOUBLE_COLON) {
            continue;
        }
        tokens.set(static_index, Token::new(Kind::T_STRING, b"self".to_vec()));
    }
    index
}
