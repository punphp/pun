//! `lint/classes/add-dynamic-properties`, ported from Rector 2.6.1's `CompleteDynamicPropertiesRector`
//! (rules/CodeQuality/Rector/Class_) with `LocalPropertyAnalyzer`, `MissingPropertiesResolver` and `ClassLikeAnalyzer`
//! (rules/CodeQuality/NodeAnalyzer), `MissingPropertiesFactory` and `PropertyTypeDecorator`
//! (rules/CodeQuality/NodeFactory), `ArrayDimFetchTypeResolver` and `PropertyPresenceChecker` (src/NodeAnalyzer).
//!
//! A property a class's methods write or read on `$this` (or on a value of the class) that neither the class, its
//! ancestors, its traits nor its PHPDoc (`@property`) declares gets declared at the top of the class. One written only
//! in `__construct` or `setUp` is `private` and typed from PHP 7.4 when its type has a declaration
//! (`StaticTypeMapper` for `TypeKind::PROPERTY`); any other is `public` with a `@var` of its type, constants
//! generalized (`TypeNormalizer::generalizeConstantTypes`). A class that is abstract, has `__get` or `__set`, has
//! `#[AllowDynamicProperties]` or extends a class that does not exist is left alone.
//!
//! The types are PHPStan's `getType()` of each assigned value ([`expr_type`]), with Rector's
//! `GenericClassStringTypeCorrector`: a string naming a class as declared is a `class-string` of it, and `X::class` of
//! a class that does not exist is a string. A class that does not exist prints without its leading `\`, as Rector's
//! `NonExistingObjectType`.
//!
//! php-parser's printer puts the new properties before the first member and its comments, the first at that place, the
//! next ones and the old first member each on a new line at the indentation of that place's line (a tab counted as
//! four spaces).

use mago_syntax::cst::{
    Access, AssignmentOperator, Class, ClassLikeMember, ClassLikeMemberSelector, Expression,
    MethodBody, Modifier, Node, NodeKind, Property, Variable,
};
use pun_phpstan::types::Type;
use pun_problem::{Edit, Fix, Safety, Span};

use super::phpdoc_writer::expr_type;
use super::phpdoc_writer::type_mapper::{
    create_mixed_passed_or_union_type, generalize_constant_types, is_mixed, map_to_phpdoc,
};
use super::rector_batch5::has_attribute;
use super::types::facts::{Facts, facts};
use super::types::names::resolve_class_name;
use super::types::namespace_of;
use super::types::property_type::{self, TYPED_PROPERTIES, Ty, property_declaration};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddDynamicProperties;

impl Rule for AddDynamicProperties {
    fn id(&self) -> &'static str {
        "lint/classes/add-dynamic-properties"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        // (an anonymous class is skipped: `ClassAnalyzer::isAnonymousClass`)
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Class(class) = node else {
            return;
        };
        let facts = facts(ctx);
        let name = class_name(ancestors, class);
        if should_skip_class(&facts, ancestors, class, &name) {
            return;
        }
        let mut path = ancestors.to_vec();
        path.push(node);
        let fetched = resolve_fetched_properties(ctx, &facts, &path, class, &name);
        let missing = resolve_missing(&facts, class, &name, fetched);
        if missing.is_empty() {
            return;
        }
        let Some(first) = class.members.first() else {
            return;
        };
        let at = with_comments_start(
            ctx.source,
            class.left_brace.end.offset as usize,
            node_span(first).start,
        );
        let indent = " ".repeat(indent_of(ctx.source, at));
        let mut text = String::new();
        for property in &missing {
            text.push_str(&create(ctx, &facts, property, &indent));
            text.push('\n');
            text.push_str(&indent);
        }
        out.push(Report {
            span: node_span(&class.name),
            message: "Declare the properties this class sets dynamically.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(Span::new(at, at), text)],
            }),
        });
    }
}

