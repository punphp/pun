//! Assignment checks: InvalidAssignVarRule with NullsafeCheck, InvalidVariableAssignRule (the VariableAssignNodes
//! AssignHandler::processAssignVar() emits) and OffsetAccessWithoutDimForReadingRule (with the expressions
//! `$scope->isInExpressionAssign()` holds for), and PropertyAssignRefRule.

use std::collections::{HashMap, HashSet};

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::visibility::Visibility;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::symbols::ByRef;

use super::types::flow::unparen;
use super::{Checker, members};
use crate::bitset::BitSet;

/// What these checks remember during a walk.
#[derive(Default)]
pub(super) struct State {
    /// `[]` fetches PHPStan visits inside `enterExpressionAssign()` for them.
    assigned: HashSet<Span>,
    /// Subtrees PHPStan only assigns virtually (VirtualAssignNodeCallback: no rule sees their nodes).
    virtual_targets: Vec<Span>,
    /// MutatingScope's currentlyAssignedExpressions, keyed by expression identity, not source occurrence.
    pub(super) expressions: HashSet<String>,
    saved: Vec<HashSet<String>>,
    enter: HashMap<Span, String>,
    exit: HashMap<Span, String>,
}

impl State {
    pub(super) fn enter_scope(&mut self) {
        self.saved.push(std::mem::take(&mut self.expressions));
    }

    pub(super) fn leave_scope(&mut self) {
        self.expressions = self.saved.pop().expect("entered assignment scope");
    }
}

/// Enter/exitExpressionAssign creates a scope with a replaced entry; exit removes it, not an outer count.
pub(super) fn enter_expression(c: &mut Checker, e: &Expression) {
    if let Some(key) = c.assign.enter.get(&e.span()) {
        c.assign.expressions.insert(key.clone());
    }
}

pub(super) fn exit_expression(c: &mut Checker, e: &Expression) {
    if let Some(key) = c.assign.exit.get(&e.span()) {
        c.assign.expressions.remove(key);
    }
}

fn expression_scope(c: &mut Checker, e: &Expression) {
    if let Some(key) = c.assignment_key(e) {
        c.assign.enter.insert(e.span(), key.clone());
        c.assign.exit.insert(e.span(), key);
    }
}

/// PHPParser converts a root short-array target, then recursively converts both Array_ kinds in its values.
/// Explicit list() is already a List_, but its children are not passed through fixupArrayDestructuring.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ListRole {
    Read,
    Target,
    ArrayItem,
}

/// A List_ item's key, value (without ArrayItem::$byRef), and the parser role of its value.
type Item<'e, 'arena> = (
    Option<&'e Expression<'arena>>,
    &'e Expression<'arena>,
    ListRole,
);

fn list_items<'e, 'arena>(
    e: &'e Expression<'arena>,
    role: ListRole,
) -> Option<Vec<Item<'e, 'arena>>> {
    let (elements, child_role) = match e {
        Expression::List(l) => (&l.elements, ListRole::Read),
        Expression::Array(a) if role != ListRole::Read => (&a.elements, ListRole::ArrayItem),
        Expression::LegacyArray(a) if role == ListRole::ArrayItem => {
            (&a.elements, ListRole::ArrayItem)
        }
        _ => return None,
    };
    Some(
        elements
            .iter()
            .filter_map(|item| {
                let (key, value) = match item {
                    ArrayElement::KeyValue(kv) => (Some(kv.key), kv.value),
                    ArrayElement::Value(ValueArrayElement { value })
                    | ArrayElement::Variadic(VariadicArrayElement { value, .. }) => (None, *value),
                    ArrayElement::Missing(_) => return None,
                };
                // php.y's ampersand variable array_pair stores byRef on ArrayItem, not on its value.
                // Do not unwrap references in arbitrary expressions or unwrap parentheses around one.
                let value = match value {
                    Expression::UnaryPrefix(reference) if reference.operator.is_reference() => {
                        reference.operand
                    }
                    value => value,
                };
                Some((key, value, child_role))
            })
            .collect(),
    )
}

