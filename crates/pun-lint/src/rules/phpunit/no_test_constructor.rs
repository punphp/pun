//! `lint/phpunit/no-test-constructor`, ported from rector-phpunit's `ConstructClassMethodToSetUpTestCaseRector`
//! (rules/CodeQuality/Rector/Class_) and `SetUpMethodDecorator`.
//!
//! The constructor of a test class named `*Test` or `*TestCase` whose parent is `PHPUnit\Framework\TestCase` and
//! whose parameters its body does not use (a `__construct()` static call is not looked into) moves to `setUp()`, without
//! its `parent::__construct(...)` statements: with no `setUp()` it is renamed `setUp`, loses its parameters, becomes
//! protected and takes the return type `TestCase::setUp()` declares; otherwise it goes and its statements are added to
//! the end of `setUp()`.
//!
//! `SetUpMethodDecorator` does nothing while PHPUnit runs, so rector-phpunit's own fixtures show no return type; Pun
//! does what Rector does outside its test suite (Oracle/ holds those two fixtures with Rector's real output).
//! Upstream's FixtureInnerClass/skip_inner_class.php.inc is in Fixture/.

use mago_codex::ttype::TType;
use mago_syntax::cst::{
    ClassLikeMember, ClassLikeMemberSelector, Expression, Method, MethodBody, Modifier, Node,
    NodeKind, Statement,
};
use pun_problem::{Edit, Safety, Span};

use super::{TEST_CASE, class_parents, is_in_test_class, own_class, remove_class_members, report};
use crate::rules::rector_batch5::{merge, remove_statements};
use crate::rules::rector_stmts::{
    List, block_list, inserted_statements, line_indent, reindent, tokens,
};
use crate::rules::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoTestConstructor;

impl Rule for NoTestConstructor {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-test-constructor"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Class(class) = node else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let Some(name) = own_class(&scope) else {
            return;
        };
        if !name.ends_with("Test") && !name.ends_with("TestCase") {
            return;
        }
        let members = &class.members;
        let Some((at, constructor)) = members.iter().enumerate().find_map(|(at, m)| match m {
            ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
                Some((at, m))
            }
            _ => None,
        }) else {
            return;
        };
        // `shouldSkip`
        let facts = facts(ctx);
        let parent_is_test_case = class_parents(ctx, &name).first().is_some_and(|p| {
            facts.codebase.get_class_like(p.as_bytes()).is_some() && p == TEST_CASE
        });
        if !parent_is_test_case || uses_param(constructor) {
            return;
        }
        let MethodBody::Concrete(block) = &constructor.body else {
            return;
        };
        let source = ctx.source;
        let list = block_list(source, block);
        let removed: Vec<bool> = list.stmts.iter().map(|s| is_parent_construct(s)).collect();
        let kept = List {
            stmts: list
                .stmts
                .iter()
                .zip(&removed)
                .filter(|(_, r)| !**r)
                .map(|(s, _)| *s)
                .collect(),
            nop: list.nop,
            frame: list.frame,
            inner: list.inner,
        };
        let set_up = members.iter().find_map(|m| match m {
            ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"setUp") => Some(m),
            _ => None,
        });
        let edits = match set_up {
            None => renamed_to_set_up(ctx, constructor, &list, &kept, &removed),
            Some(set_up) => {
                let mut only = vec![false; members.len()];
                only[at] = true;
                let mut edits = remove_class_members(source, node, &only);
                if let Some(edit) = appended(source, set_up, &kept) {
                    edits.push(edit);
                }
                merge(edits)
            }
        };
        out.push(report(
            node_span(&constructor.name),
            "Move the test's constructor to `setUp()`.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// Whether the constructor's body uses one of its parameters (a variable of the same name, case-insensitive), outside
/// static calls named `__construct`.
fn uses_param(method: &Method<'_>) -> bool {
    let params: Vec<&[u8]> = method
        .parameter_list
        .parameters
        .iter()
        .map(|p| p.variable.name)
        .collect();
    let MethodBody::Concrete(block) = &method.body else {
        return false;
    };
    let mut stack = Vec::new();
    Node::Block(block).visit_children(|c| stack.push(c));
    while let Some(node) = stack.pop() {
        match node {
            Node::StaticMethodCall(call) if matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"__construct")) =>
            {
                continue;
            }
            Node::DirectVariable(v) if params.iter().any(|p| p.eq_ignore_ascii_case(v.name)) => {
                return true;
            }
            _ => {}
        }
        node.visit_children(|c| stack.push(c));
    }
    false
}

/// `isParentCallNamed($stmt->expr, '__construct')` on an expression statement.
fn is_parent_construct(statement: &Statement<'_>) -> bool {
    let Statement::Expression(e) = statement else {
        return false;
    };
    let Expression::Call(mago_syntax::cst::Call::StaticMethod(call)) = e.expression else {
        return false;
    };
    matches!(call.class, Expression::Parent(_))
        && matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(b"__construct"))
}

