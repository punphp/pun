//! `lint/complexity/prefer-named-literal-argument`, ported from Mago 1.57.0's `literal-named-argument`
//! (mago-linter src/rule/clarity/literal_named_argument.rs). PHP 8.0 and later.
//!
//! Reports each literal (string, number, `true`, `false`, `null`) passed positionally to a function, except to the
//! variadic functions and the ones marked `@no-named-arguments` (mago-linter rule/utils/consts.rs), once a call has
//! at least `threshold` (default 1) of them. The first argument is skipped unless `check-first-argument`. Mago
//! reports only; it has no fix.

use mago_span::HasSpan;
use mago_syntax::cst::{Argument, Expression, Literal, Node, NodeKind};

use super::lint_b_names::function_call_matches_any;
use super::mago_helpers::{boolean, configure, uint};
use crate::syntax::span;
use crate::{Context, Report, Rule};

pub struct PreferNamedLiteralArgument {
    pub check_first_argument: bool,
    pub threshold: usize,
}

impl Default for PreferNamedLiteralArgument {
    fn default() -> Self {
        PreferNamedLiteralArgument {
            check_first_argument: false,
            threshold: 1,
        }
    }
}

impl Rule for PreferNamedLiteralArgument {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-named-literal-argument"
    }

    fn min_php_version(&self) -> u32 {
        80000
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        configure(options, |key, value| {
            match key {
                "check-first-argument" => self.check_first_argument = boolean(value)?,
                "threshold" => self.threshold = uint(value, u64::MAX)?,
                _ => return None,
            }
            Some(())
        })
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if function_call_matches_any(ancestors, call, &VARIADIC_FUNCTIONS).is_some()
            || function_call_matches_any(ancestors, call, &NO_NAMED_ARGUMENTS_FUNCTIONS).is_some()
        {
            return;
        }
        let literals: Vec<(&Literal<'_>, &[u8])> = call
            .argument_list
            .arguments
            .iter()
            .enumerate()
            .filter(|(i, _)| *i > 0 || self.check_first_argument)
            .filter_map(|(_, argument)| match argument {
                Argument::Positional(p) => match p.value {
                    Expression::Literal(literal) => Some(literal),
                    _ => None,
                },
                Argument::Named(_) => None,
            })
            .map(|literal| {
                let raw: &[u8] = match literal {
                    Literal::String(l) => l.raw,
                    Literal::Integer(l) => l.raw,
                    Literal::Float(l) => l.raw,
                    Literal::True(_) => b"true",
                    Literal::False(_) => b"false",
                    Literal::Null(_) => b"null",
                };
                (literal, raw)
            })
            .collect();
        if literals.len() < self.threshold {
            return;
        }
        for (literal, raw) in literals {
            let raw = String::from_utf8_lossy(raw);
            out.push(Report {
                span: span(literal.span()),
                message: format!(
                    "Literal argument `{raw}` should be passed as a named argument for clarity."
                ),
                help: Some(format!(
                    "Consider using a named argument instead: `function_name(param: {raw})`."
                )),
                fix: None,
            });
        }
    }
}

/// mago-linter's `VARIADIC_FUNCTIONS`.
const VARIADIC_FUNCTIONS: [&str; 40] = [
    "setlocale",
    "sprintf",
    "printf",
    "fprintf",
    "sscanf",
    "fscanf",
    "call_user_func",
    "forward_static_call",
    "var_dump",
    "debug_zval_dump",
    "register_shutdown_function",
    "register_tick_function",
    "pack",
    "array_merge",
    "array_merge_recursive",
    "array_replace",
    "array_replace_recursive",
    "array_intersect",
    "array_intersect_key",
    "array_intersect_assoc",
    "array_intersect_ukey",
    "array_intersect_uassoc",
    "array_uintersect",
    "array_uintersect_assoc",
    "array_uintersect_uassoc",
    "array_diff",
    "array_diff_key",
    "array_diff_assoc",
    "array_diff_ukey",
    "array_diff_uassoc",
    "array_udiff",
    "array_udiff_assoc",
    "array_udiff_uassoc",
    "array_map",
    "array_push",
    "array_unshift",
    "array_multisort",
    "compact",
    "min",
    "max",
];

/// mago-linter's `NO_NAMED_ARGUMENTS_FUNCTIONS`.
const NO_NAMED_ARGUMENTS_FUNCTIONS: [&str; 47] = [
    "func_get_args",
    "strlen",
    "strcmp",
    "strncmp",
    "strcasecmp",
    "strncasecmp",
    "str_starts_with",
    "str_ends_with",
    "str_contains",
    "str_decrement",
    "str_increment",
    "error_reporting",
    "defined",
    "get_class",
    "get_called_class",
    "get_parent_class",
    "method_exists",
    "property_exists",
    "trait_exists",
    "class_exists",
    "interface_exists",
    "function_exists",
    "enum_exists",
    "class_alias",
    "password_get_info",
    "password_hash",
    "password_needs_rehash",
    "password_verify",
    "password_algos",
    "array_merge_recursive",
    "array_replace",
    "array_replace_recursive",
    "array_keys",
    "array_values",
    "array_count_values",
    "array_column",
    "array_reverse",
    "array_diff_ukey",
    "array_udiff",
    "array_udiff_assoc",
    "array_diff_uassoc",
    "array_udiff_uassoc",
    "array_uintersect_assoc",
    "array_intersect_uassoc",
    "array_uintersect_uassoc",
    "array_intersect_ukey",
    "array_uintersect",
];
