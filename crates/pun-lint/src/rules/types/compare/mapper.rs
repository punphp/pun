//! Rector 2.6.1's `StaticTypeMapper` (src/StaticTypeMapper): `mapPhpParserNodePHPStanType()` of a native type
//! declaration (`PhpParserNodeMapper` with its `FullyQualifiedNodeMapper`, `NameNodeMapper`, `IdentifierNodeMapper`,
//! `NullableTypeNodeMapper`, `UnionTypeNodeMapper` and `IntersectionTypeNodeMapper`), and
//! `mapPHPStanPhpDocTypeNodeToPHPStanType()` of a PHPDoc type (`PhpDocTypeMapper` with the identifier, intersection,
//! nullable and union mappers, `ObjectTypeSpecifier` for class names, and PHPStan's `TypeNodeResolver` for the rest),
//! both resolved at one place of a file ([`TypeScope`]). Also Mago's types as PHPStan's ([`from_mago`]), for what the
//! scope says of an expression.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::float::TFloat;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_syntax::cst::{Expression, Hint, Modifier, NamespaceBody, Node, Statement, UseItems};
use pun_phpstan::phpdoc::Node as DocNode;
use pun_phpstan::resolve::{Scope, resolve_in};
use pun_phpstan::types::{
    Accessory, ConstArray, NeverKind, ShapeBuilder, Template, Type, intersect, mixed, union,
};

use super::{ClassInfo, Object, RType, create_union};
use crate::rules::types::facts::Facts;
use crate::rules::types::names::{class_imports, resolve_class_name, resolve_written};

/// A `use` import of the file (`UseImportsResolver::resolve()`): the group's prefix with its `\` (`''` outside a
/// group), the name and the alias.
struct Use {
    prefix: String,
    name: String,
    alias: Option<String>,
}

/// Where a type is mapped: the namespace, the file's imports and the class reflection of the node it belongs to.
pub(crate) struct TypeScope<'s, 'a, 'ast, 'arena> {
    ancestors: &'a [Node<'ast, 'arena>],
    namespace: Option<String>,
    uses: Vec<Use>,
    class: Option<String>,
    class_info: Option<ClassInfo>,
    codebase: &'s CodebaseMetadata,
}

impl<'s, 'a, 'ast, 'arena> TypeScope<'s, 'a, 'ast, 'arena> {
    /// The scope at the node whose ancestors are `ancestors`, the project's classes in `codebase` (with their PHPDocs
    /// read, for class templates).
    pub(crate) fn new(ancestors: &'a [Node<'ast, 'arena>], codebase: &'s CodebaseMetadata) -> Self {
        let namespace = class_imports(ancestors).0;
        let mut class = None;
        let mut class_info = None;
        for node in ancestors.iter().rev() {
            let (name, info) = match node {
                Node::Class(c) => (
                    Some(c.name.value),
                    ClassInfo {
                        is_class: true,
                        is_final: c.modifiers.iter().any(|m| matches!(m, Modifier::Final(_))),
                    },
                ),
                Node::Interface(i) => (Some(i.name.value), ClassInfo::default()),
                Node::Trait(t) => (Some(t.name.value), ClassInfo::default()),
                Node::Enum(e) => (Some(e.name.value), ClassInfo::default()),
                Node::AnonymousClass(_) => (
                    None,
                    ClassInfo {
                        is_class: true,
                        is_final: false,
                    },
                ),
                _ => continue,
            };
            class = name.map(|n| {
                let n = String::from_utf8_lossy(n);
                match &namespace {
                    Some(ns) => format!("{ns}\\{n}"),
                    None => n.into_owned(),
                }
            });
            class_info = Some(info);
            break;
        }
        TypeScope {
            ancestors,
            namespace,
            uses: file_uses(ancestors),
            class,
            class_info,
            codebase,
        }
    }

