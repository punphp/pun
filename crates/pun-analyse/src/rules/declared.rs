//! What PHPStan's reflection says a declaration's types are: native types (ParserNodeTypeToPHPStanType), PHPDoc
//! types (FileTypeMapper, with PhpDocInheritanceResolver's inherited tags) and the type that wins
//! (TypehintHelper::decideType()), as `pun_phpstan::types::Type`. Also the class reflection the type model asks
//! about the project's classes ([`with_classes()`]).

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::flags::MetadataFlags;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_database::file::FileId;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::resolve::resolve_in;
use pun_phpstan::symbols::{self, Class, Template};
use pun_phpstan::types::{
    NeverKind, Template as TemplateType, Type, decide_type, intersect, mixed, to_phpdoc, union,
};

use super::Checker;
use super::doc::doc_before;
use super::doc::{Doc, FileScopes, NameScope, scopes_of};

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// A class-like's name as declared.
pub fn original_name(k: &ClassLikeMetadata) -> String {
    text(k.original_name.as_bytes())
}

/// A file's contents, PHPStan's API stub's too (no file on disk).
fn file_contents<'a>(c: &Checker<'a, '_>, id: FileId) -> Option<std::borrow::Cow<'a, [u8]>> {
    c.file_contents(id).or_else(|| {
        (id == FileId::new(b"phpstan-api.php"))
            .then_some(std::borrow::Cow::Borrowed(crate::codebase::PHPSTAN_API))
    })
}

/// The doc comment before `offset` in the file `span` is in, with that file's contents.
pub fn doc_at(c: &Checker, span: Span) -> Option<(Vec<u8>, Arc<FileScopes>)> {
    let contents = file_contents(c, span.file_id)?;
    let doc = doc_before(&contents, span.start.offset as usize)?.to_vec();
    Some((doc, scopes_of(span.file_id, &contents)))
}

/// The `NameScope` of a PHPDoc at `span` in class-like `class` (`None` outside one), with `templates` in scope.
pub fn scope_at(c: &Checker, span: Span, class: Option<&ClassLikeMetadata>) -> NameScope {
    let names = c
        .file_contents(span.file_id)
        .map(|contents| scopes_of(span.file_id, &contents).at(span.start.offset))
        .unwrap_or_default();
    let mut scope = NameScope {
        names,
        class: class.map(original_name),
        parent: class.and_then(|k| parent_name(c.codebase, k)),
        templates: HashMap::new(),
    };
    if let Some(k) = class {
        for t in class_templates(c, k) {
            scope
                .templates
                .insert(t.name.clone(), Type::Template(Box::new(t)));
        }
        scope.templates.extend(super::type_aliases::class_aliases(c, k));
    }
    scope
}

pub(super) fn parent_name(codebase: &CodebaseMetadata, k: &ClassLikeMetadata) -> Option<String> {
    let parent = k.direct_parent_class.as_ref()?;
    Some(
        codebase
            .get_class_like(parent.as_bytes())
            .map_or_else(|| text(parent.as_bytes()), original_name),
    )
}

/// `TemplateTypeFactory::fromTemplateTag()` for each `@template` of a PHPDoc, in `describe` scope (`class Foo`,
/// `method Foo::bar()`, `function foo()`).
fn templates(doc: &Doc, scope: &NameScope, describe: &str) -> Vec<TemplateType> {
    let resolve_all = |scope: &NameScope| -> Vec<TemplateType> {
        doc.templates
            .iter()
            .map(|t| TemplateType {
                name: t.name.clone(),
                scope: describe.to_string(),
                bound: t
                    .bound
                    .as_ref()
                    .map_or(Type::Mixed { explicit: true }, |b| resolve_in(b, scope)),
                default: t.default.as_ref().map(|d| resolve_in(d, scope)),
                argument: false,
            })
            .collect()
    };
    // a bound may name the PHPDoc's other templates: resolve with them unbounded, then with their bounds
    let mut scope = scope.clone();
    for t in &doc.templates {
        let placeholder = TemplateType {
            name: t.name.clone(),
            scope: describe.to_string(),
            bound: Type::Mixed { explicit: true },
            default: None,
            argument: false,
        };
        scope
            .templates
            .insert(t.name.clone(), Type::Template(Box::new(placeholder)));
    }
    // (each pass resolves one more level of bounds naming bounds)
    let mut resolved = resolve_all(&scope);
    for _ in 0..doc.templates.len() {
        for t in &resolved {
            scope
                .templates
                .insert(t.name.clone(), Type::Template(Box::new(t.clone())));
        }
        let next = resolve_all(&scope);
        if next == resolved {
            break;
        }
        resolved = next;
    }
    resolved
}

/// A class-like's own `@template` types.
pub(super) fn class_templates(c: &Checker, k: &ClassLikeMetadata) -> Vec<TemplateType> {
    let Some((doc, scopes)) = doc_at(c, k.span) else {
        return Vec::new();
    };
    let doc = Doc::parse(&doc);
    if doc.templates.is_empty() {
        return Vec::new();
    }
    let scope = NameScope {
        names: scopes.at(k.span.start.offset),
        class: Some(original_name(k)),
        parent: parent_name(c.codebase, k),
        templates: HashMap::new(),
    };
    templates(&doc, &scope, &format!("class {}", original_name(k)))
}

/// FileTypeMapper's template types of trait `t` used by `context`: the `@use T<...>` arguments of the `use`
/// statement (`typeMapFromList()`), else the templates' bounds.
fn trait_templates(
    c: &Checker,
    t: &ClassLikeMetadata,
    context: &ClassLikeMetadata,
) -> Vec<(String, Type)> {
    let own = class_templates(c, t);
    if own.is_empty() {
        return Vec::new();
    }
    let trait_name = original_name(t);
    let mut args: Option<Vec<Type>> = None;
    // (the class's file is an analysed one)
    let contents = file_contents(c, context.span.file_id).unwrap_or_default();
    let scopes = scopes_of(context.span.file_id, &contents);
    let scope = scope_at(c, context.span, Some(context));
    for (offset, names) in scopes.trait_uses(context.span.start.offset, context.span.end.offset) {
        let names_here = scopes.at(*offset);
        let uses_it = names.iter().any(|n| {
            let (resolved, _) =
                names_here.resolve(mago_names::kind::NameKind::Default, n.as_bytes());
            resolved.eq_ignore_ascii_case(trait_name.as_bytes())
        });
        let Some(doc) = uses_it
            .then(|| doc_before(&contents, *offset as usize))
            .flatten()
        else {
            continue;
        };
        for node in &Doc::parse(doc).uses {
            if let Type::Object(name, found) = resolve_in(node, &scope)
                && !found.is_empty()
                && name.eq_ignore_ascii_case(&trait_name)
            {
                args = Some(found);
            }
        }
    }
    own.into_iter()
        .enumerate()
        .map(|(i, tpl)| {
            let ty = match &args {
                Some(args) => args
                    .get(i)
                    .cloned()
                    .or_else(|| tpl.default.clone())
                    .unwrap_or_else(|| tpl.bound.clone()),
                None => tpl.bound.clone(),
            };
            (tpl.name, ty)
        })
        .collect()
}

