//! `lint/pest/no-redundant-uses`, ported from pest-plugin-rector's `RemoveRedundantPestUsesRector`: a test file's
//! `uses(X::class)`, `pest()->use(X::class)` or `pest()->uses(X::class)` loses the classes that `tests/Pest.php`
//! already applies to its directory with `->in('Feature')`, and goes when none is left.
//!
//! Like the plugin, the rule reads `tests/Pest.php` from the disk, next to the file it checks.

use std::borrow::Cow;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_database::file::{File, FileType};
use mago_syntax::cst::{
    Access, Argument, ArgumentList, ClassLikeConstantSelector, CompositeString, Expression,
    Literal, MethodCall, NamespaceBody, Node, NodeKind, Statement, StringPart, UseItems,
};
use pun_problem::{Edit, Safety, Span};

use super::{
    Item, func_call, function_name, identifier, method_call, old_statements, p_stmts, rewrite_list,
    statement_expression, statement_list,
};
use crate::rules::lint_b_names::resolve_class;
use crate::rules::rector_code_quality::text;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoRedundantUses;

impl Rule for NoRedundantUses {
    fn id(&self) -> &'static str {
        "lint/pest/no-redundant-uses"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else {
            return;
        };
        let global = globally_applied_classes(ctx.path);
        if global.is_empty() {
            return;
        }
        // refactorStatements: the file's statements and those of its namespaces.
        let mut edits = Vec::new();
        let mut lists = vec![(node, vec![node])];
        for statement in &program.statements {
            if let Statement::Namespace(ns) = statement {
                let holder = match &ns.body {
                    NamespaceBody::Implicit(body) => Node::NamespaceImplicitBody(body),
                    NamespaceBody::BraceDelimited(block) => Node::Block(block),
                };
                lists.push((holder, vec![node, Node::Namespace(ns)]));
            }
        }
        for (holder, scope) in lists {
            let Some(list_edits) = refactor_list(ctx, holder, &scope, &global) else {
                continue;
            };
            edits.extend(list_edits);
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report::fixed(
            node_span(program),
            "Remove the Pest uses that tests/Pest.php already applies.",
            Safety::Safe,
            edits,
        ));
    }
}

/// The statements of one list with their redundant uses gone, printed; none when nothing changes (or the printer
/// would print the list's parent again, which only a file's list allows).
fn refactor_list(
    ctx: &Context<'_>,
    holder: Node<'_, '_>,
    scope: &[Node<'_, '_>],
    global: &HashSet<Vec<u8>>,
) -> Option<Vec<Edit>> {
    let list = statement_list(ctx, holder)?;
    let mut items = Vec::new();
    let mut changed = false;
    for (index, statement) in list.stmts.iter().enumerate() {
        let local = statement_expression(statement).and_then(|e| match_local_use(e));
        let Some(arguments) = local else {
            items.push(Item::Old(index));
            continue;
        };
        // refactorLocalUseStatement: the arguments whose class is applied globally go.
        let args = arguments.arguments.as_slice();
        let kept: Vec<usize> = (0..args.len())
            .filter(|&i| {
                local_class_name(scope, args[i].value()).is_none_or(|c| !global.contains(&c))
            })
            .collect();
        if kept.len() == args.len() {
            items.push(Item::Old(index));
            continue;
        }
        changed = true;
        if kept.is_empty() {
            continue;
        }
        // `$localUse->args = $remainingArgs`, printed by `pArray`: what is before the first argument, then each
        // kept one with the text that was right before it, and what is after the last argument.
        let span = node_span(*statement);
        let open = arguments.left_parenthesis.end.offset as usize;
        let close = arguments.right_parenthesis.start.offset as usize;
        let mut text = ctx.source[span.start..open].to_string();
        text.push_str(&ctx.source[open..node_span(&args[0]).start]);
        for (n, &k) in kept.iter().enumerate() {
            if n > 0 {
                text.push_str(&ctx.source[node_span(&args[k - 1]).end..node_span(&args[k]).start]);
            }
            text.push_str(&ctx.source[node_span(&args[k]).start..node_span(&args[k]).end]);
        }
        text.push_str(&ctx.source[node_span(&args[args.len() - 1]).end..close]);
        text.push_str(&ctx.source[close..span.end]);
        items.push(Item::Changed(index, text));
    }
    if !changed {
        return None;
    }
    if list.nop {
        items.push(Item::Old(list.stmts.len()));
    }
    if let Some(edits) = rewrite_list(ctx, holder, &[], &list, &items) {
        return Some(edits);
    }
    // php-parser gives up on the list of a namespace and prints it again (`pStmt_Namespace`): `namespace X;`, a
    // newline, and each statement on a line of its own. php-parser uses that `;` form for a braced namespace too
    // unless the file has a namespace without a name; such a namespace itself is left alone here.
    let Some(Node::Namespace(ns)) = scope.last() else {
        return None;
    };
    let name = text(ctx, ns.name.as_ref()?);
    let olds = old_statements(ctx, &list);
    let start = node_span(*ns).start;
    // A braced namespace is printed again whole, its `}` too.
    let end = match (&ns.body, olds.last()) {
        (NamespaceBody::Implicit(_), Some(last)) => last.end,
        _ => node_span(*ns).end,
    };
    let printed = format!("namespace {name};\n{}", p_stmts(ctx, &olds, &items, 0));
    Some(vec![Edit::new(Span::new(start, end), printed)])
}

