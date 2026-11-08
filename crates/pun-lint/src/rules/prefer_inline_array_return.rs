//! `lint/complexity/prefer-inline-array-return`, ported from Rector 2.8.4's `InlineArrayReturnAssignRector`
//! (rules/CodeQuality/Rector/ClassMethod), with `VariableDimFetchAssignResolver`.
//!
//! A statement list of three or more statements that only fills an array variable (`$a = [];`, then `$a['k'] = v;`,
//! `$a[] = v;`, `$a['k']['m'] = v;`) and ends with `return $a;` becomes `return ['k' => v, ...];`. Outside a
//! function, method or closure, the list must start the array with `= []` (the first such assignment counts). The
//! variable must not be a parameter, nor a closure's by-reference `use`. A `new` with arguments, a key that is not
//! known at compile time (`ExprAnalyzer::isDynamicExpr`), or a nested key under a value already set to an array
//! leaves the list alone.
//!
//! The keys are the PHP array keys Rector's `ValueResolver::getValue()` computes: `'1'` is `1`, `1.7` is `1`, `true`
//! is `1`, `null` is `''`, `C::class` is the class's name, a class constant its value, and `$a[]` takes the number
//! of statements read before it. Assigning a key again keeps its first place. `NodeFactory::createArray()` writes a
//! key only where it differs from the item's position, and Rector prints the array on one line with the values'
//! code, moved to the new indent; every comment of the list goes.
//!
//! Where Rector writes a key that means something else than the code (a global constant or a magic constant by its
//! name, an enum case or an unknown class constant as `'C::NAME'`, a heredoc), and where its run fails (a nested key
//! under a value that is not an array), Pun leaves the list alone.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{
    Access, AssignmentOperator, ClassLikeConstantSelector, Expression, Literal, Node, NodeKind,
    Statement, UnaryPrefixOperator, Variable,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::class_method::{qualified, type_of};
use super::php_parser::{List, Token, enclosing, is_dynamic_expr, previous_token_end, tokens};
use super::rector_batch5::resolve_class_name;
use super::rector_conditions::{comments_between, indentation, reprint};
use super::types::facts::{Facts, facts};
use super::used_variable::same_name;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferInlineArrayReturn;

impl Rule for PreferInlineArrayReturn {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-inline-array-return"
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
        let Node::Statement(Statement::Return(r#return)) = node else {
            return;
        };
        let Some(Expression::Variable(Variable::Direct(returned))) =
            r#return.value.map(unparenthesize)
        else {
            return;
        };
        if ancestors.is_empty() {
            return;
        }
        let source = ctx.source;
        let tokens = tokens(ctx);
        let mut chain = ancestors.to_vec();
        chain.push(node);
        let (list, index) = enclosing(source, &tokens, &chain);
        let count = list.stmts.len();
        if count < 3 || index + 1 != count || list.nop {
            return;
        }
        let name = &returned.name[1..];
        let function_like = function_like(ancestors);
        match function_like {
            Some(FunctionLike::Parameters(params)) if params.iter().any(|p| same_name(name, p)) => {
                return;
            }
            _ => {}
        }
        let empty_assign = list.stmts[..count - 1]
            .iter()
            .position(|s| is_empty_assign(s, name));
        if !exclusive(&list, empty_assign, name)
            || (empty_assign.is_none() && function_like.is_none())
        {
            return;
        }
        let facts = facts(ctx);
        let Some(items) = resolve(&facts, ancestors, &list, empty_assign) else {
            return;
        };
        if items.is_empty() {
            return;
        }
        let first = node_span(list.stmts[0]);
        let before = previous_token_end(source, &tokens, first.start).max(list.inner.start);
        let start = comments_between(&tokens, before, first.start)
            .first()
            .map_or(first.start, |c| c.start);
        let level = indentation(source, first.start);
        let array = print(source, &tokens, level, &items);
        let span = Span::new(start, node_span(list.stmts[count - 1]).end);
        out.push(Report {
            span: node_span(list.stmts[0]),
            message: "Return the array directly instead of filling a variable.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, format!("return {array};"))],
            }),
        });
    }
}

enum FunctionLike {
    /// The names of the parameters and the by-reference `use` variables (without `$`).
    Parameters(Vec<Vec<u8>>),
}

