//! What lint-b's Rector ports share: a call's arguments as php-parser sees them, the edits php-parser's
//! format-preserving printer makes when an argument is added or removed, class names resolved as `NameResolver` does,
//! and the object type of a variable where the syntax alone proves it (until analyse's types arrive, Build 8).

use mago_syntax::cst::{
    Argument, ArgumentList, Assignment, AssignmentOperator, BinaryOperator, Call, Expression,
    ForeachTarget, Hint, Identifier, Instantiation, Literal, MethodBody, NamespaceBody, Node,
    Statement, UnaryPrefixOperator, UseItem, UseItems, Variable,
};
use pun_problem::{Edit, Span};

use super::rector_code_quality::named;
use super::rector_stmts::{List, line_indent};
use crate::syntax::{node_span, unparenthesize};

/// `Arg::$value`.
pub fn value<'a>(arg: &'a Argument<'a>) -> &'a Expression<'a> {
    match arg {
        Argument::Positional(a) => a.value,
        Argument::Named(a) => a.value,
    }
}

/// `$arg->unpack`.
pub fn unpacked(arg: &Argument<'_>) -> bool {
    matches!(arg, Argument::Positional(a) if a.ellipsis.is_some())
}

/// The edit php-parser's printer makes when it removes the argument at `index`: the argument and the separator before
/// it, or, for the first, the argument and the text up to the next one (the callers never remove a sole argument).
pub fn remove_argument(list: &ArgumentList<'_>, index: usize) -> Edit {
    let items = list.arguments.as_slice();
    let span = node_span(&items[index]);
    if index > 0 {
        Edit::new(Span::new(node_span(&items[index - 1]).end, span.end), "")
    } else {
        Edit::new(Span::new(span.start, node_span(&items[1]).start), "")
    }
}

/// The edit php-parser's printer makes when it appends `code` as a new last argument.
pub fn append_argument(source: &str, list: &ArgumentList<'_>, code: &str) -> Edit {
    let spans: Vec<Span> = list.arguments.iter().map(node_span).collect();
    append_item(
        source,
        &spans,
        list.left_parenthesis.end.offset as usize,
        code,
    )
}

/// The edit for an item php-parser's format-preserving printer appends to a comma-separated list whose items span
/// `spans` and whose `(` ends at `open`: after the last item, or (`emptyListInsertionMap`) right after the `(`.
/// `isMultiline()`: with a line break between every two items, the new one goes on a line of its own at the indent
/// of the last.
pub fn append_item(source: &str, spans: &[Span], open: usize, code: &str) -> Edit {
    let Some(last) = spans.last() else {
        return Edit::new(Span::new(open, open), code.to_string());
    };
    let multiline = spans.len() >= 2
        && spans
            .windows(2)
            .all(|w| source[w[0].end..w[1].end].contains('\n'));
    if multiline {
        let indent = line_indent(source, last.start);
        return Edit::new(Span::new(last.end, last.end), format!(",\n{indent}{code}"));
    }
    Edit::new(Span::new(last.end, last.end), format!(", {code}"))
}

