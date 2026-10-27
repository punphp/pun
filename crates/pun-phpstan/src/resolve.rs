//! `TypeNodeResolver`: a parsed PHPDoc type to a [`Type`], for built-in signatures (no namespace, `use` aliases
//! or type aliases; `self` / `static` / `$this` resolve in the given class). Also reads Pun's extensions of the
//! syntax that `scripts/export-phpstan-symbols.php` writes: `__template<T, 'scope', BOUND[, DEFAULT]>`, `*ERROR*`,
//! `*NEVER*` (PHPStan's own `__benevolent<…>` is handled the way PHPStan does).

use crate::phpdoc::{self, ConstExpr, Node, ShapeKey};
use crate::symbols;
use crate::types::{
    Accessory, Conditional, NeverKind, Param, ShapeBuilder, Signature, Subject, Template, Type,
    contains_template, intersect, is_super_type_of, mixed, to_benevolent, union,
};

thread_local! {
    /// The `typeAliases` parameter while [`with_type_aliases()`] runs, and the aliases being resolved.
    static GLOBAL_ALIASES: std::cell::RefCell<(Vec<(String, String)>, Vec<String>)> = Default::default();
}

/// Runs `f` with the `typeAliases` parameter's global type aliases (name, type string) in effect.
pub fn with_type_aliases<R>(aliases: &[(String, String)], f: impl FnOnce() -> R) -> R {
    let previous = GLOBAL_ALIASES.with(|g| std::mem::replace(&mut g.borrow_mut().0, aliases.to_vec()));
    let out = f();
    GLOBAL_ALIASES.with(|g| g.borrow_mut().0 = previous);
    out
}

/// `UsefulTypeAliasResolver::hasTypeAlias()` of a global type alias.
pub fn has_global_alias(name: &str) -> bool {
    GLOBAL_ALIASES.with(|g| g.borrow().0.iter().any(|(n, _)| n == name))
}

/// `resolveGlobalTypeAlias()`: the alias's type string resolved in no namespace.
fn global_alias(name: &str) -> Option<Type> {
    let definition = GLOBAL_ALIASES.with(|g| {
        let g = g.borrow();
        g.0.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone())
    })?;
    if GLOBAL_ALIASES.with(|g| g.borrow().1.iter().any(|n| n == name)) {
        // PHPStan throws ShouldNotHappenException on a circular definition; see COVERAGE.md
        return Some(Type::Error);
    }
    GLOBAL_ALIASES.with(|g| g.borrow_mut().1.push(name.to_string()));
    let ty = resolve_str(&definition, None);
    GLOBAL_ALIASES.with(|g| g.borrow_mut().1.pop());
    Some(ty)
}

/// `TypeStringResolver::resolve($type, new NameScope(null, [], $class))`; a parse error is an `ErrorType`.
pub fn resolve_str(input: &str, class: Option<&str>) -> Type {
    match phpdoc::parse(input) {
        Ok(node) => resolve(&node, class),
        Err(_) => Type::Error,
    }
}

fn explicit_mixed() -> Type {
    Type::Mixed { explicit: true }
}

fn accessory(base: Type, accessories: &[Accessory]) -> Type {
    let mut types = vec![base];
    types.extend(accessories.iter().cloned().map(Type::Accessory));
    Type::Intersection(types)
}

/// PHPStan's `NameScope`: what a PHPDoc type is resolved in.
pub trait Scope {
    /// `getClassName()`: what `self`, `static` and `$this` are.
    fn class(&self) -> Option<&str> {
        None
    }
    /// The parent class of [`Scope::class()`], for `parent`.
    fn parent(&self) -> Option<String>;
    /// `resolveStringName()`: a written class name to its full name.
    fn resolve_name(&self, name: &str) -> String {
        name.trim_start_matches('\\').to_string()
    }
    /// `resolveConstantNames()`: the full names a written constant name may have, in lookup order.
    fn constant_names(&self, name: &str) -> Vec<String> {
        vec![name.trim_start_matches('\\').to_string()]
    }
    /// `resolveTemplateTypeName()`: a template type in scope.
    fn template(&self, _name: &str) -> Option<Type> {
        None
    }
    /// `hasUseAlias()`: a `use` statement imports the name.
    fn has_use_alias(&self, _name: &str) -> bool {
        false
    }
    /// `getNamespace() !== null`
    fn in_namespace(&self) -> bool {
        false
    }
}

/// A callable's scope: its own templates over the scope around it (`withTemplateTypeMap()`).
struct WithTemplates<'a> {
    outer: &'a dyn Scope,
    templates: &'a [Template],
}

impl Scope for WithTemplates<'_> {
    fn class(&self) -> Option<&str> {
        self.outer.class()
    }
    fn parent(&self) -> Option<String> {
        self.outer.parent()
    }
    fn resolve_name(&self, name: &str) -> String {
        self.outer.resolve_name(name)
    }
    fn constant_names(&self, name: &str) -> Vec<String> {
        self.outer.constant_names(name)
    }
    fn template(&self, name: &str) -> Option<Type> {
        match self.templates.iter().find(|t| t.name == name) {
            Some(t) => Some(Type::Template(Box::new(t.clone()))),
            None => self.outer.template(name),
        }
    }
    fn has_use_alias(&self, name: &str) -> bool {
        self.outer.has_use_alias(name)
    }
    fn in_namespace(&self) -> bool {
        self.outer.in_namespace()
    }
}

