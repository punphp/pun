//! The ImpurePoint records NodeScopeResolver collects for one function-like body (its statement result's points
//! and the property assignments the body's node gatherer adds), with whether the body has a throw point.
//! Each producer is a port of its handler's `new ImpurePoint(...)`: EchoHandler, InlineHtmlHandler, GlobalHandler,
//! StaticVariableHandler, UnsetHandler, VariableHandler, AssignHandler, StaticPropertyFetchHandler, PrintHandler,
//! ExitHandler, EvalHandler, IncludeHandler, YieldHandler, YieldFromHandler, FuncCallHandler, MethodCallHandler,
//! StaticCallHandler, NewHandler, ImplicitToStringCallHelper, ArgumentsHandler's immediately invoked callables and
//! ClosureTypeResolver's by-ref closure points. A construct without a ported producer makes the list incomplete.

use std::collections::{HashMap, HashSet};

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{Tri, Type};

use super::dead_code::{FLIP, Function as Callee, PURE_UNLESS_CALLABLE_IS_IMPURE, function_of};
use super::members::{self, Method};
use super::types::flow::unparen;
use super::{Checker, declared};

/// ImpurePoint: the line of `span`'s start, the identifier, the description and isCertain().
#[derive(Clone)]
pub(super) struct Point {
    pub span: Span,
    pub identifier: &'static str,
    pub description: String,
    pub certain: bool,
    /// a FuncCall's callee variable (without `$`): isPureUnlessCallableInvocation()
    pub invoked: Option<String>,
}

