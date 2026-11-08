//! `lint/types/add-return-type-from-typed-call`, ported from Rector 2.8.4's `ReturnTypeFromStrictTypedCallRector`
//! (rules/TypeDeclaration/Rector/ClassMethod, with `ReturnStrictTypeAnalyzer` and `TypeNodeUnwrapper`): a function or
//! method whose returns are calls with a declared return type (at least one), class constants or literals gets
//! their declared types: the one type, or, from PHP 8.0, their union.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{Access, Call, Expression, Literal, Node, NodeKind};

use super::declare::{return_declaration, text_order, union_order};
use super::facts::{Facts, facts};
use super::silent_void::Place;
use super::type_factory::{Created, infer_returns};
use super::{
    SCALAR_TYPES, add_return_type, class_scope, has_only_return_with_expr, returns_scoped, untyped,
};
use crate::rules::require_named_argument_order::{Target, callee};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromTypedCall;

/// `PhpVersionFeature::UNION_TYPES`.
const UNION_TYPES: u32 = 80000;

impl Rule for AddReturnTypeFromTypedCall {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-typed-call"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = untyped(node, ancestors, ctx) else {
            return;
        };
        let scope = class_scope(ancestors);
        let facts = facts(ctx);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let returns = returns_scoped(f.body);
        // isUnionPossibleReturnsVoid
        if let Created::Union(types) = infer_returns(&place, f.body, &returns)
            && types.iter().any(|t| matches!(t, TAtomic::Void))
        {
            return;
        }
        if !has_only_return_with_expr(&place, f.body, &returns) {
            return;
        }
        let current = scope.as_ref().and_then(|s| s.name.clone());
        // collectStrictReturnTypes
        let mut nodes: Vec<String> = Vec::new();
        let mut has_call = false;
        for value in returns.iter().filter_map(|r| r.value) {
            let declared = match value {
                Expression::Call(Call::Function(c)) => {
                    has_call = true;
                    call_return(
                        &facts,
                        node_span(value),
                        Target::Function(c.function),
                        ancestors,
                        current.as_deref(),
                        ctx.php_version,
                    )
                }
                Expression::Call(Call::Method(c)) => {
                    has_call = true;
                    call_return(
                        &facts,
                        node_span(value),
                        Target::Method(c.object, &c.method),
                        ancestors,
                        current.as_deref(),
                        ctx.php_version,
                    )
                }
                Expression::Call(Call::StaticMethod(c)) => {
                    has_call = true;
                    call_return(
                        &facts,
                        node_span(value),
                        Target::Static(c.class, &c.method),
                        ancestors,
                        current.as_deref(),
                        ctx.php_version,
                    )
                }
                // `Scope::getType`: PHPStan types a constant by its value where its docblock does not fit
                Expression::Access(Access::ClassConstant(_)) => facts
                    .native_type(node_span(value))
                    .filter(|t| !t.is_mixed())
                    .and_then(|t| declaration(&facts, &t.types, &[], ctx.php_version)),
                Expression::Literal(
                    Literal::String(_) | Literal::Integer(_) | Literal::Float(_),
                )
                | Expression::Array(_)
                | Expression::LegacyArray(_) => facts
                    .full_type(node_span(value))
                    .and_then(|t| declaration(&facts, &t.types, &[], ctx.php_version)),
                _ => return,
            };
            let Some(declared) = declared else {
                return;
            };
            if declared == "void" {
                return;
            }
            if !nodes.contains(&declared) {
                nodes.push(declared);
            }
        }
        if !has_call || nodes.is_empty() {
            return;
        }
        let r#type = match nodes.as_slice() {
            [one] => one.clone(),
            _ if ctx.php_version < UNION_TYPES => return,
            many => {
                // unwrapNullableUnionTypes: `?T` is `T|null`, unions and intersections are their members
                let mut members: Vec<String> = Vec::new();
                for node in many {
                    let parts: Vec<String> = match node.strip_prefix('?') {
                        Some(t) => vec![t.to_string(), "null".to_string()],
                        None => node
                            .split(['|', '&'])
                            .map(|p| p.trim_matches(|c| c == '(' || c == ')').to_string())
                            .collect(),
                    };
                    for p in parts {
                        if !members.contains(&p) {
                            members.push(p);
                        }
                    }
                }
                // the union must map back to a declaration
                let class = members
                    .iter()
                    .any(|m| m.starts_with('\\') || m == "self" || m == "static");
                if members
                    .iter()
                    .any(|m| m == "mixed" || m == "void" || m == "never")
                    || (class && members.iter().any(|m| m == "object"))
                {
                    return;
                }
                members.join("|")
            }
        };
        out.push(add_return_type(f.span, f.at, &r#type));
    }
}