/// The function, method or closure whose body the list is.
fn function_like(ancestors: &[Node<'_, '_>]) -> Option<FunctionLike> {
    let n = ancestors.len();
    if !matches!(ancestors.get(n.checked_sub(1)?), Some(Node::Block(_))) {
        return None;
    }
    let owner = ancestors[..n - 1]
        .iter()
        .rev()
        .find(|a| !matches!(a, Node::MethodBody(_)))?;
    let name = |p: &mago_syntax::cst::FunctionLikeParameter<'_>| p.variable.name[1..].to_vec();
    let mut names: Vec<Vec<u8>> = match owner {
        Node::Method(m) => m.parameter_list.parameters.iter().map(name).collect(),
        Node::Function(f) => f.parameter_list.parameters.iter().map(name).collect(),
        Node::Closure(c) => c.parameter_list.parameters.iter().map(name).collect(),
        _ => return None,
    };
    if let Node::Closure(c) = owner
        && let Some(clause) = &c.use_clause
    {
        names.extend(
            clause
                .variables
                .iter()
                .filter(|v| v.ampersand.is_some())
                .map(|v| v.variable.name[1..].to_vec()),
        );
    }
    Some(FunctionLike::Parameters(names))
}

/// An assignment `$name = ...` (not by reference) in an expression statement: its left and right side.
fn assignment<'a>(
    statement: &'a Statement<'a>,
) -> Option<(&'a Expression<'a>, &'a Expression<'a>)> {
    let Statement::Expression(e) = statement else {
        return None;
    };
    let Expression::Assignment(a) = e.expression else {
        return None;
    };
    if !matches!(a.operator, AssignmentOperator::Assign(_))
        || matches!(a.rhs, Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Reference(_)))
    {
        return None;
    }
    Some((a.lhs, a.rhs))
}

/// `resolveDefaultEmptyArrayAssign()`: `$name = []` or `$name = array()`.
fn is_empty_assign(statement: &Statement<'_>, name: &[u8]) -> bool {
    let Some((Expression::Variable(Variable::Direct(v)), rhs)) = assignment(statement) else {
        return false;
    };
    same_name(name, &v.name[1..])
        && match rhs {
            Expression::Array(a) => a.elements.is_empty(),
            Expression::LegacyArray(a) => a.elements.is_empty(),
            _ => false,
        }
}

/// `areAssignExclusiveToDimFetchVariable()`.
fn exclusive(list: &List<'_, '_>, empty_assign: Option<usize>, name: &[u8]) -> bool {
    let last = list.stmts.len() - 1;
    list.stmts[..last].iter().enumerate().all(|(k, s)| {
        let Some((lhs, rhs)) = assignment(s) else {
            return false;
        };
        if Some(k) == empty_assign {
            return true;
        }
        if let Expression::Instantiation(new) = rhs
            && new
                .argument_list
                .as_ref()
                .is_some_and(|l| !l.arguments.is_empty())
        {
            return false;
        }
        let mut var = lhs;
        if !matches!(var, Expression::ArrayAccess(_) | Expression::ArrayAppend(_)) {
            return false;
        }
        loop {
            match var {
                Expression::ArrayAccess(a) => var = a.array,
                Expression::ArrayAppend(a) => var = a.array,
                _ => break,
            }
        }
        matches!(var, Expression::Variable(Variable::Direct(v)) if same_name(name, &v.name[1..]))
    })
}

/// A PHP array key.
#[derive(Clone, PartialEq)]
enum Key {
    Int(i64),
    Str(String),
}

/// A value of the array Rector builds: an expression of the file, or an array of its own.
enum Value {
    Expr(Span),
    Map(Vec<(Key, Value)>),
}

