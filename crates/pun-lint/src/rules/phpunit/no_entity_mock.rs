//! `lint/phpunit/no-entity-mock`, ported from rector-phpunit's `EntityDocumentCreateMockToDirectNewRector`
//! (rules/CodeQuality/Rector/ClassMethod), with `AssignedMocksCollector::collectEntityClasses` and
//! `DoctrineEntityDocumentAnalyser`.
//!
//! In any class method (Rector does not ask for a test class), a top-level `$x = $any->createMock(X::class)` of an
//! existing, non-abstract class whose doc comment contains `@Document`, `@ORM\Document`, `@Entity` or `@ORM\Entity`
//! becomes `$x = new \X()`. Each top-level statement that is a method call with a call on `$x` inside: a
//! `->method('setFoo')` one goes; a `->method('getFoo')->willReturn($v)` one becomes `$x->setFoo($v)` (an `isFoo`
//! one `setIsFoo` or `setFoo` when the class has such a method, any other name stays). Then every `$x` in the method
//! loses `Mock` from its name.
//!
//! The doc comment is php-parser's (`getDocComment()`): the last `/** */` among the comments between the token before
//! the class and the class's first token (its first attribute), read from the class's file ([`Facts::source_of`]).
//! The `is` setter is looked up with docblocks read, so a `@method` tag counts, as in PHPStan's `hasMethod`.
//!
//! Where Rector fails on the file (a statement with a call on `$x` but no `->method('literal')`, or a `createMock()`
//! assigned to another spelling of `$x` that is no entity mock), Pun leaves the method alone. A class name written
//! with a leading `\` in a string, which Rector prints as `new \\X()`, is written `new \X()`.

use mago_codex::symbol::SymbolKind;
use mago_syntax::cst::{
    Call, ClassLikeMemberSelector, Expression, Literal, MethodBody, Node, NodeKind,
    PartialApplication, StringPart, Variable,
};
use mago_syntax::token::TokenKind;
use pun_problem::{Edit, Safety, Span};

use super::{class_string, report};
use crate::rules::class_method::walk;
use crate::rules::php_parser::{previous_token_end, tokens};
use crate::rules::rector_batch5::remove_statements;
use crate::rules::rector_stmts::{assign, block_list, expression};
use crate::rules::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoEntityMock;

const ENTITY_DOCBLOCK_MARKERS: [&str; 4] =
    ["@Document", "@ORM\\Document", "@Entity", "@ORM\\Entity"];

/// A top-level statement as the passes leave it.
#[derive(Clone)]
enum State<'a> {
    Original,
    Removed,
    /// `$stmt->expr` set to `$var->method(arg)`.
    Replaced {
        var: String,
        method: String,
        arg: &'a Expression<'a>,
    },
}

/// What `resolveMethodCallFirstArgValue` gives.
enum FirstArg<'a> {
    /// No such call, or one without arguments.
    Null,
    Str(String),
    Expr(&'a Expression<'a>),
}

/// A found method call's first argument's value: `None` for a call without arguments, and for a first-class callable
/// (php-parser's `getArgs()` asserts, and assertions are off, so Rector reads `value` of its `VariadicPlaceholder`).
type Args<'a> = Option<&'a Expression<'a>>;

/// `NodeFinder::findFirst`, pre-order, for a method call (or first-class callable, a method call to php-parser)
/// whose object's variable name and method name `found` accepts.
fn find_call<'a>(
    node: Node<'a, 'a>,
    found: &dyn Fn(Option<&[u8]>, Option<&[u8]>) -> bool,
) -> Option<Args<'a>> {
    let name = |m: &'a ClassLikeMemberSelector<'a>| match m {
        ClassLikeMemberSelector::Identifier(id) => Some(id.value),
        _ => None,
    };
    let call = match node {
        Node::MethodCall(m) => Some((
            m.object,
            name(&m.method),
            m.argument_list.arguments.first().map(super::arg_value),
        )),
        Node::MethodPartialApplication(m) => Some((m.object, name(&m.method), None)),
        _ => None,
    };
    if let Some((object, method, args)) = call
        && found(variable_name(object), method)
    {
        return Some(args);
    }
    let mut hit = None;
    node.visit_children(|child| {
        if hit.is_none() {
            hit = find_call(child, found);
        }
    });
    hit
}

