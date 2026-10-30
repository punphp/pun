//! Helpers the rules of lint 7 batch 5 share: an `if` replaced by the statements of one of its branches, PHP
//! version values as Rector reads them, and a few class-member lookups.

use mago_syntax::cst::*;
use pun_problem::Edit;

use mago_syntax::token::TokenKind;
use pun_problem::Span;

use super::rector_code_quality::named;
use super::rector_stmts::{
    Frame, List, block_list, body_list, braced, enclosing, first_comment, holder_list,
    inserted_statements, line_indent, moved_statements, remove, tokens,
};
use crate::syntax::{node_span, unparenthesize};

/// The edits for Rector's `refactor()` returning `stmts` in place of the statement `statement` (whose parent
/// `ancestors` ends with): the statement removed when `stmts` is empty (`NodeVisitor::REMOVE_NODE`), else replaced
/// by them, moved to its indent.
pub(crate) fn replace_with<'a>(
    source: &str,
    statement: &'a Statement<'a>,
    ancestors: &[Node<'a, 'a>],
    stmts: Option<&List<'a>>,
    changes: impl Fn(&Statement<'_>) -> bool,
) -> Vec<Edit> {
    let (list, index) = enclosing(source, statement, ancestors);
    // Other statements of the list the rule changes in the same pass make php-parser's diff of the list differ.
    let alone = list
        .stmts
        .iter()
        .enumerate()
        .all(|(i, s)| i == index || !changes(s));
    let Some(stmts) = stmts.filter(|s| !s.is_empty()) else {
        return remove(source, &list, &[index]);
    };
    let span = node_span(list.stmts[index]);
    let indent = line_indent(source, span.start);
    if list.frame == Frame::Bare && stmts.len() > 1 {
        // php-parser prints the statements the single statement of a control structure becomes in braces, after the
        // header at the control structure's indent; one statement takes its place.
        let from = source[..span.start].trim_end().len();
        let outer = line_indent(source, from);
        let inner = format!("{outer}    ");
        let text = format!(
            " {{\n{inner}{}\n{outer}}}",
            moved_statements(source, stmts, &inner)
        );
        return vec![Edit::new(Span::new(from, span.end), text)];
    }
    let text = inserted_statements(source, stmts, indent);
    if alone && stmts.len() > 1 {
        // php-parser's differ sees one removal and several additions (not a replacement), and pArray drops the
        // removed statement with its comments and the whitespace on one side: after it when it is first (the
        // additions wait for the next kept statement), before it otherwise.
        if index > 0 {
            let from = list.before(source, index);
            return vec![Edit::new(
                Span::new(from, span.end),
                format!("\n{indent}{text}"),
            )];
        }
        if index + 1 < list.stmts.len() {
            let from = list.with_comments(source, index).start;
            let to = list.with_comments(source, index + 1).start;
            return vec![Edit::new(Span::new(from, to), format!("{text}\n{indent}"))];
        }
    }
    vec![Edit::new(span, text)]
}

/// A value `ValueResolver::getValue` reads from the syntax alone: a literal, a negated number, or the name of a
/// constant other than `true`, `false` and `null`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Constant(String),
}

pub(crate) fn value(expr: &Expression<'_>) -> Option<Value> {
    Some(match unparenthesize(expr) {
        Expression::Literal(Literal::Integer(i)) => Value::Int(i64::try_from(i.value?).ok()?),
        Expression::Literal(Literal::Float(f)) => Value::Float(f.value.0),
        Expression::Literal(Literal::String(s)) => {
            Value::Str(String::from_utf8(s.value?.to_vec()).ok()?)
        }
        Expression::Literal(Literal::True(_)) => Value::Bool(true),
        Expression::Literal(Literal::False(_)) => Value::Bool(false),
        Expression::Literal(Literal::Null(_)) => Value::Null,
        Expression::ConstantAccess(c) => {
            let name = c.name.value();
            let name = name.strip_prefix(b"\\").unwrap_or(name);
            let name = String::from_utf8(name.to_vec()).ok()?;
            match name.to_ascii_lowercase().as_str() {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                "null" => Value::Null,
                _ => Value::Constant(name),
            }
        }
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Negation(_)) => {
            match value(u.operand)? {
                Value::Int(i) => Value::Int(-i),
                Value::Float(f) => Value::Float(-f),
                _ => return None,
            }
        }
        _ => return None,
    })
}

/// PHP's `(string)` of a float, for the floats a version is written as (no exponent).
pub(crate) fn float_string(f: f64) -> Option<String> {
    (f.is_finite() && (f == 0.0 || (1e-4..1e15).contains(&f.abs()))).then(|| format!("{f}"))
}