    /// The class reflection of the node, when it is in a class-like.
    /// The ancestors of the node the scope is at.
    pub(crate) fn ancestors(&self) -> &'a [Node<'ast, 'arena>] {
        self.ancestors
    }

    pub(crate) fn class_info(&self) -> Option<ClassInfo> {
        self.class_info
    }

    /// Whether the node is in a trait (`Scope::isInTrait()`).
    pub(crate) fn in_trait(&self) -> bool {
        self.ancestors.iter().rev().find_map(|n| match n {
            Node::Trait(_) => Some(true),
            Node::Class(_) | Node::Interface(_) | Node::Enum(_) | Node::AnonymousClass(_) => {
                Some(false)
            }
            _ => None,
        }) == Some(true)
    }

    fn has_class(&self, name: &str) -> bool {
        self.codebase.get_class_like(name.as_bytes()).is_some()
    }

    fn parent_class(&self) -> Option<String> {
        let class = self
            .codebase
            .get_class_like(self.class.as_ref()?.as_bytes())?;
        let parent = class.direct_parent_class.as_ref()?;
        Some(self.codebase.get_class_like(parent.as_bytes()).map_or_else(
            || String::from_utf8_lossy(parent.as_bytes()).into_owned(),
            |c| String::from_utf8_lossy(c.original_name.as_bytes()).into_owned(),
        ))
    }

    /// `mapPhpParserNodePHPStanType()` of a native type declaration.
    pub(crate) fn map_hint(&self, hint: &Hint<'_>) -> RType {
        match hint {
            Hint::Parenthesized(p) => self.map_hint(p.hint),
            Hint::Nullable(n) => {
                create_union(vec![self.map_hint(n.hint), RType::T(Type::Null)], false)
            }
            Hint::Union(_) => {
                let mut members = Vec::new();
                flatten(hint, true, &mut members);
                create_union(
                    members.into_iter().map(|h| self.map_hint(h)).collect(),
                    true,
                )
            }
            Hint::Intersection(_) => {
                let mut members = Vec::new();
                flatten(hint, false, &mut members);
                RType::Intersection(members.into_iter().map(|h| self.map_hint(h)).collect())
            }
            Hint::Identifier(id) => {
                // `FullyQualifiedNodeMapper` on the name PHPStan's `NameResolver` resolved, its original kept
                let written = String::from_utf8_lossy(id.value());
                let original = written.trim_start_matches('\\');
                let full = resolve_written(self.ancestors, &written).0;
                if !original.is_empty()
                    && original != full
                    && !full.ends_with(&format!("\\{original}"))
                {
                    RType::Object(Object::Aliased {
                        alias: original.to_string(),
                        full,
                    })
                } else {
                    RType::Object(Object::FullyQualified(full))
                }
            }
            // `NameNodeMapper::createClassReferenceType()`
            Hint::Static(_) => match &self.class {
                Some(class) => RType::T(Type::Static(class.clone(), vec![])),
                None => RType::mixed(),
            },
            Hint::Self_(_) => match &self.class {
                Some(class) => RType::Object(Object::SelfStatic(class.clone())),
                None => RType::mixed(),
            },
            Hint::Parent(_) => match (&self.class, self.parent_class()) {
                (None, _) => RType::mixed(),
                (Some(_), Some(parent)) => RType::Object(Object::ParentStatic(parent)),
                (Some(_), None) => RType::Object(Object::ParentWithoutClass),
            },
            Hint::Null(k) | Hint::True(k) | Hint::False(k) | Hint::Array(k) | Hint::Callable(k) => {
                RType::T(scalar_string_to_type(&String::from_utf8_lossy(k.value)))
            }
            Hint::Void(i)
            | Hint::Never(i)
            | Hint::Float(i)
            | Hint::Bool(i)
            | Hint::Integer(i)
            | Hint::String(i)
            | Hint::Object(i)
            | Hint::Mixed(i)
            | Hint::Iterable(i) => {
                RType::T(scalar_string_to_type(&String::from_utf8_lossy(i.value)))
            }
        }
    }

    /// `mapPHPStanPhpDocTypeNodeToPHPStanType()`.
    pub(crate) fn map_doc(&self, node: &DocNode) -> RType {
        match node {
            DocNode::Ident(name) => self.map_identifier(name),
            DocNode::Inter(members) => self.map_intersection(members),
            DocNode::Nullable(inner) => match &**inner {
                DocNode::Ident(name) => match self.map_identifier(name) {
                    RType::Union(types) => {
                        let mut all = vec![RType::T(Type::Null)];
                        all.extend(types);
                        RType::Union(all)
                    }
                    t => RType::Union(vec![RType::T(Type::Null), t]),
                },
                _ => self.resolve(node),
            },
            DocNode::Union(members) => {
                let types = members
                    .iter()
                    .map(|m| match m {
                        DocNode::Ident(name) => self.map_identifier(name),
                        DocNode::Inter(members) => self.map_intersection(members),
                        _ => self.resolve(m),
                    })
                    .collect();
                // to prevent missing class error, e.g. in tests
                create_union(types, true)
            }
            _ => self.resolve(node),
        }
    }

    /// `IntersectionPhpDocTypeMapper`: identifiers only, else `mixed`.
    fn map_intersection(&self, members: &[DocNode]) -> RType {
        let mut types = Vec::new();
        for m in members {
            let DocNode::Ident(name) = m else {
                return RType::mixed();
            };
            types.push(self.map_identifier(name));
        }
        RType::Intersection(types)
    }

    /// PHPStan's `TypeNodeResolver::resolve()` in the node's `NameScope` (without template types).
    fn resolve(&self, node: &DocNode) -> RType {
        RType::T(resolve_in(node, self))
    }

    /// `IdentifierPhpDocTypeMapper::mapIdentifierTypeNode()`.
    fn map_identifier(&self, name: &str) -> RType {
        let t = scalar_string_to_type(name);
        if !t.is_implicit_mixed() {
            return RType::T(t);
        }
        match name.to_ascii_lowercase().as_str() {
            "self" => {
                return match &self.class {
                    Some(class) => RType::Object(Object::SelfObject(class.clone())),
                    None => RType::mixed(),
                };
            }
            "parent" => {
                return match (&self.class, self.parent_class()) {
                    (Some(class), Some(parent)) if self.has_class(class) => {
                        RType::Object(Object::ParentStatic(parent))
                    }
                    _ => RType::mixed(),
                };
            }
            "static" => {
                return match &self.class {
                    Some(class) if self.has_class(class) => {
                        RType::T(Type::Static(class.clone(), vec![]))
                    }
                    _ => RType::mixed(),
                };
            }
            _ => {}
        }
        if let Some(stripped) = name.strip_prefix('\\') {
            return self.narrow(stripped, true);
        }
        if name == "scalar" {
            // pseudo type, see https://www.php.net/manual/en/language.types.intro.php
            return RType::Union(vec![
                RType::T(Type::Bool),
                RType::T(Type::String),
                RType::T(Type::Int),
                RType::T(Type::Float),
            ]);
        }
        let name = name.trim_start_matches('@');
        if let Some(stripped) = name.strip_prefix('\\') {
            return RType::Object(Object::FullyQualified(stripped.to_string()));
        }
        self.narrow(name, false)
    }

    /// `ObjectTypeSpecifier::narrowToFullyQualifiedOrAliasedObjectType()`.
    fn narrow(&self, class_name: &str, with_preslash: bool) -> RType {
        if !with_preslash {
            if let Some(aliased) = self.match_aliased(class_name) {
                return aliased;
            }
            if let Some(shortened) = self.match_shortened(class_name) {
                return shortened;
            }
        }
        if self.has_class(class_name) {
            return RType::Object(Object::FullyQualified(class_name.to_string()));
        }
        // probably in same namespace
        if let Some(ns) = &self.namespace {
            let name = format!("{ns}\\{class_name}");
            if self.has_class(&name) {
                return RType::Object(Object::FullyQualified(name));
            }
        }
        if let Some(class) = &self.class
            && let Some(template) = (self.codebase.get_class_like(class.as_bytes())).and_then(|c| {
                c.template_types
                    .iter()
                    .find(|(k, _)| k.as_bytes() == class_name.as_bytes())
                    .map(|(_, t)| t)
            })
        {
            return RType::T(Type::Template(Box::new(Template {
                name: class_name.to_string(),
                scope: format!("class {class}"),
                bound: from_mago(&template.constraint),
                default: None,
                argument: false,
            })));
        }
        // invalid type
        RType::Object(Object::NonExisting(match &self.namespace {
            Some(ns) if !with_preslash && !class_name.contains('\\') => {
                format!("{ns}\\{class_name}")
            }
            _ => class_name.to_string(),
        }))
    }

    /// `matchAliasedObjectType()`.
    fn match_aliased(&self, class_name: &str) -> Option<RType> {
        for u in &self.uses {
            let Some(alias) = &u.alias else { continue };
            let use_name = format!("{}{}", u.prefix, u.name);
            if alias == class_name || use_name == class_name {
                return Some(RType::Object(Object::Aliased {
                    alias: class_name.to_string(),
                    full: use_name,
                }));
            }
            if class_name.starts_with(&format!("{alias}\\")) {
                return Some(RType::Object(Object::Aliased {
                    alias: class_name.to_string(),
                    full: format!("{use_name}{}", php_ltrim(class_name, alias)),
                }));
            }
        }
        None
    }

    /// `matchShortenedObjectType()`, with `matchPartialNamespaceObjectType()`.
    fn match_shortened(&self, class_name: &str) -> Option<RType> {
        for u in self.uses.iter().filter(|u| u.alias.is_none()) {
            let last = u.name.rsplit('\\').next().unwrap_or(&u.name);
            if class_name == last {
                return Some(RType::Object(Object::Shortened {
                    short: class_name.to_string(),
                    full: format!("{}{}", u.prefix, u.name),
                }));
            }
            // partial namespace
            if let Some((_, after)) = class_name.split_once('\\')
                && class_name.starts_with(&format!("{last}\\"))
            {
                let connected = format!("{}{}\\{after}", u.prefix, u.name);
                if class_name != connected {
                    return Some(RType::Object(Object::Shortened {
                        short: class_name.to_string(),
                        full: connected,
                    }));
                }
            }
        }
        None
    }
}