/// The namespace and the class imports (`use A\B as C;`: alias lower-cased => `A\B`) that apply at a node, from its
/// ancestors.
fn scope<'a>(ancestors: &[Node<'a, '_>]) -> (String, Vec<(String, String)>) {
    let mut namespace = String::new();
    let mut statements: Option<&[Statement<'_>]> = None;
    for node in ancestors.iter().rev() {
        match node {
            Node::Namespace(ns) => {
                namespace = ns
                    .name
                    .as_ref()
                    .map(|n| String::from_utf8_lossy(n.value()).into_owned())
                    .unwrap_or_default();
                statements = Some(match &ns.body {
                    NamespaceBody::Implicit(b) => b.statements.as_slice(),
                    NamespaceBody::BraceDelimited(b) => b.statements.as_slice(),
                });
                break;
            }
            Node::Program(p) => {
                statements = Some(p.statements.as_slice());
                break;
            }
            _ => {}
        }
    }
    let mut imports = Vec::new();
    let mut add = |prefix: &str, item: &UseItem<'_>| {
        let name = String::from_utf8_lossy(item.name.value());
        let full = format!("{prefix}{}", name.trim_start_matches('\\'));
        let alias = match &item.alias {
            Some(a) => String::from_utf8_lossy(a.identifier.value).into_owned(),
            None => full.rsplit('\\').next().unwrap_or_default().to_string(),
        };
        imports.push((alias.to_ascii_lowercase(), full));
    };
    for statement in statements.unwrap_or_default() {
        let Statement::Use(u) = statement else {
            continue;
        };
        match &u.items {
            UseItems::Sequence(s) => s.items.iter().for_each(|i| add("", i)),
            UseItems::MixedList(l) => {
                let prefix = format!("{}\\", String::from_utf8_lossy(l.namespace.value()));
                l.items
                    .iter()
                    .filter(|i| i.r#type.is_none())
                    .for_each(|i| add(prefix.trim_start_matches('\\'), &i.item));
            }
            UseItems::TypedSequence(_) | UseItems::TypedList(_) => {}
        }
    }
    (namespace, imports)
}

/// `NameResolver`'s fully qualified class name for `name` (without a leading backslash), as written at a node with
/// these ancestors.
pub fn class_name(ancestors: &[Node<'_, '_>], name: &Identifier<'_>) -> String {
    let written = String::from_utf8_lossy(name.value()).into_owned();
    if let Identifier::FullyQualified(_) = name {
        return written.trim_start_matches('\\').to_string();
    }
    let (namespace, imports) = scope(ancestors);
    if let Some(rest) = written
        .strip_prefix("namespace\\")
        .or_else(|| written.strip_prefix("NAMESPACE\\"))
    {
        return join(&namespace, rest);
    }
    let (first, rest) = match written.split_once('\\') {
        Some((f, r)) => (f, Some(r)),
        None => (written.as_str(), None),
    };
    if let Some((_, full)) = imports
        .iter()
        .find(|(alias, _)| *alias == first.to_ascii_lowercase())
    {
        return match rest {
            Some(rest) => format!("{full}\\{rest}"),
            None => full.clone(),
        };
    }
    join(&namespace, &written)
}

fn join(namespace: &str, name: &str) -> String {
    if namespace.is_empty() {
        name.to_string()
    } else {
        format!("{namespace}\\{name}")
    }
}

/// Whether the syntax proves that `expr` holds an object of class `class` (compared ignoring ASCII case): `new
/// Class(...)` itself, or a variable that is a parameter declared with that class and never written, or that its scope
/// assigns exactly once, with `new Class(...)`, before `at`. PHPStan's `isObjectType` also accepts subclasses, which
/// needs reflection.
pub fn is_object_of(
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    class: &str,
    at: usize,
) -> bool {
    match known(ancestors, expr, at) {
        Some(Known::Hint(Hint::Identifier(id))) => {
            class_name(ancestors, id).eq_ignore_ascii_case(class)
        }
        Some(Known::Value(Expression::Instantiation(new))) => instantiates(ancestors, new, class),
        _ => false,
    }
}

/// What the syntax knows about an expression: the parameter declaration of a variable that is never written, or the
/// one value a variable is assigned.
pub enum Known<'a> {
    Hint(&'a Hint<'a>),
    Value(&'a Expression<'a>),
}