/// `PhpVersionFactory::createIntVersion`.
pub(crate) fn int_version(version: &str) -> i64 {
    let version = version.split('-').next().unwrap_or(version);
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 2 {
        return php_int(parts[0]) * 10000 + php_int(parts[1]) * 100;
    }
    php_int(version)
}

/// PHP's `(int)` of a string: the leading integer after leading whitespace, 0 when there is none. A `-` never
/// reaches it, since `createIntVersion` cuts the string at the first one.
fn php_int(s: &str) -> i64 {
    let s = s.trim_start_matches([' ', '\t', '\n', '\r', '\x0b', '\x0c']);
    let digits = s.strip_prefix('+').unwrap_or(s);
    let end = digits
        .bytes()
        .position(|b| !b.is_ascii_digit())
        .unwrap_or(digits.len());
    digits[..end].parse::<i64>().unwrap_or(0)
}

/// The first method of a class named `name`, ignoring ASCII case (`Class_::getMethod`).
pub(crate) fn method<'a>(
    members: &'a Sequence<'a, ClassLikeMember<'a>>,
    name: &[u8],
) -> Option<&'a Method<'a>> {
    members.iter().find_map(|m| match m {
        ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(name) => Some(m),
        _ => None,
    })
}

/// The edits php-parser's format-preserving printer makes to remove the items `removed` flags from a comma-separated
/// list whose items span `items` (`pArray`): an item after a kept one goes with the text from the end of the item
/// before it, the items before the first kept one go with the text up to it. `None` when every item goes: php-parser
/// then prints the list's parent anew.
pub(crate) fn list_removal(items: &[Span], removed: &[bool]) -> Option<Vec<Edit>> {
    let first_kept = removed.iter().position(|r| !r)?;
    let mut edits = Vec::new();
    if first_kept > 0 {
        edits.push(Edit::new(
            Span::new(items[0].start, items[first_kept].start),
            "",
        ));
    }
    for i in first_kept + 1..items.len() {
        if removed[i] {
            edits.push(Edit::new(Span::new(items[i - 1].end, items[i].end), ""));
        }
    }
    Some(merge(edits))
}

/// `edits`, sorted, with the edits that touch joined into one (a fix's edits must not touch).
pub(crate) fn merge(mut edits: Vec<Edit>) -> Vec<Edit> {
    edits.sort_by_key(|e| (e.span.start, e.span.end));
    let mut out: Vec<Edit> = Vec::new();
    for edit in edits {
        match out.last_mut() {
            Some(last) if last.span.end == edit.span.start => {
                let text = format!("{}{}", last.text, edit.text);
                *last = Edit::new(Span::new(last.span.start, edit.span.end), text);
            }
            _ => out.push(edit),
        }
    }
    out
}

/// The doc comment php-parser gives the node at `start`, whose previous token ends at or before `from`: the last
/// comment before the node, when that comment is a `/** */` one.
pub(crate) fn doc_comment(source: &str, from: usize, start: usize) -> Option<Span> {
    let last = tokens(source, from, start)
        .into_iter()
        .rfind(|t| t.kind.is_comment())?;
    (last.kind == TokenKind::DocBlockComment).then(|| Span::new(last.start, last.end))
}

/// Where the member at `index` of a class body may have its comments from: the end of the member before it, or the
/// class's `{`.
pub(crate) fn member_from(
    members: &Sequence<'_, ClassLikeMember<'_>>,
    left_brace: mago_span::Span,
    index: usize,
) -> usize {
    match index.checked_sub(1).and_then(|i| members.get(i)) {
        Some(previous) => node_span(previous).end,
        None => left_brace.end.offset as usize,
    }
}

/// The doc comment `doc` without the `@param` tags of the parameters `names` (with their `$`), as Rector's phpdoc
/// printer removes a tag node (`PhpDocTagRemover`, with [`list_removal`]'s rules over the doc's text and tag nodes).
/// `Some("")` when nothing is left, which Rector prints as no doc comment; `None` when no tag goes.
pub(crate) fn doc_without_params(doc: &str, names: &[&[u8]]) -> Option<String> {
    let nodes = doc_nodes(doc);
    let removed: Vec<bool> = nodes
        .iter()
        .map(|n| param_tag_name(&doc[n.start..n.end]).is_some_and(|name| names.contains(&name)))
        .collect();
    if !removed.contains(&true) {
        return None;
    }
    let Some(edits) = list_removal(&nodes, &removed) else {
        return Some(String::new());
    };
    let mut out = doc.to_string();
    for edit in edits.iter().rev() {
        out.replace_range(edit.span.start..edit.span.end, &edit.text);
    }
    Some(out)
}

