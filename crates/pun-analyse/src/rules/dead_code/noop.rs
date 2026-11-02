//! NoopRule and the CallTo{Function,Method,StaticMethod,Constructor}StatementWithoutSideEffects rules: their
//! NoopExpressionNode is an expression statement without impure points or explicit throw points (not `++`/`--`).
//! ponytail: the impure points are a syntactic model of the expression handlers: calls are pure only when PHPStan's
//! reflection says so (a built-in's hasSideEffects(), `@pure`), a variable call or a pipe is impure, an implicit
//! `__toString()` or a property hook never is, and a closure argument is impure unless its body is one pure return.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::super::Checker;
use super::super::declaration_effects::TESTING_FUNCTIONS;
use super::super::declarations::{doc_before, doc_tags, has_tag};
use super::super::members::{self, Method};
use super::super::types::flow::unparen;
use pun_phpstan::symbols;

const SUPERGLOBALS: &[&[u8]] = &[
    b"$GLOBALS",
    b"$_SERVER",
    b"$_GET",
    b"$_POST",
    b"$_FILES",
    b"$_COOKIE",
    b"$_SESSION",
    b"$_REQUEST",
    b"$_ENV",
];

/// An expression's impure points and assignments.
#[derive(Default)]
struct Points {
    impure: bool,
    assign: bool,
    /// the keys of the calls whose impure points PossiblyPureCallTransitivePurityResolver resolves
    keys: Vec<String>,
}

impl Points {
    fn any(&self) -> bool {
        self.impure || !self.keys.is_empty()
    }
}

/// PHPStan's `pureUnlessCallableIsImpureParameters` (resources/functionMetadata.php).
const PURE_UNLESS_CALLABLE_IS_IMPURE: &[(&str, &[&str])] = &[
    ("array_all", &["callback"]),
    ("array_any", &["callback"]),
    ("array_diff_uassoc", &["key_compare_func"]),
    ("array_diff_ukey", &["key_comp_func"]),
    ("array_filter", &["callback"]),
    ("array_find", &["callback"]),
    ("array_find_key", &["callback"]),
    ("array_intersect_uassoc", &["key_compare_func"]),
    ("array_intersect_ukey", &["key_compare_func"]),
    ("array_map", &["callback"]),
    ("array_reduce", &["callback"]),
    ("array_udiff", &["data_comp_func"]),
    ("array_udiff_assoc", &["key_comp_func"]),
    ("array_udiff_uassoc", &["data_comp_func", "key_comp_func"]),
    ("array_uintersect", &["data_compare_func"]),
    ("array_uintersect_assoc", &["data_compare_func"]),
    (
        "array_uintersect_uassoc",
        &["data_compare_func", "key_compare_func"],
    ),
    ("call_user_func", &["function"]),
    ("call_user_func_array", &["function"]),
    ("forward_static_call", &["function"]),
    ("forward_static_call_array", &["function"]),
    ("preg_replace_callback", &["callback"]),
];

/// SimpleImpurePoint::SIDE_EFFECT_FLIP_PARAMETERS: no side effects when the `return` argument is true.
const FLIP: &[&str] = &["print_r", "var_export", "highlight_string"];

const PURE_TAGS: &[&str] = &[
    "@pure",
    "@phan-pure",
    "@phan-side-effect-free",
    "@psalm-pure",
    "@phpstan-pure",
];
const IMPURE_TAGS: &[&str] = &["@impure", "@phpstan-impure"];

/// `hasSideEffects()`.
#[derive(PartialEq, Clone, Copy)]
enum Effects {
    Yes,
    No,
    Maybe,
}

fn builtin_effects(s: &str) -> Effects {
    match s {
        "No" => Effects::No,
        "Yes" => Effects::Yes,
        _ => Effects::Maybe,
    }
}

/// A `@phpstan-assert` tag.
pub(super) fn asserts(f: &FunctionLikeMetadata) -> bool {
    !f.assertions.is_empty()
        || !f.assertions_inferred
            && !(f.if_true_assertions.is_empty() && f.if_false_assertions.is_empty())
}

/// A `@throws` tag other than `void`: the call's explicit throw point.
fn throws_explicitly(f: &FunctionLikeMetadata) -> bool {
    f.thrown_types
        .iter()
        .any(|t| t.from_docblock && !t.type_union.is_void())
}

/// Php{Function,Method}Reflection::hasSideEffects(): a void return, else its `@pure` or `@impure` tag.
fn user_effects(c: &Checker, f: &FunctionLikeMetadata) -> Effects {
    let constructor =
        f.method_metadata.is_some() && f.name.as_bytes().eq_ignore_ascii_case(b"__construct");
    if !constructor
        && f.return_type_metadata
            .as_ref()
            .is_some_and(|r| r.type_union.is_void())
    {
        return Effects::Yes;
    }
    // (a `$this` return, after the tags)
    let this = f.method_metadata.is_some()
        && f.return_type_metadata.as_ref().is_some_and(|r| {
            matches!(r.type_union.types.as_ref(), [TAtomic::Object(TObject::Named(n))] if n.is_this)
        });
    let tags = c
        .file_contents(f.span.file_id)
        .and_then(|contents| doc_before(&contents, f.span.start.offset as usize).map(doc_tags))
        .unwrap_or_default();
    if has_tag(&tags, PURE_TAGS) || f.flags.is_pure() {
        Effects::No
    } else if has_tag(&tags, IMPURE_TAGS) || this {
        Effects::Yes
    } else {
        Effects::Maybe
    }
}

fn method_effects(c: &Checker, m: &Method) -> Effects {
    match m {
        Method::User { f, .. } => user_effects(c, f),
        Method::Builtin { f, .. } => builtin_effects(&f.side_effects),
    }
}

