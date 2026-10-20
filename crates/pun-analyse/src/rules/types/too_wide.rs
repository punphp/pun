//! Level 4's TooWideFunctionReturnTypehintRule, TooWideMethodReturnTypehintRule, TooWideClosureReturnTypehintRule
//! and TooWideArrowFunctionReturnTypehintRule over TooWideTypeCheck (`return.unusedType`), with the default
//! `reportTooWideBool` and `reportNestedTooWideType` (both off) and `checkTooWideReturnTypesInProtectedAndPublicMethods`
//! (off), and TooWidePropertyTypeRule (`property.unusedType`).

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Type, is_super_type_of, mixed, recommended_level, union};

use super::super::assign::{Callee, by_ref_arguments, by_ref_call_arguments};
use super::super::declarations::has_tag;
use super::super::{Checker, declared};
use super::flow::{list, unparen};
use super::missing_return::{declaring, late, method_name};
use super::returns::returns;

pub const IDENTIFIERS: &[&str] = &["return.unusedType", "property.unusedType"];

/// `isConstantScalarValue()->yes()`.
fn constant_scalar(t: &Type) -> bool {
    match t {
        Type::ConstInt(_) | Type::ConstFloat(_) | Type::ConstString(_) | Type::ConstBool(_) | Type::Null => true,
        Type::Union(ts) => ts.iter().all(constant_scalar),
        _ => false,
    }
}

/// `TypeUtils::resolveLateResolvableTypes()`: a conditional type is the branch its subject takes.
fn resolved(t: &Type) -> Type {
    match t {
        Type::Conditional(cd) => resolved(&cd.resolve()),
        t => t.traverse(&resolved),
    }
}

/// The members of a declared union `narrowType()` drops, as `createErrors()` describes them.
fn unused(declared: &Type, actual: &Type) -> Vec<String> {
    let Type::Union(members) = declared else { return Vec::new() };
    if constant_scalar(declared) && constant_scalar(actual) {
        return Vec::new();
    }
    // (`checkFunctionReturnType()` narrows with `$checkDescendantClass` false)
    let used = members.iter().filter(|m| !is_super_type_of(m, actual).no()).cloned().collect();
    let narrowed = union(used);
    members
        .iter()
        .filter(|m| is_super_type_of(&narrowed, m).no())
        .map(|m| m.describe(recommended_level(m, None)))
        .collect()
}

/// `checkFunctionReturnType()` of a body (`native` is whether it has a native return type).
fn body(c: &mut Checker, b: &Block, parent: Span, native: bool, ret: &Type, of: &str, descendant: bool) {
    let r = returns(b);
    if r.generator || r.returns.is_empty() {
        return;
    }
    let mut types = Vec::new();
    for s in &r.returns {
        match s.value {
            None => types.push(Type::Void),
            Some(v) => {
                let Some(t) = c.phpstan_type(v) else { return };
                types.push(t);
            }
        }
    }
    let results = super::flow::Results { types: &c.types, contents: &c.file.contents, codebase: Some(c.codebase) };
    let (flow, ends) = results.execution_ends(&list(b.statements.as_slice()), parent, Some(b.right_brace));
    if !flow.always {
        types.push(Type::Void);
    }
    if !native && ends.iter().any(|e| !e.flow.always) {
        types.push(Type::Null);
    }
    let actual = union(types);
    if let [only] = r.returns.as_slice()
        && only.value.is_some_and(|v| matches!(v, Expression::Literal(Literal::True(_) | Literal::False(_))))
        && ret.is_boolean().yes()
    {
        return;
    }
    if descendant && (actual.is_null().yes() || matches!(actual, Type::ConstBool(_))) {
        return;
    }
    for t in declared::with_classes(c, || unused(ret, &actual)) {
        c.report(
            parent,
            "return.unusedType",
            format!("{of} never returns {t} so it can be removed from the return type."),
            None,
        );
    }
}

pub(in super::super) fn function(c: &mut Checker, f: &Function) {
    if c.level < 4 {
        return;
    }
    let name = String::from_utf8_lossy(c.names.get(&f.name.span())).into_owned();
    let native = f.return_type_hint.is_some();
    let ret = declared::with_classes(c, || late(c, &resolved(&declared::function(c, f).ret), None, !native));
    body(c, &f.body, f.span(), native, &ret, &format!("Function {name}()"), false);
}

