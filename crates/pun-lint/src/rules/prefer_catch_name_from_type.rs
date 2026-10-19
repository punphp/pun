//! `lint/style/prefer-catch-name-from-type`, ported from Rector 2.6.1's `CatchExceptionNameMatchingTypeRector`
//! (rules/CodingStyle/Rector/Catch_) and `PropertyNaming::fqnToVariableName` (rules/Naming/Naming).
//!
//! A `try` among the statements of a function, method, closure, namespace or a file without one, with one `catch`
//! of one type and a variable, names the variable after the type as written (`catch (PDOException $e)` takes
//! `$pdoException`), unless the scope already has that variable for sure, which stops the renaming of the `try`s
//! after it too. The variable is renamed in the `catch` body, and in the statements after the `try` as Rector walks
//! them: the next one, then every second one from the one after it, each up to its first assignment to a variable or
//! a `catch` that names either variable.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::no_for_init_overwrite::defined;
use super::rector_batch5::resolve_class_name;
use super::rector_stmts::holder_list;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferCatchNameFromType;

impl Rule for PreferCatchNameFromType {
    fn id(&self) -> &'static str {
        "lint/style/prefer-catch-name-from-type"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Statement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if let Node::Statement(statement) = node {
            out.extend(rename(ctx, statement, ancestors));
        }
    }
}

fn rename(
    ctx: &Context<'_>,
    statement: &Statement<'_>,
    ancestors: &[Node<'_, '_>],
) -> Option<Report> {
    let sole = sole_catch(statement)?;
    let (catch, _, variable) = sole;
    let (&holder, above) = ancestors.split_last()?;
    if !is_rector_holder(holder, above) {
        return None;
    }
    let list = holder_list(ctx.source, holder, above)?;
    let key = list
        .stmts
        .iter()
        .position(|s| node_span(*s) == node_span(statement))?;
    // `return null` on a name the scope has stops the `try`s after it.
    for earlier in list.stmts.iter().take(key + 1) {
        if let Some((old, new)) = sole_catch(earlier).map(names)
            && old != new
            && defined(
                Node::Statement(earlier),
                ancestors,
                format!("${new}").as_bytes(),
            )
        {
            return None;
        }
    }
    let (old, new) = names(sole);
    if old == new {
        return None;
    }
    let (old_var, new_var) = (format!("${old}"), format!("${new}"));
    let mut edits = vec![Edit::new(node_span(variable), new_var.clone())];
    let mut renamed = Vec::new();
    collect(Node::Block(&catch.block), old_var.as_bytes(), &mut renamed);
    // `replaceNextUsageVariable`: the statement after the `try`, then every second one from the one after that.
    let mut visit = |stmt: &Statement<'_>| {
        let mut stopped = false;
        next_usages(
            ancestors,
            Node::Statement(stmt),
            &old_var,
            &new_var,
            &mut renamed,
            &mut stopped,
        );
    };
    if let Some(stmt) = list.stmts.get(key + 1) {
        visit(stmt);
        let mut at = key;
        while list.stmts.get(at + 1).is_some()
            && let Some(stmt) = list.stmts.get(at + 2)
        {
            visit(stmt);
            at += 2;
        }
    }
    edits.extend(
        renamed
            .into_iter()
            .map(|span| Edit::new(span, new_var.clone())),
    );
    Some(Report {
        span: node_span(variable),
        message: format!("Name the caught exception `{new_var}` after its type."),
        help: None,
        fix: Some(Fix {
            safety: Safety::Safe,
            edits,
        }),
    })
}

/// `shouldSkip`'s complement: the `catch` of a `try` with one `catch` of one type that has a variable.
type Catch<'a> = (
    &'a TryCatchClause<'a>,
    &'a Identifier<'a>,
    &'a DirectVariable<'a>,
);

fn sole_catch<'a>(statement: &'a Statement<'a>) -> Option<Catch<'a>> {
    let Statement::Try(r#try) = statement else {
        return None;
    };
    let [catch] = r#try.catch_clauses.as_slice() else {
        return None;
    };
    match (&catch.hint, &catch.variable) {
        (Hint::Identifier(id), Some(variable)) => Some((catch, id, variable)),
        _ => None,
    }
}

/// The variable's name and the one its type gives it, both without `$`.
fn names((_, id, variable): Catch<'_>) -> (String, String) {
    let written = String::from_utf8_lossy(id.value());
    let written = written.trim_start_matches('\\');
    let old = String::from_utf8_lossy(&variable.name[1..]).into_owned();
    (old, fqn_to_variable_name(&new_variable_name(written)))
}

