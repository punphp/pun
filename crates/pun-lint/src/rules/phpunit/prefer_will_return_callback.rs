//! `lint/phpunit/prefer-will-return-callback`, ported from rector-phpunit's
//! `VoidMethodWithCallbackToWillReturnCallbackRector` (rules/CodeQuality/Rector/Class_), with
//! `SetUpAssignedMockTypesResolver` and `MethodParametersAndReturnTypesResolver::resolveFromReflection`.
//!
//! In the methods of a test class, `$mock->method('name')->with($this->callback(function (...) { ... }))` (one `with()`
//! argument, a closure) becomes `->willReturnCallback(function (...): void { ... })` when the mocked method natively
//! returns `void` and the closure returns only literals, constants, variables or `throw` (at least one value): each
//! `return value;` becomes `return;`, a nested `return throw new X;` becomes `throw new X;` (one at the closure's top
//! level stays, as Rector's traverser drops a replacement there), and a now bare `return;` that ends the closure goes.
//!
//! The mocked class comes from the type PHPStan gives the receiver of `method()` (or of the `expects()` before it): an
//! intersection such as `MockObject&SomeClass`, which PHPStan reduces as `TypeCombinator::intersect()` does (a final
//! class and `MockObject` make `never`), read from the type Mago gives it. A receiver typed `MockObject` (or `never`)
//! falls back to the class `setUp()` assigns to a property or variable of its name with `createMock()` or
//! `getMockBuilder()`.
//!
//! Rector reprints the closure where the argument was (php-parser's format-preserving printer): lines move by the
//! difference between the argument's indent and the closure's, and a closure left without statements is printed anew
//! around its attributes, parameters and `use` variables as written.
//!
//! Pun leaves alone what it does not print as php-parser does
//! (tests/rector/VoidMethodWithCallbackToWillReturnCallbackRector/Pun): a closure that moves right or across lines
//! indented with tabs, a value in parentheses that stays (Rector writes `return ();`), a `?>` in the closure, a
//! comment before the argument, and a closure printed anew whose attributes, parameters or `use` variables span lines
//! or hold comments, or whose `}` goes at a tab indent.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_span::HasSpan;
use mago_syntax::cst::{
    Access, Argument, Call, ClassLikeMember, ClassLikeMemberSelector, Closure, CompositeString,
    Expression, Literal, MethodBody, MethodCall, Node, NodeKind, Statement, Variable,
};
use mago_syntax::token::TokenKind;
use pun_problem::{Edit, Safety, Span};

use super::{TEST_CASE, TEST_CASE_LEGACY, class_body, class_is, report, resolve, string_value};
use crate::rules::rector_stmts::{
    assign, block_list, expression, first_comment, line_indent, previous_token_end, reindent,
    tokens,
};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `ClassName::MOCK_OBJECT`.
const MOCK_OBJECT: &str = "PHPUnit\\Framework\\MockObject\\MockObject";

/// The classes after which `TypeWillReturnCallableArrowFunctionRector` and this rule look one call further back.
const INVOCATION_MOCKERS: [&str; 3] = [
    "PHPUnit\\Framework\\MockObject\\Builder\\InvocationMocker",
    "PHPUnit\\Framework\\MockObject\\InvocationMocker",
    "PHPUnit\\Framework\\MockObject\\InvocationStubber",
];

pub struct PreferWillReturnCallback;

impl Rule for PreferWillReturnCallback {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-will-return-callback"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else { return };
        if !is_named(&call.method, "with") {
            return;
        }
        let Some((arg, closure)) = match_callback_closure(call) else {
            return;
        };
        let Some(method_call) = mocked_method_call(call) else {
            return;
        };
        // each test class whose methods hold the call, as Rector visits them: the outermost first
        let classes = test_classes(ctx, ancestors);
        if !classes
            .iter()
            .any(|&at| is_mocked_void_method_call(ctx, &ancestors[..=at], method_call))
        {
            return;
        }
        let Some(printed) = print_closure(ctx.source, call, arg, closure) else {
            return;
        };
        let name = node_span(&call.method);
        out.push(report(
            name,
            "Use willReturnCallback() for a callback on a method that returns void.",
            Safety::Safe,
            vec![Edit::new(name, "willReturnCallback"), printed],
        ));
    }
}

