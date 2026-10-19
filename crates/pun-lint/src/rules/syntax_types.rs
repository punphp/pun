//! Types read from the syntax, where Rector asks PHPStan: Pun's stand-in until analyse's index can answer
//! (lint.md Build 8). Each answer is "yes" only when the syntax alone proves it.

use std::sync::OnceLock;

use mago_syntax::cst::*;

use super::rector_batch5::{called_function_name, doc_comment, member_from};
use crate::syntax::{node_span, unparenthesize};

const BOOL_FUNCTIONS: &str = include_str!("../../data/bool-functions.txt");

/// Whether `expr` is a `bool` (PHPStan's `isBoolean()->yes()`): `true`, `false`, a comparison, a logical operator,
/// `!`, `(bool)`, `isset`, `empty`, `instanceof`, a call of a built-in function that returns `bool`, or a `bool`
/// parameter of the enclosing function that it never assigns.
pub(crate) fn is_bool(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    if in_trait(ancestors) {
        return false;
    }
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

/// The declared type of the parameter `name` of the enclosing function, when the function never writes it (so the
/// variable keeps its native type).
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

/// The unwritten parameter `name` of the function around the code (by reference too: PHPStan types it as declared).
fn parameter<'ast, 'arena>(
    ancestors: &[Node<'ast, 'arena>],
    name: &[u8],
) -> Option<&'ast FunctionLikeParameter<'arena>> {
    if in_trait(ancestors) {
        return None;
    }
    let (body, params) = ancestors.iter().rev().find_map(|n| match n {
        Node::Function(f) => Some((n, &f.parameter_list)),
        Node::Method(m) => Some((n, &m.parameter_list)),
        Node::Closure(c) => Some((n, &c.parameter_list)),
        Node::ArrowFunction(a) => Some((n, &a.parameter_list)),
        _ => None,
    })?;
    let param = params.parameters.iter().find(|p| p.variable.name == name)?;
    let written = written(*body, name);
    if written {
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
    if in_trait(ancestors) {
        return false;
    }
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

/// Whether `expr` is an `array`: an `array` parameter never written, a local only assigned array literals, or a
/// property of the enclosing class (`$this->p`, `self::$p`, `static::$p`) declared `array`.
pub(crate) fn is_array(source: &str, ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    if in_trait(ancestors) {
        return false;
    }
    match unparenthesize(expr) {
        // A variadic parameter is an array whatever its type.
        Expression::Variable(Variable::Direct(v))
            if parameter(ancestors, v.name).is_some_and(|p| p.ellipsis.is_some()) =>
        {
            true
        }
        Expression::Variable(Variable::Direct(v)) => match parameter_hint(ancestors, v.name) {
            Some(hint) => matches!(hint, Hint::Array(_)),
            None => {
                local_of(ancestors, v.name, &|e| {
                    matches!(
                        unparenthesize(e),
                        Expression::Array(_) | Expression::LegacyArray(_)
                    )
                }) && assigned_first(ancestors, v.name)
            }
        },
        Expression::Access(Access::Property(p)) if is_this(p.object) => match &p.property {
            ClassLikeMemberSelector::Identifier(id) => {
                own_property_is_array(source, ancestors, id.value)
            }
            _ => false,
        },
        Expression::Access(Access::StaticProperty(p))
            if matches!(p.class, Expression::Self_(_) | Expression::Static(_)) =>
        {
            match &p.property {
                Variable::Direct(v) => own_property_is_array(source, ancestors, &v.name[1..]),
                _ => false,
            }
        }
        _ => false,
    }
}

fn own_property_is_array(source: &str, ancestors: &[Node<'_, '_>], name: &[u8]) -> bool {
    let Some((class, members, left_brace)) = ancestors.iter().rev().find_map(|n| match n {
        Node::Class(c) => Some((*n, &c.members, c.left_brace)),
        Node::AnonymousClass(c) => Some((*n, &c.members, c.left_brace)),
        _ => None,
    }) else {
        return false;
    };
    members.iter().enumerate().any(|(index, m)| match m {
        ClassLikeMember::Property(Property::Plain(p)) => {
            let Some(item) = p.items.iter().find(|i| &i.variable().name[1..] == name) else {
                return false;
            };
            match &p.hint {
                Some(hint) => matches!(hint, Hint::Array(_)),
                // Untyped: the scope reads an array `@var`, and `AllAssignNodePropertyTypeInferer` the default and
                // every assignment in the class.
                None => {
                    let from = member_from(members, left_brace, index);
                    doc_comment(source, from, node_span(m).start)
                        .is_some_and(|doc| var_is_array(&source[doc.start..doc.end]))
                        && matches!(item, PropertyItem::Concrete(c)
                            if matches!(unparenthesize(c.value), Expression::Array(_) | Expression::LegacyArray(_)))
                        && only_array_assigned(class, name)
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

/// Whether the doc comment's `@var` type is an array type alone: `array`, `list`, `T[]`, or one of those with
/// parameters or a shape, and no union.
fn var_is_array(doc: &str) -> bool {
    let Some(at) = doc.find("@var") else {
        return false;
    };
    let rest = doc[at + 4..].trim_start();
    let mut depth = 0i32;
    let mut end = rest.len();
    for (i, c) in rest.char_indices() {
        match c {
            '<' | '{' | '(' => depth += 1,
            '>' | '}' | ')' => depth -= 1,
            '|' | '&' if depth == 0 => return false,
            c if c.is_whitespace() && depth == 0 => {
                end = i;
                break;
            }
            _ => {}
        }
    }
    let ty = rest[..end].to_ascii_lowercase();
    let head = ty.split(['<', '{']).next().unwrap_or("");
    ty.ends_with("[]")
        || matches!(
            head,
            "array" | "list" | "non-empty-array" | "non-empty-list"
        )
}

/// `AssignToPropertyTypeInferer` on an untyped property: every plain assignment of `$this->name` (or `self::$name`,
/// `static::$name`) in the class sets an array literal, an assignment of one of its elements (`$this->name[] = $v`)
/// never takes a variable that may be an untyped parameter, and the class writes no dynamic property
/// (`$this->$x = ...`).
fn only_array_assigned(class: Node<'_, '_>, name: &[u8]) -> bool {
    !breaks_array(class, None, name)
}

/// Whether `node` holds a write of the property that [`only_array_assigned`] refuses; `method` is the method around
/// it.
fn breaks_array<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    method: Option<&'ast Method<'arena>>,
    name: &[u8],
) -> bool {
    let method = match node {
        Node::Method(m) => Some(m),
        _ => method,
    };
    if let Node::Assignment(a) = node
        && breaks_with(a, method, name)
    {
        return true;
    }
    let mut found = false;
    node.visit_children(|child| {
        if !found && breaks_array(child, method, name) {
            found = true;
        }
    });
    found
}

/// [`breaks_array`] for one assignment.
fn breaks_with(a: &Assignment<'_>, method: Option<&Method<'_>>, name: &[u8]) -> bool {
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
        // `isNonTypedFromParam`: a variable may be an untyped parameter unless it is one declared with a type.
        match unparenthesize(a.rhs) {
            Expression::Variable(Variable::Direct(v)) => !method.is_some_and(|method| {
                !written(Node::Method(method), v.name)
                    && method.parameter_list.parameters.iter().any(|p| {
                        p.variable.name == v.name
                            && p.ampersand.is_none()
                            && p.ellipsis.is_none()
                            && p.hint
                                .as_ref()
                                .is_some_and(|h| !matches!(h, Hint::Mixed(_)))
                    })
            }),
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

/// Whether the function `body` writes the variable `name`: assigns it, takes a reference to it, or binds it in a
/// `foreach`, `global`, `static` or `list()`.
fn written(body: Node<'_, '_>, name: &[u8]) -> bool {
    super::side_effect::any_node(body, &|n| match n {
        Node::Assignment(a) => {
            matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(l)) if l.name == name)
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
    })
}

/// Whether the code sits in a trait: Rector analyses a trait's methods without a class, so PHPStan knows no type
/// there.
pub(crate) fn in_trait(ancestors: &[Node<'_, '_>]) -> bool {
    ancestors.iter().any(|n| matches!(n, Node::Trait(_)))
}

/// Whether the enclosing function's own statement list assigns `name` (`$name = ...;`) before anything else
/// mentions it, so the variable is defined wherever it is read. Only asked after [`local_of`], which found that
/// function.
fn assigned_first(ancestors: &[Node<'_, '_>], name: &[u8]) -> bool {
    let statements = ancestors.iter().rev().find_map(|n| match n {
        Node::Function(f) => Some(&f.body.statements),
        Node::Method(Method {
            body: MethodBody::Concrete(b),
            ..
        }) => Some(&b.statements),
        Node::Closure(c) => Some(&c.body.statements),
        _ => None,
    });
    // The first statement that mentions it (the one being read, at the latest) must be the assignment.
    statements
        .unwrap()
        .iter()
        .find(|statement| {
            super::side_effect::any_node(
                Node::Statement(statement),
                &|n| matches!(n, Node::DirectVariable(d) if d.name == name),
            )
        })
        .is_some_and(|statement| {
            matches!(statement, Statement::Expression(e)
                if matches!(unparenthesize(e.expression), Expression::Assignment(a)
                    if matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(l)) if l.name == name)))
        })
}