/// `matchLocalUse`: the arguments of `uses(...)`, or of `pest()->use(...)` / `pest()->uses(...)`.
fn match_local_use<'a>(expr: &'a Expression<'a>) -> Option<&'a ArgumentList<'a>> {
    if let Some(call) = func_call(expr) {
        return function_name(call)
            .is_some_and(|n| n.eq_ignore_ascii_case(b"uses"))
            .then_some(&call.argument_list);
    }
    let call = method_call(expr)?;
    let name = identifier(&call.method)?.to_ascii_lowercase();
    if name != b"use" && name != b"uses" {
        return None;
    }
    let pest = func_call(call.object)?;
    function_name(pest)
        .is_some_and(|n| n.eq_ignore_ascii_case(b"pest"))
        .then_some(&call.argument_list)
}

/// `resolveLocalClassName`: the fully qualified class of `Name::class`.
fn local_class_name(scope: &[Node<'_, '_>], expr: &Expression<'_>) -> Option<Vec<u8>> {
    let Expression::Access(Access::ClassConstant(fetch)) = unparenthesize(expr) else {
        return None;
    };
    let ClassLikeConstantSelector::Identifier(name) = &fetch.constant else {
        return None;
    };
    if !name.value.eq_ignore_ascii_case(b"class") {
        return None;
    }
    class_name(scope, fetch.class)
}

/// `resolveClassName` of a class written as a name (`static`, `self` and `parent` stay as they are).
fn class_name(scope: &[Node<'_, '_>], class: &Expression<'_>) -> Option<Vec<u8>> {
    match class {
        Expression::Identifier(id) => Some(resolve_class(scope, id)),
        Expression::Static(k) | Expression::Self_(k) | Expression::Parent(k) => {
            Some(k.value.to_vec())
        }
        _ => None,
    }
}

/// `resolveGloballyAppliedClasses`: the classes tests/Pest.php applies to the directory of `path`.
fn globally_applied_classes(path: &Path) -> HashSet<Vec<u8>> {
    let mut classes = HashSet::new();
    let path = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let Some(tests) = test_directory(&path) else {
        return classes;
    };
    let pest_file = tests.join("Pest.php");
    if !pest_file.is_file() || pest_file == path {
        return classes;
    }
    // `tests` is a directory above `path`; a Pest.php that cannot be read applies nothing.
    let relative = path.strip_prefix(&tests).unwrap_or(&path);
    let relative = relative.to_string_lossy().replace('\\', "/");
    let code = std::fs::read_to_string(&pest_file).unwrap_or_default();
    for (names, paths) in global_uses(&code) {
        if paths.iter().any(|p| covers(p, &relative)) {
            classes.extend(names);
        }
    }
    classes
}

/// `findTestDirectory`: the nearest directory above `path` named `tests` (in any case).
fn test_directory(path: &Path) -> Option<PathBuf> {
    path.parent()?
        .ancestors()
        .find(|d| {
            d.file_name()
                .is_some_and(|n| n.to_string_lossy().to_lowercase() == "tests")
        })
        .map(Path::to_path_buf)
}

/// `pathCoversFile`.
fn covers(configured: &str, relative: &str) -> bool {
    configured == relative
        || relative.starts_with(&format!("{}/", configured.trim_end_matches('/')))
}

/// `parseGlobalUses`: for each top-level `->in(...)` statement of tests/Pest.php, the classes it applies and the
/// paths it applies them to. A file php-parser cannot parse, or whose imports its name resolver rejects, gives none.
fn global_uses(code: &str) -> Vec<(Vec<Vec<u8>>, Vec<String>)> {
    let arena = LocalArena::new();
    let file = File::new(
        Cow::Borrowed(b"Pest.php"),
        FileType::Host,
        None,
        Cow::Owned(code.as_bytes().to_vec()),
    );
    let program = mago_syntax::parser::parse_file(&arena, &file);
    if !program.errors.is_empty() || duplicate_import(program.statements.iter()) {
        return Vec::new();
    }
    let scope = [Node::Program(program)];
    program
        .statements
        .iter()
        .filter_map(|s| match_global_use(&scope, method_call(statement_expression(s)?)?))
        .collect()
}