pub(in super::super) fn method(c: &mut Checker, m: &Method) {
    if c.level < 4 || c.in_trait().is_some() {
        return;
    }
    let MethodBody::Concrete(b) = &m.body else {
        return;
    };
    let (Some(context), Some(declaring)) = (c.class_scope(), declaring(c, m.span())) else { return };
    let private = m.modifiers.contains_private();
    let anonymous = context.original_name.as_bytes().starts_with(b"{anonymous-class:");
    let finals = context.flags.is_final()
        || anonymous
        || m.modifiers.contains_final()
        || has_tag(&c.tags_before(m), &["@final"]);
    if !private && !finals {
        return;
    }
    let name = method_name(c, m);
    let first = context
        .overridden_method_ids
        .get(&mago_word::word(name.to_ascii_lowercase().as_bytes()))
        .is_none_or(|o| o.is_empty());
    let native = m.return_type_hint.is_some();
    let ret = declared::with_classes(c, || {
        late(c, &resolved(&declared::method(c, m, declaring, context).ret), Some(context), !native)
    });
    let of = format!("Method {}::{name}()", c.display(context));
    body(c, b, m.span(), native, &ret, &of, !first && !private);
}

/// `checkAnonymousFunction()`.
fn anonymous(c: &mut Checker, at: Span, actual: &Type, ret: &Type) {
    let Type::Union(members) = ret else { return };
    if actual.is_null().yes() {
        return;
    }
    for m in members {
        if is_super_type_of(m, actual).no() {
            let t = m.describe(recommended_level(m, None));
            c.report(
                at,
                "return.unusedType",
                format!("Anonymous function never returns {t} so it can be removed from the return type."),
                None,
            );
        }
    }
}

fn native(c: &Checker, hint: &FunctionLikeReturnTypeHint) -> Type {
    let class = c.class_scope();
    declared::with_classes(c, || late(c, &declared::native(c, &hint.hint, class), class, false))
}

pub(in super::super) fn closure(c: &mut Checker, f: &Closure) {
    let (true, Some(hint)) = (c.level >= 4, &f.return_type_hint) else { return };
    let r = returns(&f.body);
    if r.generator {
        return;
    }
    let mut types = Vec::new();
    for v in r.returns.iter().filter_map(|s| s.value) {
        let Some(t) = c.phpstan_type(v) else { return };
        types.push(t);
    }
    if types.is_empty() {
        return;
    }
    let ret = native(c, hint);
    anonymous(c, f.span(), &union(types), &ret);
}

pub(in super::super) fn arrow_function(c: &mut Checker, f: &ArrowFunction) {
    let (true, Some(hint)) = (c.level >= 4, &f.return_type_hint) else { return };
    if matches!(unparen(f.expression), Expression::Yield(_)) {
        return;
    }
    let Some(actual) = c.phpstan_type(f.expression) else { return };
    let ret = native(c, hint);
    anonymous(c, f.span(), &actual, &ret);
}

/// The writes of a class's members: assignments, `++`/`--` operands, foreach targets and calls (whose by-reference
/// arguments are written their parameters' out types, not read here).
#[derive(Default)]
struct Writes<'ast, 'arena> {
    assignments: Vec<&'ast Assignment<'arena>>,
    others: Vec<&'ast Expression<'arena>>,
    calls: Vec<Callee<'ast, 'arena>>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Writes<'ast, 'arena> {
    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        self.assignments.push(a);
    }
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        if u.operator.is_increment_or_decrement() {
            self.others.push(u.operand);
        }
    }
    fn walk_in_unary_postfix(&mut self, u: &'ast UnaryPostfix<'arena>, _: &mut ()) {
        self.others.push(u.operand);
    }
    fn walk_in_function_call(&mut self, f: &'ast FunctionCall<'arena>, _: &mut ()) {
        self.calls.push(Callee::Function(f));
    }
    fn walk_in_method_call(&mut self, m: &'ast MethodCall<'arena>, _: &mut ()) {
        self.calls.push(Callee::Method(m));
    }
    fn walk_in_static_method_call(&mut self, m: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.calls.push(Callee::StaticMethod(m));
    }
    fn walk_in_instantiation(&mut self, n: &'ast Instantiation<'arena>, _: &mut ()) {
        self.calls.push(Callee::New(n));
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        match &f.target {
            ForeachTarget::Value(v) => self.others.push(v.value),
            ForeachTarget::KeyValue(kv) => self.others.extend([kv.key, kv.value]),
        }
    }
    fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {}
}

/// What an assignment writes to a property.
enum Written {
    Value(Type),
    /// an offset of it
    Offset,
    Unknown,
}

fn without_null(t: &Type) -> Type {
    match t {
        Type::Union(ts) => union(ts.iter().filter(|t| !t.is_null().yes()).cloned().collect()),
        t => t.clone(),
    }
}