/// `resolveMethodCallReturnNode`: the declaration of the return type of what `target` calls: PHP's own functions
/// and methods by their signature, others by their native return type (`mixed` only when declared); `static` of
/// another class is that class.
fn call_return(
    facts: &Facts,
    call: pun_problem::Span,
    target: Target<'_>,
    ancestors: &[Node<'_, '_>],
    current: Option<&str>,
    php_version: u32,
) -> Option<String> {
    let called = callee(facts, target, ancestors)?;
    let (declared, order) = if called.flags.is_built_in() {
        // `getReturnType()` of the variant `ParametersAcceptorSelector` picks for the arguments: PHPStan's signature
        // of the function; where it depends on the arguments (a conditional type), Mago's type of the call stands
        // in for it, in the signature's order
        let signature = match &called.method_metadata {
            Some(_) => None,
            None => pun_phpstan::symbols::function(&called.name.to_string())
                .and_then(|f| f.variants.first())
                .map(|v| v.ret.as_str()),
        };
        match signature {
            Some(ret) if !ret.contains(" is ") => {
                (signature_types(&facts.codebase, ret)?, Vec::new())
            }
            _ => {
                let mut types: Vec<TAtomic> = facts.full_type(call)?.types.to_vec();
                if let Some(ret) = signature {
                    let order = signature_order(ret);
                    types.sort_by_key(|t| {
                        let key = atomic_key(t);
                        order.iter().position(|o| o == key).unwrap_or(order.len())
                    });
                }
                (types, Vec::new())
            }
        }
    } else {
        let t = called.return_type_declaration_metadata.as_ref()?;
        let order = facts
            .source_of(t.span.file_id)
            .and_then(|s| {
                s.get(t.span.start.offset as usize..t.span.end.offset as usize)
                    .map(text_order)
            })
            .unwrap_or_default();
        (t.type_union.types.to_vec(), order)
    };
    if declared.iter().any(|t| matches!(t, TAtomic::Mixed(_))) {
        return (php_version >= UNION_TYPES).then(|| "mixed".to_string());
    }
    let normalized: Vec<TAtomic> = declared
        .iter()
        .map(|t| match t {
            TAtomic::Object(TObject::Named(n))
                if n.is_static
                    && current.is_none_or(|c| !c.eq_ignore_ascii_case(&n.name.to_string())) =>
            {
                TAtomic::Object(TObject::Named(TNamedObject::new(n.name)))
            }
            other => other.clone(),
        })
        .collect();
    declaration(facts, &normalized, &order, php_version)
}

/// `StaticTypeMapper::mapPHPStanTypeToPhpParserNode` for a return type.
fn declaration(facts: &Facts, t: &[TAtomic], order: &[String], php_version: u32) -> Option<String> {
    if t.is_empty()
        || t.iter().any(|a| {
            matches!(
                a,
                TAtomic::GenericParameter(_) | TAtomic::Mixed(_) | TAtomic::Never
            )
        })
    {
        return None;
    }
    return_declaration(union_order(t), &facts.codebase, php_version, order, true)
}