/// [`doc_without_params`] for the tags `remove` picks by their text (from the `@` to the end of their description).
pub(crate) fn doc_without_tags(doc: &str, remove: impl Fn(&str) -> bool) -> Option<String> {
    let nodes = doc_nodes(doc);
    let removed: Vec<bool> = nodes
        .iter()
        .map(|n| doc[n.start..n.end].starts_with('@') && remove(&doc[n.start..n.end]))
        .collect();
    if !removed.contains(&true) {
        return None;
    }
    let Some(edits) = list_removal(&nodes, &removed) else {
        return Some(String::new());
    };
    let mut out = doc.to_string();
    for edit in edits.iter().rev() {
        out.replace_range(edit.span.start..edit.span.end, &edit.text);
    }
    Some(out)
}

/// The text and tag nodes of a doc comment: a tag from its `@` to the end of the last line of its description, a
/// text from its first character to the end of its paragraph. A blank line ends either.
fn doc_nodes(doc: &str) -> Vec<Span> {
    let inner_end = doc.len() - doc.strip_suffix("*/").map_or(0, |_| 2);
    let mut nodes: Vec<Span> = Vec::new();
    let mut open = false;
    let mut line_start = 0;
    for line in doc[..inner_end].split_inclusive('\n') {
        let start = line_start;
        line_start += line.len();
        let body = line.trim_end_matches(['\n', '\r']);
        let mut content = body.trim_start();
        if start == 0 {
            content = content.strip_prefix("/**").unwrap_or(content);
        } else if let Some(rest) = content.strip_prefix('*') {
            content = rest;
        }
        let content_trimmed = content.trim();
        if content_trimmed.is_empty() {
            open = false;
            continue;
        }
        let from = start + body.len() - content.trim_start().len();
        let to = from + content_trimmed.len();
        match nodes.last_mut() {
            Some(last) if open && !content_trimmed.starts_with('@') => last.end = to,
            _ => nodes.push(Span::new(from, to)),
        }
        open = true;
    }
    nodes
}

/// The parameter a `@param` tag names, with its `$`.
fn param_tag_name(tag: &str) -> Option<&[u8]> {
    let rest = tag.strip_prefix("@param")?;
    if !rest.starts_with([' ', '\t']) {
        return None;
    }
    let bytes = rest.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' {
            let end = bytes[i + 1..]
                .iter()
                .position(|b| !(b.is_ascii_alphanumeric() || *b == b'_' || *b >= 0x80))
                .map_or(bytes.len(), |p| i + 1 + p);
            if &bytes[i..end] != b"$this" && end > i + 1 {
                return Some(&bytes[i..end]);
            }
        }
        i += 1;
    }
    None
}

type IsModifier = fn(&Modifier<'_>) -> bool;

/// php-parser's `pModifiers`: the modifiers in its order, each followed by a space.
pub(crate) fn modifiers_text(modifiers: &Sequence<'_, Modifier<'_>>) -> String {
    let order: [(IsModifier, &str); 8] = [
        (|m| matches!(m, Modifier::Final(_)), "final "),
        (|m| matches!(m, Modifier::Abstract(_)), "abstract "),
        (|m| matches!(m, Modifier::Public(_)), "public "),
        (|m| matches!(m, Modifier::Protected(_)), "protected "),
        (|m| matches!(m, Modifier::Private(_)), "private "),
        (
            |m| {
                matches!(
                    m,
                    Modifier::PublicSet(_) | Modifier::ProtectedSet(_) | Modifier::PrivateSet(_)
                )
            },
            "",
        ),
        (|m| matches!(m, Modifier::Static(_)), "static "),
        (|m| matches!(m, Modifier::Readonly(_)), "readonly "),
    ];
    order
        .iter()
        .filter(|(pick, _)| modifiers.iter().any(pick))
        .map(|(_, text)| *text)
        .collect()
}

/// `pStmt_ClassMethod` for a method whose parameters all go: php-parser prints the method anew (its attributes,
/// return type and statements keep their code; blank lines between the statements go).
pub(crate) fn method_without_params(source: &str, method: &Method<'_>) -> String {
    let span = node_span(method);
    let indent = line_indent(source, span.start);
    let mut out = String::new();
    for list in method.attribute_lists.iter() {
        let s = node_span(list);
        out.push_str(&source[s.start..s.end]);
        out.push('\n');
        out.push_str(indent);
    }
    out.push_str(&modifiers_text(&method.modifiers));
    out.push_str("function ");
    if method.ampersand.is_some() {
        out.push('&');
    }
    out.push_str(std::str::from_utf8(method.name.value).unwrap_or_default());
    out.push_str("()");
    if let Some(hint) = &method.return_type_hint {
        let s = node_span(&hint.hint);
        out.push_str(": ");
        out.push_str(&source[s.start..s.end]);
    }
    match &method.body {
        MethodBody::Concrete(block) => {
            out.push('\n');
            out.push_str(indent);
            out.push_str(&braced(source, &block_list(source, block), indent));
        }
        MethodBody::Abstract(_) => out.push(';'),
    }
    out
}

/// The statements of the namespace that holds the node `ancestors` leads to, or of the file without one.
pub(crate) fn scope_statements<'a>(ancestors: &[Node<'a, 'a>]) -> (Option<String>, Vec<&'a Statement<'a>>) {
    for node in ancestors.iter().rev() {
        if let Node::Namespace(ns) = node {
            let name = ns
                .name
                .as_ref()
                .map(|n| String::from_utf8_lossy(n.value()).into_owned());
            let statements = match &ns.body {
                NamespaceBody::Implicit(body) => body.statements.iter().collect(),
                NamespaceBody::BraceDelimited(block) => block.statements.iter().collect(),
            };
            return (name, statements);
        }
    }
    match ancestors.first() {
        Some(Node::Program(program)) => (None, program.statements.iter().collect()),
        _ => (None, Vec::new()),
    }
}