/// NullsafeCheck::containsNullSafe() on an expression read (a returned or referenced expression).
pub(super) fn contains_nullsafe(e: &Expression) -> bool {
    nullsafe(e, ListRole::Read)
}

/// NullsafeCheck::containsNullSafe() on the PHPParser expression projected by [`list_items()`].
fn nullsafe(e: &Expression, role: ListRole) -> bool {
    let e = unparen(e);
    match e {
        Expression::Access(Access::NullSafeProperty(_))
        | Expression::Call(Call::NullSafeMethod(_)) => true,
        Expression::ArrayAccess(a) => contains_nullsafe(a.array),
        Expression::ArrayAppend(a) => contains_nullsafe(a.array),
        Expression::Access(Access::Property(p)) => contains_nullsafe(p.object),
        Expression::Access(Access::StaticProperty(p)) => {
            is_expr_class(p.class) && contains_nullsafe(p.class)
        }
        Expression::Call(Call::Method(m)) => contains_nullsafe(m.object),
        Expression::Call(Call::StaticMethod(m)) => {
            is_expr_class(m.class) && contains_nullsafe(m.class)
        }
        _ => list_items(e, role).is_some_and(|items| {
            items.into_iter().any(|(key, value, role)| {
                key.is_some_and(contains_nullsafe) || nullsafe(value, role)
            })
        }),
    }
}

/// `$expr->class instanceof Expr`: not a name (php-parser has `self`, `static` and `parent` as names).
fn is_expr_class(class: &Expression) -> bool {
    !matches!(
        class,
        Expression::Identifier(_)
            | Expression::Self_(_)
            | Expression::Static(_)
            | Expression::Parent(_)
    )
}

/// InvalidAssignVarRule's containsNonAssignableExpression().
fn non_assignable(e: &Expression, role: ListRole) -> bool {
    let e = unparen(e);
    match e {
        Expression::Variable(_)
        | Expression::Access(Access::Property(_) | Access::StaticProperty(_))
        | Expression::ArrayAccess(_)
        | Expression::ArrayAppend(_) => false,
        _ => list_items(e, role).is_none_or(|items| {
            items
                .into_iter()
                .any(|(_, value, role)| non_assignable(value, role))
        }),
    }
}

/// InvalidAssignVarRule on `var = expr` (`by_ref`: `=&`, with `expr` the referenced expression).
fn invalid_assign_var(c: &mut Checker, at: Span, var: &Expression, expr: Option<&Expression>) {
    if nullsafe(var, ListRole::Target) {
        c.report_non_ignorable(
            at,
            "nullsafe.assign",
            "Nullsafe operator cannot be on left side of assignment.".to_string(),
            None,
        );
    } else if expr.is_some_and(contains_nullsafe) {
        c.report_non_ignorable(
            at,
            "nullsafe.byRef",
            "Nullsafe operator cannot be on right side of assignment by reference.".to_string(),
            None,
        );
    } else if non_assignable(var, ListRole::Target) {
        c.report_non_ignorable(
            at,
            "assign.invalidExpr",
            "Expression on left side of assignment is not assignable.".to_string(),
            None,
        );
    }
}

/// An `Assign`, `AssignOp` or `AssignRef`.
pub(super) fn assignment(c: &mut Checker, a: &Assignment) {
    let referenced = match (&a.operator, unparen(a.rhs)) {
        (AssignmentOperator::Assign(_), Expression::UnaryPrefix(u))
            if u.operator.is_reference() =>
        {
            Some(u.operand)
        }
        _ => None,
    };
    invalid_assign_var(c, a.span(), a.lhs, referenced);
    if matches!(
        a.operator,
        AssignmentOperator::Assign(_) | AssignmentOperator::Coalesce(_)
    ) {
        mark_assigned(c, a.lhs, ListRole::Target);
        if let Some(r) = referenced {
            c.assign.assigned.insert(unparen(r).span());
            expression_scope(c, unparen(r));
        }
    }
    assign_var(c, a.lhs, ListRole::Target);
}

