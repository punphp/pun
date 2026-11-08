//! UnusedVariableRule (level 4, `featureToggles.unusedVariable`): local writes whose value is never read, read only
//! into values that are never used, or overwritten first, from the semantic producer's VariableWritesNode.

use std::collections::{HashMap, HashSet};

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Level, Type};

use super::super::Checker;
use super::noop;
use mago_analyzer::variable_flow::Offset;

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Assign,
    PreInc,
    PostInc,
    PreDec,
    PostDec,
    ForeachValue,
    ForeachValueWithKey,
    ForeachKey,
    Catch,
    Input,
}

/// The kind of every write target in a body, by span; the rest are assignments, list items and offset writes.
#[derive(Default)]
struct Kinds(HashMap<Span, Kind>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Kinds {
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, (): &mut ()) {
        match u.operator {
            UnaryPrefixOperator::PreIncrement(_) => self.0.insert(u.operand.span(), Kind::PreInc),
            UnaryPrefixOperator::PreDecrement(_) => self.0.insert(u.operand.span(), Kind::PreDec),
            _ => None,
        };
    }
    fn walk_in_unary_postfix(&mut self, u: &'ast UnaryPostfix<'arena>, (): &mut ()) {
        let kind = match u.operator {
            UnaryPostfixOperator::PostIncrement(_) => Kind::PostInc,
            UnaryPostfixOperator::PostDecrement(_) => Kind::PostDec,
        };
        self.0.insert(u.operand.span(), kind);
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, (): &mut ()) {
        if let Some(key) = f.target.key() {
            self.0.insert(key.span(), Kind::ForeachKey);
            self.0
                .insert(f.target.value().span(), Kind::ForeachValueWithKey);
        } else {
            self.0.insert(f.target.value().span(), Kind::ForeachValue);
        }
    }
    fn walk_in_try_catch_clause(&mut self, c: &'ast TryCatchClause<'arena>, (): &mut ()) {
        if let Some(v) = &c.variable {
            self.0.insert(v.span(), Kind::Catch);
        }
    }
    fn walk_in_function_like_parameter(
        &mut self,
        p: &'ast FunctionLikeParameter<'arena>,
        (): &mut (),
    ) {
        self.0.insert(p.variable.span(), Kind::Input);
    }
    fn walk_in_closure_use_clause_variable(
        &mut self,
        v: &'ast ClosureUseClauseVariable<'arena>,
        (): &mut (),
    ) {
        self.0.insert(v.variable.span(), Kind::Input);
    }
}

/// The write targets that are expressions (a `$a[...]` offset write prints its target), and the plain assignments
/// by target.
#[derive(Default)]
struct Targets<'ast, 'arena>(
    HashMap<Span, &'ast Expression<'arena>>,
    HashMap<Span, &'ast Assignment<'arena>>,
);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Targets<'ast, 'arena> {
    fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, (): &mut ()) {
        if matches!(e, Expression::ArrayAccess(_) | Expression::ArrayAppend(_)) {
            self.0.insert(e.span(), e);
        }
    }
    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, (): &mut ()) {
        if a.operator.is_assign() {
            self.1.insert(a.lhs.span(), a);
        }
    }
}

/// The values and offsets of plain assignments, for AssignHandler::redundant().
pub(super) fn typed(program: &Program) -> Vec<Span> {
    #[derive(Default)]
    struct Wanted(Vec<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, (): &mut ()) {
            if !a.operator.is_assign() {
                return;
            }
            self.0.push(a.rhs.span());
            let mut target = a.lhs;
            while let Expression::ArrayAccess(access) = target {
                self.0.push(access.index.span());
                target = access.array;
            }
        }
    }
    let mut wanted = Wanted::default();
    wanted.walk_program(program, &mut ());
    wanted.0
}

