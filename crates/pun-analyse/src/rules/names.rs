//! The rules that check a class name exists where code names one: class constant fetches (ClassConstantRule),
//! `instanceof`, `catch`, native parameter and return types, `extends`, `implements` and `use` of traits.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::TType;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_word::word;

use super::scope::Guard;
use super::{Checker, DISCOVERING_SYMBOLS_TIP, builtins};

/// What PHPStan's reflection knows of a class: user code from Mago, built-ins from data/symbols.json.
#[derive(Clone, Copy)]
pub enum Known<'a> {
    User(&'a ClassLikeMetadata),
    Builtin(&'static pun_phpstan::symbols::Class),
}

impl Known<'_> {
    pub fn is_trait(&self) -> bool {
        matches!(self, Known::User(k) if k.kind == SymbolKind::Trait)
    }

    pub fn is_interface(&self) -> bool {
        match self {
            Known::User(k) => k.kind == SymbolKind::Interface,
            Known::Builtin(k) => k.is_interface,
        }
    }

    pub fn is_enum(&self) -> bool {
        match self {
            Known::User(k) => k.kind == SymbolKind::Enum,
            Known::Builtin(k) => !k.cases.is_empty(),
        }
    }

    pub fn is_class(&self) -> bool {
        !self.is_trait() && !self.is_interface() && !self.is_enum()
    }

    /// `getClassTypeDescription()`.
    pub(super) fn type_description(&self) -> &'static str {
        if self.is_interface() {
            "Interface"
        } else if self.is_trait() {
            "Trait"
        } else if self.is_enum() {
            "Enum"
        } else {
            "Class"
        }
    }

    pub fn display(&self) -> String {
        match self {
            Known::User(k) => String::from_utf8_lossy(k.original_name.as_bytes()).into_owned(),
            Known::Builtin(k) => k.name.clone(),
        }
    }
}

/// `ReflectionProvider::hasClass()` / `getClass()`.
pub fn known<'a>(c: &Checker<'a, '_>, name: &str) -> Option<Known<'a>> {
    let name = name.trim_start_matches('\\');
    if let Some(k) = c
        .codebase
        .get_class_like(name.as_bytes())
        .filter(|k| !k.flags.is_built_in())
    {
        return Some(Known::User(k));
    }
    builtins()
        .classes
        .get(&name.to_ascii_lowercase())
        .map(Known::Builtin)
}

