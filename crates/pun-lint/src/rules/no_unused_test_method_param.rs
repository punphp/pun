//! `lint/dead-code/no-unused-test-method-param`, ported from Rector 2.6.1's
//! `RemoveTestsOverriddenPrivateMethodParameterRector` (rules/DeadCode/Rector/ClassMethod), with
//! `OverriddenParameterResolver` (rules/DeadCode/NodeCollector) and `PrivateMethodParamRemover`
//! (rules/DeadCode/NodeManipulator).
//!
//! In a test class, a parameter of a private method (not the constructor) whose first mention in the body is a plain
//! assignment to it, `$param = ...;`, as a statement of its own with no other mention of the variable, is never read:
//! it goes, with its `@param` tag and the matching argument of every call to the method from the class's methods. A
//! by-reference, variadic or promoted parameter stays, and so does every parameter of a method that uses a variable
//! variable or calls `compact()`, `extract()`, `func_get_arg()`, `func_get_args()`, `func_num_args()` or
//! `get_defined_vars()`.
//!
//! A caller is a method call or static call whose receiver PHPStan types as the class (`$this`, `self`, `static`,
//! the class's name, or a value of the class or a subclass). A caller that takes the method as a first-class callable,
//! unpacks its arguments or passes a named argument for a parameter that stays keeps everything as it is.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::class_method::{ClassInfo, class_info, is_private, type_of, walk};
use super::phpunit::is_in_test_class;
use super::rector_batch5::{list_removal, member_from, merge, remove_params, resolve_class_name};
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUnusedTestMethodParam;

impl Rule for NoUnusedTestMethodParam {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-test-method-param"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (members, left_brace) = match node {
            Node::Class(c) => (&c.members, c.left_brace),
            Node::AnonymousClass(c) => (&c.members, c.left_brace),
            _ => return,
        };
        // a cheap look first: a private method with a parameter
        let candidate = |m: &Method<'_>| {
            is_private(m)
                && !m.name.value.eq_ignore_ascii_case(b"__construct")
                && !m.parameter_list.parameters.is_empty()
        };
        if !members
            .iter()
            .any(|m| matches!(m, ClassLikeMember::Method(m) if candidate(m)))
        {
            return;
        }
        let mut path = ancestors.to_vec();
        path.push(node);
        if !is_in_test_class(ctx, &path, None) {
            return;
        }
        let facts = facts(ctx);
        let Some(class) = class_info(node, ancestors, &facts) else {
            return;
        };
        let source = ctx.source;
        for (index, member) in members.iter().enumerate() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            if !candidate(method) {
                continue;
            }
            let removed = overridden_parameters(method);
            if !removed.contains(&true) {
                continue;
            }
            let Some(mut edits) = caller_edits(&facts, ancestors, &class, method, &removed) else {
                continue;
            };
            let from = member_from(members, left_brace, index);
            edits.extend(remove_params(source, method, from, &removed));
            let edits = merge(edits);
            // a caller inside the method itself, which may be printed anew: leave both
            if edits.windows(2).any(|w| w[0].span.end > w[1].span.start) {
                continue;
            }
            out.push(Report {
                span: node_span(method),
                message: "This parameter is overwritten before it is read; remove it.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits,
                }),
            });
        }
    }
}

/// `OverriddenParameterResolver::resolve`, as a flag for each parameter.
fn overridden_parameters(method: &Method<'_>) -> Vec<bool> {
    let params: Vec<&FunctionLikeParameter<'_>> = method.parameter_list.parameters.iter().collect();
    let none = vec![false; params.len()];
    let MethodBody::Concrete(body) = &method.body else {
        return none;
    };
    let stmts: Vec<&Statement<'_>> = (body.statements.iter())
        .filter(|s| !matches!(s, Statement::Noop(_)))
        .collect();
    if stmts.is_empty() || has_indirect_variable_usage(Node::Method(method)) {
        return none;
    }
    params
        .iter()
        .map(|p| {
            p.ampersand.is_none()
                && p.ellipsis.is_none()
                && p.modifiers.is_empty()
                && p.hooks.is_none()
                && is_overridden_before_first_use(&stmts, p.variable.name)
        })
        .collect()
}