/// php-parser's `NameResolver` for a class name written at the node `ancestors` leads to: the fully qualified name,
/// without its leading `\`, through the `use` imports and the namespace.
pub(crate) fn resolve_class_name(ancestors: &[Node<'_, '_>], name: &Identifier<'_>) -> String {
    let written = String::from_utf8_lossy(name.value()).into_owned();
    if let Some(fq) = written.strip_prefix('\\') {
        return fq.to_string();
    }
    if let Some(imported) = imported(ancestors, &written) {
        return imported;
    }
    match scope_statements(ancestors).0 {
        Some(ns) => format!("{ns}\\{written}"),
        None => written,
    }
}

/// The name `written` (not fully qualified) resolved through a class `use` import of the scope, if one imports its
/// first part.
pub(crate) fn imported(ancestors: &[Node<'_, '_>], written: &str) -> Option<String> {
    let (first, rest) = match written.split_once('\\') {
        Some((first, rest)) => (first, Some(rest)),
        None => (written, None),
    };
    for statement in scope_statements(ancestors).1 {
        let Statement::Use(r#use) = statement else {
            continue;
        };
        let items: Vec<(Option<&[u8]>, &UseItem)> = match &r#use.items {
            UseItems::Sequence(s) => s.items.iter().map(|i| (None, i)).collect(),
            UseItems::MixedList(l) => l
                .items
                .iter()
                .filter(|i| i.r#type.is_none())
                .map(|i| (Some(l.namespace.value()), &i.item))
                .collect(),
            UseItems::TypedSequence(_) | UseItems::TypedList(_) => continue,
        };
        for (prefix, item) in items {
            let name = String::from_utf8_lossy(item.name.value());
            let full = match prefix {
                Some(prefix) => format!("{}\\{name}", String::from_utf8_lossy(prefix)),
                None => name.into_owned(),
            };
            let full = full.trim_start_matches('\\').to_string();
            let alias = match &item.alias {
                Some(alias) => String::from_utf8_lossy(alias.identifier.value).into_owned(),
                None => full.rsplit('\\').next().unwrap_or(&full).to_string(),
            };
            if alias.eq_ignore_ascii_case(first) {
                return Some(match rest {
                    Some(rest) => format!("{full}\\{rest}"),
                    None => full,
                });
            }
        }
    }
    None
}

/// The function a `use function` import of the scope gives the local name `local` (case-insensitive), without its
/// leading `\\`.
pub(crate) fn imported_function(ancestors: &[Node<'_, '_>], local: &[u8]) -> Option<String> {
    imported_member(ancestors, local, true)
}