/// `ParserNodeTypeToPHPStanType::resolve()` of a native type in the analysed file, in class `class`.
pub fn native(c: &Checker, hint: &Hint, class: Option<&ClassLikeMetadata>) -> Type {
    match hint {
        Hint::Identifier(id) => Type::Object(text(c.names.get(&id.span())), vec![]),
        Hint::Parenthesized(p) => native(c, p.hint, class),
        Hint::Nullable(n) => native(c, n.hint, class).add_null(),
        Hint::Union(u) => {
            let mut members = Vec::new();
            let mut stack = vec![u.right, u.left];
            while let Some(h) = stack.pop() {
                match h {
                    Hint::Union(inner) => stack.extend([inner.right, inner.left]),
                    h => members.push(native(c, h, class)),
                }
            }
            union(members)
        }
        Hint::Intersection(i) => {
            let mut members = Vec::new();
            let mut stack = vec![i.right, i.left];
            while let Some(h) = stack.pop() {
                match h {
                    Hint::Intersection(inner) => stack.extend([inner.right, inner.left]),
                    h => members.push(native(c, h, class)),
                }
            }
            // ParserNodeTypeToPHPStanType: a member that is not an object makes the intersection `never`
            if !members.iter().all(|m| matches!(m, Type::Object(..) | Type::Static(..) | Type::ObjectWithoutClass)) {
                return Type::Never(NeverKind::Implicit);
            }
            intersect(members)
        }
        Hint::Null(_) => Type::Null,
        Hint::True(_) => Type::ConstBool(true),
        Hint::False(_) => Type::ConstBool(false),
        Hint::Array(_) => Type::array(mixed(), mixed()),
        Hint::Callable(_) => Type::Callable(None),
        Hint::Static(_) => match class {
            Some(k) => Type::Static(original_name(k), vec![]),
            None => Type::Object("static".to_string(), vec![]),
        },
        Hint::Self_(_) => match class {
            Some(k) => Type::Object(original_name(k), vec![]),
            None => Type::Object("self".to_string(), vec![]),
        },
        Hint::Parent(_) => match class.and_then(|k| parent_name(c.codebase, k)) {
            Some(parent) => Type::Object(parent, vec![]),
            None => Type::Object("parent".to_string(), vec![]),
        },
        Hint::Void(_) => Type::Void,
        Hint::Never(_) => Type::Never(NeverKind::Explicit),
        Hint::Float(_) => Type::Float,
        Hint::Bool(_) => Type::Bool,
        Hint::Integer(_) => Type::Int,
        Hint::String(_) => Type::String,
        Hint::Object(_) => Type::ObjectWithoutClass,
        Hint::Mixed(_) => Type::Mixed { explicit: true },
        Hint::Iterable(_) => Type::Iterable(Box::new(mixed()), Box::new(mixed())),
    }
}

/// `getType()` of each parameter of a function-like from the codex (a callee in any file): its native type decided
/// with its PHPDoc type (a method's inherited one too), `declaring` being the class-like declaring a method.
pub fn codex_params(c: &Checker, f: &FunctionLikeMetadata, declaring: Option<&ClassLikeMetadata>) -> Vec<Type> {
    let doc = match declaring {
        Some(k) => method_doc(c, k, &f.name.as_bytes().to_ascii_lowercase(), 0),
        None => doc_at(c, f.span).map(|(doc, _)| {
            let doc = Doc::parse(&doc);
            let mut scope = scope_at(c, f.span, None);
            let name = text(f.original_name.as_bytes());
            for t in templates(&doc, &scope, &format!("function {name}()")) {
                scope.templates.insert(t.name.clone(), Type::Template(Box::new(t)));
            }
            Resolved {
                params: doc.param_types(&scope).into_iter().map(|(k, (t, _))| (k, t)).collect(),
                ..Resolved::default()
            }
        }),
    }
    .unwrap_or_default();
    f.parameters
        .iter()
        .map(|p| {
            let name = text(&p.name.0.as_bytes()[1..]);
            // `isParameterValueNullable()` of a parameter that is no promoted property
            let null_default = !p.flags.is_promoted_property()
                && p.default_type.as_ref().is_some_and(|d| d.type_union.is_null());
            let native = p.type_declaration_metadata.as_ref().map(|t| {
                let t = from_tunion_in(c, &t.type_union, declaring);
                if null_default { t.add_null() } else { t }
            });
            let mut phpdoc = doc.params.get(&name).cloned();
            if p.default_type.as_ref().is_some_and(|d| d.type_union.is_null())
                && let Some(t) = &phpdoc
                && !has_template(t)
            {
                phpdoc = Some(t.clone().add_null());
            }
            match native {
                // decideTypeFromReflection(null, ...): a variadic's array PHPDoc type is its item type
                None if p.flags.is_variadic() => match phpdoc {
                    Some(Type::Array(_, item)) => *item,
                    Some(t @ Type::ConstArray(_)) => t.iterable_value_type(),
                    t => t.unwrap_or_else(mixed),
                },
                None => phpdoc.unwrap_or_else(mixed),
                Some(native) => decide_type(native, phpdoc),
            }
        })
        .collect()
}

/// `TypehintHelper::decideTypeFromReflection()` of a native type declaration Mago's codex holds (the `TUnion` its
/// scanner built from the hint), declared in class-like `class`: what [`native()`] gives for the hint itself.
pub fn from_tunion(
    c: &Checker,
    t: &mago_codex::ttype::union::TUnion,
    class: &ClassLikeMetadata,
) -> Type {
    from_tunion_in(c, t, Some(class))
}

