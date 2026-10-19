//! `lint/style/prefer-spread`, ported from Rector 2.6.1's `CallUserFuncArrayToVariadicRector`
//! (rules/CodingStyle/Rector/FuncCall) and `ArrayCallableToMethodCallFactory` (src/NodeFactory).
//!
//! `call_user_func_array('name', $args)` becomes `name(...$args)`, `call_user_func_array([$object, 'method'], $args)`
//! `$object->method(...$args)` when `$object` (a variable or a property fetch) is of one class, and
//! `call_user_func_array($object->method(...), $args)` `$object->method(...$args)`. Arguments past the second go.
//! Pun reads the class from the native type; a `$this->name` of a private property without a type or doc comment
//! has the type of the value the constructor last assigns it among its own statements, as Rector's PHPStan runs with
//! `inferPrivatePropertyTypeFromConstructor` (which reads the scope at the constructor's end).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{named, text};
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferSpread;

impl Rule for PreferSpread {
    fn id(&self) -> &'static str {
        "lint/style/prefer-spread"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::ARRAY_SPREAD
        70400
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if !named(call, "call_user_func_array") {
            return;
        }
        let args = &call.argument_list.arguments;
        let (Some(first), Some(second)) = (args.get(0), args.get(1)) else {
            return;
        };
        let spread = format!("...{}", text(ctx, second.value()));
        let code = match first.value() {
            Expression::Literal(Literal::String(name)) => format!(
                "{}({spread})",
                String::from_utf8_lossy(name.value.unwrap_or_default())
            ),
            Expression::Array(Array { elements, .. })
            | Expression::LegacyArray(LegacyArray { elements, .. }) => {
                let Some(object) = method_call_object(ctx, ancestors, elements) else {
                    return;
                };
                let Some(Expression::Literal(Literal::String(method))) =
                    elements.get(1).and_then(element_value)
                else {
                    return;
                };
                format!(
                    "{}->{}({spread})",
                    text(ctx, object),
                    String::from_utf8_lossy(method.value.unwrap_or_default())
                )
            }
            Expression::PartialApplication(PartialApplication::Method(method))
                if method.argument_list.is_first_class_callable() =>
            {
                let open = method.argument_list.left_parenthesis.start.offset as usize;
                format!("{}({spread})", &ctx.source[node_span(method).start..open])
            }
            _ => return,
        };
        let span = node_span(call);
        out.push(Report {
            span,
            message: "Unpack the arguments instead of calling `call_user_func_array()`."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, code)],
            }),
        });
    }
}

/// `ArrayCallableToMethodCallFactory::create`: the first of exactly two items, a variable or property fetch whose
/// type names exactly one class.
fn method_call_object<'a>(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    elements: &'a TokenSeparatedSequence<'a, ArrayElement<'a>>,
) -> Option<&'a Expression<'a>> {
    if elements.len() != 2 {
        return None;
    }
    let object = element_value(elements.get(0)?)?;
    if !matches!(
        object,
        Expression::Variable(_) | Expression::Access(Access::Property(_))
    ) {
        return None;
    }
    // `ClassNameFromObjectTypeResolver::resolve`: `getObjectClassNames()` has one name.
    let typed = constructor_assigned(ancestors, object).unwrap_or(object);
    let facts = facts(ctx);
    let ty = facts.native_type(node_span(typed))?;
    matches!(ty.types.as_ref(), [TAtomic::Object(TObject::Named(_))]).then_some(object)
}

/// An array item's value, without the `&` of a by-reference item (php-parser's `byRef`).
fn element_value<'a>(element: &ArrayElement<'a>) -> Option<&'a Expression<'a>> {
    let (_, value) = super::php_parser::array_item(element)?;
    Some(match value {
        Expression::UnaryPrefix(u) if u.operator.is_reference() => u.operand,
        _ => value,
    })
}

/// The value `__construct` last assigns to `$this->name` among its own statements, when `object` is `$this->name` of a
/// private property of the enclosing class declared without a type or doc comment.
// ponytail: top-level constructor statements only, PHPStan merges the constructor's branches too.
fn constructor_assigned<'a>(
    ancestors: &[Node<'a, 'a>],
    object: &Expression<'_>,
) -> Option<&'a Expression<'a>> {
    let Expression::Access(Access::Property(fetch)) = object else {
        return None;
    };
    let (Expression::Variable(Variable::Direct(this)), ClassLikeMemberSelector::Identifier(name)) =
        (fetch.object, &fetch.property)
    else {
        return None;
    };
    let Node::Class(class) = ancestors
        .iter()
        .rev()
        .find(|a| matches!(a, Node::Class(_) | Node::AnonymousClass(_)))?
    else {
        return None;
    };
    if this.name != b"$this" {
        return None;
    }
    let untyped = class.members.iter().any(|m| match m {
        ClassLikeMember::Property(Property::Plain(p)) => {
            p.hint.is_none()
                && p.modifiers
                    .iter()
                    .any(|m| matches!(m, Modifier::Private(_)))
                && p.items
                    .iter()
                    .any(|i| &i.variable().name[1..] == name.value)
        }
        _ => false,
    });
    let constructor = class.members.iter().find_map(|m| match m {
        ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
            match &m.body {
                MethodBody::Concrete(block) => Some(block),
                MethodBody::Abstract(_) => None,
            }
        }
        _ => None,
    })?;
    if !untyped {
        return None;
    }
    constructor.statements.iter().rev().find_map(|s| {
        let Statement::Expression(e) = s else { return None };
        let Expression::Assignment(a) = e.expression else { return None };
        let Expression::Access(Access::Property(target)) = a.lhs else { return None };
        (matches!(target.object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
            && matches!(&target.property, ClassLikeMemberSelector::Identifier(n) if n.value == name.value)
            && matches!(a.operator, AssignmentOperator::Assign(_)))
        .then_some(a.rhs)
    })
}