/// The function (`function`) or constant a `use function` / `use const` import of the scope gives `local`: function
/// names compare without case, constant names with it.
pub(crate) fn imported_member(ancestors: &[Node<'_, '_>], local: &[u8], function: bool) -> Option<String> {
    let wanted = |t: &UseType<'_>| {
        if function {
            t.is_function()
        } else {
            t.is_const()
        }
    };
    for statement in scope_statements(ancestors).1 {
        let Statement::Use(r#use) = statement else {
            continue;
        };
        let items: Vec<(Option<&[u8]>, &UseItem)> = match &r#use.items {
            UseItems::TypedSequence(s) if wanted(&s.r#type) => {
                s.items.iter().map(|i| (None, i)).collect()
            }
            UseItems::TypedList(l) if wanted(&l.r#type) => l
                .items
                .iter()
                .map(|i| (Some(l.namespace.value()), i))
                .collect(),
            UseItems::MixedList(l) => l
                .items
                .iter()
                .filter(|i| i.r#type.as_ref().is_some_and(wanted))
                .map(|i| (Some(l.namespace.value()), &i.item))
                .collect(),
            _ => continue,
        };
        for (prefix, item) in items {
            let name = String::from_utf8_lossy(item.name.value());
            let full = match prefix {
                Some(prefix) => format!("{}\\{name}", String::from_utf8_lossy(prefix)),
                None => name.into_owned(),
            };
            let full = full.trim_start_matches('\\').to_string();
            let alias = match &item.alias {
                Some(alias) => alias.identifier.value.to_vec(),
                None => full
                    .rsplit('\\')
                    .next()
                    .unwrap_or(&full)
                    .as_bytes()
                    .to_vec(),
            };
            if (function && alias.eq_ignore_ascii_case(local)) || alias == local {
                return Some(full);
            }
        }
    }
    None
}

/// A constant name resolved as `mago-names` does: a fully qualified name without its `\\`, an unqualified one through a
/// `use const` import or else into the namespace, a qualified one through a class `use` import of its first part or
/// else into the namespace.
pub(crate) fn resolve_constant_name(ancestors: &[Node<'_, '_>], name: &Identifier<'_>) -> String {
    let written = String::from_utf8_lossy(name.value()).into_owned();
    if let Some(fq) = written.strip_prefix('\\') {
        return fq.to_string();
    }
    let imported = match name {
        Identifier::Local(local) => imported_member(ancestors, local.value, false),
        _ => imported(ancestors, &written),
    };
    if let Some(imported) = imported {
        return imported;
    }
    match scope_statements(ancestors).0 {
        Some(ns) => format!("{ns}\\{written}"),
        None => written,
    }
}

/// The name Mago's linter matches a called function by (`function_call_matches`): an unqualified name imported with
/// `use function` is its import, any other name is as written, without a leading `\\`.
pub(crate) fn called_function_name(
    ancestors: &[Node<'_, '_>],
    function: &Expression<'_>,
) -> Option<String> {
    let Expression::Identifier(identifier) = function else {
        return None;
    };
    if let Identifier::Local(local) = identifier
        && let Some(full) = imported_function(ancestors, local.value)
    {
        return Some(full);
    }
    let written = String::from_utf8_lossy(identifier.value());
    Some(written.trim_start_matches('\\').to_string())
}

/// `PhpAttributeAnalyzer::hasPhpAttribute`: whether one of the attribute lists names the class `name`.
pub(crate) fn has_attribute(
    ancestors: &[Node<'_, '_>],
    lists: &Sequence<'_, AttributeList<'_>>,
    name: &str,
) -> bool {
    lists.iter().any(|list| {
        list.attributes
            .iter()
            .any(|a| resolve_class_name(ancestors, &a.name).eq_ignore_ascii_case(name))
    })
}

/// `ParamAnalyzer::isParamUsedInClassMethod`: a promoted parameter, or one whose variable the method's statements
/// use (outside nested functions and classes, the arguments of a `new class(...)` included), a `compact()` naming it,
/// or a call to `func_get_arg()`, `func_get_args()`, `func_num_args()` or `get_defined_vars()`.
pub(crate) fn param_used(method: &Method<'_>, param: &FunctionLikeParameter<'_>) -> bool {
    if !param.modifiers.is_empty() {
        return true;
    }
    match &method.body {
        MethodBody::Concrete(block) => uses_param(Node::Block(block), param.variable.name),
        MethodBody::Abstract(_) => false,
    }
}

fn uses_param(node: Node<'_, '_>, name: &[u8]) -> bool {
    match node {
        Node::FunctionCall(call)
            if [
                "func_get_arg",
                "func_get_args",
                "func_num_args",
                "get_defined_vars",
            ]
            .iter()
            .any(|f| named(call, f)) =>
        {
            return true;
        }
        Node::FunctionCall(call) if named(call, "compact") => {
            let names = call.argument_list.arguments.iter().any(|a| {
                matches!(value(a.value()), Some(Value::Str(s)) if s.as_bytes().eq_ignore_ascii_case(&name[1..]))
            });
            if names {
                return true;
            }
        }
        Node::Function(_)
        | Node::Class(_)
        | Node::Interface(_)
        | Node::Trait(_)
        | Node::Enum(_) => {
            return false;
        }
        Node::AnonymousClass(class) => {
            return class
                .argument_list
                .as_ref()
                .is_some_and(|args| uses_param(Node::PartialArgumentList(args), name));
        }
        Node::DirectVariable(v) => return v.name == name,
        _ => {}
    }
    let mut found = false;
    node.visit_children(|child| found = found || uses_param(child, name));
    found
}