/// [`find_call`] in a statement's expression as the passes left it.
fn find_in<'a>(
    state: &State<'a>,
    expr: &'a Expression<'a>,
    found: &dyn Fn(Option<&[u8]>, Option<&[u8]>) -> bool,
) -> Option<Args<'a>> {
    match state {
        State::Replaced { var, method, arg } => {
            if found(Some(var.as_bytes()), Some(method.as_bytes())) {
                return Some(Some(arg));
            }
            find_call(Node::Expression(arg), found)
        }
        _ => find_call(Node::Expression(expr), found),
    }
}

/// A direct variable's name, without the `$`.
fn variable_name<'a>(expr: &'a Expression<'a>) -> Option<&'a [u8]> {
    match unparenthesize(expr) {
        Expression::Variable(Variable::Direct(v)) => Some(&v.name[1..]),
        _ => None,
    }
}

/// `resolveMethodCallFirstArgValue` on what [`find_call`] found; `None` for a string literal that is not UTF-8.
fn first_arg_value(found: Option<Args<'_>>) -> Option<FirstArg<'_>> {
    let Some(Some(value)) = found else {
        return Some(FirstArg::Null);
    };
    Some(match unparenthesize(value) {
        Expression::Literal(Literal::String(s)) => {
            FirstArg::Str(String::from_utf8(s.value?.to_vec()).ok()?)
        }
        _ => FirstArg::Expr(value),
    })
}

/// The variable name without `Mock`, as `str_replace('Mock', '', $name)`.
fn without_mock(name: &str) -> String {
    name.replace("Mock", "")
}

/// PHP's `ucfirst`.
fn ucfirst(s: &str) -> String {
    let mut out = s.to_string();
    if let Some(first) = out.get_mut(..1) {
        first.make_ascii_uppercase();
    }
    out
}

/// `resolveMethodName`: the setter a getter's or an `is` method's stub stands for.
fn resolve_method_name(facts: &Facts, name: &str, class: &str) -> String {
    if let Some(rest) = name.strip_prefix("get") {
        return format!("set{}", ucfirst(rest));
    }
    if let Some(rest) = name.strip_prefix("is") {
        for setter in [format!("set{}", ucfirst(name)), format!("set{rest}")] {
            if facts
                .documented()
                .method_exists(lookup(class).as_bytes(), setter.as_bytes())
            {
                return setter;
            }
        }
    }
    name.to_string()
}

impl Rule for NoEntityMock {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-entity-mock"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else { return };
        let MethodBody::Concrete(block) = &method.body else {
            return;
        };
        let list = block_list(ctx.source, block);
        let facts = facts(ctx);
        let entities = collect_entity_classes(ctx, &facts, ancestors, &list.stmts);
        if entities.is_empty() {
            return;
        }
        let mut states = vec![State::Original; list.stmts.len()];
        for (name, class) in &entities {
            for (at, statement) in list.stmts.iter().enumerate() {
                if matches!(states[at], State::Removed) {
                    continue;
                }
                let Some(expr) = expression(statement) else {
                    continue;
                };
                let is_call = matches!(
                    expr,
                    Expression::Call(Call::Method(_))
                        | Expression::PartialApplication(PartialApplication::Method(_))
                );
                if matches!(states[at], State::Original) && !is_call {
                    continue;
                }
                let state = states[at].clone();
                let on_variable = |var: Option<&[u8]>, _: Option<&[u8]>| {
                    var.is_some_and(|v| v.eq_ignore_ascii_case(name.as_bytes()))
                };
                if find_in(&state, expr, &on_variable).is_none() {
                    continue;
                }
                let named = |wanted: &'static str| {
                    move |_: Option<&[u8]>, n: Option<&[u8]>| {
                        n.is_some_and(|n| n.eq_ignore_ascii_case(wanted.as_bytes()))
                    }
                };
                // 2. find $mock->method("name")
                let Some(FirstArg::Str(method_name)) =
                    first_arg_value(find_in(&state, expr, &named("method")))
                else {
                    // Rector: \"Unable to resolve method name\"
                    return;
                };
                // is set method mocked? Just remove it
                let mut removed = false;
                if method_name.starts_with("set") {
                    states[at] = State::Removed;
                    removed = true;
                }
                let setter = resolve_method_name(&facts, &method_name, class);
                // a string (UTF-8 or not) is no expression to set
                if let Some(FirstArg::Expr(arg)) =
                    first_arg_value(find_in(&state, expr, &named("willReturn")))
                    && !setter.is_empty()
                    && setter != "0"
                    && !removed
                {
                    states[at] = State::Replaced {
                        var: name.clone(),
                        method: setter,
                        arg,
                    };
                }
            }
        }
        let names: Vec<&str> = entities.iter().map(|(n, _)| n.as_str()).collect();
        let mut edits: Vec<Edit> = Vec::new();
        // spans whose variables are not renamed on their own: what goes or is written anew
        let mut covered: Vec<Span> = Vec::new();
        let removed: Vec<bool> = states.iter().map(|s| matches!(s, State::Removed)).collect();
        for (at, statement) in list.stmts.iter().enumerate() {
            if removed[at] {
                covered.push(node_span(*statement));
            }
        }
        // replace mock assigns with direct new
        for (at, statement) in list.stmts.iter().enumerate() {
            if !matches!(states[at], State::Original) {
                continue;
            }
            let Some(assignment) = expression(statement).and_then(assign) else {
                continue;
            };
            let rhs = unparenthesize(assignment.rhs);
            let selector = match rhs {
                Expression::Call(Call::Method(call)) => &call.method,
                Expression::PartialApplication(PartialApplication::Method(call)) => &call.method,
                _ => continue,
            };
            let is_create_mock = matches!(selector, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"createMock"));
            let Some(var) = variable_name(assignment.lhs) else {
                continue;
            };
            if !is_create_mock || !names.iter().any(|n| var.eq_ignore_ascii_case(n.as_bytes())) {
                continue;
            }
            // Rector reads the class by the exact name and fails on another spelling
            let Some((_, class)) = entities.iter().find(|(n, _)| n.as_bytes() == var) else {
                return;
            };
            let span = node_span(rhs);
            covered.push(span);
            edits.push(Edit::new(span, format!("new \\{}()", lookup(class))));
        }
        let renames = renames(Node::Method(method), &names);
        let renamed = |span: Span| -> String {
            let mut text = String::new();
            let mut at = span.start;
            for (s, new) in renames
                .iter()
                .filter(|(s, _)| s.start >= span.start && s.end <= span.end)
            {
                text.push_str(&ctx.source[at..s.start]);
                text.push_str(new);
                at = s.end;
            }
            text.push_str(&ctx.source[at..span.end]);
            text
        };
        for (at, statement) in list.stmts.iter().enumerate() {
            if let State::Replaced { var, method, arg } = &states[at] {
                let span = node_span(expression(statement).unwrap());
                covered.push(span);
                edits.push(Edit::new(
                    span,
                    format!(
                        "${}->{method}({})",
                        without_mock(var),
                        renamed(node_span(unparenthesize(arg)))
                    ),
                ));
            }
        }
        for (span, new) in &renames {
            if !covered
                .iter()
                .any(|c| c.start <= span.start && span.end <= c.end)
            {
                edits.push(Edit::new(*span, new.clone()));
            }
        }
        edits.extend(remove_statements(ctx.source, &list, &removed));
        out.push(report(
            node_span(&method.name),
            "Use the entity class directly instead of a mock of it.",
            Safety::Safe,
            edits,
        ));
    }
}

