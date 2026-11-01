//! `lint/phpunit/add-instanceof-assert` and `lint/phpunit/add-instanceof-assert-for-argument`, ported from
//! rector-phpunit's `AddInstanceofAssertForNullableInstanceRector` and `AddInstanceofAssertForNullableArgumentRector`
//! (rules/CodeQuality/Rector/ClassMethod), with `NullableObjectAssignCollector`, `MethodCallParameterTypeResolver` and
//! `AssertMethodCallFactory::createAssertInstanceOf()`.
//!
//! In a test class, a method's (or a `foreach`'s) statement list of two or more statements, all of them expression
//! statements, collects each `$x = $any->call()` whose type without `null` is one class (an `ObjectType`). Before the
//! first statement that uses such an `$x` whose type there is `X|null` (exactly two types), as the receiver of a
//! method call (`add-instanceof-assert`, every such variable of the statement) or as an argument of a method call
//! whose parameter there is a class (`add-instanceof-assert-for-argument`, the first such call, assert calls aside),
//! goes `$this->assertInstanceOf(\X::class, $x);`, once for each collected assignment.
//!
//! Mago does not type a call that returns a class no file declares as that class, as PHPStan does, so Pun collects
//! no such assignment (tests/rector/AddInstanceofAssertForNullableInstanceRector/Pun/unknown_class).
//!
//! Rector prints the new statement after the one before it, on a line of its own at that statement's indent (in
//! spaces, a tab counting as 4), or before the list's first statement and its comments.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Call, ClassLikeMemberSelector, Expression, ForeachBody, MethodBody, Node, NodeKind, Statement,
    Variable,
};
use pun_problem::{Edit, Safety, Span};

use super::is_in_test_class;
use crate::rules::rector_stmts::{
    List, assign, block_list, body_list, expression, first_comment, holder_list, line_indent,
};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const MESSAGE: &str = "Assert the instance of a nullable object before using it.";

pub struct AddInstanceofAssert;

impl Rule for AddInstanceofAssert {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-instanceof-assert"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method, NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        run(node, ancestors, ctx, out, &|stmt, collection| {
            matched_receivers(ctx, stmt, collection)
        });
    }
}

pub struct AddInstanceofAssertForArgument;

impl Rule for AddInstanceofAssertForArgument {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-instanceof-assert-for-argument"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method, NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        run(node, ancestors, ctx, out, &|stmt, collection| {
            matched_argument(ctx, stmt, collection)
                .into_iter()
                .collect()
        });
    }
}

/// A collected assignment: the variable's name and its class (`VariableNameToType`).
type Collected = (Vec<u8>, String);

