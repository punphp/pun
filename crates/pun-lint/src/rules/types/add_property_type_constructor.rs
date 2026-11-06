//! `lint/types/add-property-type-constructor`, ported from Rector 2.6.1's `TypedPropertyFromStrictConstructorRector`
//! (rules/TypeDeclaration/Rector/Property) with `TrustedClassMethodPropertyTypeInferer`: an untyped property the
//! constructor assigns gets the type of what it assigns.
//!
//! The constructor's first assignment of a property named so whose value names a typed parameter (by
//! `NodeNameResolver::getName`, so `$this->a = $this->b` and `$this->a = b()` name a parameter `$b` too) gives that
//! parameter's declared type, `null` added for a `?T` or a `null` default; an `array` or variadic parameter takes the
//! type of the first variable named as the property. Without one, the native types of the other values assigned in
//! the constructor make a union. When every assignment in the class (`AssignToPropertyTypeInferer`) makes a union
//! that differs, the property is left alone.
//!
//! A private or protected property gets the type, loses its default when the constructor always assigns it, and loses
//! its `@var` when that says no more; a public one only gets a `@var` when it has none (or `@var mixed`). Properties a
//! used trait declares or writes, whose declared type is `null`, `callable` or a class that does not exist, or that a
//! parent class declares untyped, are left alone, as is a class with a Doctrine `loadMetadata()` method.

use mago_syntax::cst::{
    Access, Block, ClassLikeMemberSelector, Expression, FunctionLikeParameter, Method, MethodBody,
    Node, NodeKind, Variable,
};
use pun_problem::{Edit, Safety, Span};

use super::facts::{Facts, facts};
use super::property_type::{
    Assigned, ClassView, Prop, TYPED_PROPERTIES, Ty, VarTag, are_equal, assigned_native_ty,
    assigned_type, conflicts_with_default, declared_ty, default_span, default_ty, doc_text,
    doc_type, hint_ty, is_doctrine_collection, is_legal, is_null_literal, is_var_tag_useless,
    mixed_passed_or_union, parent_allows, property_declaration, union, variable_span,
};
use crate::rules::no_redundant_property_default::is_property_assigned;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddPropertyTypeConstructor;