/// `VariableDimFetchAssignResolver::resolveFromStmtsAndVariable()`, or `None` where Rector leaves the list alone,
/// fails, or writes a key that means something else.
fn resolve(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    list: &List<'_, '_>,
    empty_assign: Option<usize>,
) -> Option<Vec<(Key, Value)>> {
    let mut items: Vec<(Key, Value)> = Vec::new();
    let mut key = 0i64;
    for (k, statement) in list.stmts[..list.stmts.len() - 1].iter().enumerate() {
        if Some(k) == empty_assign {
            continue;
        }
        let (lhs, rhs) = assignment(statement)?;
        let mut dims = Vec::new();
        let mut var = lhs;
        loop {
            match var {
                Expression::ArrayAccess(a) => {
                    if is_dynamic_expr(a.index) {
                        return Some(Vec::new());
                    }
                    dims.push(key_value(facts, ancestors, a.index)?);
                    var = a.array;
                }
                Expression::ArrayAppend(a) => {
                    dims.push(Key::Int(key));
                    var = a.array;
                }
                _ => break,
            }
        }
        key += 1;
        dims.reverse();
        set(&mut items, &dims, Value::Expr(node_span(rhs)))?;
    }
    Some(items)
}

/// `setNestedKeysExpr()`: `None` where a key path runs through a value that is not an array of Rector's own.
fn set(items: &mut Vec<(Key, Value)>, keys: &[Key], value: Value) -> Option<()> {
    let (first, rest) = keys.split_first()?;
    let at = match items.iter().position(|(k, _)| k == first) {
        Some(at) => at,
        None => {
            items.push((first.clone(), Value::Map(Vec::new())));
            items.len() - 1
        }
    };
    if rest.is_empty() {
        items[at].1 = value;
        return Some(());
    }
    match &mut items[at].1 {
        Value::Map(map) => set(map, rest, value),
        // an `Array_` throws `NotImplementedYetException`; any other expression fails Rector's run
        Value::Expr(_) => None,
    }
}

/// `ValueResolver::getValue()` of a key Pun takes, as PHP stores it in an array.
fn key_value(facts: &Facts, ancestors: &[Node<'_, '_>], index: &Expression<'_>) -> Option<Key> {
    let index = unparenthesize(index);
    match index {
        Expression::Literal(Literal::String(s)) => {
            Some(string_key(std::str::from_utf8(s.value?).ok()?))
        }
        Expression::Literal(Literal::Integer(i)) => Some(Key::Int(i64::try_from(i.value?).ok()?)),
        Expression::Literal(Literal::Float(f)) => float_key(f.value.0),
        Expression::Literal(Literal::True(_)) => Some(Key::Int(1)),
        Expression::Literal(Literal::False(_)) => Some(Key::Int(0)),
        Expression::Literal(Literal::Null(_)) => Some(Key::Str(String::new())),
        Expression::ConstantAccess(c) => {
            let name = c.name.value();
            let name = name.strip_prefix(b"\\").unwrap_or(name);
            if name.eq_ignore_ascii_case(b"true") {
                Some(Key::Int(1))
            } else if name.eq_ignore_ascii_case(b"false") {
                Some(Key::Int(0))
            } else if name.eq_ignore_ascii_case(b"null") {
                Some(Key::Str(String::new()))
            } else {
                None
            }
        }
        Expression::UnaryPrefix(u) => {
            let sign = match u.operator {
                UnaryPrefixOperator::Plus(_) => 1,
                UnaryPrefixOperator::Negation(_) => -1,
                _ => return None,
            };
            match unparenthesize(u.operand) {
                Expression::Literal(Literal::Integer(i)) => {
                    Some(Key::Int(sign * i64::try_from(i.value?).ok()?))
                }
                Expression::Literal(Literal::Float(f)) => float_key(sign as f64 * f.value.0),
                _ => None,
            }
        }
        Expression::Access(Access::ClassConstant(c)) => {
            let ClassLikeConstantSelector::Identifier(constant) = &c.constant else {
                return None;
            };
            let class = match c.class {
                Expression::Identifier(id) => resolve_class_name(ancestors, id),
                Expression::Self_(_) => this_class(ancestors)?,
                Expression::Parent(_) => {
                    let this = this_class(ancestors)?;
                    let parent = facts
                        .codebase
                        .get_class_like(this.as_bytes())?
                        .direct_parent_class?;
                    String::from_utf8_lossy(parent.as_bytes()).into_owned()
                }
                _ => return None,
            };
            if constant.value.eq_ignore_ascii_case(b"class") {
                return Some(Key::Str(class));
            }
            // the analyser folds a constant made of other constants and operators (`A::B = TOP . 'x'`)
            literal_key(facts, index).or_else(|| class_constant(facts, &class, constant.value))
        }
        _ => None,
    }
}