/// `NodeNameResolver::isName()` for a method selector: an identifier, ASCII case-insensitively.
pub(super) fn is_named(selector: &ClassLikeMemberSelector<'_>, name: &str) -> bool {
    matches!(selector, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name.as_bytes()))
}

/// `matchCallbackClosure()`: the one argument of `with()`, a `->callback()` call whose first argument is a closure.
fn match_callback_closure<'a>(
    call: &'a MethodCall<'a>,
) -> Option<(&'a Argument<'a>, &'a Closure<'a>)> {
    let args = &call.argument_list.arguments;
    // a 2nd+ ->with() argument matches a further parameter, keep the callback matcher as is
    if args.len() != 1 {
        return None;
    }
    let arg = args.first()?;
    let Expression::Call(Call::Method(nested)) = argument_value(arg) else {
        return None;
    };
    if !is_named(&nested.method, "callback") {
        return None;
    }
    match argument_value(nested.argument_list.arguments.first()?) {
        Expression::Closure(closure) => Some((arg, closure)),
        _ => None,
    }
}

fn argument_value<'a>(arg: &'a Argument<'a>) -> &'a Expression<'a> {
    match arg {
        Argument::Positional(a) => a.value,
        Argument::Named(a) => a.value,
    }
}

/// The `->method('name')` call `with()` is called on.
fn mocked_method_call<'a>(call: &'a MethodCall<'a>) -> Option<&'a MethodCall<'a>> {
    match call.object {
        Expression::Call(Call::Method(parent)) if is_named(&parent.method, "method") => {
            Some(parent)
        }
        _ => None,
    }
}

/// The indices in `ancestors` of the test classes (`Class_` nodes, named or anonymous) whose methods hold the node,
/// outermost first.
pub(super) fn test_classes(ctx: &Context<'_>, ancestors: &[Node<'_, '_>]) -> Vec<usize> {
    (0..ancestors.len())
        .filter(|&at| {
            matches!(ancestors[at], Node::Class(_) | Node::AnonymousClass(_))
                // in one of its methods (`getMethods()`), not in another class-like of its members
                && ancestors[at + 1..]
                    .iter()
                    .find(|a| {
                        matches!(
                            a,
                            Node::Method(_)
                                | Node::Class(_)
                                | Node::AnonymousClass(_)
                                | Node::Interface(_)
                                | Node::Trait(_)
                                | Node::Enum(_)
                        )
                    })
                    .is_some_and(|a| matches!(a, Node::Method(_)))
                && class_is(ctx, &ancestors[..=at], &[TEST_CASE, TEST_CASE_LEGACY])
        })
        .collect()
}

/// `isMockedVoidMethodCall()` in the class `class` ends in.
fn is_mocked_void_method_call(
    ctx: &Context<'_>,
    class: &[Node<'_, '_>],
    method_call: &MethodCall<'_>,
) -> bool {
    let Some(name) = method_call
        .argument_list
        .arguments
        .first()
        .map(argument_value)
        .filter(|v| matches!(v, Expression::Literal(Literal::String(_))))
        .and_then(string_value)
    else {
        return false;
    };
    let Some(mocked) = setup_assigned_mock_types(ctx, class) else {
        return false;
    };
    match mocked_classes(ctx, method_call, &mocked) {
        Some(candidates) => resolve_return_is_void(ctx, &candidates, name.as_bytes()) == Some(true),
        None => false,
    }
}

/// A PHPStan type of a receiver, as far as these rules tell them apart.
#[derive(Debug, PartialEq)]
pub(super) enum Caller {
    /// An `ObjectType` of a class.
    Object(String),
    /// A `NeverType`.
    Never,
    /// An `IntersectionType` of these classes.
    Intersection(Vec<String>),
    /// Any other type.
    Other,
}

