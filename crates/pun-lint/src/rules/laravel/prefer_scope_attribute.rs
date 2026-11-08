//! `lint/laravel-12/prefer-scope-attribute`, ported from rector-laravel 2.7.3's
//! `ScopeNamedClassMethodToScopeAttributedClassMethodRector` (src/Rector/ClassMethod) and
//! `ScopeAnalyzer::isNamedScope` (src/NodeAnalyzer).
//!
//! In a model class (an anonymous one too), each non-abstract method named `scope` + an uppercase letter + more, whose first parameter
//! is untyped or an `Illuminate\Database\Eloquent\Builder`, loses the `scope` prefix (the next letter lowercased),
//! becomes `protected` (every other modifier goes) and gets `#[\Illuminate\Database\Eloquent\Attributes\Scope]`;
//! unless the class (its parents, traits and `@method` tags included) has a method of the new name already, a parent
//! class or interface declares the method (`ParentClassMethodTypeOverrideGuard::hasParentClassMethod`), or the method
//! has the attribute.

use mago_syntax::cst::{ClassLikeMember, Hint, Method, MethodBody, Modifier, Node, NodeKind};
use pun_problem::{Edit, Span};

use super::property_attribute::{ClassView, MODEL, report};
use crate::rules::rector_batch5::{has_attribute, resolve_class_name};
use crate::rules::rector_stmts::line_indent;
use crate::rules::types::compare::mapper::flatten;
use crate::rules::types::facts::facts;
use crate::rules::types::{class_scope, has_parent_class_method};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const SCOPE: &str = "Illuminate\\Database\\Eloquent\\Attributes\\Scope";
const ELOQUENT_BUILDER: &str = "Illuminate\\Database\\Eloquent\\Builder";

pub struct PreferScopeAttribute;

impl Rule for PreferScopeAttribute {
    fn id(&self) -> &'static str {
        "lint/laravel-12/prefer-scope-attribute"
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
        let Some(class) = ClassView::of(node) else {
            return;
        };
        if !class.is_object_type(ctx, ancestors, MODEL) {
            return;
        }
        let class_name = class.name(ctx, ancestors);
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let mut edits = Vec::new();
        for member in class.members {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            if matches!(method.body, MethodBody::Abstract(_))
                || !is_named_scope(ctx, &scope, method)
            {
                continue;
            }
            let name = String::from_utf8_lossy(method.name.value);
            let mut new_name = name[5..].to_string();
            new_name[..1].make_ascii_lowercase();
            let private = (method.modifiers.iter()).any(|m| matches!(m, Modifier::Private(_)));
            if has_method(ctx, &class_name, &new_name)
                || (!private
                    && class_scope(&scope)
                        .is_none_or(|c| has_parent_class_method(&facts(ctx), &c, method.name.value)))
                || has_attribute(&scope, &method.attribute_lists, SCOPE)
            {
                continue;
            }
            let Some(method_edits) = method_edits(ctx.source, method, &new_name) else {
                continue;
            };
            edits.extend(method_edits);
        }
        if !edits.is_empty() {
            out.push(report(
                &class,
                "Changes model scope methods to use the scope attribute.",
                edits,
            ));
        }
    }
}

/// `ScopeAnalyzer::isNamedScope`: the name matches `^scope[A-Z].+$`, and the first parameter is untyped or a
/// `Builder` (`isObjectType` on the type node: a class name, `?Name`, or a union of such names with `null`).
fn is_named_scope(ctx: &Context<'_>, scope: &[Node<'_, '_>], method: &Method<'_>) -> bool {
    let name = method.name.value;
    if !(name.len() > 6 && name.starts_with(b"scope") && name[5].is_ascii_uppercase()) {
        return false;
    }
    let Some(first) = method.parameter_list.parameters.first() else {
        return false;
    };
    match &first.hint {
        None => true,
        Some(hint) => is_builder(ctx, scope, hint, true),
    }
}

fn is_builder(ctx: &Context<'_>, scope: &[Node<'_, '_>], hint: &Hint<'_>, top: bool) -> bool {
    let builder = |id| {
        let name = resolve_class_name(scope, id);
        facts(ctx)
            .codebase
            .get_class_like(ELOQUENT_BUILDER.as_bytes())
            .is_some()
            && super::is_a(ctx, &name, &[ELOQUENT_BUILDER])
    };
    match hint {
        Hint::Identifier(id) => builder(id),
        Hint::Nullable(n) => top && matches!(n.hint, Hint::Identifier(id) if builder(id)),
        Hint::Union(_) if top => {
            let mut members = Vec::new();
            flatten(hint, true, &mut members);
            let classes: Vec<_> = members
                .iter()
                .filter(|h| !matches!(h, Hint::Null(_)))
                .collect();
            !classes.is_empty() && classes.iter().all(|h| is_builder(ctx, scope, h, false))
        }
        _ => false,
    }
}

/// `ClassReflection::hasMethod`: the class, its parents, interfaces and traits declare the method, or a `@method`
/// tag does (ASCII case-insensitive).
fn has_method(ctx: &Context<'_>, class: &str, method: &str) -> bool {
    let facts = facts(ctx);
    let codebase = facts.documented();
    let same = |m: &[u8]| m.eq_ignore_ascii_case(method.as_bytes());
    // the class is in the codebase: `is_object_type` found it there
    let own = codebase.get_class_like(class.as_bytes()).unwrap();
    std::iter::once(&own.name)
        .chain(own.all_parent_classes.iter())
        .chain(own.all_parent_interfaces.iter())
        .chain(own.used_traits.iter())
        .filter_map(|name| codebase.get_class_like(name.as_bytes()))
        .any(|c| {
            c.declaring_method_ids.keys().any(|m| same(m.as_bytes()))
                || c.pseudo_methods.iter().any(|m| same(m.as_bytes()))
                || c.static_pseudo_methods.iter().any(|m| same(m.as_bytes()))
        })
}

/// The method with the new name, `protected` alone as its modifiers (`$classMethod->flags = Modifiers::PROTECTED`),
/// and the attribute group after its attributes, on a line of its own; `None` for a tab in its indentation.
fn method_edits(source: &str, method: &Method<'_>, new_name: &str) -> Option<Vec<Edit>> {
    let start = node_span(method).start;
    let indent = line_indent(source, start);
    if indent.contains('\t') {
        return None;
    }
    let group = format!("#[\\{SCOPE}]");
    let function = node_span(&method.function).start;
    let modifiers_start = method
        .modifiers
        .first()
        .map_or(function, |m| node_span(m).start);
    let mut edits = Vec::new();
    let mut lead = String::new();
    match method.attribute_lists.last() {
        Some(last) => {
            let end = node_span(last).end;
            edits.push(Edit::new(Span::new(end, end), format!("\n{indent}{group}")));
        }
        None => lead = format!("{group}\n{indent}"),
    }
    edits.push(Edit::new(
        Span::new(modifiers_start, function),
        format!("{lead}protected "),
    ));
    edits.push(Edit::new(node_span(&method.name), new_name.to_string()));
    Some(edits)
}