/// A scope with only a class (built-in signatures).
struct ClassOnly<'a>(Option<&'a str>);

impl Scope for ClassOnly<'_> {
    fn class(&self) -> Option<&str> {
        self.0
    }
    fn parent(&self) -> Option<String> {
        // the direct parent is the ancestor class with the most ancestors of its own
        let class = symbols::lookup(self.0?)?;
        class
            .ancestors
            .iter()
            .filter_map(|a| symbols::lookup(a).filter(|c| !c.is_interface))
            .max_by_key(|c| c.ancestors.len())
            .map(|c| c.name.clone())
    }
}

pub fn resolve(node: &Node, class: Option<&str>) -> Type {
    resolve_in(node, &ClassOnly(class))
}

/// `TypeNodeResolver::resolve($node, $nameScope)`.
pub fn resolve_in(node: &Node, scope: &dyn Scope) -> Type {
    let r = |n: &Node| resolve_in(n, scope);
    let class = scope.class();
    match node {
        Node::Ident(name) => identifier(name, scope),
        Node::This => match class.and_then(symbols::lookup) {
            Some(c) => Type::This(c.name.clone()),
            None => Type::Error,
        },
        Node::Projection(covariant, inner) => Type::Projection(*covariant, Box::new(r(inner))),
        Node::Star(s) => match s.as_str() {
            "*" => Type::Wildcard,
            "NEVER" => Type::Never(NeverKind::Explicit),
            _ => Type::Error,
        },
        Node::Nullable(inner) => union(vec![r(inner), Type::Null]),
        Node::Union(members) => {
            let mut iterable = Vec::new();
            let mut others = Vec::new();
            for m in members {
                match m {
                    Node::Array(item) => iterable.push(r(item)),
                    m => others.push(r(m)),
                }
            }
            if !iterable.is_empty() {
                let array_item = union(iterable);
                let mut add_array = true;
                for t in &mut others {
                    if !t.is_iterable().yes()
                        || !is_super_type_of(&t.iterable_value_type(), &array_item).yes()
                    {
                        continue;
                    }
                    *t = match t {
                        Type::Object(_, args) if args.is_empty() => Type::Intersection(vec![
                            t.clone(),
                            Type::Iterable(Box::new(mixed()), Box::new(array_item.clone())),
                        ]),
                        Type::Array(..) | Type::ConstArray(_) => {
                            Type::array(mixed(), array_item.clone())
                        }
                        Type::Iterable(..) => {
                            Type::Iterable(Box::new(mixed()), Box::new(array_item.clone()))
                        }
                        _ => continue,
                    };
                    add_array = false;
                }
                if add_array {
                    others.push(Type::array(mixed(), array_item));
                }
            }
            union(others)
        }
        Node::Inter(members) => {
            let mut types = members.iter().map(r);
            let first = types.next().expect("intersection member");
            types.fold(first, |acc, t| intersect(vec![acc, t]))
        }
        Node::Conditional {
            subject,
            target,
            then,
            otherwise,
            negated,
        } => Type::Conditional(Box::new(Conditional {
            subject: Subject::Type(r(subject)),
            target: r(target),
            then: r(then),
            otherwise: r(otherwise),
            negated: *negated,
        })),
        Node::ConditionalForParameter {
            param,
            target,
            then,
            otherwise,
            negated,
        } => Type::Conditional(Box::new(Conditional {
            subject: Subject::Parameter(param.clone()),
            target: r(target),
            then: r(then),
            otherwise: r(otherwise),
            negated: *negated,
        })),
        Node::Array(item) => Type::array(mixed(), r(item)),
        Node::Generic { base, args } => generic(base, args, scope),
        Node::Callable {
            ident,
            templates,
            params,
            ret,
        } => {
            let templates: Vec<Template> = templates
                .iter()
                .map(|t| Template {
                    name: t.name.clone(),
                    scope: "anonymous function".into(),
                    bound: t.bound.as_ref().map_or_else(mixed, |b| r(b)),
                    default: t.default.as_ref().map(|d| r(d)),
                    argument: false,
                })
                .collect();
            let inner = WithTemplates {
                outer: scope,
                templates: &templates,
            };
            let r = |n: &Node| resolve_in(n, &inner);
            let main = identifier(ident, scope);
            let params: Vec<Param> = params
                .iter()
                .map(|p| Param {
                    name: p.name.trim_start_matches('$').to_string(),
                    ty: r(&p.ty),
                    optional: p.optional || p.variadic,
                    by_ref: p.by_ref,
                    variadic: p.variadic,
                })
                .collect();
            // ponytail: a conditional return on the callable's own parameter is a type assertion in PHPStan
            // (`callable(mixed $x): ($x is int ? true : false)`); kept as a plain conditional return here.
            let ret = r(ret);
            // ponytail: a pure or static callable keeps no purity or staticness (described as a plain one)
            let pure = ["pure-callable", "pure-closure", "static-pure-closure"].contains(&ident.to_ascii_lowercase().as_str());
            if pure && ret == Type::Void {
                return Type::Error;
            }
            let sig = Box::new(Signature { params, ret, templates });
            match main {
                Type::Callable(_) => Type::Callable(Some(sig)),
                Type::Object(c, args) if args.is_empty() && c == "Closure" => Type::Closure(sig),
                _ => Type::Error,
            }
        }
        Node::ArrayShape {
            kind,
            items,
            sealed,
            unsealed,
        } => {
            let template_key = items.iter().any(|i| matches!(&i.key, Some(ShapeKey::Ident(n)) if matches!(scope.template(n), Some(Type::Template(_)))));
            let late_extras = unsealed.as_ref().and_then(|(k, _)| k.as_ref()).is_some_and(|k| contains_template(&r(k)));
            if template_key || late_extras {
                return crate::types::LateArrayShape {
                    items: items.iter().map(|i| {
                        let key = i.key.as_ref().map(|k| match k {
                            ShapeKey::Ident(n) => scope.template(n).filter(|t| matches!(t, Type::Template(_))).unwrap_or_else(|| Type::ConstString(n.as_bytes().to_vec())),
                            ShapeKey::Str(n) => Type::ConstString(n.as_bytes().to_vec()),
                            ShapeKey::Int(n) => Type::ConstInt(int_value(n)),
                            ShapeKey::Fetch(c, n) => constant(&ConstExpr::Fetch(c.clone(), n.clone()), scope),
                        });
                        (key, r(&i.value), i.optional)
                    }).collect(),
                    unsealed: (!sealed).then(|| unsealed.as_ref().map_or_else(
                        || (None, mixed()), |(k, v)| (k.as_ref().map(|k| array_key(r(k))), r(v))
                    )),
                    kind: kind.clone(),
                }.create();
            }
            let mut builder = ShapeBuilder::new();
            let mut explicit = Vec::new();
            for item in items {
                let key = item.key.as_ref().map(|k| match k {
                    ShapeKey::Int(i) => Type::ConstInt(int_value(i)),
                    ShapeKey::Str(s) | ShapeKey::Ident(s) => Type::ConstString(s.as_bytes().to_vec()),
                    ShapeKey::Fetch(class, name) => {
                        constant(&ConstExpr::Fetch(class.clone(), name.clone()), scope)
                    }
                });
                explicit.extend(key.clone());
                builder.set(key, r(&item.value), item.optional);
            }
            let is_list = kind.ends_with("list");
            let default_key = || {
                if is_list {
                    Type::int_range(Some(0), None)
                } else {
                    mixed()
                }
            };
            if !sealed {
                match unsealed {
                    None => builder.make_unsealed(default_key(), mixed()),
                    Some((key, value)) => {
                        let key = key.as_ref().map_or_else(default_key, |k| array_key(r(k)));
                        let value = r(value);
                        // finite extra keys are optional keys, except those written explicitly
                        let finite = match &key {
                            Type::ConstInt(_) | Type::ConstString(_) => vec![key.clone()],
                            Type::Union(types)
                                if types.iter().all(|t| {
                                    matches!(t, Type::ConstInt(_) | Type::ConstString(_))
                                }) =>
                            {
                                types.clone()
                            }
                            _ => vec![],
                        };
                        for k in finite.iter().filter(|k| !explicit.contains(k)) {
                            builder.set(Some(k.clone()), value.clone(), true);
                        }
                        if finite.is_empty() {
                            builder.make_unsealed(key, value);
                        }
                    }
                }
            }
            let array = builder.get();
            let mut accessories = Vec::new();
            if is_list {
                accessories.push(Type::Accessory(Accessory::List));
            }
            if kind.starts_with("non-empty") {
                accessories.push(Type::Accessory(Accessory::NonEmptyArray));
            }
            if accessories.is_empty() {
                return array;
            }
            accessories.insert(0, array);
            intersect(accessories)
        }
        Node::ObjectShape(items) => Type::ObjectShape(
            items
                .iter()
                .map(|(name, optional, ty)| (name.clone(), r(ty), *optional))
                .collect(),
        ),
        Node::Const(c) => constant(c, scope),
        Node::OffsetAccess(t, o) => offset_access(r(t), r(o)),
    }
}

