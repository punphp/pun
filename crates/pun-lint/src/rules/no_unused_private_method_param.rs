//! `lint/dead-code/no-unused-private-method-param`, ported from Rector 2.6.1's
//! `RemoveUnusedPrivateMethodParameterRector` (rules/DeadCode/Rector/ClassMethod), with `UnusedParameterResolver`
//! and `PrivateMethodParamRemover`.
//!
//! A private method's unused parameters go, with their `@param` tags and the matching arguments of the calls to it
//! from the class (`$this->name()`, `self::name()`, `static::name()`). A call that unpacks its arguments, passes a
//! named argument for a parameter that stays, or takes the method as a first-class callable keeps everything.
//!
//! Rector finds the callers by their type. Pun reads it from the syntax: `$this`, `self`, `static` and the class's
//! own name are the class, `parent` and other class names are not. A call on another receiver (`$other->name()`) or
//! a string naming the method (a callable) may reach the method too; Rector leaves such calls as they are, and so
//! does Pun, but only removes parameters after the last kept one then, where the extra arguments PHP ignores do not
//! shift the others.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_batch5::{
    list_removal, member_from, merge, param_used, remove_params, resolve_class_name,
};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUnusedPrivateMethodParam;

impl Rule for NoUnusedPrivateMethodParam {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-private-method-param"
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
        let (members, left_brace, name) = match node {
            Node::Class(class) => (&class.members, class.left_brace, Some(&class.name)),
            Node::AnonymousClass(class) => (&class.members, class.left_brace, None),
            _ => return,
        };
        let mut path = ancestors.to_vec();
        path.push(node);
        let this = name.map(|n| {
            let id = Identifier::Local(*n);
            resolve_class_name(&path, &id)
        });
        let class = ClassScope {
            path: &path,
            this: this.as_deref(),
        };
        for (index, member) in members.iter().enumerate() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            if !method
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Private(_)))
            {
                continue;
            }
            let params: Vec<&FunctionLikeParameter> =
                method.parameter_list.parameters.iter().collect();
            let removed: Vec<bool> = params.iter().map(|p| !param_used(method, p)).collect();
            if !removed.contains(&true) {
                continue;
            }
            let Some(mut edits) = caller_edits(&class, members, method, &params, &removed) else {
                continue;
            };
            let from = member_from(members, left_brace, index);
            edits.extend(remove_params(ctx, method, from, &removed));
            let edits = merge(edits);
            // A caller inside the method itself, which is printed anew: leave both.
            if edits.windows(2).any(|w| w[0].span.end > w[1].span.start) {
                continue;
            }
            out.push(Report {
                span: node_span(method),
                message: "Remove the unused parameters of this private method.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits,
                }),
            });
        }
    }
}

struct ClassScope<'p, 'a> {
    path: &'p [Node<'a, 'a>],
    /// The class's fully qualified name; `None` for an anonymous class.
    this: Option<&'p str>,
}

/// A call or callable that names the method.
enum Reference<'a> {
    /// A call on the class, with its arguments.
    Caller(&'a ArgumentList<'a>),
    /// A first-class callable on the class.
    Callable,
    /// A call on a receiver of unknown type, with its arguments, or a string naming the method.
    Unknown(Option<&'a ArgumentList<'a>>),
}

