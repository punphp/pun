//! `lint/dead-code/no-redundant-property-default`, ported from Rector 2.6.1's
//! `RemoveDefaultValueFromAssignedPropertyRector` (rules/DeadCode/Rector/Property), with `ConstructorAssignDetector`,
//! `PropertyAssignMatcher` and `PropertyFetchAnalyzer::isFilledViaMethodCallInConstructStmts`.
//!
//! A typed, non-static property without hooks, private or of a final class, loses its default value when the
//! constructor always assigns it: a statement of the constructor's own body assigns it (not through an array key,
//! and not from its own value), or both branches of an `if`/`else` there do, or a method the constructor calls as a
//! statement assigns it in its own body. The constructor (or such a method) must not return early. Methods marked
//! `@required` or `@inject` (or with Symfony's `#[Required]`, Nette's `#[Inject]`) initialize too, and so do `setUp()`
//! and `setUpBeforeClass()` of a PHPUnit test case.
//!
//! Rector knows a test case and the methods a call reaches by their types. Pun counts a class as a test case when it
//! extends `PHPUnit\Framework\TestCase` itself, and follows calls on `$this`, `self` and `static` to the class's own
//! methods; it does not look into a parent class (lint.md Build 8).

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_batch5::{doc_comment, has_attribute, member_from, method, resolve_class_name};
use crate::syntax::{node_span, same_code, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoRedundantPropertyDefault;

impl Rule for NoRedundantPropertyDefault {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-redundant-property-default"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (members, left_brace, is_final, extends) = match node {
            Node::Class(c) => (
                &c.members,
                c.left_brace,
                c.modifiers.iter().any(|m| matches!(m, Modifier::Final(_))),
                c.extends.as_ref(),
            ),
            Node::AnonymousClass(c) => (&c.members, c.left_brace, false, c.extends.as_ref()),
            _ => return,
        };
        let Some(constructor) = method(members, b"__construct") else {
            return;
        };
        // An early return can skip the assignment, so the default value is still needed.
        if has_early_return(members, constructor) {
            return;
        }
        let mut path = ancestors.to_vec();
        path.push(node);
        let source = ctx.source;
        let scope = Scope {
            source,
            members,
            left_brace,
            path: &path,
            extends,
            constructor: Some(constructor),
            test_case: None,
        };
        let mut edits = Vec::new();
        for member in members.iter() {
            let ClassLikeMember::Property(Property::Plain(property)) = member else {
                continue;
            };
            // Untyped properties are RemoveNullPropertyInitializationRector's.
            if property.hint.is_none()
                || property
                    .modifiers
                    .iter()
                    .any(|m| matches!(m, Modifier::Static(_)))
                || (!property
                    .modifiers
                    .iter()
                    .any(|m| matches!(m, Modifier::Private(_)))
                    && !is_final)
            {
                continue;
            }
            for item in property.items.iter() {
                let PropertyItem::Concrete(item) = item else {
                    continue;
                };
                let name = &item.variable.name[1..];
                if !scope.is_assigned(name) || assigned_via_array_key(node, name) {
                    continue;
                }
                let start = node_span(&item.variable).end;
                edits.push(Edit::new(Span::new(start, node_span(item.value).end), ""));
            }
        }
        let Some(first) = edits.first() else { return };
        out.push(Report {
            span: first.span,
            message: "The constructor always assigns this property; drop its default value."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `ConstructorAssignDetector::isPropertyAssigned` for the property `name` (without its `$`) of the class or
/// anonymous class with `members`, `left_brace` and `extends`, which `path` leads to (and ends with): false without a
/// constructor.
pub(crate) fn is_assigned_in_constructor<'a>(
    source: &str,
    members: &'a Sequence<'a, ClassLikeMember<'a>>,
    left_brace: mago_span::Span,
    extends: Option<&'a Extends<'a>>,
    path: &[Node<'a, 'a>],
    name: &[u8],
) -> bool {
    let Some(constructor) = method(members, b"__construct") else {
        return false;
    };
    Scope {
        source,
        members,
        left_brace,
        path,
        extends,
        constructor,
    }
    .is_assigned(name)
}

/// `hasEarlyReturn`: a `return` in the constructor, or in a class method it calls as a statement on `$this`.
fn has_early_return(members: &Sequence<'_, ClassLikeMember<'_>>, constructor: &Method<'_>) -> bool {
    if has_return(Node::Method(constructor)) {
        return true;
    }
    first_level(constructor).any(|statement| {
        let Statement::Expression(e) = statement else {
            return false;
        };
        let Expression::Call(Call::Method(call)) = unparenthesize(e.expression) else {
            return false;
        };
        let ClassLikeMemberSelector::Identifier(name) = &call.method else {
            return false;
        };
        is_this(call.object)
            && method(members, name.value).is_some_and(|m| has_return(Node::Method(m)))
    })
}

/// `hasInstancesOfInFunctionLikeScoped(..., Return_::class)`: a `return` outside nested functions and classes.
fn has_return(node: Node<'_, '_>) -> bool {
    let mut found = false;
    node.visit_children(|child| {
        found = found
            || match child {
                Node::Return(_) => true,
                Node::Closure(_)
                | Node::ArrowFunction(_)
                | Node::Function(_)
                | Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_) => false,
                _ => has_return(child),
            }
    });
    found
}

/// The statements of a method's body.
fn first_level<'a>(method: &'a Method<'a>) -> impl Iterator<Item = &'a Statement<'a>> {
    let statements: Vec<&Statement> = match &method.body {
        MethodBody::Concrete(block) => block.statements.iter().collect(),
        MethodBody::Abstract(_) => Vec::new(),
    };
    statements.into_iter()
}