/// PHP's `(int)` of a phpdoc-parser integer literal: the leading decimal digits (`0x1A` is 0), saturating.
fn int_value(s: &str) -> i64 {
    let s = s.replace('_', "");
    let sign = usize::from(s.starts_with(['-', '+']));
    let digits = s[sign..].bytes().take_while(u8::is_ascii_digit).count();
    let n = &s[..sign + digits];
    n.parse::<i64>().unwrap_or(if n.starts_with('-') {
        i64::MIN
    } else {
        i64::MAX
    })
}

/// `transformUnsafeArrayKey()` without the opt-in string-key check: `intersect(toArrayKey(), int|string)`.
fn array_key(key: Type) -> Type {
    let k = intersect(vec![
        to_array_key(&key),
        Type::Union(vec![Type::Int, Type::String]),
    ]);
    to_array_key(&k)
}

/// `Type::toArrayKey()`
pub(crate) fn to_array_key(t: &Type) -> Type {
    match t {
        Type::Template(_) => t.clone(),
        Type::Mixed { .. } => Type::Benevolent(vec![Type::Int, Type::String]),
        Type::Union(types) => union(types.iter().map(to_array_key).collect()),
        Type::Benevolent(types) => match union(types.iter().map(to_array_key).collect()) {
            Type::Union(types) => Type::Benevolent(types),
            t => t,
        },
        Type::Float => Type::Int,
        Type::Bool => Type::Union(vec![Type::ConstInt(0), Type::ConstInt(1)]),
        Type::Null => Type::ConstString(Vec::new()),
        Type::ConstBool(b) => Type::ConstInt(*b as i64),
        // ObjectTypeTrait, ArrayTypeTrait, ResourceType, CallableType, IterableType, VoidType: an ErrorType (a mixed)
        Type::Void
        | Type::Resource
        | Type::ObjectWithoutClass
        | Type::Object(..)
        | Type::Static(..)
        | Type::This(_)
        | Type::EnumCase(..)
        | Type::ObjectShape(_)
        | Type::Closure(_)
        | Type::Callable(_)
        | Type::Array(..)
        | Type::ConstArray(_)
        | Type::Iterable(..) => Type::Error,
        Type::ConstString(s) if std::str::from_utf8(s).ok().and_then(|s| s.parse::<i64>().ok()).is_some_and(|i| i.to_string().as_bytes() == s) => {
            Type::ConstInt(std::str::from_utf8(s).unwrap().parse().expect("int"))
        }
        _ => t.clone(),
    }
}