impl Rule for AddPropertyTypeConstructor {
    fn id(&self) -> &'static str {
        "lint/types/add-property-type-constructor"
    }

    fn min_php_version(&self) -> u32 {
        TYPED_PROPERTIES
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
        let Some(view) = ClassView::of(&facts, node, ancestors) else {
            return;
        };
        if view.method(b"loadMetadata").is_some() {
            return;
        }
        let Some(constructor) = view.method(b"__construct") else {
            return;
        };
        let props = view.properties();
        if props.is_empty() {
            return;
        }
        let test_case = is_test_case(&facts, &view);
        let mut edits: Vec<Edit> = Vec::new();
        for prop in &props {
            if !is_legal(&facts, &view, prop, true) || !parent_allows(&facts, &view, prop.name()) {
                continue;
            }
            let t = infer(&facts, &view, prop, constructor, ctx, test_case);
            if matches!(t, Ty::Mixed) || is_doctrine_collection(&facts, &t) {
                continue;
            }
            let var = VarTag::of(ctx, prop.property, &view.path);
            if prop.is_public() {
                let mixed = match &var.tag {
                    Ok(None) => true,
                    Ok(Some(tag)) => {
                        matches!(tag.ty, crate::rules::phpunit::phpdoc_type::TagType::Name(n) if n.eq_ignore_ascii_case("mixed"))
                    }
                    Err(()) => false,
                };
                if !mixed {
                    continue;
                }
                let Some(text) = doc_text(&t, &facts.codebase) else {
                    continue;
                };
                if let Some(e) = var.set(ctx.source, &text) {
                    edits.extend(e);
                }
                continue;
            }
            let Some(declaration) = property_declaration(&t, &facts.codebase, ctx.php_version)
            else {
                continue;
            };
            let name = prop.name();
            let mut removes_default = false;
            if is_property_assigned(ctx, &view, test_case, name)
                && let Some(span) = default_span(prop)
            {
                edits.push(Edit::new(span, ""));
                removes_default = true;
            }
            if !removes_default
                && let Some(default) = prop.default()
                && conflicts_with_default(&default_ty(&facts, default), &t)
            {
                continue;
            }
            let at = variable_span(prop).start;
            edits.push(Edit::new(Span::new(at, at), format!("{declaration} ")));
            if let Ok(Some(tag)) = &var.tag
                && is_var_tag_useless(
                    tag,
                    doc_type(&facts, &view, name).as_ref(),
                    &declared_ty(&t),
                )
                && let Some(e) = var.remove(ctx.source)
            {
                edits.extend(e);
            }
        }
        let Some(first) = edits.first() else { return };
        out.push(Report::fixed(
            first.span,
            "Type the properties the constructor assigns.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// Whether the class is a PHPUnit test case (`isObjectType($class, 'PHPUnit\Framework\TestCase')`).
pub(crate) fn is_test_case(facts: &Facts, view: &ClassView<'_, '_>) -> bool {
    const TEST_CASE: &[u8] = b"phpunit\\framework\\testcase";
    view.parent
        .as_ref()
        .is_some_and(|p| p.as_bytes().eq_ignore_ascii_case(TEST_CASE))
        || facts
            .codebase
            .get_class_like(view.name.as_bytes())
            .is_some_and(|c| {
                c.all_parent_classes
                    .iter()
                    .any(|p| p.as_bytes().eq_ignore_ascii_case(TEST_CASE))
            })
}

/// `TrustedClassMethodPropertyTypeInferer::inferProperty`, of the assignments in `constructor` (the method passed in).
pub(crate) fn infer(
    facts: &Facts,
    view: &ClassView<'_, '_>,
    prop: &Prop<'_, '_>,
    constructor: &Method<'_>,
    ctx: &Context<'_>,
    test_case: bool,
) -> Ty {
    let MethodBody::Concrete(body) = &constructor.body else {
        return Ty::Mixed;
    };
    let name = prop.name();
    let assigns = constructor_assigns(body, name);
    let params: Vec<&FunctionLikeParameter<'_>> =
        constructor.parameter_list.parameters.iter().collect();
    let param_named = |n: &[u8]| params.iter().copied().find(|p| &p.variable.name[1..] == n);
    // 1. the first assignment whose value is not a call names a parameter
    let first = assigns.iter().find(|v| {
        !matches!(
            unparenthesize(v),
            Expression::Call(
                mago_syntax::cst::Call::Method(_) | mago_syntax::cst::Call::StaticMethod(_)
            )
        )
    });
    let resolved = match first
        .and_then(|v| get_name(v))
        .and_then(|n| param_named(&n))
    {
        Some(param) => {
            let Some(hint) = &param.hint else {
                return Ty::Mixed;
            };
            // resolveParamTypeToPHPStanType puts `null` before the type of a `?T`
            let t = match hint {
                mago_syntax::cst::Hint::Nullable(n) => {
                    mixed_passed_or_union(vec![Ty::Null, hint_ty(n.hint, &view.path, view)], false)
                }
                _ => hint_ty(hint, &view.path, view),
            };
            if matches!(t, Ty::Mixed) {
                return Ty::Mixed;
            }
            let mut types = Vec::new();
            if t.members()
                .iter()
                .all(|m| matches!(m, Ty::Array(..) | Ty::ConstArray(_)))
                || param.ellipsis.is_some()
            {
                types.push(first_variable_type(facts, body, name));
            } else {
                types.push(t);
            }
            let nullable = matches!(hint, mago_syntax::cst::Hint::Nullable(_))
                || param
                    .default_value
                    .as_ref()
                    .is_some_and(|d| is_null_literal(d.value));
            if nullable {
                types.push(Ty::Null);
            }
            mixed_passed_or_union(types, false)
        }
        None => {
            // 2. the other values assigned
            let param_names: Vec<&[u8]> = params.iter().map(|p| &p.variable.name[1..]).collect();
            let types: Vec<Ty> = assigns
                .iter()
                .filter(|v| get_name(v).is_none_or(|n| !param_names.contains(&n.as_slice())))
                .map(|v| assigned_native_ty(facts, view, v))
                .collect();
            if types.is_empty() {
                return Ty::Mixed;
            }
            union(types)
        }
    };
    // resolveType
    let assigned = is_property_assigned(ctx, view, test_case, name);
    match assigned_type(facts, view, prop, assigned) {
        Assigned::Type(exact) if exact.is_union() && !are_equal(&resolved, &exact) => Ty::Mixed,
        _ => resolved,
    }
}

/// The values of the constructor's assignments to a property named `name` (`$x->name`, `X::$name`), outside nested
/// classes, in source order.
fn constructor_assigns<'a>(body: &'a Block<'a>, name: &[u8]) -> Vec<&'a Expression<'a>> {
    let mut out = Vec::new();
    let mut stack: Vec<Node<'a, 'a>> = body.statements.iter().rev().map(Node::Statement).collect();
    while let Some(node) = stack.pop() {
        if matches!(node, Node::Class(_) | Node::AnonymousClass(_)) {
            continue;
        }
        if let Node::Assignment(a) = node
            && a.operator.is_assign()
            && !super::property_type::is_by_ref(a)
            && fetch_name(a.lhs).is_some_and(|n| n == name)
        {
            out.push(a.rhs);
        }
        let mut children = Vec::new();
        node.visit_children(|c| children.push(c));
        stack.extend(children.into_iter().rev());
    }
    out
}

/// The name of a property fetch (`$x->name`, `X::$name`).
fn fetch_name<'a>(expr: &'a Expression<'a>) -> Option<&'a [u8]> {
    match expr {
        Expression::Access(Access::Property(p)) => match &p.property {
            ClassLikeMemberSelector::Identifier(id) => Some(id.value),
            _ => None,
        },
        Expression::Access(Access::StaticProperty(p)) => match &p.property {
            Variable::Direct(v) => Some(&v.name[1..]),
            _ => None,
        },
        _ => None,
    }
}

/// `NodeNameResolver::getName` of an assigned value, as far as it can name a parameter: a variable, a property fetch,
/// a function call, a constant.
fn get_name(expr: &Expression<'_>) -> Option<Vec<u8>> {
    match unparenthesize(expr) {
        Expression::Variable(Variable::Direct(v)) => Some(v.name[1..].to_vec()),
        e @ Expression::Access(Access::Property(_) | Access::StaticProperty(_)) => {
            fetch_name(e).map(<[u8]>::to_vec)
        }
        Expression::Call(mago_syntax::cst::Call::Function(f)) => match f.function {
            Expression::Identifier(id) => Some(
                id.value()
                    .strip_prefix(b"\\")
                    .unwrap_or(id.value())
                    .to_vec(),
            ),
            _ => None,
        },
        Expression::ConstantAccess(c) => Some(c.name.value().to_vec()),
        _ => None,
    }
}

/// `getResolveParamStaticTypeAsPHPStanType`: the type of the first variable named `name` in the constructor's body,
/// or `array`.
fn first_variable_type(facts: &Facts, body: &Block<'_>, name: &[u8]) -> Ty {
    let mut stack: Vec<Node<'_, '_>> = body.statements.iter().rev().map(Node::Statement).collect();
    while let Some(node) = stack.pop() {
        if let Node::DirectVariable(v) = node
            && &v.name[1..] == name
        {
            let span = node_span(v);
            return (facts.full_type(span).or_else(|| facts.native_type(span)))
                .filter(|t| !t.types.is_empty())
                .map_or(Ty::Mixed, super::property_type::ty);
        }
        let mut children = Vec::new();
        node.visit_children(|c| children.push(c));
        stack.extend(children.into_iter().rev());
    }
    Ty::Array(Box::new(Ty::Mixed), Box::new(Ty::Mixed))
}