fn is_this(expr: &Expression<'_>) -> bool {
    matches!(unparenthesize(expr), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
}

/// `$this->name`, `self::$name` or `static::$name`.
fn is_fetch(expr: &Expression<'_>, name: &[u8]) -> bool {
    match unparenthesize(expr) {
        Expression::Access(Access::Property(a)) => {
            is_this(a.object)
                && matches!(&a.property, ClassLikeMemberSelector::Identifier(id) if id.value == name)
        }
        Expression::Access(Access::StaticProperty(a)) => {
            matches!(a.class, Expression::Self_(_) | Expression::Static(_))
                && matches!(&a.property, Variable::Direct(v) if &v.name[1..] == name)
        }
        _ => false,
    }
}

/// `PropertyAssignMatcher::matchPropertyAssignExpr`: an assignment to `$this->name` or `$this->name[...]`.
fn assigns<'a>(expr: &'a Expression<'a>, name: &[u8]) -> Option<&'a Assignment<'a>> {
    let Expression::Assignment(a) = unparenthesize(expr) else {
        return None;
    };
    if !matches!(a.operator, AssignmentOperator::Assign(_)) {
        return None;
    }
    let target = match unparenthesize(a.lhs) {
        Expression::ArrayAccess(access) => access.array,
        Expression::ArrayAppend(append) => append.array,
        lhs => lhs,
    };
    is_fetch(target, name).then_some(a)
}

struct Scope<'s, 'p, 'a> {
    source: &'s str,
    members: &'a Sequence<'a, ClassLikeMember<'a>>,
    left_brace: mago_span::Span,
    path: &'p [Node<'a, 'a>],
    extends: Option<&'a Extends<'a>>,
    constructor: Option<&'a Method<'a>>,
    /// Whether the class is a PHPUnit test case, when the caller knows; else it extends `PHPUnit\Framework\TestCase`.
    test_case: Option<bool>,
}

/// `ConstructorAssignDetector::isPropertyAssigned` for the property `name` of the class or anonymous class `node`,
/// whose ancestors are `ancestors`; `test_case` says whether it is a PHPUnit test case.
pub(crate) fn is_property_assigned(
    source: &str,
    node: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    test_case: bool,
    name: &[u8],
) -> bool {
    let (members, left_brace, extends) = match node {
        Node::Class(c) => (&c.members, c.left_brace, c.extends.as_ref()),
        Node::AnonymousClass(c) => (&c.members, c.left_brace, c.extends.as_ref()),
        _ => return false,
    };
    let mut path = ancestors.to_vec();
    path.push(node);
    let scope = Scope {
        source,
        members,
        left_brace,
        path: &path,
        extends,
        constructor: method(members, b"__construct"),
        test_case: Some(test_case),
    };
    scope.is_assigned(name)
}

/// What a walk of an initializing method found.
enum Found {
    Continue,
    Stop(bool),
}

