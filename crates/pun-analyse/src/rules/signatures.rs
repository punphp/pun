//! The rest of FunctionDefinitionCheck (checkParametersAcceptor, checkAnonymousFunction,
//! checkRequiredParameterAfterOptional, checkImplicitlyNullableType and checkClassMethod's `@phpstan-self-out`), the
//! unresolvable-type part of ExistingClassesInPropertiesRule (UnresolvableTypeHelper on the native type) and
//! ExistingClassesInArrowFunctionTypehintsRule's `never` check. The class-existence part is names.rs `typehints()`.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use mago_names::kind::NameKind;
use mago_names::scope::NamespaceScope;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::phpdoc::{self, Node};
use pun_phpstan::resolve::resolve;
use pun_phpstan::types::{Tri, Type, is_super_type_of};

use super::Checker;
use super::class_names::{self, Pair};
use super::declarations::doc_before;
use super::names::FunctionKind;
use super::reflection::{Known, known};

/// The names PHPDoc types resolve against: the namespace and its `use` imports (php-parser's NameScope).
pub struct State {
    scope: NamespaceScope,
}

impl Default for State {
    fn default() -> Self {
        State {
            scope: NamespaceScope::global(),
        }
    }
}

pub fn enter_namespace(c: &mut Checker, name: Option<&str>) {
    c.signatures.scope = NamespaceScope::new(name.map(|n| n.as_bytes().to_vec()));
}

pub fn leave_namespace(c: &mut Checker) {
    c.signatures.scope = NamespaceScope::global();
}

pub fn use_statement(c: &mut Checker, u: &Use) {
    c.signatures.scope.populate_from_use(u);
}

impl FunctionKind<'_> {
    /// What PHPStan's messages call the function-like after "of": `function f()`, `method C::m()`,
    /// `anonymous function`, `set hook for property C::$p`.
    pub fn named(&self) -> String {
        match self {
            FunctionKind::Function(f) => format!("function {f}()"),
            FunctionKind::Method(k, m) => format!("method {k}::{m}()"),
            FunctionKind::Anonymous => "anonymous function".to_string(),
            FunctionKind::Hook(k, p, h) => format!("{h} hook for property {k}::${p}"),
        }
    }

    /// The same, starting a sentence (`ucfirst()`).
    pub fn subject(&self) -> String {
        let named = self.named();
        let mut chars = named.chars();
        let first = chars.next().map(|f| f.to_ascii_uppercase());
        first.into_iter().chain(chars).collect()
    }
}

/// php-parser's `ConstFetch` of `null` as a default value.
fn null_default(p: &FunctionLikeParameter) -> bool {
    match p.default_value.as_ref().map(|d| d.value) {
        Some(Expression::Literal(Literal::Null(_))) => true,
        Some(Expression::ConstantAccess(a)) => a
            .name
            .value()
            .strip_prefix(b"\\")
            .unwrap_or(a.name.value())
            .eq_ignore_ascii_case(b"null"),
        _ => false,
    }
}

/// The members of a union (php-parser's flat `UnionType::$types`).
fn union_members<'h, 'arena>(hint: &'h Hint<'arena>, out: &mut Vec<&'h Hint<'arena>>) {
    match hint {
        Hint::Union(u) => {
            union_members(u.left, out);
            union_members(u.right, out);
        }
        h => out.push(h),
    }
}

fn intersection_members<'h, 'arena>(hint: &'h Hint<'arena>, out: &mut Vec<&'h Hint<'arena>>) {
    match hint {
        Hint::Intersection(i) => {
            intersection_members(i.left, out);
            intersection_members(i.right, out);
        }
        Hint::Parenthesized(p) => intersection_members(p.hint, out),
        h => out.push(h),
    }
}

/// `NodeTypePrinter::printType()`: names as NameResolver left them, keywords as written.
fn print_type(c: &Checker, hint: &Hint) -> String {
    let joined = |members: Vec<&Hint>, glue: &str| {
        members
            .iter()
            .map(|m| print_type(c, m))
            .collect::<Vec<_>>()
            .join(glue)
    };
    match hint {
        Hint::Union(_) => {
            let mut members = Vec::new();
            union_members(hint, &mut members);
            joined(members, "|")
        }
        Hint::Intersection(_) | Hint::Parenthesized(_) => {
            let mut members = Vec::new();
            intersection_members(hint, &mut members);
            joined(members, "&")
        }
        Hint::Identifier(id) => c.resolved(id),
        Hint::Self_(_) | Hint::Static(_) | Hint::Parent(_) => c.written(hint.span()),
        // php-parser lowercases built-in type names
        h => c.written(h.span()).to_ascii_lowercase(),
    }
}

/// What ParserNodeTypeToPHPStanType makes of a native type, as far as UnresolvableTypeHelper needs: whether it is an
/// implicit `never` (an intersection that cannot be inhabited), and an object type's class.
enum Native {
    Never,
    Class(String),
    Object,
    Other,
}

