//! Port of `PhpCsFixer\Fixer\Operator\NoUselessConcatOperatorFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct NoUselessConcatOperatorFixer {
    juggle_simple_strings: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(NoUselessConcatOperatorFixer::default())
}

#[derive(Clone, Copy, PartialEq)]
enum StrType {
    DoubleQuote,
    DoubleQuoteVar,
    SingleQuote,
}

/// `_ConcatOperandType`.
#[derive(Clone, Copy)]
struct Operand {
    start: usize,
    end: usize,
    kind: StrType,
}

impl Fixer for NoUselessConcatOperatorFixer {
    fn name(&self) -> &'static str {
        "no_useless_concat_operator"
    }
    fn priority(&self) -> i32 {
        // Must run before DateTimeCreateFromFormatCallFixer, EregToPregFixer, PhpUnitDedicateAssertInternalTypeFixer,
        // RegularCallableCallFixer, SetTypeToCastFixer.
        // Must run after ExplicitStringVariableFixer, NoBinaryStringFixer, SingleQuoteFixer.
        5
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let r = resolve(self.name(), options, &["juggle_simple_strings"])?;
        self.juggle_simple_strings = r.bool("juggle_simple_strings", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(".")
            && (tokens.is_token_kind_found(T_CONSTANT_ENCAPSED_STRING)
                || tokens.is_token_kind_found("\""))
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (1..tokens.count()).rev() {
            if !tokens[index].equals(".", true) {
                continue;
            }
            let next_meaningful_token_index = tokens.get_next_meaningful_token(index).unwrap();
            if contains_linebreak(tokens, index, next_meaningful_token_index) {
                continue;
            }
            let Some(second_operand) =
                get_concat_operand_type(tokens, next_meaningful_token_index, 1)
            else {
                continue;
            };
            let prev_meaningful_token_index = tokens.get_prev_meaningful_token(index).unwrap();
            if contains_linebreak(tokens, prev_meaningful_token_index, index) {
                continue;
            }
            let Some(first_operand) =
                get_concat_operand_type(tokens, prev_meaningful_token_index, -1)
            else {
                continue;
            };
            self.fix_concat_operation(tokens, first_operand, index, second_operand);
        }
    }
}

impl NoUselessConcatOperatorFixer {
    fn fix_concat_operation(
        &self,
        tokens: &mut Tokens,
        first_operand: Operand,
        concat_index: usize,
        second_operand: Operand,
    ) {
        use StrType::*;
        // if both operands are of the same type then these operands can always be merged
        if (first_operand.kind == DoubleQuote && second_operand.kind == DoubleQuote)
            || (first_operand.kind == SingleQuote && second_operand.kind == SingleQuote)
        {
            merge_constant_escaped_string_operands(
                tokens,
                first_operand,
                concat_index,
                second_operand,
            );
            return;
        }
        if first_operand.kind == DoubleQuoteVar && second_operand.kind == DoubleQuoteVar {
            if operands_can_not_be_merged(tokens, first_operand, second_operand) {
                return;
            }
            merge_constant_escaped_string_var_operands(
                tokens,
                first_operand,
                concat_index,
                second_operand,
            );
            return;
        }
        // if any is double and the other is not, check for simple other, than merge with "
        for (operand1, operand2) in [
            (first_operand, second_operand),
            (second_operand, first_operand),
        ] {
            if operand1.kind == DoubleQuoteVar && operand2.kind == DoubleQuote {
                if operands_can_not_be_merged(tokens, operand1, operand2) {
                    return;
                }
                merge_constant_escaped_string_var_operands(
                    tokens,
                    first_operand,
                    concat_index,
                    second_operand,
                );
                return;
            }
            if !self.juggle_simple_strings {
                continue;
            }
            if operand1.kind == DoubleQuote && operand2.kind == SingleQuote {
                if is_simple_quoted_string_content(tokens[operand2.start].get_content()) {
                    merge_constant_escaped_string_operands(
                        tokens,
                        first_operand,
                        concat_index,
                        second_operand,
                    );
                }
                return;
            }
            if operand1.kind == DoubleQuoteVar && operand2.kind == SingleQuote {
                if is_simple_quoted_string_content(tokens[operand2.start].get_content()) {
                    if operands_can_not_be_merged(tokens, operand1, operand2) {
                        return;
                    }
                    merge_constant_escaped_string_var_operands(
                        tokens,
                        first_operand,
                        concat_index,
                        second_operand,
                    );
                }
                return;
            }
        }
    }
}

/// `substr($s, 1, -1)`.
fn inner(s: &[u8]) -> &[u8] {
    &s[1..s.len() - 1]
}