/// The edits that remove the parameters `removed` flags from `method`, whose comments may start at `from`, with
/// their `@param` tags: a list removal, or the method printed anew when every parameter goes.
pub(crate) fn remove_params(
    source: &str,
    method: &Method<'_>,
    from: usize,
    removed: &[bool],
) -> Vec<Edit> {
    let params: Vec<&FunctionLikeParameter> = method.parameter_list.parameters.iter().collect();
    let span = node_span(method);
    let mut edits = Vec::new();
    let names: Vec<&[u8]> = params
        .iter()
        .zip(removed)
        .filter(|(_, r)| **r)
        .map(|(p, _)| p.variable.name)
        .collect();
    if let Some(doc) = doc_comment(source, from, span.start)
        && let Some(text) = doc_without_params(&source[doc.start..doc.end], &names)
    {
        edits.push(if text.is_empty() {
            Edit::new(Span::new(doc.start, span.start), "")
        } else {
            Edit::new(doc, text)
        });
    }
    let items: Vec<Span> = params.iter().map(|p| node_span(*p)).collect();
    match list_removal(&items, removed) {
        Some(list) => edits.extend(list),
        None => edits.push(Edit::new(span, method_without_params(source, method))),
    }
    merge(edits)
}

/// The edits php-parser's format-preserving printer makes to remove the items `removed` flags from a statement-like
/// list (statements, class members): `spans` are the items, `froms` the end of the token before each. An item after
/// a kept one goes with the comments and blanks before it; the items before the first kept one go with the blanks
/// after them, up to the first kept item's comments. `None` when every item goes.
pub(crate) fn remove_items(
    source: &str,
    spans: &[Span],
    froms: &[usize],
    removed: &[bool],
) -> Option<Vec<Edit>> {
    let first_kept = removed.iter().position(|r| !r)?;
    let comment_start =
        |i: usize| first_comment(source, froms[i], spans[i].start).unwrap_or(spans[i].start);
    let mut edits = Vec::new();
    if first_kept > 0 {
        edits.push(Edit::new(
            Span::new(comment_start(0), comment_start(first_kept)),
            "",
        ));
    }
    for i in first_kept + 1..spans.len() {
        if removed[i] {
            edits.push(Edit::new(Span::new(froms[i], spans[i].end), ""));
        }
    }
    Some(merge(edits))
}

/// [`remove_items`] for the statements of a list, with the empty list printed as [`remove`] prints it, and a
/// trailing `Nop` kept as the last item.
pub(crate) fn remove_statements(source: &str, list: &List<'_>, removed: &[bool]) -> Vec<Edit> {
    let mut spans: Vec<Span> = list.stmts.iter().map(|s| node_span(*s)).collect();
    let mut froms: Vec<usize> = (0..list.stmts.len())
        .map(|i| list.before(source, i))
        .collect();
    let mut flags = removed.to_vec();
    if let Some(nop) = list.nop_span(source) {
        froms.push(spans.last().map_or(list.inner.start, |s| s.end));
        spans.push(nop);
        flags.push(false);
    }
    match remove_items(source, &spans, &froms, &flags) {
        Some(edits) => edits,
        None => {
            let all: Vec<usize> = (0..list.stmts.len()).collect();
            remove(source, list, &all)
        }
    }
}

/// [`remove_items`] for the members of a class body between `left_brace` and `right_brace`; an emptied body prints
/// as `{`, a newline and `}` at the indent of the line of `{`.
pub(crate) fn remove_members(
    source: &str,
    members: &Sequence<'_, ClassLikeMember<'_>>,
    left_brace: mago_span::Span,
    right_brace: mago_span::Span,
    removed: &[bool],
) -> Vec<Edit> {
    let spans: Vec<Span> = members.iter().map(node_span).collect();
    let froms: Vec<usize> = (0..spans.len())
        .map(|i| member_from(members, left_brace, i))
        .collect();
    match remove_items(source, &spans, &froms, removed) {
        Some(edits) => edits,
        None => {
            let (open, close) = (
                left_brace.end.offset as usize,
                right_brace.start.offset as usize,
            );
            let indent = line_indent(source, open - 1);
            vec![Edit::new(Span::new(open, close), format!("\n{indent}"))]
        }
    }
}

