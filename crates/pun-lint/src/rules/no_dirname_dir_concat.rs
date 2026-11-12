//! `lint/complexity/no-dirname-dir-concat`, ported from Rector 2.8.4's
//! `DirnameDirConcatStringToDirectStringPathRector` (rules/CodeQuality/Rector/Concat).
//!
//! `dirname(__DIR__) . '/file.txt'` becomes `__DIR__ . '/../file.txt'`, and `dirname(__DIR__) . '\some\file.txt'`
//! becomes `__DIR__ . '\..\some\file.txt'`. A `dirname()` with a level count stays, and so does a string without a
//! path separator.
//!
//! The changed string prints again in its own quotes, escaped as php-parser escapes it. A heredoc stays (Rector
//! changes it too).

use mago_syntax::cst::{BinaryOperator, Call, Expression, Literal, MagicConstant, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::prefer_absolute_include::print_string;
use super::rector_batch5::called_function_name;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoDirnameDirConcat;

impl Rule for NoDirnameDirConcat {
    fn id(&self) -> &'static str {
        "lint/complexity/no-dirname-dir-concat"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(concat) = node else { return };
        if !matches!(concat.operator, BinaryOperator::StringConcat(_)) {
            return;
        }
        // Mago parses `dirname(...)` as a partial application, so a function call is never a first-class callable.
        let Expression::Call(Call::Function(call)) = unparenthesize(concat.lhs) else {
            return;
        };
        if !called_function_name(ancestors, call.function)
            .is_some_and(|name| name.eq_ignore_ascii_case("dirname"))
        {
            return;
        }
        let Expression::Literal(Literal::String(string)) = unparenthesize(concat.rhs) else {
            return;
        };
        // avoid multiple dir nesting for now
        let mut arguments = call.argument_list.arguments.iter();
        let (Some(first), None) = (arguments.next(), arguments.next()) else {
            return;
        };
        if !matches!(
            unparenthesize(first.value()),
            Expression::MagicConstant(MagicConstant::Directory(_))
        ) {
            return;
        }
        let printed =
            (string.value.and_then(direct_path)).and_then(|path| print_string(string.kind, &path));
        let Some(printed) = printed else { return };
        out.push(Report::fixed(
            node_span(concat),
            "Use `__DIR__` with `..` in the path instead of `dirname(__DIR__)`.",
            Safety::Safe,
            vec![
                Edit::new(node_span(call), "__DIR__"),
                Edit::new(node_span(string), printed),
            ],
        ));
    }
}

/// The new value of the string: `/../` (linux paths) or `\..\` (windows paths) before the value without its leading
/// separators, or none for a value without a separator.
fn direct_path(value: &[u8]) -> Option<Vec<u8>> {
    let separator = if value.contains(&b'/') {
        b'/'
    } else if value.contains(&b'\\') {
        b'\\'
    } else {
        return None;
    };
    let start = value.iter().take_while(|&&b| b == separator).count();
    let mut path = vec![separator, b'.', b'.', separator];
    path.extend_from_slice(&value[start..]);
    Some(path)
}
