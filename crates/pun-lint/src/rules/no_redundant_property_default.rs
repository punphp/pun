//! `lint/dead-code/no-redundant-property-default`, ported from Rector 2.8.4's
//! `RemoveDefaultValueFromAssignedPropertyRector` (rules/DeadCode/Rector/Property), with `ConstructorAssignDetector`,
//! `PropertyAssignMatcher` and `PropertyFetchAnalyzer::isFilledViaMethodCallInConstructStmts`.
//!
//! A typed, non-static property without hooks, private or of a final class, loses its default value when the
//! constructor always assigns it: a statement of the constructor's own body assigns it (not through an array key,
//! and not from its own value), or both branches of an `if`/`else` there do, or a method the constructor calls as a
//! statement assigns it in its own body. The constructor, or a method it calls directly on `$this`, must not return
//! early. Methods marked
//! `@required` or `@inject` (or with Symfony's `#[Required]`, Nette's `#[Inject]`) initialize too, and so do `setUp()`
//! and `setUpBeforeClass()` of a PHPUnit test case.
//!
//! Rector knows a test case and the methods a call reaches by their types. Pun counts a class as a test case when it
//! extends `PHPUnit\Framework\TestCase` itself. Calls on typed peers and inherited methods use the documented
//! receiver and its reflected declaration; only a method declared by this class or an ancestor can initialize it.

use mago_syntax::cst::*;
use pun_phpstan::types::Type;
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_batch5::{
    doc_comment, has_attribute, method, resolve_class_name, scope_statements,
};
use super::rector_stmts::previous_token_end;
use super::types::compare::classes::with_classes;
use super::types::compare::mapper::expr_type;
use super::types::facts::{self, facts, path_of};
use super::types::property_type::ClassView;
use crate::syntax::is_this;
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
        let scope = Scope {
            ctx,
            members,
            comments_from: left_brace.end.offset as usize,
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
                if !scope.is_assigned(name) || assigned_via_array_key(node, name, &path, ctx) {
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
    ctx: &Context<'_>,
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
        ctx,
        members,
        comments_from: left_brace.end.offset as usize,
        path,
        extends,
        constructor: Some(constructor),
        test_case: None,
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

/// Calls resolve the declaring class through reflection, while local fetches compare the documented object type's
/// class name exactly. Neither a dynamic class expression nor a different named class identifies the local class.
fn local_static_class(class: &Expression<'_>, path: &[Node<'_, '_>], reflected: bool) -> bool {
    let id = match unparenthesize(class) {
        Expression::Self_(_) | Expression::Static(_) => return true,
        Expression::Identifier(id) => id,
        _ => return false,
    };
    let Some(Node::Class(class)) = path.last() else {
        return false;
    };
    let namespace = scope_statements(path).0;
    let qualify = |name: &str| match &namespace {
        Some(namespace) => format!("{namespace}\\{name}"),
        None => name.to_string(),
    };
    let own = qualify(&String::from_utf8_lossy(class.name.value));
    let written = id.value();
    let resolved = if written
        .get(..10)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(b"namespace\\"))
    {
        qualify(&String::from_utf8_lossy(&written[10..]))
    } else {
        resolve_class_name(path, id)
    };
    if reflected {
        resolved.eq_ignore_ascii_case(&own)
    } else {
        resolved == own
    }
}

/// `$this->name`, `self::$name`, `static::$name` or the enclosing class's named static fetch.
fn is_fetch(expr: &Expression<'_>, name: &[u8], path: &[Node<'_, '_>], ctx: &Context<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Access(Access::Property(a)) => {
            matches!(&a.property, ClassLikeMemberSelector::Identifier(id) if id.value == name)
                && (is_this(a.object) || local_object(a.object, path, ctx))
        }
        Expression::Access(Access::StaticProperty(a)) => {
            local_static_class(a.class, path, false)
                && matches!(&a.property, Variable::Direct(v) if &v.name[1..] == name)
        }
        _ => false,
    }
}

