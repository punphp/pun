//! NodeScopeResolver's throw points (ThrowPoint): what a statement list may throw, explicitly (a `throw`, a call to
//! a function-like with a `@throws` tag) or implicitly (any other call, which may throw any Throwable), and
//! TryCatchHandler's catch matching. A point Pun cannot tell (an untyped receiver, a dynamic throw type extension,
//! a conditional `@throws`) is an implicit one that `implicitThrows: false` keeps: it only ever hides an error.

use std::collections::{BTreeMap, HashMap, HashSet};

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::visibility::Visibility;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::resolve::resolve_str;
use pun_phpstan::types::{NeverKind, Type, accepts, is_super_type_of, remove_object, union};
use serde_json::Value;

use super::super::members::{self, Method as Member};
use super::super::{Checker, declared, invalid_throws, var_tags};
use super::flow::unparen;

/// A ThrowPoint.
#[derive(Clone, Debug)]
pub(in super::super) struct Point {
    /// The node, whose start line the errors report.
    pub at: Span,
    pub ty: Type,
    pub explicit: bool,
    /// `canContainAnyThrowable()`
    pub any: bool,
    /// The node is a `throw` (TryCatchHandler's `$onlyExplicitIsThrow`).
    pub throw: bool,
    /// Pun cannot tell what the node throws: an implicit point `implicitThrows: false` does not drop.
    pub unknown: bool,
}

pub(in super::super) fn throwable() -> Type {
    Type::Object("Throwable".into(), vec![], false)
}

impl Point {
    fn explicit(at: Span, ty: Type, any: bool) -> Point {
        Point { at, ty, explicit: true, any, throw: false, unknown: false }
    }
    fn implicit(at: Span) -> Point {
        Point { at, ty: throwable(), explicit: false, any: true, throw: false, unknown: false }
    }
    fn unknown(at: Span) -> Point {
        Point { unknown: true, ..Point::implicit(at) }
    }
}

/// Built-in throw types, from `scripts/export-throw-types.php`.
pub(crate) struct ThrowData {
    functions: HashMap<String, String>,
    methods: HashMap<String, HashMap<String, String>>,
    dynamic: HashSet<String>,
    later: HashMap<String, Vec<String>>,
    immediate: HashMap<String, Vec<String>>,
}

impl ThrowData {
    pub(crate) fn load() -> ThrowData {
        let v: Value = serde_json::from_str(include_str!("../../../data/throw-types.json")).expect("throw-types.json");
        let strings = |v: &Value| -> HashMap<String, String> {
            v.as_object()
                .into_iter()
                .flatten()
                .filter_map(|(k, t)| Some((k.clone(), t.as_str()?.to_string())))
                .collect()
        };
        let lists = |v: &Value| -> HashMap<String, Vec<String>> {
            v.as_object()
                .into_iter()
                .flatten()
                .map(|(k, l)| {
                    (k.clone(), l.as_array().into_iter().flatten().filter_map(|s| Some(s.as_str()?.to_string())).collect())
                })
                .collect()
        };
        ThrowData {
            functions: strings(&v["functions"]),
            methods: v["methods"].as_object().into_iter().flatten().map(|(k, m)| (k.clone(), strings(m))).collect(),
            dynamic: v["dynamic"].as_array().into_iter().flatten().filter_map(|s| Some(s.as_str()?.to_string())).collect(),
            later: lists(&v["later"]),
            immediate: lists(&v["immediate"]),
        }
    }
}

/// `getThrowType()` of a built-in method (`class` and `method` lowercase).
pub(in super::super) fn builtin_method_throws(c: &Checker, class: &str, method: &str) -> Option<Type> {
    c.natives.throws.methods.get(class)?.get(method).map(|t| resolve_str(t, None))
}

/// What a callee is, for its throw point.
enum Callee<'a> {
    User {
        f: &'a FunctionLikeMetadata,
        /// The declaring class of a method.
        declaring: Option<&'a ClassLikeMetadata>,
        name: String,
    },
    Builtin {
        /// `name` or `class::method`, lowercase
        key: String,
        f: &'static pun_phpstan::symbols::Function,
        class: Option<String>,
    },
}

/// Whether a ported Date* dynamic throw type extension gives a call `at` of the built-in method `key`
/// (`class::method`, lowercase) an explicit throw point.
pub(in super::super) fn date_explicit(c: &Checker, at: Span, key: &str, args: &ArgumentList) -> bool {
    Points::new(c).date_extension(at, key, Some(args)).flatten().is_some_and(|p| p.explicit)
}

/// The throw points of a body (not of the functions and classes declared in it).
pub(in super::super) struct Points<'c, 'a, 'arena> {
    c: &'c Checker<'a, 'arena>,
    pub points: Vec<Point>,
    /// Inside a `throw` expression (ExpressionContext::isInThrow()).
    in_throw: usize,
    /// The hooked property whose hook is walked (`$this->name` reads it without a hook call).
    hooked: Option<Vec<u8>>,
    /// Dead catches found in the try statements walked, with the caught type described.
    pub dead: Vec<(Span, Type)>,
    /// The property accesses an assignment writes (no get hook).
    writing: HashSet<Span>,
}

impl<'c, 'a, 'arena> Points<'c, 'a, 'arena> {
    pub(in super::super) fn new(c: &'c Checker<'a, 'arena>) -> Self {
        Points { c, points: Vec::new(), in_throw: 0, hooked: None, dead: Vec::new(), writing: HashSet::new() }
    }

    pub(in super::super) fn in_hook(mut self, property: &[u8]) -> Self {
        self.hooked = Some(property.to_vec());
        self
    }

    fn implicit_throws(&self) -> bool {
        self.c.exceptions.implicit_throws
    }

    /// An implicit point, when `implicitThrows` is on.
    fn implicit(&mut self, at: Span) {
        if self.implicit_throws() {
            self.points.push(Point::implicit(at));
        }
    }

    fn with_classes<R>(&self, f: impl FnOnce() -> R) -> R {
        declared::with_classes(self.c, f)
    }

