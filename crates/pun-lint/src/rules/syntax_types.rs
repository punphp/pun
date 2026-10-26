//! Types read from the syntax, where Rector asks PHPStan: Pun's stand-in until analyse's index can answer
//! (lint.md Build 8). Each answer is "yes" only when the syntax alone proves it.

use std::sync::OnceLock;

use mago_syntax::cst::*;

use super::rector_batch5::called_function_name;
use crate::syntax::{node_span, unparenthesize};

const BOOL_FUNCTIONS: &str = include_str!("../../data/bool-functions.txt");

/// Whether `expr` is a `bool` (PHPStan's `isBoolean()->yes()`): `true`, `false`, a comparison, a logical operator,
/// `!`, `(bool)`, `isset`, `empty`, `instanceof`, a call of a built-in function that returns `bool`, or a `bool`
/// parameter of the enclosing function that it never assigns.
pub(crate) fn is_bool(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    if let Expression::Variable(Variable::Direct(v)) = unparenthesize(expr) {
        return match parameter_hint(ancestors, v.name) {
            Some(hint) => matches!(hint, Hint::Bool(_)),
            None => local_of(ancestors, v.name, &|e| bool_value(ancestors, e)),
        };
    }
    bool_value(ancestors, expr)
}

/// [`is_bool`] without following variables.
fn bool_value(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Literal(Literal::True(_) | Literal::False(_)) => true,
        Expression::Binary(b) => matches!(
            b.operator,
            BinaryOperator::Equal(_)
                | BinaryOperator::NotEqual(_)
                | BinaryOperator::Identical(_)
                | BinaryOperator::NotIdentical(_)
                | BinaryOperator::AngledNotEqual(_)
                | BinaryOperator::LessThan(_)
                | BinaryOperator::LessThanOrEqual(_)
                | BinaryOperator::GreaterThan(_)
                | BinaryOperator::GreaterThanOrEqual(_)
                | BinaryOperator::Instanceof(_)
                | BinaryOperator::And(_)
                | BinaryOperator::Or(_)
                | BinaryOperator::LowAnd(_)
                | BinaryOperator::LowOr(_)
                | BinaryOperator::LowXor(_)
        ),
        // `@expr` has the type of `expr`.
        Expression::UnaryPrefix(u)
            if matches!(u.operator, UnaryPrefixOperator::ErrorControl(_)) =>
        {
            bool_value(ancestors, u.operand)
        }
        Expression::UnaryPrefix(u) => matches!(
            u.operator,
            UnaryPrefixOperator::Not(_)
                | UnaryPrefixOperator::BoolCast(..)
                | UnaryPrefixOperator::BooleanCast(..)
        ),
        Expression::Construct(Construct::Isset(_) | Construct::Empty(_)) => true,
        Expression::Call(Call::Function(call)) => {
            static CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
            let names = CACHE.get_or_init(|| BOOL_FUNCTIONS.lines().collect());
            called_function_name(ancestors, call.function).is_some_and(|n| {
                names
                    .binary_search(&n.to_ascii_lowercase().as_str())
                    .is_ok()
            })
        }
        Expression::Call(Call::Method(call)) if is_this(call.object) => {
            own_method_returns(ancestors, &call.method, &|h| matches!(h, Hint::Bool(_)))
        }
        Expression::Call(Call::StaticMethod(call))
            if matches!(call.class, Expression::Self_(_) | Expression::Static(_)) =>
        {
            own_method_returns(ancestors, &call.method, &|h| matches!(h, Hint::Bool(_)))
        }
        _ => false,
    }
}

/// The declared type of the parameter `name` of the enclosing function, when nothing writes it before the code (so
/// the variable keeps its native type, as in PHPStan's scope there).
pub(crate) fn parameter_hint<'ast, 'arena>(
    ancestors: &[Node<'ast, 'arena>],
    name: &[u8],
) -> Option<&'ast Hint<'arena>> {
    let param = parameter(ancestors, name)?;
    if param.ellipsis.is_some() {
        return None;
    }
    param.hint.as_ref()
}