fn native(c: &Checker, hint: &Hint) -> Native {
    match hint {
        Hint::Identifier(id) => Native::Class(c.resolved(id)),
        // outside a class, an unknown class
        Hint::Self_(_) | Hint::Static(_) => Native::Class(
            c.class_scope()
                .map(|k| String::from_utf8_lossy(k.name.as_bytes()).into_owned())
                .unwrap_or_default(),
        ),
        Hint::Parent(_) => Native::Class(
            c.class_scope()
                .and_then(|k| k.direct_parent_class)
                .and_then(|p| known(c, &String::from_utf8_lossy(p.as_bytes())))
                .map_or_else(|| c.written(hint.span()), |p| p.display()),
        ),
        Hint::Object(_) => Native::Object,
        Hint::Parenthesized(p) => native(c, p.hint),
        Hint::Union(_) => {
            let mut members = Vec::new();
            union_members(hint, &mut members);
            if members
                .iter()
                .all(|m| matches!(native(c, m), Native::Never))
            {
                Native::Never
            } else {
                Native::Other
            }
        }
        Hint::Intersection(_) => {
            let mut members = Vec::new();
            intersection_members(hint, &mut members);
            let mut classes = Vec::new();
            for m in members {
                match native(c, m) {
                    Native::Class(k) => classes.push(k),
                    Native::Object => {}
                    // `!$innerType->isObject()->yes()`
                    _ => return Native::Never,
                }
            }
            // TypeCombinator::intersect(): two object types neither of which can be the other
            for (i, a) in classes.iter().enumerate() {
                for b in &classes[i + 1..] {
                    if disjoint(c, a, b) {
                        return Native::Never;
                    }
                }
            }
            Native::Object
        }
        _ => Native::Other,
    }
}

/// UnresolvableTypeHelper on a native type: an implicit `never` (it references no classes either).
pub fn unresolvable(c: &Checker, hint: &Hint) -> bool {
    matches!(native(c, hint), Native::Never)
}

/// ObjectType::isSuperTypeOf() is no between two classes: a trait, or two classes neither extending the other where
/// neither is an interface a non-final other could implement. Unknown classes are maybe.
fn disjoint(c: &Checker, a: &str, b: &str) -> bool {
    let (Some(x), Some(y)) = (known(c, a), known(c, b)) else {
        return false;
    };
    let (trait_, interface, final_) = (
        |k: &Known| matches!(k, Known::User(u) if u.kind == mago_codex::symbol::SymbolKind::Trait),
        |k: &Known| match k {
            Known::User(u) => u.kind == mago_codex::symbol::SymbolKind::Interface,
            Known::Builtin(b) => b.is_interface,
        },
        |k: &Known| match k {
            Known::User(u) => u.flags.is_final() || u.kind == mago_codex::symbol::SymbolKind::Enum,
            Known::Builtin(b) => b.is_final,
        },
    );
    if trait_(&x) || trait_(&y) {
        return true;
    }
    if x.display().eq_ignore_ascii_case(&y.display())
        || instance_of(c, &x, &y.display())
        || instance_of(c, &y, &x.display())
    {
        return false;
    }
    !(interface(&x) && !final_(&y) || interface(&y) && !final_(&x))
}

/// `isSubclassOf()` (or the class itself).
fn instance_of(c: &Checker, k: &Known, parent: &str) -> bool {
    match k {
        Known::User(u) => c
            .codebase
            .is_instance_of(u.name.as_bytes(), parent.to_ascii_lowercase().as_bytes()),
        Known::Builtin(b) => {
            b.name.eq_ignore_ascii_case(parent)
                || b.ancestors.iter().any(|a| a.eq_ignore_ascii_case(parent))
        }
    }
}

/// The class has a parent PHPStan's reflection knows (`getParentClass() !== null`).
fn has_parent(c: &Checker) -> bool {
    c.class_scope()
        .and_then(|k| k.direct_parent_class)
        .is_some_and(|p| known(c, &String::from_utf8_lossy(p.as_bytes())).is_some())
}

/// The PHPDoc tags FunctionDefinitionCheck reads, as phpdoc-parser splits them.
#[derive(Default)]
struct Doc {
    /// `@template` names with their bound and default
    templates: Vec<(String, Option<Node>, Option<Node>)>,
    params: HashMap<String, Node>,
    ret: Option<Node>,
    self_out: Option<Node>,
}

/// Each tag and its text up to the next tag (continuation lines joined with a space).
fn tag_values(doc: &[u8]) -> Vec<(String, String)> {
    let body = doc.strip_prefix(b"/**").unwrap_or(doc);
    let body = body.strip_suffix(b"*/").unwrap_or(body);
    let mut tags: Vec<(String, String)> = Vec::new();
    for line in String::from_utf8_lossy(body).lines() {
        let line = line.trim_start();
        let line = line.strip_prefix('*').unwrap_or(line).trim();
        if let Some(rest) = line.strip_prefix('@') {
            let len = rest
                .find(|ch: char| !(ch.is_ascii_alphanumeric() || matches!(ch, '-' | '\\' | ':')))
                .unwrap_or(rest.len());
            tags.push((format!("@{}", &rest[..len]), rest[len..].trim().to_string()));
        } else if let Some((_, text)) = tags.last_mut()
            && !line.is_empty()
        {
            text.push(' ');
            text.push_str(line);
        }
    }
    tags
}

/// The longest leading part of `text` ending before whitespace (or at its end) that is one type, and what follows
/// (phpdoc-parser parses the type, then the rest of the tag).
fn leading_type(text: &str) -> Option<(Node, &str)> {
    let ends = text
        .char_indices()
        .filter(|(_, ch)| ch.is_whitespace())
        .map(|(i, _)| i)
        .chain(std::iter::once(text.len()));
    let mut found = None;
    for end in ends {
        if let Ok(node) = phpdoc::parse(&text[..end]) {
            found = Some((node, text[end..].trim_start()));
        }
    }
    found
}