/// The edits for Rector's replacing the statements `start..=end` of `list` by one new statement `code` (at the list's
/// indent), as php-parser's `pArray` prints that diff: one statement replaced in place keeps the blanks before it
/// and drops its comments; several go with the blanks and comments before them, and the new statement follows the
/// kept one before them on a line of its own. With nothing kept the list's block is printed anew. `None` where Pun
/// does not know php-parser's output (a list without braces left empty, or statements removed from its start ahead of
/// kept ones).
pub(crate) fn replace_run(
    source: &str,
    list: &List<'_>,
    start: usize,
    end: usize,
    code: &str,
) -> Option<Vec<Edit>> {
    let first = node_span(list.stmts[start]);
    let indent = line_indent(source, first.start);
    let last = node_span(list.stmts[end]);
    if start == end {
        let from = list.with_comments(source, start).start;
        return Some(vec![Edit::new(Span::new(from, last.end), code.to_string())]);
    }
    if start > 0 {
        let from = list.before(source, start);
        return Some(vec![Edit::new(
            Span::new(from, last.end),
            format!("\n{indent}{code}"),
        )]);
    }
    if end + 1 < list.stmts.len() || list.nop {
        return None;
    }
    let Frame::Braces { open, close } = list.frame else {
        return None;
    };
    let outer = line_indent(source, open - 1);
    Some(vec![Edit::new(
        Span::new(open, close),
        format!("\n{outer}    {code}\n{outer}"),
    )])
}

/// The statements of a `foreach`.
pub(crate) fn foreach_list<'a>(source: &str, foreach: &'a Foreach<'a>) -> Option<List<'a>> {
    match &foreach.body {
        ForeachBody::Statement(statement) => Some(body_list(source, statement)),
        ForeachBody::ColonDelimited(body) => {
            holder_list(source, Node::ForeachColonDelimitedBody(body), &[])
        }
    }
}

/// Whether PHPStan's native type of `expr`, where `ancestors` leads, is an array, decided from the syntax alone: an
/// array literal; `$this->name` of a property declared `array`; a parameter declared `array` (without a `null`
/// default) that the function never writes; or a local variable whose every write is an array literal or an
/// array element.
pub(crate) fn native_array(expr: &Expression<'_>, ancestors: &[Node<'_, '_>]) -> bool {
    match unparenthesize(expr) {
        Expression::Array(_) | Expression::LegacyArray(_) => true,
        Expression::Access(Access::Property(fetch)) => {
            let (
                Expression::Variable(Variable::Direct(this)),
                ClassLikeMemberSelector::Identifier(name),
            ) = (fetch.object, &fetch.property)
            else {
                return false;
            };
            this.name == b"$this" && array_property(ancestors, name.value)
        }
        Expression::Variable(Variable::Direct(v)) => {
            let Some((params, body)) = function_like(ancestors) else {
                return false;
            };
            let param = params.parameters.iter().find(|p| p.variable.name == v.name);
            let writes = variable_writes(body, v.name);
            match param {
                Some(p) => {
                    matches!(p.hint, Some(Hint::Array(_)))
                        && p.ampersand.is_none()
                        && p.ellipsis.is_none()
                        // `null` in any case is a literal.
                        && !p.default_value.as_ref().is_some_and(|d| {
                            matches!(unparenthesize(d.value), Expression::Literal(Literal::Null(_)))
                        })
                        && writes == Some(0)
                }
                None => writes.is_some_and(|n| n > 0),
            }
        }
        _ => false,
    }
}

/// Whether the class around `ancestors` declares the property `name` with the type `array`.
fn array_property(ancestors: &[Node<'_, '_>], name: &[u8]) -> bool {
    let members = ancestors.iter().rev().find_map(|n| match n {
        Node::Class(c) => Some(&c.members),
        Node::AnonymousClass(c) => Some(&c.members),
        // A trait's methods run without a class in Rector's scope, so PHPStan knows no property type there.
        _ => None,
    });
    let Some(members) = members else { return false };
    members.iter().any(|m| match m {
        ClassLikeMember::Property(Property::Plain(p)) => {
            matches!(p.hint, Some(Hint::Array(_)))
                && p.items.iter().any(|i| &i.variable().name[1..] == name)
        }
        ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
            m.parameter_list.parameters.iter().any(|p| {
                !p.modifiers.is_empty()
                    && matches!(p.hint, Some(Hint::Array(_)))
                    && &p.variable.name[1..] == name
            })
        }
        _ => false,
    })
}