/// [`from_tunion()`] in a class-like or not (a function's).
fn from_tunion_in(
    c: &Checker,
    t: &mago_codex::ttype::union::TUnion,
    class: Option<&ClassLikeMetadata>,
) -> Type {
    use mago_codex::ttype::union::TUnion;
    use mago_codex::ttype::atomic::TAtomic;
    use mago_codex::ttype::atomic::callable::TCallable;
    use mago_codex::ttype::atomic::object::TObject;
    use mago_codex::ttype::atomic::reference::TReference;
    use mago_codex::ttype::atomic::scalar::TScalar;
    let atom = |a: &TAtomic| match a {
        TAtomic::Null => Type::Null,
        TAtomic::Void => Type::Void,
        TAtomic::Never => Type::Never(NeverKind::Explicit),
        TAtomic::Scalar(TScalar::Bool(b)) => b.value.map_or(Type::Bool, Type::ConstBool),
        TAtomic::Scalar(TScalar::Integer(_)) => Type::Int,
        TAtomic::Scalar(TScalar::Float(_)) => Type::Float,
        TAtomic::Scalar(TScalar::String(_)) => Type::String,
        TAtomic::Array(_) => Type::array(mixed(), mixed()),
        TAtomic::Iterable(_) => Type::Iterable(Box::new(mixed()), Box::new(mixed())),
        TAtomic::Callable(TCallable::Signature(s)) if s.is_closure => {
            Type::Object("Closure".to_string(), vec![])
        }
        TAtomic::Callable(_) => Type::Callable(None),
        TAtomic::Object(TObject::Named(n)) => {
            let name = text(n.name.as_bytes());
            let ty = if n.is_static {
                Type::Static(name, vec![])
            } else if name.eq_ignore_ascii_case("parent") {
                Type::Object(class.and_then(|k| parent_name(c.codebase, k)).unwrap_or(name), vec![])
            } else {
                Type::Object(name, vec![])
            };
            match &n.intersection_types {
                None => ty,
                Some(more) => intersect(
                    std::iter::once(ty)
                        .chain(more.iter().map(|m| from_tunion_in(c, &TUnion::from_atomic(m.clone()), class)))
                        .collect(),
                ),
            }
        }
        TAtomic::Reference(TReference::Symbol { name, .. }) => {
            Type::Object(text(name.as_bytes()), vec![])
        }
        TAtomic::Object(TObject::Any) => Type::ObjectWithoutClass,
        _ => Type::Mixed { explicit: true },
    };
    union(t.types.iter().map(atom).collect())
}

/// A function-like's types as its reflection gives them.
pub struct Signature {
    /// (name, `getType()`, `getOutType()`, `getClosureThisType()`)
    pub params: Vec<(String, Type, Option<Type>, Option<Type>)>,
    pub ret: Type,
    /// `getSelfOutType()`
    pub self_out: Option<Type>,
    /// the parameters of `getAsserts()`, its own or inherited ones (without `$`)
    pub asserts: Vec<String>,
}

/// The PHPDoc of a method as PHPStan resolves it, with what it inherits: parameter types by name, the return type.
#[derive(Clone, Default)]
struct Resolved {
    params: HashMap<String, Type>,
    ret: Option<Type>,
    /// `@param-out` and `@param-closure-this` types by parameter name
    outs: HashMap<String, Type>,
    closure_this: HashMap<String, Type>,
    /// `@phpstan-self-out`
    self_out: Option<Type>,
    /// the asserted parameters (without `$`)
    asserts: Vec<String>,
}

/// The own PHPDoc of method `m` declared in `class`, `self` being `context`.
fn resolved_own(
    c: &Checker,
    m: &FunctionLikeMetadata,
    class: &ClassLikeMetadata,
    context: &ClassLikeMetadata,
) -> Option<Resolved> {
    let (raw, _) = doc_at(c, m.span)?;
    let doc = Doc::parse(&raw);
    let mut scope = scope_at(c, m.span, Some(context));
    if class.kind == SymbolKind::Trait && !std::ptr::eq(class, context) {
        scope.templates.extend(trait_templates(c, class, context));
    }
    let method = text(m.original_name.as_bytes());
    for t in templates(
        &doc,
        &scope,
        &format!("method {}::{method}()", original_name(class)),
    ) {
        scope
            .templates
            .insert(t.name.clone(), Type::Template(Box::new(t)));
    }
    Some(Resolved {
        params: doc
            .param_types(&scope)
            .into_iter()
            .map(|(k, (t, _))| (k, t))
            .collect(),
        ret: doc.return_type(&scope),
        outs: Doc::named_types(&doc.outs, &scope),
        closure_this: Doc::named_types(&doc.closure_this, &scope),
        self_out: doc.self_out.as_ref().map(|n| resolve_in(n, &scope)),
        asserts: super::assert_tags::parameters(&raw),
    })
}

/// `getResolvedPhpDoc()` of method `name` declared in `class` (its own PHPDoc merged with the inherited one), at
/// most `depth` ancestors up.
fn method_doc(
    c: &Checker,
    class: &ClassLikeMetadata,
    name: &[u8],
    depth: usize,
) -> Option<Resolved> {
    let m = c.codebase.get_method(class.name.as_bytes(), name)?;
    if class.flags.contains(MetadataFlags::BUILTIN) {
        return builtin_method_doc(class, m, name);
    }
    let own = if m.has_docblock {
        resolved_own(c, m, class, class)
    } else {
        None
    };
    let names: Vec<String> = m
        .parameters
        .iter()
        .map(|p| text(&p.name.0.as_bytes()[1..]))
        .collect();
    inherit(c, class, name, own, &names, depth)
}

/// A built-in method's resolved PHPDoc, from PHPStan's reflection (its stubs), keyed by Mago's parameter names.
fn builtin_method_doc(
    class: &ClassLikeMetadata,
    m: &FunctionLikeMetadata,
    name: &[u8],
) -> Option<Resolved> {
    let k = symbols::class(&text(class.name.as_bytes()))?;
    let doc = k
        .methods
        .get(&text(name).to_ascii_lowercase())?
        .doc
        .as_ref()?;
    let resolve = |s: &str| pun_phpstan::resolve::resolve_str(s, Some(&k.name));
    Some(Resolved {
        params: doc
            .params
            .iter()
            .filter_map(|(i, t)| {
                let p = m.parameters.get(*i)?;
                Some((text(&p.name.0.as_bytes()[1..]), resolve(t)))
            })
            .collect(),
        ret: doc.ret.as_deref().map(resolve),
        ..Resolved::default()
    })
}