fn read_doc(c: &Checker, at: Span) -> Doc {
    let mut doc = Doc::default();
    let Some(comment) = doc_before(&c.file.contents, at.start.offset as usize) else {
        return doc;
    };
    let tags = tag_values(comment);
    // later prefixes win (`resolveParamTags()`, `resolveReturnTag()`)
    let rank = |tag: &str, order: &[&str]| order.iter().position(|t| *t == tag);
    let mut params: Vec<(usize, String, Node)> = Vec::new();
    let mut ret: Option<(usize, Node)> = None;
    let mut self_out: Option<(usize, Node)> = None;
    for (tag, text) in &tags {
        let template = tag
            .strip_suffix("-covariant")
            .or_else(|| tag.strip_suffix("-contravariant"))
            .unwrap_or(tag);
        if [
            "@template",
            "@phan-template",
            "@psalm-template",
            "@phpstan-template",
        ]
        .contains(&template)
        {
            let name: String = text.chars().take_while(|ch| !ch.is_whitespace()).collect();
            let mut rest = text[name.len()..].trim_start();
            let mut bound = None;
            if let Some(b) = rest
                .strip_prefix("of ")
                .or_else(|| rest.strip_prefix("as "))
                && let Some((node, after)) = leading_type(b.trim_start())
            {
                bound = Some(node);
                rest = after;
            }
            let default = rest
                .strip_prefix('=')
                .and_then(|d| leading_type(d.trim_start()))
                .map(|(node, _)| node);
            doc.templates.push((name, bound, default));
        } else if let Some(r) = rank(
            tag,
            &["@param", "@phan-param", "@psalm-param", "@phpstan-param"],
        ) {
            if let Some((node, rest)) = leading_type(text) {
                let rest = rest.trim_start_matches(['&', '.']);
                if let Some(name) = rest.strip_prefix('$') {
                    let name: String = name
                        .chars()
                        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_' || !ch.is_ascii())
                        .collect();
                    params.push((r, name, node));
                }
            }
        } else if let Some(r) = rank(
            tag,
            &[
                "@return",
                "@phan-return",
                "@phan-real-return",
                "@psalm-return",
                "@phpstan-return",
            ],
        ) {
            if let Some((node, _)) = leading_type(text)
                && ret.as_ref().is_none_or(|(old, _)| *old <= r)
            {
                ret = Some((r, node));
            }
        } else if let Some(r) = rank(
            tag,
            &[
                "@phpstan-this-out",
                "@phpstan-self-out",
                "@psalm-this-out",
                "@psalm-self-out",
            ],
        ) {
            // the first tag of the first name found (`resolveSelfOutTypeTag()`)
            if self_out.as_ref().is_none_or(|(old, _)| r < *old)
                && let Some((node, _)) = leading_type(text)
            {
                self_out = Some((r, node));
            }
        }
    }
    doc.self_out = self_out.map(|(_, node)| node);
    params.sort_by_key(|(r, _, _)| *r);
    for (_, name, node) in params {
        doc.params.insert(name, node);
    }
    doc.ret = ret.map(|(_, node)| node);
    doc
}

/// `TypeTraverser::map()` looking for template types: each template referenced, and its bound and default with it.
fn reference(node: &Node, doc: &Doc, seen: &mut HashSet<String>) {
    let mut go = |n: &Node| reference(n, doc, seen);
    match node {
        Node::Ident(name) => {
            if let Some((_, bound, default)) = doc.templates.iter().find(|(t, ..)| t == name)
                && seen.insert(name.clone())
            {
                for n in bound.iter().chain(default) {
                    reference(n, doc, seen);
                }
            }
        }
        Node::Nullable(n) | Node::Array(n) => go(n),
        Node::Union(members) | Node::Inter(members) => members.iter().for_each(go),
        Node::Conditional {
            subject,
            target,
            then,
            otherwise,
            ..
        } => [subject, target, then, otherwise]
            .into_iter()
            .for_each(|n| go(n)),
        Node::ConditionalForParameter {
            target,
            then,
            otherwise,
            ..
        } => [target, then, otherwise].into_iter().for_each(|n| go(n)),
        Node::Generic { args, .. } => args.iter().for_each(go),
        Node::Callable { params, ret, .. } => {
            params.iter().for_each(|p| go(&p.ty));
            go(ret);
        }
        Node::ArrayShape {
            items, unsealed, ..
        } => {
            items.iter().for_each(|i| go(&i.value));
            // PHPStan 2.3 traverses the unsealed key type too
            if let Some((key, value)) = unsealed {
                if let Some(key) = key {
                    go(key);
                }
                go(value);
            }
        }
        Node::ObjectShape(items) => items.iter().for_each(|(_, _, n)| go(n)),
        Node::Projection(_, n) => go(n),
        Node::OffsetAccess(t, o) => [t, o].into_iter().for_each(|n| go(n)),
        Node::This | Node::Star(_) | Node::Const(_) => {}
    }
}

