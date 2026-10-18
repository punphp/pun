//! Port of `PhpCsFixer\Fixer\ClassNotation\NoUnneededFinalMethodFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::tokens_analyzer::ClassyElementType;
use pun_tokens::{Tokens, TokensAnalyzer};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

struct NoUnneededFinalMethodFixer {
    private_methods: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(NoUnneededFinalMethodFixer {
        private_methods: true,
    })
}

/// One method of `getMethods`.
struct Method {
    method_final_index: Option<usize>,
    method_is_private: bool,
    method_of_enum: bool,
    class_is_final: bool,
    method_is_constructor: bool,
}

impl Fixer for NoUnneededFinalMethodFixer {
    fn name(&self) -> &'static str {
        "no_unneeded_final_method"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_FINAL, T_FUNCTION])
            && tokens.is_any_token_kinds_found(&[T_CLASS, T_ENUM])
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["private_methods"])?;
        self.private_methods = o.bool("private_methods", true)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for element in get_methods(tokens) {
            let index = element.method_final_index;
            if element.method_of_enum || element.class_is_final {
                clear_final(tokens, index);
                continue;
            }
            if !element.method_is_private || !self.private_methods || element.method_is_constructor
            {
                continue;
            }
            clear_final(tokens, index);
        }
    }
}

fn get_methods(tokens: &Tokens) -> Vec<Method> {
    let tokens_analyzer = TokensAnalyzer::new(tokens);
    let modifier_kinds = [
        T_PUBLIC,
        T_PROTECTED,
        T_PRIVATE,
        T_FINAL,
        T_ABSTRACT,
        T_STATIC,
    ];
    let mut enums: HashMap<usize, bool> = HashMap::new();
    let mut classes_are_final: HashMap<usize, bool> = HashMap::new();
    let mut out = vec![];
    for (index, element) in tokens_analyzer.get_classy_elements() {
        if element.kind != ClassyElementType::Method {
            continue; // not a method
        }
        let class_index = element.class_index;
        let is_enum = *enums
            .entry(class_index)
            .or_insert_with(|| tokens[class_index].is_kind(T_ENUM));
        let mut method_final_index = None;
        let mut method_is_private = false;
        let mut previous = index;
        loop {
            previous = tokens.get_prev_meaningful_token(previous).unwrap();
            if tokens[previous].is_kind(T_PRIVATE) {
                method_is_private = true;
            } else if tokens[previous].is_kind(T_FINAL) {
                method_final_index = Some(previous);
            }
            if !tokens[previous].is_given_kind(&modifier_kinds) {
                break;
            }
        }
        if is_enum {
            out.push(Method {
                method_final_index,
                method_is_private,
                method_of_enum: true,
                class_is_final: false,
                method_is_constructor: false,
            });
            continue;
        }
        let class_is_final = *classes_are_final.entry(class_index).or_insert_with(|| {
            tokens_analyzer
                .get_classy_modifiers(class_index)
                .final_
                .is_some()
        });
        let next = tokens.get_next_meaningful_token(index).unwrap();
        out.push(Method {
            method_final_index,
            method_is_private,
            method_of_enum: false,
            class_is_final,
            method_is_constructor: strtolower(tokens[next].get_content()) == b"__construct",
        });
    }
    out
}

fn clear_final(tokens: &mut Tokens, index: Option<usize>) {
    let Some(mut index) = index else {
        return;
    };
    tokens.clear_at(index);
    index += 1;
    if tokens[index].is_whitespace(None) {
        tokens.clear_at(index);
    }
}
