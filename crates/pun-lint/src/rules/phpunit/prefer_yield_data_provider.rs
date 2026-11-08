//! `lint/phpunit/prefer-yield-data-provider`, ported from rector-phpunit's `YieldDataProviderRector`
//! (rules/CodeQuality/Rector/Class_), with `DataProviderClassMethodFinder`, `NodeTransformer::transformArrayToYields`
//! and Rector's `IsClassMethodUsedAnalyzer`.
//!
//! In a test class, each data provider of the class (a method of the class named by the first argument, a string, of
//! a `#[PHPUnit\Framework\Attributes\DataProvider]` attribute of a method of the class or of its parents up to
//! `TestCase`, or, on a method without such attributes, by a generic `@dataProvider` tag, `()` trimmed) whose top-level
//! statements are assignments ending in a `return` of an array literal (or a single `yield from` an array literal),
//! and that nothing in the class calls, gets `: \Iterator` and yields the array's items instead: the `return` and
//! `yield from` statements go, and a `yield [key =>] value;` per item comes after the other statements, the first with
//! the comments of the last removed statement, each with the comments before its item.
//!
//! Its first `@return` tag goes when its type is not an array (with the other `@return`, `@phpstan-return` and
//! `@psalm-return` tags), and becomes `\Iterator<key, value>` when it is one, the key and the value as PHPStan reads
//! them from the array type and Rector prints them (PHPStan's `toPhpDocNode()`, then `StaticTypeMapper::styleTypeNode()`:
//! `A[]` for an array of `A` with no key or an `int` key, unions without parentheses), the blanks around the type as
//! written. Pun follows the types it reads the same way: names,
//! the keywords PHPStan knows, `array`, `list`, `non-empty-array`, `non-empty-list` and `iterable` with their generic
//! forms, `class-string<X>`, `T[]`, `?T` and unions of them that PHPStan prints without merging members. It leaves a
//! provider alone whose `@return` type is any other (a shape, a callable, an integer range, a literal, an
//! intersection, a union PHPStan merges), whose doc has a `@phpstan-return` or `@psalm-return` tag, a second `@return`,
//! a template or a type alias, and one indented with a tab, whose indentation php-parser keeps for the new statements
//! and Pun does not follow. A `@return` value phpdoc-parser cannot read (`array<int>$x`) stays as written; a
//! description, on the tag's line or below it, stays too. A `@dataProvider` tag of a parent class's method is read
//! with the `use` imports of the file being changed, as Rector reads it.
//!
//! Rector changes every provider of a class in one pass; Pun does too, and in later fix rounds leaves a class alone
//! whose span the project's codebase (scanned before the first round) no longer matches.
//!
//! An item's value whose operator binds looser than `yield` (`and`, `or`, `xor`) gets parentheses, where Rector
//! writes `yield $a and $b`, which yields `$a` only.

use std::cmp::Ordering;

use mago_syntax::cst::{
    ArrayElement, ClassLikeMember, Expression, Literal, Method, MethodBody, Node, NodeKind,
    Statement, TokenSeparatedSequence, UnaryPrefixOperator, Yield,
};
use pun_problem::{Edit, Safety, Span};

use super::annotations::{Annotated, TagValue, named_tags};
use super::{TEST_CASE, class_parents, is_in_test_class, resolve};
use crate::rules::class_method::{ClassInfo, class_info, is_class_method_used, walk};
use crate::rules::lint_b_doc::Doc;
use crate::rules::lint_b_names::resolve_class_at;
use crate::rules::php_parser::{Token, tokens};
use crate::rules::rector_batch5::remove_statements;
use crate::rules::rector_code_quality::{needs_parens, prec};
use crate::rules::rector_conditions::{comments_between, indentation, print_comments, reprint};
use crate::rules::rector_stmts::{
    List, block_list, first_comment, line_indent, previous_token_end,
};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const DATA_PROVIDER: &str = "PHPUnit\\Framework\\Attributes\\DataProvider";

pub struct PreferYieldDataProvider;

