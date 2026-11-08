//! `lint/types/no-nullsafe-on-non-null`, ported from Rector 2.8.4's `CleanupUnneededNullsafeOperatorRector`
//! (rules/CodeQuality/Rector/NullsafeMethodCall): `f()?->m()` becomes `f()->m()` when the function, method or static
//! method called before `?->` declares that it returns one object type.
//!
//! `ReturnStrictTypeAnalyzer::resolveMethodCallReturnType` finds the callee as `ReflectionResolver` does: a function
//! by its name, a method by the one class of its receiver's type (`getType`, see [`super::rector_type`]; a union,
//! nullable or not, has none), a static method by its class name, `self`, `static` or `parent`. It reads the
//! callee's native return type, or the stub's for one of PHP's own; `static` of a method of the calling class stays
//! `static`, which is no object type.
//!
//! Rector prints the call anew: the receiver's own text, `->`, the name and the arguments as `pMaybeMultiline` writes
//! them, so comments around `?->` go.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{Call, ClassLikeMemberSelector, Expression, Node, NodeKind};
use pun_problem::{Edit, Safety};

use super::facts::{Facts, facts, is_builtin_class, is_builtin_function};
use super::names::resolve_class_name;
use super::rector_type::{RectorType, get_type};
use super::{ClassScope, class_scope};
use crate::rules::pest::{indent_at, print_args};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNullsafeOnNonNull;

/// `PhpVersionFeature::NULLSAFE_OPERATOR`.
const NULLSAFE_OPERATOR: u32 = 80000;

impl Rule for NoNullsafeOnNonNull {
    fn id(&self) -> &'static str {
        "lint/types/no-nullsafe-on-non-null"
    }

    fn min_php_version(&self) -> u32 {
        NULLSAFE_OPERATOR
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::NullSafeMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::NullSafeMethodCall(call) = node else {
            return;
        };
        let ClassLikeMemberSelector::Identifier(name) = &call.method else {
            return;
        };
        let object = unparenthesize(call.object);
        let Expression::Call(inner) = object else {
            return;
        };
        let facts = facts(ctx);
        let scope = class_scope(ancestors);
        let Some(callee) = callee(&facts, ancestors, scope.as_ref(), inner) else {
            return;
        };
        if !returns_object(&facts, scope.as_ref(), &callee) {
            return;
        }
        let span = node_span(call);
        let object_span = node_span(object);
        let text = format!(
            "{}->{}({})",
            &ctx.source[object_span.start..object_span.end],
            String::from_utf8_lossy(name.value),
            print_args(ctx, &call.argument_list, indent_at(ctx, span.start)),
        );
        out.push(Report::fixed(
            span,
            "The call before `?->` never returns `null`: use `->`.",
            Safety::Safe,
            vec![Edit::new(span, text)],
        ));
    }
}

/// A function or method a call reaches: its declaring class for a method, and its name.
pub(crate) struct Callee<'f> {
    pub(crate) metadata: &'f FunctionLikeMetadata,
    class: Option<String>,
    name: Vec<u8>,
}

/// `ReflectionResolver::resolveFunctionLikeReflectionFromCall` for a function, method or static method call.
pub(crate) fn callee<'f>(
    facts: &'f Facts,
    ancestors: &[Node<'_, '_>],
    scope: Option<&ClassScope>,
    call: &Call<'_>,
) -> Option<Callee<'f>> {
    let codebase = &facts.codebase;
    let function = |name: &[u8]| {
        codebase.get_function(name).map(|metadata| Callee {
            metadata,
            class: None,
            name: name.to_vec(),
        })
    };
    let (class, method) = match call {
        Call::Function(f) => {
            let Expression::Identifier(id) = f.function else {
                return None;
            };
            let written = String::from_utf8_lossy(id.value()).into_owned();
            if let Some(fq) = written.strip_prefix('\\') {
                return function(fq.as_bytes());
            }
            let namespaced = super::namespace_of(ancestors).map(|ns| format!("{ns}\\{written}"));
            if let Some(found) = namespaced.and_then(|n| function(n.as_bytes())) {
                return Some(found);
            }
            return (!written.contains('\\'))
                .then(|| function(written.as_bytes()))
                .flatten();
        }
        Call::Method(m) => {
            let ClassLikeMemberSelector::Identifier(method) = &m.method else {
                return None;
            };
            let RectorType::Scope(Some(t)) = get_type(facts, m.object) else {
                return None;
            };
            let class = match t.types.as_ref() {
                [TAtomic::Object(TObject::Named(named))] if named.intersection_types.is_none() => {
                    if named.is_this || named.is_static {
                        scope?.name.clone()?
                    } else {
                        named.name.to_string()
                    }
                }
                [TAtomic::Object(TObject::Enum(e))] => e.get_name().to_string(),
                _ => return None,
            };
            (class, method.value)
        }
        Call::StaticMethod(s) => {
            let ClassLikeMemberSelector::Identifier(method) = &s.method else {
                return None;
            };
            let class = match unparenthesize(s.class) {
                Expression::Identifier(id) => resolve_class_name(ancestors, id),
                Expression::Self_(_) | Expression::Static(_) => scope?.name.clone()?,
                Expression::Parent(_) => scope?.parent_class.clone()?,
                _ => return None,
            };
            (class, method.value)
        }
        Call::NullSafeMethod(_) => return None,
    };
    let metadata = codebase.get_declaring_method(class.as_bytes(), method)?;
    let declaring = codebase.get_declaring_method_class(class.as_bytes(), method)?;
    Some(Callee {
        metadata,
        class: Some(declaring.to_string()),
        name: method.to_vec(),
    })
}

/// Whether the callee's return type, as `resolveMethodCallReturnType` reads it, is an `ObjectType`: one class, an
/// enum or a closure; `static` only of a class other than the calling one.
fn returns_object(facts: &Facts, scope: Option<&ClassScope>, callee: &Callee<'_>) -> bool {
    let native = match &callee.class {
        Some(class) => is_builtin_class(class.as_bytes()),
        None => is_builtin_function(&callee.name),
    };
    let declared = if native {
        documented_return(facts.documented(), callee)
    } else {
        (callee.metadata.return_type_declaration_metadata.as_ref()).map(|t| &t.type_union)
    };
    let Some(t) = declared else {
        return false;
    };
    match t.types.as_ref() {
        [TAtomic::Object(TObject::Named(named))] if named.intersection_types.is_none() => {
            if !(named.is_static || named.is_this) {
                return true;
            }
            // `normalizeStaticType`: `static` of another class is that class
            let current = scope.and_then(|s| s.name.clone());
            !(callee.class.as_ref())
                .is_some_and(|d| current.is_some_and(|c| c.eq_ignore_ascii_case(d)))
        }
        [TAtomic::Object(TObject::Enum(_))] => true,
        [TAtomic::Callable(c)] => c.is_closure(),
        _ => false,
    }
}

/// The return type PHP's stubs give a built-in function or method (with the docblock).
fn documented_return<'c>(
    codebase: &'c CodebaseMetadata,
    callee: &Callee<'_>,
) -> Option<&'c mago_codex::ttype::union::TUnion> {
    let found = match &callee.class {
        Some(class) => codebase.get_method(class.as_bytes(), &callee.name),
        None => codebase.get_function(&callee.name),
    }?;
    (found.return_type_metadata.as_ref())
        .or(found.return_type_declaration_metadata.as_ref())
        .map(|t| &t.type_union)
}
