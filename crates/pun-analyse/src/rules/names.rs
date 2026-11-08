//! The rules that check a class name exists where code names one: class constant fetches (ClassConstantRule),
//! `instanceof`, `catch`, native parameter and return types, `extends`, `implements` and `use` of traits.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use mago_word::word;
use pun_phpstan::types::{Level, Tri, Type, union};

use super::class_names::{self, Function, Location, Pair};
use super::reflection::{Known, known};
use super::scope::Guard;
use super::{Checker, DISCOVERING_SYMBOLS_TIP};

/// ClassConstantRule must infer every expression receiver independently of the restricted-member rules.
pub(super) fn typed_class_constants(program: &Program, level: u8) -> Vec<Span> {
    struct Wanted {
        spans: Vec<Span>,
        level: u8,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_class_constant_access(
            &mut self,
            a: &'ast ClassConstantAccess<'arena>,
            (): &mut (),
        ) {
            let class = super::types::flow::unparen(a.class);
            if self.level >= 2
                && !matches!(
                    class,
                    Expression::Identifier(_)
                        | Expression::Self_(_)
                        | Expression::Static(_)
                        | Expression::Parent(_)
                )
                || matches!(class, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
            {
                self.spans.push(class.span());
                if let Expression::ArrayAccess(a) = class {
                    self.spans.extend([a.array.span(), a.index.span()]);
                }
            }
        }
    }
    let mut wanted = Wanted {
        spans: Vec::new(),
        level,
    };
    wanted.walk_program(program, &mut ());
    wanted.spans
}

/// ClassConstantRule::processNode(): a written constant or every finite name of a dynamic selector. Inferred
/// expression types already reflect branch narrowing; an arbitrary string or a non-string union selects none.
pub(super) fn class_constant<'arena>(
    c: &mut Checker<'_, 'arena>,
    access: &ClassConstantAccess<'arena>,
) {
    let mut names = Vec::new();
    if let ClassLikeConstantSelector::Identifier(id) = &access.constant {
        names.push(String::from_utf8_lossy(id.value).into_owned());
    }
    if let ClassLikeConstantSelector::Expression(selector) = &access.constant
        && let Some(ty) = c.phpstan_type(selector.expression)
    {
        names = constant_strings(&ty);
    }
    // PHPStan keys constantNameScopes by the selected name, rather than checking a repeated value twice.
    let mut seen = std::collections::HashSet::new();
    for name in names {
        if seen.insert(name.clone()) {
            class_constant_named(c, access, &name);
        }
    }
}

/// `getConstantStrings()`: UnionType requires strings in every member, while IntersectionType collects them
/// from each member (an accessory contributes none). TemplateConstantStringType retains its bound's value.
pub(super) fn constant_strings(ty: &Type) -> Vec<String> {
    match ty {
        Type::ConstString(s) => vec![String::from_utf8_lossy(s).into_owned()],
        Type::Template(t) => constant_strings(&t.bound),
        Type::Union(parts) | Type::Benevolent(parts) => {
            let strings: Vec<_> = parts.iter().map(constant_strings).collect();
            if strings.iter().any(Vec::is_empty) {
                Vec::new()
            } else {
                strings.concat()
            }
        }
        Type::Intersection(parts) => parts.iter().flat_map(constant_strings).collect(),
        _ => Vec::new(),
    }
}