impl Rule for PreferYieldDataProvider {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-yield-data-provider"
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
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let facts = facts(ctx);
        let class = class_info(node, ancestors, &facts).unwrap();
        // Rector changes every provider of a class in one pass. In a later fix round the project's codebase still
        // has the class as it was before (Mago's analyser then skips its moved methods), so the class is left as the
        // first round left it.
        if !class.is_anonymous
            && facts
                .codebase
                .get_class_like(class.name.as_bytes())
                .is_some_and(|c| {
                    c.span.start.offset as usize != node_span(&node).start
                        || c.span.end.offset as usize != node_span(&node).end
                })
        {
            return;
        }
        let methods: Vec<&Method<'_>> = class
            .members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(method) => Some(method),
                _ => None,
            })
            .collect();
        // `DataProviderClassMethodFinder::find`
        let Some(mut names) = provider_names(ctx, ctx, &scope, &scope, &methods) else {
            return;
        };
        // the parents up to `TestCase`, each with a source (as the test class extends it, each is a user class)
        let parents = class_parents(ctx, &class.name);
        for (metadata, source) in parents
            .iter()
            .take_while(|p| !p.eq_ignore_ascii_case(TEST_CASE))
            .filter_map(|p| facts.codebase.get_class_like(p.as_bytes()))
            .filter_map(|m| Some((m, facts.source_of(m.span.file_id)?)))
        {
            match parent_provider_names(
                ctx,
                ancestors,
                &source,
                metadata.span.start.offset as usize,
            ) {
                Some(more) => names.extend(more),
                None => return,
            }
        }
        let mut providers: Vec<&Method<'_>> = Vec::new();
        for name in names {
            if let Some(provider) = methods
                .iter()
                .copied()
                .find(|m| m.name.value.eq_ignore_ascii_case(name.as_bytes()))
                && !providers.iter().any(|p| std::ptr::eq(*p, provider))
            {
                providers.push(provider);
            }
        }
        let tokens = tokens(ctx);
        let mut edits = Vec::new();
        let mut first = None;
        for provider in providers {
            let MethodBody::Concrete(block) = &provider.body else {
                continue;
            };
            let list = block_list(ctx.source, block);
            let Some(array) = returned_array(&list) else {
                continue;
            };
            if is_class_method_used(&facts, ancestors, &class, provider.name.value) {
                continue;
            }
            let Some(more) = transform(ctx, &tokens, &scope, &class, provider, &list, array) else {
                continue;
            };
            edits.extend(more);
            first.get_or_insert(node_span(&provider.name));
        }
        if let Some(span) = first {
            out.push(Report::fixed(
                span,
                "Yield the data provider's items instead of returning an array.",
                Safety::Safe,
                edits,
            ));
        }
    }
}

/// `resolverDataProviderClassMethodNames` over `methods`, declared in `file` at `scope`; a doc's tags are read as at
/// `tag_scope`. `None` where Pun does not read a doc as phpdoc-parser does.
fn provider_names(
    ctx: &Context<'_>,
    file: &Context<'_>,
    scope: &[Node<'_, '_>],
    tag_scope: &[Node<'_, '_>],
    methods: &[&Method<'_>],
) -> Option<Vec<String>> {
    let mut names = Vec::new();
    for method in methods {
        let mut attributes = Vec::new();
        for list in method.attribute_lists.iter() {
            for attribute in list.attributes.iter() {
                if resolve(scope, &attribute.name).eq_ignore_ascii_case(DATA_PROVIDER) {
                    // `$attribute->args[0]->value`, `null` (with a warning) without arguments
                    let argument = attribute
                        .argument_list
                        .as_ref()
                        .and_then(|list| list.arguments.iter().next());
                    attributes.push(argument.and_then(|a| a.value()));
                }
            }
        }
        if !attributes.is_empty() {
            for value in attributes {
                if let Some(Expression::Literal(Literal::String(s))) = value
                    && let Some(v) = s.value
                {
                    names.push(String::from_utf8_lossy(v).into_owned());
                }
            }
            continue;
        }
        let Some(doc) = Doc::of(file, scope, node_span(*method).start) else {
            continue;
        };
        for (_, value) in named_tags(ctx, tag_scope, &doc, "dataProvider")? {
            if let TagValue::Generic(value) = value {
                names.push(value.trim_matches(['(', ')']).to_string());
            }
        }
    }
    Some(names)
}

/// The provider names of the class declared at `start` in `source` (a parent class's file, read again as Rector's
/// `AstResolver` does). Its attributes' names resolve in that file; its doc tags in its namespace with the `use`
/// imports of the file being changed, whose ancestors are `current` (`ClassAnnotationMatcher` reads those from
/// `UseImportsResolver`, which knows the current file only).
fn parent_provider_names(
    ctx: &Context<'_>,
    current: &[Node<'_, '_>],
    source: &str,
    start: usize,
) -> Option<Vec<String>> {
    let arena = mago_allocator::LocalArena::new();
    let file = mago_database::file::File::ephemeral(
        std::borrow::Cow::Borrowed(b"parent.php"),
        std::borrow::Cow::Owned(source.as_bytes().to_vec()),
    );
    let program = mago_syntax::parser::parse_file(&arena, &file);
    let parent = Context::new(source, ctx.path, ctx.php_version);
    let mut names = Some(Vec::new());
    walk(Node::Program(program), &mut |node, path| {
        let Node::Class(class) = node else {
            return true;
        };
        if node_span(class).start != start {
            return true;
        }
        let methods: Vec<&Method<'_>> = class
            .members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(method) => Some(method),
                _ => None,
            })
            .collect();
        let mut scope = path.to_vec();
        scope.push(node);
        let tag_scope: Vec<Node<'_, '_>> = current
            .iter()
            .copied()
            .filter(|n| matches!(n, Node::Program(_)))
            .chain(
                scope
                    .iter()
                    .copied()
                    .filter(|n| !matches!(n, Node::Program(_))),
            )
            .collect();
        names = provider_names(ctx, &parent, &scope, &tag_scope, &methods);
        false
    });
    names
}