pub(super) fn is_list_target(e: &Expression) -> bool {
    list_items(unparen(e), ListRole::Target).is_some()
}

/// processAssignVar() with `$enterExpressionAssign`: the `[]` fetches of the target's dim-fetch chains.
fn mark_assigned(c: &mut Checker, var: &Expression, role: ListRole) {
    let mut var = unparen(var);
    if let Some(items) = list_items(var, role) {
        for (key, value, role) in items {
            // applyWrite(KIND_LIST) enters the value on itemScope. Only a written key's ExpressionResult
            // transfers that scope into the running result; an unkeyed item keeps the original scope.
            if let Some(key) = key
                && let Some(identity) = c.assignment_key(value)
            {
                c.assign.enter.insert(key.span(), identity);
            }
            mark_assigned(c, value, role);
        }
        return;
    }
    loop {
        match var {
            Expression::ArrayAppend(a) => {
                c.assign.assigned.insert(var.span());
                var = unparen(a.array);
            }
            Expression::ArrayAccess(a) => {
                // prepareTarget exits each written dimension after its result. The attempted enter there
                // does not replace the running scope, unlike the root's enter/exit pair.
                if let Some(key) = c.assignment_key(a.index) {
                    c.assign.exit.insert(a.index.span(), key);
                }
                var = unparen(a.array);
            }
            _ => {
                expression_scope(c, var);
                return;
            }
        }
    }
}

/// The VariableAssignNodes processAssignVar() emits for `var`: a variable, the variable a dim-fetch chain starts
/// at, and those of a list's items.
fn assign_var(c: &mut Checker, var: &Expression, role: ListRole) {
    let mut var = unparen(var);
    if let Some(items) = list_items(var, role) {
        for (_, value, role) in items {
            assign_var(c, value, role);
        }
        return;
    }
    while let Expression::ArrayAccess(ArrayAccess { array, .. })
    | Expression::ArrayAppend(ArrayAppend { array, .. }) = var
    {
        var = unparen(array);
    }
    if let Expression::Variable(Variable::Direct(v)) = var {
        variable_assign(c, v);
    }
}

/// InvalidVariableAssignRule.
pub(super) fn variable_assign(c: &mut Checker, v: &DirectVariable) {
    super::by_ref_foreach::variable_assign(c, v);
    if v.name != b"$this" {
        return;
    }
    if !c.classes.is_empty()
        && c.class_scope().is_some_and(|k| {
            k.all_parent_interfaces
                .iter()
                .any(|i| i.as_bytes().eq_ignore_ascii_case(b"ArrayAccess"))
        })
    {
        return;
    }
    c.report_non_ignorable(
        v,
        "assign.this",
        "Cannot re-assign $this.".to_string(),
        None,
    );
}

/// `foreach`: a variable key or value is a VariableAssignNode; any other target is assigned virtually.
pub(super) fn foreach(c: &mut Checker, f: &Foreach) {
    let (key, value) = match &f.target {
        ForeachTarget::Value(t) => (None, t.value),
        ForeachTarget::KeyValue(t) => (Some(t.key), t.value),
    };
    let value = match unparen(value) {
        Expression::UnaryPrefix(u) if u.operator.is_reference() => u.operand,
        v => v,
    };
    if list_items(unparen(value), ListRole::Target).is_some() {
        invalid_assign_var(c, value.span(), value, None);
    }
    for target in key.into_iter().chain([value]) {
        c.assign.virtual_targets.push(target.span());
        assign_var(c, target, ListRole::Target);
    }
}

/// `catch (E $var)`.
pub(super) fn catch(c: &mut Checker, clause: &TryCatchClause) {
    if let Some(v) = &clause.variable {
        variable_assign(c, v);
    }
}

/// `++`/`--` assign their operand virtually.
pub(super) fn increment(c: &mut Checker, operand: &Expression) {
    assign_var(c, operand, ListRole::Read);
}