/// ClassConstantRule::processSingleClassConstFetch() for the class names/keywords this rule resolves.
fn class_constant_named(c: &mut Checker, access: &ClassConstantAccess, constant_name: &str) {
    let is_class = constant_name.eq_ignore_ascii_case("class");
    let written = c.written(access.class.span());
    let (class, described) = match (access.class, c.class_scope()) {
        (Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_), None) => {
            let identifier = super::out_of_class(access.class);
            c.report(
                access,
                identifier,
                format!("Using {written} outside of class scope."),
                None,
            );
            return;
        }
        (Expression::Self_(_) | Expression::Static(_), Some(scope)) => {
            let display = c.display(scope);
            let described = if matches!(access.class, Expression::Static(_)) {
                format!("static({display})")
            } else {
                display
            };
            (Known::User(scope), described)
        }
        (Expression::Parent(_), Some(scope)) => {
            let parent = scope
                .direct_parent_class
                .and_then(|p| known(c, &String::from_utf8_lossy(p.as_bytes())));
            match parent {
                Some(p) => (p, p.display()),
                None => {
                    c.report_no_parent(
                        access,
                        scope,
                        &format!("Access to parent::{constant_name}"),
                    );
                    return;
                }
            }
        }
        (Expression::Identifier(id), _) => {
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
                    class_names::check(c, &[Pair::name(name.clone(), id.span())], true);
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
        _ => {
            class_constant_expression(c, access, constant_name);
            return;
        }
    };
    if is_class || c.has_class_constant_type(access, constant_name) {
        return;
    }
    // DefinedConstantTypeSpecifyingExtension specifies a written fetch (`X::NAME`), not the original dynamic
    // expression (`X::{$name}`). Selecting the same finite name does not make their expression keys equal.
    let guarded = match access.class {
        Expression::Identifier(id) => c.resolved(id),
        _ => written,
    };
    let guarded =
        Guard::ClassConstant(format!("{}::{constant_name}", guarded.to_ascii_lowercase()));
    if matches!(access.constant, ClassLikeConstantSelector::Identifier(_))
        && c.guards.contains(&guarded)
    {
        return;
    }
    match class {
        Known::User(k) => {
            let Some(found) = k
                .constants
                .get(&word(constant_name.as_bytes()))
                .map(|m| (m.visibility, m.span))
            else {
                if !k.enum_cases.contains_key(&word(constant_name.as_bytes())) {
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
                        .get(&word(constant_name.as_bytes()))
                        .is_some_and(|m| m.span == span)
                })
                .last()
                .unwrap_or(k);
            let declaring_display = c.display(declaring);
            let declaring_name = String::from_utf8_lossy(declaring.original_name.as_bytes());
            if !super::calls::can_call(c, c.class_scope(), &declaring_name, visibility) {
                let v = super::restricted(visibility);
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
            let name = constant_name;
            let has = |k: &pun_phpstan::symbols::Class| {
                k.constants.iter().any(|(n, _)| n == name) || k.cases.iter().any(|n| n == name)
            };
            let classes = &c.builtins.classes;
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

/// The expression branch of `processSingleClassConstFetch()`. RuleLevelHelper checks only `$this` below level 2.
fn class_constant_expression(c: &mut Checker, access: &ClassConstantAccess, name: &str) {
    let class = super::types::flow::unparen(access.class);
    if c.level < 2
        && !matches!(class, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
    {
        return;
    }
    let criteria = |t: &Type| constant_capable(c, t).yes() && has_class_constant(c, t, name).yes();
    let mut found = c.find_type_to_check(class, &criteria);
    // ConstantArrayType::getOffsetValueType() returns ErrorType for an absent sealed-shape key, not Mago's
    // null recovery value. Feed that through RuleLevelHelper as well (strict implicit mixed at level 10).
    if let Expression::ArrayAccess(a) = class
        && let Some(Type::ConstArray(array)) = c.phpstan_type(a.array)
        && array.unsealed.is_none()
        && let Some(index) = c.phpstan_type(a.index)
        && array
            .keys
            .iter()
            .all(|key| pun_phpstan::types::is_super_type_of(key, &index).no())
    {
        found.ty = c.find_type_to_check_of(Some(Type::Error), &criteria);
    }
    // NullsafeOperatorHelper runs before RuleLevelHelper's unknown-class and union checks. Remove only
    // short-circuit null, not a nullable member's own result.
    if super::assign::contains_nullsafe(class)
        && c.type_of(class).is_some_and(|t| t.has_nullsafe_null())
    {
        let nonnull = c.phpstan_type(class).map(Type::remove_null);
        found.unknown = nonnull
            .as_ref()
            .map(Type::object_class_names)
            .unwrap_or_default()
            .into_iter()
            .filter(|n| known(c, n).is_none_or(|k| k.is_trait()) && !c.in_class_exists(n))
            .collect();
        found.ty = c.find_type_to_check_of(nonnull, &criteria);
    }
    for class in found.unknown {
        c.report(
            access,
            "class.notFound",
            format!("Access to constant {name} on an unknown class {class}."),
            Some(DISCOVERING_SYMBOLS_TIP),
        );
    }
    let Some(mut ty) = found.ty else { return };
    let is_class = name.eq_ignore_ascii_case("class");
    if is_class {
        if !c.surely_at_least(80000) {
            c.report_non_ignorable(access, "classConstant.notSupported", "Accessing ::class constant on an expression is supported only on PHP 8.0 and later.".into(), None);
            return;
        }
        if !matches!(class, Expression::Literal(Literal::String(_))) && ty.is_string().yes() {
            c.report_non_ignorable(
                access,
                "classConstant.dynamicString",
                "Accessing ::class constant on a dynamic string is not supported in PHP.".into(),
                None,
            );
            return;
        }
    }
    if ty.is_string().yes() {
        return;
    }
    let described = super::declared::with_classes(c, || match &ty {
        Type::This(n) => c
            .class_like(n)
            .found()
            .map(|k| c.describe_object(k))
            .unwrap_or(Type::Object(n.clone(), vec![], false).describe(Level::TypeOnly)),
        t => t.describe(Level::TypeOnly),
    });
    // TypeCombinator::remove(StringType): string alternatives do not participate in the object checks.
    if let Type::Union(parts) | Type::Benevolent(parts) = &ty {
        let removed = union(
            parts
                .iter()
                .filter(|t| !t.is_string().yes())
                .cloned()
                .collect(),
        );
        ty = if matches!(ty, Type::Benevolent(_)) {
            pun_phpstan::types::to_benevolent(removed)
        } else {
            removed
        };
    }
    if !constant_capable(c, &ty).yes() {
        c.report(
            access,
            "classConstant.nonObject",
            format!("Cannot access constant {name} on {described}."),
            None,
        );
        return;
    }
    if is_class || c.has_class_constant_type(access, name) {
        return;
    }
    if !has_class_constant(c, &ty, name).yes() {
        c.report(
            access,
            "classConstant.notFound",
            format!("Access to undefined constant {described}::{name}."),
            None,
        );
        return;
    }
    // getInternal()/getConstant() select the first definite declaration only after the whole type passed
    // hasConstant(). This is not a first-class approximation of a partially supported receiver union.
    let constant = ty
        .object_class_names()
        .iter()
        .find_map(|n| known(c, n)?.constant(c, name));
    if let Some(constant) = constant
        && let Some(visibility) = constant.visibility
        && !super::calls::can_call(c, c.class_scope(), &constant.declaring, visibility)
    {
        let v = super::restricted(visibility);
        c.report(
            access,
            if visibility.is_private() {
                "classConstant.private"
            } else {
                "classConstant.protected"
            },
            format!(
                "Access to {v} constant {name} of class {}.",
                constant.display
            ),
            None,
        );
    }
}

/// `canAccessConstants()`: the checked StrictMixedType is distinct from ordinary mixed.
fn constant_capable(c: &Checker, ty: &Type) -> Tri {
    ty.query_members(&|ty| match ty {
        Type::Object(..)
        | Type::Static(..)
        | Type::This(_)
        | Type::EnumCase(..)
        | Type::ObjectWithoutClass
        | Type::ObjectShape(_)
        | Type::Closure(_) => Tri::Yes,
        Type::ConstString(n) => {
            if known(c, &String::from_utf8_lossy(n)).is_some() {
                Tri::Yes
            } else {
                Tri::No
            }
        }
        Type::Callable(_)
        | Type::Iterable(..)
        | Type::Accessory(pun_phpstan::types::Accessory::HasMethod(_)) => Tri::Maybe,
        _ => Tri::No,
    })
}

/// `hasConstant()`: unions require every member; intersections need one declaration.
fn has_class_constant(c: &Checker, ty: &Type, name: &str) -> Tri {
    match ty {
        Type::Union(ts) | Type::Benevolent(ts) => {
            Tri::extreme_identity(ts.iter().map(|t| has_class_constant(c, t, name)))
        }
        Type::Intersection(ts) => Tri::max_min(ts.iter().map(|t| has_class_constant(c, t, name))),
        Type::Template(t) => has_class_constant(c, &t.bound, name),
        Type::ObjectWithoutClass
        | Type::ObjectShape(_)
        | Type::Closure(_)
        | Type::Callable(_)
        | Type::Iterable(..)
        | Type::Accessory(pun_phpstan::types::Accessory::HasMethod(_)) => Tri::Maybe,
        Type::Object(n, _, _) | Type::Static(n, _) | Type::This(n) | Type::EnumCase(n, _) => {
            match known(c, n) {
                Some(k) if k.constant(c, name).is_some() => Tri::Yes,
                Some(k) if k.is_final() => Tri::No,
                _ => Tri::Maybe,
            }
        }
        Type::ConstString(n) => has_class_constant(
            c,
            &Type::Object(String::from_utf8_lossy(n).into_owned(), vec![], false),
            name,
        ),
        Type::Mixed { .. } | Type::SubtractedMixed { .. } | Type::Error => Tri::Yes,
        _ => Tri::No,
    }
}

/// ExistingClassInInstanceOfRule for `expr instanceof class`.
pub(super) fn instanceof<'arena>(
    c: &mut Checker<'_, 'arena>,
    expr: &Expression<'arena>,
    class: &Expression<'arena>,
) {
    match class {
        Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_) => {
            if c.classes.is_empty() {
                let identifier = super::out_of_class(class);
                let lower = identifier.trim_start_matches("outOfClass.");
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
                Some(k) => {
                    let case = c.on(crate::rules::Switches::CHECK_CLASS_CASE_SENSITIVITY);
                    let pair = Pair::name(name.clone(), id.span());
                    class_names::check_at(c, &[pair], Some(&Location::Instanceof), case);
                    if k.is_trait() {
                        instanceof_trait(c, expr, &name);
                    }
                }
            }
        }
        _ => {}
    }
}

/// The `instanceof.trait` part of ExistingClassInInstanceOfRule.
fn instanceof_trait<'arena>(c: &mut Checker<'_, 'arena>, expr: &Expression<'arena>, name: &str) {
    // `$this` uses its class/trait-use scope; infer::TypedExpressions requests the other operands.
    let described = match expr {
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => {
            c.class_scope().map(|k| {
                format!(
                    "$this({})",
                    String::from_utf8_lossy(k.original_name.as_bytes())
                )
            })
        }
        _ => c.phpstan_type(expr).map(|ty| ty.describe(Level::TypeOnly)),
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

/// CaughtExceptionExistenceRule.
pub(super) fn catch<'arena>(c: &mut Checker<'_, 'arena>, clause: &TryCatchClause<'arena>) {
    let mut names = Vec::new();
    hint_classes(c, &clause.hint, None, &mut names);
    for (name, at) in names {
        match known(c, &name) {
            None if c.in_class_exists(&name) => {}
            None => c.report(
                at,
                "class.notFound",
                format!("Caught class {name} not found."),
                Some(DISCOVERING_SYMBOLS_TIP),
            ),
            Some(k) => {
                if !k.is_interface() && !implements_throwable(c, k) {
                    c.report(
                        at,
                        "catch.notThrowable",
                        format!("Caught class {} is not an exception.", k.display()),
                        None,
                    );
                }
                let case = c.on(crate::rules::Switches::CHECK_CLASS_CASE_SENSITIVITY);
                class_names::check_at(c, &[Pair::name(name, at)], Some(&Location::Catch), case);
            }
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
    compound: Option<&Hint<'arena>>,
    out: &mut Vec<(String, Span)>,
) {
    let root = compound.or(Some(hint));
    match hint {
        Hint::Identifier(id) => out.push((c.resolved(id), id.span())),
        // outside a class `self` and `static` are classes of that name (ParserNodeTypeToPHPStanType)
        Hint::Self_(k) if c.class_scope().is_none() => out.push(("self".to_string(), k.span())),
        Hint::Static(k) if c.class_scope().is_none() => out.push(("static".to_string(), k.span())),
        Hint::Parent(k) => {
            // InitializerExprTypeResolver makes a bare missing parent NonexistentParentClassType (no referenced
            // classes; signatures.rs handles noParent). A compound uses ParserNodeTypeToPHPStanType instead.
            let parent = c
                .class_scope()
                .and_then(|k| k.direct_parent_class)
                .map(|p| String::from_utf8_lossy(p.as_bytes()).into_owned());
            if let Some(compound) = compound
                && parent.as_deref().is_none_or(|p| known(c, p).is_none())
                // TypeCombinator removes parent from e.g. parent|object and parent|mixed.
                && super::generics::referenced_classes(&super::declared::native(c, compound, c.class_scope()))
                    .iter().any(|name| name.eq_ignore_ascii_case(parent.as_deref().unwrap_or("parent")))
            {
                out.push((c.written(k.span()), k.span()));
            }
        }
        Hint::Parenthesized(p) => hint_classes(c, p.hint, root, out),
        Hint::Nullable(n) => hint_classes(c, n.hint, root, out),
        Hint::Union(u) => {
            hint_classes(c, u.left, root, out);
            hint_classes(c, u.right, root, out);
        }
        // an intersection that cannot be inhabited is `never`, with no classes
        Hint::Intersection(_) if super::signatures::unresolvable(c, hint) => {}
        Hint::Intersection(i) => {
            hint_classes(c, i.left, root, out);
            hint_classes(c, i.right, root, out);
        }
        _ => {}
    }
}

/// Where a function-like's typehint errors go: PHPStan's messages name the function differently per kind.
#[derive(Clone, Copy)]
pub(super) enum FunctionKind<'s> {
    Function(&'s str),
    Method(&'s str, &'s str),
    Anonymous,
    /// class, property, hook name (`get`, `set`)
    Hook(&'s str, &'s str, &'s str),
}

/// The class-existence part of FunctionDefinitionCheck for native parameter and return types.
pub(super) fn typehints<'arena>(
    c: &mut Checker<'_, 'arena>,
    kind: FunctionKind,
    parameters: &FunctionLikeParameterList<'arena>,
    return_type: Option<&FunctionLikeReturnTypeHint<'arena>>,
) {
    let parameter_message = format!("of {}", kind.named());
    let return_message = format!("{} has invalid return type", kind.subject());
    let anonymous = matches!(kind, FunctionKind::Anonymous);
    for p in &parameters.parameters {
        let Some(hint) = &p.hint else {
            continue;
        };
        let mut names = Vec::new();
        hint_classes(c, hint, None, &mut names);
        let name = String::from_utf8_lossy(p.variable.name).into_owned();
        // the parameter's line for functions and methods, the type's for closures
        let at = if anonymous { hint.span() } else { p.span() };
        for (class, _) in &names {
            let identifier = match known(c, class) {
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
        // a closure's pairs are on the type node, a function's on the parameter
        let pairs = typehint_pairs(
            c,
            &names,
            at,
            anonymous && matches!(hint, Hint::Identifier(_)),
        );
        let case = c.on(crate::rules::Switches::CHECK_CLASS_CASE_SENSITIVITY);
        let location = location_function(c, kind).map(|function| Location::ParameterType {
            parameter: name[1..].to_string(),
            function,
        });
        class_names::check_at(c, &pairs, location.as_ref(), case);
    }
    let Some(r) = return_type else {
        return;
    };
    let mut names = Vec::new();
    hint_classes(c, &r.hint, None, &mut names);
    for (class, _) in &names {
        let identifier = match known(c, class) {
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
    let pairs = typehint_pairs(
        c,
        &names,
        r.hint.span(),
        matches!(r.hint, Hint::Identifier(_)),
    );
    let case = c.on(crate::rules::Switches::CHECK_CLASS_CASE_SENSITIVITY);
    let location = location_function(c, kind).map(Location::ReturnType);
    class_names::check_at(c, &pairs, location.as_ref(), case);
}

/// The function-like a ClassNameUsageLocation names.
pub(super) fn location_function(c: &Checker, kind: FunctionKind) -> Option<Function> {
    match kind {
        FunctionKind::Anonymous => Some(Function::Anonymous),
        FunctionKind::Function(name) => Some(Function::Named(name.to_string())),
        FunctionKind::Method(_, method) => Some(Function::Method {
            name: method.to_string(),
            class: c
                .class_scope()
                .filter(|k| !k.name.as_bytes().starts_with(b"{anonymous-class:"))
                .map(|k| String::from_utf8_lossy(k.original_name.as_bytes()).into_owned()),
        }),
        // ponytail: a hook's PhpMethodFromParserNodeReflection location is not built
        FunctionKind::Hook(..) => None,
    }
}

/// FunctionDefinitionCheck's ClassNameNodePairs for a type's classes: a name written in another case than it
/// resolves to (`bar` imported by `use Foo\Bar;`) is paired as written with its own node
/// (getOriginalClassNamePairsFromTypeNode()), the others with `at`, the type node itself when `at_is_name`.
fn typehint_pairs(c: &Checker, names: &[(String, Span)], at: Span, at_is_name: bool) -> Vec<Pair> {
    names
        .iter()
        .map(|(class, span)| {
            // Unresolved `self`, `static` and `parent` are keywords, not names.
            let original = match class.as_str() {
                "self" | "static" => class.clone(),
                _ if class.eq_ignore_ascii_case("parent") => class.clone(),
                _ => class_names::original_case(c, *span),
            };
            if original != *class {
                Pair::name(original, *span)
            } else if at_is_name {
                Pair::name(class.clone(), at)
            } else {
                Pair::node(class.clone(), at)
            }
        })
        .collect()
}

/// The `#[NoDiscard]` part of FunctionDefinitionCheck: the attribute on a function-like whose return type is void or
/// never. `at` is the function-like (the line when no return type is written). A function or method reads its
/// reflection's return type: `forced_void` for the methods PHPStan always types void (constructors, `__destruct()`,
/// `__set()` & co.), else the native type, else the PHPDoc type Mago read (`docblock`). An anonymous function reads
/// only a native `void` or `never`.
pub(super) fn no_discard<'arena>(
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
    let message = format!("{described} {}", kind.named());
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
pub(super) fn class_hierarchy<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: Span,
    current: &str,
    extends: Option<&Extends<'arena>>,
    implements: Option<&Implements<'arena>>,
    interface: bool,
    readonly: bool,
) {
    let class = current.split_once(' ').map(|(_, name)| name.to_string());
    let pairs = |c: &Checker,
                 ids: Option<&TokenSeparatedSequence<'arena, Identifier<'arena>>>|
     -> Vec<Pair> {
        ids.into_iter()
            .flat_map(|ids| ids.iter())
            .map(|id| Pair::name(c.resolved(id), id.span()))
            .collect()
    };
    let extended = pairs(c, extends.map(|e| &e.types));
    let location = Location::Extends {
        subject: current.to_string(),
        class: class.clone(),
    };
    class_names::check_at(c, &extended, Some(&location), true);
    let implemented = pairs(c, implements.map(|i| &i.types));
    let location = Location::Implements {
        subject: current.to_string(),
        class,
    };
    class_names::check_at(c, &implemented, Some(&location), true);
    if let Some(e) = extends {
        for id in &e.types {
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
    for id in &i.types {
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
pub(super) fn property_type<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: Span,
    hint: Option<&Hint<'arena>>,
    names: &[&[u8]],
) {
    // properties are walked inside their class-like (trait bodies only in a class's context)
    let class = c
        .class_scope()
        .expect("a property is walked inside its class-like");
    let display = c.display(class);
    let mut native = Vec::new();
    if let Some(hint) = hint {
        hint_classes(c, hint, None, &mut native);
    }
    for name in names {
        let property =
            String::from_utf8_lossy(name.strip_prefix(b"$").unwrap_or(name)).into_owned();
        // and from level 2 (`checkThisOnly` off) the classes of its PHPDoc type
        // ponytail: a promoted property's type from the constructor's `@param` is not read
        let mut classes = native.clone();
        if c.level >= 2 {
            let phpdoc = super::declared::with_classes(c, || {
                super::declared::property(c, None, at, &name[1..], class)
            });
            let referenced = super::generics::referenced_classes(&phpdoc)
                .into_iter()
                .filter(|n| !n.contains('-'));
            classes.extend(referenced.map(|n| (n, at)));
        }
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
        let pairs: Vec<Pair> = classes
            .iter()
            .map(|(referenced, _)| Pair::node(referenced.clone(), at))
            .collect();
        let case = c.on(crate::rules::Switches::CHECK_CLASS_CASE_SENSITIVITY);
        let location = Location::PropertyType { property };
        class_names::check_at(c, &pairs, Some(&location), case);
    }
}

/// ExistingNamesInUseRule and ExistingNamesInGroupUseRule for class names (ClassNameCheck alone, so a missing name,
/// which may be a namespace, is no error).
pub(super) fn use_classes<'arena>(c: &mut Checker<'_, 'arena>, u: &Use<'arena>) {
    let text = |id: &Identifier| {
        String::from_utf8_lossy(id.value())
            .trim_start_matches('\\')
            .to_string()
    };
    let pairs: Vec<Pair> = match &u.items {
        UseItems::Sequence(s) => s
            .items
            .iter()
            .map(|item| Pair::node(text(&item.name), item.name.span()))
            .collect(),
        UseItems::MixedList(l) => {
            let prefix = text(&l.namespace);
            l.items
                .iter()
                .filter(|i| i.r#type.is_none())
                .map(|i| Pair::node(format!("{prefix}\\{}", text(&i.item.name)), i.span()))
                .collect()
        }
        UseItems::TypedSequence(_) | UseItems::TypedList(_) => Vec::new(),
    };
    class_names::check(c, &pairs, true);
}

/// ExistingNamesInUseRule and ExistingNamesInGroupUseRule for `use function` and `use const`.
pub(super) fn use_functions<'arena>(c: &mut Checker<'_, 'arena>, u: &Use<'arena>) {
    // (is a function, the name, the item)
    let mut used: Vec<(bool, String, Span)> = Vec::new();
    let text = |id: &Identifier| {
        String::from_utf8_lossy(id.value())
            .trim_start_matches('\\')
            .to_string()
    };
    let is_function = |t: &UseType| matches!(t, UseType::Function(_));
    match &u.items {
        UseItems::TypedSequence(s) => {
            for item in &s.items {
                used.push((is_function(&s.r#type), text(&item.name), item.span()));
            }
        }
        UseItems::TypedList(l) => {
            let prefix = text(&l.namespace);
            for item in &l.items {
                used.push((
                    is_function(&l.r#type),
                    format!("{prefix}\\{}", text(&item.name)),
                    item.span(),
                ));
            }
        }
        UseItems::MixedList(l) => {
            let prefix = text(&l.namespace);
            for item in &l.items {
                if let Some(t) = &item.r#type {
                    used.push((
                        is_function(t),
                        format!("{prefix}\\{}", text(&item.item.name)),
                        item.span(),
                    ));
                }
            }
        }
        UseItems::Sequence(_) => {}
    }
    for (function, name, at) in used {
        let (kind, identifier, found) = if function {
            (
                "function",
                "function.notFound",
                c.function(name.as_bytes()).is_some(),
            )
        } else {
            ("constant", "constant.notFound", has_constant(c, &name))
        };
        if !found {
            c.report(
                at,
                identifier,
                format!("Used {kind} {name} not found."),
                Some(DISCOVERING_SYMBOLS_TIP),
            );
        }
    }
}

/// `MutatingScope::fileHasCompilerHaltStatementCalls()`, on the already parsed program.
pub(super) fn file_has_compiler_halt_statement_calls(program: &Program) -> bool {
    program.statements.iter().any(|statement| match statement {
        Statement::HaltCompiler(_) => true,
        // PhpParser leaves HaltCompiler at the root of a semicolon namespace; Mago keeps it in the body.
        Statement::Namespace(namespace) if matches!(namespace.body, NamespaceBody::Implicit(_)) => {
            namespace
                .statements()
                .iter()
                .any(|statement| matches!(statement, Statement::HaltCompiler(_)))
        }
        _ => false,
    })
}

/// ConstantRule (level 1): a constant fetch that `$scope->hasConstant()` does not know: not defined in the project or
/// built in, at the namespaced name and then the global one for an unqualified name, nor guarded by `defined()`.
pub(super) fn constant_fetch(c: &mut Checker, id: &Identifier) {
    if c.level < 1 {
        return;
    }
    let resolved = c.resolved_span(id.span());
    let written = String::from_utf8_lossy(id.value())
        .trim_start_matches('\\')
        .to_string();
    // an unqualified name that no `use const` imports falls back to the global constant
    let fallback = matches!(id, Identifier::Local(l) if !c.names.is_imported(&l.span));
    let mut candidates = vec![resolved.clone()];
    if fallback {
        candidates.push(written.clone());
    }
    let name = if fallback { written } else { resolved };
    // Scope tests the Name itself before namespaced fallback, reflection or defined() guards.
    let found = if name == "__COMPILER_HALT_OFFSET__" {
        c.walk.contains(super::Walk::COMPILER_HALT)
    } else {
        candidates.iter().any(|name| {
            ["true", "false", "null"]
                .iter()
                .any(|k| name.eq_ignore_ascii_case(k))
                || has_constant(c, name)
                || c.guards.contains(&Guard::Constant(name.clone()))
        })
    };
    if !found {
        c.report(
            id,
            "constant.notFound",
            format!("Constant {name} not found."),
            Some(DISCOVERING_SYMBOLS_TIP),
        );
    }
}

/// `ReflectionProvider::hasConstant()` of a fully qualified name: a project `const` or `define()` (analysed or
/// scanned), else a built-in.
fn has_constant(c: &Checker, name: &str) -> bool {
    // PHPStan does not discover a `define()` inside a class-like
    c.codebase.get_constant(name.as_bytes()).is_some_and(|k| {
        !k.flags.is_built_in()
            && !c.codebase.class_likes.values().any(|o| {
                o.span.file_id == k.span.file_id
                    && o.span.start.offset <= k.span.start.offset
                    && k.span.end.offset <= o.span.end.offset
            })
    }) || c.builtins.constants.contains_key(name)
}