/// See [`Known`]: `expr` itself unless it is a variable; for a variable, its parameter's type when it is never
/// written, or the right side of its one plain assignment before `at` in its scope. An arrow function, and a closure
/// through `use ($x)`, read the variable from the scope around them.
pub fn known<'a>(
    ancestors: &[Node<'a, '_>],
    expr: &'a Expression<'a>,
    at: usize,
) -> Option<Known<'a>> {
    let expr = unparenthesize(expr);
    let Expression::Variable(Variable::Direct(var)) = expr else {
        return Some(Known::Value(expr));
    };
    let name = var.name;
    let mut at = at;
    for scope in ancestors.iter().rev() {
        let (params, inherits) = match scope {
            Node::Function(f) => (Some(&f.parameter_list), false),
            Node::Method(m) => (Some(&m.parameter_list), false),
            Node::Closure(c) => (
                Some(&c.parameter_list),
                c.use_clause.as_ref().is_some_and(|u| {
                    u.variables
                        .iter()
                        .any(|v| v.ampersand.is_none() && v.variable.name == name)
                }),
            ),
            Node::ArrowFunction(a) => (Some(&a.parameter_list), true),
            Node::Program(_) => (None, false),
            _ => continue,
        };
        if let Some(param) =
            params.and_then(|p| p.parameters.iter().find(|p| p.variable.name == name))
        {
            let mut writes = Writes::default();
            writes_of(*scope, name, &mut writes);
            return (writes.other.is_none()
                && writes.assignments.is_empty()
                && param.ellipsis.is_none())
            .then_some(param.hint.as_ref().map(Known::Hint))
            .flatten();
        }
        if inherits {
            // The value at the point the closure or arrow function is created.
            at = node_span(scope).start;
            continue;
        }
        if matches!(scope, Node::Closure(_) | Node::ArrowFunction(_)) {
            return None;
        }
        let mut writes = Writes::default();
        writes_of(*scope, name, &mut writes);
        if writes.other.is_some() {
            return None;
        }
        let plain = |a: &Assignment<'_>| matches!(a.operator, AssignmentOperator::Assign(_));
        return match writes.assignments.as_slice() {
            [assignment] if node_span(*assignment).end <= at && plain(assignment) => {
                Some(Known::Value(unparenthesize(assignment.rhs)))
            }
            // Statements of the scope itself run in order, so the last of them before `at` decides the value.
            assignments if assignments.iter().all(|a| plain(a) && top_level(scope, a)) => {
                assignments
                    .iter()
                    .rev()
                    .find(|a| node_span(**a).end <= at)
                    .map(|a| Known::Value(unparenthesize(a.rhs)))
            }
            _ => None,
        };
    }
    None
}

/// Whether `assignment` is a statement of its own directly in the body of `scope`.
fn top_level(scope: &Node<'_, '_>, assignment: &Assignment<'_>) -> bool {
    let statements = match scope {
        Node::Function(f) => f.body.statements.as_slice(),
        Node::Method(m) => match &m.body {
            MethodBody::Concrete(b) => b.statements.as_slice(),
            MethodBody::Abstract(_) => &[],
        },
        Node::Program(p) => p.statements.as_slice(),
        _ => &[],
    };
    let span = node_span(assignment);
    statements.iter().any(|s| {
        matches!(s, Statement::Expression(e) if node_span(unparenthesize(e.expression)) == span)
    })
}

/// A constant value of PHPStan's type for an expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Int(i64),
    Float(f64),
    String(Vec<u8>),
    Bool(bool),
    Null,
}

