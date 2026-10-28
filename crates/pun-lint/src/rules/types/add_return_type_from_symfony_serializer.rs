//! `lint/types/add-return-type-from-symfony-serializer`, ported from Rector 2.6.1's
//! `ReturnTypeFromSymfonySerializerRector` (rules/TypeDeclaration/Rector/ClassMethod): a method whose only statement
//! returns `$serializer->deserialize($data, Foo::class, $format)` on a Symfony `Serializer` returns a `Foo`.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_syntax::cst::{
    Argument, Call, ClassLikeConstantSelector, Expression, Literal, Node, NodeKind, Statement,
};

use super::facts::facts;
use super::names::resolve_class_name;
use super::{add_return_type, statement_count, untyped};
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
        // three positional arguments, no `...` placeholder
        let args: Vec<&Argument<'_>> = call.argument_list.arguments.iter().collect();
        let [_, Argument::Positional(class), _] = args.as_slice() else {
            return;
        };
        if args
            .iter()
            .any(|a| !matches!(a, Argument::Positional(p) if p.ellipsis.is_none()))
        {
            return;
        }
        let facts = facts(ctx);
        // `isObjectType($var, Serializer)`
        let Some(t) = facts.full_type(node_span(call.object)) else {
            return;
        };
        let is_serializer = !t.types.is_empty()
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
            });
        if !is_serializer {
            return;
        }
        // `ValueResolver::getValue`: a string, or `Foo::class`
        let r#type = match class.value {
            Expression::Literal(Literal::String(s)) => match s.value {
                Some(v) => String::from_utf8_lossy(v)
                    .trim_start_matches('\\')
                    .to_string(),
                None => return,
            },
            Expression::Access(mago_syntax::cst::Access::ClassConstant(c)) => {
                let ClassLikeConstantSelector::Identifier(constant) = &c.constant else {
                    return;
                };
                let Expression::Identifier(class) = c.class else {
                    return;
                };
                if !constant.value.eq_ignore_ascii_case(b"class") {
                    return;
                }
                resolve_class_name(ancestors, class)
            }
            _ => return,
        };
        out.push(add_return_type(f.span, f.at, &format!("\\{type}")));
    }
}