/// `collectReturnArrayNodesFromClassMethod`.
fn returned_array<'a>(list: &List<'a>) -> Option<ArrayParts<'a>> {
    let mut yielded_from = None;
    for statement in &list.stmts {
        let expr = match statement {
            Statement::Return(r) => return array_parts(r.value?),
            Statement::Expression(e) => unparenthesize(e.expression),
            _ => return None,
        };
        match expr {
            Expression::Yield(Yield::From(from)) => {
                if yielded_from.is_some() {
                    return None;
                }
                yielded_from = Some(array_parts(from.iterator)?);
            }
            Expression::Assignment(_) => {}
            _ => return None,
        }
    }
    // php-parser's `Nop` for comments after the last statement is neither
    if list.nop {
        return None;
    }
    yielded_from
}

/// An array literal's opening token and its elements.
type ArrayParts<'a> = (
    mago_span::Span,
    &'a TokenSeparatedSequence<'a, ArrayElement<'a>>,
);

/// The parts of `expr` when it is an array literal.
fn array_parts<'a>(expr: &'a Expression<'a>) -> Option<ArrayParts<'a>> {
    match unparenthesize(expr) {
        Expression::Array(a) => Some((a.left_bracket, &a.elements)),
        Expression::LegacyArray(a) => Some((a.left_parenthesis, &a.elements)),
        _ => None,
    }
}

/// Whether a top-level statement (a `return` or an expression, as [`returned_array`] found them) is a `return` or a
/// `yield from`, which the transform removes.
fn is_removed(statement: &Statement<'_>) -> bool {
    matches!(statement, Statement::Return(_))
        || matches!(
            statement,
            Statement::Expression(e)
                if matches!(unparenthesize(e.expression), Expression::Yield(Yield::From(_)))
        )
}

/// `transformArrayToYieldsOnMethodNode`: the edits for one provider; `None` where Pun leaves it alone.
fn transform(
    ctx: &Context<'_>,
    tokens: &[Token],
    scope: &[Node<'_, '_>],
    class: &ClassInfo<'_, '_>,
    method: &Method<'_>,
    list: &List<'_>,
    (open, elements): ArrayParts<'_>,
) -> Option<Vec<Edit>> {
    let source = ctx.source;
    let mut edits = Vec::new();
    // the doc comment's `@return` tag
    let node = Node::Method(method);
    let annotated = Annotated::of(ctx, node, scope).unwrap();
    if let Some(doc) = annotated.doc.as_ref()
        && let Some(new_doc) = new_doc(scope, class, doc)?
    {
        edits.extend(annotated.edits(source, Some(&new_doc), &[])?);
    }
    // the return type
    edits.push(match &method.return_type_hint {
        Some(hint) => Edit::new(node_span(&hint.hint), "\\Iterator"),
        None => {
            let at = node_span(&method.parameter_list).end;
            Edit::new(Span::new(at, at), ": \\Iterator")
        }
    });
    // the statements
    let removed: Vec<bool> = list.stmts.iter().map(|s| is_removed(s)).collect();
    let first_at = list
        .stmts
        .first()
        .map(|s| node_span(*s).start)
        .or_else(|| list.nop_span(source).map(|n| n.start))
        .unwrap();
    let indent = line_indent(source, first_at);
    if indent.contains('\t') {
        return None;
    }
    let nl = format!("\n{indent}");
    let level = indentation(source, first_at) as isize;
    // the comments of the last removed statement
    let last_removed = removed.iter().rposition(|r| *r).unwrap();
    let removed_comments = comments_between(
        tokens,
        list.before(ctx, last_removed),
        node_span(list.stmts[last_removed]).start,
    );
    let mut yields = Vec::new();
    let mut from = open.end.offset as usize;
    for (i, element) in elements.iter().enumerate() {
        let span = node_span(element);
        let mut comments = comments_between(
            tokens,
            previous_token_end(ctx, from, span.start),
            span.start,
        );
        if i == 0 {
            comments.splice(0..0, removed_comments.iter().copied());
        }
        from = span.end;
        let (key, value) = match element {
            ArrayElement::KeyValue(kv) => (Some(kv.key), kv.value),
            ArrayElement::Value(v) => (None, v.value),
            ArrayElement::Variadic(v) => (None, v.value),
            ArrayElement::Missing(_) => return None,
        };
        let value = match unparenthesize(value) {
            Expression::UnaryPrefix(u)
                if matches!(u.operator, UnaryPrefixOperator::Reference(_)) =>
            {
                unparenthesize(u.operand)
            }
            value => value,
        };
        let print = |expr: &Expression<'_>| {
            let span = node_span(expr);
            reprint(
                source,
                tokens,
                span,
                level - indentation(source, span.start) as isize,
            )
        };
        let mut text = String::new();
        if !comments.is_empty() {
            text.push_str(&print_comments(source, &comments, &nl));
            text.push_str(&nl);
        }
        text.push_str("yield ");
        if let Some(key) = key {
            text.push_str(&print(unparenthesize(key)));
            text.push_str(" => ");
        }
        if needs_parens(prec(value), 175, None) {
            text.push_str(&format!("({});", print(value)));
        } else {
            text.push_str(&format!("{};", print(value)));
        }
        yields.push(text);
    }
    let kept_end = list.nop_span(source).map(|n| n.end).or_else(|| {
        (0..list.stmts.len())
            .rev()
            .find(|&i| !removed[i])
            .map(|i| node_span(list.stmts[i]).end)
    });
    match kept_end {
        Some(at) => {
            let added: String = yields.iter().map(|y| format!("{nl}{y}")).collect();
            let mut body = remove_statements(ctx, list, &removed);
            match body.iter_mut().find(|e| e.span.start == at) {
                Some(edit) => edit.text.push_str(&added),
                None if added.is_empty() => {}
                None => body.push(Edit::new(Span::new(at, at), added)),
            }
            edits.extend(body);
        }
        None if yields.is_empty() => edits.extend(remove_statements(ctx, list, &removed)),
        None => {
            let first = node_span(list.stmts[0]);
            let start =
                first_comment(source, list.before(ctx, 0), first.start).unwrap_or(first.start);
            let end = node_span(*list.stmts.last().unwrap()).end;
            edits.push(Edit::new(Span::new(start, end), yields.join(&nl)));
        }
    }
    Some(edits)
}

