//! IncompatibleDefaultParameterTypeRule (functions, methods), IncompatibleClosureDefaultParameterTypeRule and
//! IncompatibleArrowFunctionDefaultParameterTypeRule (`parameter.defaultValue`): a default value the parameter's type
//! does not accept.

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::types::{Type, accepts, recommended_level};

use super::super::{Checker, declared};

pub(super) const IDENTIFIERS: &[&str] = &["parameter.defaultValue"];

/// The default values, whose types the rule reads.
pub(super) fn typed(params: &FunctionLikeParameterList, spans: &mut Vec<mago_span::Span>) {
    spans.extend(
        params
            .parameters
            .iter()
            .filter_map(|p| p.default_value.as_ref())
            .map(|d| d.value.span()),
    );
}

fn check(c: &mut Checker, params: &FunctionLikeParameterList, types: &[Type], of: &str) {
    for (i, (p, ty)) in params.parameters.iter().zip(types).enumerate() {
        let Some(d) = &p.default_value else {
            continue;
        };
        let Some(default) = c.phpstan_type(d.value) else {
            continue;
        };
        let parameter = ty.resolve_to_bounds();
        let message = declared::with_classes(c, || {
            if accepts(&parameter, &default, true).yes() {
                return None;
            }
            let level = recommended_level(&parameter, Some(&default));
            Some(format!(
                "Default value of the parameter #{} {} ({}) of {of} is incompatible with type {}.",
                i + 1,
                String::from_utf8_lossy(p.variable.name),
                default.describe(level),
                parameter.describe(level)
            ))
        });
        if let Some(message) = message {
            c.report(p, "parameter.defaultValue", message, None);
        }
    }
}

pub(in super::super) fn function(c: &mut Checker, f: &Function) {
    if c.level < 2 {
        return;
    }
    let name = c.resolved_span(f.name.span());
    let types: Vec<Type> = declared::with_classes(c, || {
        declared::function(c, f)
            .params
            .into_iter()
            .map(|p| p.1)
            .collect()
    });
    check(c, &f.parameter_list, &types, &format!("function {name}()"));
}

pub(in super::super) fn method(c: &mut Checker, m: &Method) {
    if c.level < 2 {
        return;
    }
    let Some(context) = c.class_scope() else {
        return;
    };
    let Some(declaring) = super::super::missing_types::declaring(c, m.span()) else {
        return;
    };
    let name = super::missing_return::method_name(c, m);
    let types: Vec<Type> = declared::with_classes(c, || {
        declared::method(c, m, declaring, context)
            .params
            .into_iter()
            .map(|p| p.1)
            .collect()
    });
    let of = format!("method {}::{name}()", c.display(context));
    check(c, &m.parameter_list, &types, &of);
}

/// A closure's or arrow function's parameters, by their native types.
// ponytail: a closure's `@param` tags are not read; add them with a closure signature in declared.rs
pub(in super::super) fn anonymous(c: &mut Checker, params: &FunctionLikeParameterList) {
    if c.level < 2 {
        return;
    }
    let class = c.class_scope();
    let types: Vec<Type> = declared::with_classes(c, || {
        params
            .parameters
            .iter()
            .map(|p| {
                p.hint.as_ref().map_or_else(pun_phpstan::types::mixed, |h| {
                    let t = declared::native(c, h, class);
                    if declared::default_is_null(p) {
                        t.add_null()
                    } else {
                        t
                    }
                })
            })
            .collect()
    });
    check(c, params, &types, "anonymous function");
}
