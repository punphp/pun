//! `lint/phpunit/add-data-provider-iterator-return-type`, ported from Rector 2.6.1's
//! `ReturnIteratorInDataProviderRector` (rules/CodeQuality/Rector/Class_) and its `DataProviderMethodsFinder`.
//!
//! In a test class, a data provider (a method of the class named by a generic `@dataProvider` tag or by the first
//! argument, a string, of a `#[DataProvider]` attribute) without a return type that yields (`yield`, not only
//! `yield from`) gets `: \Iterator`.

use mago_syntax::cst::{
    ClassLikeMember, Expression, Literal, Method, MethodBody, Node, NodeKind, PartialArgument,
    Yield,
};

use super::annotations::{TagValue, named_tags};
use super::{class_body, is_in_test_class};
use crate::rules::lint_b_doc::Doc;
use crate::rules::rector_batch5::resolve_class_name;
use crate::rules::types::{add_return_type, any_scoped};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const DATA_PROVIDER_ATTRIBUTES: &[&str] = &[
    "PHPUnit\\Framework\\Attributes\\DataProvider",
    "Codeception\\Attribute\\DataProvider",
];

pub struct AddDataProviderIteratorReturnType;

impl Rule for AddDataProviderIteratorReturnType {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-data-provider-iterator-return-type"
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
        let (members, _, _) = class_body(node).unwrap();
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let methods: Vec<&Method<'_>> = members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(method) => Some(method),
                _ => None,
            })
            .collect();
        // `Class_::getMethod()`: the name without case.
        let get_method = |name: &str| {
            methods
                .iter()
                .copied()
                .find(|m| m.name.value.eq_ignore_ascii_case(name.as_bytes()))
        };
        let mut providers: Vec<&Method<'_>> = Vec::new();
        for method in &methods {
            let mut names = Vec::new();
            if let Some(doc) = Doc::of(ctx.source, &scope, node_span(*method).start) {
                let Some(tags) = named_tags(ctx, &scope, &doc, "dataProvider") else {
                    return;
                };
                for (_, value) in tags {
                    if let TagValue::Generic(value) = value {
                        names.push(value);
                    }
                }
            }
            for list in method.attribute_lists.iter() {
                for attribute in list.attributes.iter() {
                    let name = resolve_class_name(&scope, &attribute.name);
                    if !DATA_PROVIDER_ATTRIBUTES
                        .iter()
                        .any(|a| a.eq_ignore_ascii_case(&name))
                    {
                        continue;
                    }
                    // `$attribute->args[0]`, whose absence Rector does not survive
                    let Some(argument) = attribute
                        .argument_list
                        .as_ref()
                        .and_then(|l| l.arguments.iter().next())
                    else {
                        return;
                    };
                    let value = match argument {
                        PartialArgument::Positional(a) => a.value,
                        PartialArgument::Named(a) => a.value,
                        _ => continue,
                    };
                    if let Expression::Literal(Literal::String(s)) = value
                        && let Some(v) = s.value
                    {
                        names.push(String::from_utf8_lossy(v).into_owned());
                    }
                }
            }
            for name in names {
                let Some(method_name) = match_method_name(&name) else {
                    continue;
                };
                if let Some(provider) = get_method(method_name)
                    && !providers.iter().any(|p| std::ptr::eq(*p, provider))
                {
                    providers.push(provider);
                }
            }
        }
        for provider in providers {
            let (None, MethodBody::Concrete(body)) = (&provider.return_type_hint, &provider.body)
            else {
                continue;
            };
            let has_yields = any_scoped(Node::Block(body), &mut |n| {
                matches!(n, Node::Yield(Yield::Value(_) | Yield::Pair(_)))
            });
            if !has_yields {
                continue;
            }
            out.push(add_return_type(
                node_span(&provider.name),
                node_span(&provider.parameter_list).end,
                "\\Iterator",
            ));
        }
    }
}

/// `DataProviderNodes::matchMethodName()`: the word characters (`\w`, ASCII) the value starts with.
fn match_method_name(content: &str) -> Option<&str> {
    let end = content
        .bytes()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == b'_')
        .count();
    (end > 0).then(|| &content[..end])
}