/// `refactor()`: `matched` gives the indices in the collection of the variables a statement asserts.
fn run(
    node: Node<'_, '_>,
    ancestors: &[Node<'_, '_>],
    ctx: &Context<'_>,
    out: &mut Vec<Report>,
    matched: &dyn Fn(&Statement<'_>, &[Collected]) -> Vec<usize>,
) {
    let list = match node {
        Node::Method(method) => {
            let MethodBody::Concrete(block) = &method.body else {
                return;
            };
            if !is_in_test_class(ctx, ancestors, Some(method)) {
                return;
            }
            block_list(ctx.source, block)
        }
        // a `foreach`, the other node kind of `targets()`
        _ => {
            let Node::Foreach(foreach) = node else { return };
            if !is_in_test_class(ctx, ancestors, None) {
                return;
            }
            match &foreach.body {
                ForeachBody::Statement(statement) => body_list(ctx.source, statement),
                ForeachBody::ColonDelimited(body) => {
                    let mut scope = ancestors.to_vec();
                    scope.push(node);
                    holder_list(ctx.source, Node::ForeachColonDelimitedBody(body), &scope).unwrap()
                }
            }
        }
    };
    if list.len() < 2 {
        return;
    }
    let mut collection = collect(ctx, &list);
    let mut edits = Vec::new();
    for (i, statement) in list.stmts.iter().enumerate() {
        let mut found = matched(statement, &collection);
        if found.is_empty() {
            continue;
        }
        let asserts: Vec<&Collected> = found.iter().map(|&index| &collection[index]).collect();
        edits.push(insert_before(ctx.source, &list, i, &asserts));
        // from now on, the variable is not nullable, remove to avoid double asserts
        found.sort_unstable();
        for index in found.into_iter().rev() {
            collection.remove(index);
        }
    }
    let Some(first) = edits.first().map(|e: &Edit| e.span) else {
        return;
    };
    out.push(Report::fixed(first, MESSAGE, Safety::Safe, edits));
}

/// `NullableObjectAssignCollector::collect()`: nothing when a statement is not an expression statement (php-parser's
/// trailing `Nop` of comments included).
fn collect(ctx: &Context<'_>, list: &List<'_>) -> Vec<Collected> {
    if list.nop
        || list
            .stmts
            .iter()
            .any(|s| !matches!(s, Statement::Expression(_)))
    {
        return Vec::new();
    }
    let facts = facts(ctx);
    let mut out = Vec::new();
    for statement in &list.stmts {
        let Some(assignment) = expression(statement).and_then(assign) else {
            continue;
        };
        if !matches!(
            unparenthesize(assignment.rhs),
            Expression::Call(Call::Method(_))
        ) {
            continue;
        }
        let Expression::Variable(Variable::Direct(variable)) = unparenthesize(assignment.lhs)
        else {
            continue;
        };
        // TypeCombinator::removeNull(), an ObjectType
        let rest: Vec<&TAtomic> = (facts.full_type(node_span(assignment)).iter())
            .flat_map(|t| t.types.iter())
            .filter(|a| !matches!(a, TAtomic::Null))
            .collect();
        let class = match rest[..] {
            [TAtomic::Object(TObject::Named(named))]
                if named.intersection_types.is_none() && !named.is_this && !named.is_static =>
            {
                named.name.to_string()
            }
            [TAtomic::Object(TObject::Enum(e))] => e.name.to_string(),
            _ => continue,
        };
        out.push((variable.name[1..].to_vec(), class));
    }
    out
}

/// `SimpleTypeAnalyzer::isNullableType()` for the type of `expr`: two types, one of them `null`.
fn is_nullable(ctx: &Context<'_>, expr: &Expression<'_>) -> bool {
    let facts = facts(ctx);
    facts
        .full_type(node_span(expr))
        .is_some_and(|t| t.types.len() == 2 && t.types.iter().any(|a| matches!(a, TAtomic::Null)))
}

/// `VariableNameToTypeCollection::matchByVariableName()`: the first collected assignment of the name.
fn match_by_name(collection: &[Collected], name: &[u8]) -> Option<usize> {
    collection.iter().position(|(n, _)| n == name)
}

/// The nodes of `statement` in php-parser's traversal order (the node, then its children), until `visit` says stop.
fn traverse<'a>(statement: &'a Statement<'a>, visit: &mut dyn FnMut(Node<'a, 'a>) -> bool) {
    let mut stack = vec![Node::Statement(statement)];
    while let Some(node) = stack.pop() {
        if visit(node) {
            return;
        }
        let mut children = Vec::new();
        node.visit_children(|child| children.push(child));
        stack.extend(children.into_iter().rev());
    }
}

/// `matchNullableVariableNameToTypes()`: the variables, by their collection index, that are the nullable receiver of
/// a method call in `statement`, each once.
fn matched_receivers(
    ctx: &Context<'_>,
    statement: &Statement<'_>,
    collection: &[Collected],
) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    let mut names: Vec<&[u8]> = Vec::new();
    traverse(statement, &mut |node| {
        let Node::MethodCall(call) = node else {
            return false;
        };
        // a variable without a name: Rector reads its name after its type, and returns alike
        let Expression::Variable(Variable::Direct(direct)) = call.object else {
            return false;
        };
        if !is_nullable(ctx, call.object) {
            return false;
        }
        let name = &direct.name[1..];
        // avoid duplicated assert for the same variable used multiple times in one statement
        if names.contains(&name) {
            return false;
        }
        if let Some(index) = match_by_name(collection, name) {
            names.push(name);
            out.push(index);
        }
        false
    });
    out
}

/// `matchedNullableArgumentNameToType()`: the collection index of the variable the first fitting method call passes,
/// nullable, where a class is expected.
fn matched_argument(
    ctx: &Context<'_>,
    statement: &Statement<'_>,
    collection: &[Collected],
) -> Option<usize> {
    let mut matched = None;
    traverse(statement, &mut |node| {
        let Node::MethodCall(call) = node else {
            return false;
        };
        // avoid double null on assert
        if is_assert_method_call(&call.method) {
            return false;
        }
        let parameter_types = parameter_types(ctx, call);
        for (position, arg) in call.argument_list.arguments.iter().enumerate() {
            let value = arg.value();
            let Expression::Variable(variable) = value else {
                continue;
            };
            // a variable without a name: Rector reads its name after its type, and returns alike
            let Variable::Direct(direct) = variable else {
                return false;
            };
            if !is_nullable(ctx, value) {
                return false;
            }
            // should not happen
            let Some(parameter_type) = parameter_types.as_ref().and_then(|t| t.get(position))
            else {
                return false;
            };
            matched = match_by_name(collection, &direct.name[1..]);
            // is object type required?
            return parameter_type.as_ref().is_some_and(is_object_type);
        }
        false
    });
    matched
}