/// The return type `SetUpMethodDecorator` gives the new `setUp()`: `Some(None)` when `TestCase::setUp()` declares
/// none or not a simple name, `None` when the codebase does not have it (the method keeps its own).
fn set_up_return_type(ctx: &Context<'_>) -> Option<Option<String>> {
    let facts = facts(ctx);
    let method = facts
        .codebase
        .get_method(TEST_CASE.to_ascii_lowercase().as_bytes(), b"setup")?;
    Some(
        method
            .return_type_declaration_metadata
            .as_ref()
            .and_then(|t| {
                let id = String::from_utf8_lossy(t.type_union.get_id().as_bytes()).into_owned();
                (!id.contains(['|', '&', '?', '<'])).then_some(id)
            }),
    )
}

/// The edits for renaming the constructor `setUp()` without parameters, protected, with `kept` as its statements.
fn renamed_to_set_up(
    ctx: &Context<'_>,
    method: &Method<'_>,
    list: &List<'_>,
    kept: &List<'_>,
    removed: &[bool],
) -> Vec<Edit> {
    let source = ctx.source;
    let return_type = set_up_return_type(ctx);
    let final_ = method
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Final(_)));
    let abstract_ = method
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Abstract(_)));
    let static_ = method
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Static(_)));
    let modifiers = format!(
        "{}{}protected {}",
        if final_ { "final " } else { "" },
        if abstract_ { "abstract " } else { "" },
        if static_ { "static " } else { "" }
    );
    let all_removed = !list.stmts.is_empty() && kept.stmts.is_empty() && !list.nop;
    if !method.parameter_list.parameters.is_empty() || all_removed {
        let span = node_span(method);
        let indent = line_indent(source, span.start);
        let mut text = String::new();
        for attributes in method.attribute_lists.iter() {
            let s = node_span(attributes);
            text.push_str(&source[s.start..s.end]);
            text.push('\n');
            text.push_str(indent);
        }
        text.push_str(&modifiers);
        text.push_str("function ");
        if method.ampersand.is_some() {
            text.push('&');
        }
        text.push_str("setUp()");
        if let Some(Some(r#type)) = &return_type {
            text.push_str(": ");
            text.push_str(r#type);
        }
        text.push('\n');
        text.push_str(indent);
        text.push_str(&super::block_anew(source, kept, indent));
        return vec![Edit::new(span, text)];
    }
    let mut edits = Vec::new();
    let function = method.function.span.start.offset as usize;
    let first = method
        .modifiers
        .iter()
        .next()
        .map_or(function, |m| node_span(m).start);
    edits.push(Edit::new(Span::new(first, function), modifiers));
    edits.push(Edit::new(node_span(&method.name), "setUp"));
    if let Some(Some(r#type)) = &return_type {
        let end = method.parameter_list.right_parenthesis.end.offset as usize;
        edits.push(Edit::new(Span::new(end, end), format!(": {type}")));
    }
    if removed.contains(&true) {
        edits.extend(remove_statements(source, list, removed));
    }
    merge(edits)
}

/// The edit for adding the statements of `kept` to the end of `set_up`'s body, as php-parser inserts them: on lines
/// of their own after the last statement, or, in an empty body, with the method printed anew.
fn appended(source: &str, set_up: &Method<'_>, kept: &List<'_>) -> Option<Edit> {
    if kept.is_empty() {
        return None;
    }
    let MethodBody::Concrete(block) = &set_up.body else {
        return None;
    };
    let list = block_list(source, block);
    let indent = line_indent(source, node_span(set_up).start);
    let inner = format!("{indent}    ");
    let end = list
        .nop_span(source)
        .map(|s| s.end)
        .or_else(|| list.stmts.last().map(|s| node_span(*s).end));
    match end {
        Some(end) => Some(Edit::new(
            Span::new(end, end),
            format!(
                "\n{inner}{}",
                inserted_statements(source, kept, &inner).trim_end_matches('\n')
            ),
        )),
        None => {
            let open = block.left_brace.start.offset as usize;
            let close = block.right_brace.end.offset as usize;
            let mut lines = Vec::new();
            for (i, statement) in kept.stmts.iter().enumerate() {
                let span = node_span(*statement);
                for t in tokens(source, kept.before(source, i), span.start)
                    .into_iter()
                    .filter(|t| t.kind.is_comment())
                {
                    lines.push(source[t.start..t.end].to_string());
                }
                lines.push(reindent(
                    source,
                    span,
                    line_indent(source, span.start),
                    &inner,
                ));
            }
            Some(Edit::new(
                Span::new(open, close),
                format!(
                    "{{\n{inner}{}\n{indent}}}",
                    lines.join(&format!("\n{inner}"))
                ),
            ))
        }
    }
}