/// The class's name as the codebase knows it.
fn class_name(ancestors: &[Node<'_, '_>], class: &Class<'_>) -> String {
    let short = String::from_utf8_lossy(class.name.value);
    match namespace_of(ancestors) {
        Some(ns) => format!("{ns}\\{short}"),
        None => short.into_owned(),
    }
}

/// `shouldSkipClass` (with `matchClassReflection`).
fn should_skip_class(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    class: &Class<'_>,
    name: &str,
) -> bool {
    if class
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Abstract(_)))
    {
        return true;
    }
    let codebase = &facts.codebase;
    if codebase.get_class_like(name.as_bytes()).is_none() {
        return true;
    }
    if has_attribute(ancestors, &class.attribute_lists, "AllowDynamicProperties") {
        return true;
    }
    if [b"__set".as_slice(), b"__get"]
        .iter()
        .any(|m| codebase.get_declaring_method(name.as_bytes(), m).is_some())
    {
        return true;
    }
    (class.extends.as_ref().and_then(|e| e.types.first())).is_some_and(|parent| {
        codebase
            .get_class_like(resolve_class_name(ancestors, parent).as_bytes())
            .is_none()
    })
}

/// `DefinedPropertyWithType`: a property's name, its type in both of Pun's models, and the method that defines it
/// (`None` once several places do).
struct Defined {
    name: Vec<u8>,
    r#type: Type,
    ty: Ty,
    method: Option<Vec<u8>>,
}

/// `LocalPropertyAnalyzer::resolveFetchedPropertiesToTypesFromClass`.
fn resolve_fetched_properties(
    ctx: &Context<'_>,
    facts: &Facts,
    path: &[Node<'_, '_>],
    class: &Class<'_>,
    name: &str,
) -> Vec<Defined> {
    let mut defined = Vec::new();
    for member in class.members.iter() {
        let ClassLikeMember::Method(method) = member else {
            continue;
        };
        let MethodBody::Concrete(body) = &method.body else {
            continue;
        };
        let mut ancestors = path.to_vec();
        ancestors.push(Node::ClassLikeMember(member));
        ancestors.push(Node::Method(method));
        ancestors.push(Node::MethodBody(&method.body));
        ancestors.push(Node::Block(body));
        let mut stack: Vec<(Node<'_, '_>, usize)> = body
            .statements
            .iter()
            .rev()
            .map(|s| (Node::Statement(s), ancestors.len()))
            .collect();
        let method_name = method.name.value.to_vec();
        while let Some((node, depth)) = stack.pop() {
            ancestors.truncate(depth);
            if should_skip(&ancestors, node) {
                continue;
            }
            if let Node::Assignment(a) = node
                && matches!(a.operator, AssignmentOperator::Assign(_))
                && !property_type::is_by_ref(a)
            {
                let (fetch, dim) = match unparenthesize(a.lhs) {
                    Expression::ArrayAccess(d) => (Some(d.array), Some(Some(d.index))),
                    Expression::ArrayAppend(d) => (Some(d.array), Some(None)),
                    fetch @ Expression::Access(Access::Property(_)) => (Some(fetch), None),
                    _ => (None, None),
                };
                if let Some(fetch) = fetch {
                    let mut at = ancestors.clone();
                    at.push(node);
                    // a property fetch, or an offset of one, is all `resolvePropertyName()` looks at
                    if let Some(property) =
                        resolve_property_name(facts, &at, unparenthesize(fetch), name)
                    {
                        let value = value_types(ctx, facts, &at, a.rhs);
                        let (r#type, ty) = match dim {
                            // `ArrayDimFetchTypeResolver::resolve()`
                            Some(index) => {
                                let key = index.map_or((mixed(), Ty::Mixed), |i| {
                                    value_types(ctx, facts, &at, i)
                                });
                                (
                                    Type::array(key.0, value.0),
                                    Ty::Array(Box::new(key.1), Box::new(value.1)),
                                )
                            }
                            None => value,
                        };
                        defined.push(Defined {
                            name: property,
                            r#type,
                            ty,
                            method: Some(method_name.clone()),
                        });
                    }
                    continue;
                }
            }
            if let Node::PropertyAccess(_) = node
                && let Some(property) = resolve_property_name_of_node(facts, &ancestors, node, name)
            {
                defined.push(Defined {
                    name: property,
                    r#type: mixed(),
                    ty: Ty::Mixed,
                    method: Some(method_name.clone()),
                });
            }
            ancestors.push(node);
            let mut children = Vec::new();
            node.visit_children(|c| children.push(c));
            stack.extend(children.into_iter().rev().map(|c| (c, depth + 1)));
        }
    }
    normalize_to_single_type(defined)
}

