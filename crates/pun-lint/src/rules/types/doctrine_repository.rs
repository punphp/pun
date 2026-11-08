//! `lint/types/add-property-type-get-repository`, `lint/phpunit/add-property-type-container-get` and
//! `lint/types/add-doctrine-repository-return-type`, ported from Rector 2.8.4's
//! `TypedPropertyFromGetRepositorySetUpRector`, `TypedPropertyFromContainerGetSetUpRector` and
//! `ChildDoctrineRepositoryClassTypeRector` (rules/TypeDeclaration/Rector/Class_).
//!
//! In a PHPUnit test case with a `setUp()` method, an untyped private non-static property that `setUp()` assigns a
//! `->getRepository(...)` call (or, for the container rule, a `->get(...)` call on `static::getContainer()`,
//! `$this->getContainer()`, `$this->container` or `$this`) gets the class its `@var` names, when that class exists;
//! the `@var` goes.
//!
//! In a class that extends `Doctrine\ORM\EntityRepository` and whose doc says `@extends EntityRepository<Entity>` (the
//! entity one name that is no template of the class), each public non-static method without a return type gets
//! `?Entity` when it calls `->getOneOrNullResult()` or `->findOneBy()`, and `array` with a new `@return Entity[]` when it
//! calls `->findBy()`; the entity is written as the doc writes it.

use crate::syntax::is_this;
use mago_syntax::cst::{
    Call, ClassLikeMember, ClassLikeMemberSelector, Expression, Method, MethodBody, Modifier, Node,
    NodeKind, Variable,
};
use pun_problem::{Edit, Safety, Span};

use super::add_property_type_constructor::is_test_case;
use super::facts::facts;
use super::property_type::{ClassView, TYPED_PROPERTIES, VarTag, variable_span};
use crate::rules::phpunit::annotations::Annotated;
use crate::rules::phpunit::phpdoc_type::{TagType, is_name};
use crate::rules::rector_batch5::imported;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddPropertyTypeGetRepository;

pub struct AddPropertyTypeContainerGet;

pub struct AddDoctrineRepositoryReturnType;

impl Rule for AddPropertyTypeGetRepository {
    fn id(&self) -> &'static str {
        "lint/types/add-property-type-get-repository"
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
        type_from_var(
            node,
            ancestors,
            ctx,
            out,
            assigned_via_get_repository,
            "Type the repositories `setUp()` assigns from their `@var`.",
        );
    }
}

impl Rule for AddPropertyTypeContainerGet {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-property-type-container-get"
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
        type_from_var(
            node,
            ancestors,
            ctx,
            out,
            assigned_via_container_get,
            "Type the services `setUp()` fetches from the container from their `@var`.",
        );
    }
}

/// The shared `refactor()`: in a test case's `setUp()`, the untyped private non-static properties that `assigned`
/// accepts get the existing class their `@var` names, and the `@var` goes.
fn type_from_var<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    ancestors: &[Node<'ast, 'arena>],
    ctx: &Context<'_>,
    out: &mut Vec<Report>,
    assigned: fn(&Method<'_>, &[u8]) -> bool,
    message: &str,
) {
    let facts = facts(ctx);
    let view = ClassView::of(&facts, node, ancestors).expect("the rules target classes");
    if !is_test_case(&facts, &view) {
        return;
    }
    let Some(set_up) = view.method(b"setUp") else {
        return;
    };
    let mut edits: Vec<Edit> = Vec::new();
    for prop in &view.properties() {
        if prop.item.is_none()
            || !prop.is_private()
            || prop
                .modifiers()
                .iter()
                .any(|m| matches!(m, Modifier::Static(_)))
            || !assigned(set_up, prop.name())
        {
            continue;
        }
        let var = VarTag::of(ctx, prop.property, &view.path);
        if var.annotated.doc.is_none() {
            continue;
        }
        let Ok(Some(tag)) = &var.tag else { continue };
        let Some(written) = object_name(&tag.ty) else {
            continue;
        };
        let class = doc_class(&view.path, written);
        // must be an existing object type
        let Some(removed) = facts
            .codebase
            .get_class_like(class.as_bytes())
            .and(var.remove(ctx.source))
        else {
            continue;
        };
        let at = variable_span(prop).start;
        edits.push(Edit::new(Span::new(at, at), format!("\\{class} ")));
        edits.extend(removed);
    }
    let Some(first) = edits.first() else { return };
    out.push(Report::fixed(
        first.span,
        message.to_string(),
        Safety::Unsafe,
        edits,
    ));
}