/// The doc comment after `removeReturnTag`: `Some(None)` when it stays, `None` where Pun leaves the provider alone.
fn new_doc(
    scope: &[Node<'_, '_>],
    class: &ClassInfo<'_, '_>,
    doc: &Doc<'_>,
) -> Option<Option<String>> {
    let mut returns = Vec::new();
    for i in 0..doc.len() {
        let text = doc.text(i);
        let name = text.split([' ', '\t', '\n']).next().unwrap_or_default();
        if ["@phpstan-return", "@psalm-return"].contains(&name)
            || name.starts_with("@template")
            || name.starts_with("@phpstan-template")
            || name.starts_with("@psalm-template")
            || name.ends_with("-type")
        {
            return None;
        }
        // a tag's child runs on to the next tag, its description's lines and blank lines included
        if name == "@return" {
            returns.push(i);
        }
    }
    let [index] = returns[..] else {
        return if returns.is_empty() { Some(None) } else { None };
    };
    let text = doc.text(index).trim_end();
    let after = &text["@return".len()..];
    let value = after.trim_start_matches([' ', '\t']);
    // the blanks around the type stay as written
    let before = &after[..after.len() - value.len()];
    let reader = Reader {
        scope,
        class,
        at: doc.span.start,
    };
    let (ty, rest) = parse_type(value)?;
    // a description, which may start on or go on over the next lines, as written
    let description = rest.trim_start_matches([' ', '\t']);
    if !rest.is_empty() && !rest.starts_with([' ', '\t', '\r', '\n']) {
        // phpdoc-parser's `InvalidTagValueNode`: no `@return` value to change
        return Some(None);
    }
    let mut removed = vec![false; doc.len()];
    match reader.top(&ty)? {
        Top::Remove => {
            removed[index] = true;
            Some(Some(doc.print(&removed)))
        }
        Top::Change(key, value) => {
            // the tag with its new type, the blanks and the description as written; a description on the next line
            // follows the type directly
            let blank = if description.starts_with(['\r', '\n']) {
                ""
            } else {
                &rest[..rest.len() - description.len()]
            };
            let tag = format!("@return{before}\\Iterator<{key}, {value}>{blank}{description}");
            let all = doc.text_all();
            let from = text.as_ptr() as usize - all.as_ptr() as usize;
            Some(Some(format!(
                "{}{}{}",
                &all[..from],
                tag.trim_end(),
                &all[from + text.len()..]
            )))
        }
    }
}