/// ClassConstantRule for `X::NAME` with a written class and constant name.
pub fn class_constant<'arena>(c: &mut Checker<'_, 'arena>, access: &ClassConstantAccess<'arena>) {
    let ClassLikeConstantSelector::Identifier(constant) = &access.constant else {
        return; // ponytail: dynamic constant names (`X::{$name}`) unchecked
    };
    let constant_name = String::from_utf8_lossy(constant.value).into_owned();
    let is_class = constant_name.eq_ignore_ascii_case("class");
    let in_class = !c.classes.is_empty();
    let written = String::from_utf8_lossy(&c.file.contents[access.class.span().to_range_usize()])
        .into_owned();
    let (class, described) = match access.class {
        Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_) if !in_class => {
            let identifier = match access.class {
                Expression::Self_(_) => "outOfClass.self",
                Expression::Static(_) => "outOfClass.static",
                _ => "outOfClass.parent",
            };
            c.report(
                access,
                identifier,
                format!("Using {written} outside of class scope."),
                None,
            );
            return;
        }
        Expression::Self_(_) | Expression::Static(_) => {
            let Some(scope) = c.class_scope() else {
                return; // ponytail: trait and anonymous class scopes
            };
            let display = String::from_utf8_lossy(scope.original_name.as_bytes()).into_owned();
            let described = if matches!(access.class, Expression::Static(_)) {
                format!("static({display})")
            } else {
                display
            };
            (Known::User(scope), described)
        }
        Expression::Parent(_) => {
            let Some(scope) = c.class_scope() else {
                return;
            };
            let parent = scope
                .direct_parent_class
                .and_then(|p| known(c, &String::from_utf8_lossy(p.as_bytes())));
            match parent {
                Some(p) => (p, p.display()),
                None => {
                    let display =
                        String::from_utf8_lossy(scope.original_name.as_bytes()).into_owned();
                    c.report(
                        access,
                        "class.noParent",
                        format!("Access to parent::{constant_name} but {display} does not extend any class."),
                        None,
                    );
                    return;
                }
            }
        }
        Expression::Identifier(id) => {
            let name = c.resolved(id);
            match known(c, &name) {
                Some(k) => {
                    if !is_class && k.is_trait() {
                        c.report(
                            access,
                            "classConstant.onTrait",
                            format!(
                                "Cannot access constant {constant_name} on trait {}.",
                                k.display()
                            ),
                            None,
                        );
                        return;
                    }
                    (k, k.display())
                }
                None if c.in_class_exists(&name) => return,
                None => {
                    let message = if is_class {
                        format!("Class {name} not found.")
                    } else {
                        format!("Access to constant {constant_name} on an unknown class {name}.")
                    };
                    c.report(
                        access,
                        "class.notFound",
                        message,
                        Some(DISCOVERING_SYMBOLS_TIP),
                    );
                    return;
                }
            }
        }
        _ => return, // ponytail: `$expr::NAME` needs the expression's type
    };
    if is_class {
        return;
    }
    // `defined('X::NAME')` makes the fetch's type known (`hasExpressionType()`)
    let guarded = match access.class {
        Expression::Identifier(id) => c.resolved(id),
        _ => written.clone(),
    };
    if c.guards.contains(&Guard::ClassConstant(format!(
        "{}::{constant_name}",
        guarded.to_ascii_lowercase()
    ))) {
        return;
    }
    match class {
        Known::User(k) => {
            let Some(found) = k
                .constants
                .get(&word(constant.value))
                .map(|m| (m.visibility, m.span))
            else {
                if !k.enum_cases.contains_key(&word(constant.value)) {
                    c.report(
                        access,
                        "classConstant.notFound",
                        format!("Access to undefined constant {described}::{constant_name}."),
                        None,
                    );
                }
                return;
            };
            let (visibility, span) = found;
            if visibility.is_public() {
                return;
            }
            // the declaring class: the farthest ancestor holding the same declaration
            let declaring = std::iter::once(&k.name)
                .chain(k.all_parent_classes.iter())
                .filter_map(|n| c.codebase.get_class_like(n.as_bytes()))
                .filter(|a| {
                    a.constants
                        .get(&word(constant.value))
                        .is_some_and(|m| m.span == span)
                })
                .last()
                .unwrap_or(k);
            let declaring_display =
                String::from_utf8_lossy(declaring.original_name.as_bytes()).into_owned();
            if !super::calls::can_call(c, c.class_scope(), &declaring_display, visibility) {
                let v = if visibility.is_private() {
                    "private"
                } else {
                    "protected"
                };
                c.report(
                    access,
                    if visibility.is_private() {
                        "classConstant.private"
                    } else {
                        "classConstant.protected"
                    },
                    format!("Access to {v} constant {constant_name} of class {declaring_display}."),
                    None,
                );
            }
        }
        Known::Builtin(k) => {
            let name = constant_name.as_str();
            let has = |k: &pun_phpstan::symbols::Class| {
                k.constants.iter().any(|(n, _)| n == name) || k.cases.iter().any(|n| n == name)
            };
            let classes = &builtins().classes;
            if !has(k)
                && !k
                    .ancestors
                    .iter()
                    .filter_map(|a| classes.get(&a.to_ascii_lowercase()))
                    .any(has)
            {
                c.report(
                    access,
                    "classConstant.notFound",
                    format!("Access to undefined constant {described}::{constant_name}."),
                    None,
                );
            }
        }
    }
}

/// ExistingClassInInstanceOfRule for `expr instanceof class`.
pub fn instanceof<'arena>(
    c: &mut Checker<'_, 'arena>,
    expr: &Expression<'arena>,
    class: &Expression<'arena>,
) {
    match class {
        Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_) => {
            if c.classes.is_empty() {
                let (identifier, lower) = match class {
                    Expression::Self_(_) => ("outOfClass.self", "self"),
                    Expression::Static(_) => ("outOfClass.static", "static"),
                    _ => ("outOfClass.parent", "parent"),
                };
                c.report(
                    class,
                    identifier,
                    format!("Using {lower} outside of class scope."),
                    None,
                );
            }
        }
        Expression::Identifier(id) => {
            let name = c.resolved(id);
            match known(c, &name) {
                None if c.in_class_exists(&name) => {}
                None => c.report(
                    id,
                    "class.notFound",
                    format!("Class {name} not found."),
                    Some(DISCOVERING_SYMBOLS_TIP),
                ),
                Some(k) if k.is_trait() => {
                    // `$scope->getType($expr)->describe()`: only `$this` (ThisType) and `mixed` are described until
                    // analyse 4 maps Mago's types to PHPStan's descriptions
                    let described = match expr {
                        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => {
                            c.class_scope().map(|k| {
                                format!(
                                    "$this({})",
                                    String::from_utf8_lossy(k.original_name.as_bytes())
                                )
                            })
                        }
                        _ => c
                            .type_of(expr)
                            .filter(|t| t.get_id().as_bytes() == b"mixed")
                            .map(|_| "mixed".to_string()),
                    };
                    if let Some(described) = described {
                        c.report(
                            expr,
                            "instanceof.trait",
                            format!(
                                "Instanceof between {described} and trait {name} will always evaluate to false."
                            ),
                            None,
                        );
                    }
                }
                Some(_) => {}
            }
        }
        _ => {}
    }
}