/// TooWidePropertyTypeRule on class `k` declared with `members`: its private properties (not promoted ones) against
/// their default and the values its members assign them.
// ponytail: the assignments in the traits the class uses are not read, nor are those of a list element, `++`/`--`, a
// foreach target or a by-reference argument, whose properties are skipped
pub(in super::super) fn properties(c: &mut Checker, k: &ClassLikeMetadata, members: &[ClassLikeMember]) {
    if c.level < 4 || c.in_trait().is_some() {
        return;
    }
    let mut w = Writes::default();
    for m in members {
        w.walk_class_like_member(m, &mut ());
    }
    // the properties of `k` by name: their assigned types, `None` when one is unknown
    let mut assigned: Vec<(String, Written)> = Vec::new();
    let mut write = |name: String, t: Written| assigned.push((name, t));
    let own = |declaring: &ClassLikeMetadata| declaring.name == k.name;
    for a in &w.assignments {
        // `= &$x`: the referenced side's type
        let rhs = match a.rhs {
            Expression::UnaryPrefix(u) if u.operator.is_reference() => u.operand,
            rhs => rhs,
        };
        for found in super::properties::written(c, a.lhs) {
            let Some((declaring, name, direct, offset)) = found else { return };
            if !own(declaring) {
                continue;
            }
            let t = match (&a.operator, direct) {
                _ if offset => Written::Offset,
                (AssignmentOperator::Assign(_), true) => c.phpstan_type(rhs).map_or(Written::Unknown, Written::Value),
                (_, true) => c.types.get(&a.span()).map_or(Written::Unknown, |t| Written::Value(c.to_phpstan(t))),
                (_, false) => Written::Unknown,
            };
            write(name, t);
        }
    }
    for call in &w.calls {
        let (refs, args) = match *call {
            Callee::Function(f) => (by_ref_arguments(c, f), &f.argument_list),
            Callee::Method(m) => (by_ref_call_arguments(c, Callee::Method(m), &m.argument_list), &m.argument_list),
            Callee::StaticMethod(m) => {
                (by_ref_call_arguments(c, Callee::StaticMethod(m), &m.argument_list), &m.argument_list)
            }
            Callee::New(n) => {
                let Some(args) = &n.argument_list else { continue };
                (by_ref_call_arguments(c, Callee::New(n), args), args)
            }
        };
        w.others.extend(args.arguments.iter().zip(refs).filter(|(_, r)| *r).map(|(a, _)| a.value()));
    }
    for e in &w.others {
        for (declaring, name, ..) in super::properties::written(c, e).into_iter().flatten() {
            if own(declaring) {
                write(name, Written::Unknown);
            }
        }
    }
    let display = c.display(k);
    for m in members {
        let ClassLikeMember::Property(p) = m else { continue };
        if !p.modifiers().contains_private() {
            continue;
        }
        let static_ = p.modifiers().contains_static();
        let items: Vec<&PropertyItem> = match p {
            Property::Plain(p) => p.items.iter().collect(),
            Property::Hooked(h) => vec![&h.item],
        };
        for item in items {
            let name = &item.variable().name[1..];
            let text = String::from_utf8_lossy(name).into_owned();
            let mut types = Vec::new();
            let mut known = true;
            let ret = declared::with_classes(c, || {
                late(c, &resolved(&declared::property(c, p.hint(), p.span(), name, k)), Some(k), true)
            });
            for (_, t) in assigned.iter().filter(|(n, _)| *n == text) {
                match t {
                    Written::Value(t) => types.push(t.clone()),
                    // ponytail: an offset write gives an array property its array type (`setOffsetValueType()` of
                    // the type before), other properties are skipped
                    Written::Offset if is_super_type_of(&Type::array(mixed(), mixed()), &without_null(&ret)).yes() => {
                        types.push(Type::array(mixed(), mixed()));
                    }
                    _ => known = false,
                }
            }
            if let PropertyItem::Concrete(d) = item {
                // (a default value's type is always asked for)
                types.extend(c.phpstan_type(d.value));
            } else if p.hint().is_none() {
                types.push(Type::Null);
            }
            if !known || types.is_empty() {
                continue;
            }
            let actual = union(types);
            let described = declared::with_classes(c, || ret.describe(recommended_level(&ret, None)));
            let kind = if static_ { "Static property" } else { "Property" };
            for t in declared::with_classes(c, || unused(&ret, &actual)) {
                c.report(
                    p.span(),
                    "property.unusedType",
                    format!("{kind} {display}::${text} ({described}) is never assigned {t} so it can be removed from the property type."),
                    None,
                );
            }
        }
    }
}