/// A phpdoc type as far as Pun reads it.
#[derive(Debug)]
enum Syn<'s> {
    Name(&'s str),
    Generic(&'s str, Vec<Syn<'s>>),
    /// `T[]`
    List(Box<Syn<'s>>),
    Union(Vec<Syn<'s>>),
}

/// The type at the start of `s` and what follows it; `None` for a type Pun does not read.
fn parse_type(s: &str) -> Option<(Syn<'_>, &str)> {
    let (ty, rest) = parse_atom(s)?;
    // a union at the top needs no spaces around `|`
    if !rest.starts_with('|') {
        if rest.trim_start_matches([' ', '\t']).starts_with(['|', '&']) {
            return None;
        }
        return Some((ty, rest));
    }
    let mut members = vec![ty];
    let mut rest = rest;
    while let Some(after) = rest.strip_prefix('|') {
        let (ty, more) = parse_atom(after)?;
        members.push(ty);
        rest = more;
    }
    if rest.starts_with('&') {
        return None;
    }
    Some((Syn::Union(members), rest))
}

/// A type inside `<...>` or `(...)`, with blanks around its parts.
fn parse_inner(s: &str) -> Option<(Syn<'_>, &str)> {
    let s = s.trim_start_matches([' ', '\t']);
    let (ty, rest) = parse_atom(s)?;
    let mut members = vec![ty];
    let mut rest = rest.trim_start_matches([' ', '\t']);
    while let Some(after) = rest.strip_prefix('|') {
        let (ty, more) = parse_atom(after.trim_start_matches([' ', '\t']))?;
        members.push(ty);
        rest = more.trim_start_matches([' ', '\t']);
    }
    let ty = if members.len() == 1 {
        members.pop().unwrap()
    } else {
        Syn::Union(members)
    };
    Some((ty, rest))
}

fn parse_atom(s: &str) -> Option<(Syn<'_>, &str)> {
    if let Some(rest) = s.strip_prefix('?') {
        let (ty, rest) = parse_atom(rest)?;
        return Some((Syn::Union(vec![ty, Syn::Name("null")]), rest));
    }
    if let Some(rest) = s.strip_prefix('(') {
        let (ty, rest) = parse_inner(rest)?;
        let rest = rest.strip_prefix(')')?;
        if rest.starts_with(['[', '<', '(', '{', ':']) {
            return None;
        }
        return Some((ty, rest));
    }
    let b = s.as_bytes();
    let mut i = 0;
    if s.starts_with("$this") {
        i = 5;
    } else {
        loop {
            if b.get(i) == Some(&b'\\') {
                i += 1;
            }
            let start = i;
            while i < b.len()
                && (b[i].is_ascii_alphanumeric()
                    || b[i] == b'_'
                    || b[i] >= 0x80
                    || (i > start && b[i] == b'-'))
            {
                i += 1;
            }
            if i == start || b[start].is_ascii_digit() {
                return None;
            }
            if b.get(i) != Some(&b'\\') {
                break;
            }
        }
    }
    let name = &s[..i];
    let mut rest = &s[i..];
    let mut ty = Syn::Name(name);
    if let Some(after) = rest.strip_prefix('<') {
        let mut args = Vec::new();
        let mut after = after;
        loop {
            let (arg, more) = parse_inner(after)?;
            args.push(arg);
            if let Some(more) = more.strip_prefix(',') {
                after = more;
                continue;
            }
            rest = more.strip_prefix('>')?;
            break;
        }
        ty = Syn::Generic(name, args);
    }
    while let Some(more) = rest.strip_prefix("[]") {
        ty = Syn::List(Box::new(ty));
        rest = more;
    }
    if rest.starts_with(['[', '<', '(', '{', ':', '.']) {
        return None;
    }
    // `array <int>`: phpdoc-parser reads the generic over the blank
    if !matches!(ty, Syn::Generic(..)) && rest.trim_start_matches([' ', '\t']).starts_with('<') {
        return None;
    }
    Some((ty, rest))
}

/// What happens to a `@return` tag.
enum Top {
    Remove,
    /// `\Iterator<key, value>`
    Change(String, String),
}

/// A union member as PHPStan orders it.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Rank {
    Other,
    ConstantBool,
    Null,
}

/// What a union member is, for the unions Pun prints.
#[derive(PartialEq, Clone, Copy)]
enum Member {
    Null,
    ConstantBool,
    Bool,
    Scalar,
    Class,
    Array,
    /// a keyword type that only joins `null`
    Alone,
    /// a type PHPStan merges with any other in a union
    Solo,
}

/// The PHPStan keywords Pun does not read (`TypeNodeResolver::resolveIdentifierTypeNode`).
const UNREAD_KEYWORDS: &[&str] = &[
    "__stringandstringable",
    "__stringnotstringable",
    "associative-array",
    "callable-array",
    "callable-object",
    "callable-string",
    "closed-resource",
    "decimal-int-string",
    "empty",
    "empty-scalar",
    "enum-string",
    "interface-string",
    "literal-string",
    "lowercase-string",
    "never-return",
    "never-returns",
    "no-return",
    "non-decimal-int-string",
    "non-empty-literal-string",
    "non-empty-lowercase-string",
    "non-empty-mixed",
    "non-empty-scalar",
    "non-empty-uppercase-string",
    "non-falsy-string",
    "non-positive-int",
    "non-zero-int",
    "noreturn",
    "number",
    "numeric",
    "open-resource",
    "pure-callable",
    "pure-closure",
    "static-closure",
    "static-pure-closure",
    "trait-string",
    "truthy-string",
    "uppercase-string",
    "int-mask",
    "int-mask-of",
    "key-of",
    "value-of",
];

/// How a `@return` type reads in the class it is in.
struct Reader<'r, 'ast, 'arena> {
    scope: &'r [Node<'ast, 'arena>],
    class: &'r ClassInfo<'ast, 'arena>,
    /// The doc comment's start, for the imports before it.
    at: usize,
}