impl Scope for TypeScope<'_, '_, '_, '_> {
    fn class(&self) -> Option<&str> {
        self.class.as_deref()
    }

    fn parent(&self) -> Option<String> {
        self.parent_class()
    }

    /// `NameScope::resolveStringName()`, its uses keyed by the lowercased alias.
    fn resolve_name(&self, name: &str) -> String {
        if let Some(stripped) = name.strip_prefix('\\') {
            return stripped.to_string();
        }
        let (first, rest) = match name.split_once('\\') {
            Some((first, rest)) => (first, Some(rest)),
            None => (name, None),
        };
        if let Some(full) = self.use_by_alias(first) {
            return match rest {
                Some(rest) => format!("{full}\\{rest}"),
                None => full,
            };
        }
        match &self.namespace {
            Some(ns) => format!("{ns}\\{name}"),
            None => name.to_string(),
        }
    }

    fn has_use_alias(&self, name: &str) -> bool {
        self.use_by_alias(name).is_some()
    }

    fn in_namespace(&self) -> bool {
        self.namespace.is_some()
    }
}

impl TypeScope<'_, '_, '_, '_> {
    /// `NameScopeFactory::resolveUseNamesByAlias()`: the last import whose alias is `alias`, in any case.
    fn use_by_alias(&self, alias: &str) -> Option<String> {
        let lower = alias.to_ascii_lowercase();
        self.uses
            .iter()
            .rev()
            .find(|u| {
                let a = u
                    .alias
                    .as_deref()
                    .unwrap_or_else(|| u.name.rsplit('\\').next().unwrap_or(&u.name));
                a.to_ascii_lowercase() == lower
            })
            .map(|u| format!("{}{}", u.prefix, u.name))
    }
}