/// SimpleImpurePoint, a callable's point without a node.
type Simple = (&'static str, String, bool);

pub(super) struct Points {
    pub points: Vec<Point>,
    /// The statement result has a throw point (an overapproximation: never false when PHPStan has one).
    pub throws: bool,
    /// Every construct had a ported producer.
    pub complete: bool,
}

/// `pureUnlessParameterPassedParameters` of resources/functionMetadata.php.
const PURE_UNLESS_PARAMETER_PASSED: &[(&str, &[&str])] = &[
    ("similar_text", &["percent"]),
    ("str_ireplace", &["count", "replace_count"]),
    ("str_replace", &["count", "replace_count"]),
];

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

/// The impure points of a function or method body. `constructor` exempts the `$this->prop` assignments of a class's
/// own constructor (ClassMethodHandler's gatherer); `raw` is the declaration's PHPDoc (its `pure-callable` params).
pub(super) fn body(c: &Checker, statements: &[Statement], constructor: bool, raw: &[u8]) -> Points {
    let mut p = Producer::new(c, constructor);
    p.pure_params = pure_params(raw);
    p.statements(statements);
    p.finish()
}

/// The impure points of an arrow-function-bodied property hook (`get => expr`).
pub(super) fn expression(c: &Checker, e: &Expression, raw: &[u8]) -> Points {
    let mut p = Producer::new(c, false);
    p.pure_params = pure_params(raw);
    p.expr(e);
    p.finish()
}

/// The parameters whose `@param` type is a `pure-callable` or `pure-Closure` (without `$`).
fn pure_params(raw: &[u8]) -> HashSet<Vec<u8>> {
    let raw = String::from_utf8_lossy(raw);
    let mut out = HashSet::new();
    for line in raw.lines() {
        let line = line.trim_start().trim_start_matches('*').trim_start();
        let Some(rest) = ["@param ", "@phpstan-param ", "@psalm-param "]
            .iter()
            .find_map(|t| line.strip_prefix(t))
        else {
            continue;
        };
        let Some(dollar) = rest.find('$') else {
            continue;
        };
        let ty = rest[..dollar].trim();
        let name: String = rest[dollar + 1..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        let lower = ty.to_ascii_lowercase();
        if (lower.starts_with("pure-callable") || lower.starts_with("pure-closure"))
            && !ty.contains('|')
        {
            out.insert(name.into_bytes());
        }
    }
    out
}

fn purity(points: &[Simple]) -> Tri {
    if points.is_empty() {
        Tri::Yes
    } else if points.iter().any(|p| p.2) {
        Tri::No
    } else {
        Tri::Maybe
    }
}

fn and(a: Tri, b: Tri) -> Tri {
    match (a, b) {
        (Tri::No, _) | (_, Tri::No) => Tri::No,
        (Tri::Yes, Tri::Yes) => Tri::Yes,
        _ => Tri::Maybe,
    }
}

/// The conditional purity a callable value's acceptor carries: its parameters, the
/// `@pure-unless-callable-is-impure` ones and the `@pure-unless-parameter-passed` ones (whether the flag is certain).
type Cond = (Vec<Param>, Vec<String>, Vec<(String, bool)>);

/// A local bound to a known callable: its acceptor's impure points and conditional purity.
#[derive(Clone, PartialEq)]
struct Bound {
    points: Vec<Simple>,
    cond: Option<Cond>,
}

impl Bound {
    fn plain(points: Vec<Simple>) -> Self {
        Bound { points, cond: None }
    }
}

/// TrinaryLogic::lazyExtremeIdentity(): the common value, else maybe.
fn identity(values: impl Iterator<Item = Tri>) -> Tri {
    let mut out = None;
    for v in values {
        match out {
            None => out = Some(v),
            Some(o) if o != v => return Tri::Maybe,
            _ => {}
        }
    }
    out.unwrap_or(Tri::Maybe)
}

/// A callable argument's value: `getCallableParametersAcceptors()` of a `isCallable()->yes()` type, or not callable.
enum Value {
    Null,
    NotCallable,
    Points(Vec<Simple>),
}

/// A callee's parameter, for argument matching.
#[derive(Clone, PartialEq)]
struct Param {
    name: String,
    variadic: bool,
    /// TypeUtils::findCallableType() of its type is not null
    callable: bool,
    /// isImmediatelyInvokedCallable(): `Some(true)` / `Some(false)` from the tags, `None` maybe
    immediately: Option<bool>,
}

struct Producer<'c, 'a, 'b> {
    c: &'c Checker<'a, 'b>,
    points: Vec<Point>,
    /// the node gatherer's property assignments
    assigns: Vec<Point>,
    throws: bool,
    complete: bool,
    constructor: bool,
    pure_params: HashSet<Vec<u8>>,
    /// locals holding a closure literal, with its ClosureType's impure points (`None`: not known)
    closures: HashMap<Vec<u8>, Option<Bound>>,
}

impl<'c, 'a, 'b> Producer<'c, 'a, 'b> {
    fn new(c: &'c Checker<'a, 'b>, constructor: bool) -> Self {
        Producer {
            c,
            points: Vec::new(),
            assigns: Vec::new(),
            throws: false,
            complete: true,
            constructor,
            pure_params: HashSet::new(),
            closures: HashMap::new(),
        }
    }

    fn finish(self) -> Points {
        let mut points = self.points;
        points.extend(self.assigns);
        Points {
            points,
            throws: self.throws,
            complete: self.complete,
        }
    }

    fn push(
        &mut self,
        at: Span,
        identifier: &'static str,
        description: impl Into<String>,
        certain: bool,
    ) {
        self.points.push(Point {
            span: at,
            identifier,
            description: description.into(),
            certain,
            invoked: None,
        });
    }

    fn incomplete(&mut self) {
        self.complete = false;
    }

    /// StatementsHandler: the statements after an always-terminating one are not processed (declarations aside).
    fn statements(&mut self, statements: &[Statement]) -> bool {
        let mut terminates = false;
        for s in statements {
            if terminates {
                if !matches!(
                    s,
                    Statement::Function(_)
                        | Statement::Class(_)
                        | Statement::Interface(_)
                        | Statement::Trait(_)
                        | Statement::Enum(_)
                        | Statement::Noop(_)
                        | Statement::OpeningTag(_)
                        | Statement::ClosingTag(_)
                ) {
                    continue;
                }
            }
            terminates |= self.statement(s);
        }
        terminates
    }

    fn statement(&mut self, s: &Statement) -> bool {
        match s {
            Statement::Noop(_)
            | Statement::OpeningTag(_)
            | Statement::ClosingTag(_)
            | Statement::Function(_)
            | Statement::Class(_)
            | Statement::Interface(_)
            | Statement::Trait(_)
            | Statement::Enum(_) => false,
            Statement::Block(b) => self.statements(b.statements.as_slice()),
            Statement::Expression(s) => {
                self.expr(s.expression);
                matches!(
                    unparen(s.expression),
                    Expression::Throw(_)
                        | Expression::Construct(Construct::Exit(_) | Construct::Die(_))
                ) || self
                    .c
                    .type_of(s.expression)
                    .is_some_and(TUnion::is_explicit_never)
            }
            Statement::Echo(e) => {
                self.echo(e.values.iter().copied(), s.span());
                false
            }
            Statement::EchoTag(e) => {
                self.echo(e.values.iter().copied(), s.span());
                false
            }
            Statement::Inline(i) => {
                // PHP's closing tag token takes the newline that follows it.
                let mut at = i.span;
                if i.value.first() == Some(&b'\n') {
                    at.start.offset += 1;
                } else if i.value.starts_with(b"\r\n") {
                    at.start.offset += 2;
                }
                self.push(
                    at,
                    "betweenPhpTags",
                    "output between PHP opening and closing tags",
                    true,
                );
                false
            }
            Statement::Global(g) => {
                for v in g.variables.iter() {
                    if !matches!(v, Variable::Direct(_)) {
                        self.incomplete();
                    }
                }
                self.push(s.span(), "global", "global variable", true);
                false
            }
            Statement::Static(st) => {
                for item in st.items.iter() {
                    if let Some(value) = item.value() {
                        self.expr(value);
                    }
                }
                self.push(s.span(), "static", "static variable", true);
                false
            }
            Statement::Unset(u) => {
                for value in u.values.iter() {
                    self.unset(value);
                }
                false
            }
            Statement::Return(r) => {
                if let Some(value) = r.value {
                    self.expr(value);
                }
                true
            }
            Statement::Break(b) => {
                if b.level.is_some() {
                    self.incomplete();
                }
                true
            }
            Statement::Continue(k) => {
                if k.level.is_some() {
                    self.incomplete();
                }
                true
            }
            Statement::If(i) => {
                self.expr(i.condition);
                let mut all = self.branch(i.body.statements());
                for (condition, statements) in i.body.else_if_clauses() {
                    self.expr(condition);
                    all &= self.branch(statements);
                }
                match i.body.else_statements() {
                    Some(statements) => all & self.branch(statements),
                    None => false,
                }
            }
            Statement::While(w) => {
                self.expr(w.condition);
                self.branch(w.body.statements());
                self.closures.clear();
                false
            }
            Statement::DoWhile(d) => {
                self.branch(std::slice::from_ref(d.statement));
                self.closures.clear();
                self.expr(d.condition);
                false
            }
            Statement::For(f) => {
                for e in f.initializations.iter().chain(f.conditions.iter()) {
                    self.expr(e);
                }
                self.branch(f.body.statements());
                for e in f.increments.iter() {
                    self.expr(e);
                }
                self.closures.clear();
                false
            }
            Statement::Foreach(f) => {
                self.expr(f.expression);
                if let Some(key) = f.target.key() {
                    self.target(key);
                }
                self.target(f.target.value());
                self.branch(f.body.statements());
                self.closures.clear();
                false
            }
            Statement::Switch(sw) => {
                self.expr(sw.expression);
                for case in sw.body.cases() {
                    if let Some(value) = case.expression() {
                        self.expr(value);
                    }
                    self.branch(case.statements());
                }
                self.closures.clear();
                false
            }
            Statement::Try(t) => {
                let mut all = self.branch(t.block.statements.as_slice());
                for catch in t.catch_clauses.iter() {
                    all &= self.branch(catch.block.statements.as_slice());
                }
                self.closures.clear();
                match &t.finally_clause {
                    Some(f) => self.branch(f.block.statements.as_slice()) | all,
                    None => all,
                }
            }
            _ => {
                self.incomplete();
                false
            }
        }
    }

    /// A nested block: the locals it binds to closures are dropped afterwards.
    fn branch(&mut self, statements: &[Statement]) -> bool {
        let saved = self.closures.clone();
        let terminates = self.statements(statements);
        for (name, value) in std::mem::replace(&mut self.closures, saved) {
            if self.closures.get(&name) != Some(&value) {
                self.closures.remove(&name);
            }
        }
        terminates
    }

    fn echo<'e, 'arena: 'e>(
        &mut self,
        values: impl Iterator<Item = &'e Expression<'arena>>,
        at: Span,
    ) {
        for v in values {
            self.expr(v);
            self.to_string(v);
        }
        self.push(at, "echo", "echo", true);
    }

    /// ImplicitToStringCallHelper::processImplicitToStringCall().
    fn to_string(&mut self, e: &Expression) {
        let Some(t) = self.c.phpstan_type(e) else {
            self.incomplete();
            return;
        };
        if t.is_object() == Tri::No {
            return;
        }
        let names = t.object_class_names();
        let [name] = names.as_slice() else {
            self.incomplete();
            return;
        };
        if t.is_object() != Tri::Yes {
            self.incomplete();
            return;
        }
        let Some(k) = self.c.codebase.get_class_like(name.as_bytes()) else {
            self.incomplete();
            return;
        };
        let Some(m) = members::method(self.c, k, b"__toString") else {
            return;
        };
        self.throws = true;
        if let Some((certain, description)) = self.method_effects(&m) {
            self.push(e.span(), "methodCall", description, certain);
        }
    }

    /// UnsetHandler: a property unset is an impure point.
    fn unset(&mut self, e: &Expression) {
        let mut root = unparen(e);
        while let Expression::ArrayAccess(a) = root {
            root = unparen(a.array);
        }
        match root {
            Expression::Access(
                Access::Property(_) | Access::NullSafeProperty(_) | Access::StaticProperty(_),
            ) => {
                self.unset_operand(e);
                self.push(e.span(), "propertyUnset", "property unset", true);
            }
            Expression::Variable(Variable::Direct(_)) => self.unset_operand(e),
            _ => self.incomplete(),
        }
    }

    fn unset_operand(&mut self, e: &Expression) {
        match unparen(e) {
            Expression::ArrayAccess(a) => {
                self.unset_operand(a.array);
                self.expr(a.index);
            }
            Expression::Access(Access::Property(a)) => {
                self.expr(a.object);
                self.selector(&a.property);
            }
            Expression::Variable(Variable::Direct(v)) => {
                self.closures.remove(v.name);
            }
            e => self.expr(e),
        }
    }

    fn selector(&mut self, s: &ClassLikeMemberSelector) {
        if let ClassLikeMemberSelector::Expression(e) = s {
            self.expr(e.expression);
        }
    }

    fn expr(&mut self, e: &Expression) {
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
            Expression::Parenthesized(p) => self.expr(p.expression),
            Expression::Variable(Variable::Direct(v)) => {
                if SUPERGLOBALS.contains(&v.name) {
                    self.push(
                        e.span(),
                        "superglobal",
                        "access to superglobal variable",
                        true,
                    );
                }
            }
            Expression::Binary(b) => {
                self.expr(b.lhs);
                self.expr(b.rhs);
                if matches!(b.operator, BinaryOperator::StringConcat(_)) {
                    self.to_string(b.lhs);
                    self.to_string(b.rhs);
                } else if matches!(
                    b.operator,
                    BinaryOperator::Division(_) | BinaryOperator::Modulo(_)
                ) {
                    self.throws = true;
                }
            }
            Expression::UnaryPrefix(u) => match u.operator {
                UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_) => {
                    self.modify(u.operand)
                }
                UnaryPrefixOperator::StringCast(..) => {
                    self.expr(u.operand);
                    self.to_string(u.operand);
                }
                UnaryPrefixOperator::Reference(_) => self.incomplete(),
                _ => self.expr(u.operand),
            },
            Expression::UnaryPostfix(u) => self.modify(u.operand),
            Expression::Conditional(t) => {
                self.expr(t.condition);
                if let Some(then) = t.then {
                    self.expr(then);
                }
                self.expr(t.r#else);
            }
            Expression::Assignment(a) => self.assignment(a),
            Expression::Array(a) => self.elements(a.elements.iter()),
            Expression::LegacyArray(a) => self.elements(a.elements.iter()),
            Expression::ArrayAccess(a) => {
                self.expr(a.array);
                self.expr(a.index);
                // ArrayDimFetchHandler: an ArrayAccess receiver's offsetGet() call
                if !self
                    .c
                    .phpstan_type(a.array)
                    .is_some_and(|t| t.is_array() == Tri::Yes || t.is_string() == Tri::Yes)
                {
                    self.incomplete();
                }
            }
            Expression::CompositeString(s) => {
                for part in s.parts().iter() {
                    let e = match part {
                        StringPart::Expression(e) => *e,
                        StringPart::BracedExpression(b) => b.expression,
                        StringPart::Literal(_) => continue,
                    };
                    self.expr(e);
                    self.to_string(e);
                }
            }
            Expression::Access(Access::Property(a)) => {
                self.expr(a.object);
                self.selector(&a.property);
                self.property_hook(a.object, &a.property);
            }
            Expression::Access(Access::NullSafeProperty(a)) => {
                self.expr(a.object);
                self.selector(&a.property);
                self.property_hook(a.object, &a.property);
            }
            Expression::Access(Access::StaticProperty(a)) => {
                self.static_property(a);
                self.push(
                    e.span(),
                    "staticPropertyAccess",
                    "static property access",
                    true,
                );
            }
            Expression::Access(Access::ClassConstant(a)) => {
                self.expr(a.class);
                if let ClassLikeConstantSelector::Expression(s) = &a.constant {
                    self.expr(s.expression);
                }
            }
            Expression::Construct(construct) => self.construct(e, construct),
            Expression::Clone(k) => {
                self.expr(k.object);
                self.incomplete(); // __clone()
            }
            Expression::Match(m) => {
                self.expr(m.expression);
                for arm in m.arms.iter() {
                    if let MatchArm::Expression(a) = arm {
                        for condition in a.conditions.iter() {
                            self.expr(condition);
                        }
                    }
                    self.expr(arm.expression());
                }
                if !self.c.exhaustive(m) {
                    self.throws = true;
                }
            }
            Expression::Throw(t) => {
                self.expr(t.exception);
                self.throws = true;
            }
            Expression::Yield(y) => match y {
                Yield::Value(v) => {
                    if let Some(value) = v.value {
                        self.expr(value);
                    }
                    self.push(e.span(), "yield", "yield", true);
                }
                Yield::Pair(p) => {
                    self.expr(p.key);
                    self.expr(p.value);
                    self.push(e.span(), "yield", "yield", true);
                }
                Yield::From(f) => {
                    self.expr(f.iterator);
                    self.push(e.span(), "yieldFrom", "yield from", true);
                    self.throws = true;
                }
            },
            Expression::Call(Call::Function(call)) => self.function_call(e, call),
            Expression::Call(Call::Method(m)) => {
                self.expr(m.object);
                self.selector(&m.method);
                self.method_call(e, m.object, &m.method, &m.argument_list, false);
            }
            Expression::Call(Call::NullSafeMethod(m)) => {
                self.expr(m.object);
                self.selector(&m.method);
                self.method_call(e, m.object, &m.method, &m.argument_list, false);
            }
            Expression::Call(Call::StaticMethod(m)) => {
                if !matches!(
                    unparen(m.class),
                    Expression::Identifier(_)
                        | Expression::Self_(_)
                        | Expression::Static(_)
                        | Expression::Parent(_)
                ) {
                    self.expr(m.class);
                }
                self.selector(&m.method);
                self.method_call(e, m.class, &m.method, &m.argument_list, true);
            }
            // a first-class callable only creates a Closure
            Expression::PartialApplication(p) if p.is_first_class_callable() => {}
            Expression::Instantiation(new) => self.instantiation(e, new),
            _ => self.incomplete(),
        }
    }

    fn construct(&mut self, e: &Expression, construct: &Construct) {
        match construct {
            Construct::Isset(i) => {
                for v in i.values.iter() {
                    self.expr(v);
                }
            }
            Construct::Empty(i) => self.expr(i.value),
            Construct::Print(p) => {
                self.expr(p.value);
                self.to_string(p.value);
                self.push(e.span(), "print", "print", true);
            }
            Construct::Eval(v) => {
                self.expr(v.value);
                self.push(e.span(), "eval", "eval", true);
                self.throws = true;
            }
            Construct::Include(v) => self.include(e, v.value, "include"),
            Construct::IncludeOnce(v) => self.include(e, v.value, "include"),
            Construct::Require(v) => self.include(e, v.value, "require"),
            Construct::RequireOnce(v) => self.include(e, v.value, "require"),
            Construct::Exit(x) => {
                for arg in x.arguments.iter().flat_map(|a| a.arguments.iter()) {
                    self.expr(arg.value());
                }
                self.push(e.span(), "exit", "exit", true);
            }
            Construct::Die(x) => {
                for arg in x.arguments.iter().flat_map(|a| a.arguments.iter()) {
                    self.expr(arg.value());
                }
                self.push(e.span(), "die", "die", true);
            }
        }
    }

    fn include(&mut self, e: &Expression, value: &Expression, identifier: &'static str) {
        self.expr(value);
        self.push(e.span(), identifier, identifier, true);
        self.throws = true;
    }

    fn static_property(&mut self, a: &StaticPropertyAccess) {
        if !matches!(
            unparen(a.class),
            Expression::Identifier(_)
                | Expression::Self_(_)
                | Expression::Static(_)
                | Expression::Parent(_)
        ) {
            self.expr(a.class);
        }
        if !matches!(a.property, Variable::Direct(_)) {
            self.incomplete();
        }
    }

    /// NodeScopeResolver::getImpurePointsFromPropertyHook() of a property read: a hooked property is not ported.
    fn property_hook(&mut self, object: &Expression, property: &ClassLikeMemberSelector) {
        let ClassLikeMemberSelector::Identifier(name) = property else {
            self.incomplete();
            return;
        };
        let Some(t) = self.c.phpstan_type(object) else {
            self.incomplete();
            return;
        };
        for class in t.object_class_names() {
            let Some(k) = self.c.codebase.get_class_like(class.as_bytes()) else {
                continue;
            };
            let mut property = Vec::with_capacity(name.value.len() + 1);
            property.push(b'$');
            property.extend_from_slice(name.value);
            if self
                .c
                .codebase
                .get_property(k.name.as_bytes(), &property)
                .is_some_and(|p| !p.hooks.is_empty())
            {
                self.incomplete();
            }
        }
    }

    fn elements<'e, 'arena: 'e>(&mut self, items: impl Iterator<Item = &'e ArrayElement<'arena>>) {
        for item in items {
            match item {
                ArrayElement::KeyValue(kv) => {
                    self.expr(kv.key);
                    self.expr(kv.value);
                }
                ArrayElement::Value(v) => self.expr(v.value),
                ArrayElement::Variadic(v) => {
                    self.expr(v.value);
                    self.incomplete(); // an iterable's getIterator()
                }
                ArrayElement::Missing(_) => {}
            }
        }
    }

    fn assignment(&mut self, a: &Assignment) {
        let simple = matches!(a.operator, AssignmentOperator::Assign(_));
        if !simple {
            // AssignOpHandler: the target is read first
            self.modify(a.lhs);
            self.expr(a.rhs);
            match a.operator {
                AssignmentOperator::Concat(_) => self.to_string(a.rhs),
                AssignmentOperator::Division(_) | AssignmentOperator::Modulo(_) => {
                    self.throws = true;
                }
                _ => {}
            }
            return;
        }
        if let Expression::UnaryPrefix(u) = unparen(a.rhs)
            && u.operator.is_reference()
        {
            // AssignHandler: a reference to a property
            let mut referenced = unparen(u.operand);
            while let Expression::ArrayAccess(a) = referenced {
                referenced = unparen(a.array);
            }
            self.reference_operand(u.operand);
            if matches!(
                referenced,
                Expression::Access(Access::Property(_) | Access::StaticProperty(_))
            ) {
                self.push(
                    a.span(),
                    "propertyAssignByRef",
                    "property assignment by reference",
                    false,
                );
            }
            self.target(a.lhs);
            return;
        }
        // AssignHandler::prepareTarget() walks the target's sub-expressions before the value
        self.target_operands(a.lhs);
        self.expr(a.rhs);
        let closure = self.bind(a.rhs);
        self.write(a.lhs, a.span());
        if let Expression::Variable(Variable::Direct(v)) = unparen(a.lhs) {
            match closure {
                Some(points) => {
                    self.closures.insert(v.name.to_vec(), points);
                }
                None => {
                    self.closures.remove(v.name);
                }
            }
        }
    }

    /// The operand of `&`: its sub-expressions without reading it.
    fn reference_operand(&mut self, e: &Expression) {
        match unparen(e) {
            Expression::Variable(Variable::Direct(v)) => {
                self.closures.remove(v.name);
            }
            Expression::ArrayAccess(a) => {
                self.reference_operand(a.array);
                self.expr(a.index);
            }
            Expression::ArrayAppend(a) => self.reference_operand(a.array),
            Expression::Access(Access::Property(a)) => {
                self.expr(a.object);
                self.selector(&a.property);
            }
            Expression::Access(Access::StaticProperty(a)) => self.static_property(a),
            _ => self.incomplete(),
        }
    }

    /// The callable a value literally is: a closure, a first-class callable, a function name or a
    /// `[$object, 'method']` array (`None` for other values, `Some(None)` when it is not known).
    fn bind(&mut self, e: &Expression) -> Option<Option<Bound>> {
        match unparen(e) {
            Expression::Closure(k) => Some(self.closure_type(k).map(Bound::plain)),
            Expression::ArrowFunction(f) => Some(self.arrow_type(f).map(Bound::plain)),
            Expression::PartialApplication(PartialApplication::Function(f))
                if f.argument_list.is_first_class_callable() =>
            {
                Some(function_of(self.c, f.function).map(|callee| self.function_bound(&callee)))
            }
            Expression::PartialApplication(PartialApplication::Method(m))
                if m.argument_list.is_first_class_callable() =>
            {
                Some(
                    self.c
                        .method_target(m.object, &m.method)
                        .map(|method| self.method_bound(&method)),
                )
            }
            Expression::PartialApplication(PartialApplication::StaticMethod(m))
                if m.argument_list.is_first_class_callable() =>
            {
                Some(
                    self.c
                        .method_target(m.class, &m.method)
                        .map(|method| self.method_bound(&method)),
                )
            }
            Expression::Literal(Literal::String(s)) => {
                let lower = String::from_utf8_lossy(s.value?)
                    .trim_start_matches('\\')
                    .to_ascii_lowercase();
                if lower.contains("::") {
                    return Some(None);
                }
                let callee = match self
                    .c
                    .codebase
                    .get_function(lower.as_bytes())
                    .filter(|f| !f.flags.is_built_in())
                {
                    Some(f) => Callee::User(f),
                    None => Callee::Builtin(self.c.builtins.functions.get(&lower)?),
                };
                Some(Some(self.function_bound(&callee)))
            }
            Expression::Array(a) => {
                let [ArrayElement::Value(object), ArrayElement::Value(method)] =
                    a.elements.as_slice()
                else {
                    return None;
                };
                let Expression::Literal(Literal::String(name)) = unparen(method.value) else {
                    return None;
                };
                let found = self
                    .receiver(object.value, false)
                    .filter(|(classes, _)| classes.len() == 1)
                    .and_then(|(classes, _)| members::method(self.c, classes[0], name.value?));
                Some(found.map(|m| self.method_bound(&m)))
            }
            _ => None,
        }
    }

    /// A function's callable acceptor: createFromVariant() without a scope, its conditional parameters.
    fn function_bound(&mut self, callee: &Callee) -> Bound {
        let points = self.function_points(callee, None).into_iter().collect();
        let (callable, passed) = self.function_cond(callee);
        let params = self.function_params(callee);
        Bound {
            points,
            cond: (!callable.is_empty() || !passed.is_empty())
                .then_some((params, callable, passed)),
        }
    }

    fn method_bound(&mut self, m: &Method) -> Bound {
        let points = self
            .method_effects(m)
            .map(|(certain, d)| ("methodCall", d, certain))
            .into_iter()
            .collect();
        let (callable, passed) = self.method_cond(std::slice::from_ref(m));
        let params = match m {
            Method::User { f, .. } => self.user_params(f),
            Method::Builtin { .. } => Vec::new(),
        };
        Bound {
            points,
            cond: (!callable.is_empty() || !passed.is_empty())
                .then_some((params, callable, passed)),
        }
    }

    /// A function's `@pure-unless-callable-is-impure` and `@pure-unless-parameter-passed` parameters.
    fn function_cond(&self, function: &Callee) -> (Vec<String>, Vec<(String, bool)>) {
        match function {
            Callee::User(f) => {
                let raw = self.doc(f);
                (
                    declared::tagged_params(
                        &raw,
                        &[
                            "@pure-unless-callable-is-impure",
                            "@phpstan-pure-unless-callable-is-impure",
                        ],
                    ),
                    declared::tagged_params(
                        &raw,
                        &[
                            "@pure-unless-parameter-passed",
                            "@phpstan-pure-unless-parameter-passed",
                        ],
                    )
                    .into_iter()
                    .map(|n| (n, true))
                    .collect(),
                )
            }
            Callee::Builtin(f) => {
                let lower = f.name.to_ascii_lowercase();
                let names = |table: &[(&str, &[&str])]| -> Vec<String> {
                    table
                        .iter()
                        .find(|(n, _)| *n == lower)
                        .map(|(_, p)| p.iter().map(|s| s.to_string()).collect())
                        .unwrap_or_default()
                };
                (
                    names(PURE_UNLESS_CALLABLE_IS_IMPURE),
                    names(PURE_UNLESS_PARAMETER_PASSED)
                        .into_iter()
                        .map(|n| (n, true))
                        .collect(),
                )
            }
        }
    }

    /// MergedConditionalPurityParameters of the methods a call resolves to: a flag on some of them only is maybe.
    fn method_cond(&self, methods: &[Method]) -> (Vec<String>, Vec<(String, bool)>) {
        let flags: Vec<(Vec<String>, Vec<String>)> = methods
            .iter()
            .map(|m| match m {
                Method::User { declaring, f, .. } => {
                    let (callable, passed, _) =
                        declared::method_conditional_purity(self.c, declaring, f.name.as_bytes());
                    (callable, passed)
                }
                Method::Builtin { .. } => (Vec::new(), Vec::new()),
            })
            .collect();
        let mut callable = Vec::new();
        let mut passed: Vec<(String, bool)> = Vec::new();
        for (c, p) in &flags {
            for n in c {
                if !callable.contains(n) {
                    callable.push(n.clone());
                }
            }
            for n in p {
                if !passed.iter().any(|(m, _)| m == n) {
                    passed.push((n.clone(), flags.iter().all(|(_, p)| p.contains(n))));
                }
            }
        }
        (callable, passed)
    }

    /// A target's sub-expressions, walked before the assigned value.
    fn target_operands(&mut self, e: &Expression) {
        match unparen(e) {
            Expression::Variable(Variable::Direct(_)) => {}
            Expression::ArrayAccess(a) => {
                self.target_root(a.array);
                self.expr(a.index);
            }
            Expression::ArrayAppend(a) => self.target_root(a.array),
            Expression::Access(Access::Property(a)) => {
                self.expr(a.object);
                self.selector(&a.property);
            }
            Expression::Access(Access::StaticProperty(a)) => self.static_property(a),
            Expression::Array(_) | Expression::LegacyArray(_) | Expression::List(_) => {}
            _ => self.incomplete(),
        }
    }

    /// The container of an offset write: read (a superglobal's access point), its own offsets walked.
    fn target_root(&mut self, e: &Expression) {
        match unparen(e) {
            Expression::Variable(Variable::Direct(v)) => {
                if SUPERGLOBALS.contains(&v.name) {
                    self.push(
                        e.span(),
                        "superglobal",
                        "access to superglobal variable",
                        true,
                    );
                }
            }
            Expression::ArrayAccess(a) => {
                self.target_root(a.array);
                self.expr(a.index);
            }
            Expression::ArrayAppend(a) => self.target_root(a.array),
            Expression::Access(Access::Property(a)) => {
                self.expr(a.object);
                self.selector(&a.property);
            }
            Expression::Access(Access::StaticProperty(a)) => self.static_property(a),
            _ => self.incomplete(),
        }
        if !self
            .c
            .phpstan_type(e)
            .is_some_and(|t| t.is_array() == Tri::Yes)
            && !matches!(unparen(e), Expression::Variable(_))
        {
            self.incomplete(); // offsetSet() of an ArrayAccess container
        }
    }

    /// The write itself: a superglobal assignment or a PropertyAssignNode.
    fn write(&mut self, e: &Expression, at: Span) {
        let mut root = unparen(e);
        while let Expression::ArrayAccess(ArrayAccess { array, .. })
        | Expression::ArrayAppend(ArrayAppend { array, .. }) = root
        {
            root = unparen(array);
        }
        match root {
            Expression::Variable(Variable::Direct(v)) => {
                if std::ptr::eq(root, unparen(e)) && SUPERGLOBALS.contains(&v.name) {
                    self.push(
                        e.span(),
                        "superglobal",
                        "assign to superglobal variable",
                        true,
                    );
                }
                if !std::ptr::eq(root, unparen(e)) {
                    self.closures.remove(v.name);
                }
            }
            Expression::Access(Access::Property(p)) => {
                if !(self.constructor
                    && matches!(unparen(p.object), Expression::Variable(Variable::Direct(v)) if v.name == b"$this"))
                {
                    self.assigns.push(Point {
                        span: at,
                        identifier: "propertyAssign",
                        description: "property assignment".into(),
                        certain: true,
                        invoked: None,
                    });
                }
            }
            Expression::Access(Access::StaticProperty(_)) => self.assigns.push(Point {
                span: at,
                identifier: "propertyAssign",
                description: "property assignment".into(),
                certain: true,
                invoked: None,
            }),
            Expression::Array(a) => {
                for item in a.elements.iter() {
                    self.destructure(item);
                }
            }
            Expression::LegacyArray(a) => {
                for item in a.elements.iter() {
                    self.destructure(item);
                }
            }
            Expression::List(a) => {
                for item in a.elements.iter() {
                    self.destructure(item);
                }
            }
            _ => self.incomplete(),
        }
    }

    fn destructure(&mut self, item: &ArrayElement) {
        match item {
            ArrayElement::KeyValue(kv) => {
                self.expr(kv.key);
                self.target(kv.value);
            }
            ArrayElement::Value(v) => self.target(v.value),
            ArrayElement::Missing(_) => {}
            ArrayElement::Variadic(_) => self.incomplete(),
        }
    }

    /// A write target without a value walk (foreach, destructuring, by-ref assignment).
    fn target(&mut self, e: &Expression) {
        let e = match unparen(e) {
            Expression::UnaryPrefix(u) if u.operator.is_reference() => u.operand,
            e => e,
        };
        self.target_operands(e);
        let at = e.span();
        self.write(e, at);
        if let Expression::Variable(Variable::Direct(v)) = unparen(e) {
            self.closures.remove(v.name);
        }
    }

    /// A read-modify-write target (`++`, `--`, compound assignment): read, then written.
    fn modify(&mut self, e: &Expression) {
        match unparen(e) {
            Expression::Variable(Variable::Direct(v)) => {
                if SUPERGLOBALS.contains(&v.name) {
                    self.incomplete();
                }
                self.closures.remove(v.name);
            }
            Expression::Access(Access::StaticProperty(a)) => {
                self.static_property(a);
                self.push(
                    e.span(),
                    "staticPropertyAccess",
                    "static property access",
                    true,
                );
                self.write(e, e.span());
            }
            Expression::Access(Access::Property(p)) => {
                self.expr(p.object);
                self.selector(&p.property);
                self.property_hook(p.object, &p.property);
                self.write(e, e.span());
            }
            Expression::ArrayAccess(a) => {
                self.target_root(a.array);
                self.expr(a.index);
                self.write(e, e.span());
            }
            _ => self.incomplete(),
        }
    }

    /// ClosureTypeResolver: a closure's own points (property assignments included), then its by-ref use and
    /// by-ref parameter points. `None` when its body is not fully produced.
    fn closure_type(&mut self, k: &Closure) -> Option<Vec<Simple>> {
        let (points, assigns) = self.closure_body(k)?;
        let mut simple: Vec<Simple> = assigns
            .iter()
            .chain(points.iter())
            .map(|p| (p.identifier, p.description.clone(), p.certain))
            .collect();
        if k.use_clause
            .iter()
            .flat_map(|u| u.variables.iter())
            .any(|u| u.ampersand.is_some())
        {
            simple.push((
                "functionCall",
                "call to a Closure with by-ref use".into(),
                true,
            ));
        }
        for p in k.parameter_list.parameters.iter() {
            if p.ampersand.is_some() {
                simple.push((
                    "functionCall",
                    "call to a Closure with by-ref parameter".into(),
                    true,
                ));
            }
        }
        Some(simple)
    }

    /// A closure body's statement points and its gathered property assignments.
    fn closure_body(&mut self, k: &Closure) -> Option<(Vec<Point>, Vec<Point>)> {
        let mut inner = Producer::new(self.c, false);
        inner.statements(k.body.statements.as_slice());
        inner.complete.then_some((inner.points, inner.assigns))
    }

    fn arrow_type(&mut self, f: &ArrowFunction) -> Option<Vec<Simple>> {
        let (points, assigns) = self.arrow_body(f)?;
        let mut simple: Vec<Simple> = assigns
            .iter()
            .chain(points.iter())
            .map(|p| (p.identifier, p.description.clone(), p.certain))
            .collect();
        for p in f.parameter_list.parameters.iter() {
            if p.ampersand.is_some() {
                simple.push((
                    "functionCall",
                    "call to a Closure with by-ref parameter".into(),
                    true,
                ));
            }
        }
        Some(simple)
    }

    fn arrow_body(&mut self, f: &ArrowFunction) -> Option<(Vec<Point>, Vec<Point>)> {
        let mut inner = Producer::new(self.c, false);
        inner.closures = self.closures.clone();
        inner.expr(f.expression);
        inner.complete.then_some((inner.points, inner.assigns))
    }

    /// The callable value of an argument or callee: `None` when it is not known.
    fn value(&mut self, e: &Expression) -> Option<Value> {
        match unparen(e) {
            Expression::Literal(Literal::Null(_)) => Some(Value::Null),
            Expression::Variable(Variable::Direct(v)) if self.closures.contains_key(v.name) => self
                .closures
                .get(v.name)
                .cloned()
                .flatten()
                .map(|b| Value::Points(b.points)),
            Expression::Literal(Literal::String(_)) => match self.bind(e) {
                Some(Some(b)) => Some(Value::Points(b.points)),
                _ => Some(Value::NotCallable),
            },
            Expression::Closure(_)
            | Expression::ArrowFunction(_)
            | Expression::PartialApplication(_)
            | Expression::Array(_)
                if self.bind(e).is_some() =>
            {
                self.bind(e).flatten().map(|b| Value::Points(b.points))
            }
            Expression::Conditional(t) => {
                let then = self.value(t.then.unwrap_or(t.condition))?;
                let otherwise = self.value(t.r#else)?;
                Some(match (then, otherwise) {
                    (Value::Points(mut a), Value::Points(b)) => {
                        // a union's acceptors; isPure() and()s them
                        let pure = and(purity(&a), purity(&b));
                        a.extend(b);
                        if pure == Tri::Maybe && a.iter().all(|p| p.2) {
                            a.push(("functionCall", "call to a callable".into(), false));
                        }
                        Value::Points(a)
                    }
                    _ => Value::NotCallable,
                })
            }
            e => {
                let t = self.c.phpstan_type(e)?;
                let name = match e {
                    Expression::Variable(Variable::Direct(v)) => Some(&v.name[1..]),
                    _ => None,
                };
                let pure = name.is_some_and(|n| self.pure_params.contains(n));
                self.typed_value(&t, pure)
            }
        }
    }

    /// getCallableParametersAcceptors()'s impure points of a declared callable type.
    fn typed_value(&self, t: &Type, pure: bool) -> Option<Value> {
        match t {
            Type::Null => Some(Value::Null),
            Type::Callable(_) => Some(Value::Points(if pure {
                vec![]
            } else {
                vec![("functionCall", "call to a callable".into(), false)]
            })),
            Type::Closure(_) => Some(Value::Points(if pure {
                vec![]
            } else {
                vec![("functionCall", "call to a Closure".into(), false)]
            })),
            Type::Object(name, _, _) if name.eq_ignore_ascii_case("Closure") => {
                Some(Value::Points(if pure {
                    vec![]
                } else {
                    vec![("functionCall", "call to a Closure".into(), false)]
                }))
            }
            _ if t.is_callable() == Tri::No => Some(Value::NotCallable),
            Type::Union(types) if t.is_callable() == Tri::Maybe => {
                // only maybe callable: no acceptor is consulted
                types
                    .iter()
                    .all(|t| {
                        matches!(
                            t,
                            Type::Callable(_)
                                | Type::Closure(_)
                                | Type::Null
                                | Type::Int
                                | Type::String
                                | Type::Object(..)
                        )
                    })
                    .then_some(Value::NotCallable)
            }
            _ => None,
        }
    }

    /// The impure points of a callee invoked through a callable value (`$f()`): the acceptors of its type.
    fn callee_points(&mut self, callee: &Expression) -> Option<Bound> {
        if let Expression::Variable(Variable::Direct(v)) = unparen(callee)
            && let Some(bound) = self.closures.get(v.name)
        {
            return bound.clone();
        }
        if let Some(bound) = self.bind(callee) {
            return bound;
        }
        let t = self.c.phpstan_type(callee)?;
        let pure = matches!(unparen(callee), Expression::Variable(Variable::Direct(v)) if self.pure_params.contains(&v.name[1..]));
        let members: Vec<&Type> = match &t {
            Type::Union(types) => types.iter().collect(),
            t => vec![t],
        };
        let mut points = Vec::new();
        for member in members {
            match member {
                Type::String => points.push(("functionCall", "call to a callable".into(), false)),
                Type::Null => {}
                m => match self.typed_value(m, pure)? {
                    Value::Points(p) => points.extend(p),
                    _ => return None,
                },
            }
        }
        Some(Bound::plain(points))
    }

    fn function_call(&mut self, e: &Expression, call: &FunctionCall) {
        let callee = unparen(call.function);
        if !matches!(callee, Expression::Identifier(_)) {
            // FuncCallHandler: the callable's acceptor points, at the call
            self.expr(callee);
            match self.callee_points(callee) {
                Some(Bound { mut points, cond }) => {
                    // SimpleImpurePoint::narrowByConditionalPurity() with the call's arguments
                    if let Some((params, callable, passed)) = cond {
                        match self.verdict(&params, &callable, &passed, Some(&call.argument_list)) {
                            Some(Tri::Yes) => points.clear(),
                            Some(Tri::No) => points.iter_mut().for_each(|p| p.2 = true),
                            _ => {}
                        }
                    }
                    let invoked = match callee {
                        Expression::Variable(Variable::Direct(v)) => {
                            Some(String::from_utf8_lossy(&v.name[1..]).into_owned())
                        }
                        _ => None,
                    };
                    for (identifier, description, certain) in points {
                        self.push(e.span(), identifier, description, certain);
                        self.points.last_mut().expect("pushed").invoked = invoked.clone();
                    }
                }
                None => self.incomplete(),
            }
            self.throws = true;
            self.arguments(&call.argument_list, &[]);
            return;
        }
        let Some(function) = function_of(self.c, callee) else {
            self.arguments(&call.argument_list, &[]);
            self.push(e.span(), "functionCall", "call to unknown function", false);
            self.throws = true;
            return;
        };
        if let Callee::User(f) = &function
            && self.c.file_contents(f.span.file_id).is_none()
        {
            self.incomplete();
        }
        let params = self.function_params(&function);
        self.arguments(&call.argument_list, &params);
        if super::declaration_effects::TESTING_FUNCTIONS
            .iter()
            .any(|t| {
                t.as_bytes()
                    .eq_ignore_ascii_case(self.c.names.get(&callee.span()))
            })
        {
            self.incomplete();
            return;
        }
        if let Some((identifier, description, certain)) =
            self.function_points(&function, Some(&call.argument_list))
        {
            self.push(e.span(), identifier, description, certain);
        }
        match &function {
            Callee::User(f) if !throws_void(f) => self.throws = true,
            Callee::User(_) => {}
            Callee::Builtin(_) => self.throws = true,
        }
    }

    /// SimpleImpurePoint::createFromVariant() of a function, with the call's arguments when `args` is given.
    fn function_points(
        &mut self,
        function: &Callee,
        args: Option<&ArgumentList>,
    ) -> Option<Simple> {
        match function {
            Callee::User(f) => {
                let name = String::from_utf8_lossy(f.original_name.as_bytes()).into_owned();
                let raw = self.doc(f);
                let void = f
                    .return_type_metadata
                    .as_ref()
                    .is_some_and(|r| r.type_union.is_void());
                let pure = super::declaration_effects::purity(&raw)
                    .or_else(|| f.flags.is_pure().then_some(true));
                if !void && pure == Some(true) {
                    return None;
                }
                let mut certain = pure == Some(false) || void;
                if !certain && let Some(args) = args {
                    certain = self.narrow(function, args)?;
                }
                Some((
                    "functionCall",
                    format!("call to function {name}()"),
                    certain,
                ))
            }
            Callee::Builtin(f) => {
                let void = f.variants.first().is_some_and(|v| v.ret == "void");
                let effects = if void { "Yes" } else { f.side_effects.as_str() };
                if effects == "No" {
                    return None;
                }
                let mut certain = effects == "Yes" || void;
                let lower = f.name.to_ascii_lowercase();
                if !certain && let Some(args) = args {
                    certain = self.narrow(function, args)?;
                }
                if let Some(args) = args
                    && FLIP.contains(&lower.as_str())
                {
                    // SIDE_EFFECT_FLIP_PARAMETERS: no side effect with a truthy `return`
                    let mut named = false;
                    for (i, arg) in args.arguments.iter().enumerate() {
                        let flip = match arg {
                            Argument::Named(n) => {
                                named = true;
                                n.name.value == b"return"
                            }
                            Argument::Positional(_) => !named && i == 1,
                        };
                        if flip {
                            if matches!(
                                self.c.phpstan_type(arg.value()),
                                Some(Type::ConstBool(true))
                            ) {
                                return None;
                            }
                            break;
                        }
                    }
                }
                Some((
                    "functionCall",
                    format!("call to function {}()", f.name),
                    certain,
                ))
            }
        }
    }

    /// resolveConditionalPurityVerdict() of a possibly impure call: `None` when it is pure, else isCertain().
    fn narrow(&mut self, function: &Callee, args: &ArgumentList) -> Option<bool> {
        let (callable, passed) = self.function_cond(function);
        let params = self.function_params(function);
        match self.verdict(&params, &callable, &passed, Some(args)) {
            Some(Tri::Yes) => None,
            Some(Tri::No) => Some(true),
            _ => Some(false),
        }
    }

    /// A user declaration's PHPDoc.
    fn doc(&self, f: &FunctionLikeMetadata) -> &'a [u8] {
        declared::doc_at(self.c, f.span)
            .map(|(raw, _, _)| raw)
            .unwrap_or_default()
    }

    fn function_params(&self, function: &Callee) -> Vec<Param> {
        match function {
            Callee::User(f) => self.user_params(f),
            Callee::Builtin(f) => f
                .variants
                .first()
                .map(|v| {
                    v.params
                        .iter()
                        .map(|p| Param {
                            name: p.name.clone(),
                            variadic: p.variadic,
                            callable: p.ty.contains("callable") || p.ty.contains("Closure"),
                            immediately: None,
                        })
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    fn user_params(&self, f: &FunctionLikeMetadata) -> Vec<Param> {
        let raw = self.doc(f);
        let immediately = declared::tagged_params(
            &raw,
            &[
                "@param-immediately-invoked-callable",
                "@phpstan-param-immediately-invoked-callable",
            ],
        );
        let later = declared::tagged_params(
            &raw,
            &[
                "@param-later-invoked-callable",
                "@phpstan-param-later-invoked-callable",
            ],
        );
        f.parameters
            .iter()
            .map(|p| {
                let name = String::from_utf8_lossy(&p.name.0.as_bytes()[1..]).into_owned();
                let callable = p
                    .type_metadata
                    .as_ref()
                    .or(p.type_declaration_metadata.as_ref())
                    .is_some_and(|t| t.type_union.types.iter().any(callable_atom));
                Param {
                    immediately: if immediately.contains(&name) {
                        Some(true)
                    } else if later.contains(&name) {
                        Some(false)
                    } else {
                        None
                    },
                    variadic: p.flags.is_variadic(),
                    callable,
                    name,
                }
            })
            .collect()
    }

    /// SimpleImpurePoint::resolveConditionalPurityVerdict().
    fn verdict(
        &mut self,
        params: &[Param],
        callable: &[String],
        passed: &[(String, bool)],
        args: Option<&ArgumentList>,
    ) -> Option<Tri> {
        let mut verdict: Option<Tri> = None;
        for (i, p) in params.iter().enumerate() {
            if callable.contains(&p.name) {
                let v = verdict.get_or_insert(Tri::Yes);
                let (arg, unpacked) = match_arg(args, params, i);
                let Some(arg) = arg else {
                    if unpacked {
                        *v = and(*v, Tri::Maybe);
                    }
                    continue;
                };
                let value = self.value(arg.value());
                let v = verdict.get_or_insert(Tri::Yes);
                match value {
                    None => {
                        self.incomplete();
                        *v = and(*v, Tri::Maybe);
                    }
                    Some(Value::Null) => {}
                    Some(Value::NotCallable) => *v = and(*v, Tri::Maybe),
                    Some(Value::Points(points)) => *v = and(*v, purity(&points)),
                }
            }
        }
        let mut passed_verdict: Option<Tri> = None;
        for (i, p) in params.iter().enumerate() {
            if let Some((_, certain)) = passed.iter().find(|(n, _)| *n == p.name) {
                let v = passed_verdict.get_or_insert(Tri::Yes);
                let (arg, unpacked) = match_arg(args, params, i);
                if arg.is_some() {
                    *v = and(*v, if *certain { Tri::No } else { Tri::Maybe });
                } else if unpacked {
                    *v = and(*v, Tri::Maybe);
                }
            }
        }
        match (verdict, passed_verdict) {
            (v, None) => v,
            (None, p) => p,
            (Some(a), Some(b)) => Some(and(a, b)),
        }
    }

    /// ArgumentsHandler::processArgs(): each argument's points, and an immediately invoked callable argument's.
    fn arguments(&mut self, args: &ArgumentList, params: &[Param]) {
        let mut named = false;
        for (i, arg) in args.arguments.iter().enumerate() {
            let param = match arg {
                Argument::Named(n) => {
                    named = true;
                    params.iter().find(|p| p.name.as_bytes() == n.name.value)
                }
                Argument::Positional(p) => {
                    if p.ellipsis.is_some() {
                        self.incomplete();
                    }
                    if named {
                        None
                    } else {
                        params
                            .get(i)
                            .or_else(|| params.last().filter(|p| p.variadic))
                    }
                }
            };
            let immediately = param.is_some_and(|p| p.immediately.unwrap_or(p.callable));
            match unparen(arg.value()) {
                Expression::Closure(k) => {
                    if immediately {
                        match self.closure_body(k) {
                            Some((points, _)) => self.points.extend(points),
                            None => self.incomplete(),
                        }
                        self.throws = true;
                    }
                }
                Expression::ArrowFunction(f) => {
                    if immediately {
                        match self.arrow_body(f) {
                            Some((points, _)) => self.points.extend(points),
                            None => self.incomplete(),
                        }
                        self.throws = true;
                    }
                }
                value => {
                    self.expr(value);
                    if immediately {
                        match self.value(value) {
                            Some(Value::Points(points)) => {
                                for (identifier, description, certain) in points {
                                    self.push(value.span(), identifier, description, certain);
                                }
                                self.throws = true;
                            }
                            Some(_) => {}
                            None => self.incomplete(),
                        }
                    }
                }
            }
        }
    }

    /// MethodCallHandler/StaticCallHandler: the method's createFromVariant() point, or `call to unknown method`.
    fn method_call(
        &mut self,
        e: &Expression,
        on: &Expression,
        selector: &ClassLikeMemberSelector,
        args: &ArgumentList,
        r#static: bool,
    ) {
        self.throws = true;
        let ClassLikeMemberSelector::Identifier(name) = selector else {
            self.arguments(args, &[]);
            self.incomplete();
            return;
        };
        let Some((classes, intersection)) = self.receiver(on, r#static) else {
            self.arguments(args, &[]);
            self.incomplete();
            return;
        };
        let mut methods = Vec::new();
        for k in &classes {
            match members::method(self.c, k, name.value) {
                Some(m) => methods.push(m),
                None if intersection => {}
                None => {
                    self.arguments(args, &[]);
                    let magic = members::method(self.c, k, b"__call").is_some()
                        || members::method(self.c, k, b"__callStatic").is_some();
                    if classes.len() == 1 && !magic {
                        self.push(e.span(), "methodCall", "call to unknown method", false);
                    } else {
                        self.incomplete();
                    }
                    return;
                }
            }
        }
        // IntersectionTypeMethodReflection of several methods is not ported
        if methods.is_empty() || (intersection && methods.len() > 1) {
            self.arguments(args, &[]);
            self.incomplete();
            return;
        }
        let params = match &methods[0] {
            Method::User { f, .. } => self.user_params(f),
            Method::Builtin { .. } => Vec::new(),
        };
        // a method's parameters are not immediately invoked unless tagged so
        let params: Vec<Param> = params
            .into_iter()
            .map(|p| Param {
                callable: false,
                ..p
            })
            .collect();
        self.arguments(args, &params);
        // MethodThrowPointHelper: `@throws void` has no throw point
        self.throws = !methods
            .iter()
            .all(|m| matches!(m, Method::User { f, .. } if throws_void(f)));
        let tris: Vec<(Tri, Tri)> = methods.iter().map(|m| self.method_tris(m)).collect();
        if identity(tris.iter().map(|t| t.0)) == Tri::No {
            return;
        }
        let mut certain =
            identity(tris.iter().map(|t| t.1)) == Tri::No || methods.iter().all(method_void);
        if !certain {
            let (callable, passed) = self.method_cond(&methods);
            match self.verdict(&params, &callable, &passed, Some(args)) {
                Some(Tri::Yes) => return,
                Some(Tri::No) => certain = true,
                _ => {}
            }
        }
        let description = format!(
            "call to method {}::{}()",
            methods[0].declaring_display(self.c),
            methods[0].name()
        );
        self.push(e.span(), "methodCall", description, certain);
    }

    /// The classes a call is made on, and whether they are an intersection.
    fn receiver(
        &self,
        on: &Expression,
        r#static: bool,
    ) -> Option<(Vec<&'a ClassLikeMetadata>, bool)> {
        match unparen(on) {
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => {
                Some((vec![self.c.class_scope()?], false))
            }
            Expression::Self_(_) | Expression::Static(_) => {
                Some((vec![self.c.class_scope()?], false))
            }
            Expression::Parent(_) => Some((
                vec![
                    self.c
                        .class_scope()?
                        .direct_parent_class
                        .and_then(|p| self.c.codebase.get_class_like(p.as_bytes()))?,
                ],
                false,
            )),
            Expression::Identifier(id) => Some((
                vec![self.c.class_like(&self.c.resolved(id)).found()?],
                false,
            )),
            e => {
                let t = self.c.phpstan_type(e)?;
                let t = if r#static {
                    match t {
                        Type::ClassString(Some(inner)) => *inner,
                        t => t,
                    }
                } else {
                    t.remove_null()
                };
                if t.is_object() != Tri::Yes {
                    return None;
                }
                let intersection = matches!(t, Type::Intersection(_));
                let classes = t
                    .object_class_names()
                    .iter()
                    .map(|n| self.c.codebase.get_class_like(n.as_bytes()))
                    .collect::<Option<Vec<_>>>()?;
                Some((classes, intersection))
            }
        }
    }

    /// (hasSideEffects(), isPure()) of a method's reflection.
    fn method_tris(&self, m: &Method) -> (Tri, Tri) {
        match m {
            Method::User { declaring, f, .. } => {
                let name = f.name.as_bytes();
                // the BackedEnum methods (resources/functionMetadata.php)
                if declaring.kind.is_enum()
                    && (name.eq_ignore_ascii_case(b"from") || name.eq_ignore_ascii_case(b"tryFrom"))
                {
                    return (Tri::No, Tri::Yes);
                }
                let constructor = name.eq_ignore_ascii_case(b"__construct");
                let (pure, _, _) = declared::method_purity(self.c, declaring, name);
                let this = f.return_type_metadata.as_ref().is_some_and(|r| {
                    matches!(r.type_union.types.as_ref(), [TAtomic::Object(TObject::Named(n))] if n.is_this)
                });
                let effects = if !constructor && method_void(m) {
                    Tri::Yes
                } else if let Some(pure) = pure {
                    Tri::from(!pure)
                } else if this {
                    Tri::Yes
                } else {
                    Tri::Maybe
                };
                (effects, pure.map_or(Tri::Maybe, Tri::from))
            }
            Method::Builtin { f, .. } => {
                let effects = match f.side_effects.as_str() {
                    _ if method_void(m) => Tri::Yes,
                    "No" => Tri::No,
                    "Yes" => Tri::Yes,
                    _ => Tri::Maybe,
                };
                (
                    effects,
                    if effects == Tri::Yes {
                        Tri::No
                    } else {
                        effects.negate()
                    },
                )
            }
        }
    }

    /// SimpleImpurePoint::createFromVariant() of a method without a verdict: isCertain() and the description,
    /// `None` when hasSideEffects() is no.
    fn method_effects(&self, m: &Method) -> Option<(bool, String)> {
        let (effects, pure) = self.method_tris(m);
        if effects == Tri::No {
            return None;
        }
        let description = format!(
            "call to method {}::{}()",
            m.declaring_display(self.c),
            m.name()
        );
        Some((pure == Tri::No || method_void(m), description))
    }

    /// NewHandler: the constructor's point (with its pure-unless verdict), or an unknown class's.
    fn instantiation(&mut self, e: &Expression, new: &Instantiation) {
        self.throws = true;
        let class = match unparen(new.class) {
            Expression::Identifier(id) => match self.c.class_like(&self.c.resolved(id)).found() {
                Some(k) => Some(k),
                None => {
                    self.new_arguments(new, &[]);
                    if matches!(
                        self.c.class_like(&self.c.resolved(id)),
                        super::reflection::Lookup::NotFound
                    ) {
                        self.push(e.span(), "new", "instantiation of unknown class", false);
                    } else {
                        self.incomplete();
                    }
                    return;
                }
            },
            Expression::Self_(_)
            | Expression::Static(_)
            | Expression::Parent(_)
            | Expression::AnonymousClass(_) => {
                self.incomplete();
                return;
            }
            class => {
                self.expr(class);
                match self.c.phpstan_type(class) {
                    Some(Type::String) => {
                        self.new_arguments(new, &[]);
                        self.push(e.span(), "new", "instantiation of unknown class", false);
                        return;
                    }
                    _ => {
                        self.incomplete();
                        None
                    }
                }
            }
        };
        let Some(k) = class else {
            self.new_arguments(new, &[]);
            return;
        };
        let Some(constructor) = members::method(self.c, k, b"__construct") else {
            self.new_arguments(new, &[]);
            return;
        };
        let Method::User { declaring, f, .. } = &constructor else {
            self.new_arguments(new, &[]);
            if constructor_effects(&constructor) != Some(Tri::No) {
                self.incomplete();
            }
            return;
        };
        let params: Vec<Param> = self
            .user_params(f)
            .into_iter()
            .map(|p| Param {
                callable: false,
                ..p
            })
            .collect();
        self.new_arguments(new, &params);
        let (pure, _, _) = declared::method_purity(self.c, declaring, b"__construct");
        if pure == Some(true) {
            return;
        }
        let mut certain = pure == Some(false);
        let (callable, passed) = self.method_cond(std::slice::from_ref(&constructor));
        match self.verdict(&params, &callable, &passed, new.argument_list.as_ref()) {
            Some(Tri::Yes) => return,
            Some(Tri::No) => certain = true,
            _ => {}
        }
        self.push(
            e.span(),
            "new",
            format!("instantiation of class {}", self.c.display(declaring)),
            certain,
        );
    }

    fn new_arguments(&mut self, new: &Instantiation, params: &[Param]) {
        if let Some(args) = &new.argument_list {
            self.arguments(args, params);
        }
    }
}

/// A built-in constructor's hasSideEffects().
fn constructor_effects(m: &Method) -> Option<Tri> {
    match m {
        Method::Builtin { f, .. } => Some(match f.side_effects.as_str() {
            "No" => Tri::No,
            "Yes" => Tri::Yes,
            _ => Tri::Maybe,
        }),
        Method::User { .. } => None,
    }
}

/// A method whose return type is void (a built-in constructor's is not).
fn method_void(m: &Method) -> bool {
    match m {
        Method::User { f, .. } => f
            .return_type_metadata
            .as_ref()
            .is_some_and(|r| r.type_union.is_void()),
        Method::Builtin { f, .. } => {
            !f.name.eq_ignore_ascii_case("__construct")
                && f.variants.first().is_some_and(|v| v.ret == "void")
        }
    }
}

/// `@throws void`: no implicit throw point.
fn throws_void(f: &FunctionLikeMetadata) -> bool {
    !f.thrown_types.is_empty()
        && f.thrown_types
            .iter()
            .all(|t| t.from_docblock && t.type_union.is_void())
}

fn callable_atom(a: &TAtomic) -> bool {
    matches!(a, TAtomic::Callable(_))
        || matches!(a, TAtomic::Object(TObject::Named(n)) if n.name.as_bytes().eq_ignore_ascii_case(b"Closure"))
}

/// SimpleImpurePoint::matchArgForParameter(): the argument a parameter receives, and whether an unpacked argument
/// might supply it.
fn match_arg<'x, 'arena>(
    args: Option<&'x ArgumentList<'arena>>,
    params: &[Param],
    index: usize,
) -> (Option<&'x Argument<'arena>>, bool) {
    let param = &params[index];
    let Some(args) = args else {
        return (None, false);
    };
    let mut unpacked = false;
    let mut named = false;
    for (i, arg) in args.arguments.iter().enumerate() {
        match arg {
            Argument::Positional(p) if p.ellipsis.is_some() => unpacked = true,
            Argument::Named(n) => {
                named = true;
                if n.name.value == param.name.as_bytes() {
                    return (Some(arg), unpacked);
                }
                if param.variadic && !params.iter().any(|p| p.name.as_bytes() == n.name.value) {
                    return (Some(arg), unpacked);
                }
            }
            Argument::Positional(_) => {
                if !named && (i == index || (param.variadic && i > index)) {
                    return (Some(arg), unpacked);
                }
            }
        }
    }
    (None, unpacked)
}
