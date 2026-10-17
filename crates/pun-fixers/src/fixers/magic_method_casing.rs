//! Port of `PhpCsFixer\Fixer\Casing\MagicMethodCasingFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct MagicMethodCasingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(MagicMethodCasingFixer)
}

const MAGIC_NAMES: &[(&str, &str)] = &[
    ("__call", "__call"),
    ("__callstatic", "__callStatic"),
    ("__clone", "__clone"),
    ("__construct", "__construct"),
    ("__debuginfo", "__debugInfo"),
    ("__destruct", "__destruct"),
    ("__get", "__get"),
    ("__invoke", "__invoke"),
    ("__isset", "__isset"),
    ("__serialize", "__serialize"),
    ("__set", "__set"),
    ("__set_state", "__set_state"),
    ("__sleep", "__sleep"),
    ("__tostring", "__toString"),
    ("__unserialize", "__unserialize"),
    ("__unset", "__unset"),
    ("__wakeup", "__wakeup"),
];

impl Fixer for MagicMethodCasingFixer {
    fn name(&self) -> &'static str {
        "magic_method_casing"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
            && tokens.is_any_token_kinds_found(&[
                T_FUNCTION,
                T_DOUBLE_COLON,
                T_OBJECT_OPERATOR,
                T_NULLSAFE_OBJECT_OPERATOR,
            ])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut in_class = 0;
        let token_count = tokens.count() as isize;
        let mut index = 1isize;
        while index < token_count - 2 {
            let i = index as usize;
            if in_class == 0 && tokens[i].is_classy() {
                in_class = 1;
                // Upstream `++$index` on a null index restarts at 1; every classy in valid code has a `{`.
                index = tokens
                    .get_next_token_of_kind(i, &["{".into()], true)
                    .map_or(1, |n| n as isize + 1);
                continue;
            }
            index += 1;
            if in_class != 0 {
                if tokens[i].equals("{", true) {
                    in_class += 1;
                    continue;
                }
                if tokens[i].equals("}", true) {
                    in_class -= 1;
                    continue;
                }
            }
            if !tokens[i].is_kind(T_STRING) {
                continue;
            }
            let content = &tokens[i].content;
            if !content.starts_with(b"__") {
                continue;
            }
            let name = content.to_ascii_lowercase();
            let Some(&(_, name_in_correct_casing)) =
                MAGIC_NAMES.iter().find(|(n, _)| n.as_bytes() == &name[..])
            else {
                continue; // method name is not one of the magic ones we can fix
            };
            if name_in_correct_casing.as_bytes() == &content[..] {
                continue;
            }
            if is_function_signature(tokens, i) {
                if in_class != 0 {
                    tokens.set(i, Token::new(T_STRING, name_in_correct_casing));
                }
                continue;
            }
            if is_method_call(tokens, i) {
                tokens.set(i, Token::new(T_STRING, name_in_correct_casing));
                continue;
            }
            if (name == b"__callstatic" || name == b"__set_state")
                && is_static_method_call(tokens, i)
            {
                tokens.set(i, Token::new(T_STRING, name_in_correct_casing));
            }
        }
    }
}

fn next_is_open_paren(tokens: &Tokens, index: usize) -> bool {
    tokens[tokens.get_next_meaningful_token(index).unwrap()].equals("(", true)
}

fn is_function_signature(tokens: &Tokens, index: usize) -> bool {
    let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
    if !tokens[prev_index].is_kind(T_FUNCTION) {
        return false;
    }
    next_is_open_paren(tokens, index)
}

fn is_method_call(tokens: &Tokens, index: usize) -> bool {
    let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
    if !tokens[prev_index].is_object_operator() {
        return false; // not a "simple" method call
    }
    next_is_open_paren(tokens, index)
}

fn is_static_method_call(tokens: &Tokens, index: usize) -> bool {
    let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
    if !tokens[prev_index].is_kind(T_DOUBLE_COLON) {
        return false; // not a "simple" static method call
    }
    next_is_open_paren(tokens, index)
}