/// The parameters and body of the nearest function-like node around `ancestors`.
fn function_like<'a>(
    ancestors: &[Node<'a, 'a>],
) -> Option<(&'a FunctionLikeParameterList<'a>, Node<'a, 'a>)> {
    ancestors.iter().rev().find_map(|n| match n {
        Node::Method(
            m @ Method {
                body: MethodBody::Concrete(b),
                ..
            },
        ) => Some((&m.parameter_list, Node::Block(b))),
        Node::Function(f) => Some((&f.parameter_list, Node::Block(&f.body))),
        // An arrow function holds no statement, so no `foreach` asks from inside one.
        Node::Closure(c) => Some((&c.parameter_list, Node::Block(&c.body))),
        _ => None,
    })
}

/// The number of writes of an array literal to the variable `name` in `node` (outside nested function-likes and
/// classes), `None` when another write may change its type: any other assignment, a reference to it, a `foreach`,
/// `list()`, `global` or `static` target, `unset()`, or a closure's `use`.
fn variable_writes(node: Node<'_, '_>, name: &[u8]) -> Option<usize> {
    let is_var = |e: &Expression<'_>| matches!(unparenthesize(e), Expression::Variable(Variable::Direct(v)) if v.name == name);
    let mut count = 0;
    match node {
        Node::Assignment(a) if is_var(a.lhs) => {
            if !matches!(a.operator, AssignmentOperator::Assign(_))
                || !matches!(
                    unparenthesize(a.rhs),
                    Expression::Array(_) | Expression::LegacyArray(_)
                )
            {
                return None;
            }
            count += 1;
            return variable_writes(Node::Expression(a.rhs), name).map(|n| n + count);
        }
        Node::Assignment(a)
            if matches!(
                unparenthesize(a.lhs),
                Expression::Array(_) | Expression::LegacyArray(_) | Expression::List(_)
            ) =>
        {
            if uses_variable(Node::Expression(a.lhs), name) {
                return None;
            }
        }
        Node::UnaryPrefix(u) if u.operator.is_reference() && is_var(u.operand) => return None,
        Node::UnaryPrefix(u)
            if matches!(
                u.operator,
                UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_)
            ) && is_var(u.operand) =>
        {
            return None;
        }
        Node::UnaryPostfix(u) if is_var(u.operand) => return None,
        Node::Foreach(f) => {
            let targets: Vec<&Expression> = match &f.target {
                ForeachTarget::Value(t) => vec![t.value],
                ForeachTarget::KeyValue(t) => vec![t.key, t.value],
            };
            if targets
                .iter()
                .any(|t| uses_variable(Node::Expression(t), name))
            {
                return None;
            }
        }
        Node::Global(_) | Node::Static(_) | Node::Unset(_) if uses_variable(node, name) => {
            return None;
        }
        Node::Closure(c) => {
            let captured = c.use_clause.as_ref().is_some_and(|u| {
                u.variables
                    .iter()
                    .any(|v| v.variable.name == name && v.ampersand.is_some())
            });
            return if captured { None } else { Some(0) };
        }
        Node::Function(_)
        | Node::ArrowFunction(_)
        | Node::Class(_)
        | Node::AnonymousClass(_)
        | Node::Interface(_)
        | Node::Trait(_)
        | Node::Enum(_) => return Some(0),
        _ => {}
    }
    let mut failed = false;
    node.visit_children(|child| {
        if failed {
            return;
        }
        match variable_writes(child, name) {
            Some(n) => count += n,
            None => failed = true,
        }
    });
    (!failed).then_some(count)
}

/// Whether `node` holds the variable `name`.
fn uses_variable(node: Node<'_, '_>, name: &[u8]) -> bool {
    if let Node::DirectVariable(v) = node {
        return v.name == name;
    }
    let mut found = false;
    node.visit_children(|child| found = found || uses_variable(child, name));
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_version_is_php_version_factory() {
        // PhpVersionFactory::createIntVersion: major * 10000 + minor * 100, the patch level ignored.
        assert_eq!(int_version("7.2"), 70200);
        assert_eq!(int_version("8.1.12"), 80100);
        assert_eq!(int_version("8.3-dev"), 80300);
        assert_eq!(int_version("80100"), 80100);
        assert_eq!(int_version("x"), 0);
        // explode('-') keeps " ", and (int) " " is 0.
        assert_eq!(int_version(" -5"), 0);
        assert_eq!(int_version(" 7"), 7);
        assert_eq!(int_version("+7.4"), 70400);
    }

    #[test]
    fn float_string_is_php_cast() {
        // php -r 'var_dump((string) 7.2, (string) 8.0, (string) 7.10);'
        assert_eq!(float_string(7.2).as_deref(), Some("7.2"));
        assert_eq!(float_string(8.0).as_deref(), Some("8"));
        assert_eq!(float_string(7.10).as_deref(), Some("7.1"));
        assert_eq!(float_string(0.0).as_deref(), Some("0"));
        assert_eq!(float_string(1e20), None);
    }
}
