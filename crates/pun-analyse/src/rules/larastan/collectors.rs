//! UnusedViewsRule and NoMissingTranslationsRule: their collectors run on each analysed file (the values travel
//! with the file's errors as [`COLLECTED_VIEW`] and [`COLLECTED_TRANSLATION`] entries), then [`finish`] turns the
//! whole project's values into the rules' errors.

use std::collections::HashSet;

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::ignore::Error;

use super::super::Checker;
use super::calls::written_name;
use super::{Larastan, is_a, is_object_of};
use crate::larastan::views;

const COLLECTED_VIEW: &str = "larastan.collected.view";
const COLLECTED_TRANSLATION: &str = "larastan.collected.translation";

/// `$template->value` of a `Scalar\String_` argument.
fn literal(e: Option<&Argument>) -> Option<String> {
    match e?.value() {
        Expression::Literal(Literal::String(s)) => {
            Some(String::from_utf8_lossy(s.value?).into_owned())
        }
        _ => None,
    }
}

/// `ViewName::normalize()`.
fn normalize(name: &str) -> String {
    match name.split_once("::") {
        None => name.replace('/', "."),
        Some((namespace, name)) => {
            let name = name.split("::").next().unwrap_or_default();
            format!("{namespace}::{}", name.replace('/', "."))
        }
    }
}

impl Checker<'_, '_> {
    fn collect(&mut self, identifier: &str, value: String, line: Option<i64>) {
        let file = self.file_name();
        self.errors.push(Error {
            identifier: Some(identifier.to_string()),
            ..Error::new(value, file, line)
        });
    }

    fn collect_view(&mut self, name: String) {
        self.collect(COLLECTED_VIEW, normalize(&name), None);
    }

    fn collect_translation(&mut self, at: &impl HasSpan, key: String) {
        let line = i64::from(self.file.line_number(at.span().start.offset)) + 1;
        self.collect(COLLECTED_TRANSLATION, key, Some(line));
    }
}

fn collecting(c: &Checker) -> Option<(bool, bool)> {
    c.larastan
        .map(|l| {
            (
                l.switches
                    .contains(crate::LarastanSwitches::CHECK_UNUSED_VIEWS),
                l.switches
                    .contains(crate::LarastanSwitches::CHECK_MISSING_TRANSLATIONS),
            )
        })
        .filter(|(v, t)| *v || *t)
}

/// UsedViewFunctionCollector and UsedTranslationFunctionCollector.
pub(crate) fn function_call<'arena>(
    c: &mut Checker<'_, 'arena>,
    call: &FunctionCall<'arena>,
    id: &Identifier,
) {
    let Some((views, translations)) = collecting(c) else {
        return;
    };
    let name = written_name(c.names, id);
    let Some(value) = literal(call.argument_list.arguments.first()) else {
        return;
    };
    if views && name == "view" {
        c.collect_view(value);
    } else if translations && matches!(name.as_str(), "__" | "trans" | "trans_choice") {
        c.collect_translation(call, value);
    }
}

/// UsedViewFacadeMakeCollector, UsedRouteFacadeViewCollector and UsedTranslationFacadeCollector.
pub(crate) fn static_method_call<'arena>(
    c: &mut Checker<'_, 'arena>,
    call: &StaticMethodCall<'arena>,
) {
    let Some((views, translations)) = collecting(c) else {
        return;
    };
    let (ClassLikeMemberSelector::Identifier(m), Expression::Identifier(class)) =
        (&call.method, call.class)
    else {
        return;
    };
    let class = c.names.get(&class.span());
    let args = call.argument_list.arguments.as_slice();
    match m.value {
        b"make" if views && is_a(c, class, b"illuminate\\support\\facades\\view") => {
            if let Some(v) = literal(args.first()) {
                c.collect_view(v);
            }
        }
        b"view"
            if views
                && args.len() >= 2
                && is_a(c, class, b"illuminate\\support\\facades\\route") =>
        {
            if let Some(v) = literal(args.get(1)) {
                c.collect_view(v);
            }
        }
        b"get" | b"choice"
            if translations && is_a(c, class, b"illuminate\\support\\facades\\lang") =>
        {
            if let Some(v) = literal(args.first()) {
                c.collect_translation(call, v);
            }
        }
        _ => {}
    }
}