/// CaughtExceptionExistenceRule.
pub fn catch<'arena>(c: &mut Checker<'_, 'arena>, clause: &TryCatchClause<'arena>) {
    let mut names = Vec::new();
    hint_classes(c, &clause.hint, &mut names);
    for (name, at) in names {
        match known(c, &name) {
            None if c.in_class_exists(&name) => {}
            None => c.report(
                at,
                "class.notFound",
                format!("Caught class {name} not found."),
                Some(DISCOVERING_SYMBOLS_TIP),
            ),
            Some(k) if !k.is_interface() && !implements_throwable(c, k) => c.report(
                at,
                "catch.notThrowable",
                format!("Caught class {} is not an exception.", k.display()),
                None,
            ),
            Some(_) => {}
        }
    }
}

fn implements_throwable(c: &Checker, k: Known) -> bool {
    match k {
        Known::User(k) => c.codebase.is_instance_of(k.name.as_bytes(), b"throwable"),
        Known::Builtin(k) => k
            .ancestors
            .iter()
            .any(|a| a.eq_ignore_ascii_case("Throwable")),
    }
}
/// The class names a native type names, resolved, with where each is written (`getReferencedClasses()`).
fn hint_classes<'arena>(
    c: &Checker<'_, 'arena>,
    hint: &Hint<'arena>,
    out: &mut Vec<(String, Span)>,
) {
    match hint {
        Hint::Identifier(id) => out.push((c.resolved(id), id.span())),
        Hint::Parenthesized(p) => hint_classes(c, p.hint, out),
        Hint::Nullable(n) => hint_classes(c, n.hint, out),
        Hint::Union(u) => {
            hint_classes(c, u.left, out);
            hint_classes(c, u.right, out);
        }
        Hint::Intersection(i) => {
            hint_classes(c, i.left, out);
            hint_classes(c, i.right, out);
        }
        _ => {}
    }
}

/// Where a function-like's typehint errors go: PHPStan's messages name the function differently per kind.
#[derive(Clone, Copy)]
pub enum FunctionKind<'s> {
    Function(&'s str),
    Method(&'s str, &'s str),
    Anonymous,
}

/// The class-existence part of FunctionDefinitionCheck for native parameter and return types.
pub fn typehints<'arena>(
    c: &mut Checker<'_, 'arena>,
    kind: FunctionKind,
    parameters: &FunctionLikeParameterList<'arena>,
    return_type: Option<&FunctionLikeReturnTypeHint<'arena>>,
) {
    let (parameter_message, return_message) = match kind {
        FunctionKind::Function(f) => (
            format!("of function {f}()"),
            format!("Function {f}() has invalid return type"),
        ),
        FunctionKind::Method(k, m) => (
            format!("of method {k}::{m}()"),
            format!("Method {k}::{m}() has invalid return type"),
        ),
        FunctionKind::Anonymous => (
            "of anonymous function".to_string(),
            "Anonymous function has invalid return type".to_string(),
        ),
    };
    let anonymous = matches!(kind, FunctionKind::Anonymous);
    for p in parameters.parameters.iter() {
        let Some(hint) = &p.hint else {
            continue;
        };
        let mut names = Vec::new();
        hint_classes(c, hint, &mut names);
        let name = String::from_utf8_lossy(p.variable.name).into_owned();
        // the parameter's line for functions and methods, the type's for closures
        let at = if anonymous { hint.span() } else { p.span() };
        for (class, _) in names {
            let identifier = match known(c, &class) {
                None => "class.notFound",
                Some(k) if k.is_trait() => "parameter.trait",
                Some(_) => continue,
            };
            c.report(
                at,
                identifier,
                format!("Parameter {name} {parameter_message} has invalid type {class}."),
                None,
            );
        }
    }
    let Some(r) = return_type else {
        return;
    };
    let mut names = Vec::new();
    hint_classes(c, &r.hint, &mut names);
    for (class, _) in names {
        let identifier = match known(c, &class) {
            None => "class.notFound",
            Some(k) if k.is_trait() => "return.trait",
            Some(_) => continue,
        };
        c.report(
            r.hint.span(),
            identifier,
            format!("{return_message} {class}."),
            None,
        );
    }
}