/// PHP's `ltrim($string, $characters)`.
fn php_ltrim<'a>(s: &'a str, characters: &str) -> &'a str {
    s.trim_start_matches(|c| characters.contains(c))
}

/// The members of a union (`union`) or intersection, nested ones flattened as php-parser keeps them.
fn flatten<'h, 'arena>(hint: &'h Hint<'arena>, union: bool, out: &mut Vec<&'h Hint<'arena>>) {
    match hint {
        Hint::Union(u) if union => {
            flatten(u.left, union, out);
            flatten(u.right, union, out);
        }
        Hint::Intersection(i) if !union => {
            flatten(i.left, union, out);
            flatten(i.right, union, out);
        }
        _ => out.push(hint),
    }
}

/// `FileNode::getUsesAndGroupUses()`: the imports of the file's first namespace, or of the file.
fn file_uses(ancestors: &[Node<'_, '_>]) -> Vec<Use> {
    let Some(Node::Program(program)) = ancestors.first() else {
        return Vec::new();
    };
    let statements: Vec<&Statement<'_>> = program
        .statements
        .iter()
        .find_map(|s| match s {
            Statement::Namespace(ns) => Some(match &ns.body {
                NamespaceBody::Implicit(body) => body.statements.iter().collect(),
                NamespaceBody::BraceDelimited(block) => block.statements.iter().collect(),
            }),
            _ => None,
        })
        .unwrap_or_else(|| program.statements.iter().collect());
    let text = |b: &[u8]| {
        String::from_utf8_lossy(b)
            .trim_start_matches('\\')
            .to_string()
    };
    let mut uses = Vec::new();
    for statement in statements {
        let Statement::Use(r#use) = statement else {
            continue;
        };
        let mut push = |prefix: String, item: &mago_syntax::cst::UseItem<'_>| {
            uses.push(Use {
                prefix,
                name: text(item.name.value()),
                alias: (item.alias.as_ref()).map(|a| text(a.identifier.value)),
            })
        };
        match &r#use.items {
            UseItems::Sequence(s) => s.items.iter().for_each(|i| push(String::new(), i)),
            UseItems::TypedSequence(s) => s.items.iter().for_each(|i| push(String::new(), i)),
            UseItems::MixedList(l) => {
                let prefix = format!("{}\\", text(l.namespace.value()));
                l.items.iter().for_each(|i| push(prefix.clone(), &i.item));
            }
            UseItems::TypedList(l) => {
                let prefix = format!("{}\\", text(l.namespace.value()));
                l.items.iter().for_each(|i| push(prefix.clone(), i));
            }
        }
    }
    uses
}