/// The method calls the collectors read the object's type of, by their method name.
pub(super) fn typed_method_call<'a>(larastan: &Larastan, call: &'a MethodCall) -> Option<&'a [u8]> {
    let ClassLikeMemberSelector::Identifier(m) = &call.method else {
        return None;
    };
    let read = !call.argument_list.arguments.is_empty()
        && match m.value {
            b"make" | b"markdown" | b"view" => larastan
                .switches
                .contains(crate::LarastanSwitches::CHECK_UNUSED_VIEWS),
            b"get" | b"choice" => larastan
                .switches
                .contains(crate::LarastanSwitches::CHECK_MISSING_TRANSLATIONS),
            _ => false,
        };
    read.then_some(m.value)
}

/// UsedViewMakeCollector, UsedEmailViewCollector and UsedTranslationTranslatorCollector.
pub(crate) fn method_call<'arena>(c: &mut Checker<'_, 'arena>, call: &MethodCall<'arena>) {
    let Some(method) = c.larastan.and_then(|l| typed_method_call(l, call)) else {
        return;
    };
    let ty = c.type_of(call.object).cloned();
    let (Some(ty), Some(value)) = (ty, literal(call.argument_list.arguments.first())) else {
        return;
    };
    match method {
        b"make" if is_object_of(c, &ty, b"illuminate\\contracts\\view\\factory") => {
            c.collect_view(value)
        }
        b"markdown" | b"view"
            if is_object_of(c, &ty, b"illuminate\\mail\\mailable")
                || is_object_of(c, &ty, b"illuminate\\notifications\\messages\\mailmessage") =>
        {
            c.collect_view(value)
        }
        b"get" | b"choice" if is_object_of(c, &ty, b"illuminate\\translation\\translator") => {
            c.collect_translation(call, value)
        }
        _ => {}
    }
}

/// The rules on the project's collected values: the errors without the collected entries, and the rules' errors.
pub(crate) fn finish(larastan: Option<&Larastan>, errors: Vec<Error>) -> Vec<Error> {
    let (collected, mut errors): (Vec<Error>, Vec<Error>) = errors.into_iter().partition(|e| {
        matches!(
            e.identifier.as_deref(),
            Some(COLLECTED_VIEW | COLLECTED_TRANSLATION)
        )
    });
    let Some(larastan) = larastan else {
        return errors;
    };
    let finder = views::Finder::new(&larastan.root);
    if larastan
        .switches
        .contains(crate::LarastanSwitches::CHECK_UNUSED_VIEWS)
    {
        let directories = if larastan.view_directories.is_empty() {
            finder.paths.clone()
        } else {
            larastan.view_directories.clone()
        };
        let mut used: HashSet<String> = collected
            .iter()
            .filter(|e| e.identifier.as_deref() == Some(COLLECTED_VIEW))
            .map(|e| e.message.clone())
            .collect();
        used.extend(views::views_used_in_views(&directories));
        let mut seen = HashSet::new();
        for name in views::view_names(&directories) {
            let Some(path) = finder.find(&name) else {
                continue;
            };
            if !used.contains(&name) && seen.insert(name) {
                errors.push(Error {
                    identifier: Some("larastan.unusedViews".to_string()),
                    ..Error::new(
                        "This view is not used in the project.",
                        path.to_string_lossy(),
                        Some(0),
                    )
                });
            }
        }
    }
    if larastan
        .switches
        .contains(crate::LarastanSwitches::CHECK_MISSING_TRANSLATIONS)
    {
        let directories = if larastan.translation_directories.is_empty() {
            // lang_path(): resources/lang when the app has one, else lang
            let resources = larastan.root.join("resources/lang");
            vec![if resources.is_dir() {
                resources
            } else {
                larastan.root.join("lang")
            }]
        } else {
            larastan.translation_directories.clone()
        };
        let available: HashSet<String> = views::available_translations(&directories)
            .into_iter()
            .collect();
        let mut used: Vec<(String, String, Option<i64>)> = collected
            .into_iter()
            .filter(|e| e.identifier.as_deref() == Some(COLLECTED_TRANSLATION))
            .map(|e| (e.file, e.message, e.line))
            .collect();
        for (file, found) in views::translations_in_views(&finder) {
            let file = file.to_string_lossy().into_owned();
            used.extend(
                found
                    .into_iter()
                    .map(|(t, line)| (file.clone(), t, Some(i64::from(line)))),
            );
        }
        for (file, translation, line) in used {
            if available.contains(&translation) || translation.contains("::") {
                continue;
            }
            errors.push(Error {
                identifier: Some("larastan.missingTranslations".to_string()),
                ..Error::new(
                    format!("Translation \"{translation}\" has not been found."),
                    file,
                    line,
                )
            });
        }
    }
    errors
}
