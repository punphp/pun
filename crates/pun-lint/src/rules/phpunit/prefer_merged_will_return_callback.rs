//! `lint/phpunit/prefer-merged-will-return-callback`, ported from rector-phpunit's
//! `MergeWithCallableAndWillReturnRector` (rules/CodeQuality/Rector/MethodCall).
//!
//! In a test class, `->with($this->callback(function (...) { ...; return true; }))->willReturn($value)` becomes
//! `->willReturnCallback(function (...) use (...) { ...; return $value; })`: the closure's last `return true;` returns
//! the value, the variables the value uses that the closure lacks are added to its `use`, and its return type is the
//! value's type as Rector declares it (`mapPHPStanTypeToPhpParserNode` of the scope's type, `true` and `false` kept).
//! Arguments of `with()` after the callback go.

use mago_syntax::cst::{Call, Expression, Node, NodeKind, Statement, Variable};
use pun_problem::{Edit, Safety, Span};

use super::{
    AnyCall, NewArg, arg_value, expr_text, is_constant, is_in_test_class, report, rewrite_args,
};
use crate::rules::rector_stmts::block_list;
use crate::rules::types::declare::{return_declaration, union_order};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferMergedWillReturnCallback;

impl Rule for PreferMergedWillReturnCallback {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-merged-will-return-callback"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let call = AnyCall::of(node).unwrap();
        if !is_in_test_class(ctx, ancestors, None) || !call.is_name(&["willReturn"]) {
            return;
        }
        let Expression::Call(Call::Method(with)) = unparenthesize(call.receiver) else {
            return;
        };
        let Some(with) = AnyCall::of(Node::MethodCall(with)).filter(|c| c.is_name(&["with"]))
        else {
            return;
        };
        let Some(first) = with.args().into_iter().next() else {
            return;
        };
        let Expression::Call(Call::Method(callback)) = arg_value(first) else {
            return;
        };
        let Some(callback) =
            AnyCall::of(Node::MethodCall(callback)).filter(|c| c.is_name(&["callback"]))
        else {
            return;
        };
        let Some(closure_arg) = callback.args().into_iter().next() else {
            return;
        };
        let Expression::Closure(closure) = arg_value(closure_arg) else {
            return;
        };
        let source = ctx.source;
        let list = block_list(source, &closure.body);
        if list.is_empty() {
            return;
        }
        let Some(Statement::Return(last)) = list.last() else {
            return;
        };
        let Some(old_value) = last.value.filter(|v| is_constant(v, "true")) else {
            return;
        };
        let Some(returned) = call.args().into_iter().next().map(arg_value) else {
            return;
        };
        let facts = facts(ctx);
        let Some(t) = facts.full_type(node_span(returned)) else {
            return;
        };
        let declared = return_declaration(
            union_order(t.types.as_ref()),
            &facts.codebase,
            ctx.php_version,
            &[],
            true,
        );

        let mut edits = vec![Edit::new(node_span(old_value), expr_text(ctx, returned))];
        // `resolveExternalUses`
        let params: Vec<&[u8]> = closure
            .parameter_list
            .parameters
            .iter()
            .map(|p| p.variable.name)
            .collect();
        let existing: Vec<&[u8]> = closure
            .use_clause
            .iter()
            .flat_map(|u| u.variables.iter())
            .map(|v| v.variable.name)
            .collect();
        let mut seen: Vec<&[u8]> = Vec::new();
        let mut stack = vec![Node::Expression(returned)];
        while let Some(node) = stack.pop() {
            if let Node::Expression(Expression::Variable(Variable::Direct(v))) = node {
                let name = v.name;
                if name != b"$this"
                    && !params.contains(&name)
                    && !existing.contains(&name)
                    && !seen.contains(&name)
                {
                    seen.push(name);
                }
            }
            let mut children = Vec::new();
            node.visit_children(|c| children.push(c));
            stack.extend(children.into_iter().rev());
        }
        if !seen.is_empty() {
            let names: Vec<String> = seen
                .iter()
                .map(|n| String::from_utf8_lossy(n).into_owned())
                .collect();
            let names = names.join(", ");
            edits.push(match &closure.use_clause {
                Some(clause) => {
                    let end = clause
                        .variables
                        .iter()
                        .last()
                        .map_or(0, |v| node_span(v).end);
                    Edit::new(Span::new(end, end), format!(", {names}"))
                }
                None => {
                    let end = closure.parameter_list.right_parenthesis.end.offset as usize;
                    Edit::new(Span::new(end, end), format!(" use ({names})"))
                }
            });
        }
        match (&closure.return_type_hint, &declared) {
            (Some(hint), Some(declared)) => {
                edits.push(Edit::new(node_span(&hint.hint), declared.clone()))
            }
            (Some(hint), None) => {
                let end = match &closure.use_clause {
                    Some(clause) => clause.right_parenthesis.end.offset as usize,
                    None => closure.parameter_list.right_parenthesis.end.offset as usize,
                };
                edits.push(Edit::new(Span::new(end, node_span(&hint.hint).end), ""));
            }
            (None, Some(declared)) => {
                let end = match &closure.use_clause {
                    Some(clause) => clause.right_parenthesis.end.offset as usize,
                    None => closure.parameter_list.right_parenthesis.end.offset as usize,
                };
                edits.push(Edit::new(Span::new(end, end), format!(": {declared}")));
            }
            (None, None) => {}
        }
        let closure_text = applied(source, node_span(closure), edits);
        let with_edits = rewrite_args(
            ctx,
            &with,
            Some("willReturnCallback"),
            &[NewArg::New(closure_text)],
        );
        let text = applied(source, with.span, with_edits);
        out.push(report(
            call.span,
            "Merge `with()` of a callback and `willReturn()` into `willReturnCallback()`.",
            Safety::Safe,
            vec![Edit::new(call.span, text)],
        ));
    }
}

/// `source[span]` with `edits` (inside it, not overlapping) applied.
fn applied(source: &str, span: Span, mut edits: Vec<Edit>) -> String {
    edits.sort_by_key(|e| (e.span.start, e.span.end));
    let mut out = String::new();
    let mut at = span.start;
    for edit in edits {
        out.push_str(&source[at..edit.span.start]);
        out.push_str(&edit.text);
        at = edit.span.end;
    }
    out.push_str(&source[at..span.end]);
    out
}