/// `ScalarStringToTypeMapper::mapScalarStringToType()`.
pub(crate) fn scalar_string_to_type(name: &str) -> Type {
    use Accessory as A;
    let accessory = |accessories: &[Accessory]| {
        let mut types = vec![Type::String];
        types.extend(accessories.iter().cloned().map(Type::Accessory));
        intersect(types)
    };
    match name.to_lowercase().as_str() {
        "false" => Type::ConstBool(false),
        "true" => Type::ConstBool(true),
        "positive-int" => Type::int_range(Some(1), None),
        "negative-int" => Type::int_range(None, Some(-1)),
        "non-negative-int" => Type::int_range(Some(0), None),
        "non-positive-int" => Type::int_range(None, Some(0)),
        "array-key" => Type::Union(vec![Type::Int, Type::String]),
        "callable-string" => intersect(vec![Type::String, Type::Callable(None)]),
        "non-empty-lowercase-string" => accessory(&[A::NonEmptyString, A::LowercaseString]),
        "non-empty-uppercase-string" => accessory(&[A::NonEmptyString, A::UppercaseString]),
        "non-empty-literal-string" => accessory(&[A::NonEmptyString, A::LiteralString]),
        "decimal-int-string" => accessory(&[A::DecimalIntString { inverse: false }]),
        "non-decimal-int-string" => accessory(&[A::DecimalIntString { inverse: true }]),
        "string" => Type::String,
        "non-empty-string" => Type::Accessory(A::NonEmptyString),
        "literal-string" => Type::Accessory(A::LiteralString),
        "numeric-string" => Type::Accessory(A::NumericString),
        "non-falsy-string" | "truthy-string" => Type::Accessory(A::NonFalsyString),
        "lowercase-string" => Type::Accessory(A::LowercaseString),
        "uppercase-string" => Type::Accessory(A::UppercaseString),
        "non-empty-array" => Type::Accessory(A::NonEmptyArray),
        "class-string" => Type::ClassString(None),
        "float" | "real" | "double" => Type::Float,
        "int" | "integer" => Type::Int,
        "bool" | "boolean" => Type::Bool,
        "null" => Type::Null,
        "void" => Type::Void,
        "resource" => Type::Resource,
        "callback" | "callable" => Type::Callable(None),
        "object" => Type::ObjectWithoutClass,
        "never" | "never-return" | "never-returns" | "no-return" => {
            Type::Never(NeverKind::Implicit)
        }
        "list" => intersect(vec![
            Type::array(mixed(), mixed()),
            Type::Accessory(A::List),
        ]),
        "array" => Type::array(mixed(), mixed()),
        "iterable" => Type::Iterable(Box::new(mixed()), Box::new(mixed())),
        "mixed" => Type::Mixed { explicit: true },
        _ => mixed(),
    }
}

