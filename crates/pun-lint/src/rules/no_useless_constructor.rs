//! `lint/dead-code/no-useless-constructor`, ported from Rector 2.6.1's `RemoveParentDelegatingConstructorRector`
//! (rules/DeadCode/Rector/ClassMethod).
//!
//! A public, non-final constructor whose whole body is `parent::__construct($a, $b)` with its own parameters, in
//! order, goes when the parent class has a constructor, each typed parameter has the type of the parent's parameter
//! at its place (PHPStan's type, docblock included), each default equals the parent's default, no parameter is a
//! private promoted property or has an attribute, and the doc comment has no `@param` (nor its `@phpstan-`/`@psalm-`
//! form). A parent Pun does not know keeps it. Union and intersection types count as different, and a default Pun
//! cannot read on either side counts as different.

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::class_method::{
    ClassInfo, class_info, default_value, doc_tags, expr_value, hint_text, is_final, is_public,
    mapped_type_text, member_doc,
};
use super::rector_batch5::remove_members;
use super::types::facts::{Facts, facts};
use super::types::statement_count;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUselessConstructor;

impl Rule for NoUselessConstructor {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-useless-constructor"
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
                message: "Remove the constructor, which only calls the parent constructor with the same arguments."
                    .to_string(),
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
    if !method.name.value.eq_ignore_ascii_case(b"__construct") {
        return false;
    }
    let MethodBody::Concrete(body) = &method.body else {
        return false;
    };
    if statement_count(ctx.source, body) != 1 || is_final(method) || !is_public(method) {
        return false;
    }
    // `matchParentConstructorReflection`, with the parent's PHPDoc types.
    let Some(parent) = class.extends.as_deref() else {
        return false;
    };
    let codebase = facts.documented();
    if codebase.get_class(parent.as_bytes()).is_none() {
        return false;
    }
    let Some(constructor) = codebase.get_declaring_method(parent.as_bytes(), b"__construct") else {
        return false;
    };
    // `matchParentConstructorCallArgs` and `isParameterAndArgCountAndOrderIdentical`.
    let Some(Statement::Expression(statement)) = body.statements.first() else {
        return false;
    };
    let Expression::Call(Call::StaticMethod(call)) = unparenthesize(statement.expression) else {
        return false;
    };
    let (Expression::Parent(_), ClassLikeMemberSelector::Identifier(called)) =
        (call.class, &call.method)
    else {
        return false;
    };
    if !called.value.eq_ignore_ascii_case(b"__construct") {
        return false;
    }
    let params: Vec<&FunctionLikeParameter<'_>> = method.parameter_list.parameters.iter().collect();
    let args: Vec<&Argument<'_>> = call.argument_list.arguments.iter().collect();
    if params.len() != args.len() {
        return false;
    }
    let same_names = params.iter().zip(&args).all(|(param, arg)| {
        let (Argument::Positional(PositionalArgument { value, .. })
        | Argument::Named(NamedArgument { value, .. })) = arg;
        matches!(value, Expression::Variable(Variable::Direct(v)) if v.name == param.variable.name)
    });
    if !same_names {
        return false;
    }
    // `areConstructorAndParentParameterTypesMatching`.
    for (position, param) in params.iter().enumerate() {
        let private_promoted = param
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Private(_)));
        if private_promoted {
            return false;
        }
        let parent_param = constructor.parameters.get(position);
        let different_default = |default: &FunctionLikeParameterDefaultValue<'_>| {
            let Some(parent_param) = parent_param else {
                return false;
            };
            if !parent_param.flags.has_default() {
                return true;
            }
            let child = expr_value(default.value);
            child.is_none() || child != default_value(facts, parent_param)
        };
        let Some(hint) = &param.hint else {
            if param.default_value.as_ref().is_some_and(different_default) {
                return false;
            }
            continue;
        };
        let Some(parent_param) = parent_param else {
            continue;
        };
        let parent_type = parent_param
            .get_type_metadata()
            .filter(|t| t.from_docblock || parent_param.get_type_declaration_metadata().is_some())
            .and_then(|t| mapped_type_text(&t.type_union));
        if parent_type.is_none() || parent_type != hint_text(ctx.source, ancestors, hint) {
            return false;
        }
        if param.default_value.as_ref().is_some_and(different_default) {
            return false;
        }
    }
    if params.iter().any(|p| !p.attribute_lists.is_empty()) {
        return false;
    }
    !member_doc(ctx.source, class, index).is_some_and(|doc| {
        doc_tags(doc)
            .iter()
            .any(|t| ["@param", "@phpstan-param", "@psalm-param"].contains(t))
    })
}