/// The class a method or static call is on: `$this`, `self`, `static`, a name, or one named object type (`null`
/// aside, as a nullsafe call's).
fn class_of<'a>(c: &Checker<'a, '_>, e: &Expression) -> Option<&'a ClassLikeMetadata> {
    match unparen(e) {
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => c.class_scope(),
        Expression::Self_(_) | Expression::Static(_) => c.class_scope(),
        Expression::Parent(_) => c
            .class_scope()?
            .direct_parent_class
            .and_then(|p| c.codebase.get_class_like(p.as_bytes())),
        Expression::Identifier(id) => c.class_like(&c.resolved(id)).found(),
        e => {
            let t = c.type_of(e)?;
            let objects: Vec<_> = t.types.iter().filter(|a| !a.is_null()).collect();
            match objects.as_slice() {
                [TAtomic::Object(TObject::Named(n))] => {
                    c.codebase.get_class_like(n.name.as_bytes())
                }
                [TAtomic::Object(TObject::Enum(n))] => c.codebase.get_class_like(n.name.as_bytes()),
                _ => None,
            }
        }
    }
}

/// ReflectionProvider::getFunction() of a named call.
pub(in super::super) enum Function<'a> {
    User(&'a FunctionLikeMetadata),
    Builtin(&'static symbols::Function),
}

pub(in super::super) fn function_of<'a>(c: &Checker<'a, '_>, callee: &Expression) -> Option<Function<'a>> {
    let Expression::Identifier(id) = callee else {
        return None;
    };
    let fallback = match id {
        Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
        _ => None,
    };
    std::iter::once(c.names.get(&id.span()))
        .chain(fallback)
        .find_map(|name| {
            if let Some(f) = c
                .codebase
                .get_function(name)
                .filter(|f| !f.flags.is_built_in())
            {
                return Some(Function::User(f));
            }
            c.builtins
                .functions
                .get(&String::from_utf8_lossy(name).to_ascii_lowercase())
                .map(Function::Builtin)
        })
}

impl Function<'_> {
    fn name(&self) -> String {
        match self {
            Function::User(f) => String::from_utf8_lossy(f.original_name.as_bytes()).into_owned(),
            Function::Builtin(f) => f.name.clone(),
        }
    }

    fn asserts(&self) -> bool {
        matches!(self, Function::User(f) if asserts(f))
    }
}

impl Checker<'_, '_> {
    fn points(&self, e: &Expression, p: &mut Points) {
        let all = |c: &Checker, es: &mut dyn Iterator<Item = &Expression>, p: &mut Points| {
            for e in es {
                c.points(e, p);
            }
        };
        match e {
            Expression::Literal(_)
            | Expression::MagicConstant(_)
            | Expression::ConstantAccess(_)
            | Expression::Identifier(_)
            | Expression::Self_(_)
            | Expression::Static(_)
            | Expression::Parent(_)
            | Expression::Closure(_)
            | Expression::ArrowFunction(_) => {}
            Expression::Variable(Variable::Direct(v)) => p.impure |= SUPERGLOBALS.contains(&v.name),
            Expression::Variable(Variable::Indirect(v)) => self.points(v.expression, p),
            Expression::Variable(Variable::Nested(v)) => {
                self.points(&Expression::Variable(v.variable.clone()), p)
            }
            Expression::Parenthesized(e) => self.points(e.expression, p),
            Expression::Binary(b) => {
                self.points(b.lhs, p);
                self.points(b.rhs, p);
            }
            Expression::UnaryPrefix(u) => match u.operator {
                UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_) => {
                    self.assign_points(u.operand, p)
                }
                _ => self.points(u.operand, p),
            },
            Expression::UnaryPostfix(u) => self.assign_points(u.operand, p),
            Expression::Conditional(t) => {
                self.points(t.condition, p);
                if let Some(then) = t.then {
                    self.points(then, p);
                }
                self.points(t.r#else, p);
            }
            Expression::Assignment(a) => {
                match (&a.operator, unparen(a.rhs)) {
                    (AssignmentOperator::Assign(_), Expression::UnaryPrefix(u))
                        if u.operator.is_reference() =>
                    {
                        // a property assigned by reference
                        let mut target = unparen(u.operand);
                        while let Expression::ArrayAccess(ArrayAccess { array, .. }) = target {
                            target = unparen(array);
                        }
                        p.impure |= matches!(
                            target,
                            Expression::Access(Access::Property(_) | Access::StaticProperty(_))
                        );
                        self.points(u.operand, p);
                    }
                    _ => self.points(a.rhs, p),
                }
                self.assign_points(a.lhs, p);
            }
            Expression::Array(a) => all(self, &mut a.elements.iter().flat_map(element), p),
            Expression::LegacyArray(a) => all(self, &mut a.elements.iter().flat_map(element), p),
            Expression::ArrayAccess(a) => {
                self.points(a.array, p);
                self.points(a.index, p);
            }
            Expression::ArrayAppend(a) => self.points(a.array, p),
            Expression::CompositeString(s) => all(
                self,
                &mut s.parts().iter().filter_map(|part| match part {
                    StringPart::Expression(e) => Some(*e),
                    StringPart::BracedExpression(b) => Some(b.expression),
                    StringPart::Literal(_) => None,
                }),
                p,
            ),
            Expression::Access(Access::Property(a)) => {
                self.points(a.object, p);
                self.selector_points(&a.property, p);
            }
            Expression::Access(Access::NullSafeProperty(a)) => {
                self.points(a.object, p);
                self.selector_points(&a.property, p);
            }
            // a static property fetch on a name is impure; on an expression, the expression's points replace it
            Expression::Access(Access::StaticProperty(a)) => match unparen(a.class) {
                Expression::Identifier(_)
                | Expression::Self_(_)
                | Expression::Static(_)
                | Expression::Parent(_) => p.impure = true,
                class => self.points(class, p),
            },
            Expression::Access(Access::ClassConstant(a)) => {
                self.points(a.class, p);
                if let ClassLikeConstantSelector::Expression(e) = &a.constant {
                    self.points(e.expression, p);
                }
            }
            Expression::Construct(Construct::Isset(i)) => {
                all(self, &mut i.values.iter().copied(), p)
            }
            Expression::Construct(Construct::Empty(i)) => self.points(i.value, p),
            Expression::Clone(k) => self.points(k.object, p),
            Expression::Match(m) => {
                // a match that isn't exhaustive has an explicit UnhandledMatchError throw point
                p.impure |= !self.exhaustive(m);
                self.points(m.expression, p);
                for arm in m.arms.iter() {
                    if let MatchArm::Expression(a) = arm {
                        all(self, &mut a.conditions.iter().copied(), p);
                    }
                    self.points(arm.expression(), p);
                }
            }
            Expression::Call(call) => {
                let (pure, args) = match call {
                    Call::Function(f) => match unparen(f.function) {
                        Expression::Identifier(_) => (
                            self.function_pure(f.function, Some(&f.argument_list)),
                            &f.argument_list,
                        ),
                        // FuncCallHandler: the callable's impure points, none for a string naming no function
                        callee => {
                            self.points(callee, p);
                            let pure = match callee {
                                Expression::Literal(Literal::String(s)) => {
                                    s.value.and_then(|name| self.string_callable(name))
                                        != Some(false)
                                }
                                _ => false,
                            };
                            (pure, &f.argument_list)
                        }
                    },
                    Call::Method(m) => {
                        self.points(m.object, p);
                        (self.method_call_pure(e, m.object, &m.method, &m.argument_list), &m.argument_list)
                    }
                    Call::NullSafeMethod(m) => {
                        self.points(m.object, p);
                        (self.method_call_pure(e, m.object, &m.method, &m.argument_list), &m.argument_list)
                    }
                    Call::StaticMethod(m) => {
                        self.points(m.class, p);
                        (self.method_call_pure(e, m.class, &m.method, &m.argument_list), &m.argument_list)
                    }
                };
                if !pure {
                    self.call_point(e, p);
                }
                self.arguments(args, p);
            }
            // a first-class callable only creates a Closure
            Expression::PartialApplication(_) => {}
            Expression::Pipe(pipe) => {
                self.points(pipe.input, p);
                p.impure |= !match unparen(pipe.callable) {
                    Expression::Literal(Literal::String(s)) => {
                        s.value.and_then(|name| self.string_callable(name)) != Some(false)
                    }
                    callable => self.callable_pure(callable),
                };
            }
            Expression::AnonymousClass(a) => {
                let name = mago_codex::get_anonymous_class_name(self.file, a.span());
                // its own constructor runs here: ponytail, pure only when its body is empty
                let own = a.members.iter().find_map(|m| match m {
                    ClassLikeMember::Method(m)
                        if m.name.value.eq_ignore_ascii_case(b"__construct") =>
                    {
                        Some(m)
                    }
                    _ => None,
                });
                p.impure |= !match own {
                    Some(m) => {
                        matches!(&m.body, MethodBody::Concrete(b) if b.statements.is_empty())
                    }
                    None => self
                        .codebase
                        .get_class_like(name.as_bytes())
                        .is_some_and(|k| self.class_constructor_pure(k)),
                };
                for arg in a
                    .argument_list
                    .iter()
                    .flat_map(|args| args.arguments.iter())
                {
                    if let Some(value) = arg.value() {
                        self.points(value, p);
                    }
                }
            }
            Expression::Instantiation(new) => {
                if !self.constructor_pure(new) {
                    self.call_point(e, p);
                }
                if let Some(args) = &new.argument_list {
                    self.arguments(args, p);
                }
            }
            _ => p.impure = true,
        }
    }