/// `unset($a[$k])` assigns its dim-fetch chain virtually.
pub(super) fn unset(c: &mut Checker, u: &Unset) {
    for value in &u.values {
        if let Expression::ArrayAccess(_) = unparen(value) {
            assign_var(c, value, ListRole::Read);
        }
    }
}

/// OffsetAccessWithoutDimForReadingRule.
pub(super) fn array_append(c: &mut Checker, a: &ArrayAppend) {
    let span = a.span();
    if c.assign.assigned.contains(&span)
        || c.assignment_key(&Expression::ArrayAppend(a.clone()))
            .is_some_and(|key| c.assign.expressions.contains(&key))
        || c.assign
            .virtual_targets
            .iter()
            .any(|t| t.start.offset <= span.start.offset && span.end.offset <= t.end.offset)
    {
        return;
    }
    c.report_non_ignorable(
        a,
        "offsetAccess.noDim",
        "Cannot use [] for reading.".to_string(),
        None,
    );
}

/// A parameter as far as passing by reference goes.
struct Param {
    name: Vec<u8>,
    /// `passedByReference()->createsNewVariable()`
    by_ref: bool,
    variadic: bool,
}

fn of_metadata(f: &FunctionLikeMetadata) -> Vec<Param> {
    f.parameters
        .iter()
        .map(|p| Param {
            name: p.get_name().0.as_bytes()[1..].to_vec(),
            by_ref: p.flags.is_by_reference(),
            variadic: p.flags.is_variadic(),
        })
        .collect()
}

fn of_builtin(variants: &[pun_phpstan::symbols::Variant], n: usize) -> Option<Vec<Param>> {
    let accepts = |v: &&pun_phpstan::symbols::Variant| {
        v.params
            .iter()
            .filter(|p| !p.optional && !p.variadic)
            .count()
            <= n
            && (v.params.iter().any(|p| p.variadic) || n <= v.params.len())
    };
    // ponytail: PHPStan tells several accepting variants apart by the arguments' types; the first wins here
    let v = variants.iter().find(accepts).or(variants.first())?;
    Some(
        v.params
            .iter()
            .map(|p| Param {
                name: p.name.as_bytes().to_vec(),
                by_ref: p.by_ref == ByRef::CreatesNewVariable,
                variadic: p.variadic,
            })
            .collect(),
    )
}

fn of_method(m: &members::Method, n: usize) -> Option<Vec<Param>> {
    match m {
        members::Method::User { f, .. } => Some(of_metadata(f)),
        members::Method::Builtin { f, .. } => of_builtin(&f.variants, n),
    }
}

fn of_syntax(params: &FunctionLikeParameterList) -> Vec<Param> {
    params
        .parameters
        .iter()
        .map(|p| Param {
            name: p.variable.name[1..].to_vec(),
            by_ref: p.ampersand.is_some(),
            variadic: p.ellipsis.is_some(),
        })
        .collect()
}

/// A call whose arguments [`arguments()`] reads.
pub(super) enum Callee<'e, 'arena> {
    Function(&'e FunctionCall<'arena>),
    Method(&'e MethodCall<'arena>),
    StaticMethod(&'e StaticMethodCall<'arena>),
    New(&'e Instantiation<'arena>),
}