/// `PhpDocInheritanceResolver::resolvePhpDocForMethod()`: the PHPDoc `current` with what the parent class's,
/// else the first interface's, else an abstract trait method's PHPDoc adds.
fn inherit(
    c: &Checker,
    class: &ClassLikeMetadata,
    name: &[u8],
    current: Option<Resolved>,
    positional: &[String],
    depth: usize,
) -> Option<Resolved> {
    if depth > 32 {
        return current;
    }
    let lower = name.to_ascii_lowercase();
    let from = |parent: &ClassLikeMetadata| -> Option<Resolved> {
        let declaring = declaring_method(c.codebase, parent, &lower)?;
        if declaring
            .1
            .method_metadata
            .as_ref()
            .is_some_and(|m| m.visibility == mago_codex::visibility::Visibility::Private)
        {
            return None;
        }
        // (a PHPDoc at all: `getResolvedPhpDoc() !== null`)
        let parent_doc = method_doc(c, declaring.0, &lower, depth + 1)?;
        let parent_doc = {
            let map = |t: Type| ancestor_types(class, declaring.0, &t);
            Resolved {
                params: parent_doc.params.into_iter().map(|(k, t)| (k, map(t))).collect(),
                ret: parent_doc.ret.map(map),
                outs: parent_doc.outs.into_iter().map(|(k, t)| (k, map(t))).collect(),
                closure_this: parent_doc.closure_this.into_iter().map(|(k, t)| (k, map(t))).collect(),
                ..parent_doc
            }
        };
        Some(merge(
            current.clone(),
            parent_doc,
            positional,
            &declaring.1.parameters,
            is_constructor(declaring.0, &lower),
        ))
    };
    if let Some(parent) = class
        .direct_parent_class
        .as_ref()
        .and_then(|p| c.codebase.get_class_like(p.as_bytes()))
        && let Some(found) = from(parent)
        && !(lower == b"__construct" && parent.flags.contains(MetadataFlags::BUILTIN))
    {
        return Some(found);
    }
    // Mago gives a backed enum an internal `__internal_do_not_use__IntBackedEnum` (or String) interface, which
    // stands for BackedEnum in PHPStan's reflection
    let interfaces = class.direct_parent_interfaces.iter().map(|i| {
        if i.as_bytes().starts_with(b"__internal_do_not_use__") { &b"BackedEnum"[..] } else { i.as_bytes() }
    });
    for interface in interfaces {
        if let Some(i) = c.codebase.get_class_like(interface)
            && let Some(found) = from(i)
        {
            return Some(found);
        }
    }
    // an abstract method with a PHPDoc in one of the class's own traits
    for t in class.used_traits.iter() {
        let (Some(trait_), Some(m)) = (
            c.codebase.get_class_like(t.as_bytes()),
            c.codebase.get_method(t.as_bytes(), &lower),
        ) else {
            continue;
        };
        let abstract_with_doc =
            m.has_docblock && m.method_metadata.as_ref().is_some_and(|mm| mm.is_abstract);
        let Some(own) = abstract_with_doc
            .then(|| resolved_own(c, m, trait_, class))
            .flatten()
        else {
            continue;
        };
        return Some(merge(
            current,
            own,
            positional,
            &m.parameters,
            is_constructor(trait_, &lower),
        ));
    }
    current
}

/// The class-like whose method `lower` `parent` has natively (declared, inherited or from a trait), and the method.
fn declaring_method<'c>(
    codebase: &'c CodebaseMetadata,
    parent: &ClassLikeMetadata,
    lower: &[u8],
) -> Option<(&'c ClassLikeMetadata, &'c FunctionLikeMetadata)> {
    let id = parent.declaring_method_ids.get(&mago_word::word(lower))?;
    let m = codebase.get_method_by_id(id)?;
    if m.flags.contains(MetadataFlags::MAGIC_METHOD) {
        return None;
    }
    let k = codebase.get_class_like(id.get_class_name().as_bytes())?;
    Some((k, m))
}

/// `ResolvedPhpDocBlock::merge()` of the parameter and return tags: the parent's tags fill what `current` lacks,
/// with the parent's parameter names mapped to the child's by position.
/// `resolveTemplateTypeInTag()`: an inherited PHPDoc type `t` of ancestor `declaring`, with its templates as
/// `class`'s `@extends`/`@implements` tags (or their defaults) give them.
fn ancestor_types(class: &ClassLikeMetadata, declaring: &ClassLikeMetadata, t: &Type) -> Type {
    let (Some(k), Some(d)) = (symbols::lookup(&original_name(class)), symbols::lookup(&original_name(declaring))) else {
        return t.clone();
    };
    let Some(args) = k.generics.get(&d.name) else {
        return t.clone();
    };
    let args: Vec<Type> = args.iter().map(|s| pun_phpstan::resolve::resolve_str(s, Some(&k.name))).collect();
    let scope = format!("class {}", d.name);
    fn map(t: &Type, scope: &str, d: &symbols::Class, args: &[Type]) -> Type {
        match t {
            Type::Template(tpl) if tpl.scope == scope => d
                .templates
                .iter()
                .position(|x| x.name == tpl.name)
                .and_then(|i| args.get(i))
                .cloned()
                .unwrap_or_else(|| t.clone()),
            _ => t.traverse(&|i| map(i, scope, d, args)),
        }
    }
    map(t, &scope, &d, &args)
}

fn merge(
    current: Option<Resolved>,
    parent: Resolved,
    positional: &[String],
    parent_params: &[mago_codex::metadata::parameter::FunctionLikeParameterMetadata],
    constructor: bool,
) -> Resolved {
    let mut out = current.unwrap_or_default();
    // a constructor's parameters map by name (`resolveMethodPhpDocFromParentClass()`)
    let parent_names: Vec<String> = if constructor {
        positional.to_vec()
    } else {
        parent_params
            .iter()
            .map(|p| text(&p.name.0.as_bytes()[1..]))
            .collect()
    };
    let mapped = |name: &str| {
        parent_names
            .iter()
            .position(|n| n == name)
            .and_then(|i| positional.get(i))
            .cloned()
    };
    let rename = |name: String| mapped(&name).unwrap_or(name);
    // `transformConditionalReturnTypeWithParameterNameMapping()`
    let types = |t: Type| rename_conditionals(&t, &mapped);
    if out.ret.is_none() {
        out.ret = parent.ret.map(types);
    }
    for (name, ty) in parent.params {
        out.params.entry(rename(name)).or_insert_with(|| types(ty));
    }
    if out.self_out.is_none() {
        out.self_out = parent.self_out;
    }
    // `mergeAssertTags()`
    if out.asserts.is_empty() {
        out.asserts = parent.asserts.into_iter().map(rename).collect();
    }
    for (name, ty) in parent.outs {
        out.outs.entry(rename(name)).or_insert(ty);
    }
    for (name, ty) in parent.closure_this {
        out.closure_this.entry(rename(name)).or_insert_with(|| types(ty));
    }
    out
}

/// The conditional types on a parameter `mapped` names differently, renamed.
fn rename_conditionals(t: &Type, mapped: &dyn Fn(&str) -> Option<String>) -> Type {
    let t = match t {
        Type::Conditional(c) => match &c.subject {
            pun_phpstan::types::Subject::Parameter(p) => match mapped(&p[1..]) {
                Some(name) => Type::Conditional(Box::new(pun_phpstan::types::Conditional {
                    subject: pun_phpstan::types::Subject::Parameter(format!("${name}")),
                    ..(**c).clone()
                })),
                None => t.clone(),
            },
            _ => t.clone(),
        },
        _ => t.clone(),
    };
    t.traverse(&|i| rename_conditionals(i, mapped))
}

/// The `NameScope` of method `m`'s own PHPDoc `doc`: the class's templates (a trait's as `context` uses it) and
/// the method's.
pub fn method_scope(
    c: &Checker,
    m: &Method,
    doc: &Doc,
    k: &ClassLikeMetadata,
    context: &ClassLikeMetadata,
) -> NameScope {
    let mut scope = scope_at(c, m.span(), Some(context));
    if k.kind == SymbolKind::Trait && !std::ptr::eq(k, context) {
        scope.templates.extend(trait_templates(c, k, context));
    } else {
        for t in class_templates(c, k) {
            scope
                .templates
                .insert(t.name.clone(), Type::Template(Box::new(t)));
        }
    }
    let method = text(m.name.value);
    for t in templates(
        doc,
        &scope,
        &format!("method {}::{method}()", original_name(context)),
    ) {
        scope
            .templates
            .insert(t.name.clone(), Type::Template(Box::new(t)));
    }
    scope
}