    fn selector_points(&self, s: &ClassLikeMemberSelector, p: &mut Points) {
        if let ClassLikeMemberSelector::Expression(e) = s {
            self.points(e.expression, p);
        }
    }

    /// An argument list: a closure or arrow function argument runs its body (when the callee calls it).
    fn arguments(&self, args: &ArgumentList, p: &mut Points) {
        for arg in args.arguments.iter() {
            match unparen(arg.value()) {
                Expression::ArrowFunction(f) => self.points(f.expression, p),
                Expression::Closure(k) => match k.body.statements.as_slice() {
                    [] => {}
                    [Statement::Return(Return { value: Some(e), .. })] => self.points(e, p),
                    _ => p.impure = true,
                },
                e => self.points(e, p),
            }
        }
    }

    /// processAssignVar(): a write to a local variable (or its offsets) is no impure point.
    fn assign_points(&self, var: &Expression, p: &mut Points) {
        p.assign = true;
        let mut target = unparen(var);
        while let Expression::ArrayAccess(ArrayAccess { array, .. })
        | Expression::ArrayAppend(ArrayAppend { array, .. }) = target
        {
            target = unparen(array);
        }
        match target {
            Expression::Variable(Variable::Direct(v)) if !SUPERGLOBALS.contains(&v.name) => {}
            Expression::Array(a) => {
                for item in a.elements.iter().flat_map(element) {
                    self.assign_points(item, p);
                }
            }
            Expression::List(a) => {
                for item in a.elements.iter().flat_map(element) {
                    self.assign_points(item, p);
                }
            }
            _ => p.impure = true,
        }
        if !std::ptr::eq(target, unparen(var)) {
            self.points(var, p);
        }
    }

    /// MatchHandler's `$isExhaustive`: a `default` arm, or a subject of finite values the arms all match (its remaining
    /// type is never).
    pub(in super::super) fn exhaustive(&self, m: &Match) -> bool {
        let values = |e: &Expression| -> Option<Vec<Vec<u8>>> {
            let mut values = Vec::new();
            for atom in self.type_of(e)?.types.iter() {
                match atom {
                    TAtomic::Scalar(TScalar::Integer(i)) => {
                        values.push(format!("i{}", i.get_literal_value()?).into_bytes());
                    }
                    TAtomic::Scalar(TScalar::String(s)) => {
                        values.push([b"s".as_slice(), s.literal.as_ref()?.get_value()?].concat());
                    }
                    TAtomic::Scalar(scalar @ TScalar::ClassLikeString(_)) => {
                        let name = scalar.get_literal_class_string_value()?;
                        values.push([b"s".as_slice(), name.as_bytes()].concat());
                    }
                    TAtomic::Scalar(TScalar::Bool(b)) => match b.value {
                        Some(true) => values.push(b"t".to_vec()),
                        Some(false) => values.push(b"f".to_vec()),
                        None => values.extend([b"t".to_vec(), b"f".to_vec()]),
                    },
                    TAtomic::Null => values.push(b"n".to_vec()),
                    TAtomic::Object(TObject::Enum(e)) => {
                        let k = self.codebase.get_class_like(e.name.as_bytes())?;
                        values.extend(
                            k.enum_cases
                                .keys()
                                .filter(|c| e.case.is_none_or(|case| case == **c))
                                .map(|c| {
                                    [e.name.to_ascii_lowercase().as_bytes(), b"::", c.as_bytes()]
                                        .concat()
                                }),
                        );
                    }
                    _ => return None,
                }
            }
            Some(values)
        };
        let mut covered: Vec<Vec<u8>> = Vec::new();
        for arm in m.arms.iter() {
            let MatchArm::Expression(a) = arm else {
                return true;
            };
            // a condition of one value
            covered.extend(
                a.conditions
                    .iter()
                    .filter_map(|e| values(e))
                    .filter(|v| v.len() == 1)
                    .flatten(),
            );
        }
        values(m.expression).is_some_and(|v| v.iter().all(|v| covered.contains(v)))
    }