/// `tryResolvePseudoTypeClassType()`: a keyword imported by `use`, or naming a class of the namespace, is that class.
fn pseudo_type_class(name: &str, scope: &dyn Scope) -> Option<Type> {
    if scope.has_use_alias(name) {
        return Some(Type::Object(scope.resolve_name(name), vec![]));
    }
    if !scope.in_namespace() {
        return None;
    }
    let class = scope.resolve_name(name);
    symbols::lookup(&class).map(|_| Type::Object(class, vec![]))
}

fn identifier(name: &str, scope: &dyn Scope) -> Type {
    use Accessory as A;
    let string = || Type::String;
    let lower = name.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "integer"
            | "scalar"
            | "number"
            | "numeric"
            | "boolean"
            | "double"
            | "resource"
            | "never"
            | "noreturn"
            | "empty"
    ) && let Some(class) = pseudo_type_class(name, scope)
    {
        return class;
    }
    match lower.as_str() {
        "int" | "integer" => return Type::Int,
        "positive-int" => return Type::int_range(Some(1), None),
        "negative-int" => return Type::int_range(None, Some(-1)),
        "non-positive-int" => return Type::int_range(None, Some(0)),
        "non-negative-int" => return Type::int_range(Some(0), None),
        "non-zero-int" => {
            return Type::Union(vec![
                Type::int_range(None, Some(-1)),
                Type::int_range(Some(1), None),
            ]);
        }
        "string" => return Type::String,
        "decimal-int-string" => {
            return accessory(string(), &[A::DecimalIntString { inverse: false }]);
        }
        "non-decimal-int-string" => {
            return accessory(string(), &[A::DecimalIntString { inverse: true }]);
        }
        "lowercase-string" => return accessory(string(), &[A::LowercaseString]),
        "uppercase-string" => return accessory(string(), &[A::UppercaseString]),
        "literal-string" => return accessory(string(), &[A::LiteralString]),
        "class-string" | "interface-string" | "trait-string" => return Type::ClassString(None),
        "enum-string" => {
            return Type::ClassString(Some(Box::new(Type::Object("UnitEnum".into(), vec![]))));
        }
        "callable-string" => return Type::Intersection(vec![string(), Type::Callable(None)]),
        "array-key" => return Type::Benevolent(vec![Type::Int, Type::String]),
        "scalar" => return Type::Union(vec![Type::Int, Type::Float, Type::String, Type::Bool]),
        "number" => return Type::Union(vec![Type::Int, Type::Float]),
        "numeric" => {
            return Type::Union(vec![
                Type::Int,
                Type::Float,
                accessory(string(), &[A::NumericString]),
            ]);
        }
        "numeric-string" => return accessory(string(), &[A::NumericString]),
        "non-empty-string" => return accessory(string(), &[A::NonEmptyString]),
        "non-empty-lowercase-string" => {
            return accessory(string(), &[A::NonEmptyString, A::LowercaseString]);
        }
        "non-empty-uppercase-string" => {
            return accessory(string(), &[A::NonEmptyString, A::UppercaseString]);
        }
        "truthy-string" | "non-falsy-string" => return accessory(string(), &[A::NonFalsyString]),
        "non-empty-literal-string" => {
            return accessory(string(), &[A::NonEmptyString, A::LiteralString]);
        }
        "bool" | "boolean" => return Type::Bool,
        "true" => return Type::ConstBool(true),
        "false" => return Type::ConstBool(false),
        "null" => return Type::Null,
        "float" | "double" => return Type::Float,
        "array" | "associative-array" => return Type::array(mixed(), mixed()),
        "non-empty-array" => return accessory(Type::array(mixed(), mixed()), &[A::NonEmptyArray]),
        "iterable" => return Type::Iterable(Box::new(mixed()), Box::new(mixed())),
        "callable" | "pure-callable" => return Type::Callable(None),
        "pure-closure" | "static-closure" | "static-pure-closure" => return Type::Object("Closure".into(), vec![]),
        "resource" | "open-resource" | "closed-resource" => return Type::Resource,
        "mixed" => return explicit_mixed(),
        "void" => return Type::Void,
        "object" => return Type::ObjectWithoutClass,
        "callable-object" => {
            return Type::Intersection(vec![Type::ObjectWithoutClass, Type::Callable(None)]);
        }
        "callable-array" => {
            return Type::Intersection(vec![Type::array(mixed(), mixed()), Type::Callable(None)]);
        }
        "never" | "noreturn" | "never-return" | "never-returns" | "no-return" => {
            return Type::Never(NeverKind::NonAccepting);
        }
        "list" => return Type::list(mixed()),
        "non-empty-list" => {
            return Type::Intersection(vec![
                Type::array(Type::IntRange(Some(0), None), mixed()),
                Type::Accessory(A::NonEmptyArray),
                Type::Accessory(A::List),
            ]);
        }
        "__stringandstringable" => return Type::StringAcceptingStringable,
        "__stringnotstringable" => return Type::StringNotAcceptingStringable,
        _ => {}
    }
    if let Some(class) = scope.class() {
        match name.to_ascii_lowercase().as_str() {
            "self" => return Type::Object(class.to_string(), vec![]),
            "parent" => {
                return match scope.parent() {
                    Some(parent) => Type::Object(parent, vec![]),
                    // NonexistentParentClassType, described as `parent`
                    // ponytail: it is an object of a class that does not exist; add its own type if a rule asks more
                    None => Type::Object("parent".to_string(), vec![]),
                };
            }
            "static" => {
                return match symbols::lookup(class) {
                    Some(c) => Type::Static(c.name.clone(), vec![]),
                    None => Type::Error,
                };
            }
            _ => {}
        }
    }
    if let Some(alias) = global_alias(name) {
        return alias;
    }
    if let Some(template) = scope.template(name) {
        return template;
    }
    let written = name;
    let resolved = scope.resolve_name(name);
    let name = resolved.as_str();
    if name.contains('-') && !name.starts_with("OCI-") {
        return Type::Error;
    }
    // `tryResolveConstant()`
    let constants = &symbols::symbols(symbols::PhpVersion::Php84).constants;
    if might_be_constant(written)
        && symbols::lookup(name).is_none()
        && let Some(ty) = scope
            .constant_names(written)
            .iter()
            .find_map(|n| constants.get(n.as_str()))
    {
        return match ty.as_str() {
            "INF" => Type::ConstFloat(f64::INFINITY),
            "-INF" => Type::ConstFloat(f64::NEG_INFINITY),
            "NAN" => Type::ConstFloat(f64::NAN),
            // ConstantResolver builds some unions by hand (`new UnionType([...])`, e.g. PHP_SAPI's
            // 'cli'|…|non-falsy-string), so a constant's top-level union is not normalized again.
            ty => match phpdoc::parse(ty) {
                Ok(Node::Union(members)) => {
                    Type::Union(members.iter().map(|m| resolve(m, None)).collect())
                }
                _ => resolve_str(ty, None),
            },
        };
    }
    Type::Object(name.to_string(), vec![])
}