/// The `NameScope` of function `f`'s own PHPDoc `doc`, with its templates.
pub fn function_scope(c: &Checker, f: &Function, doc: &Doc) -> NameScope {
    let mut scope = scope_at(c, f.span(), None);
    let name = text(c.names.get(&f.name.span()));
    for t in templates(doc, &scope, &format!("function {name}()")) {
        scope
            .templates
            .insert(t.name.clone(), Type::Template(Box::new(t)));
    }
    scope
}

/// The parameters and return type of a method declared in the analysed file, in class-like `class` (`k`, the one
/// it is declared in; `context`, the class a trait method is analysed for).
pub fn method(
    c: &Checker,
    m: &Method,
    k: &ClassLikeMetadata,
    context: &ClassLikeMetadata,
) -> Signature {
    let resolved = method_resolved(c, m, k, context);
    signature(
        c,
        Some(m.name.value),
        &m.parameter_list,
        m.return_type_hint.as_ref(),
        resolved,
        Some(context),
    )
}

/// The PHPDoc types FunctionDefinitionCheck reads from level 2: per parameter name its `@param`, `@param-out` and
/// `@param-closure-this` types (`getPhpDocType()`, `getOutType()`, `getClosureThisType()`), and `@return`'s.
pub struct PhpDocTypes {
    pub params: HashMap<String, Vec<Type>>,
    pub ret: Option<Type>,
}

impl From<Resolved> for PhpDocTypes {
    fn from(r: Resolved) -> PhpDocTypes {
        let mut params: HashMap<String, Vec<Type>> = HashMap::new();
        for (name, t) in r.params.into_iter().chain(r.outs).chain(r.closure_this) {
            params.entry(name).or_default().push(t);
        }
        PhpDocTypes { params, ret: r.ret }
    }
}

/// [`PhpDocTypes`] of a method declared in the analysed file (see [`method()`]).
pub fn method_phpdoc(c: &Checker, m: &Method, k: &ClassLikeMetadata, context: &ClassLikeMetadata) -> PhpDocTypes {
    let mut resolved = method_resolved(c, m, k, context);
    // a promoted property's `@var` is its parameter's PHPDoc type
    if m.name.value.eq_ignore_ascii_case(b"__construct") {
        for p in m.parameter_list.parameters.iter() {
            if let Some(t) = promoted_var(c, p, Some(context)) {
                resolved.params.insert(text(&p.variable.name[1..]), t);
            }
        }
    }
    resolved.into()
}

/// [`PhpDocTypes`] of a function declared in the analysed file.
pub fn function_phpdoc(c: &Checker, f: &Function) -> PhpDocTypes {
    function_resolved(c, f).into()
}

fn method_resolved(c: &Checker, m: &Method, k: &ClassLikeMetadata, context: &ClassLikeMetadata) -> Resolved {
    let own = doc_before(&c.file.contents, m.span().start.offset as usize).map(|raw| {
        let doc = Doc::parse(raw);
        let scope = method_scope(c, m, &doc, k, context);
        Resolved {
            params: doc
                .param_types(&scope)
                .into_iter()
                .map(|(k, (t, _))| (k, t))
                .collect(),
            ret: doc.return_type(&scope),
            outs: Doc::named_types(&doc.outs, &scope),
            closure_this: Doc::named_types(&doc.closure_this, &scope),
            self_out: doc.self_out.as_ref().map(|n| resolve_in(n, &scope)),
            asserts: super::assert_tags::parameters(raw),
        }
    });
    let positional: Vec<String> = m
        .parameter_list
        .parameters
        .iter()
        .map(|p| text(&p.variable.name[1..]))
        .collect();
    inherit(c, context, m.name.value, own, &positional, 0).unwrap_or_default()
}

/// The parameters and return type of a function declared in the analysed file.
pub fn function(c: &Checker, f: &Function) -> Signature {
    let resolved = function_resolved(c, f);
    signature(
        c,
        None,
        &f.parameter_list,
        f.return_type_hint.as_ref(),
        resolved,
        None,
    )
}

fn function_resolved(c: &Checker, f: &Function) -> Resolved {
    doc_before(&c.file.contents, f.span().start.offset as usize)
        .map(|doc| {
            let doc = Doc::parse(doc);
            let scope = function_scope(c, f, &doc);
            Resolved {
                params: doc
                    .param_types(&scope)
                    .into_iter()
                    .map(|(k, (t, _))| (k, t))
                    .collect(),
                ret: doc.return_type(&scope),
                outs: Doc::named_types(&doc.outs, &scope),
                closure_this: Doc::named_types(&doc.closure_this, &scope),
                self_out: doc.self_out.as_ref().map(|n| resolve_in(n, &scope)),
                asserts: Vec::new(),
            }
        })
        .unwrap_or_default()
}

fn signature(
    c: &Checker,
    method: Option<&[u8]>,
    params: &FunctionLikeParameterList,
    ret: Option<&FunctionLikeReturnTypeHint>,
    doc: Resolved,
    class: Option<&ClassLikeMetadata>,
) -> Signature {
    let params = params
        .parameters
        .iter()
        .map(|p| {
            let name = text(&p.variable.name[1..]);
            let native = p.hint.as_ref().map(|h| {
                let t = native(c, h, class);
                // `isParameterValueNullable($parameter) && $parameter->flags === 0`: not for a promoted property
                if default_is_null(p) && p.modifiers.is_empty() { t.add_null() } else { t }
            });
            let promoted = if method.is_some_and(|m| m.eq_ignore_ascii_case(b"__construct")) {
                promoted_var(c, p, class)
            } else {
                None
            };
            let mut phpdoc = promoted.or_else(|| doc.params.get(&name).cloned());
            // PhpParameterFromParserNodeReflection::getType(): a `null` default makes the PHPDoc type nullable
            // unless it infers a template type from it
            if default_is_null(p)
                && let Some(t) = &phpdoc
                && !has_template(t)
            {
                phpdoc = Some(t.clone().add_null());
            }
            let ty = decide_type(native.unwrap_or_else(mixed), phpdoc);
            let out = doc.outs.get(&name).cloned();
            let closure_this = doc.closure_this.get(&name).cloned();
            (name, ty, out, closure_this)
        })
        .collect();
    let native_ret = ret
        .map(|r| native(c, &r.hint, class))
        .or_else(|| method.and_then(magic_return));
    let ret = decide_type(native_ret.unwrap_or_else(mixed), doc.ret);
    Signature {
        params,
        ret,
        self_out: doc.self_out,
        asserts: doc.asserts,
    }
}

