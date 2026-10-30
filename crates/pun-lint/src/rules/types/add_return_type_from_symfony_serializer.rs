//! `lint/types/add-return-type-from-symfony-serializer`, ported from Rector 2.6.1's
//! `ReturnTypeFromSymfonySerializerRector` (rules/TypeDeclaration/Rector/ClassMethod): a method whose only statement
//! returns `$serializer->deserialize($data, Foo::class, $format)` on a Symfony `Serializer` returns a `Foo`.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_syntax::cst::{
    Access, Argument, Call, ClassLikeConstantSelector, Expression, Literal, LiteralString, Node,
    NodeKind, Statement,
};

use super::facts::facts;
use super::names::resolve_class_name;
use super::{add_return_type, class_scope, statement_count, untyped};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromSymfonySerializer;

const SERIALIZER: &str = "Symfony\\Component\\Serializer\\Serializer";

impl Rule for AddReturnTypeFromSymfonySerializer {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-symfony-serializer"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = untyped(node, ancestors, ctx) else {
            return;
        };
        if statement_count(ctx.source, f.body) != 1 {
            return;
        }
        let Some(Statement::Return(r)) = f.body.statements.first() else {
            return;
        };
        let Some(Expression::Call(Call::Method(call))) = r.value else {
            return;
        };
        let mago_syntax::cst::ClassLikeMemberSelector::Identifier(name) = &call.method else {
            return;
        };
        if !name.value.eq_ignore_ascii_case(b"deserialize") {
            return;
        }
        // three arguments (an unpacked one too), none named
        let args: Vec<&Argument<'_>> = call.argument_list.arguments.iter().collect();
        let [_, Argument::Positional(class), Argument::Positional(_)] = args.as_slice() else {
            return;
        };
        let facts = facts(ctx);
        // `isObjectType($var, Serializer)`
        let is_serializer = facts.full_type(node_span(call.object)).is_some_and(|t| {
            !t.types.is_empty()
                && t.types.iter().all(|a| {
                    let name = match a {
                        TAtomic::Object(TObject::Named(n)) => n.name,
                        // a class Mago does not know
                        TAtomic::Reference(TReference::Symbol { name, .. }) => *name,
                        _ => return false,
                    };
                    name.as_bytes().eq_ignore_ascii_case(SERIALIZER.as_bytes())
                        || facts
                            .codebase
                            .is_instance_of(name.as_bytes(), SERIALIZER.as_bytes())
                })
        });
        if !is_serializer {
            return;
        }
        // `ValueResolver::getValue`: a string, `Foo::class`, `self::class`, `static::class`, `parent::class`, or
        // another expression with a constant string type (a class constant, a concatenation)
        let class_constant = match class.value {
            Expression::Access(Access::ClassConstant(c)) => match &c.constant {
                ClassLikeConstantSelector::Identifier(k)
                    if k.value.eq_ignore_ascii_case(b"class") =>
                {
                    Some(c.class)
                }
                _ => None,
            },
            _ => None,
        };
        let r#type = match (class.value, class_constant) {
            (Expression::Literal(Literal::String(LiteralString { value: Some(v), .. })), _) => {
                String::from_utf8_lossy(v)
                    .trim_start_matches('\\')
                    .to_string()
            }
            (_, Some(Expression::Identifier(class))) => resolve_class_name(ancestors, class),
            (
                _,
                Some(
                    keyword
                    @ (Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)),
                ),
            ) => {
                let scope = class_scope(ancestors);
                let name = match keyword {
                    Expression::Parent(_) => scope.and_then(|s| s.parent_class),
                    _ => scope.and_then(|s| s.name),
                };
                let Some(name) = name else {
                    return;
                };
                name
            }
            // an object's class (`$holder::class`): PHPStan's type is a class string, not a value
            (_, Some(_)) => return,
            (other, _) => {
                let Some(v) = facts.full_type(node_span(other)).and_then(|t| {
                    (t.get_single_literal_string_value())
                        .map(|v| String::from_utf8_lossy(v).into_owned())
                        .or_else(|| t.get_single_class_string_value().map(|c| c.to_string()))
                }) else {
                    return;
                };
                v
            }
        };
        out.push(add_return_type(f.span, f.at, &format!("\\{type}")));
    }
}