impl Reader<'_, '_, '_> {
    /// The `@return` type: removed when it is not an array, changed when it is one.
    fn top(&self, ty: &Syn<'_>) -> Option<Top> {
        let array = |key: &str, value: String| Some(Top::Change(key.to_string(), value));
        match ty {
            Syn::Name(name) => match name.to_ascii_lowercase().as_str() {
                "array" => array("int|string", "mixed".into()),
                "list" => array("int<0, max>", "mixed".into()),
                "non-empty-array" => array("mixed", "mixed".into()),
                _ => self.is_other(ty).then_some(Top::Remove),
            },
            Syn::List(inner) => array("int|string", self.value(inner)?.text),
            Syn::Generic(name, args) => match (name.to_ascii_lowercase().as_str(), &args[..]) {
                ("array" | "non-empty-array", [value]) => {
                    array("int|string", self.value(value)?.text)
                }
                ("array" | "non-empty-array", [key, value]) => {
                    array(&top_key(key)?, self.value(value)?.text)
                }
                ("list" | "non-empty-list", [value]) => {
                    array("int<0, max>", self.value(value)?.text)
                }
                _ => self.is_other(ty).then_some(Top::Remove),
            },
            Syn::Union(members) => {
                let arrays = members.iter().filter(|m| is_array_form(m)).count();
                (arrays < 2 && members.iter().all(|m| is_array_form(m) || self.is_other(m)))
                    .then_some(Top::Remove)
            }
        }
    }

    /// Whether a type that is no array form is one Pun knows is not an array: a name or a generic object.
    fn is_other(&self, ty: &Syn<'_>) -> bool {
        match ty {
            Syn::Generic(name, _) => {
                let lower = name.to_ascii_lowercase();
                matches!(lower.as_str(), "iterable" | "class-string")
                    || (self
                        .value(&Syn::Name(name))
                        .is_some_and(|p| p.member == Member::Class)
                        && !matches!(lower.as_str(), "self" | "parent"))
            }
            _ => self.value(ty).is_some(),
        }
    }

    /// An array's or an iterable's key written as `int`, `string` or `int|string`, as Rector prints it.
    fn key(&self, ty: &Syn<'_>) -> Option<String> {
        match ty {
            Syn::Name(n) => match n.to_ascii_lowercase().as_str() {
                "int" | "integer" => Some("int".into()),
                "string" => Some("string".into()),
                _ => None,
            },
            Syn::Union(_) => top_key(ty).filter(|k| k == "int|string"),
            _ => None,
        }
    }

    /// The key of a nested array: `None` for `array-key`, which PHPStan's benevolent `int|string` leaves unprinted.
    fn nested_key(&self, ty: &Syn<'_>) -> Option<Option<String>> {
        if let Syn::Name(n) = ty
            && n.eq_ignore_ascii_case("array-key")
        {
            return Some(None);
        }
        self.key(ty).map(Some)
    }

    /// A type inside the `\Iterator<...>`, printed.
    fn value(&self, ty: &Syn<'_>) -> Option<Printed> {
        let one = |text: String, member: Member| Some(Printed::new(text, member, Shape::Other));
        let generic =
            |text: String, member: Member| Some(Printed::new(text, member, Shape::Generic));
        match ty {
            Syn::Name(name) => {
                let lower = name.to_ascii_lowercase();
                let keyword = |text: &str, member: Member| one(text.to_string(), member);
                match lower.as_str() {
                    "int" | "integer" => keyword("int", Member::Scalar),
                    "string" => keyword("string", Member::Scalar),
                    "float" | "double" => keyword("float", Member::Scalar),
                    "bool" | "boolean" => keyword("bool", Member::Bool),
                    "true" | "false" => keyword(&lower, Member::ConstantBool),
                    "null" => keyword("null", Member::Null),
                    "mixed" => keyword("mixed", Member::Solo),
                    "void" | "never" => keyword(&lower, Member::Solo),
                    "array-key" => Some(keywords(&["int", "string"])),
                    "scalar" => Some(keywords(&["bool", "float", "int", "string"])),
                    "positive-int" => keyword("int<1, max>", Member::Solo),
                    "negative-int" => keyword("int<min, -1>", Member::Solo),
                    "non-negative-int" => keyword("int<0, max>", Member::Solo),
                    "resource" | "callable" | "iterable" | "static" | "$this"
                    | "numeric-string" | "non-empty-string" | "class-string" | "object" => {
                        keyword(&lower, Member::Alone)
                    }
                    "list" | "non-empty-list" => keyword(&lower, Member::Array),
                    "array" => Some(style_array(None, Printed::new("mixed".into(), Member::Solo, Shape::Other))),
                    "non-empty-array" => keyword(&lower, Member::Array),
                    "self" => {
                        if self.class.is_anonymous {
                            return None;
                        }
                        one(format!("\\{}", self.class.name), Member::Class)
                    }
                    "parent" => one(format!("\\{}", self.class.extends.as_ref()?), Member::Class),
                    _ if UNREAD_KEYWORDS.contains(&lower.as_str()) => None,
                    _ if lower.starts_with("namespace\\") => None,
                    _ if name.starts_with('\\') => one(name.to_string(), Member::Class),
                    _ => {
                        let resolved =
                            resolve_class_at(self.scope, self.at as u32, name.as_bytes());
                        one(
                            format!("\\{}", String::from_utf8_lossy(&resolved)),
                            Member::Class,
                        )
                    }
                }
            }
            Syn::List(inner) => Some(style_array(None, self.value(inner)?)),
            Syn::Generic(name, args) => {
                let lower = name.to_ascii_lowercase();
                match (lower.as_str(), &args[..]) {
                    ("array", [value]) => Some(style_array(None, self.value(value)?)),
                    ("array", [key, value]) => {
                        let value = self.value(value)?;
                        Some(style_array(self.nested_key(key)?.as_deref(), value))
                    }
                    ("non-empty-array", [value]) => generic(
                        format!("{lower}<{}>", self.value(value)?.text),
                        Member::Array,
                    ),
                    ("non-empty-array", [key, value]) => {
                        let value = self.value(value)?.text;
                        match self.nested_key(key)? {
                            Some(key) => generic(format!("{lower}<{key}, {value}>"), Member::Array),
                            None => generic(format!("{lower}<{value}>"), Member::Array),
                        }
                    }
                    ("list" | "non-empty-list", [value]) => generic(
                        format!("{lower}<{}>", self.value(value)?.text),
                        Member::Array,
                    ),
                    ("iterable", [value]) => generic(
                        format!("iterable<{}>", self.value(value)?.text),
                        Member::Alone,
                    ),
                    ("iterable", [key, value]) => {
                        let key = self.key(key).filter(|k| k != "int|string")?;
                        generic(
                            format!("iterable<{key}, {}>", self.value(value)?.text),
                            Member::Alone,
                        )
                    }
                    ("class-string", [Syn::Name(class)]) => {
                        let class = self.value(&Syn::Name(class))?;
                        (class.member == Member::Class).then(|| {
                            Printed::new(
                                format!("class-string<{}>", class.text),
                                Member::Alone,
                                Shape::Generic,
                            )
                        })
                    }
                    _ => None,
                }
            }
            Syn::Union(members) => self.union(members),
        }
    }

    /// A union as `UnionType::toPhpDocNode()` prints it: its members in `UnionTypeHelper::sortTypes()`'s order, then
    /// `StaticTypeMapper::styleTypeNode()`'s, which merges the members of a styled array of a union (`A[]|B[]`) in
    /// and drops repeats.
    fn union(&self, members: &[Syn<'_>]) -> Option<Printed> {
        let mut printed: Vec<Printed> = Vec::new();
        for member in members {
            if matches!(member, Syn::Union(_)) {
                return None;
            }
            let p = self.value(member)?;
            if !printed.iter().any(|q| q.text == p.text) {
                printed.push(p);
            }
        }
        if printed.len() == 1 {
            return printed.pop();
        }
        let count = |m: Member| printed.iter().filter(|p| p.member == m).count();
        let alone = count(Member::Alone);
        if count(Member::Solo) > 0
            || (alone > 0 && (printed.len() != 2 || count(Member::Null) != 1))
            || count(Member::Array) > 1
            || count(Member::ConstantBool) > 1
            || (count(Member::ConstantBool) > 0 && count(Member::Bool) > 0)
        {
            return None;
        }
        let rank = |p: &Printed| match p.member {
            Member::Null => Rank::Null,
            Member::ConstantBool => Rank::ConstantBool,
            _ => Rank::Other,
        };
        printed.sort_by(|a, b| {
            rank(a)
                .cmp(&rank(b))
                .then_with(|| compare_strings(&a.sort, &b.sort))
        });
        let mut flat: Vec<Printed> = Vec::new();
        for p in printed {
            for p in p.into_members() {
                if !flat.iter().any(|q| q.text == p.text) {
                    flat.push(p);
                }
            }
        }
        Some(Printed::union(flat, Member::Solo))
    }
}