impl<'a> Scope<'_, '_, 'a> {
    /// `ConstructorAssignDetector::isPropertyAssigned`.
    fn is_assigned(&self, name: &[u8]) -> bool {
        let mut assigned = false;
        for method in self.initializers() {
            let first: Vec<Span> = first_level(method)
                .filter_map(|s| match s {
                    Statement::Expression(e) => Some(node_span(unparenthesize(e.expression))),
                    _ => None,
                })
                .collect();
            for statement in first_level(method) {
                if let Found::Stop(value) =
                    self.walk(Node::Statement(statement), name, &first, false)
                {
                    assigned = value;
                    break;
                }
            }
        }
        assigned || self.filled_via_method_call(name)
    }

    /// The constructor, `setUp()` and `setUpBeforeClass()` of a test case, and the autowired methods.
    fn initializers(&self) -> Vec<&'a Method<'a>> {
        let mut out: Vec<&'a Method<'a>> = self.constructor.into_iter().collect();
        let test_case = self.test_case.unwrap_or_else(|| {
            self.extends.is_some_and(|e| {
                e.types.iter().any(|t| {
                    resolve_class_name(self.path, t)
                        .eq_ignore_ascii_case("PHPUnit\\Framework\\TestCase")
                })
            })
        });
        if test_case {
            out.extend(method(self.members, b"setUp"));
            out.extend(method(self.members, b"setUpBeforeClass"));
        }
        for (index, member) in self.members.iter().enumerate() {
            let ClassLikeMember::Method(m) = member else {
                continue;
            };
            if self.autowired(index, m) {
                out.push(m);
            }
        }
        out
    }

    /// `AutowiredClassMethodOrPropertyAnalyzer::detect`.
    fn autowired(&self, index: usize, m: &Method<'_>) -> bool {
        let doc = doc_comment(
            self.source,
            member_from(self.members, self.left_brace, index),
            node_span(m).start,
        );
        let tagged = doc.is_some_and(|d| {
            self.source[d.start..d.end].lines().any(|line| {
                let line = line
                    .trim_start()
                    .trim_start_matches("/**")
                    .trim_start_matches('*')
                    .trim_start();
                ["@required", "@inject"].iter().any(|tag| {
                    line.strip_prefix(tag).is_some_and(|rest| {
                        rest.is_empty() || rest.starts_with(|c: char| c.is_whitespace() || c == '*')
                    })
                })
            })
        });
        tagged
            || has_attribute(
                self.path,
                &m.attribute_lists,
                "Symfony\\Contracts\\Service\\Attribute\\Required",
            )
            || has_attribute(
                self.path,
                &m.attribute_lists,
                "Nette\\DI\\Attributes\\Inject",
            )
    }