/// `isLocalPropertyFetch`: a documented object must name the enclosing class exactly; `ThisType` is local.
fn local_object(object: &Expression<'_>, path: &[Node<'_, '_>], ctx: &Context<'_>) -> bool {
    let facts = facts(ctx);
    with_classes(facts.documented(), || {
        let ty = expr_type(&facts, path, unparenthesize(object), false);
        let name = match &ty {
            Type::This(_) => return true,
            Type::Object(name, _, _) | Type::Static(name, _) => name,
            Type::Template(_) => {
                // TemplateObjectType and TemplateGenericObjectType extend ObjectType, unlike a template union
                // or intersection. A template bound can itself be a TemplateObjectType.
                let mut bound = &ty;
                while let Type::Template(template) = bound {
                    bound = &template.bound;
                }
                let Type::Object(name, _, _) = bound else {
                    return false;
                };
                name
            }
            _ => return false,
        };
        let view = ClassView::of(&facts, path[path.len() - 1], &path[..path.len() - 1])
            .expect("the assignment scope is a class");
        *name == view.name
    })
}

/// `PropertyAssignMatcher::matchPropertyAssignExpr`: an assignment to `$this->name` or `$this->name[...]`.
fn assigns<'a>(
    expr: &'a Expression<'a>,
    name: &[u8],
    path: &[Node<'_, '_>],
    ctx: &Context<'_>,
) -> Option<&'a Assignment<'a>> {
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
    is_fetch(target, name, path, ctx).then_some(a)
}

struct Scope<'s, 'p, 'a> {
    ctx: &'s Context<'s>,
    members: &'a Sequence<'a, ClassLikeMember<'a>>,
    /// A lower bound for the first member's leading comments.
    comments_from: usize,
    path: &'p [Node<'a, 'a>],
    extends: Option<&'a Extends<'a>>,
    constructor: Option<&'a Method<'a>>,
    /// Whether the class is a PHPUnit test case, when the caller knows; else it extends `PHPUnit\Framework\TestCase`.
    test_case: Option<bool>,
}

/// `ConstructorAssignDetector::isPropertyAssigned` for the property `name` of `view`;
/// `test_case` says whether it is a PHPUnit test case.
pub(crate) fn is_property_assigned(
    ctx: &Context<'_>,
    view: &ClassView<'_, '_>,
    test_case: bool,
    name: &[u8],
) -> bool {
    let scope = Scope {
        ctx,
        members: view.members,
        comments_from: node_span(&view.node).start,
        path: &view.path,
        // The caller already resolves test-case ancestry, so the syntactic fallback is unused.
        extends: None,
        constructor: view.method(b"__construct"),
        test_case: Some(test_case),
    };
    scope.is_assigned(name)
}

/// What a walk of an initializing method found.
enum Found {
    Continue,
    Stop(bool),
}

enum Receiver<'e, 'a> {
    Object(&'e Expression<'a>),
    Static(&'e Expression<'a>),
}