/// `ValueResolver::getValue` where the syntax decides it: a literal, `-`/`+` on a number, `~` on a number or string, a
/// constant PHPStan knows the value of (`PHP_INT_MAX` and the like; not the `PHP_FLOAT_*` ones, which it types
/// `float`), or a variable whose one value (see [`known`]) is one of those.
pub fn constant(ancestors: &[Node<'_, '_>], expr: &Expression<'_>, at: usize) -> Option<Constant> {
    let expr = match known(ancestors, expr, at)? {
        Known::Value(v) => v,
        Known::Hint(_) => return None,
    };
    match expr {
        Expression::Literal(Literal::Integer(i)) => {
            Some(match i.value.and_then(|v| i64::try_from(v).ok()) {
                Some(v) => Constant::Int(v),
                None => Constant::Float(float_of(i.raw)?),
            })
        }
        Expression::Literal(Literal::Float(f)) => Some(Constant::Float(f.value.into_inner())),
        Expression::Literal(Literal::String(s)) => Some(Constant::String(s.value?.to_vec())),
        Expression::Literal(Literal::True(_)) => Some(Constant::Bool(true)),
        Expression::Literal(Literal::False(_)) => Some(Constant::Bool(false)),
        Expression::Literal(Literal::Null(_)) => Some(Constant::Null),
        Expression::UnaryPrefix(u)
            if matches!(
                u.operator,
                UnaryPrefixOperator::Negation(_) | UnaryPrefixOperator::Plus(_)
            ) =>
        {
            let negate = matches!(u.operator, UnaryPrefixOperator::Negation(_));
            match constant(ancestors, u.operand, at)? {
                Constant::Int(v) if negate => Some(
                    v.checked_neg()
                        .map_or(Constant::Float(-(v as f64)), Constant::Int),
                ),
                Constant::Float(v) if negate => Some(Constant::Float(-v)),
                c @ (Constant::Int(_) | Constant::Float(_)) => Some(c),
                _ => None,
            }
        }
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::BitwiseNot(_)) => {
            match constant(ancestors, u.operand, at)? {
                Constant::Int(v) => Some(Constant::Int(!v)),
                Constant::Float(v) => Some(Constant::Int(!(v as i64))),
                Constant::String(s) => Some(Constant::String(s.iter().map(|b| !b).collect())),
                _ => None,
            }
        }
        Expression::ConstantAccess(c) => {
            let name = String::from_utf8_lossy(c.name.value());
            match name.trim_start_matches('\\') {
                "PHP_INT_MAX" => Some(Constant::Int(i64::MAX)),
                "PHP_INT_MIN" => Some(Constant::Int(i64::MIN)),
                "PHP_INT_SIZE" => Some(Constant::Int(8)),
                _ => None,
            }
        }
        _ => None,
    }
}

/// PHP reads an integer literal too big for `int` as a float.
fn float_of(raw: &[u8]) -> Option<f64> {
    let digits: String = String::from_utf8_lossy(raw)
        .chars()
        .filter(|c| *c != '_')
        .collect();
    let lower = digits.to_ascii_lowercase();
    let (radix, body) = if let Some(h) = lower.strip_prefix("0x") {
        (16, h)
    } else if let Some(b) = lower.strip_prefix("0b") {
        (2, b)
    } else if let Some(o) = lower.strip_prefix("0o") {
        (8, o)
    } else if lower.len() > 1 && lower.starts_with('0') {
        (8, &lower[1..])
    } else {
        (10, lower.as_str())
    };
    body.chars().try_fold(0f64, |acc, c| {
        Some(acc * f64::from(radix) + f64::from(c.to_digit(radix)?))
    })
}

fn instantiates(ancestors: &[Node<'_, '_>], new: &Instantiation<'_>, class: &str) -> bool {
    match new.class {
        Expression::Identifier(id) => class_name(ancestors, id).eq_ignore_ascii_case(class),
        _ => false,
    }
}

#[derive(Default)]
struct Writes<'ast, 'arena> {
    assignments: Vec<&'ast Assignment<'arena>>,
    /// Where the variable is written some other way: by reference, as a loop variable, in `list()`, `global`,
    /// `static`, `unset` or a closure's `use (&$x)`.
    other: Option<usize>,
}