/// `getType()` of `expr` in both of Pun's models. A parameter declared with a class the codebase does not know is an
/// object of that class, as PHPStan types it from the declaration, where Mago has no type.
fn value_types(
    ctx: &Context<'_>,
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
) -> (Type, Ty) {
    if let Expression::Variable(Variable::Direct(v)) = unparenthesize(expr)
        && let Some(Node::Method(method)) = ancestors.iter().rev().find(|a| {
            matches!(
                a,
                Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_) | Node::Function(_)
            )
        })
        && let Some(mago_syntax::cst::Hint::Identifier(id)) =
            (method.parameter_list.parameters.iter())
                .find(|p| p.variable.name == v.name)
                .and_then(|p| p.hint.as_ref())
    {
        let class = resolve_class_name(ancestors, id);
        if facts.codebase.get_class_like(class.as_bytes()).is_none() {
            return (Type::Object(class.clone(), vec![]), Ty::Object(class));
        }
    }
    (
        corrected(facts, expr_type(ctx, ancestors, expr)),
        property_type::full_ty(facts, expr),
    )
}

fn mixed() -> Type {
    Type::Mixed { explicit: false }
}

/// `shouldSkip`: a nested class or function, a method called on a closure (`(function () {...})->call($o)`), and a
/// static call on Laravel's `Collection` (a macro, bound to the collection).
fn should_skip(ancestors: &[Node<'_, '_>], node: Node<'_, '_>) -> bool {
    match node {
        Node::Class(_) | Node::AnonymousClass(_) | Node::Function(_) => true,
        Node::MethodCall(c) => matches!(unparenthesize(c.object), Expression::Closure(_)),
        Node::StaticMethodCall(c) => matches!(c.class, Expression::Identifier(id)
            if resolve_class_name(ancestors, id) == "Illuminate\\Support\\Collection"),
        _ => false,
    }
}

/// `resolvePropertyName()` of a node of the walk.
fn resolve_property_name_of_node(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    node: Node<'_, '_>,
    class: &str,
) -> Option<Vec<u8>> {
    let Node::PropertyAccess(access) = node else {
        return None;
    };
    resolve_fetch(facts, ancestors, access.object, &access.property, class)
}

/// `resolvePropertyName()` of an expression: a local property fetch, outside a closure bind, by an identifier.
fn resolve_property_name(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    fetch: &Expression<'_>,
    class: &str,
) -> Option<Vec<u8>> {
    let Expression::Access(Access::Property(access)) = fetch else {
        return None;
    };
    resolve_fetch(facts, ancestors, access.object, &access.property, class)
}

fn resolve_fetch(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    object: &Expression<'_>,
    property: &ClassLikeMemberSelector<'_>,
    class: &str,
) -> Option<Vec<u8>> {
    if !is_local_property_fetch(facts, object, class) || is_in_closure_bind(ancestors) {
        return None;
    }
    match property {
        ClassLikeMemberSelector::Identifier(id) => Some(id.value.to_vec()),
        _ => None,
    }
}