/// `(?:^|\\)[A-Z_][A-Z0-9_]*$`
fn might_be_constant(name: &str) -> bool {
    let last = name.rsplit('\\').next().unwrap_or(name);
    let mut bytes = last.bytes();
    bytes
        .next()
        .is_some_and(|b| b.is_ascii_uppercase() || b == b'_')
        && bytes.all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
}

fn generic(base: &str, args: &[Node], scope: &dyn Scope) -> Type {
    let lower = base.to_ascii_lowercase();
    // Pun's template serialization (`__template<T, 'scope', bound, default>`)
    if lower == "__template"
        && let [
            Node::Ident(name),
            Node::Const(ConstExpr::Str(described)),
            bound,
            rest @ ..,
        ] = args
    {
        return Type::Template(Box::new(Template {
            name: name.clone(),
            scope: String::from_utf8_lossy(described).into_owned(),
            bound: resolve_in(bound, scope),
            default: rest.first().map(|d| resolve_in(d, scope)),
            argument: false,
        }));
    }
    // (the call-site variances are a generic object's or `static`'s only)
    let projected: Vec<Type> = args.iter().map(|a| resolve_in(a, scope)).collect();
    let types: Vec<Type> = projected
        .iter()
        .map(|t| match t {
            Type::Projection(_, t) => (**t).clone(),
            t => t.clone(),
        })
        .collect();
    match lower.as_str() {
        "array" | "non-empty-array" => {
            let array = match types.as_slice() {
                [item] => Type::array(mixed(), item.clone()),
                [key, item] => {
                    let key = array_key(key.clone());
                    match key {
                        Type::ConstInt(_) | Type::ConstString(_) => {
                            let mut builder = ShapeBuilder::new();
                            builder.set(Some(key), item.clone(), true);
                            builder.get()
                        }
                        key => Type::array(key, item.clone()),
                    }
                }
                _ => return Type::Error,
            };
            if lower == "non-empty-array" {
                return intersect(vec![array, Type::Accessory(Accessory::NonEmptyArray)]);
            }
            array
        }
        "list" | "non-empty-list" => {
            let [item] = types.as_slice() else {
                return Type::Error;
            };
            let list = Type::list(item.clone());
            if lower == "non-empty-list" {
                return intersect(vec![list, Type::Accessory(Accessory::NonEmptyArray)]);
            }
            list
        }
        "iterable" => match types.as_slice() {
            [item] => Type::Iterable(Box::new(explicit_mixed()), Box::new(item.clone())),
            [key, item] => Type::Iterable(Box::new(key.clone()), Box::new(item.clone())),
            _ => Type::Error,
        },
        "class-string" | "interface-string" => match types.as_slice() {
            // (a `TemplateMixedType` is a `MixedType`)
            [g] if g.is_object_yes()
                // (an `ErrorType` is a `MixedType`)
                || matches!(g, Type::Mixed { .. } | Type::Error)
                || matches!(g, Type::Template(t) if matches!(t.bound, Type::Mixed { .. })) =>
            {
                Type::ClassString(Some(Box::new(g.clone())))
            }
            _ => Type::Error,
        },
        "int" => {
            let bound = |t: &Type, node: &Node, keyword: &str| match t {
                Type::ConstInt(i) => Ok(Some(*i)),
                _ if *node == Node::Ident(keyword.to_string()) => Ok(None),
                _ => Err(()),
            };
            match (types.as_slice(), args) {
                ([min, max], [min_node, max_node]) => {
                    match (bound(min, min_node, "min"), bound(max, max_node, "max")) {
                        (Ok(min), Ok(max)) => Type::int_range(min, max),
                        _ => Type::Error,
                    }
                }
                _ => Type::Error,
            }
        }
        "int-mask" => int_mask(union(types)).unwrap_or(Type::Error),
        "int-mask-of" => match types.as_slice() {
            [t] => int_mask(t.clone()).unwrap_or(Type::Error),
            _ => Type::Error,
        },
        // GenericStaticType
        "static" => match scope.class().and_then(symbols::lookup) {
            Some(c) => Type::Static(c.name.clone(), projected),
            None => Type::Error,
        },
        "__benevolent" => match types.as_slice() {
            [t] => to_benevolent(t.clone()),
            _ => Type::Error,
        },
        "key-of" | "value-of" => match types.as_slice() {
            // ponytail: a KeyOfType/ValueOfType of a template (resolved late) is not modelled; kept as a
            // `key-of` / `value-of` class the checks skip
            [t] if crate::types::contains_template(t) => Type::Object(lower.clone(), types),
            [t] => key_or_value_of(t, lower == "value-of"),
            _ => Type::Error,
        },
        _ => generic_object(base, projected, scope),
    }
}