/// Whether the statement list `holder` is one Rector visits: a function's, method's or closure's body, a namespace's,
/// or a file's without a namespace.
fn is_rector_holder(holder: Node<'_, '_>, above: &[Node<'_, '_>]) -> bool {
    let parent = above
        .iter()
        .rev()
        .find(|a| !matches!(a, Node::MethodBody(_) | Node::NamespaceBody(_)));
    match holder {
        Node::Program(program) => !program
            .statements
            .iter()
            .any(|s| matches!(s, Statement::Namespace(_))),
        Node::NamespaceImplicitBody(_) => true,
        Node::Block(_) => matches!(
            parent,
            Some(Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::Namespace(_))
        ),
        _ => false,
    }
}

/// Every variable named `name` under `node`.
fn collect(node: Node<'_, '_>, name: &[u8], out: &mut Vec<pun_problem::Span>) {
    if let Node::DirectVariable(v) = node
        && v.name == name
    {
        out.push(node_span(v));
    }
    node.visit_children(|child| collect(child, name, out));
}

/// The variables named `old` in `node`, in php-parser's order, up to an assignment to a variable or a `catch` that
/// `shouldSkipFollowingCatch`.
fn next_usages(
    ancestors: &[Node<'_, '_>],
    node: Node<'_, '_>,
    old: &str,
    new: &str,
    out: &mut Vec<pun_problem::Span>,
    stopped: &mut bool,
) {
    if *stopped {
        return;
    }
    match node {
        Node::Assignment(a)
            if matches!(a.operator, AssignmentOperator::Assign(_))
                && matches!(a.lhs, Expression::Variable(_)) =>
        {
            *stopped = true;
            return;
        }
        Node::TryCatchClause(c) if skip_following_catch(ancestors, c, old, new) => {
            *stopped = true;
            return;
        }
        Node::DirectVariable(v) if v.name == old.as_bytes() => out.push(node_span(v)),
        _ => {}
    }
    node.visit_children(|child| next_usages(ancestors, child, old, new, out, stopped));
}

fn skip_following_catch(
    ancestors: &[Node<'_, '_>],
    catch: &TryCatchClause<'_>,
    old: &str,
    new: &str,
) -> bool {
    if let Some(v) = &catch.variable {
        return v.name == old.as_bytes() || v.name == new.as_bytes();
    }
    match &catch.hint {
        Hint::Identifier(id) => format!("${}", lcfirst(&resolve_class_name(ancestors, id))) == new,
        _ => false,
    }
}

fn lcfirst(s: &str) -> String {
    let mut out = s.to_string();
    if let Some(first) = out.get_mut(0..1) {
        first.make_ascii_lowercase();
    }
    out
}

/// `resolveNewVariableName`: `lcfirst`, and a leading abbreviation lowered (`#^([A-Za-z]+?)([A-Z]{1}[a-z]{1})([A-Za-z]*)#`).
fn new_variable_name(type_name: &str) -> String {
    let name = lcfirst(type_name);
    let b = name.as_bytes();
    for k in 1..b.len().saturating_sub(1) {
        if !b[k - 1].is_ascii_alphabetic() {
            break;
        }
        if b[k].is_ascii_uppercase() && b[k + 1].is_ascii_lowercase() {
            return format!("{}{}", name[..k].to_ascii_lowercase(), &name[k..]);
        }
    }
    name
}

/// `PropertyNaming::fqnToVariableName` of `new ObjectType($class)`.
fn fqn_to_variable_name(class: &str) -> String {
    let short = class.rsplit('\\').next().unwrap_or(class);
    let name = remove_interface_suffix_prefix(short, "interface");
    let name = remove_interface_suffix_prefix(&name, "abstract");
    let name = name.replace('_', "");
    // `prolongIfTooShort`
    if (name == "Factory" || name == "Repository")
        && !class.ends_with("Repository")
        && !class.ends_with("Factory")
    {
        let parts: Vec<&str> = class.split('\\').collect();
        let above = if parts.len() >= 3 {
            parts[parts.len() - 2]
        } else {
            ""
        };
        return format!("{}{name}", lcfirst(above));
    }
    lcfirst(&name)
}

fn remove_interface_suffix_prefix(class: &str, category: &str) -> String {
    let lower = class.to_ascii_lowercase();
    if lower.ends_with(category) {
        return class[..class.len() - category.len()].to_string();
    }
    if lower.starts_with(category) {
        return class[category.len()..].to_string();
    }
    let b = class.as_bytes();
    if b.len() >= 2 && b[0] == b'I' && b[1].is_ascii_uppercase() {
        return class[1..].to_string();
    }
    class.to_string()
}