/// `PropertyFetchAnalyzer::isLocalPropertyFetch` for `$object->...`: `$this`, or a value whose type is the class.
fn is_local_property_fetch(facts: &Facts, object: &Expression<'_>, class: &str) -> bool {
    let object = unparenthesize(object);
    if matches!(object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this") {
        return true;
    }
    use mago_codex::ttype::atomic::TAtomic;
    use mago_codex::ttype::atomic::object::TObject;
    (facts.full_type(node_span(object))).is_some_and(|t| {
        matches!(t.types.as_ref(), [TAtomic::Object(TObject::Named(n))]
            if n.intersection_types.is_none() && n.name.as_bytes() == class.as_bytes())
    })
}

/// `Scope::isInClosureBind()`: inside a closure passed first to `Closure::bind()`.
fn is_in_closure_bind(ancestors: &[Node<'_, '_>]) -> bool {
    ancestors.windows(2).rev().any(|pair| {
        let [Node::StaticMethodCall(call), _] = pair else {
            return false;
        };
        let bind = matches!(call.class, Expression::Identifier(id)
            if resolve_class_name(ancestors, id).eq_ignore_ascii_case("Closure"))
            && matches!(&call.method, ClassLikeMemberSelector::Identifier(m) if m.value.eq_ignore_ascii_case(b"bind"));
        bind && call.argument_list.arguments.first().is_some_and(|a| {
            let value = match a {
                mago_syntax::cst::Argument::Positional(p) => p.value,
                mago_syntax::cst::Argument::Named(n) => n.value,
            };
            matches!(unparenthesize(value), Expression::Closure(c)
                if ancestors.iter().any(|n| matches!(n, Node::Closure(d) if std::ptr::eq(*d, c))))
        })
    })
}

/// `GenericClassStringTypeCorrector::correct()` on `getType()`, and PHPStan's `X::class` of a class it does not know.
fn corrected(facts: &Facts, t: Option<Type>) -> Type {
    let Some(t) = t else {
        return mixed();
    };
    let exists = |name: &str| facts.codebase.get_class_like(name.as_bytes());
    match t {
        Type::ConstString(ref value)
            if exists(value).is_some_and(|c| c.original_name.as_bytes() == value.as_bytes()) =>
        {
            Type::ClassString(Some(Box::new(Type::Object(value.clone(), vec![]))))
        }
        Type::ClassString(Some(ref inner)) if matches!(&**inner, Type::Object(name, _) if exists(name).is_none()) => {
            Type::String
        }
        t => t,
    }
}

/// `normalizeToSingleType`: the types of a property defined more than once made one, in the order first seen.
fn normalize_to_single_type(defined: Vec<Defined>) -> Vec<Defined> {
    let mut by_name: Vec<Vec<Defined>> = Vec::new();
    for d in defined {
        match by_name.iter_mut().find(|g| g[0].name == d.name) {
            Some(group) => group.push(d),
            None => by_name.push(vec![d]),
        }
    }
    by_name
        .into_iter()
        .map(|mut group| {
            if group.len() == 1 {
                return group.pop().expect("one");
            }
            let types: Vec<Type> = group.iter().map(|d| d.r#type.clone()).collect();
            let tys: Vec<Ty> = group.iter().map(|d| d.ty.clone()).collect();
            Defined {
                name: group[0].name.clone(),
                r#type: create_mixed_passed_or_union_type(&types, false),
                ty: property_type::mixed_passed_or_union(tys, false),
                method: None,
            }
        })
        .collect()
}

/// `MissingPropertiesResolver::resolve`.
fn resolve_missing(
    facts: &Facts,
    class: &Class<'_>,
    name: &str,
    defined: Vec<Defined>,
) -> Vec<Defined> {
    // `ClassLikeAnalyzer::resolvePropertyNames`
    let existing: Vec<&[u8]> = (class.members.iter())
        .filter_map(|m| match m {
            ClassLikeMember::Property(Property::Plain(p)) => Some(
                p.items
                    .iter()
                    .map(|i| &i.variable().name[1..])
                    .collect::<Vec<_>>(),
            ),
            ClassLikeMember::Property(Property::Hooked(p)) => {
                Some(vec![&p.item.variable().name[1..]])
            }
            _ => None,
        })
        .flatten()
        .collect();
    // `PromotedPropertyResolver::resolveFromClass`
    let promoted: Vec<&[u8]> = (class.members.iter())
        .filter_map(|m| match m {
            ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
                Some(m)
            }
            _ => None,
        })
        .flat_map(|m| m.parameter_list.parameters.iter())
        .filter(|p| !p.modifiers.is_empty())
        .map(|p| &p.variable.name[1..])
        .collect();
    let documented = facts.documented();
    defined
        .into_iter()
        .filter(|d| {
            !existing.contains(&d.name.as_slice())
                // `hasInstanceProperty()`: an inherited, a trait's or a PHPDoc `@property`
                && documented.get_declaring_property(name.as_bytes(), &[b"$", d.name.as_slice()].concat()).is_none()
                && !promoted.contains(&d.name.as_slice())
        })
        .collect()
}