/// `resolveOffsetAccessNode()`: an error type when `t` has no `o` offset, else an OffsetAccessType (resolved
/// late, [`resolve_late()`]).
// ponytail: a template's offset is not checked against its bound
fn offset_access(t: Type, o: Type) -> Type {
    if !crate::types::contains_template(&t) && offset_value(&t, &o) == Some(Type::Error) {
        return Type::Error;
    }
    Type::OffsetAccess(Box::new(t), Box::new(o))
}

/// `TypeUtils::resolveLateResolvableTypes()` of the late array shapes and OffsetAccessTypes Pun models.
// ponytail: an object's offset (ArrayAccess) and a non-constant offset of a constant array stay unresolved
pub fn resolve_late(t: &Type) -> Type {
    if let Type::LateArrayShape(shape) = t {
        return resolve_late(&shape.resolve());
    }
    let t = t.traverse(&resolve_late);
    if let Type::OffsetAccess(on, o) = &t
        && let Some(value) = offset_value(&on.resolve_to_bounds(), &o.resolve_to_bounds())
    {
        return value;
    }
    t
}

/// `getOffsetValueType()` of the types modelled (`None` for the others), an error type when there is no offset `o`.
fn offset_value(t: &Type, o: &Type) -> Option<Type> {
    let key = array_key(o.clone());
    Some(match t {
        Type::Mixed { .. } => Type::Mixed { explicit: false },
        Type::ConstArray(a) => {
            if !matches!(key, Type::ConstInt(_) | Type::ConstString(_)) {
                return None;
            }
            match a.keys.iter().position(|k| *k == key) {
                Some(i) => a.values[i].clone(),
                None => match &a.unsealed {
                    Some((k, v)) if !is_super_type_of(k, &key).no() => v.clone(),
                    _ => Type::Error,
                },
            }
        }
        Type::Array(k, v) => {
            if is_super_type_of(k, &key).no() {
                Type::Error
            } else {
                (**v).clone()
            }
        }
        // `list<T>`, `non-empty-array<T>`: the array's
        Type::Intersection(types) => {
            let array = types.iter().find(|t| matches!(t, Type::Array(..)))?;
            offset_value(array, o)?
        }
        Type::String => match key {
            Type::ConstInt(_) | Type::Int => intersect(vec![Type::String, Type::Accessory(Accessory::NonEmptyString)]),
            _ => Type::Error,
        },
        Type::Int | Type::ConstInt(_) | Type::Float | Type::ConstFloat(_) | Type::Bool | Type::ConstBool(_) | Type::Null => {
            Type::Error
        }
        Type::Union(types) => {
            let found: Vec<Type> = types.iter().map(|m| offset_value(m, o)).collect::<Option<_>>()?;
            let found: Vec<Type> = found.into_iter().filter(|f| *f != Type::Error).collect();
            if found.is_empty() { Type::Error } else { union(found) }
        }
        _ => return None,
    })
}

