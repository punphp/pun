//! `lint/functions/no-missing-format-argument`, ported from Mago 1.57.0's linter rule `no-missing-format-argument`
//! (rule/correctness/no_missing_format_argument.rs).
//!
//! A `sprintf`-family call whose literal format string needs more values than the call passes, or a
//! `vsprintf`-family call without its values array, is reported on its argument list.

use mago_syntax::cst::{Argument, Expression, Literal, Node, NodeKind};

use super::lint_b_names::function_call_matches_any;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoMissingFormatArgument;

enum ValuesModel {
    Variadic,
    Array,
}

/// `FORMAT_FUNCTION_MODELS`: the function, the index of its format argument and how it takes the values.
const MODELS: &[(&str, usize, ValuesModel)] = &[
    ("sprintf", 0, ValuesModel::Variadic),
    ("printf", 0, ValuesModel::Variadic),
    ("fprintf", 1, ValuesModel::Variadic),
    ("vsprintf", 0, ValuesModel::Array),
    ("vprintf", 0, ValuesModel::Array),
    ("vfprintf", 1, ValuesModel::Array),
    ("Psl\\Str\\format", 0, ValuesModel::Variadic),
    ("Psl\\IO\\write", 0, ValuesModel::Variadic),
    ("Psl\\IO\\write_line", 0, ValuesModel::Variadic),
    ("Psl\\IO\\write_error", 0, ValuesModel::Variadic),
    ("Psl\\IO\\write_error_line", 0, ValuesModel::Variadic),
];

impl Rule for NoMissingFormatArgument {
    fn id(&self) -> &'static str {
        "lint/functions/no-missing-format-argument"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        let names: Vec<&str> = MODELS.iter().map(|m| m.0).collect();
        let Some(name) = function_call_matches_any(ancestors, c, &names) else {
            return;
        };
        let (_, format_index, model) = MODELS.iter().find(|m| m.0 == name).unwrap();
        let arguments = &c.argument_list.arguments.nodes;
        let total = arguments.len();
        let Some(Argument::Positional(format)) = arguments.get(*format_index) else {
            return;
        };
        if format.ellipsis.is_some() {
            return;
        }
        let Expression::Literal(Literal::String(literal)) = format.value else {
            return;
        };
        let Some(format) = literal.value else { return };
        let (placeholders, required) = parse_format_string(format);
        let message = match model {
            ValuesModel::Variadic => {
                let given = total.saturating_sub(format_index + 1);
                if given >= required {
                    return;
                }
                (
                    format!(
                        "Call to `{name}` contains {placeholders} {}, {required} value{} given.",
                        if placeholders == 1 {
                            "placeholder"
                        } else {
                            "placeholders"
                        },
                        if required == 1 { "" } else { "s" },
                    ),
                    format!(
                        "Add the missing value{} to the c, or remove the corresponding %s placeholders from the format string.",
                        if required - given == 1 { "" } else { "s" },
                    ),
                )
            }
            ValuesModel::Array => {
                if total > format_index + 1 {
                    return;
                }
                (
                    format!(
                        "Call to `{name}` expects a format string and a values array, but only {total} argument{} given.",
                        if total == 1 { "" } else { "s" },
                    ),
                    "Provide an array of values as the second argument.".to_string(),
                )
            }
        };
        out.push(Report {
            span: node_span(&c.argument_list),
            message: message.0,
            help: Some(message.1),
            fix: None,
        });
    }
}

/// `parse_format_string`: the number of placeholders, and how many values they need (`%%` is not one; `%n$` takes
/// value n).
fn parse_format_string(bytes: &[u8]) -> (usize, usize) {
    let len = bytes.len();
    let mut i = 0;
    let mut placeholders = 0;
    let mut max_position = 0;
    let mut has_positional = false;
    let mut sequential = 0;
    while i < len {
        if bytes[i] != b'%' {
            i += 1;
            continue;
        }
        i += 1;
        if i >= len {
            break;
        }
        if bytes[i] == b'%' {
            i += 1;
            continue;
        }
        placeholders += 1;
        match parse_position(bytes, &mut i) {
            Some(p) => {
                has_positional = true;
                max_position = max_position.max(p);
            }
            None => sequential += 1,
        }
        skip_format_spec(bytes, &mut i);
    }
    let required = if has_positional {
        max_position.max(sequential)
    } else {
        sequential
    };
    (placeholders, required)
}

fn parse_position(bytes: &[u8], i: &mut usize) -> Option<usize> {
    let start = *i;
    let n = parse_number(bytes, i);
    if *i > start && *i < bytes.len() && bytes[*i] == b'$' {
        *i += 1;
        return Some(n);
    }
    *i = start;
    None
}

fn skip_format_spec(bytes: &[u8], i: &mut usize) {
    let len = bytes.len();
    while *i < len {
        match bytes[*i] {
            b'-' | b'+' | b' ' | b'0' => *i += 1,
            b'\'' => *i += 2,
            _ => break,
        }
    }
    parse_number(bytes, i);
    if *i < len && bytes[*i] == b'.' {
        *i += 1;
        parse_number(bytes, i);
    }
    if *i < len
        && matches!(
            bytes[*i],
            b's' | b'd' | b'u' | b'f' | b'F' | b'e' | b'E' | b'x' | b'X' | b'o' | b'b' | b'c'
        )
    {
        *i += 1;
    }
}

fn parse_number(bytes: &[u8], i: &mut usize) -> usize {
    let mut n: usize = 0;
    while *i < bytes.len() && bytes[*i].is_ascii_digit() {
        n = n.wrapping_mul(10).wrapping_add((bytes[*i] - b'0') as usize);
        *i += 1;
    }
    n
}