/// The parameter `name` of the function around the code, when nothing writes it before the code: no
/// write starts before the end of the code's parent, and when the code is in a loop, no write anywhere in the
/// function (the loop can carry a later write back). By reference too: PHPStan types it as declared.
fn parameter<'ast, 'arena>(
    ancestors: &[Node<'ast, 'arena>],
    name: &[u8],
) -> Option<&'ast FunctionLikeParameter<'arena>> {
    let at = ancestors.iter().rposition(|n| {
        matches!(
            n,
            Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_)
        )
    })?;
    let body = &ancestors[at];
    let params = match body {
        Node::Function(f) => &f.parameter_list,
        Node::Method(m) => &m.parameter_list,
        Node::Closure(c) => &c.parameter_list,
        Node::ArrowFunction(a) => &a.parameter_list,
        _ => unreachable!(),
    };
    let param = params.parameters.iter().find(|p| p.variable.name == name)?;
    let in_loop = ancestors[at..].iter().any(|n| {
        matches!(
            n,
            Node::For(_) | Node::Foreach(_) | Node::While(_) | Node::DoWhile(_)
        )
    });
    let until = match in_loop {
        true => usize::MAX,
        false => ancestors.last().map_or(usize::MAX, |n| node_span(n).end),
    };
    if written(*body, name, until) {
        return None;
    }
    Some(param)
}

fn is_this(expr: &Expression<'_>) -> bool {
    matches!(unparenthesize(expr), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
}

/// Whether the enclosing class declares the method `selector` names with a return type that `check` accepts.
fn own_method_returns(
    ancestors: &[Node<'_, '_>],
    selector: &ClassLikeMemberSelector<'_>,
    check: &dyn Fn(&Hint<'_>) -> bool,
) -> bool {
    let ClassLikeMemberSelector::Identifier(name) = selector else {
        return false;
    };
    let Some(members) = ancestors.iter().rev().find_map(|n| match n {
        Node::Class(c) => Some(&c.members),
        Node::Enum(e) => Some(&e.members),
        Node::AnonymousClass(c) => Some(&c.members),
        _ => None,
    }) else {
        return false;
    };
    members.iter().any(|m| match m {
        ClassLikeMember::Method(method) => {
            method.name.value.eq_ignore_ascii_case(name.value)
                && method
                    .return_type_hint
                    .as_ref()
                    .is_some_and(|r| check(&r.hint))
        }
        _ => false,
    })
}

/// Whether `name` is a local variable of the enclosing function (not a parameter or a `use`d variable) that is
/// written only by plain assignments, at least one, each of a value `check` accepts.
pub(crate) fn local_of(
    ancestors: &[Node<'_, '_>],
    name: &[u8],
    check: &dyn Fn(&Expression<'_>) -> bool,
) -> bool {
    let Some(body) = ancestors
        .iter()
        .rev()
        .find(|n| matches!(n, Node::Function(_) | Node::Method(_) | Node::Closure(_)))
    else {
        return false;
    };
    if let Node::Closure(c) = body
        && c.use_clause
            .as_ref()
            .is_some_and(|u| u.variables.iter().any(|x| x.variable.name == name))
    {
        return false;
    }
    let assigned = std::cell::Cell::new(false);
    let other = super::side_effect::any_node(*body, &|n| match n {
        Node::Assignment(a) if matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(l)) if l.name == name) =>
        {
            assigned.set(true);
            !(matches!(a.operator, AssignmentOperator::Assign(_)) && check(a.rhs))
        }
        Node::UnaryPrefix(u) => {
            matches!(u.operator, UnaryPrefixOperator::Reference(_))
                && matches!(u.operand, Expression::Variable(Variable::Direct(d)) if d.name == name)
        }
        Node::Foreach(_) | Node::Global(_) | Node::Static(_) | Node::List(_) => {
            super::side_effect::any_node(
                n,
                &|m| matches!(m, Node::DirectVariable(d) if d.name == name),
            )
        }
        _ => false,
    });
    !other && assigned.get()
}

const INT_FUNCTIONS: &str = include_str!("../../data/int-functions.txt");

/// Whether `expr` is an `int`: an integer literal (or its negation), a call of a built-in function that returns an
/// `int`, an own method declared `: int`, an `int` parameter never written, or a local only assigned one of those.
pub(crate) fn is_int(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    if let Expression::Variable(Variable::Direct(v)) = unparenthesize(expr) {
        return match parameter_hint(ancestors, v.name) {
            Some(hint) => matches!(hint, Hint::Integer(_)),
            None => local_of(ancestors, v.name, &|e| int_value(ancestors, e)),
        };
    }
    int_value(ancestors, expr)
}

fn int_value(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Literal(Literal::Integer(_)) => true,
        Expression::UnaryPrefix(u)
            if matches!(
                u.operator,
                UnaryPrefixOperator::Negation(_) | UnaryPrefixOperator::Plus(_)
            ) =>
        {
            matches!(
                unparenthesize(u.operand),
                Expression::Literal(Literal::Integer(_))
            )
        }
        Expression::UnaryPrefix(u)
            if matches!(
                u.operator,
                UnaryPrefixOperator::IntCast(..) | UnaryPrefixOperator::IntegerCast(..)
            ) =>
        {
            true
        }
        Expression::Call(Call::Function(call)) => {
            static CACHE: OnceLock<Vec<&'static str>> = OnceLock::new();
            let names = CACHE.get_or_init(|| INT_FUNCTIONS.lines().collect());
            called_function_name(ancestors, call.function).is_some_and(|n| {
                names
                    .binary_search(&n.to_ascii_lowercase().as_str())
                    .is_ok()
            })
        }
        Expression::Call(Call::Method(call)) if is_this(call.object) => {
            own_method_returns(ancestors, &call.method, &|h| matches!(h, Hint::Integer(_)))
        }
        Expression::Call(Call::StaticMethod(call))
            if matches!(call.class, Expression::Self_(_) | Expression::Static(_)) =>
        {
            own_method_returns(ancestors, &call.method, &|h| matches!(h, Hint::Integer(_)))
        }
        _ => false,
    }
}