/// The callee's parameters (`$parametersAcceptor->getParameters()`), when PHPStan knows it here.
fn callee_params(c: &Checker, call: &Callee, n: usize) -> Option<Vec<Param>> {
    match call {
        Callee::Function(f) => match unparen(f.function) {
            Expression::Closure(k) => Some(of_syntax(&k.parameter_list)),
            Expression::ArrowFunction(k) => Some(of_syntax(&k.parameter_list)),
            Expression::Identifier(id) => {
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
                            return Some(of_metadata(f));
                        }
                        let f = c
                            .builtins
                            .functions
                            .get(&String::from_utf8_lossy(name).to_ascii_lowercase())?;
                        of_builtin(&f.variants, n)
                    })
            }
            _ => {
                // FuncCallHandler consumes the evaluated callee's callable acceptors at every reporting level.
                let ty = c.phpstan_type(f.function)?;
                let all = super::types::callables::acceptors(c, &ty, f.function.span(), &mut None)?;
                let counts = super::params::count(&f.argument_list);
                let acceptor = super::params::select_metadata(
                    c,
                    super::params::Variants { all, named: None },
                    &counts,
                );
                Some(
                    acceptor
                        .names
                        .into_iter()
                        .zip(acceptor.by_ref)
                        .zip(acceptor.variadics.iter())
                        .map(|((name, by_ref), variadic)| Param {
                            name: name.into_bytes(),
                            by_ref: by_ref == ByRef::CreatesNewVariable,
                            variadic,
                        })
                        .collect(),
                )
            }
        },
        Callee::Method(m) => {
            let ClassLikeMemberSelector::Identifier(name) = &m.method else {
                return None;
            };
            let class = match m.object {
                Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => {
                    c.class_scope()?
                }
                object => object_class(c, object)?,
            };
            of_method(&members::method(c, class, name.value)?, n)
        }
        Callee::StaticMethod(m) => {
            let ClassLikeMemberSelector::Identifier(name) = &m.method else {
                return None;
            };
            let class = named_class(c, m.class).or_else(|| object_class(c, m.class))?;
            of_method(&members::method(c, class, name.value)?, n)
        }
        Callee::New(new) => {
            let class = named_class(c, new.class)?;
            of_method(&members::method(c, class, b"__construct")?, n)
        }
    }
}

/// The class of an object expression's type, as Mago infers it, when it is one named object.
fn object_class<'a>(
    c: &Checker<'a, '_>,
    object: &Expression,
) -> Option<&'a mago_codex::metadata::class_like::ClassLikeMetadata> {
    let t = c.type_of(object)?;
    let [TAtomic::Object(TObject::Named(o))] = t.types.as_ref() else {
        return None;
    };
    c.class_like(&String::from_utf8_lossy(o.name.as_bytes()))
        .found()
}

fn named_class<'a>(
    c: &Checker<'a, '_>,
    class: &Expression,
) -> Option<&'a mago_codex::metadata::class_like::ClassLikeMetadata> {
    match class {
        Expression::Identifier(id) => c.class_like(&c.resolved(id)).found(),
        Expression::Self_(_) | Expression::Static(_) => c.class_scope(),
        Expression::Parent(_) => c
            .class_scope()?
            .direct_parent_class
            .and_then(|p| c.class_like(&String::from_utf8_lossy(p.as_bytes())).found()),
        _ => None,
    }
}

/// `$normalizedExpr->getArgs()[i]->value`: the argument at position `i`, or the one named for that parameter.
fn normalized_arg<'e, 'arena>(
    args: &'e ArgumentList<'arena>,
    i: usize,
    name: &[u8],
) -> Option<&'e Expression<'arena>> {
    args.arguments
        .iter()
        .enumerate()
        .find_map(|(j, a)| match a {
            Argument::Positional(p) if j == i => Some(p.value),
            Argument::Named(n) if n.name.value == name => Some(n.value),
            _ => None,
        })
}

/// FuncCallHandler's processVirtualAssign() of a built-in's array argument after processArgs() (array_walk with a
/// by-reference closure, array_pop, sort and the rest).
pub(super) fn function_virtual_assign(c: &mut Checker, call: &FunctionCall) {
    if let Some(array) = virtual_assigned(c, call) {
        // A mutated argument keeps its expression kind; PHPParser does not convert its arrays to List_.
        assign_var(c, array, ListRole::Read);
    }
}