impl<'a> Scope<'_, '_, 'a> {
    /// `ConstructorAssignDetector::isPropertyAssigned`.
    fn is_assigned(&self, name: &[u8]) -> bool {
        let mut assigned = false;
        for method in self.initializers() {
            // `IS_FIRST_LEVEL_STATEMENT`: the method's own statements, and the expression of each expression
            // statement.
            let first: Vec<Span> = first_level(method)
                .map(|s| match s {
                    Statement::Expression(e) => node_span(unparenthesize(e.expression)),
                    s => node_span(s),
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
        let from = index
            .checked_sub(1)
            .and_then(|i| self.members.get(i))
            .map_or(self.comments_from, |previous| node_span(previous).end);
        // From the class's start, the last non-comment token before its first member is its `{`.
        let from = previous_token_end(self.ctx, from, node_span(m).start);
        let doc = doc_comment(self.ctx.source, from, node_span(m).start);
        let tagged = doc.is_some_and(|d| {
            self.ctx.source[d.start..d.end].lines().any(|line| {
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
        // A nested if/else does not guarantee the assignment; the outer condition can skip it.
        if let Node::If(r#if) = node
            && first.contains(&node_span(r#if))
            && if_else_assign(r#if, name, self.path, self.ctx)
        {
            return Found::Stop(!in_loop);
        }
        if let Node::Assignment(a) = node
            && let Some(a) = assigns_node(a, name, self.path, self.ctx)
        {
            // Is the property's own value merged into the assignment?
            if uses_fetch(self.ctx.source, Node::Expression(a.rhs), a.lhs) {
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
                if matches!(child, Node::Class(_) | Node::AnonymousClass(_)) {
                    let mut path = self.path.to_vec();
                    path.push(child);
                    let scope = Scope {
                        path: &path,
                        ..*self
                    };
                    found = scope.walk(child, name, first, in_loop);
                } else {
                    found = self.walk(child, name, first, in_loop);
                }
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
            let (selector, own, receiver) = match unparenthesize(e.expression) {
                Expression::Call(Call::Method(call)) => (
                    &call.method,
                    is_this(call.object),
                    Receiver::Object(call.object),
                ),
                Expression::Call(Call::StaticMethod(call)) => (
                    &call.method,
                    local_static_class(call.class, self.path, true),
                    Receiver::Static(call.class),
                ),
                _ => return false,
            };
            let ClassLikeMemberSelector::Identifier(called) = selector else {
                return false;
            };
            if own && let Some(m) = method(self.members, called.value) {
                return first_level(m).any(|s| {
                    matches!(s, Statement::Expression(e)
                        if matches!(unparenthesize(e.expression), Expression::Assignment(a)
                            if matches!(a.operator, AssignmentOperator::Assign(_)) && is_fetch(a.lhs, name, self.path, self.ctx)))
                });
            }
            self.reflected_method_assigns(receiver, called.value, name)
        })
    }

    /// `AstResolver::resolveClassMethodFromCall`, followed by `isPropertyAssignFoundInClassMethod`.
    fn reflected_method_assigns(
        &self,
        receiver: Receiver<'_, '_>,
        called: &[u8],
        name: &[u8],
    ) -> bool {
        let facts = facts(self.ctx);
        let own = ClassView::of(
            &facts,
            self.path[self.path.len() - 1],
            &self.path[..self.path.len() - 1],
        )
        .expect("the constructor belongs to a class");
        let codebase = facts.documented();
        let receiver = with_classes(codebase, || match receiver {
            Receiver::Object(object) => expr_type(&facts, self.path, unparenthesize(object), false),
            Receiver::Static(class) => match unparenthesize(class) {
                Expression::Self_(_) | Expression::Static(_) => {
                    Type::Object(own.name.clone(), vec![], false)
                }
                Expression::Parent(_) => match codebase
                    .get_class_like(own.name.as_bytes())
                    .and_then(|class| class.direct_parent_class)
                {
                    Some(parent) => Type::Object(
                        String::from_utf8_lossy(parent.as_bytes()).into_owned(),
                        vec![],
                        false,
                    ),
                    None => Type::Mixed { explicit: false },
                },
                Expression::Identifier(id) => {
                    let written = id.value();
                    let name = if written
                        .get(..10)
                        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(b"namespace\\"))
                    {
                        let local = String::from_utf8_lossy(&written[10..]);
                        match scope_statements(self.path).0 {
                            Some(namespace) => format!("{namespace}\\{local}"),
                            None => local.into_owned(),
                        }
                    } else {
                        resolve_class_name(self.path, id)
                    };
                    Type::Object(name, vec![], false)
                }
                other => expr_type(&facts, self.path, other, false),
            },
        });
        let names = receiver.object_class_names();
        let [receiver] = names.as_slice() else {
            return false;
        };
        let Some((declaring, method)) = codebase
            .get_declaring_method_class(receiver.as_bytes(), called)
            .and_then(|class| codebase.get_class_like(class.as_bytes()))
            .zip(codebase.get_declaring_method(receiver.as_bytes(), called))
        else {
            return false;
        };
        if !declaring.kind.is_class()
            || !codebase.is_instance_of(own.name.as_bytes(), declaring.name.as_bytes())
        {
            return false;
        }
        let Some((source, path)) = facts
            .source_of(method.span.file_id)
            .zip(path_of(method.span.file_id))
        else {
            return false;
        };
        let ctx = Context::new(&source, std::path::Path::new(path.as_ref()), self.ctx.php_version);
        let body_facts = facts::facts(&ctx);
        let arena = mago_allocator::LocalArena::new();
        let file = mago_database::file::File::new(
            std::borrow::Cow::Owned(path.as_bytes().to_vec()),
            mago_database::file::FileType::Host,
            None,
            std::borrow::Cow::Owned(source.as_bytes().to_vec()),
        );
        let program = mago_syntax::parser::parse_file(&arena, &file);
        let mut found = false;
        super::class_method::walk(Node::Program(program), &mut |node, path| {
            if found {
                return false;
            }
            if !matches!(node, Node::Class(_) | Node::AnonymousClass(_)) {
                return true;
            }
            let view = ClassView::of(&body_facts, node, path).expect("the declaration is a class");
            if !view
                .name
                .as_bytes()
                .eq_ignore_ascii_case(declaring.original_name.as_bytes())
            {
                return true;
            }
            found = view.method(method.original_name.as_bytes()).is_some_and(|m| {
                first_level(m).any(|s| matches!(s, Statement::Expression(e)
                    if matches!(unparenthesize(e.expression), Expression::Assignment(a)
                        if matches!(a.operator, AssignmentOperator::Assign(_)) && is_fetch(a.lhs, name, &view.path, &ctx))))
            });
            false
        });
        found
    }
}

fn assigns_node<'a>(
    a: &'a Assignment<'a>,
    name: &[u8],
    path: &[Node<'_, '_>],
    ctx: &Context<'_>,
) -> Option<&'a Assignment<'a>> {
    if !matches!(a.operator, AssignmentOperator::Assign(_)) {
        return None;
    }
    let target = match unparenthesize(a.lhs) {
        Expression::ArrayAccess(access) => access.array,
        Expression::ArrayAppend(append) => append.array,
        lhs => lhs,
    };
    is_fetch(target, name, path, ctx).then_some(a)
}

/// `isIfElseAssign`: an `if` with an `else` and no `elseif` whose branches both assign the property in a list of
/// expression statements only.
fn if_else_assign(r#if: &If<'_>, name: &[u8], path: &[Node<'_, '_>], ctx: &Context<'_>) -> bool {
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
    assigned_in(&stmts, name, path, ctx) && assigned_in(&else_stmts, name, path, ctx)
}

fn body_statements<'a>(statement: &'a Statement<'a>) -> Vec<&'a Statement<'a>> {
    match statement {
        Statement::Block(block) => block.statements.iter().collect(),
        statement => vec![statement],
    }
}

/// `isAssignedInStmts`.
fn assigned_in(
    stmts: &[&Statement<'_>],
    name: &[u8],
    path: &[Node<'_, '_>],
    ctx: &Context<'_>,
) -> bool {
    let mut assigned = false;
    for statement in stmts {
        match statement {
            Statement::Expression(e) => {
                if assigns(e.expression, name, path, ctx).is_some() {
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
fn assigned_via_array_key(
    node: Node<'_, '_>,
    name: &[u8],
    path: &[Node<'_, '_>],
    ctx: &Context<'_>,
) -> bool {
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
        if dim && is_fetch(target, name, path, ctx) {
            return true;
        }
    }
    let mut found = false;
    node.visit_children(|child| {
        if !found {
            if matches!(child, Node::Class(_) | Node::AnonymousClass(_)) {
                let mut path = path.to_vec();
                path.push(child);
                found = assigned_via_array_key(child, name, &path, ctx);
            } else {
                found = assigned_via_array_key(child, name, path, ctx);
            }
        }
    });
    found
}
