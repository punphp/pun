//! Checks that read the syntax alone: ContinueBreakInLoopRule, ThisInGlobalStatementRule,
//! ThisInStaticStatementRule, ReturnNullsafeByRefRule, ArrowFunctionReturnNullsafeByRefRule, InvalidTypesInUnionRule
//! and UsedNamesRule.

use std::collections::HashMap;

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::Checker;
use super::assign::contains_nullsafe;

/// A statement ParentStmtTypesVisitor records that ContinueBreakInLoopRule reads.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Parent {
    /// `for`, `foreach`, `do`, `while` or `switch`.
    Loop,
    Closure,
}

/// What these checks remember during a walk.
#[derive(Default)]
pub struct State {
    /// The loops and closures around a node, innermost last.
    pub parents: Vec<Parent>,
    /// Per function-like with return statements, innermost last: it returns by reference.
    pub by_ref: crate::bitset::BitSet,
}

/// ContinueBreakInLoopRule.
pub fn continue_break(
    c: &mut Checker,
    at: impl HasSpan,
    level: Option<&Expression>,
    keyword: &str,
) {
    // php-parser makes an integer too large for PHP a float, which counts as 1
    let mut value: i128 = match level {
        Some(Expression::Literal(Literal::Integer(i))) => i
            .value
            .filter(|v| i64::try_from(*v).is_ok())
            .map_or(1, i128::from),
        _ => 1,
    };
    let mut in_closure = false;
    for parent in c.syntax.parents.iter().rev() {
        if *parent == Parent::Closure {
            in_closure = true;
            break;
        }
        value -= 1;
        if value == 0 {
            break;
        }
    }
    if in_closure || value > 0 {
        c.report_non_ignorable(
            at,
            &format!("{keyword}.outOfLoop"),
            format!("Keyword {keyword} used outside of a loop or a switch statement."),
            None,
        );
    }
}

/// ThisInGlobalStatementRule.
pub fn global(c: &mut Checker, g: &Global) {
    for v in g.variables.iter() {
        if matches!(v, Variable::Direct(d) if d.name == b"$this") {
            c.report_non_ignorable(
                g,
                "global.this",
                "Cannot use $this as global variable.".to_string(),
                None,
            );
        }
    }
}

/// ThisInStaticStatementRule.
pub fn r#static(c: &mut Checker, s: &Static) {
    for item in s.items.iter() {
        if item.variable().name == b"$this" {
            c.report_non_ignorable(
                s,
                "static.this",
                "Cannot use $this as static variable.".to_string(),
                None,
            );
        }
    }
}

/// ReturnNullsafeByRefRule, for each return statement of the function-like it is in.
pub fn r#return(c: &mut Checker, r: &Return) {
    if c.syntax.by_ref.last() != Some(true) {
        return;
    }
    if r.value.is_some_and(contains_nullsafe) {
        c.report_non_ignorable(
            r,
            "nullsafe.byRef",
            "Nullsafe cannot be returned by reference.".to_string(),
            None,
        );
    }
}

/// ArrowFunctionReturnNullsafeByRefRule.
pub fn arrow_function(c: &mut Checker, f: &ArrowFunction) {
    if f.ampersand.is_some() && contains_nullsafe(f.expression) {
        c.report_non_ignorable(
            f,
            "nullsafe.byRef",
            "Nullsafe cannot be returned by reference.".to_string(),
            None,
        );
    }
}

/// InvalidTypesInUnionRule on a `Node\FunctionLike`: its parameters' and return type's native types.
pub fn function_like_types(
    c: &mut Checker,
    params: Option<&FunctionLikeParameterList>,
    ret: Option<&FunctionLikeReturnTypeHint>,
) {
    for p in params.iter().flat_map(|l| l.parameters.iter()) {
        if let Some(hint) = &p.hint {
            complex_type(c, hint);
        }
    }
    if let Some(ret) = ret {
        complex_type(c, &ret.hint);
    }
}

/// InvalidTypesInUnionRule's processComplexType(), also for a ClassPropertyNode's native type.
pub fn complex_type(c: &mut Checker, hint: &Hint) {
    match hint {
        Hint::Union(_) => {
            let mut types = Vec::new();
            union_members(hint, &mut types);
            if let Some(name) = types.into_iter().find_map(standalone) {
                let name = String::from_utf8_lossy(name).to_ascii_lowercase();
                c.report_non_ignorable(
                    hint,
                    &format!("unionType.{name}"),
                    format!("Type {name} cannot be part of a union type declaration."),
                    None,
                );
            }
        }
        Hint::Nullable(n) => {
            if let Some(name) = standalone(n.hint) {
                let name = String::from_utf8_lossy(name).to_ascii_lowercase();
                c.report_non_ignorable(
                    hint,
                    &format!("nullableType.{name}"),
                    format!("Type {name} cannot be part of a nullable type declaration."),
                    None,
                );
            }
        }
        _ => {}
    }
}

