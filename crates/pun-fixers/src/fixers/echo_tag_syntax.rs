//! Port of `PhpCsFixer\Fixer\PhpTag\EchoTagSyntaxFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct EchoTagSyntaxFixer {
    /// `format`: `short` (true) or `long`.
    short: bool,
    /// `long_function`: `print` (true) or `echo`.
    print: bool,
    shorten_simple_statements_only: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(EchoTagSyntaxFixer::default())
}

impl Fixer for EchoTagSyntaxFixer {
    fn name(&self) -> &'static str {
        "echo_tag_syntax"
    }
    fn priority(&self) -> i32 {
        // Must run before NoMixedEchoPrintFixer.
        // Must run after NoUselessPrintfFixer.
        0
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let r = resolve(
            self.name(),
            options,
            &["format", "long_function", "shorten_simple_statements_only"],
        )?;
        self.short = r.string("format", "long", &["long", "short"])? == "short";
        self.print = r.string("long_function", "echo", &["echo", "print"])? == "print";
        self.shorten_simple_statements_only = r.bool("shorten_simple_statements_only", true)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        if self.short {
            return tokens.is_any_token_kinds_found(&[T_ECHO, T_PRINT]);
        }
        tokens.is_token_kind_found(T_OPEN_TAG_WITH_ECHO)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        if self.short {
            self.long_to_short(tokens);
        } else {
            self.short_to_long(tokens);
        }
    }
}

impl EchoTagSyntaxFixer {
    fn long_to_short(&self, tokens: &mut Tokens) {
        let count = tokens.count();
        let mut index = 0;
        while index < count {
            if !tokens[index].is_kind(T_OPEN_TAG) {
                index += 1;
                continue;
            }
            let Some(echo_print_index) = tokens.get_next_meaningful_token(index) else {
                return;
            };
            if !tokens[echo_print_index].is_given_kind(&[T_ECHO, T_PRINT])
                || (self.shorten_simple_statements_only
                    && is_complex_code(tokens, echo_print_index + 1))
            {
                index = echo_print_index + 1;
                continue;
            }
            tokens.set(index, Token::new(T_OPEN_TAG_WITH_ECHO, "<?="));
            tokens.ensure_whitespace_at_index(index + 1, 0, b"");
            fix_whitespace_before_echo_print(tokens, echo_print_index);
            tokens.clear_token_and_merge_surrounding_whitespace(echo_print_index);
            index += 1;
        }
    }

    fn short_to_long(&self, tokens: &mut Tokens) {
        let echo_token = if self.print {
            Token::new(T_PRINT, "print")
        } else {
            Token::new(T_ECHO, "echo")
        };
        // `$index = -1; getNextTokenOfKind($index, ...)`: the first search starts at index 0.
        let mut index = None;
        loop {
            let found = match index {
                None if tokens[0].is_kind(T_OPEN_TAG_WITH_ECHO) => Some(0),
                None => {
                    tokens.get_next_token_of_kind(0, &[Proto::Kind(T_OPEN_TAG_WITH_ECHO)], true)
                }
                Some(i) => {
                    tokens.get_next_token_of_kind(i, &[Proto::Kind(T_OPEN_TAG_WITH_ECHO)], true)
                }
            };
            let Some(i) = found else {
                return;
            };
            let mut replace = vec![Token::new(T_OPEN_TAG, "<?php "), echo_token.clone()];
            if !tokens[i + 1].is_whitespace(None) {
                replace.push(Token::new(T_WHITESPACE, " "));
            }
            tokens.override_range(i, i, replace);
            index = Some(i + 1);
        }
    }
}

/// Check if `tokens`, starting at `index`, contains "complex code", that is, the content of the echo tag contains
/// more than a simple "echo something": non-whitespace tokens after a semicolon.
fn is_complex_code(tokens: &Tokens, mut index: usize) -> bool {
    let mut semicolon_found = false;
    let count = tokens.count();
    while index < count {
        let token = &tokens[index];
        if token.is_kind(T_CLOSE_TAG) {
            return false;
        }
        if token.get_content() == b";" {
            semicolon_found = true;
        } else if semicolon_found && !token.is_whitespace(None) {
            return true;
        }
        index += 1;
    }
    false
}

fn fix_whitespace_before_echo_print(tokens: &mut Tokens, echo_print_index: usize) {
    let whitespace_before_echo_print_index =
        tokens.get_non_empty_sibling(echo_print_index, -1).unwrap();
    if !tokens[whitespace_before_echo_print_index].is_whitespace(None) {
        return;
    }
    let prev = tokens
        .get_non_empty_sibling(whitespace_before_echo_print_index, -1)
        .unwrap();
    if tokens[prev].is_kind(T_COMMENT) && !Preg::r#match(r"/\*\/$/", tokens[prev].get_content()) {
        return;
    }
    tokens.clear_at(whitespace_before_echo_print_index);
}