/// The type PHPStan's scope gives `expr`, whose ancestors are `ancestors` (`getNativeType()` when `native`, else
/// `getType()`), from Mago's analysis; mixed when Mago has none. PHPStan's `NewHandler` types `new Name()` as that
/// class's object even when no such class exists, where Mago infers `never`.
pub(crate) fn expr_type(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    native: bool,
) -> Type {
    let span = expr.span();
    let span = pun_problem::Span::new(span.start.offset as usize, span.end.offset as usize);
    let t = if native {
        facts.native_type(span)
    } else {
        facts.full_type(span)
    };
    let t = t.map_or_else(mixed, from_mago);
    if let (Type::Never(_), Expression::Instantiation(new)) = (&t, expr)
        && let Expression::Identifier(class) = new.class
    {
        let written = String::from_utf8_lossy(class.value());
        if !["self", "static", "parent"]
            .iter()
            .any(|k| written.eq_ignore_ascii_case(k))
        {
            return Type::Object(resolve_class_name(ancestors, class), vec![], false);
        }
    }
    t
}

/// A Mago type (what its analyser gives an expression) as PHPStan's.
pub(crate) fn from_mago(t: &TUnion) -> Type {
    if t.types.is_empty() {
        return Type::Never(NeverKind::Implicit);
    }
    union(t.types.iter().map(from_atomic).collect())
}

