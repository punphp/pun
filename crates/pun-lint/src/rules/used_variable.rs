//! Rector's `ExprUsedInNodeAnalyzer` (with `UsedVariableNameAnalyzer` and `CompactFuncCallAnalyzer`) and
//! `StmtsManipulator::isVariableUsedInNextStmt`, as the dead-code rules call them.

use mago_syntax::cst::*;

use super::php_parser::{List, array_item};
use crate::syntax::unparenthesize;

/// `BetterNodeFinder::findFirst($node, fn ($n) => ExprUsedInNodeAnalyzer::isUsed($n, $variable))`: whether `node`
/// or anything under it uses the variable `name` (without the `$`). A use is a variable of that name (ASCII case
/// insensitive, as `NodeNameResolver::isName`), any variable variable, any `include` or `require`, or a `compact()`
/// argument naming it.
pub(crate) fn used_in(node: Node<'_, '_>, parent: Option<Node<'_, '_>>, name: &[u8]) -> bool {
    match node {
        // php-parser names a property declaration with an identifier, not a variable.
        Node::DirectVariable(v) => {
            return !matches!(
                parent,
                Some(Node::PropertyAbstractItem(_) | Node::PropertyConcreteItem(_))
            ) && same_name(name, &v.name[1..]);
        }
        Node::IndirectVariable(_) | Node::NestedVariable(_) => return true,
        Node::IncludeConstruct(_)
        | Node::IncludeOnceConstruct(_)
        | Node::RequireConstruct(_)
        | Node::RequireOnceConstruct(_) => return true,
        // `A::$e` names a property (php-parser: VarLikeIdentifier); `A::$$e` and `A::${e}` hold an expression.
        Node::StaticPropertyAccess(access) => {
            return used_in(Node::Expression(access.class), Some(node), name)
                || match &access.property {
                    Variable::Direct(_) => false,
                    Variable::Nested(n) => used_in(Node::Variable(n.variable), Some(node), name),
                    Variable::Indirect(i) => {
                        used_in(Node::Expression(i.expression), Some(node), name)
                    }
                };
        }
        Node::FunctionCall(call) if is_compact(call.function) => {
            let mut arguments = call.argument_list.arguments.iter().map(|a| match a {
                Argument::Positional(a) => a.value,
                Argument::Named(a) => a.value,
            });
            if arguments.any(|v| names_in_compact(v, name)) {
                return true;
            }
        }
        _ => {}
    }
    let mut found = false;
    node.visit_children(|child| found = found || used_in(child, Some(node), name));
    found
}

/// `NodeNameResolver::isStringName($variableName, $usedName)`: an empty name matches nothing, `Object` matches only
/// itself, anything else matches ignoring ASCII case.
pub(crate) fn same_name(variable: &[u8], used: &[u8]) -> bool {
    match used {
        b"" => false,
        b"Object" => variable == used,
        _ => variable.eq_ignore_ascii_case(used),
    }
}

/// `NodeNameResolver::isName($funcCall, 'compact')`: an unqualified or fully qualified `compact`.
fn is_compact(function: &Expression<'_>) -> bool {
    match function {
        Expression::Identifier(Identifier::Local(id)) => id.value.eq_ignore_ascii_case(b"compact"),
        Expression::Identifier(Identifier::FullyQualified(id)) => {
            id.value.eq_ignore_ascii_case(b"\\compact")
        }
        _ => false,
    }
}

/// `CompactFuncCallAnalyzer::isInArgOrArrayItemNodes` for one argument or array item value.
fn names_in_compact(value: &Expression<'_>, name: &[u8]) -> bool {
    match unparenthesize(value) {
        Expression::Literal(Literal::String(s)) => s.value == Some(name),
        Expression::Array(a) => array_names(&a.elements, name),
        Expression::LegacyArray(a) => array_names(&a.elements, name),
        _ => false,
    }
}

fn array_names(elements: &TokenSeparatedSequence<'_, ArrayElement<'_>>, name: &[u8]) -> bool {
    elements
        .iter()
        .filter_map(array_item)
        .any(|(_, value)| names_in_compact(value, name))
}

/// `StmtsManipulator::isVariableUsedInNextStmt($stmtsAware, $from, $name)`: whether the statements of `list` from
/// index `from` use the variable. When the list is a `try` block (`try_` is that `try`), its catches and its finally
/// block come after it.
pub(crate) fn used_in_next(
    list: &List<'_, '_>,
    from: usize,
    try_: Option<&Try<'_>>,
    name: &[u8],
) -> bool {
    list.stmts[from..]
        .iter()
        .any(|s| used_in(Node::Statement(s), None, name))
        || try_.is_some_and(|t| {
            t.catch_clauses
                .iter()
                .any(|c| used_in(Node::TryCatchClause(c), None, name))
                || t.finally_clause
                    .iter()
                    .any(|f| used_in(Node::Block(&f.block), None, name))
        })
}

/// The `try` whose block holds the statement `ancestors` ends with, if any: `ancestors` ends with the statement, the
/// block, then the `try`.
pub(crate) fn try_of<'ast, 'arena>(ancestors: &[Node<'ast, 'arena>]) -> Option<&'ast Try<'arena>> {
    let n = ancestors.len();
    match (
        ancestors.get(n.checked_sub(3)?),
        ancestors.get(n.checked_sub(2)?),
    ) {
        (Some(Node::Try(t)), Some(Node::Block(b))) if std::ptr::eq(*b, &t.block) => Some(t),
        _ => None,
    }
}