/// `KeyOfType::getResult()` / `ValueOfType::getResult()`: an enum's case values, else the iterable key or value type.
pub fn key_or_value_of(t: &Type, value: bool) -> Type {
    if let Type::Template(tt) = t {
        // (a template's enum cases are its bound's; one bounded by a backed-enum interface has none)
        let backed = |c: &symbols::Class| {
            !c.is_enum && (c.name.eq_ignore_ascii_case("BackedEnum") || c.ancestors.iter().any(|a| a.eq_ignore_ascii_case("BackedEnum")))
        };
        if value && matches!(&tt.bound, Type::Object(n, _) if symbols::lookup(n).is_some_and(|c| backed(&c))) {
            return Type::Union(vec![Type::Int, Type::String]);
        }
        return key_or_value_of(&tt.bound, value);
    }
    let cases = match t {
        Type::Object(class, _) => symbols::lookup(class)
            .filter(|c| c.is_enum)
            .map(|c| (c.case_values.clone(), c.name.clone())),
        Type::EnumCase(class, case) => symbols::lookup(class).map(|c| {
            let values = c.case_values.iter().filter(|(n, _)| n == case).cloned().collect();
            (values, c.name.clone())
        }),
        _ => None,
    };
    if value && let Some((values, class)) = cases {
        let mut types: Vec<Type> = values.iter().map(|(_, v)| resolve_str(v, Some(&class))).collect();
        return match types.len() {
            0 => Type::Never(NeverKind::Implicit),
            1 => types.pop().expect("one"),
            _ => Type::Union(types),
        };
    }
    if t.is_iterable().no() {
        return Type::Error;
    }
    if value { t.iterable_value_type() } else { t.iterable_key_type() }
}

impl Type {
    fn is_object_yes(&self) -> bool {
        match self {
            Type::Object(..)
            | Type::ObjectWithoutClass
            | Type::Static(..)
            | Type::This(_)
            | Type::ObjectShape(_)
            | Type::Closure(_) => true,
            Type::Template(t) => t.bound.is_object_yes(),
            Type::Union(types) | Type::Intersection(types) => {
                if matches!(self, Type::Union(_)) {
                    types.iter().all(Type::is_object_yes)
                } else {
                    types.iter().any(Type::is_object_yes)
                }
            }
            _ => false,
        }
    }
}

fn generic_object(base: &str, mut types: Vec<Type>, scope: &dyn Scope) -> Type {
    let main = identifier(base, scope);
    let class_name = match &main {
        Type::Object(name, _) => name.clone(),
        // a template's `getObjectClassNames()`: its object bound's class (`T<U>` is `Bound<U>`)
        Type::Template(t) => match &t.bound {
            Type::Object(name, _) => name.clone(),
            _ => return Type::Error,
        },
        _ => return Type::Error,
    };
    let Some(reflection) = symbols::lookup(&class_name) else {
        return Type::Object(class_name, types);
    };
    let iterable = main.is_iterable().yes();
    // the defaults of the templates after the given arguments (a required one is skipped, not filled)
    let defaults: Vec<Type> = reflection
        .templates
        .iter()
        .skip(types.len())
        .filter_map(|t| t.default.as_deref())
        .map(|d| resolve_str(d, Some(&class_name)))
        .collect();
    types.extend(defaults);
    if !reflection.templates.is_empty() {
        if ["Traversable", "IteratorAggregate", "Iterator"].contains(&class_name.as_str())
            && types.len() == 1
        {
            return Type::Object(
                class_name,
                vec![explicit_mixed(), types.pop().expect("one type")],
            );
        }
        if class_name == "Generator" && (types.len() == 1 || types.len() == 2) {
            if types.len() == 1 {
                types.insert(0, explicit_mixed());
            }
            types.extend([explicit_mixed(), explicit_mixed()]);
            return Type::Object(class_name, types);
        }
        if !iterable || types.len() != 1 || reflection.templates.len() == 1 {
            return Type::Object(class_name, types);
        }
    }
    if iterable {
        match types.len() {
            1 => {
                return intersect(vec![
                    main,
                    Type::Iterable(
                        Box::new(explicit_mixed()),
                        Box::new(types.pop().expect("one")),
                    ),
                ]);
            }
            2 => {
                let item = types.pop().expect("two");
                let key = types.pop().expect("two");
                return intersect(vec![main, Type::Iterable(Box::new(key), Box::new(item))]);
            }
            _ => {}
        }
    }
    Type::Object(class_name, types)
}

/// `expandIntMaskToType()`
fn int_mask(t: Type) -> Option<Type> {
    // `TypeUtils::getConstantIntegers()`: every member a constant integer
    let members = match &t {
        Type::Union(types) => types.as_slice(),
        t => std::slice::from_ref(t),
    };
    let ints: Vec<i64> = members
        .iter()
        .map(|t| match t {
            Type::ConstInt(i) => Some(*i),
            _ => None,
        })
        .collect::<Option<_>>()?;
    let mut values: Vec<i64> = Vec::new();
    for int in ints {
        if int != 0 && !values.contains(&int) {
            for v in values.clone() {
                let computed = v | int;
                if !values.contains(&computed) {
                    values.push(computed);
                }
            }
        }
        if !values.contains(&int) {
            values.push(int);
        }
    }
    if !values.contains(&0) {
        values.push(0);
    }
    let min = *values.iter().min().expect("values");
    let max = *values.iter().max().expect("values");
    if max - min == values.len() as i64 - 1 || values.len() > 256 {
        return Some(Type::int_range(Some(min), Some(max)));
    }
    Some(union(values.into_iter().map(Type::ConstInt).collect()))
}