/// Every write of the variable `name` under `node`; nested functions, closures and classes have their own scope.
fn writes_of<'ast, 'arena>(node: Node<'ast, 'arena>, name: &[u8], out: &mut Writes<'ast, 'arena>) {
    node.visit_children(|child| match child {
        Node::Function(_)
        | Node::Class(_)
        | Node::Interface(_)
        | Node::Trait(_)
        | Node::Enum(_)
        | Node::AnonymousClass(_)
        | Node::ArrowFunction(_) => {}
        Node::Closure(c) => {
            if let Some(uses) = &c.use_clause
                && uses
                    .variables
                    .iter()
                    .any(|u| u.ampersand.is_some() && u.variable.name == name)
            {
                out.other = Some(node_span(c).start);
            }
        }
        Node::Assignment(a) => {
            match a.lhs {
                Expression::Variable(Variable::Direct(v)) if v.name == name => {
                    out.assignments.push(a)
                }
                lhs if mentions(Node::Expression(lhs), name) => {
                    out.other = Some(node_span(a).start)
                }
                _ => {}
            }
            if let Expression::UnaryPrefix(u) = a.rhs
                && u.operator.is_reference()
                && mentions(Node::Expression(u.operand), name)
            {
                out.other = Some(node_span(a).start);
            }
            writes_of(child, name, out);
        }
        Node::Foreach(f) => {
            let target = match &f.target {
                ForeachTarget::Value(t) => mentions(Node::Expression(t.value), name),
                ForeachTarget::KeyValue(t) => {
                    mentions(Node::Expression(t.key), name)
                        || mentions(Node::Expression(t.value), name)
                }
            };
            if target {
                out.other = Some(node_span(f).start);
            }
            writes_of(child, name, out);
        }
        Node::Global(_) | Node::Static(_) | Node::Unset(_) => {
            if mentions(child, name) {
                out.other = Some(node_span(&child).start);
            }
            writes_of(child, name, out);
        }
        _ => writes_of(child, name, out),
    });
}

/// Whether the direct variable `name` appears anywhere under `node`.
fn mentions(node: Node<'_, '_>, name: &[u8]) -> bool {
    if let Node::DirectVariable(v) = node {
        return v.name == name;
    }
    let mut found = false;
    node.visit_children(|child| found = found || mentions(child, name));
    found
}

/// A native type PHPStan's `getNativeType()` proves for an expression, as far as lint-b's rules ask.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Native {
    String,
    Int,
}

/// `getNativeType($expr)->isString()->yes()` or `->isInteger()->yes()`, where the syntax proves it: literals, an
/// interpolated string, `.`, the string and int casts, `-`/`+` on an int, and a variable whose declaration or one value
/// (see [`known`]) is one of those.
pub fn native(ancestors: &[Node<'_, '_>], expr: &Expression<'_>, at: usize) -> Option<Native> {
    match known(ancestors, expr, at)? {
        Known::Hint(Hint::String(_)) => Some(Native::String),
        Known::Hint(Hint::Integer(_)) => Some(Native::Int),
        Known::Hint(_) => None,
        Known::Value(value) => match value {
            Expression::Literal(Literal::String(_)) | Expression::CompositeString(_) => {
                Some(Native::String)
            }
            Expression::Literal(Literal::Integer(i)) => i
                .value
                .and_then(|v| i64::try_from(v).ok())
                .map(|_| Native::Int),
            Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)) => {
                Some(Native::String)
            }
            Expression::UnaryPrefix(u) => match u.operator {
                UnaryPrefixOperator::StringCast(..) | UnaryPrefixOperator::BinaryCast(..) => {
                    Some(Native::String)
                }
                UnaryPrefixOperator::IntCast(..) | UnaryPrefixOperator::IntegerCast(..) => {
                    Some(Native::Int)
                }
                UnaryPrefixOperator::Negation(_) | UnaryPrefixOperator::Plus(_) => {
                    matches!(constant(ancestors, u.operand, at), Some(Constant::Int(_)))
                        .then_some(Native::Int)
                }
                _ => None,
            },
            Expression::Variable(_) => {
                let at = node_span(value).start;
                native(ancestors, value, at)
            }
            _ => None,
        },
    }
}