/// The type PHPStan gives `expr`, from the type Mago gives it: Mago keeps an intersection of classes as written, and
/// PHPStan reduces it as `TypeCombinator::intersect()` does.
pub(super) fn caller(ctx: &Context<'_>, expr: &Expression<'_>) -> Caller {
    let facts = facts(ctx);
    let types = facts
        .full_type(node_span(expr))
        .map_or(&[][..], |t| t.types.as_ref());
    let [atomic @ TAtomic::Object(_)] = types else {
        return Caller::Other;
    };
    let mut members: Vec<String> = Vec::new();
    for member in
        std::iter::once(atomic).chain(atomic.get_intersection_types().into_iter().flatten())
    {
        let name = match member {
            TAtomic::Object(TObject::Named(named)) if !named.is_this => named.name.to_string(),
            TAtomic::Object(TObject::Enum(e)) => e.name.to_string(),
            // `object` adds nothing to a class
            TAtomic::Object(TObject::Any) => continue,
            _ => return Caller::Other,
        };
        if !members.iter().any(|m| m.eq_ignore_ascii_case(&name)) {
            members.push(name);
        }
    }
    // `TypeCombinator::intersect()`: a class another member is a subtype of goes, and two members no object can be
    // both make `never`
    let codebase = &facts.codebase;
    let all = members.clone();
    members.retain(|m| {
        !all.iter()
            .any(|n| !n.eq_ignore_ascii_case(m) && is_super_type_of(codebase, m, n) == Some(true))
    });
    let excluded = members.iter().enumerate().any(|(i, a)| {
        members[i + 1..]
            .iter()
            .any(|b| is_super_type_of(codebase, b, a) == Some(false))
    });
    if excluded {
        return Caller::Never;
    }
    match members.len() {
        0 => Caller::Other,
        1 => Caller::Object(members.remove(0)),
        _ => Caller::Intersection(members),
    }
}

/// `ObjectType::isSuperTypeOf()` between two classes of other names: `Some(true)` for yes, `Some(false)` for no,
/// `None` for maybe (a class it does not know included).
fn is_super_type_of(codebase: &CodebaseMetadata, this: &str, that: &str) -> Option<bool> {
    let this_class = codebase.get_class_like(this.as_bytes())?;
    let that_class = codebase.get_class_like(that.as_bytes())?;
    if this_class.kind == SymbolKind::Trait || that_class.kind == SymbolKind::Trait {
        return Some(false);
    }
    if codebase.is_instance_of(that.as_bytes(), this.as_bytes()) {
        return Some(true);
    }
    if codebase.is_instance_of(this.as_bytes(), that.as_bytes()) {
        return None;
    }
    let is_final = |c: &mago_codex::metadata::class_like::ClassLikeMetadata| {
        c.flags.is_final() || c.kind == SymbolKind::Enum
    };
    if this_class.kind == SymbolKind::Interface && !is_final(that_class)
        || that_class.kind == SymbolKind::Interface && !is_final(this_class)
    {
        return None;
    }
    Some(false)
}

/// `SetUpAssignedMockTypesResolver::resolveFromClass()`: the class each property or variable (by name; `None` for a
/// name that is not a plain one, which PHP keys as `""`) is assigned a `createMock()` or `getMockBuilder()` of at the
/// top level of `setUp()`. `None` where Rector fails (a class that is not a name).
pub(super) fn setup_assigned_mock_types(
    ctx: &Context<'_>,
    class: &[Node<'_, '_>],
) -> Option<Vec<(Option<Vec<u8>>, String)>> {
    let (members, _, _) = class_body(*class.last().unwrap())?;
    let mut out: Vec<(Option<Vec<u8>>, String)> = Vec::new();
    let Some(set_up) = members.iter().find_map(|m| match m {
        ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"setUp") => Some(m),
        _ => None,
    }) else {
        return Some(out);
    };
    let MethodBody::Concrete(block) = &set_up.body else {
        return Some(out);
    };
    for statement in block_list(ctx.source, block).stmts {
        let Some(assignment) = expression(statement).and_then(assign) else {
            continue;
        };
        let Expression::Call(Call::Method(call)) = assignment.rhs else {
            continue;
        };
        if !is_named(&call.method, "createMock") && !is_named(&call.method, "getMockBuilder") {
            continue;
        }
        let name = match assignment.lhs {
            Expression::Variable(_) | Expression::Access(Access::Property(_)) => {
                target_name(assignment.lhs)
            }
            _ => continue,
        };
        let Some(Expression::Access(Access::ClassConstant(fetch))) =
            call.argument_list.arguments.first().map(argument_value)
        else {
            continue;
        };
        let mocked = match fetch.class {
            Expression::Identifier(id) => resolve(class, id),
            Expression::Self_(_) => "self".to_string(),
            Expression::Static(_) => "static".to_string(),
            Expression::Parent(_) => "parent".to_string(),
            // `getName()` of a variable is its name, which names no class
            Expression::Variable(Variable::Direct(v)) => {
                String::from_utf8_lossy(&v.name[1..]).into()
            }
            // Assert::string() fails
            _ => return None,
        };
        match out.iter_mut().find(|(n, _)| *n == name) {
            Some(entry) => entry.1 = mocked,
            None => out.push((name, mocked)),
        }
    }
    Some(out)
}