fn constant(c: &ConstExpr, scope: &dyn Scope) -> Type {
    let class = scope.class();
    match c {
        ConstExpr::Int(i) => Type::ConstInt(int_value(i)),
        ConstExpr::Float(f) => Type::ConstFloat(f.replace('_', "").parse().unwrap_or(0.0)),
        ConstExpr::Str(s) => Type::ConstString(s.clone()),
        // PHPStan throws ShouldNotHappenException here (a bare `null` is an identifier); see COVERAGE.md
        ConstExpr::True | ConstExpr::False | ConstExpr::Null | ConstExpr::Array(_) => Type::Error,
        ConstExpr::Fetch(class_name, name) => {
            let is_static = class.is_some() && class_name.eq_ignore_ascii_case("static");
            let class_name = match (class, class_name.to_ascii_lowercase().as_str()) {
                (Some(c), "self" | "static") => c.to_string(),
                (Some(c), "parent") if symbols::lookup(c).is_some() => match scope.parent() {
                    Some(parent) => parent,
                    None => return Type::Error,
                },
                _ => scope.resolve_name(class_name),
            };
            let Some(reflection) = symbols::lookup(&class_name) else {
                return Type::Error;
            };
            if name.eq_ignore_ascii_case("class") {
                if is_static && !reflection.is_final {
                    return Type::ClassString(Some(Box::new(Type::Static(
                        reflection.name.clone(),
                        vec![],
                    ))));
                }
                return Type::ConstString(reflection.name.as_bytes().to_vec());
            }
            let matches = |constant: &str| match name.split_once('*') {
                None => constant == name,
                Some(_) => wildcard(name, constant),
            };
            let cases = reflection
                .cases
                .iter()
                .filter(|c| matches(c))
                .map(|c| Type::EnumCase(reflection.name.clone(), c.clone()));
            // declared, then inherited (as PHP's reflection lists them)
            let mut seen = Vec::new();
            let classes: Vec<_> = std::iter::once(reflection.clone())
                .chain(
                    reflection
                        .ancestors
                        .iter()
                        .filter_map(|a| symbols::lookup(a)),
                )
                .collect();
            let constants = classes
                .iter()
                .flat_map(|c| c.constants.iter().map(move |(n, t)| (c, n, t)))
                .filter(|(_, n, _)| {
                    let new = !seen.contains(n);
                    seen.push(*n);
                    new && matches(n)
                })
                .map(|(c, _, t)| resolve_str(t, Some(&c.name)));
            let types: Vec<Type> = cases.chain(constants).collect();
            if types.is_empty() {
                return Type::Error;
            }
            union(types)
        }
    }
}

/// `ConstExprNodeResolver::resolve()`: a constant expression's type (a `@method` parameter's default value).
pub fn const_expr(c: &ConstExpr, scope: &dyn Scope) -> Type {
    match c {
        ConstExpr::Array(items) => {
            let mut builder = ShapeBuilder::new();
            for (key, value) in items {
                let key = key.as_ref().map(|k| const_expr(k, scope));
                builder.set(key, const_expr(value, scope), false);
            }
            builder.get()
        }
        ConstExpr::False => Type::ConstBool(false),
        ConstExpr::True => Type::ConstBool(true),
        ConstExpr::Null => Type::Null,
        ConstExpr::Fetch(class_name, name) => {
            // ponytail: `parent` in a class PHPStan does not know resolves as a class name there
            let class_name = match (scope.class(), class_name.to_ascii_lowercase().as_str()) {
                (Some(c), "self" | "static") => c.to_string(),
                (Some(_), "parent") => match scope.parent() {
                    Some(p) => p,
                    None => return Type::Error,
                },
                _ => scope.resolve_name(class_name),
            };
            let Some(reflection) = symbols::lookup(&class_name) else {
                return Type::Error;
            };
            if reflection.cases.iter().any(|c| c == name) {
                return Type::EnumCase(reflection.name.clone(), name.clone());
            }
            std::iter::once(reflection.clone())
                .chain(reflection.ancestors.iter().filter_map(|a| symbols::lookup(a)))
                .find_map(|c| {
                    c.constants
                        .iter()
                        .find(|(n, _)| n == name)
                        .map(|(_, t)| resolve_str(t, Some(&c.name)))
                })
                .unwrap_or(Type::Error)
        }
        _ => constant(c, scope),
    }
}

/// `{^PREFIX.*?REST$}` with `*` the only metacharacter.
fn wildcard(pattern: &str, s: &str) -> bool {
    let mut parts = pattern.split('*');
    let Some(mut rest) = s.strip_prefix(parts.next().expect("a part")) else {
        return false;
    };
    let last = parts.next_back().expect("a `*`");
    for part in parts {
        let Some(pos) = rest.find(part) else {
            return false;
        };
        rest = &rest[pos + part.len()..];
    }
    rest.ends_with(last)
}