/// php-parser's `pDereferenceLhs`: `expr` as the left side of `[...]`, `->` or `::`, in parentheses unless it is a
/// variable, a name, a fetch, a call, an array or a plain string.
pub fn dereferenceable(source: &str, expr: &Expression<'_>) -> String {
    let expr = unparenthesize(expr);
    let code = &source[node_span(expr).start..node_span(expr).end];
    match expr {
        Expression::Variable(_)
        | Expression::Identifier(_)
        | Expression::ConstantAccess(_)
        | Expression::ArrayAccess(_)
        | Expression::Access(_)
        | Expression::Call(_)
        | Expression::Array(_)
        | Expression::LegacyArray(_)
        | Expression::Literal(Literal::String(_)) => code.to_string(),
        _ => format!("({code})"),
    }
}

/// The built-in functions whose native return type is `array` (PHP 8.5's reflection, `ReflectionFunction::getReturnType()`
/// without null), which PHPStan's native type of a call to them is.
const ARRAY_FUNCTIONS: &[&str] = &[
    "array_change_key_case",
    "array_chunk",
    "array_column",
    "array_combine",
    "array_count_values",
    "array_diff",
    "array_diff_assoc",
    "array_diff_key",
    "array_diff_uassoc",
    "array_diff_ukey",
    "array_fill",
    "array_fill_keys",
    "array_filter",
    "array_flip",
    "array_intersect",
    "array_intersect_assoc",
    "array_intersect_key",
    "array_intersect_uassoc",
    "array_intersect_ukey",
    "array_keys",
    "array_map",
    "array_merge",
    "array_merge_recursive",
    "array_pad",
    "array_replace",
    "array_replace_recursive",
    "array_reverse",
    "array_slice",
    "array_splice",
    "array_udiff",
    "array_udiff_assoc",
    "array_udiff_uassoc",
    "array_uintersect",
    "array_uintersect_assoc",
    "array_uintersect_uassoc",
    "array_unique",
    "array_values",
    "bcdivmod",
    "bzerror",
    "cal_from_jd",
    "cal_info",
    "compact",
    "date_parse",
    "date_parse_from_format",
    "date_sun_info",
    "dba_handlers",
    "dba_list",
    "debug_backtrace",
    "explode",
    "filter_list",
    "func_get_args",
    "gc_status",
    "gd_info",
    "get_class_methods",
    "get_class_vars",
    "get_declared_classes",
    "get_declared_interfaces",
    "get_declared_traits",
    "get_defined_constants",
    "get_defined_functions",
    "get_defined_vars",
    "get_html_translation_table",
    "get_included_files",
    "get_loaded_extensions",
    "get_mangled_object_vars",
    "get_object_vars",
    "get_required_files",
    "get_resources",
    "getdate",
    "gmp_div_qr",
    "gmp_gcdext",
    "gmp_rootrem",
    "gmp_sqrtrem",
    "hash_algos",
    "hash_hmac_algos",
    "headers_list",
    "imagecolorsforindex",
    "imagegetclip",
    "imap_rfc822_parse_adrlist",
    "intlcal_get_available_locales",
    "iterator_to_array",
    "ldap_get_attributes",
    "libxml_get_errors",
    "localeconv",
    "localtime",
    "mb_encoding_aliases",
    "mb_list_encodings",
    "mb_str_split",
    "mysqli_error_list",
    "mysqli_fetch_all",
    "mysqli_fetch_fields",
    "mysqli_get_client_stats",
    "mysqli_get_connection_stats",
    "mysqli_get_links_stats",
    "mysqli_stmt_error_list",
    "ob_get_status",
    "ob_list_handlers",
    "openssl_get_cert_locations",
    "openssl_get_cipher_methods",
    "openssl_get_md_methods",
    "password_algos",
    "password_get_info",
    "pdo_drivers",
    "pg_fetch_all",
    "pg_fetch_all_columns",
    "pg_jit",
    "pg_version",
    "proc_get_status",
    "range",
    "readline_list_history",
    "realpath_cache_get",
    "request_parse_body",
    "session_get_cookie_params",
    "socket_addrinfo_explain",
    "socket_get_status",
    "sodium_crypto_kx_client_session_keys",
    "sodium_crypto_kx_server_session_keys",
    "sodium_crypto_secretstream_xchacha20poly1305_init_push",
    "spl_autoload_functions",
    "spl_classes",
    "str_getcsv",
    "str_split",
    "stream_context_get_options",
    "stream_context_get_params",
    "stream_get_filters",
    "stream_get_meta_data",
    "stream_get_transports",
    "stream_get_wrappers",
    "timezone_abbreviations_list",
    "timezone_identifiers_list",
    "token_get_all",
];