    pub(super) fn method_target(
        &self,
        on: &Expression,
        method: &ClassLikeMemberSelector,
    ) -> Option<Method<'_>> {
        let ClassLikeMemberSelector::Identifier(name) = method else {
            return None;
        };
        let class = class_of(self, on)?;
        members::method(self, class, name.value)
    }

    /// A method call without an impure point or an explicit throw point.
    fn method_pure(&self, on: &Expression, method: &ClassLikeMemberSelector) -> bool {
        self.method_target(on, method)
            .is_some_and(|m| self.method_without_effects(&m))
    }

    /// `method_pure()` of a call, which a Date* dynamic throw type extension's explicit throw point makes impure.
    fn method_call_pure(
        &self,
        call: &Expression,
        on: &Expression,
        method: &ClassLikeMemberSelector,
        args: &ArgumentList,
    ) -> bool {
        self.method_pure(on, method)
            && !matches!(self.method_target(on, method), Some(Method::Builtin { declaring, f })
                if super::super::types::throw_points::date_explicit(self, call.span(), &format!(
                    "{}::{}", declaring.name.to_ascii_lowercase(), f.name.to_ascii_lowercase()), args))
    }

    fn method_without_effects(&self, m: &Method) -> bool {
        method_effects(self, m) == Effects::No
            && match m {
                Method::User { f, .. } => !throws_explicitly(f),
                Method::Builtin { declaring, f } => !self
                    .codebase
                    .get_method(declaring.name.as_bytes(), f.name.as_bytes())
                    .is_some_and(throws_explicitly),
            }
    }

    /// A function call without an impure point (SimpleImpurePoint::createFromVariant()) or an explicit throw point.
    fn function_pure(&self, callee: &Expression, args: Option<&ArgumentList>) -> bool {
        let Some(function) = function_of(self, callee) else {
            return false;
        };
        match function {
            Function::User(f) => user_effects(self, f) == Effects::No && !throws_explicitly(f),
            Function::Builtin(f) => {
                if self
                    .codebase
                    .get_function(f.name.as_bytes())
                    .is_some_and(throws_explicitly)
                {
                    return false;
                }
                let effects = builtin_effects(&f.side_effects);
                if effects == Effects::No {
                    return true;
                }
                let Some(args) = args else {
                    return false;
                };
                let void = f.variants.first().is_some_and(|v| v.ret == "void");
                let lower = f.name.to_ascii_lowercase();
                if effects == Effects::Maybe
                    && !void
                    && let Some((_, params)) = PURE_UNLESS_CALLABLE_IS_IMPURE
                        .iter()
                        .find(|(n, _)| *n == lower)
                    && params.iter().all(|param| {
                        let Some(position) = f
                            .variants
                            .first()
                            .and_then(|v| v.params.iter().position(|p| p.name == *param))
                        else {
                            return false;
                        };
                        argument(args, param, Some(position)).is_none_or(|arg| {
                            matches!(unparen(arg), Expression::Literal(Literal::Null(_)))
                                || self.callable_pure(arg)
                        })
                    })
                {
                    return true;
                }
                FLIP.contains(&lower.as_str())
                    && argument(args, "return", Some(1)).is_some_and(|arg| {
                        matches!(unparen(arg), Expression::Literal(Literal::True(_)))
                            || self.type_of(arg).is_some_and(|t| t.is_true())
                    })
            }
        }
    }

    /// A function named by a string: whether it has no side effects, `None` when there is no such function.
    fn string_callable(&self, name: &[u8]) -> Option<bool> {
        let lower = String::from_utf8_lossy(name)
            .trim_start_matches('\\')
            .to_ascii_lowercase();
        match self
            .codebase
            .get_function(lower.as_bytes())
            .filter(|f| !f.flags.is_built_in())
        {
            Some(f) => Some(user_effects(self, f) == Effects::No),
            None => self
                .builtins
                .functions
                .get(&lower)
                .map(|f| f.side_effects == "No"),
        }
    }

    /// `$acceptor->isPure()->yes()` of a callable argument.
    fn callable_pure(&self, e: &Expression) -> bool {
        match unparen(e) {
            Expression::Literal(Literal::String(s)) => {
                s.value.and_then(|name| self.string_callable(name)) == Some(true)
            }
            Expression::PartialApplication(PartialApplication::Function(f)) => {
                self.function_pure(f.function, None)
            }
            Expression::PartialApplication(PartialApplication::Method(m)) => {
                self.method_pure(m.object, &m.method)
            }
            Expression::PartialApplication(PartialApplication::StaticMethod(m)) => {
                self.method_pure(m.class, &m.method)
            }
            Expression::ArrowFunction(f) => {
                let mut p = Points::default();
                self.points(f.expression, &mut p);
                !p.any()
            }
            Expression::Closure(k) => match k.body.statements.as_slice() {
                [] => true,
                [Statement::Return(Return { value: Some(e), .. })] => {
                    let mut p = Points::default();
                    self.points(e, &mut p);
                    !p.any()
                }
                _ => false,
            },
            _ => false,
        }
    }

    /// NewHandler: a known class without a constructor, or a constructor without side effects.
    fn constructor_pure(&self, new: &Instantiation) -> bool {
        // ponytail: a class only PHPStan's reflection knows (Lookup::Opaque) counts as impure
        matches!(unparen(new.class), Expression::Identifier(id)
            if self.class_like(&self.resolved(id)).found().is_some_and(|k| self.class_constructor_pure(k)))
    }

    fn class_constructor_pure(&self, k: &ClassLikeMetadata) -> bool {
        members::method(self, k, b"__construct").is_none_or(|m| self.method_without_effects(&m))
    }
}