/// A union PHPStan merges with any other type in a union (`array-key`, `scalar`), of the keywords `names`.
fn keywords(names: &[&str]) -> Printed {
    let members = names
        .iter()
        .map(|n| Printed::new(n.to_string(), Member::Scalar, Shape::Other))
        .collect();
    Printed::union(members, Member::Solo)
}

/// `styleTypeNode()` of PHPStan's `array<value>` (`key` `None`) or `array<key, value>`: `value[]`, or `A[]|B[]` for a
/// union of values, when `isArrayOfItemsOnly()` holds; the generic as it is otherwise.
fn style_array(key: Option<&str>, value: Printed) -> Printed {
    let generic = match key {
        Some(key) => format!("array<{key}, {}>", value.text),
        None => format!("array<{}>", value.text),
    };
    let items = match &value.shape {
        Shape::Union(members) => members.iter().collect(),
        _ => vec![&value],
    };
    // "array<int, A>" can be printed as "A[]", unless that loses the int key of nested arrays, or turns
    // "array<A<X>|A<Y>>" into "A<X>[]|A<Y>[]"
    let items_only = key.is_none_or(|k| k == "int")
        && items.iter().all(|item| {
            !(item.shape == Shape::Generic && items.len() > 1)
                && !(key.is_some() && matches!(item.shape, Shape::Generic | Shape::ArrayOf))
        });
    if !items_only {
        return Printed::new(generic, Member::Array, Shape::Generic).sorted("array");
    }
    let of = |item: &Printed| Printed::new(format!("{}[]", item.text), Member::Array, Shape::ArrayOf);
    match &value.shape {
        Shape::Union(members) => {
            Printed::union(members.iter().map(of).collect(), Member::Array).sorted("array")
        }
        _ => of(&value).sorted("array"),
    }
}