/// Whether a variable that sets an element of a property may be an untyped parameter (`isNonTypedFromParam`),
/// given the method it is in.
pub(crate) type NonTyped<'f> = &'f dyn Fn(&Expression<'_>, Option<&Method<'_>>) -> bool;

/// Whether `expr` is a property of the enclosing class (`$this->p`, `self::$p`, `static::$p`) declared `array`, or
/// untyped and only ever assigned arrays (`non_typed` judges the variables that set its elements).
pub(crate) fn is_array_property(
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    non_typed: NonTyped<'_>,
) -> bool {
    let name = match unparenthesize(expr) {
        Expression::Access(Access::Property(PropertyAccess {
            object,
            property: ClassLikeMemberSelector::Identifier(id),
            ..
        })) if is_this(object) => id.value,
        Expression::Access(Access::StaticProperty(StaticPropertyAccess {
            class: Expression::Self_(_) | Expression::Static(_),
            property: Variable::Direct(v),
            ..
        })) => &v.name[1..],
        _ => return false,
    };
    own_property_is_array(ancestors, name, non_typed)
}

fn own_property_is_array(ancestors: &[Node<'_, '_>], name: &[u8], non_typed: NonTyped<'_>) -> bool {
    let Some((class, members)) = ancestors.iter().rev().find_map(|n| match n {
        Node::Class(c) => Some((*n, &c.members)),
        Node::AnonymousClass(c) => Some((*n, &c.members)),
        _ => None,
    }) else {
        return false;
    };
    members.iter().any(|m| match m {
        ClassLikeMember::Property(Property::Plain(p)) => {
            let Some(item) = p.items.iter().find(|i| &i.variable().name[1..] == name) else {
                return false;
            };
            match &p.hint {
                Some(hint) => matches!(hint, Hint::Array(_)),
                // Untyped: `AllAssignNodePropertyTypeInferer` reads the default and every assignment in the class.
                None => {
                    matches!(item, PropertyItem::Concrete(c)
                            if matches!(unparenthesize(c.value), Expression::Array(_) | Expression::LegacyArray(_)))
                        && only_array_assigned(class, name, non_typed)
                }
            }
        }
        ClassLikeMember::Property(Property::Hooked(p)) => {
            matches!(p.hint, Some(Hint::Array(_))) && &p.item.variable().name[1..] == name
        }
        ClassLikeMember::Method(method) if method.name.value.eq_ignore_ascii_case(b"__construct") => {
            method.parameter_list.parameters.iter().any(|p| {
                p.is_promoted_property() && matches!(p.hint, Some(Hint::Array(_))) && &p.variable.name[1..] == name
            })
        }
        _ => false,
    })
}