/// The class a name written in a doc names, resolved against the file's namespace and imports (`path` ends at the
/// class).
pub(crate) fn doc_class(path: &[Node<'_, '_>], written: &str) -> String {
    match written.strip_prefix('\\') {
        Some(fq) => fq.to_string(),
        None => imported(path, written).unwrap_or_else(|| match super::namespace_of(path) {
            Some(ns) => format!("{ns}\\{written}"),
            None => written.to_string(),
        }),
    }
}

/// The class a `@var` names, as `getVarType()` reads an `ObjectType` there: one name that is no keyword, or a generic
/// of one. `self` is skipped (`Name('self')`, which `hasClass()` does not find).
pub(crate) fn object_name<'s>(ty: &TagType<'s>) -> Option<&'s str> {
    let name = match *ty {
        TagType::Name(n) => n,
        TagType::Other(o) => match o.split_once('<') {
            Some((n, _)) if is_name(n) && o.ends_with('>') => n,
            _ => return None,
        },
        _ => return None,
    };
    let keyword = matches!(
        name.to_ascii_lowercase().as_str(),
        "int"
            | "integer"
            | "string"
            | "bool"
            | "boolean"
            | "float"
            | "double"
            | "real"
            | "array"
            | "iterable"
            | "callable"
            | "callback"
            | "object"
            | "mixed"
            | "void"
            | "null"
            | "true"
            | "false"
            | "resource"
            | "never"
            | "self"
            | "static"
            | "parent"
            | "scalar"
            | "number"
            | "numeric"
            | "list"
            | "array-key"
            | "class-string"
    );
    (!keyword).then_some(name)
}