/// AssignHandler::redundant(): the value the target certainly already has, in both lanes.
///
/// ponytail: Mago's native lane is partial (it gives up after an `if` on a call, a loop, ...). When it has no type,
/// `constant` (every write of the variable in the body is a plain `=` of a constant expression) stands in: then the
/// native type is the full one, short of a narrowing by a PHPDoc-typed condition.
fn redundant(c: &Checker, assignment: &Assignment, constant: bool) -> Option<Type> {
    let (full, native) = c.variables.assigned_roots.get(&assignment.lhs.span())?;
    let native = native.as_ref().or(constant.then_some(full))?;
    let mut dimensions = Vec::new();
    let mut target = assignment.lhs;
    while let Expression::ArrayAccess(access) = target {
        dimensions.push(access.index);
        target = access.array;
    }
    // A literal's native type is its type; Mago's native lane types an array literal less precisely
    let native_of = |e: &Expression| {
        let literal = matches!(
            super::super::types::flow::unparen(e),
            Expression::Literal(_) | Expression::Array(_) | Expression::LegacyArray(_)
        );
        (if literal && super::haystack::constant_expression(e) {
            None
        } else {
            c.phpstan_type_in(e, &c.native_types)
        })
        .or_else(|| {
            super::haystack::constant_expression(e)
                .then(|| c.phpstan_type(e))
                .flatten()
        })
    };
    let mut ty = c.to_phpstan(full);
    let mut native_ty = c.to_phpstan(native);
    for dimension in dimensions.into_iter().rev() {
        let offset = array_key(&c.phpstan_type(dimension)?)?;
        let native_offset = array_key(&native_of(dimension)?)?;
        ty = offset_value(&ty, &offset)?;
        native_ty = offset_value(&native_ty, &native_offset)?;
    }
    let value = c.phpstan_type(assignment.rhs)?;
    let native_value = native_of(assignment.rhs)?;
    let single = |t: &Type| match super::conditions::finite(c, t).as_slice() {
        [one] => Some(one.clone()),
        _ => match t {
            Type::ConstArray(a)
                if a.optional.is_empty()
                    && a.unsealed.is_none()
                    && a.values.iter().all(|v| single_value(c, v)) =>
            {
                Some(t.clone())
            }
            _ => None,
        },
    };
    (single(&ty)? == value && single(&native_ty)? == native_value).then_some(value)
}

fn single_value(c: &Checker, t: &Type) -> bool {
    match t {
        Type::ConstArray(a) => {
            a.optional.is_empty()
                && a.unsealed.is_none()
                && a.values.iter().all(|v| single_value(c, v))
        }
        t => super::conditions::finite(c, t).len() == 1,
    }
}

/// `toArrayKey()->getConstantScalarValues()` of exactly one value.
fn array_key(t: &Type) -> Option<Type> {
    match t {
        Type::ConstInt(_) => Some(t.clone()),
        Type::ConstString(s) => {
            let text = std::str::from_utf8(s).ok()?;
            match text.parse::<i64>() {
                Ok(i) if i.to_string() == text => Some(Type::ConstInt(i)),
                _ => Some(t.clone()),
            }
        }
        Type::ConstBool(b) => Some(Type::ConstInt(i64::from(*b))),
        Type::Null => Some(Type::ConstString(Vec::new())),
        _ => None,
    }
}

/// `isArray()->yes() && hasOffsetValueType()->yes()`, then `getOffsetValueType()`, of a constant array.
fn offset_value(t: &Type, key: &Type) -> Option<Type> {
    let Type::ConstArray(a) = t else { return None };
    let i = a.keys.iter().position(|k| k == key)?;
    (!a.optional.contains(&i)).then(|| a.values[i].clone())
}