/// `getNativeType($expr)->isArray()->yes()` where the syntax proves it: an array literal, an `(array)` cast, or a
/// variable declared `array` or whose one value (see [`known`]) is one of those.
pub fn is_native_array(ancestors: &[Node<'_, '_>], expr: &Expression<'_>, at: usize) -> bool {
    match known(ancestors, expr, at) {
        Some(Known::Hint(Hint::Array(_))) => true,
        Some(Known::Value(value)) => match value {
            Expression::Array(_) | Expression::LegacyArray(_) => true,
            Expression::UnaryPrefix(u) => matches!(u.operator, UnaryPrefixOperator::ArrayCast(..)),
            Expression::Variable(_) => is_native_array(ancestors, value, node_span(value).start),
            Expression::Call(Call::Function(call)) => {
                ARRAY_FUNCTIONS.iter().any(|f| named(call, f))
            }
            _ => false,
        },
        _ => false,
    }
}

/// The edit for Rector's removal of the statements `first..=last` of `list` with one new statement added in their
/// place, as php-parser's `pArray()` prints that diff (two removals and an addition, never coalesced into a
/// replacement). After a kept statement, the new one goes on the next line at that statement's indent, so the blanks
/// and comments before the removed statements are lost and those after them are kept. From the list's start, the
/// blanks before the removed statements are kept and the new statement runs straight into the next kept one (or the
/// comments of the trailing `Nop`); with nothing kept after them, php-parser reprints the whole list.
pub fn replace_run(source: &str, list: &List<'_>, first: usize, last: usize, new: &str) -> Edit {
    let end = node_span(list.stmts[last]).end;
    if first > 0 {
        let prev = node_span(list.stmts[first - 1]);
        let indent = line_indent(source, prev.start);
        return Edit::new(Span::new(prev.end, end), format!("\n{indent}{new}"));
    }
    let start = list.with_comments(source, first).start;
    let next = match list.stmts.get(last + 1) {
        Some(_) => Some(list.with_comments(source, last + 1).start),
        None => list.nop_span(source).map(|s| s.start),
    };
    match next {
        Some(next) => Edit::new(
            Span::new(start, next),
            format!("{new}\n{}", line_indent(source, next)),
        ),
        None => Edit::new(Span::new(start, end), new),
    }
}

/// `File::containsHTML()`, which keeps Rector's `HTMLAverseRectorInterface` rules off a file: an `InlineHTML`
/// statement anywhere in it (the newline php-parser's `?>` takes is not one). `node` is the visited node.
pub fn contains_html(node: Node<'_, '_>, ancestors: &[Node<'_, '_>]) -> bool {
    fn walk(node: Node<'_, '_>, found: &mut bool) {
        if *found {
            return;
        }
        if let Node::Statement(Statement::Inline(inline)) = node
            && !(matches!(inline.value, b"\n" | b"\r\n") && inline.span.start.offset > 0)
        {
            *found = true;
            return;
        }
        node.visit_children(|child| walk(child, found));
    }
    let mut found = false;
    walk(ancestors.first().copied().unwrap_or(node), &mut found);
    found
}

/// The number of line breaks in `source[from..to]`.
pub fn lines_between(source: &str, from: usize, to: usize) -> usize {
    source[from..to].bytes().filter(|&b| b == b'\n').count()
}