/// `getName()` of a variable, or of a property fetch's name: `None` where it is not a plain name.
fn target_name(expr: &Expression<'_>) -> Option<Vec<u8>> {
    match expr {
        Expression::Variable(Variable::Direct(v)) => Some(v.name[1..].to_vec()),
        Expression::Access(Access::Property(p)) => match &p.property {
            ClassLikeMemberSelector::Identifier(id) => Some(id.value.to_vec()),
            _ => None,
        },
        _ => None,
    }
}

/// The classes `resolveFromReflection()` reads the mocked method on, from the receiver of `method_call` (or of the
/// `expects()` call before it): the members of its intersection type, after `fallbackMockedObjectInSetUp()`. `None`
/// when the type is no intersection.
pub(super) fn mocked_classes(
    ctx: &Context<'_>,
    method_call: &MethodCall<'_>,
    mocked: &[(Option<Vec<u8>>, String)],
) -> Option<Vec<String>> {
    let mut expr = method_call;
    let mut caller_type = caller(ctx, expr.object);
    let mut stepped_to_other = false;
    if let Caller::Object(class) = &caller_type
        && INVOCATION_MOCKERS
            .iter()
            .any(|m| m.eq_ignore_ascii_case(class))
    {
        match expr.object {
            Expression::Call(Call::Method(previous)) => {
                expr = previous;
                caller_type = caller(ctx, previous.object);
            }
            _ => stepped_to_other = true,
        }
    }
    // fallbackMockedObjectInSetUp
    let falls_back = match &caller_type {
        Caller::Object(class) => class.eq_ignore_ascii_case(MOCK_OBJECT),
        Caller::Never => true,
        _ => false,
    };
    if falls_back
        && !stepped_to_other
        && matches!(
            expr.object,
            Expression::Variable(_) | Expression::Access(Access::Property(_))
        )
    {
        let name = target_name(expr.object);
        if let Some((_, class)) = mocked.iter().find(|(n, _)| *n == name) {
            return Some(vec![class.clone()]);
        }
    }
    match caller_type {
        Caller::Intersection(classes) => Some(
            classes
                .into_iter()
                .filter(|c| !c.eq_ignore_ascii_case(MOCK_OBJECT))
                .collect(),
        ),
        _ => None,
    }
}

/// Whether the mocked method's native return type is `void`, as `resolveFromReflection()` reads it on the first of
/// `classes` that has the method natively; `None` when none has it, or when those that have it disagree: Rector reads
/// the first as written, and Mago keeps an intersection's members in an order of its own (Pun/intersection_order).
fn resolve_return_is_void(ctx: &Context<'_>, classes: &[String], method: &[u8]) -> Option<bool> {
    let facts = facts(ctx);
    let mut found: Option<bool> = None;
    for class in classes {
        if facts.codebase.get_class_like(class.as_bytes()).is_none() {
            continue;
        }
        let Some(metadata) = facts
            .codebase
            .get_declaring_method(class.as_bytes(), method)
        else {
            continue;
        };
        let is_void = metadata
            .return_type_declaration_metadata
            .as_ref()
            .is_some_and(|t| matches!(t.type_union.types.as_ref(), [TAtomic::Void]));
        match found {
            Some(other) if other != is_void => return None,
            _ => found = Some(is_void),
        }
    }
    found
}