/// A name PHPDoc resolves to a class (not a keyword type, a template or `self`/`static`/`parent`).
fn class_name(c: &Checker, name: &str, templates: &[String]) -> Option<String> {
    if templates.iter().any(|t| t == name)
        || ["self", "static", "parent"]
            .iter()
            .any(|k| name.eq_ignore_ascii_case(k))
        || !matches!(
            resolve(&Node::Ident(name.to_string()), None),
            Type::Object(..)
        )
    {
        return None;
    }
    Some(match name.strip_prefix('\\') {
        Some(fq) => fq.to_string(),
        None => String::from_utf8_lossy(&c.signatures.scope.resolve(NameKind::Default, name).0)
            .into_owned(),
    })
}

/// The classes a PHPDoc type references (`getReferencedClasses()`).
fn referenced(c: &Checker, node: &Node, templates: &[String], out: &mut Vec<String>) {
    match node {
        Node::Ident(name) => out.extend(class_name(c, name, templates)),
        Node::Generic { base, args } => {
            out.extend(class_name(c, base, templates));
            for a in args {
                referenced(c, a, templates, out);
            }
        }
        Node::Nullable(n) | Node::Array(n) => referenced(c, n, templates, out),
        Node::Union(members) | Node::Inter(members) => {
            for m in members {
                referenced(c, m, templates, out);
            }
        }
        Node::Callable { params, ret, .. } => {
            for p in params {
                referenced(c, &p.ty, templates, out);
            }
            referenced(c, ret, templates, out);
        }
        Node::ArrayShape { items, .. } => {
            for i in items {
                referenced(c, &i.value, templates, out);
            }
        }
        Node::ObjectShape(items) => {
            for (_, _, n) in items {
                referenced(c, n, templates, out);
            }
        }
        Node::Conditional {
            subject,
            target,
            then,
            otherwise,
            ..
        } => {
            for n in [subject, target, then, otherwise] {
                referenced(c, n, templates, out);
            }
        }
        Node::ConditionalForParameter {
            target,
            then,
            otherwise,
            ..
        } => {
            for n in [target, then, otherwise] {
                referenced(c, n, templates, out);
            }
        }
        Node::Projection(_, n) => referenced(c, n, templates, out),
        Node::OffsetAccess(t, o) => {
            referenced(c, t, templates, out);
            referenced(c, o, templates, out);
        }
        Node::This | Node::Star(_) | Node::Const(_) => {}
    }
}

/// The PHPDoc type with its class names resolved and its templates replaced by their bounds
/// (`TemplateTypeHelper::resolveToBounds()`), for comparing it with the native type.
fn to_bounds(c: &Checker, node: &Node, doc: &Doc, depth: usize) -> Node {
    let names: Vec<String> = doc.templates.iter().map(|(t, ..)| t.clone()).collect();
    let go = |n: &Node| to_bounds(c, n, doc, depth + 1);
    match node {
        Node::Ident(name) => {
            if let Some((_, bound, _)) = doc.templates.iter().find(|(t, ..)| t == name) {
                return match bound {
                    Some(b) if depth < 8 => go(b),
                    _ => Node::Ident("mixed".to_string()),
                };
            }
            let class = c.class_scope().filter(|_| {
                name.eq_ignore_ascii_case("self") || name.eq_ignore_ascii_case("static")
            });
            match class {
                Some(k) => Node::Ident(String::from_utf8_lossy(k.name.as_bytes()).into_owned()),
                None => Node::Ident(class_name(c, name, &names).unwrap_or_else(|| name.clone())),
            }
        }
        Node::Generic { base, args } => Node::Generic {
            base: class_name(c, base, &names).unwrap_or_else(|| base.clone()),
            args: args.iter().map(go).collect(),
        },
        Node::Nullable(n) => Node::Nullable(Box::new(go(n))),
        Node::Array(n) => Node::Array(Box::new(go(n))),
        Node::Union(members) => Node::Union(members.iter().map(go).collect()),
        Node::Inter(members) => Node::Inter(members.iter().map(go).collect()),
        n => n.clone(),
    }
}

/// A native type as a PHPDoc type (resolved names; `self` and `static` the class).
fn hint_node(c: &Checker, hint: &Hint) -> Node {
    match hint {
        Hint::Identifier(id) => Node::Ident(c.resolved(id)),
        Hint::Nullable(n) => Node::Nullable(Box::new(hint_node(c, n.hint))),
        Hint::Parenthesized(p) => hint_node(c, p.hint),
        Hint::Union(_) => {
            let mut members = Vec::new();
            union_members(hint, &mut members);
            Node::Union(members.into_iter().map(|m| hint_node(c, m)).collect())
        }
        Hint::Intersection(_) => {
            let mut members = Vec::new();
            intersection_members(hint, &mut members);
            Node::Inter(members.into_iter().map(|m| hint_node(c, m)).collect())
        }
        Hint::Self_(_) | Hint::Static(_) => Node::Ident(
            c.class_scope()
                .map(|k| String::from_utf8_lossy(k.name.as_bytes()).into_owned())
                .unwrap_or_else(|| "object".to_string()),
        ),
        h => Node::Ident(c.written(h.span()).to_ascii_lowercase()),
    }
}

fn without_null(node: Node) -> Node {
    let is_null = |n: &Node| matches!(n, Node::Ident(i) if i.eq_ignore_ascii_case("null"));
    match node {
        Node::Nullable(inner) => *inner,
        Node::Union(members) if members.iter().any(is_null) => {
            let mut rest: Vec<Node> = members.into_iter().filter(|m| !is_null(m)).collect();
            if rest.len() == 1 {
                rest.remove(0)
            } else {
                Node::Union(rest)
            }
        }
        n => n,
    }
}