/// `removeCallerArgs`: the edits removing the arguments of the callers, or `None` when a caller keeps the method as
/// it is.
fn caller_edits(
    class: &ClassScope<'_, '_>,
    members: &Sequence<'_, ClassLikeMember<'_>>,
    method: &Method<'_>,
    params: &[&FunctionLikeParameter<'_>],
    removed: &[bool],
) -> Option<Vec<Edit>> {
    let mut references = Vec::new();
    for member in members.iter() {
        find_references(
            class,
            Node::ClassLikeMember(member),
            method.name.value,
            &mut references,
        );
    }
    let trailing = removed
        .iter()
        .position(|r| *r)
        .is_some_and(|first| removed[first..].iter().all(|r| *r));
    let removed_name = |key: usize, name: &[u8]| {
        removed.get(key).copied().unwrap_or(false) && params[key].variable.name[1..] == *name
    };
    let mut edits = Vec::new();
    for reference in &references {
        match reference {
            Reference::Callable => return None,
            Reference::Unknown(arguments) => {
                if !trailing {
                    return None;
                }
                // A named argument for a parameter that goes would be an error there.
                let named = arguments.iter().flat_map(|a| a.arguments.iter()).any(|a| {
                    matches!(a, Argument::Named(n) if params.iter().zip(removed).any(|(p, r)| *r && p.variable.name[1..] == *n.name.value))
                });
                if named {
                    return None;
                }
            }
            Reference::Caller(arguments) => {
                let args: Vec<&Argument> = arguments.arguments.iter().collect();
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
                    // php-parser prints the call anew; its object and name keep their code.
                    None => edits.push(Edit::new(node_span(*arguments), "()".to_string())),
                }
            }
        }
    }
    Some(edits)
}

/// The calls, first-class callables and strings in `node` that may reach the method `name`, outside nested classes.
fn find_references<'a>(
    class: &ClassScope<'_, '_>,
    node: Node<'a, 'a>,
    name: &[u8],
    out: &mut Vec<Reference<'a>>,
) {
    let named = |selector: &ClassLikeMemberSelector<'_>| match selector {
        ClassLikeMemberSelector::Identifier(id) => Some(id.value.eq_ignore_ascii_case(name)),
        _ => None,
    };
    // A class-like can't be declared inside a method, so only an anonymous class nests one.
    match node {
        Node::AnonymousClass(c) => {
            if let Some(args) = &c.argument_list {
                find_references(class, Node::PartialArgumentList(args), name, out);
            }
            return;
        }
        Node::MethodCall(call) => match named(&call.method) {
            Some(false) => {}
            Some(true) if is_this(call.object) => out.push(Reference::Caller(&call.argument_list)),
            _ => out.push(Reference::Unknown(Some(&call.argument_list))),
        },
        Node::NullSafeMethodCall(call) if named(&call.method) != Some(false) => {
            out.push(Reference::Unknown(Some(&call.argument_list)));
        }
        Node::StaticMethodCall(call) => match (named(&call.method), receiver(class, call.class)) {
            (Some(false), _) | (_, Some(false)) => {}
            (Some(true), Some(true)) => out.push(Reference::Caller(&call.argument_list)),
            _ => out.push(Reference::Unknown(Some(&call.argument_list))),
        },
        Node::MethodPartialApplication(call) if named(&call.method) != Some(false) => {
            out.push(if is_this(call.object) {
                Reference::Callable
            } else {
                Reference::Unknown(None)
            });
        }
        Node::StaticMethodPartialApplication(call) if named(&call.method) != Some(false) => {
            match receiver(class, call.class) {
                Some(false) => {}
                Some(true) => out.push(Reference::Callable),
                None => out.push(Reference::Unknown(None)),
            }
        }
        Node::LiteralString(s) if s.value.is_some_and(|v| v.eq_ignore_ascii_case(name)) => {
            out.push(Reference::Unknown(None));
        }
        _ => {}
    }
    node.visit_children(|child| find_references(class, child, name, out));
}

/// Whether `expr` is `$this`.
fn is_this(expr: &Expression<'_>) -> bool {
    matches!(expr, Expression::Variable(Variable::Direct(v)) if v.name.eq_ignore_ascii_case(b"$this"))
}

/// Whether the class part of a static call is the class (`Some(true)`), another class (`Some(false)`) or unknown.
fn receiver(class: &ClassScope<'_, '_>, expr: &Expression<'_>) -> Option<bool> {
    match expr {
        Expression::Self_(_) | Expression::Static(_) => Some(true),
        Expression::Parent(_) => Some(false),
        Expression::Identifier(id) => {
            let resolved = resolve_class_name(class.path, id);
            Some(
                class
                    .this
                    .is_some_and(|this| this.eq_ignore_ascii_case(&resolved)),
            )
        }
        e if is_this(e) => Some(true),
        _ => None,
    }
}