/// isPure() and hasSideEffects() maybe, no `@phpstan-assert` and no `@throws` but void.
pub(super) fn possibly_pure(c: &Checker, f: &FunctionLikeMetadata) -> bool {
    user_effects(c, f) == Effects::Maybe && !asserts(f) && !throws_explicitly(f)
}

impl Checker<'_, '_> {
    /// A call's impure point: the keys of the callables it targets, else an impure point nothing resolves.
    fn call_point(&self, e: &Expression, p: &mut Points) {
        match self.call_keys(e) {
            Some(keys) => p.keys.extend(keys),
            None => p.impure = true,
        }
    }

    /// PossiblyPureCallTransitivePurityResolver::resolveCall(), none for a callee with an explicit throw point.
    fn call_keys(&self, e: &Expression) -> Option<Vec<String>> {
        let user = |f: &FunctionLikeMetadata| (!throws_explicitly(f)).then_some(());
        match e {
            Expression::Call(Call::Function(f)) => {
                let function = function_of(self, f.function)?;
                if let Function::User(f) = &function {
                    user(f)?;
                }
                Some(vec![super::impure_points::function_key(&function.name())])
            }
            Expression::Call(Call::Method(m)) => self.method_keys(m.object, &m.method, true),
            Expression::Call(Call::NullSafeMethod(m)) => self.method_keys(m.object, &m.method, true),
            Expression::Call(Call::StaticMethod(m))
                if matches!(unparen(m.class), Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) =>
            {
                self.method_keys(m.class, &m.method, false)
            }
            Expression::Instantiation(Instantiation { class: Expression::Identifier(id), .. }) => {
                let k = self.class_like(&self.resolved(id)).found()?;
                let m = members::method(self, k, b"__construct")?;
                if let Method::User { f, .. } = &m {
                    user(f)?;
                }
                Some(vec![super::impure_points::method_key(&m.declaring_name(), "__construct")])
            }
            _ => None,
        }
    }

    /// A method call's key: an instance method must not be overridable.
    fn method_keys(&self, on: &Expression, method: &ClassLikeMemberSelector, instance: bool) -> Option<Vec<String>> {
        if !instance {
            let m = self.method_target(on, method)?;
            if let Method::User { f, .. } = &m {
                (!throws_explicitly(f)).then_some(())?;
            }
            return Some(vec![super::impure_points::method_key(&m.declaring_name(), &m.name())]);
        }
        let methods = self.fixed_methods(on, method)?;
        methods.iter().all(|m| !matches!(m, Method::User { f, .. } if throws_explicitly(f))).then_some(())?;
        Some(methods.iter().map(|m| super::impure_points::method_key(&m.declaring_name(), &m.name())).collect())
    }

    /// The methods an instance call targets on each class of its receiver, when none can be overridden: private,
    /// final, of a final class, or on an object a `new` created.
    fn fixed_methods(&self, on: &Expression, method: &ClassLikeMemberSelector) -> Option<Vec<Method<'_>>> {
        let ClassLikeMemberSelector::Identifier(name) = method else { return None };
        let exact = self.conditions.exact.contains(&unparen(on).span());
        let classes = match unparen(on) {
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => vec![self.class_scope()?],
            e => {
                // in PHPStan's union order
                let names = self.phpstan_type(e)?.remove_null().object_class_names();
                names.iter().map(|n| self.codebase.get_class_like(n.as_bytes())).collect::<Option<Vec<_>>>()?
            }
        };
        let mut methods = Vec::new();
        for k in classes {
            let m = members::method(self, k, name.value)?;
            let fixed = m.visibility() == mago_codex::visibility::Visibility::Private
                || m.is_final()
                || matches!(&m, Method::User { declaring, .. } if declaring.flags.is_final())
                || k.flags.is_final()
                || exact;
            fixed.then_some(())?;
            methods.push(m);
        }
        (!methods.is_empty()).then_some(methods)
    }

    /// resolveDependencies() of a body: the keys its calls depend on, none when it has another impure point or an
    /// explicit throw point.
    pub(super) fn body_dependencies(&self, statements: &[Statement]) -> Option<Vec<String>> {
        struct Body<'c, 'a, 'b> {
            c: &'c Checker<'a, 'b>,
            p: Points,
        }
        impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Body<'_, '_, '_> {
            fn walk_expression(&mut self, e: &'ast Expression<'arena>, _: &mut ()) {
                self.c.points(e, &mut self.p);
            }
            fn walk_in_echo(&mut self, _: &'ast Echo<'arena>, _: &mut ()) {
                self.p.impure = true;
            }
            fn walk_in_echo_tag(&mut self, _: &'ast EchoTag<'arena>, _: &mut ()) {
                self.p.impure = true;
            }
            fn walk_in_global(&mut self, _: &'ast Global<'arena>, _: &mut ()) {
                self.p.impure = true;
            }
            fn walk_in_static(&mut self, _: &'ast Static<'arena>, _: &mut ()) {
                self.p.impure = true;
            }
            fn walk_in_inline(&mut self, _: &'ast Inline<'arena>, _: &mut ()) {
                self.p.impure = true;
            }
            fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, _: &mut ()) {
                // a property unset
                for value in u.values.iter() {
                    let mut target = unparen(value);
                    while let Expression::ArrayAccess(a) = target {
                        target = unparen(a.array);
                    }
                    self.p.impure |= matches!(target, Expression::Access(_));
                }
            }
            fn walk_function(&mut self, _: &'ast mago_syntax::cst::Function<'arena>, _: &mut ()) {}
            fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
        }
        let mut body = Body { c: self, p: Points::default() };
        for s in statements {
            body.walk_statement(s, &mut ());
        }
        (!body.p.impure).then_some(body.p.keys)
    }

    /// Possibly Pure{FuncCall,MethodCall,StaticCall,New}Collector: a call statement to a callable whose purity the
    /// declarations decide, with the keys of its targets.
    pub(super) fn possibly_pure_call(&self, e: &Expression) -> Option<(&'static str, Vec<String>)> {
        let possibly = |m: &Method| matches!(m, Method::User { f, .. } if user_effects(self, f) == Effects::Maybe);
        match e {
            Expression::Call(Call::Function(FunctionCall { function: callee, .. }))
            | Expression::PartialApplication(PartialApplication::Function(FunctionPartialApplication { function: callee, .. })) => match function_of(self, callee)? {
                Function::User(u) if user_effects(self, u) == Effects::Maybe => {
                    Some(("function.resultUnused", vec![super::impure_points::function_key(&Function::User(u).name())]))
                }
                _ => None,
            },
            Expression::Call(Call::Method(m)) => self.possibly_pure_method(m.object, &m.method),
            Expression::Call(Call::NullSafeMethod(m)) => self.possibly_pure_method(m.object, &m.method),
            Expression::PartialApplication(PartialApplication::Method(m)) => self.possibly_pure_method(m.object, &m.method),
            Expression::PartialApplication(PartialApplication::StaticMethod(StaticMethodPartialApplication { class, method, .. }))
            | Expression::Call(Call::StaticMethod(StaticMethodCall { class, method, .. }))
                if matches!(unparen(class), Expression::Identifier(_) | Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_)) =>
            {
                let target = self.method_target(class, method).filter(possibly)?;
                Some(("staticMethod.resultUnused", vec![super::impure_points::method_key(&target.declaring_name(), &target.name())]))
            }
            Expression::Instantiation(Instantiation { class: Expression::Identifier(id), .. }) => {
                let k = self.class_like(&self.resolved(id)).found()?;
                let m = members::method(self, k, b"__construct").filter(|m| matches!(m, Method::User { f, .. } if user_effects(self, f) == Effects::Maybe))?;
                Some(("new.resultUnused", vec![super::impure_points::method_key(&m.declaring_name(), "__construct")]))
            }
            _ => None,
        }
    }

    fn possibly_pure_method(&self, on: &Expression, method: &ClassLikeMemberSelector) -> Option<(&'static str, Vec<String>)> {
        let methods = self.fixed_methods(on, method)?;
        methods.iter().all(|m| matches!(m, Method::User { f, .. } if user_effects(self, f) == Effects::Maybe)).then_some(())?;
        Some(("method.resultUnused", methods.iter().map(|m| super::impure_points::method_key(&m.declaring_name(), &m.name())).collect()))
    }
}

