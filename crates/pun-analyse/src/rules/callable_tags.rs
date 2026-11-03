//! IncompatibleParamImmediatelyInvokedCallableRule (level 2): a function's or method's
//! `@param-immediately-invoked-callable` and `@param-later-invoked-callable` tags name a parameter whose native
//! type may be callable.

use mago_span::Span;
use mago_syntax::cst::FunctionLikeParameterList;
use pun_phpstan::phpdoc::TagValue;
use pun_phpstan::types::{Level, mixed};

use super::doc::doc_before;
use super::{Checker, declared};

pub const IDENTIFIERS: &[&str] = &[
    "parameter.notFound",
    "paramImmediatelyInvokedCallable.nonCallable",
    "paramLaterInvokedCallable.nonCallable",
];

/// The function or method declared at `at` with `params`.
pub fn function_like(c: &mut Checker, at: Span, params: &FunctionLikeParameterList) {
    if c.level < 2 {
        return;
    }
    let Some(doc) = doc_before(&c.file.contents, at.start.offset as usize) else {
        return;
    };
    let tags = super::doc::phpdoc_tags(doc);
    // `resolveParamImmediatelyInvokedCallable()`: by name, a later tag keeping an earlier one's place
    let mut invoked: Vec<(String, bool)> = Vec::new();
    for (names, immediately) in [
        (["@param-immediately-invoked-callable", "@phpstan-param-immediately-invoked-callable"], true),
        (["@param-later-invoked-callable", "@phpstan-param-later-invoked-callable"], false),
    ] {
        for name in names {
            for tag in tags.iter().filter(|t| t.name == name) {
                let TagValue::ParamName(parameter) = &tag.value else {
                    continue;
                };
                let parameter = parameter[1..].to_string();
                match invoked.iter_mut().find(|(n, _)| *n == parameter) {
                    Some(entry) => entry.1 = immediately,
                    None => invoked.push((parameter, immediately)),
                }
            }
        }
    }
    let class = c.class_scope();
    for (name, immediately) in invoked {
        let tag = if immediately {
            "@param-immediately-invoked-callable"
        } else {
            "@param-later-invoked-callable"
        };
        let Some(p) = params
            .parameters
            .iter()
            .find(|p| p.variable.name[1..] == *name.as_bytes())
        else {
            c.report(
                at,
                "parameter.notFound",
                format!("PHPDoc tag {tag} references unknown parameter: ${name}"),
                None,
            );
            continue;
        };
        // `getFunctionType($type, isParameterValueNullable(), false)`
        let mut ty = p
            .hint
            .as_ref()
            .map_or_else(mixed, |h| declared::native(c, h, class));
        if declared::default_is_null(p) {
            ty = ty.add_null();
        }
        if ty.is_callable().no() {
            let identifier = if immediately {
                "paramImmediatelyInvokedCallable.nonCallable"
            } else {
                "paramLaterInvokedCallable.nonCallable"
            };
            c.report(
                at,
                identifier,
                format!(
                    "PHPDoc tag {tag} is for parameter ${name} with non-callable type {}.",
                    ty.describe(Level::TypeOnly)
                ),
                None,
            );
        }
    }
}
