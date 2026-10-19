//! `lint/php-8.4/add-csv-escape-argument`, ported from Rector 2.6.1's `AddEscapeArgumentRector`
//! (rules/Php84/Rector/FuncCall).
//!
//! PHP 8.4 deprecates relying on the default `$escape` of the CSV functions, so a call that leaves it out gets
//! `escape: '\\'`, the old default: `fputcsv()`, `fgetcsv()` and `str_getcsv()`, and `SplFileObject`'s
//! `setCsvControl()`, `fputcsv()` and `fgetcsv()`. Rector asks PHPStan whether the method's object is an
//! `SplFileObject`; Pun knows it only where the syntax proves it (see `lint_b_helpers::is_object_of`).

use mago_syntax::cst::{Argument, ArgumentList, ClassLikeMemberSelector, Node, NodeKind};
use pun_problem::{Fix, Safety};

use super::lint_b_helpers::{append_argument, is_object_of};
use super::rector_code_quality::named;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddCsvEscapeArgument;

impl Rule for AddCsvEscapeArgument {
    fn id(&self) -> &'static str {
        "lint/php-8.4/add-csv-escape-argument"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::REQUIRED_ESCAPE_PARAMETER
        80400
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall, NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (list, at_most) = match node {
            Node::FunctionCall(call) => {
                let at_most = if named(call, "fputcsv") || named(call, "fgetcsv") {
                    4
                } else if named(call, "str_getcsv") {
                    3
                } else {
                    return;
                };
                (&call.argument_list, at_most)
            }
            Node::MethodCall(call) => {
                // `$this->getName($node->name)` compared with `in_array(..., true)`: the name as written.
                let ClassLikeMemberSelector::Identifier(name) = &call.method else {
                    return;
                };
                let at_most = match name.value {
                    b"setCsvControl" | b"fgetcsv" => 2,
                    b"fputcsv" => 3,
                    _ => return,
                };
                if !is_object_of(
                    ancestors,
                    call.object,
                    "SplFileObject",
                    node_span(call).start,
                ) {
                    return;
                }
                (&call.argument_list, at_most)
            }
            _ => return,
        };
        if skip(list, at_most) {
            return;
        }
        out.push(Report {
            span: node_span(&node),
            message: "Add the escape argument to the CSV call, as PHP 8.4 requires.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![append_argument(list, "escape: '\\\\'")],
            }),
        });
    }
}

/// `shouldSkipNamedArg` (an argument named `escape`), or an escape argument already passed by position.
fn skip(list: &ArgumentList<'_>, at_most: usize) -> bool {
    list.arguments.len() > at_most
        || list
            .arguments
            .iter()
            .any(|a| matches!(a, Argument::Named(n) if n.name.value == b"escape"))
}