    /// The points of a nested body (a closure's), walked separately.
    fn nested(&self) -> Points<'c, 'a, 'arena> {
        Points::new(self.c)
    }

    /// The PHPStan type of a typed expression.
    fn type_of(&self, e: &Expression) -> Option<Type> {
        self.c.phpstan_type(e)
    }

    /// `getThrowType()` of a callee, `Err(())` when Pun cannot tell it.
    fn throw_type(&self, callee: &Callee) -> Result<Option<Type>, ()> {
        let ty = match callee {
            Callee::User { f, declaring, name } => {
                // AnnotationMethodReflection: a `@method`'s throw type is its class's `__call` / `__callStatic`
                if f.flags.is_magic_method() {
                    let Some(class) = declaring else { return Err(()) };
                    let magic = if f.flags.is_static() {
                        "__callStatic"
                    } else {
                        "__call"
                    };
                    return match members::method(self.c, class, magic.as_bytes()) {
                        Some(m) => self.throw_type(&self.callee_of(m)),
                        None => Ok(None),
                    };
                }
                self.with_classes(|| declared::reflected_throws(self.c, f, *declaring, name))
            }
            Callee::Builtin { key, class, .. } => {
                if self.c.natives.throws.dynamic.contains(key) {
                    return Err(());
                }
                match class {
                    Some(_) => {
                        let (k, m) = key.split_once("::").expect("a method key");
                        builtin_method_throws(self.c, k, m)
                    }
                    None => self.c.natives.throws.functions.get(key).map(|t| resolve_str(t, None)),
                }
            }
        };
        // ConditionalTypeResolver::resolveForCall() needs the arguments' types
        if ty.as_ref().is_some_and(|t| has_conditional(t)) {
            return Err(());
        }
        Ok(ty)
    }

    fn callee_of(&self, m: Member<'a>) -> Callee<'a> {
        match m {
            Member::User { declaring, f, name, .. } => Callee::User {
                f,
                declaring: Some(declaring),
                name: String::from_utf8_lossy(name.as_bytes()).into_owned(),
            },
            Member::Builtin { declaring, f } => Callee::Builtin {
                key: format!("{}::{}", declaring.name.to_ascii_lowercase(), f.name.to_ascii_lowercase()),
                f,
                class: Some(declaring.name.clone()),
            },
        }
    }

    /// The callee's return type (`getReturnType()` of its variant).
    fn return_type(&self, callee: &Callee) -> Type {
        self.with_classes(|| match callee {
            Callee::User { f, declaring, name } => declared::codex_return_type(self.c, f, *declaring, name),
            Callee::Builtin { f, class, .. } => {
                resolve_str(f.variants.first().map_or("mixed", |v| v.ret.as_str()), class.as_deref())
            }
        })
    }

    /// FuncCallHandler::getFunctionThrowPoint() and MethodThrowPointHelper::getThrowPoint() (`function`: the zero
    /// arguments exemption of a built-in function).
    fn call_point(&mut self, at: Span, callee: &Callee, function: bool, args: usize) {
        let ty = match self.throw_type(callee) {
            Ok(ty) => ty,
            Err(()) => {
                self.points.push(Point::unknown(at));
                return;
            }
        };
        let ret = self.return_type(callee);
        let ty = ty.or_else(|| {
            matches!(ret, Type::Never(NeverKind::Explicit | NeverKind::NonAccepting)).then(throwable)
        });
        match ty {
            Some(ty) => {
                if !ty.is_void().yes() {
                    self.points.push(Point::explicit(at, ty, true));
                }
            }
            None if self.implicit_throws() => {
                if function
                    && let Callee::Builtin { f, .. } = callee
                    && args == 0
                    && f.variants.first().is_some_and(|v| v.params.iter().all(|p| p.optional))
                {
                    return;
                }
                let throwable_return = self.with_classes(|| is_super_type_of(&throwable(), &ret).yes());
                if self.in_throw == 0 || !throwable_return {
                    self.points.push(Point::implicit(at));
                }
            }
            None => {}
        }
    }

    /// The ported dynamic function throw type extensions (JsonThrowTypeExtension, AssertThrowTypeExtension): the
    /// call's throw point, `None` for the other functions.
    fn function_extension(&self, at: Span, key: &str, list: &ArgumentList) -> Option<Option<Point>> {
        let args: Vec<&Expression> = list.arguments.iter().map(|a| a.value()).collect();
        match key {
            "json_decode" | "json_encode" => {
                // the normalized call's argument at the `$flags` position
                let named = list.arguments.iter().find_map(|a| match a {
                    Argument::Named(n) if n.name.value == b"flags" => Some(n.value),
                    _ => None,
                });
                let position = if key == "json_encode" { 1 } else { 3 };
                let positional = list.arguments.iter().nth(position).filter(|a| matches!(a, Argument::Positional(_)));
                let Some(flags) = named.or(positional.map(|a| a.value())) else { return Some(None) };
                // BitwiseFlagHelper::bitwiseOrContainsConstant() is not no
                fn contains(c: &Checker, e: &Expression) -> Option<bool> {
                    match unparen(e) {
                        Expression::ConstantAccess(k) => {
                            // (a namespaced name falls back to the global constant)
                            let name = String::from_utf8_lossy(k.name.value()).into_owned();
                            Some(name.trim_start_matches('\\') == "JSON_THROW_ON_ERROR")
                        }
                        Expression::Binary(b) if matches!(b.operator, BinaryOperator::BitwiseOr(_)) => {
                            Some(contains(c, b.lhs)? || contains(c, b.rhs)?)
                        }
                        // exprContainsIntFlag()
                        e => {
                            use mago_codex::ttype::atomic::scalar::TScalar;
                            let t = c.type_of(e)?;
                            Some(t.types.iter().any(|a| match a {
                                TAtomic::Scalar(TScalar::Integer(i)) => {
                                    i.get_literal_value().is_none_or(|v| v & 4_194_304 != 0)
                                }
                                TAtomic::Mixed(_) => true,
                                _ => false,
                            }))
                        }
                    }
                }
                Some(match contains(self.c, flags) {
                    Some(true) => Some(Point::explicit(at, Type::Object("JsonException".into(), vec![], false), false)),
                    Some(false) => None,
                    None => Some(Point::unknown(at)),
                })
            }
            "assert" => {
                let custom = args.get(1).and_then(|e| self.type_of(e));
                let thrown = custom.filter(|t| self.with_classes(|| is_super_type_of(&throwable(), t).yes()));
                Some(match (args.len() < 2, thrown) {
                    (false, Some(t)) => Some(Point::explicit(at, t, false)),
                    (false, None) if self.type_of(args[1]).is_none() => Some(Point::unknown(at)),
                    // getThrowType() is null: the extension's null means no throw point
                    _ => None,
                })
            }
            _ => None,
        }
    }

    /// The ported Date* dynamic throw type extensions (DateTimeZone, DateInterval, DateTime and DateTimeImmutable
    /// constructors, `modify()`, `sub()` and `DateInterval::createFromDateString()`): the call's throw point, `None`
    /// for the other methods.
    // ponytail: a constant string argument needs PHP's date parser to tell whether it throws, so it stays unknown
    fn date_extension(&self, at: Span, key: &str, args: Option<&ArgumentList>) -> Option<Option<Point>> {
        use mago_codex::ttype::atomic::scalar::TScalar;
        let exceptions = self.c.php_version >= 80300;
        let (exception, needs_exceptions) = match key {
            "datetimezone::__construct" => ("DateInvalidTimeZoneException", false),
            "dateinterval::__construct" => ("DateMalformedIntervalStringException", false),
            "datetime::__construct" | "datetimeimmutable::__construct" => ("DateMalformedStringException", false),
            "datetime::modify" | "datetimeimmutable::modify" => ("DateMalformedStringException", true),
            // (the extension gives the reflected throw type)
            "dateinterval::createfromdatestring" => ("DateMalformedIntervalStringException", true),
            "datetime::sub" | "datetimeimmutable::sub" => ("DateInvalidOperationException", true),
            _ => return None,
        };
        let typed = |name: &str| Point::explicit(at, Type::Object(name.into(), vec![], false), false);
        let first = args.and_then(|l| l.arguments.first()).map(|a| a.value());
        let Some(first) = first else {
            // DateInterval's constructor gives its reflected throw type
            return Some((key == "dateinterval::__construct").then(|| typed(exception)));
        };
        if needs_exceptions && !exceptions {
            return Some(None);
        }
        let point = typed(if exceptions || key == "dateinterval::createfromdatestring" { exception } else { "Exception" });
        if key.ends_with("::sub") {
            return Some(Some(point));
        }
        let Some(t) = self.c.type_of(first) else { return Some(Some(Point::unknown(at))) };
        // what is left after removing the constant strings
        let constant = |a: &TAtomic| matches!(a, TAtomic::Scalar(TScalar::String(s)) if s.get_known_literal_value().is_some());
        Some(Some(if t.types.iter().all(constant) { Point::unknown(at) } else { point }))
    }

    /// ReflectionProvider::getFunction() of a named call.
    fn function(&self, callee: &Expression) -> Option<Callee<'a>> {
        match super::super::dead_code::function_of(self.c, callee)? {
            super::super::dead_code::Function::User(f) => Some(Callee::User {
                f,
                declaring: None,
                name: String::from_utf8_lossy(f.name.as_bytes()).into_owned(),
            }),
            super::super::dead_code::Function::Builtin(f) => Some(Callee::Builtin {
                key: f.name.to_ascii_lowercase(),
                f,
                class: None,
            }),
        }
    }

    /// The class-likes an expression's type holds, null aside: `None` when Pun cannot tell, an empty list when the
    /// type has no class (mixed, a scalar).
    fn classes_of(&self, e: &Expression) -> Option<Vec<&'a ClassLikeMetadata>> {
        let c = self.c;
        match unparen(e) {
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => return Some(vec![c.class_scope()?]),
            Expression::Self_(_) | Expression::Static(_) => return Some(vec![c.class_scope()?]),
            Expression::Parent(_) => {
                let parent = c.class_scope()?.direct_parent_class?;
                return Some(vec![c.codebase.get_class_like(parent.as_bytes())?]);
            }
            Expression::Identifier(id) => return Some(vec![c.class_like(&c.resolved(id)).found()?]),
            _ => {}
        }
        let t = c.type_of(e)?;
        let mut out = Vec::new();
        for a in t.types.iter() {
            match a {
                TAtomic::Null => {}
                TAtomic::Object(TObject::Named(n)) => out.push(c.codebase.get_class_like(n.name.as_bytes())?),
                TAtomic::Object(TObject::Enum(n)) => out.push(c.codebase.get_class_like(n.name.as_bytes())?),
                // `getObjectTypeOrClassStringObjectType()`
                TAtomic::Scalar(mago_codex::ttype::atomic::scalar::TScalar::ClassLikeString(k)) => {
                    match k.get_object_type(c.codebase) {
                        TAtomic::Object(TObject::Named(n)) => out.push(c.codebase.get_class_like(n.name.as_bytes())?),
                        TAtomic::GenericParameter(g) => match g.constraint.types.as_ref() {
                            [TAtomic::Object(TObject::Named(n))] => {
                                out.push(c.codebase.get_class_like(n.name.as_bytes())?)
                            }
                            _ => return None,
                        },
                        _ => return None,
                    }
                }
                TAtomic::Scalar(_) | TAtomic::Array(_) | TAtomic::Void | TAtomic::Never => {}
                _ => return None,
            }
        }
        Some(out)
    }

    /// The throw points of a call to method `name` on what `on` holds (`getMethodReflection()` and
    /// MethodThrowPointHelper::getThrowPoint()): implicit without a method.
    fn member_call(
        &mut self,
        at: Span,
        on: &Expression,
        method: &ClassLikeMemberSelector,
        args: &ArgumentList,
        is_static: bool,
    ) {
        let ClassLikeMemberSelector::Identifier(name) = method else {
            // ponytail: a dynamic method name
            self.points.push(Point::unknown(at));
            return;
        };
        let Some(classes) = self.classes_of(on) else {
            self.points.push(Point::unknown(at));
            return;
        };
        self.method_on(at, &classes, name.value, is_static, Some(args));
    }

    fn method_on(
        &mut self,
        at: Span,
        classes: &[&'a ClassLikeMetadata],
        name: &[u8],
        is_static: bool,
        args: Option<&ArgumentList>,
    ) {
        // Larastan's Macro::getThrowType(): `validate()` and `validateWithBag()` throw a ValidationException,
        // other macros have no throw type
        if let [class] = classes
            && super::super::larastan::macro_method(self.c, class, name).is_some()
        {
            if name.eq_ignore_ascii_case(b"validate") || name.eq_ignore_ascii_case(b"validateWithBag") {
                let thrown = Type::Object("Illuminate\\Validation\\ValidationException".into(), vec![], false);
                self.points.push(Point::explicit(at, thrown, true));
            } else {
                self.implicit(at);
            }
            return;
        }
        let found: Vec<Member<'a>> = classes
            .iter()
            .filter_map(|k| members::method(self.c, k, name).or_else(|| members::annotation_method(self.c, k, name)))
            .collect();
        // a union keeps its members that have the method
        if found.is_empty() {
            self.points.push(Point::implicit(at));
            return;
        }
        if found.len() > 1 {
            // UnionTypeMethodReflection: the union of the throw types
            let mut types = Vec::new();
            for m in found {
                match self.throw_type(&self.callee_of(m)) {
                    Ok(Some(t)) => types.push(t),
                    Ok(None) => {}
                    Err(()) => {
                        self.points.push(Point::unknown(at));
                        return;
                    }
                }
            }
            if types.is_empty() {
                self.implicit(at);
            } else {
                let t = self.with_classes(|| union(types));
                if !t.is_void().yes() {
                    self.points.push(Point::explicit(at, t, true));
                }
            }
            return;
        }
        let m = found.into_iter().next().expect("one method");
        if let Member::Builtin { declaring, f } = &m
            && matches!(declaring.name.as_str(), "ReflectionMethod" | "ReflectionFunction")
            && matches!(f.name.to_ascii_lowercase().as_str(), "invoke" | "invokeargs")
            && !is_static
        {
            self.points.push(Point::implicit(at));
            return;
        }
        let callee = self.callee_of(m);
        if let Some(args) = args {
            let params = self.immediate_params(&callee);
            self.callbacks(args, &params);
        }
        if let Callee::Builtin { key, .. } = &callee
            && let Some(point) = self.date_extension(at, key, args)
        {
            self.points.extend(point);
            return;
        }
        self.call_point(at, &callee, false, 0);
    }

    /// `$scope->getMethodReflection($type, $name)`'s class-likes: the members of a union that have the method (none
    /// when the type may lack it); `None` when Pun cannot tell.
    fn method_holders(&self, t: &Type, name: &str) -> Option<Vec<&'a ClassLikeMetadata>> {
        let members = self.with_classes(|| match t {
            Type::Union(ts) => ts.iter().filter(|m| m.has_method(name).yes()).cloned().collect(),
            t if t.has_method(name).yes() => vec![t.clone()],
            _ => Vec::new(),
        });
        let mut out = Vec::new();
        for m in members {
            let names = m.object_class_names();
            if names.len() != 1 {
                return None;
            }
            out.push(self.c.codebase.get_class_like(names[0].as_bytes())?);
        }
        Some(out)
    }

    /// MethodThrowPointHelper::getThrowPointsForCallOnType() of an ArrayAccess method on what `on` holds, when its
    /// type is maybe an ArrayAccess and not an array.
    fn array_access(&mut self, at: Span, on: &Expression, method: &str) {
        let Some(t) = self.type_of(on) else {
            self.points.push(Point::unknown(at));
            return;
        };
        let array_access = Type::Object("ArrayAccess".into(), vec![], false);
        if self.with_classes(|| t.is_array().yes() || is_super_type_of(&array_access, &t).no()) {
            return;
        }
        match self.method_holders(&t, method) {
            None => self.points.push(Point::unknown(at)),
            Some(classes) if classes.is_empty() => self.points.push(Point::implicit(at)),
            Some(classes) => self.method_on(at, &classes, method.as_bytes(), false, None),
        }
    }

    /// ImplicitToStringCallHelper: an object's `__toString()` call.
    fn to_string(&mut self, e: &Expression) {
        let Some(t) = self.type_of(e) else {
            self.points.push(Point::unknown(e.span()));
            return;
        };
        if self.with_classes(|| t.is_object().no()) {
            return;
        }
        match self.method_holders(&t, "__toString") {
            None => self.points.push(Point::unknown(e.span())),
            Some(classes) if classes.is_empty() => {}
            Some(classes) => self.method_on(e.span(), &classes, b"__toString", false, None),
        }
    }

    /// PropertyHookThrowPointsResolver::getThrowPointsFromPropertyHook() for an instance property access.
    fn property_hook(&mut self, at: Span, object: &Expression, property: &ClassLikeMemberSelector, hook: &str) {
        if self.c.php_version < 80400 {
            return;
        }
        let ClassLikeMemberSelector::Identifier(name) = property else {
            // a dynamic name
            self.points.push(Point::implicit(at));
            return;
        };
        if self.hooked.as_deref() == Some(name.value)
            && matches!(unparen(object), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
        {
            return;
        }
        let Some(classes) = self.classes_of(object) else {
            self.points.push(Point::unknown(at));
            return;
        };
        let [class] = classes.as_slice() else {
            if !classes.is_empty() {
                self.points.push(Point::unknown(at));
            }
            return;
        };
        let variable = [b"$", name.value].concat();
        let (Some(declaring), Some(p)) = (
            self.c.codebase.get_declaring_property_class(class.name.as_bytes(), &variable),
            self.c.codebase.get_declaring_property(class.name.as_bytes(), &variable),
        ) else {
            return;
        };
        let Some(declaring) = self.c.codebase.get_class_like(declaring.as_bytes()) else { return };
        match p.hooks.get(&mago_word::word(hook.as_bytes())) {
            None => {
                let private = p.read_visibility == Visibility::Private;
                if private || p.flags.is_final() || declaring.flags.is_final() {
                    return;
                }
                self.implicit(at);
            }
            Some(h) => {
                let ty = declared::doc_at(self.c, h.span).and_then(|(_, entry, _)| {
                    let scope = declared::scope_at(self.c, h.span, Some(declaring));
                    self.with_classes(|| invalid_throws::throws_type(&entry.phpdoc, &scope))
                });
                match ty {
                    Some(t) if has_conditional(&t) => self.points.push(Point::unknown(at)),
                    Some(t) => {
                        if !t.is_void().yes() {
                            self.points.push(Point::explicit(at, t, true));
                        }
                    }
                    None => self.implicit(at),
                }
            }
        }
    }

    /// AssignHandler's property target: a TypeError when the native type may not take the value, the set hook's
    /// points, or `__set()`'s for a property the class lacks.
    fn property_assignment(&mut self, a: &Assignment, object: &Expression, property: &ClassLikeMemberSelector) {
        let at = a.lhs.span();
        let ClassLikeMemberSelector::Identifier(name) = property else {
            if self.c.php_version >= 80400 {
                self.points.push(Point::implicit(at));
            }
            return;
        };
        let Some(classes) = self.classes_of(object) else {
            self.points.push(Point::unknown(at));
            return;
        };
        let [class] = classes.as_slice() else {
            if !classes.is_empty() {
                self.points.push(Point::unknown(at));
            }
            return;
        };
        let variable = [b"$", name.value].concat();
        let Some(p) = self.c.codebase.get_declaring_property(class.name.as_bytes(), &variable) else {
            if members::method(self.c, class, b"__set").is_some() {
                self.method_on(at, &[class], b"__set", false, None);
            }
            return;
        };
        if let Some(native) = &p.type_declaration_metadata {
            let assigned = match a.operator {
                AssignmentOperator::Assign(_) => self.type_of(a.rhs),
                // ponytail: a compound assignment's result type
                _ => None,
            };
            let compatible = assigned.map(|assigned| {
                self.with_classes(|| {
                    let native = declared::from_tunion(self.c, &native.type_union, class);
                    is_super_type_of(&native, &assigned).yes()
                        || !matches!(assigned, Type::Mixed { .. })
                            && flatten(&assigned).iter().all(|t| accepts(&native, t, true).yes())
                })
            });
            match compatible {
                Some(true) => {}
                Some(false) => self.points.push(Point::explicit(
                    a.rhs.span(),
                    Type::Object("TypeError".into(), vec![], false),
                    false,
                )),
                None => self.points.push(Point::unknown(a.rhs.span())),
            }
        }
        self.property_hook(at, object, property, "set");
    }

    /// The throw points of a closure or arrow function's body.
    fn closure_points(&self, e: &Expression) -> Option<Vec<Point>> {
        let mut inner = self.nested();
        match unparen(e) {
            Expression::Closure(f) => inner.walk_block(&f.body, &mut ()),
            Expression::ArrowFunction(f) => inner.walk_expression(f.expression, &mut ()),
            _ => return None,
        }
        Some(inner.points)
    }

    /// ArgumentsHandler: the closures passed where the callee calls them immediately.
    fn callbacks(&mut self, args: &ArgumentList, params: &[(String, bool)]) {
        for (i, a) in args.arguments.iter().enumerate() {
            let param = match a {
                Argument::Named(n) => params.iter().find(|(name, _)| name.as_bytes() == n.name.value),
                Argument::Positional(_) => params.get(i).or_else(|| params.last()),
            };
            if !param.is_some_and(|(_, immediate)| *immediate) {
                continue;
            }
            let value = a.value();
            let at = value.span();
            match self.closure_points(value) {
                Some(points) => {
                    for p in points {
                        self.points.push(if p.explicit {
                            Point { at, ..p }
                        } else {
                            Point { at, ..Point::implicit(at) }
                        });
                    }
                }
                // ponytail: a callable that is not a closure (a string, an array, a first-class callable)
                None => {
                    if !matches!(unparen(value), Expression::Literal(Literal::Null(_))) {
                        self.points.push(Point::unknown(at));
                    }
                }
            }
        }
    }

    /// The parameters of a callee and whether ArgumentsHandler::callCallbackImmediately() holds for each.
    fn immediate_params(&self, callee: &Callee) -> Vec<(String, bool)> {
        match callee {
            Callee::User { f, declaring, name } => {
                let raw = declared::doc_at(self.c, f.span).map(|(raw, _, _)| raw).unwrap_or_default();
                let tagged = |tags: &[&str], p: &str| {
                    String::from_utf8_lossy(&raw).lines().any(|l| {
                        let mut w = l.trim_start().trim_start_matches('*').split_whitespace();
                        w.next().is_some_and(|t| tags.contains(&t)) && w.next() == Some(&format!("${p}"))
                    })
                };
                let types = self.with_classes(|| declared::reflected_params(self.c, f, *declaring, name));
                f.parameters
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        let n = String::from_utf8_lossy(&p.name.0.as_bytes()[1..]).into_owned();
                        let immediate = if tagged(&["@param-immediately-invoked-callable", "@phpstan-param-immediately-invoked-callable"], &n) {
                            true
                        } else if tagged(&["@param-later-invoked-callable", "@phpstan-param-later-invoked-callable"], &n) {
                            false
                        } else {
                            // (a method's parameters are only called immediately when tagged)
                            declaring.is_none() && types.get(i).is_some_and(|t| self.with_classes(|| finds_callable(t)))
                        };
                        (n, immediate)
                    })
                    .collect()
            }
            Callee::Builtin { key, f, class } => {
                let Some(v) = f.variants.first() else { return Vec::new() };
                v.params
                    .iter()
                    .map(|p| {
                        let immediate = if class.is_some() {
                            self.c.natives.throws.immediate.get(key).is_some_and(|l| l.contains(&p.name))
                        } else if self.c.natives.throws.later.get(key).is_some_and(|l| l.contains(&p.name)) {
                            false
                        } else {
                            self.with_classes(|| finds_callable(&resolve_str(&p.ty, None)))
                        };
                        (p.name.clone(), immediate)
                    })
                    .collect()
            }
        }
    }

    /// StatementsHandler::getOverridingThrowPoints(): a statement's own `@throws` PHPDoc replaces its points.
    fn overriding(&self, s: &Statement) -> Option<Vec<Point>> {
        if matches!(
            s,
            Statement::Static(_)
                | Statement::Global(_)
                | Statement::Constant(_)
                | Statement::Class(_)
                | Statement::Interface(_)
                | Statement::Trait(_)
                | Statement::Enum(_)
                | Statement::Function(_)
        ) {
            return None;
        }
        let (_, entry) = self.c.doc_before(self.c.file.id, s.span().start.offset)?;
        let scope = var_tags::scope(self.c, s.span());
        let ty = self.with_classes(|| invalid_throws::throws_type(&entry.phpdoc, &scope))?;
        if ty.is_void().yes() {
            return Some(Vec::new());
        }
        Some(vec![Point::explicit(s.span(), ty, false)])
    }

    /// TryCatchHandler: the try block's points the catches leave, and the catches' and finally's own.
    pub(in super::super) fn try_statement(&mut self, t: &Try) {
        let mut inner = self.nested();
        inner.hooked = self.hooked.clone();
        inner.walk_block(&t.block, &mut ());
        self.dead.append(&mut inner.dead);
        let block = inner.points;
        let class = self.c.class_scope();
        let mut points = block.clone();
        let mut later = Vec::new();
        let mut past = Type::Never(NeverKind::Implicit);
        let throwable = throwable();
        for k in t.catch_clauses.iter() {
            let (originals, catches, original) = self.with_classes(|| {
                let originals: Vec<Type> =
                    catch_hints(&k.hint).into_iter().map(|h| declared::native(self.c, h, class)).collect();
                let catches: Vec<Type> = originals.iter().map(|o| remove_object(o, &past)).collect();
                let original = union(originals.clone());
                (originals, catches, original)
            });
            past = self.with_classes(|| union(vec![past.clone(), original.clone()]));
            let mut matching: BTreeMap<usize, Point> = BTreeMap::new();
            let mut matched = crate::bitset::BitSet::new(originals.len());
            let mut extra = Vec::new();
            self.with_classes(|| {
                // throwable matches all
                for (i, o) in originals.iter().enumerate() {
                    if is_super_type_of(o, &throwable).yes() {
                        for (pi, p) in points.iter().enumerate() {
                            matching.insert(pi, p.clone());
                            matched.set(i, true);
                        }
                    }
                }
                // explicit only
                let mut only_throw = true;
                if matching.is_empty() {
                    for (pi, p) in points.iter().enumerate() {
                        if p.ty.is_never() {
                            continue;
                        }
                        for (i, k) in catches.iter().enumerate() {
                            if is_super_type_of(k, &p.ty).no() {
                                continue;
                            }
                            matched.set(i, true);
                            if !p.explicit {
                                continue;
                            }
                            if !p.throw {
                                only_throw = false;
                            }
                            matching.insert(pi, p.clone());
                        }
                    }
                }
                // implicit only
                let broad = |name: &str| is_super_type_of(&original, &Type::Object(name.into(), vec![], false)).yes();
                if matching.is_empty() || only_throw || broad("Exception") || broad("Error") {
                    for (pi, p) in points.iter().enumerate() {
                        if p.explicit || p.ty.is_never() {
                            continue;
                        }
                        if catches.iter().any(|k| !is_super_type_of(k, &p.ty).no()) {
                            matching.insert(pi, p.clone());
                        }
                    }
                }
                // include previously removed throw points
                if matching.is_empty() && is_super_type_of(&original, &throwable).yes() {
                    for p in block.iter().filter(|p| p.any) {
                        extra.push(p.clone());
                        (0..matched.len()).for_each(|m| matched.set(m, true));
                    }
                }
            });
            for (i, m) in matched.iter().enumerate() {
                if !m && !catches[i].is_never() {
                    self.dead.push((k.r#catch.span(), catches[i].clone()));
                }
            }
            if matching.is_empty() && extra.is_empty() {
                continue;
            }
            // recompute throw points
            points = self.with_classes(|| {
                points
                    .iter()
                    .filter_map(|p| {
                        let ty = remove_object(&p.ty, &original);
                        if ty.is_never() {
                            if !p.any || is_super_type_of(&original, &throwable).yes() {
                                return None;
                            }
                            return Some(Point { ty, explicit: false, any: true, throw: false, ..p.clone() });
                        }
                        Some(Point { ty, ..p.clone() })
                    })
                    .collect()
            });
            let mut body = self.nested();
            body.hooked = self.hooked.clone();
            body.walk_block(&k.block, &mut ());
            self.dead.append(&mut body.dead);
            later.extend(body.points);
        }
        if let Some(f) = &t.finally_clause {
            let mut body = self.nested();
            body.hooked = self.hooked.clone();
            body.walk_block(&f.block, &mut ());
            self.dead.append(&mut body.dead);
            later.extend(body.points);
        }
        self.points.extend(points);
        self.points.extend(later);
    }
}

/// A catch's types, in their written order.
fn catch_hints<'h, 'arena>(h: &'h Hint<'arena>) -> Vec<&'h Hint<'arena>> {
    match h {
        Hint::Union(u) => {
            let mut out = catch_hints(u.left);
            out.extend(catch_hints(u.right));
            out
        }
        Hint::Parenthesized(p) => catch_hints(p.hint),
        h => vec![h],
    }
}

/// `TypeUtils::flattenTypes()`.
pub(in super::super) fn flatten(t: &Type) -> Vec<Type> {
    match t {
        Type::Union(ts) => ts.clone(),
        t => vec![t.clone()],
    }
}

fn has_conditional(t: &Type) -> bool {
    match t {
        Type::Conditional(_) => true,
        Type::Union(ts) => ts.iter().any(has_conditional),
        _ => false,
    }
}

/// `TypeUtils::findCallableType() !== null`.
fn finds_callable(t: &Type) -> bool {
    if t.is_callable().yes() {
        return true;
    }
    match t {
        Type::Union(ts) => ts.iter().any(finds_callable),
        _ => false,
    }
}

/// `$type->toNumber()->isSuperTypeOf(new ConstantIntegerType(0))` is not no: the divisor may be zero.
fn may_be_zero(c: &Checker, e: &Expression) -> Option<bool> {
    let t = c.type_of(e)?;
    Some(t.types.iter().any(|a| match a {
        TAtomic::Scalar(s) => {
            use mago_codex::ttype::atomic::scalar::TScalar;
            match s {
                TScalar::Integer(i) => {
                    let (min, max) = (i.get_minimum_value(), i.get_maximum_value());
                    min.is_none_or(|m| m <= 0) && max.is_none_or(|m| m >= 0)
                }
                TScalar::Float(_) => false,
                // (a string, a bool or a numeric's number may be 0)
                _ => true,
            }
        }
        // (an array's or an object's toNumber() is an error type, which is a super type of anything)
        _ => true,
    }))
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Points<'_, '_, 'arena> {
    fn walk_statement(&mut self, s: &'ast Statement<'arena>, context: &mut ()) {
        if self.c.unreachable.skipped.contains(&s.span()) {
            return;
        }
        let before = self.points.len();
        match s {
            Statement::Try(t) => self.try_statement(t),
            _ => mago_syntax::walker::walk_statement_mut(self, s, context),
        }
        if let Some(points) = self.overriding(s) {
            self.points.truncate(before);
            self.points.extend(points);
        }
    }

    fn walk_in_throw(&mut self, _: &'ast Throw<'arena>, _: &mut ()) {
        self.in_throw += 1;
    }
    fn walk_out_throw(&mut self, t: &'ast Throw<'arena>, _: &mut ()) {
        self.in_throw -= 1;
        match self.type_of(t.exception) {
            Some(ty) => self.points.push(Point { throw: true, ..Point::explicit(t.span(), ty, false) }),
            None => self.points.push(Point::unknown(t.span())),
        }
    }

    fn walk_out_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        let at = call.span();
        match unparen(call.function) {
            Expression::Identifier(_) => match self.function(call.function) {
                Some(callee) => {
                    let params = self.immediate_params(&callee);
                    self.callbacks(&call.argument_list, &params);
                    if let Callee::Builtin { key, .. } = &callee
                        && let Some(point) = self.function_extension(at, key, &call.argument_list)
                    {
                        self.points.extend(point);
                        return;
                    }
                    self.call_point(at, &callee, true, call.argument_list.arguments.len());
                }
                None => self.points.push(Point::implicit(at)),
            },
            // an immediately invoked closure: its body's points
            callee @ (Expression::Closure(_) | Expression::ArrowFunction(_)) => {
                for p in self.closure_points(callee).unwrap_or_default() {
                    if p.explicit {
                        self.points.push(Point { at, ..p });
                    } else if self.implicit_throws() {
                        self.points.push(Point::implicit(at));
                    }
                }
            }
            // ponytail: a callable value's throw points
            _ => self.points.push(Point::unknown(at)),
        }
    }

    fn walk_out_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        self.member_call(call.span(), call.object, &call.method, &call.argument_list, false);
    }
    fn walk_out_null_safe_method_call(&mut self, call: &'ast NullSafeMethodCall<'arena>, _: &mut ()) {
        self.member_call(call.span(), call.object, &call.method, &call.argument_list, false);
    }
    fn walk_out_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.member_call(call.span(), call.class, &call.method, &call.argument_list, true);
    }

    fn walk_out_instantiation(&mut self, n: &'ast Instantiation<'arena>, _: &mut ()) {
        let at = n.span();
        let args = n.argument_list.as_ref().map_or(0, |l| l.arguments.len());
        let mut dynamic = false;
        let class = match unparen(n.class) {
            Expression::Identifier(id) => self.c.class_like(&self.c.resolved(id)).found(),
            // the anonymous class's constructor statements
            Expression::AnonymousClass(_) => return,
            e => {
                dynamic = !matches!(e, Expression::Self_(_) | Expression::Parent(_));
                match self.classes_of(n.class) {
                    None => {
                        self.points.push(Point::unknown(at));
                        return;
                    }
                    Some(k) if k.len() == 1 => Some(k[0]),
                    Some(_) => {
                        self.points.push(Point::implicit(at));
                        return;
                    }
                }
            }
        };
        let Some(class) = class else {
            self.points.push(Point::implicit(at));
            return;
        };
        let Some(constructor) = members::method(self.c, class, b"__construct") else {
            if dynamic && !class.flags.is_final() {
                self.points.push(Point::implicit(at));
            }
            return;
        };
        let callee = self.callee_of(constructor);
        if let Some(l) = &n.argument_list {
            let params = self.immediate_params(&callee);
            self.callbacks(l, &params);
        }
        if let Callee::Builtin { key, .. } = &callee
            && let Some(point) = self.date_extension(at, key, n.argument_list.as_ref())
        {
            self.points.extend(point);
            return;
        }
        match self.throw_type(&callee) {
            Err(()) => self.points.push(Point::unknown(at)),
            Ok(Some(t)) => {
                if !t.is_void().yes() {
                    self.points.push(Point::explicit(at, t, true));
                }
            }
            Ok(None) => {
                let throwable_class = class.name.as_bytes().eq_ignore_ascii_case(b"Throwable")
                    || self.c.codebase.is_instance_of(class.name.as_bytes(), b"Throwable");
                if self.in_throw == 0 || !throwable_class {
                    self.implicit(at);
                }
            }
        }
        let _ = args;
    }

    fn walk_anonymous_class(&mut self, a: &'ast AnonymousClass<'arena>, context: &mut ()) {
        if let Some(l) = &a.argument_list {
            self.walk_partial_argument_list(l, context);
        }
        for m in a.members.iter() {
            if let ClassLikeMember::Method(m) = m
                && m.name.value.eq_ignore_ascii_case(b"__construct")
                && let MethodBody::Concrete(b) = &m.body
            {
                let mut inner = self.nested();
                inner.walk_block(b, &mut ());
                self.points.extend(inner.points);
            }
        }
    }

    fn walk_out_binary(&mut self, b: &'ast Binary<'arena>, _: &mut ()) {
        match b.operator {
            BinaryOperator::Division(_) | BinaryOperator::Modulo(_) => match may_be_zero(self.c, b.rhs) {
                Some(true) => self.points.push(Point::explicit(
                    b.span(),
                    Type::Object("DivisionByZeroError".into(), vec![], false),
                    false,
                )),
                Some(false) => {}
                None => self.points.push(Point::unknown(b.span())),
            },
            BinaryOperator::StringConcat(_) => {
                self.to_string(b.lhs);
                self.to_string(b.rhs);
            }
            _ => {}
        }
    }

    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        if let AssignmentOperator::Assign(_) = a.operator
            && matches!(unparen(a.lhs), Expression::Access(Access::Property(_)) | Expression::ArrayAccess(_))
        {
            self.writing.insert(unparen(a.lhs).span());
        }
    }
    fn walk_out_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        match a.operator {
            AssignmentOperator::Division(_) | AssignmentOperator::Modulo(_) => match may_be_zero(self.c, a.rhs) {
                Some(true) => self.points.push(Point::explicit(
                    a.span(),
                    Type::Object("DivisionByZeroError".into(), vec![], false),
                    false,
                )),
                Some(false) => {}
                None => self.points.push(Point::unknown(a.span())),
            },
            AssignmentOperator::Concat(_) => self.to_string(a.rhs),
            _ => {}
        }
        match unparen(a.lhs) {
            Expression::ArrayAccess(access) => self.array_access(a.lhs.span(), access.array, "offsetSet"),
            Expression::ArrayAppend(append) => self.array_access(a.lhs.span(), append.array, "offsetSet"),
            Expression::Access(Access::Property(p)) => self.property_assignment(a, p.object, &p.property),
            _ => {}
        }
    }

    fn walk_out_array_access(&mut self, a: &'ast ArrayAccess<'arena>, _: &mut ()) {
        if !self.writing.contains(&a.span()) {
            self.array_access(a.span(), a.array, "offsetGet");
        }
    }

    fn walk_in_construct(&mut self, k: &'ast Construct<'arena>, _: &mut ()) {
        if let Construct::Isset(i) = k {
            for v in i.values.iter() {
                if let Expression::ArrayAccess(a) = unparen(v) {
                    self.writing.insert(a.span());
                }
            }
        }
    }

    fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, _: &mut ()) {
        for v in u.values.iter() {
            if let Expression::ArrayAccess(a) = unparen(v) {
                self.writing.insert(a.span());
            }
        }
    }

    fn walk_out_match(&mut self, m: &'ast Match<'arena>, _: &mut ()) {
        if !m.arms.iter().any(|a| matches!(a, MatchArm::Default(_))) && !self.c.exhaustive(m) {
            self.points.push(Point::explicit(m.span(), Type::Object("UnhandledMatchError".into(), vec![], false), false));
        }
    }

    fn walk_out_yield(&mut self, y: &'ast Yield<'arena>, _: &mut ()) {
        self.points.push(Point::implicit(y.span()));
    }

    fn walk_out_construct(&mut self, k: &'ast Construct<'arena>, _: &mut ()) {
        match k {
            Construct::Eval(_)
            | Construct::Include(_)
            | Construct::IncludeOnce(_)
            | Construct::Require(_)
            | Construct::RequireOnce(_) => self.points.push(Point::implicit(k.span())),
            Construct::Print(p) => self.to_string(p.value),
            Construct::Isset(i) => {
                for v in i.values.iter() {
                    if let Expression::ArrayAccess(a) = unparen(v) {
                        self.array_access(v.span(), a.array, "offsetExists");
                    }
                }
            }
            _ => {}
        }
    }

    fn walk_out_unset(&mut self, u: &'ast Unset<'arena>, _: &mut ()) {
        for v in u.values.iter() {
            if let Expression::ArrayAccess(a) = unparen(v) {
                self.array_access(v.span(), a.array, "offsetUnset");
            }
        }
    }

    fn walk_out_composite_string(&mut self, s: &'ast CompositeString<'arena>, _: &mut ()) {
        let parts = match s {
            CompositeString::ShellExecute(s) => &s.parts,
            CompositeString::Interpolated(s) => &s.parts,
            CompositeString::Document(s) => &s.parts,
        };
        for e in parts.iter().filter_map(super::casts::part) {
            self.to_string(e);
        }
    }

    fn walk_out_echo(&mut self, e: &'ast Echo<'arena>, _: &mut ()) {
        for v in e.values.iter() {
            self.to_string(v);
        }
    }

    fn walk_out_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        if let UnaryPrefixOperator::StringCast(..) = u.operator {
            self.to_string(u.operand);
        }
    }

    fn walk_out_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        let at = f.expression.span();
        let Some(t) = self.c.type_of(f.expression) else {
            self.points.push(Point::unknown(at));
            return;
        };
        if t.types.iter().all(|a| matches!(a, TAtomic::Array(_) | TAtomic::Null | TAtomic::Scalar(_))) {
            return;
        }
        // ponytail: an IteratorAggregate's getIterator() `@throws`
        self.points.push(Point::unknown(at));
    }

    fn walk_out_property_access(&mut self, p: &'ast PropertyAccess<'arena>, _: &mut ()) {
        if !self.writing.contains(&p.span()) {
            self.property_hook(p.span(), p.object, &p.property, "get");
        }
    }
    fn walk_out_null_safe_property_access(&mut self, p: &'ast NullSafePropertyAccess<'arena>, _: &mut ()) {
        self.property_hook(p.span(), p.object, &p.property, "get");
    }

    fn walk_closure(&mut self, _: &'ast Closure<'arena>, _: &mut ()) {}
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {}
    fn walk_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {}
    fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
    fn walk_interface(&mut self, _: &'ast Interface<'arena>, _: &mut ()) {}
    fn walk_trait(&mut self, _: &'ast Trait<'arena>, _: &mut ()) {}
    fn walk_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {}
}