fn get_concat_operand_type(tokens: &Tokens, index: usize, direction: isize) -> Option<Operand> {
    if tokens[index].is_kind(T_CONSTANT_ENCAPSED_STRING) {
        let first_char = tokens[index].get_content()[0];
        if first_char == b'b' || first_char == b'B' {
            // we don't care about these, priorities are set to do deal with these cases
            return None;
        }
        return Some(Operand {
            start: index,
            end: index,
            kind: if first_char == b'"' {
                StrType::DoubleQuote
            } else {
                StrType::SingleQuote
            },
        });
    }
    if tokens[index].equals("\"", true) {
        let end = tokens
            .get_token_of_kind_sibling(index, direction, &[Proto::Char(b"\"")], true)
            .unwrap();
        let (start, end) = if direction == 1 {
            (index, end)
        } else {
            (end, index)
        };
        return Some(Operand {
            start,
            end,
            kind: StrType::DoubleQuoteVar,
        });
    }
    None
}

fn merge_constant_escaped_string_operands(
    tokens: &mut Tokens,
    first_operand: Operand,
    concat_operator_index: usize,
    second_operand: Operand,
) {
    let quote: &[u8] = if first_operand.kind == StrType::DoubleQuote
        || second_operand.kind == StrType::DoubleQuote
    {
        b"\""
    } else {
        b"'"
    };
    let content = [
        quote,
        inner(tokens[first_operand.start].get_content()),
        inner(tokens[second_operand.start].get_content()),
        quote,
    ]
    .concat();
    tokens.set(
        first_operand.start,
        Token::new(T_CONSTANT_ENCAPSED_STRING, content),
    );
    clear_concat_and_around(tokens, concat_operator_index);
    tokens.clear_token_and_merge_surrounding_whitespace(second_operand.start);
}

fn merge_constant_escaped_string_var_operands(
    tokens: &mut Tokens,
    first_operand: Operand,
    concat_operator_index: usize,
    second_operand: Operand,
) {
    // build up the new content
    let mut new_content = Vec::new();
    for operand in [first_operand, second_operand] {
        let mut operand_content = Vec::new();
        let mut i = operand.start;
        while i <= operand.end {
            operand_content.extend_from_slice(tokens[i].get_content());
            i = tokens.get_next_meaningful_token(i).unwrap();
        }
        new_content.extend_from_slice(inner(&operand_content));
    }
    // remove tokens making up the concat statement
    let mut i = Some(second_operand.end);
    while let Some(j) = i.filter(|&j| j >= second_operand.start) {
        tokens.clear_token_and_merge_surrounding_whitespace(j);
        i = tokens.get_prev_meaningful_token(j);
    }
    clear_concat_and_around(tokens, concat_operator_index);
    let mut i = Some(first_operand.end);
    while let Some(j) = i.filter(|&j| j > first_operand.start) {
        tokens.clear_token_and_merge_surrounding_whitespace(j);
        i = tokens.get_prev_meaningful_token(j);
    }
    // insert new tokens based on the new content
    let new_tokens = Tokens::from_code(&[&b"<?php \""[..], &new_content, b"\";"].concat());
    let insert_tokens = (1..new_tokens.count() - 1)
        .map(|i| new_tokens[i].clone())
        .collect();
    tokens.override_range(first_operand.start, first_operand.start, insert_tokens);
}

fn clear_concat_and_around(tokens: &mut Tokens, concat_operator_index: usize) {
    if tokens[concat_operator_index + 1].is_whitespace(None) {
        tokens.clear_token_and_merge_surrounding_whitespace(concat_operator_index + 1);
    }
    tokens.clear_token_and_merge_surrounding_whitespace(concat_operator_index);
    if tokens[concat_operator_index - 1].is_whitespace(None) {
        tokens.clear_token_and_merge_surrounding_whitespace(concat_operator_index - 1);
    }
}

fn is_simple_quoted_string_content(candidate: &[u8]) -> bool {
    !Preg::r#match(r#"#[\$"'\\]#"#, inner(candidate))
}

fn contains_linebreak(tokens: &Tokens, start_index: usize, end_index: usize) -> bool {
    ((start_index + 1)..=end_index).any(|i| Preg::r#match(r"/\R/", tokens[i].get_content()))
}

fn operands_can_not_be_merged(
    tokens: &Tokens,
    first_operand: Operand,
    second_operand: Operand,
) -> bool {
    // If the first operand does not end with a variable, no variables would be broken by concatenation. (Upstream
    // first returns false when it is not a `"..."` with variables, but every caller passes one.)
    if !tokens[first_operand.end - 1].is_kind(T_VARIABLE) {
        return false;
    }
    // e.g. " foo", ' bar', " $baz"; "-foo", '-bar', "-$baz"
    let allowed_patterns_for_second_operand = [r"/^ .*/", r"/^-(?!\>)/"];
    // If the first operand ends with a variable, the second operand should match one of the allowed patterns.
    // Otherwise, the concatenation can break a variable in the first operand.
    let code = tokens.generate_partial_code(second_operand.start, second_operand.end);
    let second_operand_inner_content = inner(&code);
    !allowed_patterns_for_second_operand
        .iter()
        .any(|p| Preg::r#match(p, second_operand_inner_content))
}