/// `AssertCallAnalyzer::isAssertMethodCall()`: the name starts with an assert prefix, case-sensitively.
fn is_assert_method_call(method: &ClassLikeMemberSelector<'_>) -> bool {
    const PREFIXES: [&[u8]; 6] = [
        b"expectNotToPerformAssertions",
        b"assert",
        b"expectException",
        b"setExpectedException",
        b"expectOutput",
        b"should",
    ];
    matches!(method, ClassLikeMemberSelector::Identifier(id) if PREFIXES.iter().any(|p| id.value.starts_with(p)))
}

/// `MethodCallParameterTypeResolver::resolve()`: the type of each parameter of the called method, docblocks read
/// (`None` for one without a type); `None` without arguments or a method found as `ReflectionResolver` finds it: on
/// the one class of the receiver's type.
fn parameter_types(
    ctx: &Context<'_>,
    call: &mago_syntax::cst::MethodCall<'_>,
) -> Option<Vec<Option<TUnion>>> {
    if call.argument_list.arguments.is_empty() {
        return None;
    }
    let ClassLikeMemberSelector::Identifier(method) = &call.method else {
        return None;
    };
    let facts = facts(ctx);
    let mut classes: Vec<String> = Vec::new();
    for atomic in facts.full_type(node_span(call.object))?.types.iter() {
        object_class_names(atomic, &mut classes);
    }
    let [class] = &classes[..] else {
        return None;
    };
    let codebase = facts.documented();
    codebase.get_class_like(class.as_bytes())?;
    let metadata = codebase.get_declaring_method(class.as_bytes(), method.value)?;
    Some(
        metadata
            .parameters
            .iter()
            .map(|p| p.type_metadata.as_ref().map(|t| t.type_union.clone()))
            .collect(),
    )
}

/// `Type::getObjectClassNames()` of an atomic type, added to `out` once each.
fn object_class_names(atomic: &TAtomic, out: &mut Vec<String>) {
    let mut add = |name: String| {
        if !out.contains(&name) {
            out.push(name);
        }
    };
    match atomic {
        TAtomic::Object(TObject::Named(named)) => {
            add(named.name.to_string());
            for member in named.intersection_types.iter().flat_map(|i| i.iter()) {
                if let TAtomic::Object(TObject::Named(n)) = member {
                    add(n.name.to_string());
                }
            }
        }
        TAtomic::Object(TObject::Enum(e)) => add(e.name.to_string()),
        TAtomic::GenericParameter(parameter) => {
            for a in parameter.constraint.types.iter() {
                object_class_names(a, out);
            }
        }
        _ => {}
    }
}

/// `$type instanceof ObjectType`: one class, a template bound by one class included.
fn is_object_type(t: &TUnion) -> bool {
    match t.types.as_ref() {
        [TAtomic::Object(TObject::Named(named))] => {
            named.intersection_types.is_none() && !named.is_this && !named.is_static
        }
        [TAtomic::Object(TObject::Enum(_))] => true,
        [TAtomic::GenericParameter(parameter)] => is_object_type(&parameter.constraint),
        _ => false,
    }
}

/// The edit that puts the asserts before the statement at `i` of `list`, as php-parser's format-preserving printer
/// inserts statements: each on a line of its own after the statement before, at that statement's indent, or before
/// the first statement and its comments, each followed by a line break to its indent.
fn insert_before(source: &str, list: &List<'_>, i: usize, asserts: &[&Collected]) -> Edit {
    let indent_of = |pos: usize| {
        let width: usize = line_indent(source, pos)
            .bytes()
            .map(|b| if b == b'\t' { 4 } else { 1 })
            .sum();
        format!("\n{}", " ".repeat(width))
    };
    let printed = asserts.iter().map(|(name, class)| {
        format!(
            "$this->assertInstanceOf(\\{class}::class, ${});",
            String::from_utf8_lossy(name)
        )
    });
    if i == 0 {
        let span = node_span(list.stmts[0]);
        let at = first_comment(source, list.before(source, 0), span.start).unwrap_or(span.start);
        let nl = indent_of(span.start);
        let text: String = printed.map(|a| format!("{a}{nl}")).collect();
        return Edit::new(Span::new(at, at), text);
    }
    let previous = node_span(list.stmts[i - 1]);
    let nl = indent_of(previous.start);
    let text: String = printed.map(|a| format!("{nl}{a}")).collect();
    Edit::new(Span::new(previous.end, previous.end), text)
}