/// The expressions whose types the throw points read: receivers, thrown values, divisors, stringified values,
/// array accesses, iterated values and match subjects.
pub(in super::super) fn typed(program: &Program, wanted: bool) -> Vec<Span> {
    if !wanted {
        return Vec::new();
    }
    let mut t = Typed(Vec::new());
    t.walk_program(program, &mut ());
    t.0
}

struct Typed(Vec<Span>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Typed {
    fn walk_in_method_call(&mut self, c: &'ast MethodCall<'arena>, _: &mut ()) {
        self.0.push(c.object.span());
    }
    fn walk_in_null_safe_method_call(&mut self, c: &'ast NullSafeMethodCall<'arena>, _: &mut ()) {
        self.0.push(c.object.span());
    }
    fn walk_in_static_method_call(&mut self, c: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.0.push(c.class.span());
    }
    fn walk_in_property_access(&mut self, p: &'ast PropertyAccess<'arena>, _: &mut ()) {
        self.0.push(p.object.span());
    }
    fn walk_in_null_safe_property_access(&mut self, p: &'ast NullSafePropertyAccess<'arena>, _: &mut ()) {
        self.0.push(p.object.span());
    }
    fn walk_in_throw(&mut self, t: &'ast Throw<'arena>, _: &mut ()) {
        self.0.push(t.exception.span());
    }
    fn walk_in_function_call(&mut self, f: &'ast FunctionCall<'arena>, _: &mut ()) {
        self.0.extend(f.argument_list.arguments.iter().map(|a| a.value().span()));
    }
    fn walk_in_instantiation(&mut self, n: &'ast Instantiation<'arena>, _: &mut ()) {
        self.0.push(n.class.span());
    }
    fn walk_in_binary(&mut self, b: &'ast Binary<'arena>, _: &mut ()) {
        match b.operator {
            BinaryOperator::Division(_) | BinaryOperator::Modulo(_) => self.0.push(b.rhs.span()),
            BinaryOperator::StringConcat(_) => self.0.extend([b.lhs.span(), b.rhs.span()]),
            _ => {}
        }
    }
    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        self.0.push(a.rhs.span());
    }
    fn walk_in_array_access(&mut self, a: &'ast ArrayAccess<'arena>, _: &mut ()) {
        self.0.push(a.array.span());
    }
    fn walk_in_array_append(&mut self, a: &'ast ArrayAppend<'arena>, _: &mut ()) {
        self.0.push(a.array.span());
    }
    fn walk_in_composite_string(&mut self, s: &'ast CompositeString<'arena>, _: &mut ()) {
        let parts = match s {
            CompositeString::ShellExecute(s) => &s.parts,
            CompositeString::Interpolated(s) => &s.parts,
            CompositeString::Document(s) => &s.parts,
        };
        self.0.extend(parts.iter().filter_map(super::casts::part).map(|e| e.span()));
    }
    fn walk_in_echo(&mut self, e: &'ast Echo<'arena>, _: &mut ()) {
        self.0.extend(e.values.iter().map(|v| v.span()));
    }
    fn walk_in_print_construct(&mut self, p: &'ast PrintConstruct<'arena>, _: &mut ()) {
        self.0.push(p.value.span());
    }
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        if let UnaryPrefixOperator::StringCast(..) = u.operator {
            self.0.push(u.operand.span());
        }
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        self.0.push(f.expression.span());
    }
    fn walk_in_match(&mut self, m: &'ast Match<'arena>, _: &mut ()) {
        self.0.push(m.expression.span());
        for arm in m.arms.iter() {
            if let MatchArm::Expression(a) = arm {
                self.0.extend(a.conditions.iter().map(|c| c.span()));
            }
        }
    }
}