/// `PhpMethodReflection::getReturnType()` of a magic method without a native return type.
fn magic_return(name: &[u8]) -> Option<Type> {
    match name.to_ascii_lowercase().as_slice() {
        b"__construct" | b"__destruct" | b"__unset" | b"__wakeup" | b"__clone" | b"__set" | b"__unserialize" => {
            Some(Type::Void)
        }
        b"__serialize" => Some(Type::array(Type::Mixed { explicit: true }, Type::Mixed { explicit: true })),
        b"__debuginfo" => Some(Type::Union(vec![
            Type::array(Type::Mixed { explicit: true }, Type::Mixed { explicit: true }),
            Type::Null,
        ])),
        b"__tostring" => Some(Type::String),
        b"__isset" => Some(Type::Bool),
        b"__sleep" => Some(Type::array(Type::Int, Type::String)),
        b"__set_state" => Some(Type::ObjectWithoutClass),
        _ => None,
    }
}

/// `getPhpDocs()`: a promoted constructor parameter's own `@var` (the only unnamed one, else the one naming it).
fn promoted_var(
    c: &Checker,
    p: &FunctionLikeParameter,
    class: Option<&ClassLikeMetadata>,
) -> Option<Type> {
    if !p.is_promoted_property() {
        return None;
    }
    let doc = doc_before(&c.file.contents, p.span().start.offset as usize)?;
    // the doc comment must be the parameter's own (after the previous parameter)
    let doc = Doc::parse(doc);
    let scope = scope_at(c, p.span(), class);
    let vars = doc.var_types(&scope);
    let name = text(&p.variable.name[1..]);
    match vars.as_slice() {
        [(n, t)] if n.is_empty() => Some(t.clone()),
        _ => vars.into_iter().find(|(n, _)| *n == name).map(|(_, t)| t),
    }
}

fn has_template(t: &Type) -> bool {
    let found = std::cell::Cell::new(false);
    fn walk(t: &Type, found: &std::cell::Cell<bool>) {
        if matches!(t, Type::Template(_)) {
            found.set(true);
        }
        t.traverse(&|inner| {
            walk(inner, found);
            inner.clone()
        });
    }
    walk(t, &found);
    found.get()
}

/// A constructor: `__construct` or a method named as its class (`resolveMethodPhpDocFromParentClass()`).
fn is_constructor(k: &ClassLikeMetadata, lower: &[u8]) -> bool {
    let short = k
        .name
        .as_bytes()
        .rsplit(|b| *b == b'\\')
        .next()
        .unwrap_or(b"");
    lower == b"__construct" || short.eq_ignore_ascii_case(lower)
}

/// A `= null` default.
pub fn default_is_null(p: &FunctionLikeParameter) -> bool {
    match p.default_value.as_ref().map(|d| d.value) {
        Some(Expression::Literal(Literal::Null(_))) => true,
        Some(Expression::ConstantAccess(a)) => a.name.value().eq_ignore_ascii_case(b"null"),
        _ => false,
    }
}

/// A property's `getReadableType()`: the native type, narrowed by its `@var` (inherited from a parent's property
/// when it has none).
pub fn property(
    c: &Checker,
    hint: Option<&Hint>,
    at: Span,
    name: &[u8],
    context: &ClassLikeMetadata,
) -> Type {
    let native = hint.map(|h| native(c, h, Some(context)));
    let mut phpdoc = doc_before(&c.file.contents, at.start.offset as usize).and_then(|doc| {
        let doc = Doc::parse(doc);
        // in a trait, its templates as the class's `@use` tag gives them (else their bounds)
        let mut scope = scope_at(c, at, Some(context));
        if let Some(t) = c
            .in_trait()
            .and_then(|t| c.codebase.get_class_like(t.as_bytes()))
        {
            scope.templates.extend(trait_templates(c, t, context));
        }
        doc.var_types(&scope)
            .into_iter()
            .find(|(n, _)| n.is_empty() || n.as_bytes() == name)
            .map(|(_, t)| t)
    });
    if phpdoc.is_none() {
        phpdoc = inherited_property_doc(c, context, name, 0);
    }
    match native {
        Some(n) => decide_type(n, phpdoc),
        None => phpdoc.unwrap_or_else(mixed),
    }
}

/// `PhpDocInheritanceResolver::resolvePhpDocForProperty()`'s `@var` from the parent class or an interface.
fn inherited_property_doc(
    c: &Checker,
    class: &ClassLikeMetadata,
    name: &[u8],
    depth: usize,
) -> Option<Type> {
    if depth > 32 {
        return None;
    }
    let mut parents: Vec<&ClassLikeMetadata> = class
        .direct_parent_class
        .iter()
        .filter_map(|p| c.codebase.get_class_like(p.as_bytes()))
        .collect();
    parents.extend(
        class
            .direct_parent_interfaces
            .iter()
            .filter_map(|i| c.codebase.get_class_like(i.as_bytes())),
    );
    let mut var = Vec::with_capacity(name.len() + 1);
    var.push(b'$');
    var.extend_from_slice(name);
    for parent in parents {
        let Some((declaring, p)) = parent
            .declaring_property_ids
            .get(&mago_word::word(&var))
            .and_then(|k| c.codebase.get_class_like(k.as_bytes()))
            .and_then(|d| Some((d, d.properties.get(&mago_word::word(&var))?)))
        else {
            continue;
        };
        if p.read_visibility == mago_codex::visibility::Visibility::Private {
            continue;
        }
        if declaring.flags.contains(MetadataFlags::BUILTIN) {
            let class = symbols::class(&text(declaring.name.as_bytes()))?;
            return class
                .property_docs
                .get(&text(name))
                .map(|t| pun_phpstan::resolve::resolve_str(t, Some(&class.name)));
        }
        let own = own_property_doc(c, declaring, p, name);
        return own
            .or_else(|| inherited_property_doc(c, declaring, name, depth + 1))
            .map(|t| ancestor_types(class, declaring, &t));
    }
    None
}

/// A property's declaration: Mago gives a plain property only its name's span, and the declaration starts at the
/// modifiers and type before it.
// ponytail: not past an attribute, nor to the first item of `public $a, $b`
pub fn property_span(c: &Checker, p: &mago_codex::metadata::property::PropertyMetadata) -> Option<Span> {
    p.span.or_else(|| {
        let mut span = p.name_span?;
        let contents = file_contents(c, span.file_id)?;
        let before = contents[..span.start.offset as usize]
            .iter()
            .rev()
            .take_while(|b| b.is_ascii_alphanumeric() || b" \t\r\n_\\?|&()".contains(b))
            .count();
        span.start.offset -= before as u32;
        Some(span)
    })
}