/// The argument for parameter `name` at `position`: named, or positional before any named one.
fn argument<'e, 'arena>(
    args: &'e ArgumentList<'arena>,
    name: &str,
    position: Option<usize>,
) -> Option<&'e Expression<'arena>> {
    for (i, arg) in args.arguments.iter().enumerate() {
        match arg {
            Argument::Named(a) if a.name.value == name.as_bytes() => return Some(a.value),
            Argument::Named(_) => {}
            Argument::Positional(a) if Some(i) == position => return Some(a.value),
            Argument::Positional(_) => {}
        }
    }
    None
}

fn element<'e, 'arena>(e: &'e ArrayElement<'arena>) -> Vec<&'e Expression<'arena>> {
    match e {
        ArrayElement::KeyValue(kv) => vec![kv.key, kv.value],
        ArrayElement::Value(v) => vec![v.value],
        ArrayElement::Variadic(v) => vec![v.value],
        ArrayElement::Missing(_) => Vec::new(),
    }
}

/// The NoopExpressionNode rules on an expression statement.
pub(super) fn statement(c: &mut Checker, s: &ExpressionStatement) {
    if c.level < 4 {
        return;
    }
    let e = unparen(s.expression);
    if matches!(e, Expression::UnaryPostfix(_))
        || matches!(e, Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_)))
    {
        return;
    }
    let mut p = Points::default();
    c.points(e, &mut p);
    if p.any() {
        return;
    }
    noop(c, e, p.assign);
}

fn noop(c: &mut Checker, e: &Expression, assign: bool) {
    const TIP: &str = "This operator has unexpected precedence, try disambiguating the logic with parentheses ().";
    if let Expression::Binary(b) = e {
        let low = match b.operator {
            BinaryOperator::LowXor(_) => Some(("xor", "logicalXor")),
            BinaryOperator::LowAnd(_) => Some(("and", "logicalAnd")),
            BinaryOperator::LowOr(_) => Some(("or", "logicalOr")),
            _ => None,
        };
        if let Some((sigil, id)) = low {
            c.report(
                e,
                &format!("{id}.resultUnused"),
                format!("Unused result of \"{sigil}\" operator."),
                Some(TIP),
            );
            return;
        }
    }
    if assign {
        return;
    }
    match e {
        Expression::Binary(b)
            if matches!(b.operator, BinaryOperator::And(_) | BinaryOperator::Or(_)) =>
        {
            let (sigil, id) = if matches!(b.operator, BinaryOperator::And(_)) {
                ("&&", "booleanAnd")
            } else {
                ("||", "booleanOr")
            };
            c.report(
                e,
                &format!("{id}.resultUnused"),
                format!("Unused result of \"{sigil}\" operator."),
                None,
            );
        }
        Expression::Conditional(_) => {
            c.report(
                e,
                "ternary.resultUnused",
                "Unused result of ternary operator.".into(),
                None,
            );
        }
        Expression::Pipe(pipe) => {
            let callable = unparen(pipe.callable);
            side_effects(c, callable);
            noop_expression(c, callable);
        }
        e => {
            side_effects(c, e);
            noop_expression(c, e);
        }
    }
}

/// NoopRule's checks after the operators: every expression but a call, a named `new`, an assignment or a closure.
fn noop_expression(c: &mut Checker, e: &Expression) {
    match e {
        // NoopRule: a call is printed only when its callee is callable: here, a string naming a function
        Expression::Call(Call::Function(f)) if matches!(unparen(f.function), Expression::Literal(Literal::String(s)) if s.value.and_then(|n| c.string_callable(n)).is_some()) => {
            report_expression(c, e)
        }
        Expression::Call(_)
        | Expression::PartialApplication(_)
        | Expression::Assignment(_)
        | Expression::Closure(_) => {}
        Expression::Instantiation(new) if matches!(new.class, Expression::Identifier(_)) => {}
        e => report_expression(c, e),
    }
}