/// `AssignToPropertyTypeInferer` on an untyped property: every plain assignment of `$this->name` (or `self::$name`,
/// `static::$name`) in the class sets an array literal, an assignment of one of its elements (`$this->name[] = $v`)
/// never takes a variable that may be an untyped parameter, and the class writes no dynamic property
/// (`$this->$x = ...`).
fn only_array_assigned(class: Node<'_, '_>, name: &[u8], non_typed: NonTyped<'_>) -> bool {
    !breaks_array(class, None, name, non_typed)
}

/// Whether `node` holds a write of the property that [`only_array_assigned`] refuses; `method` is the method around
/// it.
fn breaks_array<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    method: Option<&'ast Method<'arena>>,
    name: &[u8],
    non_typed: NonTyped<'_>,
) -> bool {
    let method = match node {
        Node::Method(m) => Some(m),
        _ => method,
    };
    if let Node::Assignment(a) = node
        && breaks_with(a, method, name, non_typed)
    {
        return true;
    }
    let mut found = false;
    node.visit_children(|child| {
        if !found && breaks_array(child, method, name, non_typed) {
            found = true;
        }
    });
    found
}

/// [`breaks_array`] for one assignment.
fn breaks_with(
    a: &Assignment<'_>,
    method: Option<&Method<'_>>,
    name: &[u8],
    non_typed: NonTyped<'_>,
) -> bool {
    let mut target = unparenthesize(a.lhs);
    let mut element = false;
    loop {
        match target {
            Expression::ArrayAccess(x) => target = unparenthesize(x.array),
            Expression::ArrayAppend(x) => target = unparenthesize(x.array),
            _ => break,
        }
        element = true;
    }
    let named = match target {
        Expression::Access(Access::Property(p)) if is_this(p.object) => match &p.property {
            ClassLikeMemberSelector::Identifier(id) => id.value == name,
            _ => return true,
        },
        Expression::Access(Access::StaticProperty(p))
            if matches!(p.class, Expression::Self_(_) | Expression::Static(_)) =>
        {
            match &p.property {
                Variable::Direct(v) => &v.name[1..] == name,
                _ => return true,
            }
        }
        _ => false,
    };
    if !named || !matches!(a.operator, AssignmentOperator::Assign(_)) {
        return named && !element;
    }
    if element {
        // `isNonTypedFromParam` on the value of an element.
        match unparenthesize(a.rhs) {
            Expression::Variable(Variable::Direct(_)) => non_typed(a.rhs, method),
            Expression::Variable(_) => true,
            _ => false,
        }
    } else {
        !matches!(
            unparenthesize(a.rhs),
            Expression::Array(_) | Expression::LegacyArray(_)
        )
    }
}

/// Whether a write of `name` in the function `body` starts before `until`: an assignment, a reference to it, or a
/// binding in a `foreach`, `global`, `static` or `list()`.
fn written(body: Node<'_, '_>, name: &[u8], until: usize) -> bool {
    super::side_effect::any_node(body, &|n| {
        node_span(&n).start < until
            && match n {
                Node::Assignment(a) => {
                    matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(l)) if l.name == name)
                }
                Node::UnaryPrefix(u) => {
                    matches!(u.operator, UnaryPrefixOperator::Reference(_))
                        && matches!(u.operand, Expression::Variable(Variable::Direct(d)) if d.name == name)
                }
                Node::ForeachTarget(_) | Node::Global(_) | Node::Static(_) | Node::List(_) => {
                    super::side_effect::any_node(
                        n,
                        &|m| matches!(m, Node::DirectVariable(d) if d.name == name),
                    )
                }
                _ => false,
            }
    })
}