/// The variables of `root` that `traverseNodesWithCallable` renames: each php-parser `Variable` named one of `names`
/// (exactly), with its new text, in source order. A variable that stands alone in a string (`"$a"`, `"${a}"`) is
/// printed anew as `{$a}`; a property's name (`public $a`, `A::$a`) is no `Variable` to php-parser.
fn renames(root: Node<'_, '_>, names: &[&str]) -> Vec<(Span, String)> {
    let mut skip: Vec<Span> = Vec::new();
    let mut found: Vec<(Span, String)> = Vec::new();
    walk(root, &mut |node, _| {
        match node {
            Node::StaticPropertyAccess(access) => skip.push(node_span(&access.property)),
            Node::PropertyAbstractItem(item) => skip.push(node_span(&item.variable)),
            Node::PropertyConcreteItem(item) => skip.push(node_span(&item.variable)),
            Node::StringPart(StringPart::Expression(expr)) => {
                let name = match expr {
                    Expression::Variable(Variable::Direct(v)) => Some(&v.name[1..]),
                    Expression::Variable(Variable::Indirect(v)) => match v.expression {
                        // `"${a}"`: php-parser's `Variable` named `a`
                        Expression::Identifier(id) => Some(id.value()),
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(name) = name
                    && let Some(n) = names.iter().find(|n| n.as_bytes() == name)
                {
                    let span = node_span(*expr);
                    skip.push(span);
                    found.push((span, format!("{{${}}}", without_mock(n))));
                }
            }
            Node::DirectVariable(v) => {
                let span = node_span(v);
                if !skip.contains(&span)
                    && let Some(n) = names.iter().find(|n| n.as_bytes() == &v.name[1..])
                {
                    found.push((span, format!("${}", without_mock(n))));
                }
            }
            _ => {}
        }
        true
    });
    found.sort_by_key(|(s, _)| s.start);
    found
}

/// `AssignedMocksCollector::collectEntityClasses`: (variable name, class) of each top-level assignment of a
/// `createMock()` of an existing, non-abstract entity class, by first assignment, the last class of each.
fn collect_entity_classes(
    ctx: &Context<'_>,
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    statements: &[&mago_syntax::cst::Statement<'_>],
) -> Vec<(String, String)> {
    let mut found: Vec<(String, String)> = Vec::new();
    for statement in statements {
        let Some(assignment) = expression(statement).and_then(assign) else {
            continue;
        };
        let Some(name) = variable_name(assignment.lhs) else {
            continue;
        };
        let Expression::Call(Call::Method(call)) = unparenthesize(assignment.rhs) else {
            continue;
        };
        let is_create_mock = matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"createMock"));
        let Some(first) = call.argument_list.arguments.first() else {
            continue;
        };
        if !is_create_mock {
            continue;
        }
        let Some(class) = class_string(ctx, ancestors, super::arg_value(first)) else {
            continue;
        };
        let Some(metadata) = facts.codebase.get_class_like(lookup(&class).as_bytes()) else {
            continue;
        };
        // these cannot be instantiated
        if metadata.kind == SymbolKind::Class && metadata.flags.is_abstract() {
            continue;
        }
        let name = String::from_utf8_lossy(name).into_owned();
        match found.iter_mut().find(|(n, _)| *n == name) {
            Some(entry) => entry.1 = class,
            None => found.push((name, class)),
        }
    }
    found.retain(|(_, class)| is_entity_class(facts, class));
    found
}

/// A class name as PHPStan's reflection looks it up: without a leading `\`.
fn lookup(class: &str) -> &str {
    class.strip_prefix('\\').unwrap_or(class)
}

/// `DoctrineEntityDocumentAnalyser::isEntityClass`.
fn is_entity_class(facts: &Facts, class: &str) -> bool {
    // a class without a source file (a built-in one) has no doc comment
    let Some((metadata, source)) = facts
        .codebase
        .get_class_like(lookup(class).as_bytes())
        .and_then(|m| Some((m, facts.source_of(m.span.file_id)?)))
    else {
        return false;
    };
    let start = metadata.span.start.offset as usize;
    let all = tokens(&source);
    let from = previous_token_end(&source, &all, start);
    all.iter()
        .filter(|t| t.start >= from && t.end <= start && t.kind == TokenKind::DocBlockComment)
        .last()
        .is_some_and(|doc| {
            let doc = &source[doc.start..doc.end];
            ENTITY_DOCBLOCK_MARKERS.iter().any(|m| doc.contains(m))
        })
}