fn report_expression(c: &mut Checker, e: &Expression) {
    let printed = print(c, e);
    let printed = match printed.split_once(['\n', '\r']) {
        Some((first, _)) => format!("{first}…"),
        None => printed,
    };
    c.report(
        e,
        "expr.resultUnused",
        format!("Expression \"{printed}\" on a separate line does not do anything."),
        None,
    );
}

/// CallTo{Function,Method,StaticMethod,Constructor}StatementWithoutSideEffectsRule.
fn side_effects(c: &mut Checker, e: &Expression) {
    let message = match e {
        Expression::Call(Call::Function(FunctionCall {
            function: callee, ..
        }))
        | Expression::PartialApplication(PartialApplication::Function(
            FunctionPartialApplication {
                function: callee, ..
            },
        )) => match function_of(c, callee) {
            Some(f)
                if !f.asserts()
                    && !TESTING_FUNCTIONS.contains(&f.name().as_str())
                    && !c.type_of(e).is_some_and(|t| t.is_explicit_never()) =>
            {
                Some((
                    "function.resultUnused",
                    format!(
                        "Call to function {}() on a separate line has no effect.",
                        f.name()
                    ),
                ))
            }
            _ => None,
        },
        Expression::PartialApplication(PartialApplication::Method(m)) => {
            method_message(c, e, m.object, &m.method, "method.resultUnused")
        }
        Expression::PartialApplication(PartialApplication::StaticMethod(m)) => {
            method_message(c, e, m.class, &m.method, "staticMethod.resultUnused")
        }
        Expression::Call(Call::Method(m)) => {
            method_message(c, e, m.object, &m.method, "method.resultUnused")
        }
        Expression::Call(Call::NullSafeMethod(m)) => {
            method_message(c, e, m.object, &m.method, "method.resultUnused")
        }
        Expression::Call(Call::StaticMethod(m)) => {
            method_message(c, e, m.class, &m.method, "staticMethod.resultUnused").filter(|_| {
                !matches!(&m.method, ClassLikeMemberSelector::Identifier(n) if n.value.eq_ignore_ascii_case(b"__construct"))
            })
        }
        Expression::Instantiation(Instantiation { class: Expression::Identifier(id), .. }) => new_message(c, id).map(|m| ("new.resultUnused", m)),
        _ => None,
    };
    if let Some((id, message)) = message {
        c.report(e, id, message, None);
    }
}

/// CallToConstructorStatementWithoutSideEffectsRule: a pure `new` names a class Pun knows.
fn new_message(c: &Checker, id: &Identifier) -> Option<String> {
    let k = c.class_like(&c.resolved(id)).found()?;
    let display = c.display(k);
    match members::method(c, k, b"__construct") {
        Some(Method::User { f, .. }) if asserts(f) => None,
        Some(m) => Some(format!(
            "Call to {display}::{}() on a separate line has no effect.",
            m.name()
        )),
        None => Some(format!(
            "Call to new {display}() on a separate line has no effect."
        )),
    }
}

fn method_message(
    c: &Checker,
    call: &Expression,
    on: &Expression,
    method: &ClassLikeMemberSelector,
    id: &'static str,
) -> Option<(&'static str, String)> {
    let m = c.method_target(on, method)?;
    if matches!(&m, Method::User { f, .. } if asserts(f))
        || c.type_of(call).is_some_and(|t| t.is_explicit_never())
    {
        return None;
    }
    let kind = if m.is_static() {
        "static method"
    } else {
        "method"
    };
    Some((
        id,
        format!(
            "Call to {kind} {}::{}() on a separate line has no effect.",
            m.declaring_display(c),
            m.name()
        ),
    ))
}

/// The expressions [`statement()`] asks the type of: the statement's calls and the receivers of its method calls.
pub(super) fn typed(s: &ExpressionStatement) -> Vec<Span> {
    struct Wanted(Vec<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_method_call(&mut self, m: &'ast MethodCall<'arena>, _: &mut ()) {
            self.0.push(unparen(m.object).span());
        }
        fn walk_in_null_safe_method_call(
            &mut self,
            m: &'ast NullSafeMethodCall<'arena>,
            _: &mut (),
        ) {
            self.0.push(unparen(m.object).span());
        }
        fn walk_in_method_partial_application(
            &mut self,
            m: &'ast MethodPartialApplication<'arena>,
            _: &mut (),
        ) {
            self.0.push(unparen(m.object).span());
        }
        fn walk_in_static_method_partial_application(
            &mut self,
            m: &'ast StaticMethodPartialApplication<'arena>,
            _: &mut (),
        ) {
            self.0.push(unparen(m.class).span());
        }
        fn walk_in_static_method_call(&mut self, m: &'ast StaticMethodCall<'arena>, _: &mut ()) {
            self.0.push(unparen(m.class).span());
        }
        fn walk_in_match(&mut self, m: &'ast Match<'arena>, _: &mut ()) {
            self.0.push(m.expression.span());
            for arm in m.arms.iter() {
                if let MatchArm::Expression(a) = arm {
                    self.0.extend(a.conditions.iter().map(|c| c.span()));
                }
            }
        }
        fn walk_closure(&mut self, _: &'ast Closure<'arena>, _: &mut ()) {}
        fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {}
    }
    let mut w = Wanted(vec![s.expression.span(), unparen(s.expression).span()]);
    w.walk_expression(s.expression, &mut ());
    w.0
}