/// What becomes of a `return` with a value in the closure.
enum Change<'a> {
    /// `return value;` becomes `return;`.
    Bare,
    /// A nested `return throw ...;` becomes `throw ...;`.
    Throw(&'a mago_syntax::cst::Throw<'a>),
    /// A `return throw ...;` at the closure's top level stays.
    Keep,
}

/// The edit that puts the closure, with `removeValueReturns()` done, in place of `with()`'s argument, as php-parser's
/// format-preserving printer writes it; `None` where the closure stays (`removeValueReturns()` is false) or Pun does
/// not print it as php-parser does.
fn print_closure(
    source: &str,
    call: &MethodCall<'_>,
    arg: &Argument<'_>,
    closure: &Closure<'_>,
) -> Option<Edit> {
    let span = node_span(closure);
    let body = &closure.body;
    // nested function-likes have their own return scope, skip to stay safe; returns in other class-likes' methods count
    let mut nested = false;
    let mut returns = Vec::new();
    let mut stack: Vec<Node<'_, '_>> = body.statements.iter().map(Node::Statement).collect();
    while let Some(node) = stack.pop() {
        match node {
            Node::Closure(_) | Node::ArrowFunction(_) | Node::Function(_) => nested = true,
            Node::Return(r) => returns.push(r),
            _ => {}
        }
        node.visit_children(|child| stack.push(child));
    }
    if nested {
        return None;
    }
    let list = block_list(source, body);
    let top_level = |r: &mago_syntax::cst::Return<'_>| {
        list.stmts
            .iter()
            .any(|s| matches!(s, Statement::Return(t) if std::ptr::eq(t, r)))
    };
    let mut changes = Vec::new();
    for r in &returns {
        let Some(value) = r.value else {
            continue;
        };
        // "return throw new X" becomes a bare "throw new X" statement
        let change = match unparenthesize(value) {
            Expression::Throw(_) if top_level(r) => Change::Keep,
            Expression::Throw(throw) => Change::Throw(throw),
            // only strip side-effect-free values
            Expression::Literal(_)
            | Expression::CompositeString(
                CompositeString::Interpolated(_) | CompositeString::Document(_),
            )
            | Expression::MagicConstant(_)
            | Expression::ConstantAccess(_)
            | Expression::Variable(_) => Change::Bare,
            _ => return None,
        };
        changes.push((*r, value, change));
    }
    // nothing to drop, keep the callback matcher as is
    if changes.is_empty() {
        return None;
    }
    if tokens(source, span.start, span.end).iter().any(|t| {
        matches!(
            t.kind,
            TokenKind::CloseTag | TokenKind::OpenTag | TokenKind::EchoTag
        )
    }) {
        return None;
    }
    let arg_span = node_span(arg);
    let open = call.argument_list.left_parenthesis.end.offset as usize;
    // a comment before the argument goes with it
    if first_comment(source, open, arg_span.start).is_some() {
        return None;
    }
    // php-parser moves the closure's lines by the difference between the indents of the argument's line and of the
    // closure's (counting a tab as 4 columns); Pun follows a move to the left in spaces
    let arg_indent = line_indent(source, arg_span.start);
    let closure_indent = line_indent(source, span.start);
    let cut = if !source[arg_span.start..span.start].contains('\n') {
        String::new()
    } else if arg_indent.contains('\t')
        || closure_indent.contains('\t')
        || arg_indent.len() > closure_indent.len()
    {
        return None;
    } else {
        " ".repeat(closure_indent.len() - arg_indent.len())
    };
    // the drop of a now-empty trailing "return;"
    let popped = list.last().and_then(|last| match last {
        Statement::Return(r)
            if r.value.is_none()
                || changes.iter().any(|(c, _, change)| {
                    std::ptr::eq(*c, r) && matches!(change, Change::Bare)
                }) =>
        {
            Some(list.stmts.len() - 1)
        }
        _ => None,
    });
    let text = if popped == Some(0) {
        // no statements left: php-parser prints the closure anew
        if arg_indent.contains('\t') {
            return None;
        }
        pretty_closure(source, closure, arg_indent)?
    } else {
        let mut edits: Vec<Edit> = Vec::new();
        if let Some(i) = popped {
            let last = node_span(list.stmts[i]);
            edits.push(Edit::new(Span::new(list.before(source, i), last.end), ""));
        }
        for (r, value, change) in &changes {
            let r_span = node_span(*r);
            if edits.first().is_some_and(|e| {
                popped.is_some() && e.span.start <= r_span.start && r_span.end <= e.span.end
            }) {
                continue;
            }
            match change {
                Change::Keep => {}
                Change::Bare => {
                    if matches!(value, Expression::Parenthesized(_)) {
                        return None;
                    }
                    // a `;`: a `?>` in the closure declined it above
                    edits.push(Edit::new(
                        Span::new(
                            r.r#return.span.end.offset as usize,
                            r.terminator.span().start.offset as usize,
                        ),
                        "",
                    ));
                }
                Change::Throw(throw) => {
                    let before = previous_token_end(
                        source,
                        body.left_brace.end.offset as usize,
                        r_span.start,
                    );
                    let start = first_comment(source, before, r_span.start).unwrap_or(r_span.start);
                    let printed = reindent(source, node_span(*throw), &cut, "");
                    edits.push(Edit::new(
                        Span::new(start, r_span.end),
                        format!("{printed};"),
                    ));
                }
            }
        }
        // the mocked method returns void, so type the callback the same way
        match &closure.return_type_hint {
            Some(hint) => edits.push(Edit::new(node_span(&hint.hint), "void")),
            None => {
                let close = match &closure.use_clause {
                    Some(uses) => uses.right_parenthesis,
                    None => closure.parameter_list.right_parenthesis,
                };
                let at = close.end.offset as usize;
                edits.push(Edit::new(Span::new(at, at), ": void"));
            }
        }
        edits.sort_by_key(|e| (e.span.start, e.span.end));
        let mut text = String::new();
        let mut at = span.start;
        for edit in &edits {
            text.push_str(&reindent(source, Span::new(at, edit.span.start), &cut, ""));
            text.push_str(&edit.text);
            at = edit.span.end;
        }
        text.push_str(&reindent(source, Span::new(at, span.end), &cut, ""));
        text
    };
    Some(Edit::new(arg_span, text))
}