/// TypehintHelper::decideType(): the PHPDoc type is the parameter's (or return) type when the native type is a
/// supertype of it with its templates at their bounds; else the native type is.
fn phpdoc_applies(
    c: &Checker,
    native: Option<&Hint>,
    nullable: bool,
    phpdoc: &Node,
    doc: &Doc,
) -> bool {
    let Some(native) = native else {
        return true;
    };
    if matches!(native, Hint::Callable(_)) && callable(c, &without_null(phpdoc.clone())) {
        return true;
    }
    let mut native_node = hint_node(c, native);
    if nullable {
        native_node = Node::Nullable(Box::new(native_node));
    }
    let native_type = resolve(&native_node, None);
    let mut bounds = to_bounds(c, phpdoc, doc, 0);
    if !native_type.accepts_null() {
        bounds = without_null(bounds);
    }
    let phpdoc_type = resolve(&bounds, None);
    match is_super_type_of(&native_type, &phpdoc_type) {
        Tri::Yes => true,
        Tri::No => false,
        // the classes pun-phpstan's symbols don't know: ObjectType::isSuperTypeOf() on Mago's codebase
        // each member a supertype, or a class pun-phpstan's symbols don't know that is a subclass of a native one
        // (ObjectType::isSuperTypeOf() on Mago's codebase)
        Tri::Maybe => {
            let members = |t: &Type| match t {
                Type::Union(members) => members.clone(),
                t => vec![t.clone()],
            };
            let natives = members(&native_type);
            members(&phpdoc_type).iter().all(|m| match m {
                Type::Object(class, _, _) => known(c, class).is_some_and(|k| {
                    natives
                        .iter()
                        .any(|n| matches!(n, Type::Object(parent, _, _) if instance_of(c, &k, parent)))
                }),
                m => is_super_type_of(&native_type, m).yes(),
            })
        }
    }
}

/// `isCallable()->yes()` of a PHPDoc type: callable types, and classes with `__invoke()`.
fn callable(c: &Checker, node: &Node) -> bool {
    let class = |name: &str| {
        let Some(name) = class_name(c, name, &[]) else {
            return [
                "callable",
                "callable-string",
                "callable-array",
                "callable-object",
            ]
            .iter()
            .any(|k| name.eq_ignore_ascii_case(k));
        };
        match known(c, &name) {
            Some(Known::User(k)) => k
                .appearing_method_ids
                .contains_key(&mago_word::word(b"__invoke")),
            Some(Known::Builtin(k)) => k.name == "Closure" || k.methods.contains_key("__invoke"),
            None => false,
        }
    };
    match node {
        Node::Callable { .. } => true,
        Node::Ident(name) | Node::Generic { base: name, .. } => class(name),
        Node::Union(members) => members.iter().all(|m| callable(c, m)),
        Node::Inter(members) => members.iter().any(|m| callable(c, m)),
        _ => false,
    }
}

trait AcceptsNull {
    fn accepts_null(&self) -> bool;
}

impl AcceptsNull for Type {
    fn accepts_null(&self) -> bool {
        match self {
            Type::Null | Type::Mixed { .. } => true,
            Type::Union(members) => members.iter().any(AcceptsNull::accepts_null),
            _ => false,
        }
    }
}

