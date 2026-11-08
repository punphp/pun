//! Declaration checks on function-likes: RedefinedParametersRule, VariadicParametersDeclarationRule,
//! InvalidParameterNameRule, InvalidLexicalVariablesInClosureUseRule and InnerFunctionRule.

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::Checker;
use crate::text;

/// `Scope::SUPERGLOBAL_VARIABLES`.
pub(super) const SUPERGLOBALS: [&[u8]; 9] = [
    b"GLOBALS",
    b"_SERVER",
    b"_GET",
    b"_POST",
    b"_FILES",
    b"_COOKIE",
    b"_SESSION",
    b"_REQUEST",
    b"_ENV",
];

/// `$param->var->name`: the parameter's name without its `$`.
fn name<'arena>(v: &DirectVariable<'arena>) -> &'arena [u8] {
    &v.name[1..]
}

/// The rules registered for `Node\FunctionLike` (functions, methods, closures, arrow functions, property hooks),
/// reported at `at`, the node's start.
pub(super) fn function_like(
    c: &mut Checker,
    at: impl HasSpan + Copy,
    params: &FunctionLikeParameterList,
) {
    redefined_parameters(c, at, params);
    variadic_parameters_declaration(c, at, params);
}

/// RedefinedParametersRule.
fn redefined_parameters(
    c: &mut Checker,
    at: impl HasSpan + Copy,
    params: &FunctionLikeParameterList,
) {
    let mut seen: Vec<&[u8]> = Vec::new();
    for p in &params.parameters {
        let var = name(&p.variable);
        if !seen.contains(&var) {
            seen.push(var);
            continue;
        }
        c.report_non_ignorable(
            at,
            "parameter.duplicate",
            format!("Redefinition of parameter ${}.", text(var)),
            None,
        );
    }
}

/// VariadicParametersDeclarationRule.
fn variadic_parameters_declaration(
    c: &mut Checker,
    at: impl HasSpan + Copy,
    params: &FunctionLikeParameterList,
) {
    let count = params.parameters.len();
    for (index, p) in params.parameters.iter().enumerate() {
        if p.ellipsis.is_none() || index == count - 1 {
            continue;
        }
        c.report_non_ignorable(
            at,
            "parameter.variadicNotLast",
            "Only the last parameter can be variadic.".to_string(),
            None,
        );
    }
}

/// InvalidParameterNameRule (`Node\Param`).
pub(super) fn parameter(c: &mut Checker, p: &FunctionLikeParameter) {
    let var = name(&p.variable);
    if SUPERGLOBALS.contains(&var) {
        c.report_non_ignorable(
            p,
            "parameter.superglobal",
            format!(
                "Superglobal variable ${} cannot be used as a parameter.",
                text(var)
            ),
            None,
        );
    } else if var == b"this" {
        c.report_non_ignorable(
            p,
            "parameter.this",
            "Cannot use $this as parameter.".to_string(),
            None,
        );
    }
}

/// InvalidLexicalVariablesInClosureUseRule.
pub(super) fn closure_use(c: &mut Checker, closure: &Closure) {
    let Some(clause) = &closure.use_clause else {
        return;
    };
    let params: Vec<&[u8]> = closure
        .parameter_list
        .parameters
        .iter()
        .map(|p| name(&p.variable))
        .collect();
    for u in &clause.variables {
        let var = name(&u.variable);
        let (identifier, message) = if var == b"this" {
            (
                "closure.useThis",
                "Cannot use $this as lexical variable.".to_string(),
            )
        } else if SUPERGLOBALS.contains(&var) {
            (
                "closure.useSuperGlobal",
                format!(
                    "Cannot use superglobal variable ${} as lexical variable.",
                    text(var)
                ),
            )
        } else if params.contains(&var) {
            (
                "closure.useDuplicate",
                format!(
                    "Cannot use lexical variable ${} since a parameter with the same name already exists.",
                    text(var)
                ),
            )
        } else {
            continue;
        };
        c.report_non_ignorable(u, identifier, message, None);
    }
}

/// InnerFunctionRule: a named function declared inside another function-like's body.
pub(super) fn inner_function(c: &mut Checker, f: &Function) {
    if !c.in_named_function() {
        return;
    }
    c.report(
        f,
        "function.inner",
        "Inner named functions are not supported by PHPStan. Consider refactoring to an anonymous function, class method, or a top-level-defined function. See issue #165 (https://github.com/phpstan/phpstan/issues/165) for more details.".to_string(),
        None,
    );
}