pub(in super::super) fn check(c: &mut Checker, span: Span, node: Node<'_, '_>) {
    if !c
        .toggled
        .on
        .contains(crate::rules::driver::Toggles::UNUSED_VARIABLE)
    {
        return;
    }
    let Some(writes) = c.variable_writes.get(&span) else {
        return;
    };
    if writes.opaque {
        return;
    }
    let mut kinds = Kinds::default();
    let mut targets = Targets::default();
    match node {
        Node::Function(f) => {
            kinds.walk_function(f, &mut ());
            targets.walk_function(f, &mut ());
        }
        Node::Method(m) => {
            kinds.walk_method(m, &mut ());
            targets.walk_method(m, &mut ());
        }
        Node::Closure(f) => {
            kinds.walk_closure(f, &mut ());
            targets.walk_closure(f, &mut ());
        }
        _ => return,
    }
    let with_read_write: HashSet<_> = writes
        .writes
        .values()
        .filter(|w| writes.read.contains(&w.node))
        .map(|w| w.name)
        .collect();
    let mut not_constant = HashSet::new();
    for write in writes.writes.values() {
        // the offsets an array literal writes belong to its assignment
        if write.parent.is_none()
            && !targets
                .1
                .get(&write.node)
                .is_some_and(|a| super::haystack::constant_expression(a.rhs))
        {
            not_constant.insert(write.name);
        }
    }
    let mut errors = Vec::new();
    for write in writes.writes.values() {
        let id = write.node;
        let name = &write.name.as_bytes()[1..];
        let kind = kinds.0.get(&id).copied().unwrap_or(Kind::Assign);
        if writes.escaped.contains(&write.name) || name.starts_with(b"_") || kind == Kind::Input {
            continue;
        }
        if kind == Kind::Catch && c.php_version < 80000 {
            continue;
        }
        if write.parent.is_none()
            && (write.offset_write || writes.used.contains(&id))
            && let Some(value) = targets
                .1
                .get(&id)
                .and_then(|a| redundant(c, a, !not_constant.contains(&write.name)))
        {
            let description = if write.offset_write {
                let Some(target) = targets.0.get(&id) else {
                    continue;
                };
                format!("Offset {}", noop::print(c, target))
            } else {
                format!("Variable ${}", String::from_utf8_lossy(name))
            };
            let value = super::super::declared::with_classes(c, || value.describe(Level::Value));
            errors.push((
                id,
                "assign.redundant".to_string(),
                format!("{description} is assigned value {value} but it already has that value."),
            ));
            continue;
        }
        if writes.used.contains(&id) || writes.covered.contains(&id) {
            continue;
        }
        let read = writes.read.contains(&id);
        let overwritten = writes.overwritten.contains(&id);
        if !read && matches!(kind, Kind::Catch | Kind::ForeachValueWithKey) {
            continue;
        }
        let name = String::from_utf8_lossy(name);
        let unused_variable =
            !with_read_write.contains(&write.name) && !writes.read_names.contains(&write.name);
        let (message, identifier) = if let Some(parent) = write.parent {
            let Some(parent) = writes.writes.get(&parent) else {
                continue;
            };
            let Some(offset) = &write.offset else {
                continue;
            };
            if !writes.used.contains(&parent.node) {
                continue;
            }
            let offset = match offset {
                Offset::Integer(i) => Type::ConstInt(*i),
                Offset::String(s) => Type::ConstString(s.clone()),
            }
            .describe(Level::Value);
            let (outcome, identifier) = if read {
                (
                    "only flows into values that are never used",
                    "array.unusedOffsetFlow",
                )
            } else if overwritten {
                (
                    "is never read before being overwritten",
                    "array.offsetOverwritten",
                )
            } else {
                ("is never read", "array.unusedOffset")
            };
            (
                format!("Offset {offset} of array assigned to variable ${name} {outcome}."),
                identifier.to_string(),
            )
        } else if write.offset_write {
            let Some(target) = targets.0.get(&id) else {
                continue;
            };
            let target = noop::print(c, target);
            (
                message(kind, &target, false, read, overwritten),
                identifier(kind, false, read, overwritten),
            )
        } else {
            let target = format!("variable ${name}");
            (
                message(kind, &target, unused_variable, read, overwritten),
                identifier(kind, unused_variable, read, overwritten),
            )
        };
        errors.push((id, identifier, message));
    }
    for (at, identifier, message) in errors {
        let line = c.line_of(at);
        c.report_at_line(line, &identifier, message, None);
    }
}

/// getMessage()
fn message(
    kind: Kind,
    target: &str,
    unused_variable: bool,
    read: bool,
    overwritten: bool,
) -> String {
    let outcome = if read {
        "only flows into values that are never used"
    } else if overwritten {
        "is never read before being overwritten"
    } else {
        "is never read"
    };
    match kind {
        Kind::Assign | Kind::Input if unused_variable => {
            let mut target = target.to_string();
            target[..1].make_ascii_uppercase();
            format!("{target} is never read.")
        }
        Kind::Assign | Kind::Input => format!("Value assigned to {target} {outcome}."),
        Kind::PreInc | Kind::PostInc => format!("Value of {target} after ++ {outcome}."),
        Kind::PreDec | Kind::PostDec => format!("Value of {target} after -- {outcome}."),
        Kind::ForeachValue | Kind::ForeachValueWithKey => {
            format!("Foreach value {target} {outcome}.")
        }
        Kind::ForeachKey => format!("Foreach key {target} {outcome}."),
        Kind::Catch => format!("Catch {target} {outcome}."),
    }
}

/// getIdentifier()
fn identifier(kind: Kind, unused_variable: bool, read: bool, overwritten: bool) -> String {
    let prefix = match kind {
        Kind::Assign | Kind::Input if unused_variable => return "variable.unused".into(),
        Kind::Assign | Kind::Input => "assign",
        Kind::PreInc => "preInc",
        Kind::PostInc => "postInc",
        Kind::PreDec => "preDec",
        Kind::PostDec => "postDec",
        Kind::ForeachValueWithKey => return "foreach.unusedValueFlow".into(),
        Kind::ForeachValue => {
            return (if read {
                "foreach.unusedValueFlow"
            } else if overwritten {
                "foreach.valueOverwritten"
            } else {
                "foreach.unusedValue"
            })
            .into();
        }
        Kind::ForeachKey => {
            return (if read {
                "foreach.unusedKeyFlow"
            } else if overwritten {
                "foreach.keyOverwritten"
            } else {
                "foreach.unusedKey"
            })
            .into();
        }
        Kind::Catch => return "catch.unusedVariableFlow".into(),
    };
    let suffix = if read {
        "unusedFlow"
    } else if overwritten {
        "overwritten"
    } else {
        "unused"
    };
    format!("{prefix}.{suffix}")
}
