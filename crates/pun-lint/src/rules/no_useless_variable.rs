//! `lint/dead-code/no-useless-variable`, ported from Rector 2.8.4's `SimplifyUselessVariableRector`
//! (rules/CodeQuality/Rector/FunctionLike), with the attributes of its `ByRefVariableNodeVisitor`,
//! `GlobalVariableNodeVisitor`, `StaticVariableNodeVisitor` and `ByRefReturnNodeVisitor`.
//!
//! In a statement list, the first `return` after another statement decides: when that statement assigns the variable
//! the `return` returns (`$x = expr; return $x;`), it goes and the `return` returns the value (`return expr;`); a
//! compound assignment stays. Left alone: a `return` by reference, an assignment with a comment, a `return` after a
//! comment that names the variable (commented-out code), a `return` with a doc comment's `@var` type, a variable `global` or `static` earlier in a list that holds
//! the `return`, one a function-like around it takes or a closure before it uses by reference, and one that held an
//! object of an instantiable class before the assignment (its native type, read where the assignment's right side or
//! else the closest place before it names the variable).

use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::visibility::Visibility;
use mago_syntax::cst::*;
use mago_syntax::token::TokenKind;
use pun_problem::{Edit, Fix, Safety};

use super::rector_batch5::resolve_class_name;
use super::rector_code_quality::text;
use super::rector_stmts::{HOLDERS, holder_list, previous_token_end, remove, tokens};
use super::side_effect::any_node;
use super::types::facts::facts;
use crate::syntax::{node_span, same_expression, statements, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessVariable;

impl Rule for NoUselessVariable {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-variable"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(HOLDERS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(list) = holder_list(ctx.source, node, ancestors) else {
            return;
        };
        let Some((key, r#return)) =
            list.stmts
                .iter()
                .enumerate()
                .skip(1)
                .find_map(|(k, s)| match s {
                    Statement::Return(r) => Some((k, r)),
                    _ => None,
                })
        else {
            return;
        };
        let previous = list.stmts[key - 1];
        let Some(variable) = r#return
            .value
            .filter(|v| matches!(v, Expression::Variable(_)))
        else {
            return;
        };
        let Statement::Expression(statement) = previous else {
            return;
        };
        let Expression::Assignment(assign) = statement.expression else {
            return;
        };
        if !matches!(assign.operator, AssignmentOperator::Assign(_))
            || matches!(assign.rhs, Expression::UnaryPrefix(u) if u.operator.is_reference())
        {
            return;
        }
        if !same_expression(ctx.source, assign.lhs, variable)
            || returns_by_reference(ancestors, node)
            || is_static_or_global(node, ancestors, previous, variable)
            || is_used_by_reference(node, ancestors, variable)
            || is_new_instance(ctx, ancestors, node, assign, variable)
            || has_comment(ctx, list.inner.start, previous)
            || is_mentioned_in_comment(ctx.source, previous, r#return, variable)
            || has_var_type(ctx.source, previous, r#return)
        {
            return;
        }
        let value = text(ctx, unparenthesize(assign.rhs)).to_string();
        let mut edits = remove(ctx, &list, &[key - 1]);
        edits.push(Edit::new(node_span(variable), value));
        out.push(Report {
            span: node_span(assign),
            message: "Return the value instead of assigning it to a variable first.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `AssignAndBinaryMap::getAlternative`: the binary operator of a compound assignment, with its operand limits; none
/// for `??=`.
/// `IS_INSIDE_BYREF_FUNCTION_LIKE`: the list is in a function-like that returns by reference (`function &f()`).
fn returns_by_reference(ancestors: &[Node<'_, '_>], holder: Node<'_, '_>) -> bool {
    let function = std::iter::once(holder)
        .chain(ancestors.iter().rev().copied())
        .find(|a| {
            matches!(
                a,
                Node::Function(_)
                    | Node::Method(_)
                    | Node::Closure(_)
                    | Node::ArrowFunction(_)
                    | Node::PropertyHook(_)
                    | Node::Class(_)
                    | Node::AnonymousClass(_)
                    | Node::Interface(_)
                    | Node::Trait(_)
                    | Node::Enum(_)
            )
        });
    match function {
        Some(Node::Function(f)) => f.ampersand.is_some(),
        Some(Node::Method(m)) => m.ampersand.is_some(),
        Some(Node::Closure(c)) => c.ampersand.is_some(),
        Some(Node::PropertyHook(h)) => h.ampersand.is_some(),
        _ => false,
    }
}

/// `IS_GLOBAL_VAR` and `IS_STATIC_VAR`: a `global $x;` or `static $x;` before the statement on the way to the
/// `return` in a list that holds it, up to the class it is in.
fn is_static_or_global(
    holder: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    previous: &Statement<'_>,
    variable: &Expression<'_>,
) -> bool {
    let Expression::Variable(Variable::Direct(v)) = variable else {
        return false;
    };
    let path: Vec<Node<'_, '_>> = ancestors
        .iter()
        .copied()
        .chain(std::iter::once(holder))
        .collect();
    let mut before = node_span(previous).end;
    for list in path.iter().rev() {
        if matches!(
            list,
            Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
        ) {
            break;
        }
        if HOLDERS.contains(&list.kind()) {
            let declared = statements(*list)
                .into_iter()
                .filter(|s| node_span(*s).end <= before)
                .any(|s| match s {
                    Statement::Global(g) => g
                        .variables
                        .iter()
                        .any(|g| matches!(g, Variable::Direct(d) if d.name == v.name)),
                    Statement::Static(s) => s.items.iter().any(|i| i.variable().name == v.name),
                    _ => false,
                });
            if declared {
                return true;
            }
        }
        before = node_span(list).start;
    }
    false
}

/// `IS_BYREF_VAR`: a function-like around the `return` takes the variable by reference (a parameter, or a closure's
/// `use`), or a closure inside one, before the variable, uses it by reference.
fn is_used_by_reference(
    holder: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    variable: &Expression<'_>,
) -> bool {
    let Expression::Variable(Variable::Direct(v)) = variable else {
        return false;
    };
    let at = node_span(variable).start;
    let by_ref_use = |c: &Closure<'_>| {
        c.use_clause.as_ref().is_some_and(|u| {
            u.variables
                .iter()
                .any(|u| u.ampersand.is_some() && u.variable.name == v.name)
        })
    };
    let by_ref_param = |list: &FunctionLikeParameterList<'_>| {
        list.parameters
            .iter()
            .any(|p| p.ampersand.is_some() && p.variable.name == v.name)
    };
    std::iter::once(holder)
        .chain(ancestors.iter().copied())
        .any(|a| {
            let own = match a {
                Node::Function(f) => by_ref_param(&f.parameter_list),
                Node::Method(m) => by_ref_param(&m.parameter_list),
                Node::Closure(c) => by_ref_param(&c.parameter_list) || by_ref_use(c),
                Node::ArrowFunction(f) => by_ref_param(&f.parameter_list),
                _ => return false,
            };
            own || any_node(
                a,
                &|n| matches!(n, Node::Closure(c) if node_span(c).start < at && by_ref_use(c)),
            )
        })
}

/// `CallAnalyzer::isNewInstance`: before the assignment, the variable's native type is one class, which can be
/// instantiated (a class, not abstract, its constructor public).
fn is_new_instance(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    holder: Node<'_, '_>,
    assign: &Assignment<'_>,
    variable: &Expression<'_>,
) -> bool {
    let Expression::Variable(Variable::Direct(v)) = variable else {
        return false;
    };
    let scope = std::iter::once(holder)
        .chain(ancestors.iter().rev().copied())
        .find(|a| matches!(a, Node::Function(_) | Node::Method(_) | Node::Closure(_)))
        .unwrap_or_else(|| ancestors.first().copied().unwrap_or(holder));
    let mut read = None;
    find_last(Node::Expression(assign.rhs), v.name, usize::MAX, &mut read);
    if read.is_none() {
        find_last(scope, v.name, node_span(assign).start, &mut read);
    }
    let Some(read) = read else { return false };
    let params = match scope {
        Node::Function(f) => Some(&f.parameter_list),
        Node::Method(m) => Some(&m.parameter_list),
        Node::Closure(c) => Some(&c.parameter_list),
        _ => None,
    };
    let facts = facts(ctx);
    // A parameter's native type is its declaration's.
    let class =
        match params.and_then(|l| l.parameters.iter().find(|p| node_span(&p.variable) == read)) {
            Some(param) => match &param.hint {
                Some(Hint::Identifier(id)) => resolve_class_name(ancestors, id),
                _ => return false,
            },
            None => {
                let Some(ty) = facts.native_type(read) else {
                    return false;
                };
                let [TAtomic::Object(TObject::Named(object))] = ty.types.as_ref() else {
                    return false;
                };
                object.name.to_string()
            }
        };
    let codebase = &facts.codebase;
    codebase.get_class_like(class.as_bytes()).is_some_and(|c| {
        c.kind == SymbolKind::Class
            && !c.flags.is_abstract()
            && codebase
                .get_method(class.as_bytes(), b"__construct")
                .and_then(|m| m.method_metadata.as_ref())
                .is_none_or(|m| m.visibility == Visibility::Public)
    })
}

/// Where the variable `name` last got its value under `node` before `before`, not inside a class or a function-like
/// other than `node`: the last place that names it, or the right side of a later plain assignment to it.
pub(crate) fn find_last(
    node: Node<'_, '_>,
    name: &[u8],
    before: usize,
    found: &mut Option<pun_problem::Span>,
) {
    node.visit_children(|child| {
        if node_span(&child).start >= before
            || matches!(
                child,
                Node::Closure(_)
                    | Node::ArrowFunction(_)
                    | Node::Function(_)
                    | Node::Class(_)
                    | Node::AnonymousClass(_)
            )
        {
            return;
        }
        match child {
            Node::Assignment(a)
                if matches!(a.operator, AssignmentOperator::Assign(_))
                    && matches!(a.lhs, Expression::Variable(Variable::Direct(v)) if v.name == name) =>
            {
                find_last(Node::Expression(a.rhs), name, before, found);
                *found = Some(node_span(a.rhs));
                return;
            }
            Node::DirectVariable(v) if v.name == name => *found = Some(node_span(v)),
            _ => {}
        }
        find_last(child, name, before, found);
    });
}

/// `hasSomeComment`: comments before the statement, which php-parser gives it.
fn has_comment(ctx: &crate::Context<'_>, from: usize, statement: &Statement<'_>) -> bool {
    let source = ctx.source;
    let start = node_span(statement).start;
    tokens(source, previous_token_end(ctx, from, start), start)
        .iter()
        .any(|t| t.kind.is_comment())
}

/// `isVariableMentionedInComment`: a comment before the `return` (php-parser's comments of the `return`, from the end
/// of the statement before it) names the returned variable (`$name`), which may be commented-out code.
fn is_mentioned_in_comment(
    source: &str,
    previous: &Statement<'_>,
    r#return: &Return<'_>,
    variable: &Expression<'_>,
) -> bool {
    let Expression::Variable(Variable::Direct(direct)) = variable else {
        return false;
    };
    let name = String::from_utf8_lossy(direct.name);
    tokens(source, node_span(previous).end, node_span(r#return).start)
        .iter()
        .any(|t| t.kind.is_comment() && source[t.start..t.end].contains(name.as_ref()))
}

/// `isReturnWithVarAnnotation`: the `return`'s doc comment has a `@var` type that is not `mixed`.
fn has_var_type(source: &str, previous: &Statement<'_>, r#return: &Return<'_>) -> bool {
    let start = node_span(r#return).start;
    let Some(doc) = tokens(source, node_span(previous).end, start)
        .into_iter()
        .rfind(|t| t.kind == TokenKind::DocBlockComment)
    else {
        return false;
    };
    let doc = &source[doc.start..doc.end];
    doc.split("@var")
        .nth(1)
        .and_then(|rest| rest.split_whitespace().next())
        .is_some_and(|ty| ty != "mixed" && ty != "*/")
}