/// `UnionType::$types`: Mago nests a union by pairs.
fn union_members<'h, 'arena>(hint: &'h Hint<'arena>, out: &mut Vec<&'h Hint<'arena>>) {
    match hint {
        Hint::Union(u) => {
            union_members(u.left, out);
            union_members(u.right, out);
        }
        _ => out.push(hint),
    }
}

/// `ONLY_STANDALONE_TYPES` written as an Identifier (not a fully qualified name; php-parser lowercases it).
fn standalone<'arena>(hint: &Hint<'arena>) -> Option<&'arena [u8]> {
    match hint {
        Hint::Mixed(l) | Hint::Never(l) | Hint::Void(l) => Some(l.value),
        _ => None,
    }
}

/// UsedNamesRule: the file's top-level `use` and class-like statements, and those directly in its namespaces. Each
/// namespace declaration starts a new import scope, even for a namespace declared earlier in the file.
pub fn used_names(c: &mut Checker, program: &Program) {
    let mut declared: HashMap<String, Vec<String>> = HashMap::new();
    let mut file_scope = Vec::new();
    for s in program.statements.iter() {
        if let Statement::Namespace(n) = s {
            let namespace = n
                .name
                .as_ref()
                .map(|id| c.written(id.span()))
                .unwrap_or_default();
            let mut namespace_scope = Vec::new();
            for s in n.statements().iter() {
                names_of(c, s, &namespace, &mut declared, &mut namespace_scope);
            }
            continue;
        }
        names_of(c, s, "", &mut declared, &mut file_scope);
    }
}

/// findErrorsForNode().
fn names_of(
    c: &mut Checker,
    s: &Statement,
    namespace: &str,
    declared: &mut HashMap<String, Vec<String>>,
    scope: &mut Vec<String>,
) {
    let (kind, name, at) = match s {
        Statement::Use(u) => {
            let (prefix, items): (String, Vec<&UseItem>) = match &u.items {
                UseItems::Sequence(s) => (String::new(), s.items.iter().collect()),
                UseItems::TypedSequence(_) | UseItems::TypedList(_) => return,
                UseItems::MixedList(l) => (
                    c.written(l.namespace.span()),
                    l.items
                        .iter()
                        .filter(|i| i.r#type.is_none())
                        .map(|i| &i.item)
                        .collect(),
                ),
            };
            uses(c, &prefix, &items, scope);
            return;
        }
        Statement::Class(k) => ("class", k.name.value, s.span()),
        Statement::Interface(k) => ("interface", k.name.value, s.span()),
        Statement::Trait(k) => ("trait", k.name.value, s.span()),
        Statement::Enum(k) => ("enum", k.name.value, s.span()),
        _ => return,
    };
    let name = String::from_utf8_lossy(name).into_owned();
    let lower = name.to_ascii_lowercase();
    let names = declared.entry(namespace.to_ascii_lowercase()).or_default();
    if scope.contains(&lower) || names.contains(&lower) {
        let full = if namespace.is_empty() {
            name
        } else {
            format!("{namespace}\\{name}")
        };
        c.report_non_ignorable(
            at,
            &format!("{kind}.nameInUse"),
            format!("Cannot declare {kind} {full} because the name is already in use."),
            None,
        );
        return;
    }
    scope.push(lower.clone());
    names.push(lower);
}

/// findErrorsInUses().
fn uses(
    c: &mut Checker,
    prefix: &str,
    items: &[&UseItem],
    scope: &mut Vec<String>,
) {
    for item in items {
        // php-parser drops a use name's leading `\`
        let name = c.written(item.name.span());
        let name = name.trim_start_matches('\\');
        let alias = match &item.alias {
            Some(a) => String::from_utf8_lossy(a.identifier.value).into_owned(),
            None => name.rsplit('\\').next().unwrap_or(name).to_string(),
        };
        if scope.contains(&alias.to_ascii_lowercase()) {
            let full = if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{prefix}\\{name}")
            };
            c.report_non_ignorable(
                *item,
                "use.nameInUse",
                format!("Cannot use {full} as {alias} because the name is already in use."),
                None,
            );
            continue;
        }
        scope.push(alias.to_ascii_lowercase());
    }
}