    /// The traversal of `isPropertyAssigned`, in php-parser's order: `in_loop` inside a loop or `switch` body.
    fn walk(&self, node: Node<'a, 'a>, name: &[u8], first: &[Span], in_loop: bool) -> Found {
        if let Node::If(r#if) = node
            && if_else_assign(r#if, name)
        {
            return Found::Stop(!in_loop);
        }
        if let Node::Assignment(a) = node
            && let Some(a) = assigns_node(a, name)
        {
            // Is the property's own value merged into the assignment?
            if uses_fetch(self.source, Node::Expression(a.rhs), a.lhs) {
                return Found::Stop(false);
            }
            if first.contains(&node_span(a)) {
                return Found::Stop(true);
            }
        }
        let loop_body = matches!(
            node,
            Node::For(_) | Node::Foreach(_) | Node::While(_) | Node::DoWhile(_) | Node::Switch(_)
        );
        let scope_break = matches!(
            node,
            Node::Closure(_) | Node::Function(_) | Node::Class(_) | Node::AnonymousClass(_)
        );
        let in_loop = (in_loop || loop_body) && !scope_break;
        let mut found = Found::Continue;
        node.visit_children(|child| {
            if matches!(found, Found::Continue) {
                found = self.walk(child, name, first, in_loop);
            }
        });
        found
    }

    /// `isFilledViaMethodCallInConstructStmts`: a constructor statement calling a class method that assigns the
    /// property in a statement of its own body.
    fn filled_via_method_call(&self, name: &[u8]) -> bool {
        let Some(constructor) = self.constructor else {
            return false;
        };
        first_level(constructor).any(|statement| {
            let Statement::Expression(e) = statement else { return false };
            let (selector, own) = match unparenthesize(e.expression) {
                Expression::Call(Call::Method(call)) => (&call.method, is_this(call.object)),
                Expression::Call(Call::StaticMethod(call)) => (
                    &call.method,
                    matches!(call.class, Expression::Self_(_) | Expression::Static(_)),
                ),
                _ => return false,
            };
            let ClassLikeMemberSelector::Identifier(called) = selector else {
                return false;
            };
            own && method(self.members, called.value).is_some_and(|m| {
                first_level(m).any(|s| {
                    matches!(s, Statement::Expression(e)
                        if matches!(unparenthesize(e.expression), Expression::Assignment(a)
                            if matches!(a.operator, AssignmentOperator::Assign(_)) && is_fetch(a.lhs, name)))
                })
            })
        })
    }
}

fn assigns_node<'a>(a: &'a Assignment<'a>, name: &[u8]) -> Option<&'a Assignment<'a>> {
    if !matches!(a.operator, AssignmentOperator::Assign(_)) {
        return None;
    }
    let target = match unparenthesize(a.lhs) {
        Expression::ArrayAccess(access) => access.array,
        Expression::ArrayAppend(append) => append.array,
        lhs => lhs,
    };
    is_fetch(target, name).then_some(a)
}

/// `isIfElseAssign`: an `if` with an `else` and no `elseif` whose branches both assign the property in a list of
/// expression statements only.
fn if_else_assign(r#if: &If<'_>, name: &[u8]) -> bool {
    let (stmts, else_stmts): (Vec<&Statement>, Vec<&Statement>) = match &r#if.body {
        IfBody::Statement(body) => {
            let Some(else_clause) = &body.else_clause else {
                return false;
            };
            if !body.else_if_clauses.is_empty() {
                return false;
            }
            (
                body_statements(body.statement),
                body_statements(else_clause.statement),
            )
        }
        IfBody::ColonDelimited(body) => {
            let Some(else_clause) = &body.else_clause else {
                return false;
            };
            if !body.else_if_clauses.is_empty() {
                return false;
            }
            (
                body.statements.iter().collect(),
                else_clause.statements.iter().collect(),
            )
        }
    };
    assigned_in(&stmts, name) && assigned_in(&else_stmts, name)
}

fn body_statements<'a>(statement: &'a Statement<'a>) -> Vec<&'a Statement<'a>> {
    match statement {
        Statement::Block(block) => block.statements.iter().collect(),
        statement => vec![statement],
    }
}

/// `isAssignedInStmts`.
fn assigned_in(stmts: &[&Statement<'_>], name: &[u8]) -> bool {
    let mut assigned = false;
    for statement in stmts {
        match statement {
            Statement::Expression(e) => {
                if assigns(e.expression, name).is_some() {
                    assigned = true;
                }
            }
            Statement::Noop(_) => {}
            _ => return false,
        }
    }
    assigned
}

/// `isPropertyUsedInAssign`: a property fetch in `node` that prints as `target`.
fn uses_fetch(source: &str, node: Node<'_, '_>, target: &Expression<'_>) -> bool {
    if let Node::PropertyAccess(a) = node
        && matches!(&a.property, ClassLikeMemberSelector::Identifier(_))
        && same_code(source, node_span(a), node_span(unparenthesize(target)))
    {
        return true;
    }
    let mut found = false;
    node.visit_children(|child| found = found || uses_fetch(source, child, target));
    found
}

/// `isAssignedViaArrayDimFetch`: an assignment anywhere in the class to `$this->name[...]`, at any depth of keys.
fn assigned_via_array_key(node: Node<'_, '_>, name: &[u8]) -> bool {
    if let Node::Assignment(a) = node
        && matches!(a.operator, AssignmentOperator::Assign(_))
    {
        let mut target = unparenthesize(a.lhs);
        let mut dim = false;
        loop {
            match target {
                Expression::ArrayAccess(access) => target = unparenthesize(access.array),
                Expression::ArrayAppend(append) => target = unparenthesize(append.array),
                _ => break,
            }
            dim = true;
        }
        if dim && is_fetch(target, name) {
            return true;
        }
    }
    let mut found = false;
    node.visit_children(|child| found = found || assigned_via_array_key(child, name));
    found
}
