//! Level 2's ValueAssignedToClassConstantRule (`classConstant.value`, `classConstant.phpDocType`): a class constant's
//! value its native type or `@var` tag does not take. (The constants of `dynamicConstantNames` are bleeding edge.)

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{Level, decide_type, is_super_type_of, recommended_level};

use super::super::{Checker, declared};

pub const IDENTIFIERS: &[&str] = &["classConstant.value", "classConstant.phpDocType"];

/// The values, whose types the rule reads.
pub(super) fn typed(k: &ClassLikeConstant, spans: &mut Vec<Span>) {
    spans.extend(k.items.iter().map(|i| i.value.span()));
}

pub(in super::super) fn class_constant(c: &mut Checker, k: &ClassLikeConstant) {
    if c.level < 2 {
        return;
    }
    let Some(context) = c.class_scope() else { return };
    let display = c.display(context);
    for item in k.items.iter() {
        let Some(value) = c.phpstan_type(item.value) else { continue };
        let name = String::from_utf8_lossy(item.name.value);
        let found = declared::with_classes(c, || {
            let native = k.hint.as_ref().map(|h| declared::native(c, h, Some(context)));
            let phpdoc = declared::constant(c, k.span(), item.name.value, context);
            let level = |t| recommended_level(t, Some(&value));
            match (phpdoc, native) {
                (None, None) => None,
                (None, Some(native)) => (!pun_phpstan::types::accepts(&native, &value, true).yes()).then(|| {
                    let message = format!(
                        "Constant {display}::{name} ({}) does not accept value {}.",
                        native.describe(Level::TypeOnly),
                        value.describe(Level::Value)
                    );
                    ("classConstant.value", message, true)
                }),
                (Some(phpdoc), None) => {
                    let fits = is_super_type_of(&phpdoc, &value);
                    let relation = if fits.no() {
                        "is incompatible with"
                    } else if fits.maybe() {
                        "is not subtype of"
                    } else { return None };
                    let message = format!(
                        "PHPDoc tag @var for constant {display}::{name} with type {} {relation} value {}.",
                        phpdoc.describe(level(&phpdoc)),
                        value.describe(Level::Value)
                    );
                    Some(("classConstant.phpDocType", message, false))
                }
                (Some(phpdoc), Some(native)) => {
                    let ty = decide_type(native, Some(phpdoc));
                    (!pun_phpstan::types::accepts(&ty, &value, true).yes()).then(|| {
                        let message = format!(
                            "Constant {display}::{name} ({}) does not accept value {}.",
                            ty.describe(Level::TypeOnly),
                            value.describe(level(&ty))
                        );
                        ("classConstant.value", message, false)
                    })
                }
            }
        });
        match found {
            Some((identifier, message, true)) => c.report_non_ignorable(k, identifier, message, None),
            Some((identifier, message, false)) => c.report(k, identifier, message, None),
            None => {}
        }
    }
}
