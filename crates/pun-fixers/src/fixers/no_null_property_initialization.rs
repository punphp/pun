//! Port of `PhpCsFixer\Fixer\ClassNotation\NoNullPropertyInitializationFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use std::collections::HashMap;
use std::path::Path;

struct NoNullPropertyInitializationFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoNullPropertyInitializationFixer)
}

impl Fixer for NoNullPropertyInitializationFixer {
    fn name(&self) -> &'static str {
        "no_null_property_initialization"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_CLASS, T_TRAIT])
            && tokens.is_any_token_kinds_found(&[T_PUBLIC, T_PROTECTED, T_PRIVATE, T_VAR, T_STATIC])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let next = |tokens: &Tokens, i: usize| tokens.get_next_meaningful_token(i).unwrap();
        let mut in_class: HashMap<usize, i64> = HashMap::new();
        let mut class_level = 0;
        let count = tokens.count();
        let mut index = 0;
        while index < count {
            if tokens[index].is_given_kind(&[T_CLASS, T_TRAIT]) {
                // Enums and interfaces do not have properties
                class_level += 1;
                in_class.insert(class_level, 1);
                index = tokens
                    .get_next_token_of_kind(index, &["{".into()], true)
                    .unwrap()
                    + 1;
                continue;
            }
            if class_level == 0 {
                index += 1;
                continue;
            }
            if tokens[index].equals("{", true) {
                *in_class.get_mut(&class_level).unwrap() += 1;
                index += 1;
                continue;
            }
            if tokens[index].equals("}", true) {
                let level = in_class.get_mut(&class_level).unwrap();
                *level -= 1;
                if *level == 0 {
                    in_class.remove(&class_level);
                    class_level -= 1;
                }
                index += 1;
                continue;
            }
            // Ensure we are in a class but not in a method in case there are static variables defined
            if in_class[&class_level] != 1
                || !tokens[index].is_given_kind(&[
                    T_PUBLIC,
                    T_PROTECTED,
                    T_PRIVATE,
                    T_VAR,
                    T_STATIC,
                ])
            {
                index += 1;
                continue;
            }
            loop {
                index = next(tokens, index);
                let mut var_token_index = index;
                if tokens[index].is_kind(T_STATIC) {
                    index = next(tokens, index);
                    var_token_index = index;
                }
                if !tokens[index].is_kind(T_VARIABLE) {
                    break;
                }
                index = next(tokens, index);
                if tokens[index].equals("=", true) {
                    index = next(tokens, index);
                    if tokens[index].is_kind(T_NS_SEPARATOR) {
                        index = next(tokens, index);
                    }
                    if tokens[index].equals((T_STRING, "null"), false) {
                        for i in var_token_index + 1..=index {
                            let t = &tokens[i];
                            if !(t.is_whitespace(None) && t.get_content().contains(&b'\n'))
                                && !t.is_comment()
                            {
                                tokens.clear_at(i);
                            }
                        }
                    }
                    index += 1;
                }
                if !tokens[index].equals(",", true) {
                    break;
                }
            }
            index += 1;
        }
    }
}