/// The `#[NoDiscard]` part of FunctionDefinitionCheck: the attribute on a function-like whose return type is void or
/// never. `at` is the function-like (the line when no return type is written). A function or method reads its
/// reflection's return type: `forced_void` for the methods PHPStan always types void (constructors, `__destruct()`,
/// `__set()` & co.), else the native type, else the PHPDoc type Mago read (`docblock`). An anonymous function reads
/// only a native `void` or `never`.
pub fn no_discard<'arena>(
    c: &mut Checker<'_, 'arena>,
    kind: FunctionKind,
    at: Span,
    lists: &[AttributeList<'arena>],
    return_type: Option<&FunctionLikeReturnTypeHint<'arena>>,
    forced_void: bool,
    docblock: Option<&mago_codex::ttype::union::TUnion>,
) {
    if !lists
        .iter()
        .flat_map(|l| l.attributes.iter())
        .any(|a| c.resolved(&a.name).eq_ignore_ascii_case("nodiscard"))
    {
        return;
    }
    let native = return_type.and_then(|r| match r.hint {
        Hint::Void(_) => Some("void"),
        Hint::Never(_) => Some("never"),
        _ => None,
    });
    let described = match kind {
        FunctionKind::Anonymous => native,
        _ if forced_void => Some("void"),
        _ if return_type.is_some() => native,
        _ => docblock.and_then(|t| {
            if t.is_void() {
                Some("void")
            } else {
                t.is_never().then_some("never")
            }
        }),
    };
    let Some(described) = described else {
        return;
    };
    let message = match kind {
        FunctionKind::Function(f) => format!("{described} function {f}()"),
        FunctionKind::Method(k, m) => format!("{described} method {k}::{m}()"),
        FunctionKind::Anonymous => format!("{described} anonymous function"),
    };
    c.report(
        return_type.map_or(at, |r| r.hint.span()),
        "attribute.target",
        format!("Attribute NoDiscard cannot be used on {message}."),
        None,
    );
}

/// ExistingClassInClassExtendsRule and ExistingClassesInClassImplementsRule (class), ExistingClassesInInterfaceExtendsRule
/// (interface) and ExistingClassesInEnumImplementsRule (enum): `current` is e.g. `Class Foo` or `Anonymous class`,
/// `readonly` whether the class is declared `readonly`.
pub fn class_hierarchy<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: Span,
    current: &str,
    extends: Option<&Extends<'arena>>,
    implements: Option<&Implements<'arena>>,
    interface: bool,
    readonly: bool,
) {
    if let Some(e) = extends {
        for id in e.types.iter() {
            let name = c.resolved(id);
            match known(c, &name) {
                None if c.in_class_exists(&name) => {}
                None if interface => c.report_non_ignorable(
                    at,
                    "interface.notFound",
                    format!("{current} extends unknown interface {name}."),
                    Some(DISCOVERING_SYMBOLS_TIP),
                ),
                None => c.report_non_ignorable(
                    at,
                    "class.notFound",
                    format!("{current} extends unknown class {name}."),
                    Some(DISCOVERING_SYMBOLS_TIP),
                ),
                Some(k) if interface => {
                    let what = if k.is_class() {
                        Some(("interfaceExtends.class", "class"))
                    } else if k.is_trait() {
                        Some(("interfaceExtends.trait", "trait"))
                    } else if k.is_enum() {
                        Some(("interfaceExtends.enum", "enum"))
                    } else {
                        None
                    };
                    if let Some((identifier, what)) = what {
                        c.report_non_ignorable(
                            at,
                            identifier,
                            format!("{current} extends {what} {}.", k.display()),
                            None,
                        );
                    }
                }
                Some(k) => {
                    let what = if k.is_interface() {
                        Some(("class.extendsInterface", "interface"))
                    } else if k.is_trait() {
                        Some(("class.extendsTrait", "trait"))
                    } else if k.is_enum() {
                        Some(("class.extendsEnum", "enum"))
                    } else {
                        None
                    };
                    if let Some((identifier, what)) = what {
                        c.report_non_ignorable(
                            at,
                            identifier,
                            format!("{current} extends {what} {}.", k.display()),
                            None,
                        );
                    } else {
                        super::hierarchy::extends_final(c, at, current, k);
                    }
                    if k.is_class() {
                        super::hierarchy::extends_readonly(c, at, current, readonly, k);
                    }
                }
            }
        }
    }
    let Some(i) = implements else {
        return;
    };
    let prefix = if current.starts_with("Enum ") {
        "enumImplements"
    } else {
        "classImplements"
    };
    for id in i.types.iter() {
        let name = c.resolved(id);
        match known(c, &name) {
            None if c.in_class_exists(&name) => {}
            None => c.report_non_ignorable(
                at,
                "interface.notFound",
                format!("{current} implements unknown interface {name}."),
                Some(DISCOVERING_SYMBOLS_TIP),
            ),
            Some(k) => {
                let what = if k.is_enum() {
                    "enum"
                } else if k.is_trait() {
                    "trait"
                } else if k.is_class() {
                    "class"
                } else {
                    continue;
                };
                c.report_non_ignorable(
                    at,
                    &format!("{prefix}.{what}"),
                    format!("{current} implements {what} {}.", k.display()),
                    None,
                );
            }
        }
    }
}