/// Whether a type is one of PHPStan's array types.
fn is_array_form(ty: &Syn<'_>) -> bool {
    match ty {
        Syn::Name(n) | Syn::Generic(n, _) => matches!(
            n.to_ascii_lowercase().as_str(),
            "array" | "list" | "non-empty-array" | "non-empty-list"
        ),
        Syn::List(_) => true,
        Syn::Union(_) => false,
    }
}

/// The key of a `@return` array type as PHPStan's `getIterableKeyType()` gives it and Rector prints it.
fn top_key(ty: &Syn<'_>) -> Option<String> {
    let name = |t: &Syn<'_>| match t {
        Syn::Name(n) => Some(n.to_ascii_lowercase()),
        _ => None,
    };
    match ty {
        Syn::Union(members) => {
            let mut names: Vec<String> = members.iter().map(name).collect::<Option<_>>()?;
            names.sort();
            (names == ["int", "string"]).then(|| "int|string".into())
        }
        _ => Some(
            match name(ty)?.as_str() {
                "int" | "integer" => "int",
                "string" => "string",
                "array-key" => "int|string",
                "non-empty-string" => "non-empty-string",
                "class-string" => "class-string",
                "positive-int" => "int<1, max>",
                _ => return None,
            }
            .to_string(),
        ),
    }
}

/// A printed type and what it is in a union.
struct Printed {
    text: String,
    member: Member,
    /// `describe(VerbosityLevel::typeOnly())`, which `UnionTypeHelper::sortTypes()` compares.
    sort: String,
    shape: Shape,
}

/// The phpdoc-parser node a printed type is, for `styleTypeNode()`.
#[derive(PartialEq)]
enum Shape {
    Other,
    /// `GenericTypeNode`
    Generic,
    /// `ArrayTypeNode`, `T[]`
    ArrayOf,
    /// `UnionTypeNode`
    Union(Vec<Printed>),
}

impl PartialEq for Printed {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl Printed {
    /// The sort key is the text without a leading `\` and the generic part (`list<int>` sorts as `list`).
    fn new(text: String, member: Member, shape: Shape) -> Self {
        let sort = text.trim_start_matches('\\');
        let sort = sort[..sort.find('<').unwrap_or(sort.len())].to_string();
        Printed {
            text,
            member,
            sort,
            shape,
        }
    }

    fn union(members: Vec<Printed>, member: Member) -> Self {
        let text = members.iter().map(|m| m.text.as_str()).collect::<Vec<_>>().join("|");
        Printed {
            sort: text.clone(),
            text,
            member,
            shape: Shape::Union(members),
        }
    }

    fn sorted(mut self, sort: &str) -> Self {
        self.sort = sort.to_string();
        self
    }

    /// A union's members, or the type itself.
    fn into_members(self) -> Vec<Printed> {
        match self.shape {
            Shape::Union(members) => members,
            _ => vec![self],
        }
    }
}

/// `UnionTypeHelper::compareStrings`: `strcasecmp`, then the bytes.
fn compare_strings(a: &str, b: &str) -> Ordering {
    a.to_ascii_lowercase()
        .cmp(&b.to_ascii_lowercase())
        .then_with(|| a.cmp(b))
}