/// The `@var` of property `name` (without `$`) in its own declaration's PHPDoc.
fn own_property_doc(
    c: &Checker,
    declaring: &ClassLikeMetadata,
    p: &mago_codex::metadata::property::PropertyMetadata,
    name: &[u8],
) -> Option<Type> {
    property_span(c, p).and_then(|span| {
        let (doc, _) = doc_at(c, span)?;
        let doc = Doc::parse(&doc);
        let scope = scope_at(c, span, Some(declaring));
        doc.var_types(&scope).into_iter().find(|(n, _)| n.is_empty() || n.as_bytes() == name).map(|(_, t)| t)
    })
}

/// A property of any file: `decideType()` of its native type and its `@var` (its own, else an inherited one).
pub fn property_of(
    c: &Checker,
    declaring: &ClassLikeMetadata,
    p: &mago_codex::metadata::property::PropertyMetadata,
    name: &[u8],
) -> Type {
    let native = p.type_declaration_metadata.as_ref().map(|t| from_tunion(c, &t.type_union, declaring));
    let phpdoc = own_property_doc(c, declaring, p, name).or_else(|| inherited_property_doc(c, declaring, name, 0));
    match native {
        Some(n) => decide_type(n, phpdoc),
        None => phpdoc.unwrap_or_else(mixed),
    }
}

/// The `NameScope` of a class-like's own PHPDoc (at `span`), with its templates.
pub fn class_scope(c: &Checker, span: Span, k: &ClassLikeMetadata) -> NameScope {
    scope_at(c, span, Some(k))
}

/// `scope` with a `@method` tag's own templates (`TemplateTypeScope::createWithMethod()`).
pub fn with_method_templates(
    scope: &NameScope,
    k: &ClassLikeMetadata,
    method: &str,
    tags: &[super::doc::TemplateTag],
) -> NameScope {
    let mut scope = scope.clone();
    let described = format!("method {}::{method}()", original_name(k));
    for t in tags {
        let template = TemplateType {
            name: t.name.clone(),
            scope: described.clone(),
            bound: t
                .bound
                .as_ref()
                .map_or(Type::Mixed { explicit: false }, |b| resolve_in(b, &scope)),
            default: t.default.as_ref().map(|d| resolve_in(d, &scope)),
            argument: false,
        };
        scope
            .templates
            .insert(t.name.clone(), Type::Template(Box::new(template)));
    }
    scope
}

/// The doc comment of the class constant declaration whose item starts at `offset`: the one before its modifiers.
fn constant_doc(contents: &[u8], offset: usize) -> Option<&[u8]> {
    let before = &contents[..offset.min(contents.len())];
    let stop = before
        .iter()
        .rposition(|b| matches!(b, b';' | b'{' | b'}'))?;
    let end = before.windows(2).rposition(|w| w == b"*/")?;
    if end < stop {
        return None;
    }
    doc_before(contents, end + 2)
}

/// The single unnamed `@var` of a constant's PHPDoc (`resolveConstantVarPhpDocType()`).
fn constant_var(
    c: &Checker,
    doc: &[u8],
    at: Span,
    class: &ClassLikeMetadata,
) -> Option<Option<Type>> {
    let doc = Doc::parse(doc);
    let scope = scope_at(c, at, Some(class));
    let vars = doc.var_types(&scope);
    Some(match vars.as_slice() {
        [(n, t)] if n.is_empty() => Some(t.clone()),
        _ => None,
    })
}

/// A class constant's `getPhpDocType()`: its own `@var`, else the parent class's or an interface's.
pub fn constant(c: &Checker, at: Span, name: &[u8], context: &ClassLikeMetadata) -> Option<Type> {
    match doc_before(&c.file.contents, at.start.offset as usize) {
        Some(doc) => constant_var(c, doc, at, context)?,
        None => inherited_constant_doc(c, context, name, 0)?,
    }
}

/// `resolvePhpDocForConstant()` from the parent class or the first immediate interface with a PHPDoc for it:
/// `Some(type)` when one is found (its type `None` without a single unnamed `@var`).
fn inherited_constant_doc(
    c: &Checker,
    class: &ClassLikeMetadata,
    name: &[u8],
    depth: usize,
) -> Option<Option<Type>> {
    if depth > 32 {
        return None;
    }
    let parents = class
        .direct_parent_class
        .iter()
        .chain(class.direct_parent_interfaces.iter())
        .filter_map(|p| c.codebase.get_class_like(p.as_bytes()));
    for parent in parents {
        let Some((declaring, constant)) = std::iter::once(parent)
            .chain(
                parent
                    .all_parent_classes
                    .iter()
                    .chain(parent.all_parent_interfaces.iter())
                    .filter_map(|a| c.codebase.get_class_like(a.as_bytes())),
            )
            .find_map(|k| k.constants.get(&mago_word::word(name)).map(|m| (k, m)))
        else {
            continue;
        };
        if constant.visibility == mago_codex::visibility::Visibility::Private {
            continue;
        }
        let own = file_contents(c, constant.span.file_id).and_then(|contents| {
            let doc = constant_doc(&contents, constant.span.start.offset as usize)?.to_vec();
            constant_var(c, &doc, constant.span, declaring)
        });
        match own {
            Some(found) => return Some(found),
            None => match inherited_constant_doc(c, declaring, name, depth + 1) {
                Some(found) => return Some(found),
                None => continue,
            },
        }
    }
    None
}

/// The class reflection the type model asks about classes PHP does not define: the project's and its
/// dependencies', from Mago's codebase and the classes' PHPDocs.
pub struct Classes<'c, 'a, 'arena> {
    checker: &'c Checker<'a, 'arena>,
    cache: RefCell<HashMap<String, Option<Arc<Class>>>>,
    /// classes being built (a cyclic hierarchy finds no class)
    building: RefCell<std::collections::HashSet<String>>,
}

impl<'c, 'a, 'arena> Classes<'c, 'a, 'arena> {
    pub fn new(checker: &'c Checker<'a, 'arena>) -> Self {
        Classes {
            checker,
            cache: RefCell::new(HashMap::new()),
            building: RefCell::new(std::collections::HashSet::new()),
        }
    }

    pub fn get(&self, name: &str) -> Option<Arc<Class>> {
        let key = name.to_ascii_lowercase();
        if let Some(found) = self.cache.borrow().get(&key) {
            return found.clone();
        }
        if !self.building.borrow_mut().insert(key.clone()) {
            return None;
        }
        let built = self.build(name).map(Arc::new);
        self.building.borrow_mut().remove(&key);
        self.cache.borrow_mut().insert(key, built.clone());
        built
    }