/// The members of a conditional PHPStan type string (`($num is float ? float : int<0, max>)`) in the order they
/// first appear, as the keys [`return_declaration`] orders by: what a declaration writes for each, its last name
/// segment in lowercase. Generic arguments and shapes are skipped. (The one such signature with integer literals,
/// `db2_autocommit()`, is not in Mago's prelude.)
fn signature_order(ret: &str) -> Vec<String> {
    let mut flat = String::new();
    let mut depth = 0usize;
    for c in ret.chars() {
        match c {
            '<' | '{' => depth += 1,
            '>' | '}' => depth = depth.saturating_sub(1),
            _ if depth == 0 => flat.push(c),
            _ => {}
        }
    }
    let mut order: Vec<String> = Vec::new();
    for word in
        flat.split(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '_' | '\\' | '-' | '$')))
    {
        if word.is_empty() || word.starts_with('$') || word == "is" {
            continue;
        }
        let lower = word.to_ascii_lowercase();
        let key = if lower.ends_with("-string") {
            "string".to_string()
        } else if lower == "list" || lower.ends_with("-array") || lower.ends_with("-list") {
            "array".to_string()
        } else {
            lower.rsplit('\\').next().unwrap_or(&lower).to_string()
        };
        if !order.contains(&key) {
            order.push(key);
        }
    }
    order
}

/// The members of a PHPStan type string without conditional types (`array|string`, `non-empty-string|false`,
/// `list<string>`), as Mago's types; `None` for one a declaration has no word for.
fn signature_types(codebase: &CodebaseMetadata, ret: &str) -> Option<Vec<TAtomic>> {
    let mut ret = ret.trim();
    if let Some(inner) = ret
        .strip_prefix("__benevolent<")
        .and_then(|r| r.strip_suffix('>'))
    {
        ret = inner;
    }
    while let Some(inner) = ret.strip_prefix('(').and_then(|r| r.strip_suffix(')')) {
        ret = inner.trim();
    }
    let mut members: Vec<&str> = Vec::new();
    let (mut depth, mut start) = (0usize, 0usize);
    for (i, c) in ret.char_indices() {
        match c {
            '<' | '{' | '(' => depth += 1,
            '>' | '}' | ')' => depth = depth.saturating_sub(1),
            '|' if depth == 0 => {
                members.push(&ret[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    members.push(&ret[start..]);
    let mut types = Vec::new();
    for member in members {
        let member = member.trim().trim_matches(|c| c == '(' || c == ')').trim();
        let base = member
            .split(['<', '{'])
            .next()
            .unwrap_or(member)
            .trim()
            .to_ascii_lowercase();
        types.push(match base.as_str() {
            "mixed" => TAtomic::Mixed(Default::default()),
            "null" => TAtomic::Null,
            "void" => TAtomic::Void,
            "true" => TAtomic::Scalar(TScalar::r#true()),
            "false" => TAtomic::Scalar(TScalar::r#false()),
            "bool" => TAtomic::Scalar(TScalar::bool()),
            "float" => TAtomic::Scalar(TScalar::float()),
            "object" => TAtomic::Object(TObject::Any),
            // a list or a non-empty array is an intersection in PHPStan, which a return declaration does not write
            "list" | "non-empty-array" | "non-empty-list" => return None,
            "array" => TAtomic::Array(TArray::Keyed(TKeyedArray::new_with_parameters(
                std::sync::Arc::new(mago_codex::ttype::get_arraykey()),
                std::sync::Arc::new(mago_codex::ttype::get_mixed()),
            ))),
            b if b == "int" || b.ends_with("-int") || b.parse::<i64>().is_ok() => {
                TAtomic::Scalar(TScalar::int())
            }
            b if b == "string" || b.ends_with("-string") || b.starts_with('\'') => {
                TAtomic::Scalar(TScalar::string())
            }
            _ => {
                let class = codebase.get_class_like(member.trim_start_matches('\\').as_bytes())?;
                TAtomic::Object(TObject::Named(TNamedObject::new(class.name)))
            }
        });
    }
    Some(types)
}

/// The key [`signature_order`] gives the declaration of `t`.
fn atomic_key(t: &TAtomic) -> &'static str {
    match t {
        TAtomic::Scalar(TScalar::Integer(_)) => "int",
        TAtomic::Scalar(TScalar::Float(_)) => "float",
        TAtomic::Scalar(TScalar::Bool(b)) if b.is_true() => "true",
        TAtomic::Scalar(TScalar::Bool(b)) if b.is_false() => "false",
        TAtomic::Scalar(TScalar::Bool(_)) => "bool",
        TAtomic::Array(_) => "array",
        TAtomic::Null => "null",
        // a string: no conditional signature returns another kind with a declaration
        _ => "string",
    }
}