/// The argument [`function_virtual_assign()`] assigns.
pub(super) fn virtual_assigned<'e, 'arena>(
    c: &Checker,
    call: &'e FunctionCall<'arena>,
) -> Option<&'e Expression<'arena>> {
    let Expression::Identifier(id) = call.function else {
        return None;
    };
    let fallback = match id {
        Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
        _ => None,
    };
    let (name, _) = std::iter::once(c.names.get(&id.span()))
        .chain(fallback)
        .find_map(|n| c.function(n))?;
    let args = &call.argument_list;
    let min = match name.as_str() {
        "array_walk" => {
            let by_ref_closure = matches!(
                normalized_arg(args, 1, b"callback"),
                Some(Expression::Closure(k))
                    if k.parameter_list.parameters.first().is_some_and(|p| p.ampersand.is_some())
            );
            if !by_ref_closure {
                return None;
            }
            2
        }
        "array_push" | "array_unshift" | "array_splice" => 2,
        "array_pop" | "array_shift" | "shuffle" | "sort" | "rsort" | "usort" | "natcasesort"
        | "natsort" | "arsort" | "asort" | "ksort" | "krsort" | "uasort" | "uksort" => 1,
        _ => return None,
    };
    if args.arguments.len() < min {
        return None;
    }
    normalized_arg(args, 0, b"array")
}

/// processArgs() for arguments passed by reference to a parameter that creates a new variable: a `[]` argument is
/// visited inside `enterExpressionAssign()`, and any argument but `$this` is assigned virtually.
pub(super) fn arguments(c: &mut Checker, call: Callee) {
    let args = match &call {
        Callee::Function(f) => &f.argument_list,
        Callee::Method(m) => &m.argument_list,
        Callee::StaticMethod(m) => &m.argument_list,
        Callee::New(new) => match &new.argument_list {
            Some(args) => args,
            None => return,
        },
    };
    // AssignToByRefExprFromForeachRule reads the VariableAssignNode of every variable passed by reference
    let variables = !c.by_ref_foreach.is_empty()
        && args.arguments.iter().any(|a| {
            matches!(
                unparen(a.value()),
                Expression::Variable(Variable::Direct(_))
            )
        });
    if !variables && !args.arguments.iter().any(|a| by_ref_candidate(a.value())) {
        return;
    }
    let Some(params) = callee_params(c, &call, args.arguments.len()) else {
        return;
    };
    for (i, arg) in args.arguments.iter().enumerate() {
        let param = match arg {
            Argument::Named(a) => params.iter().find(|p| p.name == a.name.value),
            Argument::Positional(_) => params
                .get(i)
                .or_else(|| params.last().filter(|p| p.variadic)),
        };
        if !param.is_some_and(|p| p.by_ref) {
            continue;
        }
        let value = unparen(arg.value());
        if let Expression::ArrayAppend(_) = value {
            c.assign.assigned.insert(value.span());
            expression_scope(c, value);
        }
        if !matches!(value, Expression::Variable(Variable::Direct(v)) if v.name == b"$this") {
            assign_var(c, value, ListRole::Read);
        }
    }
}

/// Whether each of function call `f`'s arguments is passed by reference (empty when the function is unknown).
pub(super) fn by_ref_arguments(c: &Checker, f: &FunctionCall) -> BitSet {
    let args = &f.argument_list.arguments;
    let Some(params) = callee_params(c, &Callee::Function(f), args.len()) else {
        return BitSet::default();
    };
    args.iter()
        .enumerate()
        .map(|(i, arg)| {
            let param = match arg {
                Argument::Named(a) => params.iter().find(|p| p.name == a.name.value),
                Argument::Positional(_) => params
                    .get(i)
                    .or_else(|| params.last().filter(|p| p.variadic)),
            };
            param.is_some_and(|p| p.by_ref)
        })
        .collect()
}

/// Whether each argument of a method or static call, or of `new`, is passed by reference (empty when unknown).
pub(super) fn by_ref_call_arguments(c: &Checker, call: Callee, args: &ArgumentList) -> BitSet {
    let Some(params) = callee_params(c, &call, args.arguments.len()) else {
        return BitSet::default();
    };
    args.arguments
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let param = match arg {
                Argument::Named(a) => params.iter().find(|p| p.name == a.name.value),
                Argument::Positional(_) => params
                    .get(i)
                    .or_else(|| params.last().filter(|p| p.variadic)),
            };
            param.is_some_and(|p| p.by_ref)
        })
        .collect()
}

