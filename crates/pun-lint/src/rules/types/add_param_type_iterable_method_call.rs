//! `lint/types/add-param-type-iterable-method-call`, ported from Rector 2.8.4's
//! `AddClosureParamTypeFromIterableMethodCallRector` (rules/TypeDeclaration/Rector/FunctionLike) with
//! `TypeUnwrapper`: a closure passed to a method of an iterable object whose docblock gives it key and value types
//! (`Collection<int, string>` of a class that declares no templates, PHPStan's `Collection&iterable<int, string>`)
//! gets them on the parameters that the method's callable parameter names `TKey` and `TValue` (classes of the
//! object's namespace, as PHPStan reads names no template declares).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::callable::{TCallable, TCallableSignature};
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Argument, ClassLikeMemberSelector, Expression, FunctionLikeParameter, Hint, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::facts::{Facts, facts};
use super::names::resolve_class_name;
use super::param::{add_hint, param_declaration};
use crate::rules::class_method::type_of;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PhpVersionFeature::MIXED_TYPE`.
const MIXED_TYPE: u32 = 80000;

pub struct AddParamTypeIterableMethodCall;

impl Rule for AddParamTypeIterableMethodCall {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-iterable-method-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else { return };
        let ClassLikeMemberSelector::Identifier(name) = &call.method else {
            return;
        };
        let facts = facts(ctx);
        let Some((class, key, value)) = type_of(&facts, call.object).and_then(|t| iterable_object(&facts, t)) else {
            return;
        };
        let Some(method) = facts.documented().get_method(class.as_bytes(), name.value) else {
            return;
        };
        let callables: Vec<Option<&TCallableSignature>> = method
            .parameters
            .iter()
            .map(|p| p.type_metadata.as_ref().and_then(|t| first_callable(&t.type_union)))
            .collect();
        // methodSignatureUsesCallableWithIteratorTypes
        if !callables.iter().flatten().any(|c| {
            c.parameters.iter().any(|p| {
                p.get_type_signature().is_some_and(|t| iterable_part(&class, t).is_some())
            })
        }) {
            return;
        }
        let args: Vec<&Expression<'_>> = call
            .argument_list
            .arguments
            .iter()
            .map(Argument::value)
            .collect();
        // callUsesClosures
        if !args.iter().any(|v| matches!(unparenthesize(v), Expression::Closure(_))) {
            return;
        }
        let mut edits: Vec<Edit> = Vec::new();
        // `getArgs()` is a list, so a named argument too takes the parameter at its position (checked with Rector 2.8.4)
        for (index, arg) in args.iter().enumerate() {
            let Expression::Closure(closure) = unparenthesize(arg) else {
                continue;
            };
            let Some(Some(callable)) = callables.get(index) else {
                continue;
            };
            let params: Vec<&FunctionLikeParameter<'_>> = closure.parameter_list.parameters.iter().collect();
            for (i, callable_param) in callable.parameters.iter().enumerate() {
                let (Some(param), Some(t)) = (params.get(i), callable_param.get_type_signature()) else {
                    continue;
                };
                let new: Option<&TUnion> = match iterable_part(&class, t) {
                    Some(Part::Value) => Some(&value),
                    Some(Part::Key) => key.as_ref(),
                    None => continue,
                };
                edits.extend(refactor_parameter(&facts, ctx.source, ancestors, param, new, ctx.php_version));
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report::fixed(
            node_span(call),
            "Add the iterable's key and value types to the closure's parameters.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// The iterable object of `t`: PHPStan's intersection of a class that declares no templates and is `Traversable` with
/// `iterable<key, value>` from the docblock's type arguments (`X<value>` or `X<key, value>`); its class, key and
/// value types (no key type for `X<value>`, whose key is an explicit `mixed`).
fn iterable_object(facts: &Facts, t: &TUnion) -> Option<(String, Option<TUnion>, TUnion)> {
    let [TAtomic::Object(TObject::Named(object))] = t.types.as_ref() else {
        return None;
    };
    let arguments = object.type_parameters.as_deref()?;
    let class = facts.documented().get_class_like(object.name.as_bytes())?;
    if !class.template_types.is_empty()
        || !class
            .all_parent_interfaces
            .iter()
            .any(|i| i.as_bytes().eq_ignore_ascii_case(b"Traversable"))
    {
        return None;
    }
    let (key, value) = match arguments {
        [value] => (None, value.clone()),
        [key, value] => (Some(key.clone()), value.clone()),
        _ => return None,
    };
    Some((String::from_utf8_lossy(object.name.as_bytes()).into_owned(), key, value))
}

/// `TypeUnwrapper::unwrapFirstCallableTypeFromUnionType`: the first callable of a union.
fn first_callable(t: &TUnion) -> Option<&TCallableSignature> {
    t.types.iter().find_map(|a| match a {
        TAtomic::Callable(TCallable::Signature(signature)) => Some(signature),
        _ => None,
    })
}

enum Part {
    Key,
    Value,
}

/// `TypeUnwrapper::isIterableTypeValue` and `isIterableTypeKey`: the type is a class of `class`'s namespace named
/// `TValue` or `TKey`.
fn iterable_part(class: &str, t: &TUnion) -> Option<Part> {
    let [atomic] = t.types.as_ref() else {
        return None;
    };
    // a name no class has (`TValue`) is a reference to a symbol
    let TAtomic::Reference(TReference::Symbol { name, .. }) = atomic else { return None };
    let name = name.as_bytes();
    let name = String::from_utf8_lossy(name);
    let namespace = |n: &str| n.rsplit_once('\\').map_or(String::new(), |(ns, _)| ns.to_string());
    if namespace(&name) != namespace(class) {
        return None;
    }
    if name.ends_with("\\TValue") {
        Some(Part::Value)
    } else if name.ends_with("\\TKey") {
        Some(Part::Key)
    } else {
        None
    }
}

/// `refactorParameter()`: the type (`None` for an explicit `mixed`) for the parameter, unless it has that type already (a different one is
/// replaced) or none maps.
fn refactor_parameter(
    facts: &Facts,
    source: &str,
    ancestors: &[Node<'_, '_>],
    param: &FunctionLikeParameter<'_>,
    t: Option<&TUnion>,
    php_version: u32,
) -> Option<Edit> {
    let declaration = match t {
        Some(t) => param_declaration(t, &facts.codebase, php_version)?,
        // `MixedTypeMapper`: an explicit `mixed` from PHP 8.0
        None if php_version >= MIXED_TYPE => "mixed".to_string(),
        None => return None,
    };
    let Some(hint) = &param.hint else {
        return Some(add_hint(param, &declaration));
    };
    if same_type(source, ancestors, hint, &declaration) {
        return None;
    }
    Some(Edit::new(node_span(hint), declaration))
}

/// Whether the hint names the type `declaration` writes: the same words, class names resolved.
fn same_type(source: &str, ancestors: &[Node<'_, '_>], hint: &Hint<'_>, declaration: &str) -> bool {
    let written = match hint {
        Hint::Identifier(id) => format!("\\{}", resolve_class_name(ancestors, id)),
        other => {
            let span = node_span(other);
            source.get(span.start..span.end).unwrap_or_default().to_string()
        }
    };
    written.eq_ignore_ascii_case(declaration)
}