fn from_atomic(a: &TAtomic) -> Type {
    let text = |b: &[u8]| String::from_utf8_lossy(b).into_owned();
    match a {
        TAtomic::Null => Type::Null,
        TAtomic::Void => Type::Void,
        TAtomic::Never => Type::Never(NeverKind::Explicit),
        TAtomic::Scalar(s) => match s {
            TScalar::Bool(b) => b.value.map_or(Type::Bool, Type::ConstBool),
            TScalar::Integer(TInteger::Literal(i)) => Type::ConstInt(*i),
            TScalar::Integer(TInteger::From(i)) => Type::int_range(Some(*i), None),
            TScalar::Integer(TInteger::To(i)) => Type::int_range(None, Some(*i)),
            TScalar::Integer(TInteger::Range(a, b)) => Type::int_range(Some(*a), Some(*b)),
            TScalar::Integer(_) => Type::Int,
            TScalar::Float(TFloat::Literal(f)) => Type::ConstFloat(f.0),
            TScalar::Float(_) => Type::Float,
            TScalar::String(s) => match s.get_known_literal_value() {
                Some(v) => Type::ConstString(v.to_vec()),
                None => Type::String,
            },
            TScalar::ClassLikeString(_) => Type::ClassString(None),
            TScalar::ArrayKey => Type::Union(vec![Type::Int, Type::String]),
            TScalar::Numeric => Type::Union(vec![
                Type::Int,
                Type::Float,
                intersect(vec![
                    Type::String,
                    Type::Accessory(Accessory::NumericString),
                ]),
            ]),
            _ => Type::Union(vec![Type::Int, Type::Float, Type::String, Type::Bool]),
        },
        TAtomic::Array(TArray::List(l)) => match &l.known_elements {
            Some(items) => {
                let mut builder = ShapeBuilder::new();
                for (i, (optional, v)) in items.iter() {
                    builder.set(Some(Type::ConstInt(*i as i64)), from_mago(v), *optional);
                }
                builder.get()
            }
            None => Type::list(from_mago(&l.element_type)),
        },
        TAtomic::Array(TArray::Keyed(k)) => match (&k.known_items, &k.parameters) {
            (Some(items), None) => {
                let mut builder = ShapeBuilder::new();
                for (key, (optional, v)) in items.iter() {
                    let key = match key {
                        ArrayKey::Integer(i) => Type::ConstInt(*i),
                        ArrayKey::String(s) => Type::ConstString(s.as_bytes().to_vec()),
                        _ => continue,
                    };
                    builder.set(Some(key), from_mago(v), *optional);
                }
                builder.get()
            }
            (_, Some((key, value))) => Type::array(from_mago(key), from_mago(value)),
            (None, None) => Type::ConstArray(Box::new(ConstArray::empty())),
        },
        TAtomic::Iterable(i) => Type::Iterable(
            Box::new(from_mago(i.get_key_type())),
            Box::new(from_mago(i.get_value_type())),
        ),
        TAtomic::Callable(TCallable::Signature(s)) if s.is_closure() => {
            Type::Object("Closure".to_string(), vec![], false)
        }
        TAtomic::Callable(_) => Type::Callable(None),
        TAtomic::Object(TObject::Named(n)) => {
            let name = text(n.name.as_bytes());
            let object = if n.is_this {
                Type::This(name)
            } else if n.is_static {
                Type::Static(name, vec![])
            } else {
                Type::Object(name, vec![], false)
            };
            let Some(intersections) = &n.intersection_types else {
                return object;
            };
            intersect(
                std::iter::once(object)
                    .chain(intersections.iter().map(from_atomic))
                    .collect(),
            )
        }
        TAtomic::Object(TObject::Enum(e)) => match &e.case {
            Some(case) => Type::EnumCase(text(e.name.as_bytes()), text(case.as_bytes())),
            None => Type::Object(text(e.name.as_bytes()), vec![], false),
        },
        TAtomic::Object(_) => Type::ObjectWithoutClass,
        TAtomic::Resource(_) => Type::Resource,
        TAtomic::GenericParameter(g) => Type::Template(Box::new(Template {
            name: text(g.parameter_name.as_bytes()),
            scope: String::new(),
            bound: from_mago(&g.constraint),
            default: None,
            argument: false,
        })),
        TAtomic::Mixed(_) => mixed(),
        _ => mixed(),
    }
}