/// The key of an expression the analyser types as one literal integer or string.
fn literal_key(facts: &Facts, expr: &Expression<'_>) -> Option<Key> {
    let t = type_of(facts, expr)?;
    let [atomic] = &t.types[..] else { return None };
    match atomic {
        TAtomic::Scalar(TScalar::Integer(i)) => Some(Key::Int(i.get_literal_value()?)),
        TAtomic::Scalar(s @ TScalar::String(_)) => Some(string_key(
            std::str::from_utf8(s.get_known_literal_string_value()?).ok()?,
        )),
        _ => None,
    }
}

/// The value of the class constant `class::name`, when it is an integer or a string Mago knows (an enum case is
/// none).
fn class_constant(facts: &Facts, class: &str, name: &[u8]) -> Option<Key> {
    let metadata = facts.codebase.get_class_like(class.as_bytes())?;
    if metadata.kind.is_enum() && metadata.enum_cases.keys().any(|c| c.as_bytes() == name) {
        return None;
    }
    let owners = std::iter::once(class.as_bytes().to_vec())
        .chain(
            metadata
                .all_parent_classes
                .iter()
                .map(|c| c.as_bytes().to_vec()),
        )
        .chain(
            metadata
                .all_parent_interfaces
                .iter()
                .map(|c| c.as_bytes().to_vec()),
        );
    for owner in owners {
        if let Some(constant) = facts.codebase.get_class_constant(&owner, name) {
            return match constant.inferred_type.as_ref().filter(|t| t.is_single())?.get_single() {
                TAtomic::Scalar(TScalar::Integer(i)) => Some(Key::Int(i.get_literal_value()?)),
                TAtomic::Scalar(s @ TScalar::String(_)) => Some(string_key(
                    std::str::from_utf8(s.get_known_literal_string_value()?).ok()?,
                )),
                _ => None,
            };
        }
    }
    None
}

/// The class `self` names: the enclosing class, enum, interface or trait.
fn this_class(ancestors: &[Node<'_, '_>]) -> Option<String> {
    ancestors
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, a)| match a {
            Node::Class(c) => Some(qualified(&ancestors[..i], c.name.value)),
            Node::Enum(e) => Some(qualified(&ancestors[..i], e.name.value)),
            Node::Interface(e) => Some(qualified(&ancestors[..i], e.name.value)),
            _ => None,
        })
}

/// PHP's key for a string: a decimal integer without leading zeros or `+` is that integer.
fn string_key(s: &str) -> Key {
    let digits = s.strip_prefix('-').unwrap_or(s);
    let canonical = !digits.is_empty()
        && digits.bytes().all(|b| b.is_ascii_digit())
        && (digits == "0" || !digits.starts_with('0'))
        && s != "-0";
    match s.parse::<i64>() {
        Ok(i) if canonical => Key::Int(i),
        _ => Key::Str(s.to_string()),
    }
}

/// PHP's key for a float: truncated, when finite and in range.
fn float_key(f: f64) -> Option<Key> {
    (f.is_finite() && f.abs() < 9.2e18).then(|| Key::Int(f.trunc() as i64))
}

/// `NodeFactory::createArray()` printed: `[item, key => item]` on one line.
fn print(source: &str, tokens: &[Token], level: usize, items: &[(Key, Value)]) -> String {
    let printed: Vec<String> = items
        .iter()
        .enumerate()
        .map(|(position, (key, value))| {
            let value = match value {
                Value::Expr(span) => reprint(
                    source,
                    tokens,
                    *span,
                    level as isize - indentation(source, span.start) as isize,
                ),
                Value::Map(map) => print(source, tokens, level, map),
            };
            match key {
                Key::Int(i) if *i == position as i64 => value,
                Key::Int(i) => format!("{i} => {value}"),
                Key::Str(s) => format!("{} => {value}", single_quoted(s)),
            }
        })
        .collect();
    format!("[{}]", printed.join(", "))
}

/// php-parser's `pSingleQuotedString()`: `'` and a `\` before `'`, `\` or the end get a `\`.
fn single_quoted(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::from("'");
    for (i, c) in s.char_indices() {
        if c == '\'' || (c == '\\' && matches!(bytes.get(i + 1), None | Some(b'\'' | b'\\'))) {
            out.push('\\');
        }
        out.push(c);
    }
    out.push('\'');
    out
}