/// `MissingPropertiesFactory::create` for one property, printed at `indent`.
fn create(ctx: &Context<'_>, facts: &Facts, property: &Defined, indent: &str) -> String {
    let name = String::from_utf8_lossy(&property.name);
    // `isFromAlwaysDefinedMethod`
    let always = matches!(property.method.as_deref(), Some(b"__construct" | b"setUp"));
    if always
        && ctx.php_version >= TYPED_PROPERTIES
        && let Some(declaration) =
            property_declaration(&property.ty, &facts.codebase, ctx.php_version)
    {
        return format!("private {declaration} ${name};");
    }
    let visibility = if always { "private" } else { "public" };
    // `PropertyTypeDecorator::decorateProperty`, `PhpDocTypeChanger::changeVarType` on an empty doc
    let t = generalize_constant_types(&property.r#type);
    let legal =
        !is_mixed(&t) && !matches!(&t, Type::Union(members) if members.iter().any(is_mixed));
    if !legal {
        return format!("{visibility} ${name};");
    }
    let printed = without_leading_backslash_of_unknown(facts, &map_to_phpdoc(&t).to_string());
    format!("/**\n{indent} * @var {printed}\n{indent} */\n{indent}{visibility} ${name};")
}

/// The printed type with the classes the codebase does not know written without their leading `\`.
fn without_leading_backslash_of_unknown(facts: &Facts, printed: &str) -> String {
    let is_name = |c: char| c.is_alphanumeric() || c == '_' || c == '\\';
    let mut out = String::new();
    let mut rest = printed;
    while let Some(i) = rest.find('\\') {
        out.push_str(&rest[..i]);
        let tail = &rest[i + 1..];
        let end = tail.find(|c: char| !is_name(c)).unwrap_or(tail.len());
        let name = &tail[..end];
        if facts.codebase.get_class_like(name.as_bytes()).is_none() {
            out.push_str(name);
        } else {
            out.push('\\');
            out.push_str(name);
        }
        rest = &tail[end..];
    }
    out.push_str(rest);
    out
}

/// Where php-parser's first member starts: at the first comment between the class's `{` and the member, else the
/// member.
fn with_comments_start(source: &str, brace_end: usize, member: usize) -> usize {
    let between = &source[brace_end..member];
    match between.find(|c: char| !c.is_whitespace()) {
        Some(i) => brace_end + i,
        None => member,
    }
}

/// php-parser's indentation at `at`: the whitespace starting its line, a tab as four columns.
fn indent_of(source: &str, at: usize) -> usize {
    let line = source[..at].rfind('\n').map_or(0, |i| i + 1);
    (source[line..at].chars())
        .take_while(|c| *c == ' ' || *c == '\t')
        .map(|c| if c == '\t' { 4 } else { 1 })
        .sum()
}