/// An argument whose parameter decides an error: a `[]` fetch or a dim-fetch chain on `$this`.
fn by_ref_candidate(value: &Expression) -> bool {
    let value = unparen(value);
    if let Expression::ArrayAppend(_) = value {
        return true;
    }
    let mut var = value;
    while let Expression::ArrayAccess(ArrayAccess { array, .. })
    | Expression::ArrayAppend(ArrayAppend { array, .. }) = var
    {
        var = unparen(array);
    }
    !std::ptr::eq(var, value)
        && matches!(var, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
}

/// The method-call object or static-call class expression whose type [`callee_params()`] reads.
pub(super) fn typed_object<'e, 'arena>(
    object: &'e Expression<'arena>,
    args: &ArgumentList<'arena>,
) -> Option<&'e Expression<'arena>> {
    let named = matches!(object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
        || !is_expr_class(object);
    (!named && args.arguments.iter().any(|a| by_ref_candidate(a.value()))).then_some(object)
}

/// The property fetch `=&` references, whose holder's type PropertyAssignRefRule reads (`$this` aside).
pub(super) fn referenced_property<'e, 'arena>(
    a: &'e Assignment<'arena>,
) -> Option<&'e PropertyAccess<'arena>> {
    let (AssignmentOperator::Assign(_), Expression::UnaryPrefix(u)) = (&a.operator, unparen(a.rhs))
    else {
        return None;
    };
    match u.operator.is_reference().then(|| unparen(u.operand))? {
        Expression::Access(Access::Property(p)) => Some(p),
        _ => None,
    }
}

/// PropertyAssignRefRule.
pub(super) fn property_assign_ref(c: &mut Checker, a: &Assignment) {
    // PhpVersion::supportsAsymmetricVisibility()
    if c.php_version < 80400 {
        return;
    }
    let Some(fetch) = referenced_property(a) else {
        return;
    };
    // ponytail: a dynamic name (`$o->$name`) needs its constant strings
    let ClassLikeMemberSelector::Identifier(name) = &fetch.property else {
        return;
    };
    let scope = c.class_scope();
    let holder = match unparen(fetch.object) {
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => scope,
        object => object_class(c, object),
    };
    let Some(holder) = holder else {
        return;
    };
    let mut property = Vec::with_capacity(name.value.len() + 1);
    property.push(b'$');
    property.extend_from_slice(name.value);
    let (Some(p), Some(declaring)) = (
        c.codebase
            .get_declaring_property(holder.name.as_bytes(), &property),
        c.codebase
            .get_declaring_property_class(holder.name.as_bytes(), &property)
            .and_then(|d| c.class_like(&String::from_utf8_lossy(d.as_bytes())).found()),
    ) else {
        return;
    };
    if p.flags.is_static() {
        return;
    }
    let declaring_name = String::from_utf8_lossy(declaring.name.as_bytes()).into_owned();
    // `isPrivateSet()` / `isProtectedSet()`: a set visibility narrower than the read one (readonly is
    // protected(set) when public)
    let rank = |v: Visibility| match v {
        Visibility::Private => 0,
        Visibility::Protected => 1,
        Visibility::Public => 2,
    };
    let asymmetric = rank(p.write_visibility) < rank(p.read_visibility);
    let visibility = if asymmetric {
        p.write_visibility
    } else {
        p.read_visibility
    };
    // `$scope->canWriteProperty()`
    // ponytail: a closure bound with Closure::bind() & co. keeps no class scope here
    if super::calls::can_call(c, scope, &declaring_name, visibility) {
        return;
    }
    let label = match (asymmetric, visibility) {
        (true, Visibility::Private) => "private(set)",
        (true, _) => "protected(set)",
        (false, Visibility::Private) => "private",
        (false, _) => "protected",
    };
    c.report(
        a,
        "property.assignByRef",
        format!(
            "Property {}::${} with {label} visibility is assigned by reference.",
            String::from_utf8_lossy(declaring.original_name.as_bytes()),
            String::from_utf8_lossy(name.value),
        ),
        None,
    );
}