/// ExistingClassesInPropertiesRule (native types only at level 0, `checkThisOnly`): one property declaration or
/// promoted constructor parameter, `at` its node.
pub fn property_type<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: Span,
    hint: Option<&Hint<'arena>>,
    names: &[&[u8]],
) {
    let Some(hint) = hint else {
        return;
    };
    let Some(class) = c.class_scope() else {
        return; // ponytail: traits and anonymous classes
    };
    let display = String::from_utf8_lossy(class.original_name.as_bytes()).into_owned();
    let mut classes = Vec::new();
    hint_classes(c, hint, &mut classes);
    for name in names {
        let property =
            String::from_utf8_lossy(name.strip_prefix(b"$").unwrap_or(name)).into_owned();
        for (referenced, _) in &classes {
            match known(c, referenced) {
                None => c.report(
                    at,
                    "class.notFound",
                    format!("Property {display}::${property} has unknown class {referenced} as its type."),
                    Some(DISCOVERING_SYMBOLS_TIP),
                ),
                Some(k) if k.is_trait() => c.report(
                    at,
                    "property.trait",
                    format!("Property {display}::${property} has invalid type {referenced}."),
                    None,
                ),
                Some(_) => {}
            }
        }
    }
}

/// ExistingNamesInUseRule and ExistingNamesInGroupUseRule for `use function`.
pub fn use_functions<'arena>(c: &mut Checker<'_, 'arena>, u: &Use<'arena>) {
    let mut functions: Vec<(String, Span)> = Vec::new();
    let text = |id: &Identifier| {
        String::from_utf8_lossy(id.value())
            .trim_start_matches('\\')
            .to_string()
    };
    match &u.items {
        UseItems::TypedSequence(s) if matches!(s.r#type, UseType::Function(_)) => {
            for item in s.items.iter() {
                functions.push((text(&item.name), item.span()));
            }
        }
        UseItems::TypedList(l) if matches!(l.r#type, UseType::Function(_)) => {
            let prefix = text(&l.namespace);
            for item in l.items.iter() {
                functions.push((format!("{prefix}\\{}", text(&item.name)), item.span()));
            }
        }
        UseItems::MixedList(l) => {
            let prefix = text(&l.namespace);
            for item in l
                .items
                .iter()
                .filter(|i| matches!(i.r#type, Some(UseType::Function(_))))
            {
                functions.push((format!("{prefix}\\{}", text(&item.item.name)), item.span()));
            }
        }
        _ => {}
    }
    for (name, at) in functions {
        if c.function(name.as_bytes()).is_none() {
            c.report(
                at,
                "function.notFound",
                format!("Used function {name} not found."),
                Some(DISCOVERING_SYMBOLS_TIP),
            );
        }
    }
}