/// FunctionDefinitionCheck for one function-like, after the class-existence checks: `at` is the function-like node,
/// the line of its template and self-out errors.
pub fn check<'arena>(
    c: &mut Checker<'_, 'arena>,
    kind: FunctionKind,
    at: Span,
    parameters: &FunctionLikeParameterList<'arena>,
    return_type: Option<&FunctionLikeReturnTypeHint<'arena>>,
) {
    let anonymous = matches!(kind, FunctionKind::Anonymous);
    let of = kind.named();
    let subject = kind.subject();
    let params: Vec<&FunctionLikeParameter> = parameters.parameters.iter().collect();
    if !c.surely_at_least(80000) {
        // (a hook's message starts lowercase, but PHPStan's parser rejects hooks before PHP 8.4)
        let message = format!(
            "{subject} uses native union types but they're supported only on PHP 8.0 and later."
        );
        if let Some(p) = params
            .iter()
            .find(|p| matches!(p.hint, Some(Hint::Union(_))))
        {
            c.report_non_ignorable(*p, "parameter.unionTypeNotSupported", message, None);
        } else if let Some(r) = return_type.filter(|r| matches!(r.hint, Hint::Union(_))) {
            c.report_non_ignorable(r.hint.span(), "return.unionTypeNotSupported", message, None);
        }
    }
    parameter_defaults(c, parameters);
    let doc = if anonymous {
        Doc::default()
    } else {
        read_doc(c, at)
    };
    let pure_intersections = c.php_version >= 80100;
    for p in &params {
        let name = String::from_utf8_lossy(p.variable.name).into_owned();
        // PHPStan 2.3 reads a function's or method's parameter with a `null` default as nullable, and `never|null` is
        // `null` (oracle-checked; not for a closure's or a promoted property's)
        if let Some(hint) = &p.hint
            && pure_intersections
            && (anonymous || p.is_promoted_property() || !null_default(p))
            && unresolvable(c, hint)
        {
            let line = if anonymous { hint.span() } else { p.span() };
            c.report_non_ignorable(
                line,
                "parameter.unresolvableNativeType",
                format!("Parameter {name} of {of} has unresolvable native type."),
                None,
            );
        }
        if anonymous || has_parent(c) {
            continue;
        }
        let parent = match &p.hint {
            Some(hint) => matches!(hint, Hint::Parent(_)) && !null_default(p),
            // outside a class, PHPDoc's `parent` is a class name
            None => {
                c.class_scope().is_some()
                    && doc.params.get(&name[1..]).is_some_and(
                        |n| matches!(n, Node::Ident(i) if i.eq_ignore_ascii_case("parent")),
                    )
            }
        };
        if parent {
            c.report(
                *p,
                "parameter.noParent",
                format!("Parameter {name} of {of} has invalid type parent."),
                None,
            );
        }
    }
    if let Some(r) = return_type
        && pure_intersections
        && unresolvable(c, &r.hint)
    {
        c.report_non_ignorable(
            r.hint.span(),
            "return.unresolvableNativeType",
            format!("{subject} has unresolvable native return type."),
            None,
        );
    }
    if anonymous {
        return;
    }
    let returns_parent = match return_type {
        Some(r) => matches!(r.hint, Hint::Parent(_)),
        None => {
            c.class_scope().is_some()
                && matches!(&doc.ret, Some(Node::Ident(i)) if i.eq_ignore_ascii_case("parent"))
        }
    };
    if returns_parent && !has_parent(c) {
        c.report(
            return_type.map_or(at, |r| r.hint.span()),
            "return.noParent",
            format!("{subject} has invalid return type parent."),
            None,
        );
    }
    templates(c, at, kind, &params, return_type, &doc);
    if matches!(kind, FunctionKind::Method(..)) {
        self_out(c, at, &subject, &doc);
    }
}

/// TypeTraverser on resolved PHPDoc: only real TemplateTypes count, with their original bounds and defaults.
/// In particular, an inherited template must not be rebound to a same-named template in the child's scope.
fn reference_type(ty: &Type, seen: &RefCell<HashSet<String>>) {
    if let Type::Template(t) = ty {
        seen.borrow_mut().insert(t.name.clone());
    }
    ty.traverse(&|inner| {
        reference_type(inner, seen);
        inner.clone()
    });
}

/// The template types no parameter's type (or a non-negated conditional return type) references.
fn templates(
    c: &mut Checker,
    at: Span,
    kind: FunctionKind,
    params: &[&FunctionLikeParameter],
    return_type: Option<&FunctionLikeReturnTypeHint>,
    doc: &Doc,
) {
    if doc.templates.is_empty() {
        return;
    }
    let mut seen = HashSet::new();
    for p in params {
        let name = String::from_utf8_lossy(&p.variable.name[1..]).into_owned();
        if let Some(node) = doc.params.get(&name)
            && phpdoc_applies(c, p.hint.as_ref(), null_default(p), node, doc)
        {
            reference(node, doc, &mut seen);
        }
    }
    if let Some(node @ Node::ConditionalForParameter { negated: false, .. }) = &doc.ret
        && phpdoc_applies(c, return_type.map(|r| &r.hint), false, node, doc)
    {
        reference(node, doc, &mut seen);
    }
    // getParameters()->getType() includes inherited tags, renamed by position and filled only where a local tag
    // is absent. Reuse the reflection resolver rather than interpreting the parent's text in the child's scope.
    if let FunctionKind::Method(_, method) = kind
        && let Some(class) = c.class_scope()
        && let Some(method) = c
            .codebase
            .get_method(class.name.as_bytes(), method.as_bytes())
    {
        let inherited = super::declared::with_classes(c, || {
            let inherited = RefCell::new(HashSet::new());
            let types = super::declared::codex_params(c, method, Some(class));
            for (p, ty) in params.iter().zip(&types) {
                let name = String::from_utf8_lossy(&p.variable.name[1..]);
                if !doc.params.contains_key(name.as_ref()) {
                    reference_type(ty, &inherited);
                }
            }
            if doc.ret.is_none() {
                let native = return_type.map_or_else(pun_phpstan::types::mixed, |r| {
                    super::declared::native(c, &r.hint, Some(class))
                });
                let ret = pun_phpstan::types::decide_type(
                    native,
                    super::declared::codex_return(c, method, Some(class)),
                );
                if let Type::Conditional(conditional) = &ret
                    && !conditional.negated
                    && matches!(
                        conditional.subject,
                        pun_phpstan::types::Subject::Parameter(_)
                    )
                {
                    reference_type(&ret, &inherited);
                }
            }
            inherited.into_inner()
        });
        seen.extend(inherited);
    }
    let of = kind.named();
    let mut reported = HashSet::new();
    for (name, ..) in &doc.templates {
        if !seen.contains(name) && reported.insert(name.clone()) {
            c.report(
                at,
                "method.templateTypeNotInParameter",
                format!("Template type {name} of {of} is not referenced in a parameter."),
                None,
            );
        }
    }
}