/// php-parser's `NameResolver` throws on an alias imported twice in one scope (class and function names ignoring
/// case, constants not). Only the file's own imports matter: a file with a namespace has no global uses to read.
fn duplicate_import<'a>(statements: impl Iterator<Item = &'a Statement<'a>>) -> bool {
    let kind = |function: bool| if function { "function" } else { "const" };
    let mut seen = HashSet::new();
    for statement in statements {
        let Statement::Use(r#use) = statement else {
            continue;
        };
        let items: Vec<_> = match &r#use.items {
            UseItems::Sequence(s) => s.items.iter().map(|i| ("class", i)).collect(),
            UseItems::TypedSequence(s) => {
                let k = kind(s.r#type.is_function());
                s.items.iter().map(|i| (k, i)).collect()
            }
            UseItems::TypedList(l) => {
                let k = kind(l.r#type.is_function());
                l.items.iter().map(|i| (k, i)).collect()
            }
            UseItems::MixedList(l) => (l.items.iter())
                .map(|i| {
                    let k = i.r#type.as_ref().map_or("class", |t| kind(t.is_function()));
                    (k, &i.item)
                })
                .collect(),
        };
        for (k, item) in items {
            let alias = match &item.alias {
                Some(a) => a.identifier.value,
                None => item.name.last_segment(),
            };
            let alias = match k {
                "const" => alias.to_vec(),
                _ => alias.to_ascii_lowercase(),
            };
            if !seen.insert((k, alias)) {
                return true;
            }
        }
    }
    false
}

/// `matchGlobalUse`: `uses(...)->in(...)`, or a `pest()` chain whose `->use()` / `->uses()` calls name classes, ended
/// by `->in(...)`.
fn match_global_use(
    scope: &[Node<'_, '_>],
    outer: &MethodCall<'_>,
) -> Option<(Vec<Vec<u8>>, Vec<String>)> {
    if !identifier(&outer.method)?.eq_ignore_ascii_case(b"in") {
        return None;
    }
    let paths = static_paths(&outer.argument_list);
    if paths.is_empty() {
        return None;
    }
    if let Some(uses) = func_call(outer.object)
        && function_name(uses).is_some_and(|n| n.eq_ignore_ascii_case(b"uses"))
    {
        let names = static_class_names(scope, &uses.argument_list);
        return (!names.is_empty()).then_some((names, paths));
    }
    let mut names: Vec<Vec<u8>> = Vec::new();
    let mut current = outer.object;
    while let Some(call) = method_call(current) {
        let name = String::from_utf8_lossy(identifier(&call.method)?).to_lowercase();
        // Of `GLOBAL_CHAIN_TOLERATED_METHODS` (`extend`, `use`, `uses`), the last two name the classes.
        if name == "use" || name == "uses" {
            let configured = static_class_names(scope, &call.argument_list);
            if configured.is_empty() {
                return None;
            }
            for c in configured {
                if !names.contains(&c) {
                    names.push(c);
                }
            }
        }
        current = call.object;
    }
    let pest = func_call(current)?;
    if !function_name(pest).is_some_and(|n| n.eq_ignore_ascii_case(b"pest")) || names.is_empty() {
        return None;
    }
    Some((names, paths))
}

/// `resolveStaticClassNames`: the classes of a list of `Name::class` arguments only.
fn static_class_names(scope: &[Node<'_, '_>], list: &ArgumentList<'_>) -> Vec<Vec<u8>> {
    list.arguments
        .iter()
        .map(|a| local_class_name(scope, a.value()))
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default()
}

/// `resolveStaticPaths`: the paths of a list of plain string arguments, relative, without `..` or a glob.
fn static_paths(list: &ArgumentList<'_>) -> Vec<String> {
    let mut paths = Vec::new();
    for argument in &list.arguments {
        let Some(value) = string_value(argument) else {
            return Vec::new();
        };
        let path = String::from_utf8_lossy(&value).replace('\\', "/");
        if path.starts_with('/') {
            return Vec::new();
        }
        let path = path.trim_matches('/');
        let parent = path.split('/').any(|segment| segment == "..");
        if path.is_empty()
            || parent
            || path.contains(['*', '?', '[', ']', '{', '}'])
            || path.contains(':')
        {
            return Vec::new();
        }
        paths.push(path.to_string());
    }
    paths
}

/// The value of an argument that php-parser reads as a `String_`: a quoted string, or a heredoc without
/// interpolation. An unpacked argument has the same value.
fn string_value(argument: &Argument<'_>) -> Option<Vec<u8>> {
    match unparenthesize(argument.value()) {
        Expression::Literal(Literal::String(s)) => s.value.map(<[u8]>::to_vec),
        Expression::CompositeString(CompositeString::Document(d)) => d
            .parts
            .iter()
            .map(|p| match p {
                StringPart::Literal(l) => Some(l.value.unwrap_or(l.raw).to_vec()),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()
            .map(|parts| parts.concat()),
        _ => None,
    }
}