/// `isAssignedViaGetRepositoryInSetUp`: an assignment anywhere in `setUp()` of `$this->name = $x->getRepository(...)`.
fn assigned_via_get_repository(set_up: &Method<'_>, name: &[u8]) -> bool {
    let mut stack = vec![Node::Method(set_up)];
    while let Some(node) = stack.pop() {
        if let Node::Assignment(a) = node
            && a.operator.is_assign()
            && let Expression::Access(mago_syntax::cst::Access::Property(fetch)) =
                unparenthesize(a.lhs)
            && matches!(unparenthesize(fetch.object), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
            && matches!(&fetch.property, ClassLikeMemberSelector::Identifier(id) if id.value == name)
            && matches!(unparenthesize(a.rhs), Expression::Call(Call::Method(call))
                if matches!(&call.method, ClassLikeMemberSelector::Identifier(m)
                    if m.value.eq_ignore_ascii_case(b"getRepository")))
        {
            return true;
        }
        node.visit_children(|c| stack.push(c));
    }
    false
}

/// `isAssignedViaContainerGetInSetUp`: an assignment anywhere in `setUp()` of `$this->name = <container>->get(...)`,
/// names compared as `isName` does, without letter case.
fn assigned_via_container_get(set_up: &Method<'_>, name: &[u8]) -> bool {
    let mut stack = vec![Node::Method(set_up)];
    while let Some(node) = stack.pop() {
        if let Node::Assignment(a) = node
            && a.operator.is_assign()
            && let Expression::Access(mago_syntax::cst::Access::Property(fetch)) =
                unparenthesize(a.lhs)
            && is_this(fetch.object)
            && is_selector(&fetch.property, name)
            && is_container_get(a.rhs)
        {
            return true;
        }
        node.visit_children(|c| stack.push(c));
    }
    false
}

/// `isContainerGetCall`: `->get(...)` on `X::getContainer()`, `$x->getContainer()`, `$x->container` or `$this`.
fn is_container_get(expr: &Expression<'_>) -> bool {
    let Expression::Call(Call::Method(call)) = unparenthesize(expr) else {
        return false;
    };
    if !is_selector(&call.method, b"get") {
        return false;
    }
    match unparenthesize(call.object) {
        Expression::Call(Call::StaticMethod(c)) => is_selector(&c.method, b"getContainer"),
        Expression::Call(Call::Method(c)) => is_selector(&c.method, b"getContainer"),
        Expression::Access(mago_syntax::cst::Access::Property(p)) => {
            is_selector(&p.property, b"container")
        }
        caller => is_this(caller),
    }
}

/// `isName` on a member name written as an identifier.
fn is_selector(selector: &ClassLikeMemberSelector<'_>, name: &[u8]) -> bool {
    matches!(selector, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
}

impl Rule for AddDoctrineRepositoryReturnType {
    fn id(&self) -> &'static str {
        "lint/types/add-doctrine-repository-return-type"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        const ENTITY_REPOSITORY: &[u8] = b"doctrine\\orm\\entityrepository";
        let facts = facts(ctx);
        let Some(view) = ClassView::of(&facts, node, ancestors) else {
            return;
        };
        let is_repository = view.name.as_bytes().eq_ignore_ascii_case(ENTITY_REPOSITORY)
            || facts
                .codebase
                .get_class_like(view.name.as_bytes())
                .is_some_and(|c| {
                    c.all_parent_classes
                        .iter()
                        .any(|p| p.as_bytes().eq_ignore_ascii_case(ENTITY_REPOSITORY))
                });
        if !is_repository {
            return;
        }
        let Some(entity) = entity_from_doc(ctx, node, ancestors) else {
            return;
        };
        let mut edits: Vec<Edit> = Vec::new();
        for member in view.members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            // shouldSkipClassMethod
            let public = !method
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Private(_) | Modifier::Protected(_)));
            let is_static = method
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Static(_)));
            if !public || is_static || method.return_type_hint.is_some() {
                continue;
            }
            let return_type = if calls(method, b"findBy") {
                let annotated = Annotated::of(ctx, Node::Method(method), &view.path);
                let tag = format!("@return {entity}[]");
                let new_doc = match annotated.as_ref().and_then(|a| a.doc.as_ref()) {
                    Some(doc) => doc.print_appending(&vec![false; doc.len()], &[&tag]),
                    None => format!("/**\n * {tag}\n */"),
                };
                let Some(doc_edits) =
                    annotated.and_then(|a| a.edits(ctx.source, Some(&new_doc), &[]))
                else {
                    continue;
                };
                edits.extend(doc_edits);
                "array".to_string()
            } else if calls(method, b"getOneOrNullResult") || calls(method, b"findOneBy") {
                format!("?{entity}")
            } else {
                continue;
            };
            let at = node_span(&method.parameter_list).end;
            edits.push(Edit::new(Span::new(at, at), format!(": {return_type}")));
        }
        let Some(first) = edits.first() else { return };
        out.push(Report::fixed(
            first.span,
            "Type the repository's finder methods with its entity.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// `resolveEntityClassnameFromPhpDoc`: the entity of the class doc's first `@extends EntityRepository<Entity, ...>`,
/// when it is one name that no `@template` of the doc declares.
fn entity_from_doc(ctx: &crate::Context<'_>, node: Node<'_, '_>, ancestors: &[Node<'_, '_>]) -> Option<String> {
    let doc = Annotated::of(ctx, node, ancestors)?.doc?;
    let mut extends: Option<&str> = None;
    let mut templates: Vec<&str> = Vec::new();
    for index in 0..doc.len() {
        let text = doc.text(index).trim();
        let mut words = text.split_whitespace();
        let Some(tag) = words.next() else { continue };
        if tag == "@extends" {
            extends = extends.or(Some(text["@extends".len()..].trim_start()));
        } else if ["@template", "@phpstan-template", "@psalm-template"]
            .iter()
            .any(|t| tag.starts_with(t))
            && let Some(name) = words.next()
        {
            templates.push(name);
        }
    }
    let rest = extends?.strip_prefix("EntityRepository<")?;
    let end = rest.find(|c: char| c == ',' || c == '>')?;
    let entity = rest[..end].trim();
    (!entity.is_empty() && is_name(entity) && !templates.contains(&entity))
        .then(|| entity.to_string())
}

/// `containsMethodCallNamed`: a `->name(...)` call anywhere in the method's statements.
fn calls(method: &Method<'_>, name: &[u8]) -> bool {
    let MethodBody::Concrete(body) = &method.body else {
        return false;
    };
    let mut stack: Vec<Node<'_, '_>> = body.statements.iter().map(Node::Statement).collect();
    while let Some(node) = stack.pop() {
        if let Node::MethodCall(call) = node
            && matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value == name)
        {
            return true;
        }
        node.visit_children(|c| stack.push(c));
    }
    false
}