/// FunctionDefinitionCheck's classes of the PHPDoc parameter and return types once `checkThisOnly` is off (level 2;
/// the native ones are names.rs's).
pub fn phpdoc_classes(
    c: &mut Checker,
    kind: FunctionKind,
    at: Span,
    parameters: &FunctionLikeParameterList,
    return_type: Option<&FunctionLikeReturnTypeHint>,
    types: &super::declared::PhpDocTypes,
) {
    let params = parameters
        .parameters
        .iter()
        .map(|p| (String::from_utf8_lossy(&p.variable.name[1..]).into_owned(), p.span()))
        .collect();
    phpdoc_classes_at(c, kind, params, return_type.map_or(at, |r| r.hint.span()), types);
}

/// [`phpdoc_classes()`] of parameters (name without `$`, node) and a return type at `ret_at`.
fn phpdoc_classes_at(
    c: &mut Checker,
    kind: FunctionKind,
    params: Vec<(String, Span)>,
    ret_at: Span,
    types: &super::declared::PhpDocTypes,
) {
    let (of, subject) = (kind.named(), kind.subject());
    // then ClassCaseSensitivityCheck on the same classes, at the parameter or the return type
    let report = |c: &mut Checker,
                  at: Span,
                  types: &[Type],
                  trait_id: &str,
                  message: &dyn Fn(&str) -> String,
                  location: Option<super::class_names::Location>| {
        // (no class name has a `-`: a keyword type the resolver leaves, such as an unresolved `value-of`)
        let classes: Vec<String> =
            types.iter().flat_map(super::generics::referenced_classes).filter(|n| !n.contains('-')).collect();
        for class in &classes {
            let identifier = match known(c, class) {
                None => "class.notFound",
                Some(Known::User(k)) if k.kind == mago_codex::symbol::SymbolKind::Trait => trait_id,
                Some(_) => continue,
            };
            c.report(at, identifier, message(class), None);
        }
        let pairs: Vec<Pair> = classes.into_iter().map(|class| Pair::node(class, at)).collect();
        let case = c.case_sensitivity;
        class_names::check_at(c, &pairs, location.as_ref(), case);
    };
    for (name, at) in params {
        let types = types.params.get(&name).map_or(&[][..], Vec::as_slice);
        let location = super::names::location_function(c, kind).map(|function| super::class_names::Location::ParameterType {
            parameter: name.clone(),
            function,
        });
        report(c, at, types, "parameter.trait", &|class| format!("Parameter ${name} of {of} has invalid type {class}."), location);
    }
    let location = super::names::location_function(c, kind).map(super::class_names::Location::ReturnType);
    report(c, ret_at, types.ret.as_slice(), "return.trait", &|class| format!("{subject} has invalid return type {class}."), location);
}

/// [`phpdoc_classes()`] of property `name`'s hooks (ExistingClassesInPropertyHookTypehintsRule): a get hook returns
/// the property's PHPDoc type, a set hook's parameter has its own `@param` type, else the property's. `decl` is
/// where the property's PHPDoc is read from.
pub fn hook_phpdoc_classes(c: &mut Checker, name: &[u8], list: &PropertyHookList, decl: Span) {
    if c.level < 2 {
        return;
    }
    let (context, display) = c.member_scope();
    let property = String::from_utf8_lossy(&name[1..]).into_owned();
    let var = super::declared::with_classes(c, || super::declared::property(c, None, decl, &name[1..], context));
    for hook in list.hooks.iter() {
        let hook_name = String::from_utf8_lossy(hook.name.value).to_ascii_lowercase();
        let kind = FunctionKind::Hook(&display, &property, &hook_name);
        let mut types = super::declared::PhpDocTypes { params: HashMap::new(), ret: None };
        let params = if hook_name == "set" {
            let (param, at) = hook
                .parameter_list
                .as_ref()
                .and_then(|l| l.parameters.first())
                .map_or(("value".to_string(), hook.span()), |p| {
                    (String::from_utf8_lossy(&p.variable.name[1..]).into_owned(), p.span())
                });
            let own = doc_before(&c.file.contents, hook.span().start.offset as usize).and_then(|doc| {
                let scope = super::declared::scope_at(c, hook.span(), Some(context));
                super::declared::with_classes(c, || {
                    super::doc::Doc::parse(doc).param_types(&scope).remove(&param).map(|(t, _)| t)
                })
            });
            types.params.insert(param.clone(), vec![own.unwrap_or_else(|| var.clone())]);
            vec![(param, at)]
        } else {
            types.ret = Some(var.clone());
            Vec::new()
        };
        phpdoc_classes_at(c, kind, params, hook.span(), &types);
    }
}

/// checkClassMethod()'s `@phpstan-self-out` classes.
fn self_out(c: &mut Checker, at: Span, subject: &str, doc: &Doc) {
    let Some(node) = &doc.self_out else {
        return;
    };
    let class_templates = c
        .class_scope()
        .into_iter()
        .flat_map(|k| k.template_types.keys());
    let templates: Vec<String> = doc
        .templates
        .iter()
        .map(|(t, ..)| t.clone())
        .chain(class_templates.map(|t| String::from_utf8_lossy(t.as_bytes()).into_owned()))
        .collect();
    let mut classes = Vec::new();
    referenced(c, node, &templates, &mut classes);
    let pairs: Vec<Pair> = classes
        .iter()
        .map(|class| Pair::node(class.clone(), at))
        .collect();
    for class in classes {
        let identifier = match known(c, &class) {
            None => "class.notFound",
            Some(Known::User(k)) if k.kind == mago_codex::symbol::SymbolKind::Trait => {
                "selfOut.trait"
            }
            Some(_) => continue,
        };
        c.report(
            at,
            identifier,
            format!("{subject} has invalid @phpstan-self-out type {class}."),
            None,
        );
    }
    let case = c.case_sensitivity;
    class_names::check(c, &pairs, case);
}