/// `isOverriddenBeforeFirstUse`: the first statement that mentions the variable mentions it once, as the variable a
/// plain assignment statement writes.
fn is_overridden_before_first_use(stmts: &[&Statement<'_>], name: &[u8]) -> bool {
    for stmt in stmts {
        let found = variables_named(Node::Statement(stmt), name);
        if found.is_empty() {
            continue;
        }
        if found.len() != 1 {
            return false;
        }
        let Statement::Expression(e) = stmt else {
            return false;
        };
        let Expression::Assignment(a) = unparenthesize(e.expression) else {
            return false;
        };
        if !matches!(a.operator, AssignmentOperator::Assign(_))
            || matches!(a.rhs, Expression::UnaryPrefix(u) if u.operator.is_reference())
        {
            return false;
        }
        return node_span(unparenthesize(a.lhs)) == found[0];
    }
    false
}

/// The spans of php-parser's `Variable` nodes named `name` (with its `$`) in `node`: not a property's name in a
/// declaration nor in a static property fetch, which php-parser keeps as identifiers.
fn variables_named(node: Node<'_, '_>, name: &[u8]) -> Vec<Span> {
    let mut out = Vec::new();
    walk(node, &mut |n, path| {
        if let Node::DirectVariable(v) = n
            && v.name == name
            && !matches!(
                path.iter().rev().find(|p| !matches!(p, Node::Variable(_) | Node::Expression(_))),
                Some(
                    Node::PropertyAbstractItem(_)
                        | Node::PropertyConcreteItem(_)
                        | Node::StaticPropertyAccess(_)
                )
            )
        {
            out.push(node_span(&n));
        }
        true
    });
    out
}

/// `hasIndirectVariableUsage`: a variable variable, or a call of a function that reads variables by name.
fn has_indirect_variable_usage(node: Node<'_, '_>) -> bool {
    const NAMES: [&[u8]; 6] = [
        b"compact",
        b"extract",
        b"func_get_arg",
        b"func_get_args",
        b"func_num_args",
        b"get_defined_vars",
    ];
    let mut found = false;
    walk(node, &mut |n, _| {
        found = found
            || match n {
                Node::IndirectVariable(_) | Node::NestedVariable(_) => true,
                Node::FunctionCall(call) => match call.function {
                    Expression::Identifier(id) => {
                        let name = id.value().rsplit(|b| *b == b'\\').next().unwrap_or_default();
                        NAMES.iter().any(|n| n.eq_ignore_ascii_case(name))
                    }
                    _ => false,
                },
                _ => false,
            };
        !found
    });
    found
}

/// A caller of the method: a call with its arguments, or a first-class callable.
enum Caller<'a> {
    Call(&'a ArgumentList<'a>),
    Callable,
}

/// `removeCallerArgs`: the edits removing the callers' arguments, or `None` when a caller keeps everything.
fn caller_edits(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    class: &ClassInfo<'_, '_>,
    method: &Method<'_>,
    removed: &[bool],
) -> Option<Vec<Edit>> {
    let params: Vec<&FunctionLikeParameter<'_>> = method.parameter_list.parameters.iter().collect();
    let name = method.name.value;
    let mut callers = Vec::new();
    for member in class.members.iter() {
        let ClassLikeMember::Method(m) = member else {
            continue;
        };
        walk(Node::Method(m), &mut |n, _| {
            let (receiver, selector, caller, is_static) = match n {
                Node::MethodCall(c) => (c.object, &c.method, Caller::Call(&c.argument_list), false),
                Node::StaticMethodCall(c) => (c.class, &c.method, Caller::Call(&c.argument_list), true),
                Node::MethodPartialApplication(c) => (c.object, &c.method, Caller::Callable, false),
                Node::StaticMethodPartialApplication(c) => {
                    (c.class, &c.method, Caller::Callable, true)
                }
                _ => return true,
            };
            if matches!(selector, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
                && is_class(facts, ancestors, class, receiver, is_static)
            {
                callers.push(caller);
            }
            true
        });
    }
    let removed_name = |key: usize, arg: &[u8]| {
        removed.get(key).copied().unwrap_or(false) && params[key].variable.name[1..] == *arg
    };
    let mut edits = Vec::new();
    for caller in &callers {
        let Caller::Call(arguments) = caller else {
            return None;
        };
        let args: Vec<&Argument<'_>> = arguments.arguments.iter().collect();
        for (key, arg) in args.iter().enumerate() {
            match arg {
                Argument::Positional(p) if p.ellipsis.is_some() => return None,
                Argument::Named(n) if !removed_name(key, n.name.value) => return None,
                _ => {}
            }
        }
        let drop: Vec<bool> = (0..args.len())
            .map(|k| removed.get(k).copied().unwrap_or(false))
            .collect();
        if !drop.contains(&true) {
            continue;
        }
        let items: Vec<Span> = args.iter().map(|a| node_span(*a)).collect();
        match list_removal(&items, &drop) {
            Some(list) => edits.extend(list),
            None => edits.push(Edit::new(node_span(*arguments), "()".to_string())),
        }
    }
    Some(edits)
}

/// `isObjectType($receiver, new ObjectType($className))`: the receiver is the class, or a value of the class or a
/// subclass.
fn is_class(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    class: &ClassInfo<'_, '_>,
    receiver: &Expression<'_>,
    is_static: bool,
) -> bool {
    let name = class.name.as_bytes();
    if is_static {
        return match receiver {
            Expression::Self_(_) | Expression::Static(_) => true,
            Expression::Parent(_) => false,
            Expression::Identifier(id) => {
                let resolved = resolve_class_name(ancestors, id);
                resolved.eq_ignore_ascii_case(&class.name)
                    || facts.codebase.is_instance_of(resolved.as_bytes(), name)
            }
            other => is_of_class(facts, other, name),
        };
    }
    is_of_class(facts, receiver, name)
}

/// Whether PHPStan's type of `expr` is an object of the class `name` or a subclass, every part of it.
fn is_of_class(facts: &Facts, expr: &Expression<'_>, name: &[u8]) -> bool {
    let Some(t) = type_of(facts, expr) else {
        return false;
    };
    !t.types.is_empty()
        && t.types.iter().all(|a| match a {
            TAtomic::Object(TObject::Named(n)) => {
                n.name.as_bytes().eq_ignore_ascii_case(name)
                    || facts.codebase.is_instance_of(n.name.as_bytes(), name)
            }
            _ => false,
        })
}