/// php-parser's Standard printer, for the expressions NoopRule prints; other nodes print as written.
pub(super) fn print(c: &Checker, e: &Expression) -> String {
    let written = || c.written(e.span());
    match e {
        Expression::Variable(Variable::Direct(v)) => String::from_utf8_lossy(v.name).into_owned(),
        Expression::Match(m) => format!("match ({}) {{\n", print(c, m.expression)),
        Expression::Variable(Variable::Nested(v)) => format!(
            "${{{}}}",
            print(c, &Expression::Variable(v.variable.clone()))
        ),
        Expression::Variable(Variable::Indirect(v)) => format!("${{{}}}", print(c, v.expression)),
        // pCallLhs(): a string callee is parenthesized
        Expression::Call(Call::Function(f)) => format!(
            "({})({})",
            print(c, f.function),
            f.argument_list
                .arguments
                .iter()
                .map(|a| print(c, a.value()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        // (a `new` binds tighter than any operator)
        Expression::Parenthesized(p) if matches!(p.expression, Expression::Instantiation(_)) => print(c, p.expression),
        Expression::Parenthesized(p) => format!("({})", print(c, p.expression)),
        Expression::Instantiation(n) if matches!(n.class, Expression::Identifier(_)) => format!(
            "new {}({})",
            print(c, n.class),
            n.argument_list
                .iter()
                .flat_map(|args| args.arguments.iter())
                .map(|a| print(c, a.value()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expression::ArrayAccess(a) => format!("{}[{}]", print(c, a.array), print(c, a.index)),
        Expression::Access(Access::Property(a)) => {
            format!("{}->{}", print(c, a.object), selector(c, &a.property))
        }
        Expression::Access(Access::NullSafeProperty(a)) => {
            format!("{}?->{}", print(c, a.object), selector(c, &a.property))
        }
        Expression::Access(Access::StaticProperty(a)) => {
            format!(
                "{}::{}",
                print(c, a.class),
                print(c, &Expression::Variable(a.property.clone()))
            )
        }
        Expression::Access(Access::ClassConstant(a)) => {
            let constant = match &a.constant {
                ClassLikeConstantSelector::Expression(e) => braced(c, e.expression),
                other => c.written(other.span()),
            };
            format!("{}::{constant}", print(c, a.class))
        }
        Expression::Identifier(id) => format!("\\{}", c.resolved(id)),
        // the operator as written; a cast in php-parser's canonical spelling
        Expression::UnaryPrefix(u) => {
            let op: String = c
                .written(u.operator.span())
                .to_ascii_lowercase()
                .split_whitespace()
                .collect();
            let op = match op.as_str() {
                "(integer)" => "(int) ".to_string(),
                "(boolean)" => "(bool) ".to_string(),
                "(binary)" => "(string) ".to_string(),
                cast if cast.starts_with('(') => format!("{cast} "),
                op => op.to_string(),
            };
            format!("{op}{}", print(c, u.operand))
        }
        Expression::Binary(b) => format!(
            "{} {} {}",
            print(c, b.lhs),
            String::from_utf8_lossy(b.operator.as_bytes()),
            print(c, b.rhs)
        ),
        Expression::Construct(Construct::Isset(i)) => {
            format!(
                "isset({})",
                i.values
                    .iter()
                    .map(|v| print(c, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        Expression::Construct(Construct::Empty(i)) => format!("empty({})", print(c, i.value)),
        Expression::CompositeString(CompositeString::Interpolated(s)) => {
            let mut out = String::from("\"");
            for part in s.parts.iter() {
                match part {
                    StringPart::Literal(l) => out.push_str(&String::from_utf8_lossy(l.raw)),
                    StringPart::Expression(e) => out.push_str(&format!("{{{}}}", print(c, e))),
                    StringPart::BracedExpression(b) => {
                        out.push_str(&format!("{{{}}}", print(c, b.expression)))
                    }
                }
            }
            out.push('"');
            out
        }
        Expression::Array(a) => format!(
            "[{}]",
            a.elements
                .iter()
                .map(|e| print_element(c, e))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        // pNewAnonymous() then the class body on the next line
        Expression::AnonymousClass(a) => {
            let mut out = String::from("new class");
            let args: Vec<String> = a
                .argument_list
                .iter()
                .flat_map(|args| args.arguments.iter())
                .map(|arg| c.written(arg.span()))
                .collect();
            if !args.is_empty() {
                out.push_str(&format!("({})", args.join(", ")));
            }
            if let Some(extends) = &a.extends {
                out.push_str(" extends ");
                out.push_str(
                    &extends
                        .types
                        .iter()
                        .map(|t| format!("\\{}", c.resolved(t)))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
            }
            out.push('\n');
            out
        }
        // pExpr_ArrowFunction(): inline attribute groups with resolved names, then `fn(...) => `
        Expression::ArrowFunction(f) => {
            let mut out = String::new();
            for list in f.attribute_lists.iter() {
                let attributes: Vec<String> = list
                    .attributes
                    .iter()
                    .map(|a| {
                        let args: Vec<String> = a
                            .argument_list
                            .iter()
                            .flat_map(|args| args.arguments.iter())
                            .map(|arg| c.written(arg.span()))
                            .collect();
                        let name = format!("\\{}", c.resolved(&a.name));
                        match a.argument_list {
                            Some(_) => format!("{name}({})", args.join(", ")),
                            None => name,
                        }
                    })
                    .collect();
                out.push_str(&format!("#[{}] ", attributes.join(", ")));
            }
            if f.r#static.is_some() {
                out.push_str("static ");
            }
            out.push_str("fn");
            if f.ampersand.is_some() {
                out.push('&');
            }
            let params: Vec<String> = f
                .parameter_list
                .parameters
                .iter()
                .map(|p| c.written(p.span()))
                .collect();
            out.push_str(&format!("({})", params.join(", ")));
            if let Some(hint) = &f.return_type_hint {
                out.push_str(&format!(": {}", c.written(hint.hint.span())));
            }
            out.push_str(&format!(" => {}", print(c, f.expression)));
            out
        }
        _ => written(),
    }
}

fn print_element(c: &Checker, e: &ArrayElement) -> String {
    match e {
        ArrayElement::KeyValue(kv) => format!("{} => {}", print(c, kv.key), print(c, kv.value)),
        ArrayElement::Value(v) => print(c, v.value),
        e => c.written(e.span()),
    }
}

fn selector(c: &Checker, s: &ClassLikeMemberSelector) -> String {
    match s {
        ClassLikeMemberSelector::Variable(v) => braced(c, &Expression::Variable(v.clone())),
        ClassLikeMemberSelector::Expression(e) => braced(c, e.expression),
        s => c.written(s.span()),
    }
}

/// A dynamic member name: PHPStan prints a constant string as the name, anything else in braces.
fn braced(c: &Checker, e: &Expression) -> String {
    match unparen(e) {
        Expression::Literal(Literal::String(LiteralString {
            value: Some(name), ..
        })) => String::from_utf8_lossy(name).into_owned(),
        e => format!("{{{}}}", print(c, e)),
    }
}
