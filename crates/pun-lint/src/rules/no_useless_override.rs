//! `lint/dead-code/no-useless-override`, ported from Rector 2.6.1's `RemoveParentDelegatingClassMethodRector`
//! (rules/DeadCode/Rector/ClassMethod).
//!
//! A method (not the constructor) whose whole body is `parent::sameName($a, $b)` or `return parent::sameName($a, $b);`
//! with its own parameters in order goes, when the parent class has a method of that name that is not private, with
//! the same `static` and `public`, the same native parameter types where the child declares one and the same native
//! return type where the child declares one. It stays when it is final, abstract, private, returns by reference, has
//! attributes, a parameter that is variadic, by reference, has a default or attributes, or a doc comment with
//! `@param`, `@return` (or their `@phpstan-`/`@psalm-` forms) or `@deprecated`. A parent Pun does not know keeps it.
//! Union and intersection types count as different (PHPStan prints them in its own order).

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::class_method::{
    ClassInfo, class_info, doc_tags, hint_text, is_final, is_private, is_public, is_static,
    mapped_type_text, member_doc,
};
use super::rector_batch5::remove_members;
use super::types::facts::{Facts, facts};
use super::types::statement_count;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessOverride;

impl Rule for NoUselessOverride {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-override"
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
        let facts = facts(ctx);
        let Some(class) = class_info(node, ancestors, &facts) else {
            return;
        };
        for (index, member) in class.members.iter().enumerate() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            if !is_removable(ctx, ancestors, &facts, &class, method, index) {
                continue;
            }
            let mut removed = vec![false; class.members.len()];
            removed[index] = true;
            out.push(Report {
                span: node_span(method),
                message: format!(
                    "Remove the method `{}()`, which only calls its parent with the same arguments.",
                    String::from_utf8_lossy(method.name.value)
                ),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: remove_members(
                        ctx.source,
                        class.members,
                        class.left_brace,
                        class.right_brace,
                        &removed,
                    ),
                }),
            });
        }
    }
}

/// `refactor()` returning `REMOVE_NODE`.
fn is_removable(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    facts: &Facts,
    class: &ClassInfo<'_, '_>,
    method: &Method<'_>,
    index: usize,
) -> bool {
    let name = method.name.value;
    if name.eq_ignore_ascii_case(b"__construct") {
        return false;
    }
    // An abstract method has no body.
    let MethodBody::Concrete(body) = &method.body else {
        return false;
    };
    if is_final(method) || is_private(method) || method.ampersand.is_some() {
        return false;
    }
    if statement_count(ctx.source, body) != 1 || !method.attribute_lists.is_empty() {
        return false;
    }
    const REFINING: [&str; 7] = [
        "@param",
        "@phpstan-param",
        "@psalm-param",
        "@return",
        "@phpstan-return",
        "@psalm-return",
        "@deprecated",
    ];
    if member_doc(ctx.source, class, index)
        .is_some_and(|doc| doc_tags(doc).iter().any(|t| REFINING.contains(t)))
    {
        return false;
    }
    // `matchParentMethodReflection`.
    let Some(parent) = class.extends.as_deref() else {
        return false;
    };
    let codebase = &facts.codebase;
    let Some((parent_method, meta)) = (codebase.get_class(parent.as_bytes()))
        .and_then(|_| codebase.get_declaring_method(parent.as_bytes(), name))
        .and_then(|m| Some((m, m.method_metadata.as_ref()?)))
        .filter(|(_, meta)| !meta.visibility.is_private())
    else {
        return false;
    };
    let Some(statement) = body.statements.first() else {
        return false;
    };
    if !is_parent_call_delegating_params(method, statement) {
        return false;
    }
    // `isSignatureMatching`.
    if is_static(method) != meta.is_static || is_public(method) != meta.visibility.is_public() {
        return false;
    }
    for (position, param) in method.parameter_list.parameters.iter().enumerate() {
        let Some(parent_param) = parent_method.parameters.get(position) else {
            return false;
        };
        let Some(hint) = &param.hint else {
            continue;
        };
        let parent_type = parent_param
            .get_type_declaration_metadata()
            .and_then(|t| mapped_type_text(&t.type_union));
        if parent_type.is_none() || parent_type != hint_text(ctx.source, ancestors, hint) {
            return false;
        }
    }
    let Some(return_hint) = &method.return_type_hint else {
        return true;
    };
    let parent_type = parent_method
        .return_type_declaration_metadata
        .as_ref()
        .and_then(|t| mapped_type_text(&t.type_union));
    parent_type.is_some() && parent_type == hint_text(ctx.source, ancestors, &return_hint.hint)
}

/// `isParentCallDelegatingParams`: the sole statement is `parent::name(...)` (or `return` of it) passing each
/// parameter, in order, as a plain variable.
fn is_parent_call_delegating_params(method: &Method<'_>, statement: &Statement<'_>) -> bool {
    let expr = match statement {
        Statement::Return(r) => r.value,
        Statement::Expression(e) => Some(e.expression),
        _ => None,
    };
    let Some(Expression::Call(Call::StaticMethod(call))) = expr.map(unparenthesize) else {
        return false;
    };
    let (Expression::Parent(_), ClassLikeMemberSelector::Identifier(called)) =
        (call.class, &call.method)
    else {
        return false;
    };
    if !called.value.eq_ignore_ascii_case(method.name.value) {
        return false;
    }
    let params: Vec<&FunctionLikeParameter<'_>> = method.parameter_list.parameters.iter().collect();
    let args: Vec<&Argument<'_>> = call.argument_list.arguments.iter().collect();
    if params.len() != args.len() {
        return false;
    }
    params.iter().zip(&args).all(|(param, arg)| {
        if param.ellipsis.is_some()
            || param.ampersand.is_some()
            || param.default_value.is_some()
            || !param.attribute_lists.is_empty()
        {
            return false;
        }
        let Argument::Positional(arg) = arg else {
            return false;
        };
        arg.ellipsis.is_none()
            && matches!(arg.value, Expression::Variable(Variable::Direct(v)) if v.name == param.variable.name)
    })
}