    fn build(&self, name: &str) -> Option<Class> {
        let c = self.checker;
        let k = c.codebase.get_class_like(name.as_bytes())?;
        let canonical = |w: &mago_word::Word| {
            c.codebase
                .get_class_like(w.as_bytes())
                .map_or_else(|| text(w.as_bytes()), original_name)
        };
        let mut ancestors: Vec<String> = k
            .all_parent_classes
            .iter()
            .chain(k.all_parent_interfaces.iter())
            .filter(|w| !w.as_bytes().starts_with(b"__internal_do_not_use__"))
            .map(canonical)
            .collect();
        // an enum's implicit interfaces (Mago names a backed one's by its backing type)
        if k.kind == SymbolKind::Enum {
            ancestors.retain(|a| a != "UnitEnum" && a != "BackedEnum");
            ancestors.push("UnitEnum".to_string());
            ancestors.extend(k.enum_type.is_some().then(|| "BackedEnum".to_string()));
        }
        let own = class_templates(c, k);
        let generics = self.generics(k, &own);
        // (`class_templates()` reads the templates of this PHPDoc, in its order)
        let variances = doc_at(c, k.span).map(|(doc, _)| Doc::parse(&doc).template_variances).unwrap_or_default();
        let templates = own
            .into_iter()
            .zip(variances)
            .map(|(t, variance)| Template {
                name: t.name,
                bound: serialize(&t.bound),
                default: t.default.as_ref().map(serialize),
                variance,
            })
            .collect();
        Some(Class {
            name: original_name(k),
            ancestors,
            templates,
            cases: k.enum_cases.keys().map(|w| text(w.as_bytes())).collect(),
            case_values: k
                .enum_cases
                .values()
                .filter_map(|m| {
                    let value = super::enums::constant_value(c, m.value_type.as_ref()?)?;
                    Some((text(m.name.as_bytes()), to_phpdoc(&value)))
                })
                .collect(),
            is_interface: k.kind == SymbolKind::Interface,
            is_trait: k.kind == SymbolKind::Trait,
            is_enum: k.kind == SymbolKind::Enum,
            is_final: k.flags.contains(MetadataFlags::FINAL),
            properties: Vec::new(),
            // ponytail: a constant whose value is not a literal (arrays, expressions) reads as `mixed`
            constants: k
                .constants
                .values()
                .map(|m| {
                    let value = m
                        .inferred_type
                        .as_ref()
                        .and_then(|a| super::enums::constant_value(c, a));
                    (
                        text(m.name.as_bytes()),
                        value.map_or_else(|| "mixed".to_string(), |t| to_phpdoc(&t)),
                    )
                })
                .collect(),
            // the names and staticness only (StringType::accepts() asks for `__toString()`, a callable string or array
            // for its method)
            methods: k
                .declaring_method_ids
                .iter()
                .map(|(m, id)| {
                    let f = symbols::Function {
                        name: text(m.as_bytes()),
                        side_effects: "Maybe".to_string(),
                        deprecated: false,
                        variants: Vec::new(),
                        named: None,
                        is_static: c.codebase.get_method_by_id(id).and_then(|f| f.method_metadata.as_ref()).is_some_and(|m| m.is_static),
                        doc: None,
                    };
                    (text(m.as_bytes()).to_ascii_lowercase(), f)
                })
                .collect(),
            iterable: None,
            generics,
            property_docs: HashMap::new(),
        })
    }

    /// The type arguments of each generic ancestor (`@extends` / `@implements`, through the parents' own), over the
    /// class's templates.
    fn generics(
        &self,
        k: &ClassLikeMetadata,
        own: &[TemplateType],
    ) -> HashMap<String, Vec<String>> {
        let c = self.checker;
        let canonical = |w: &mago_word::Word| {
            c.codebase
                .get_class_like(w.as_bytes())
                .map_or_else(|| text(w.as_bytes()), original_name)
        };
        let mut direct: HashMap<String, Vec<Type>> = HashMap::new();
        if let Some((doc, scopes)) = doc_at(c, k.span) {
            let doc = Doc::parse(&doc);
            let mut scope = NameScope {
                names: scopes.at(k.span.start.offset),
                class: Some(original_name(k)),
                parent: parent_name(c.codebase, k),
                templates: HashMap::new(),
            };
            for t in own {
                scope
                    .templates
                    .insert(t.name.clone(), Type::Template(Box::new(t.clone())));
            }
            for node in &doc.ancestors {
                if let Type::Object(name, args) = resolve_in(node, &scope)
                    && !args.is_empty()
                {
                    let name = c
                        .codebase
                        .get_class_like(name.as_bytes())
                        .map_or(name, original_name);
                    direct.insert(name.to_ascii_lowercase(), args);
                }
            }
        }
        let mut out = HashMap::new();
        let parents = k
            .direct_parent_class
            .iter()
            .chain(k.direct_parent_interfaces.iter())
            .map(canonical);
        for parent in parents {
            let Some(p) = symbols::lookup(&parent) else {
                continue;
            };
            if p.templates.is_empty() {
                // a non-generic parent passes its own ancestors' arguments on as they are
                for (ancestor, args) in &p.generics {
                    out.entry(ancestor.clone()).or_insert_with(|| args.clone());
                }
                continue;
            }
            // without `@extends`, the parent's templates resolve to their defaults, else their bounds
            let args = direct
                .get(&parent.to_ascii_lowercase())
                .cloned()
                .unwrap_or_else(|| {
                    p.templates
                        .iter()
                        .map(|t| {
                            let s = t.default.as_deref().unwrap_or(&t.bound);
                            pun_phpstan::resolve::resolve_str(s, Some(&p.name))
                        })
                        .collect()
                });
            let mapped = p.generics.keys().filter_map(|ancestor| {
                Some((
                    ancestor,
                    pun_phpstan::types::ancestor_args(&p, ancestor, &args)?,
                ))
            });
            for (ancestor, mapped) in mapped {
                out.entry(ancestor.clone())
                    .or_insert_with(|| mapped.iter().map(serialize).collect());
            }
            out.insert(p.name.clone(), args.iter().map(serialize).collect());
        }
        out
    }
}

/// A type as the symbols' type strings write it: `to_phpdoc()` with each template as
/// `__template<T, 'scope', bound[, default]>`, so it reads back as the same template.
fn serialize(t: &Type) -> String {
    fn encode(t: &Type) -> Type {
        match t {
            Type::Template(tpl) => {
                let mut args = vec![
                    Type::Object(tpl.name.clone(), vec![]),
                    Type::ConstString(tpl.scope.clone()),
                    encode(&tpl.bound),
                ];
                args.extend(tpl.default.as_ref().map(encode));
                Type::Object("__template".to_string(), args)
            }
            Type::Benevolent(types) => Type::Object(
                "__benevolent".to_string(),
                vec![Type::Union(types.iter().map(encode).collect())],
            ),
            _ => t.traverse(&encode),
        }
    }
    to_phpdoc(&encode(t))
}

/// Runs `f` with the type model knowing the project's classes.
pub fn with_classes<R>(c: &Checker, f: impl FnOnce() -> R) -> R {
    let classes = Classes::new(c);
    let functions = |name: &str| c.codebase.get_function(name.as_bytes()).is_some();
    symbols::with_classes(&|name| classes.get(name), || symbols::with_functions(&functions, f))
}