/// checkImplicitlyNullableType() and checkRequiredParameterAfterOptional().
fn parameter_defaults(c: &mut Checker, parameters: &FunctionLikeParameterList) {
    let params: Vec<&FunctionLikeParameter> = parameters.parameters.iter().collect();
    for (i, p) in params.iter().enumerate() {
        implicitly_nullable(c, p, i + 1);
    }
    if c.php_version >= 80000 {
        required_after_optional(c, &params);
    }
}

/// checkImplicitlyNullableType().
fn implicitly_nullable(c: &mut Checker, p: &FunctionLikeParameter, order: usize) {
    let Some(hint) = &p.hint else {
        return;
    };
    if !null_default(p)
        || matches!(
            hint,
            Hint::Nullable(_) | Hint::Intersection(_) | Hint::Mixed(_) | Hint::Null(_)
        )
        || c.php_version < 80400
    {
        return;
    }
    let mut members = Vec::new();
    union_members(hint, &mut members);
    if members.iter().any(|m| matches!(m, Hint::Null(_))) {
        return;
    }
    let name = String::from_utf8_lossy(p.variable.name).into_owned();
    let printed = print_type(c, hint);
    c.report(
        p,
        "parameter.implicitlyNullable",
        format!(
            "Deprecated in PHP 8.4: Parameter #{order} {name} ({printed}) is implicitly nullable via default value null."
        ),
        None,
    );
}

/// checkRequiredParameterAfterOptional().
fn required_after_optional(c: &mut Checker, params: &[&FunctionLikeParameter]) {
    let mut optional: Option<String> = None;
    let mut target: Option<&str> = None;
    for p in params {
        let name = String::from_utf8_lossy(p.variable.name).into_owned();
        if let Some(previous) = &optional
            && p.default_value.is_none()
            && p.ellipsis.is_none()
        {
            c.report(
                *p,
                "parameter.requiredAfterOptional",
                format!(
                    "Deprecated in PHP {}: Required parameter {name} follows optional parameter {previous}.",
                    target.unwrap_or("8.0")
                ),
                None,
            );
            target = None;
            continue;
        }
        if p.default_value.is_none() {
            continue;
        }
        let Some(hint) = &p.hint else {
            optional = Some(name);
            continue;
        };
        if null_default(p) {
            if c.php_version < 80100 {
                continue;
            }
            if matches!(hint, Hint::Nullable(_)) {
                target = Some("8.1");
            }
            if c.php_version >= 80300 {
                let mut members = Vec::new();
                union_members(hint, &mut members);
                if members
                    .iter()
                    .any(|m| matches!(m, Hint::Null(_) | Hint::Mixed(_)))
                {
                    target = Some("8.3");
                }
            }
            if target.is_none() {
                continue;
            }
        }
        optional = Some(name);
    }
}

/// ExistingClassesInArrowFunctionTypehintsRule: a `never` return type before PHP 8.2.
pub fn arrow_never(c: &mut Checker, f: &ArrowFunction) {
    if !c.surely_at_least(80200)
        && matches!(
            f.return_type_hint.as_ref().map(|r| &r.hint),
            Some(Hint::Never(_))
        )
    {
        c.report_non_ignorable(
            f,
            "return.neverTypeNotSupported",
            "Never return type in arrow function is supported only on PHP 8.2 and later."
                .to_string(),
            None,
        );
    }
}

/// ExistingClassesInPropertiesRule's unresolvable native type, for one property declaration or promoted parameter.
pub fn property<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: Span,
    hint: Option<&Hint<'arena>>,
    names: &[&[u8]],
) {
    let (Some(hint), Some(class)) = (hint, c.class_scope()) else {
        return;
    };
    if c.php_version < 80100 || !unresolvable(c, hint) {
        return;
    }
    let display = c.display(class);
    for name in names {
        let property =
            String::from_utf8_lossy(name.strip_prefix(b"$").unwrap_or(name)).into_owned();
        c.report(
            at,
            "property.unresolvableNativeType",
            format!("Property {display}::${property} has unresolvable native type."),
            None,
        );
    }
}

/// ExistingClassesInPropertyHookTypehintsRule: FunctionDefinitionCheck on each hook of property `name`.
pub fn hooks<'arena>(c: &mut Checker<'_, 'arena>, name: &[u8], list: &PropertyHookList<'arena>) {
    // properties are walked inside their class-like (trait bodies only in a class's context)
    let class = c
        .class_scope()
        .expect("a property hook is walked inside its class-like");
    let display = c.display(class);
    let property = String::from_utf8_lossy(name.strip_prefix(b"$").unwrap_or(name)).into_owned();
    for hook in list.hooks.iter() {
        let Some(params) = &hook.parameter_list else {
            continue;
        };
        let hook_name = String::from_utf8_lossy(hook.name.value).to_ascii_lowercase();
        let kind = FunctionKind::Hook(&display, &property, &hook_name);
        super::names::typehints(c, kind, params, None);
        check(c, kind, hook.span(), params, None);
    }
}