/// The closure as php-parser's pretty printer writes it with no statements and a `void` return type, its `}` at the
/// indent `indent`: its attributes, parameters and `use` variables as written (php-parser prints each original node
/// as it was); `None` for one written over several lines or with comments, which Pun does not print as it does.
fn pretty_closure(source: &str, closure: &Closure<'_>, indent: &str) -> Option<String> {
    let as_written = |span: Span| {
        let text = &source[span.start..span.end];
        (!text.contains('\n')
            && tokens(source, span.start, span.end)
                .iter()
                .all(|t| !t.kind.is_comment()))
        .then(|| text.to_string())
    };
    // a comment between the parameters or the `use` variables goes with the node after it
    let from = closure.function.span.end.offset as usize;
    let to = closure.body.left_brace.start.offset as usize;
    if tokens(source, from, to).iter().any(|t| t.kind.is_comment()) {
        return None;
    }
    let mut attributes = String::new();
    for list in closure.attribute_lists.iter() {
        attributes.push_str(&as_written(node_span(list))?);
        attributes.push(' ');
    }
    let params = closure
        .parameter_list
        .parameters
        .iter()
        .map(|p| as_written(node_span(p)))
        .collect::<Option<Vec<_>>>()?;
    let uses = match &closure.use_clause {
        Some(uses) => {
            let variables = uses
                .variables
                .iter()
                .map(|v| as_written(node_span(v)))
                .collect::<Option<Vec<_>>>()?;
            format!(" use ({})", variables.join(", "))
        }
        None => String::new(),
    };
    Some(format!(
        "{attributes}{}function {}({}){uses}: void {{\n{indent}}}",
        if closure.r#static.is_some() {
            "static "
        } else {
            ""
        },
        if closure.ampersand.is_some() { "&" } else { "" },
        params.join(", "),
    ))
}
