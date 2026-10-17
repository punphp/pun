//! Port of `PhpCsFixer\Fixer\Alias\NoMixedEchoPrintFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct NoMixedEchoPrintFixer {
    candidate_token_type: Kind,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoMixedEchoPrintFixer {
        candidate_token_type: T_PRINT,
    })
}

impl Fixer for NoMixedEchoPrintFixer {
    fn name(&self) -> &'static str {
        "no_mixed_echo_print"
    }
    fn priority(&self) -> i32 {
        -10
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["use"])?;
        self.candidate_token_type = if c.string("use", "echo", &["print", "echo"])? == "echo" {
            T_PRINT
        } else {
            T_ECHO
        };
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(self.candidate_token_type)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            if tokens[index].is_kind(self.candidate_token_type) {
                if self.candidate_token_type == T_PRINT {
                    fix_print_to_echo(tokens, index);
                } else {
                    fix_echo_to_print(tokens, index);
                }
            }
        }
    }
}

fn fix_echo_to_print(tokens: &mut Tokens, index: usize) {
    let next_token_index = tokens.get_next_meaningful_token(index).unwrap();
    let end_token_index = tokens
        .get_next_token_of_kind(index, &[";".into(), T_CLOSE_TAG.into()], true)
        .unwrap();
    let mut i = next_token_index;
    while i < end_token_index {
        if tokens[i].equals_any(&["(".into(), CT_ARRAY_BRACKET_OPEN.into()], true) {
            let (block_type, _) = Tokens::detect_block_type(&tokens[i]).unwrap();
            i = tokens.find_block_end(block_type, i);
        }
        if tokens[i].equals(",", true) {
            return;
        }
        i += 1;
    }
    tokens.set(index, Token::new(T_PRINT, "print"));
}

fn fix_print_to_echo(tokens: &mut Tokens, index: usize) {
    let prev = tokens.get_prev_meaningful_token(index).unwrap();
    if !tokens[prev].equals_any(
        &[
            ";".into(),
            "{".into(),
            "}".into(),
            ")".into(),
            T_OPEN_TAG.into(),
            T_ELSE.into(),
        ],
        true,
    ) {
        return;
    }
    tokens.set(index, Token::new(T_ECHO, "echo"));
}
