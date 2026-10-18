//! ClassNameCheck, the check every rule that reads a written class name runs: ClassCaseSensitivityCheck
//! (`<kind>.nameCase`) and ClassForbiddenNameCheck (`class.prefixed`).

use std::collections::HashMap;

use mago_span::Span;
use mago_syntax::cst::*;

use super::Checker;
use super::reflection::{Known, known};

/// ClassNameNodePair: a class name as the rule reads it, and the node whose line its errors take.
pub struct Pair {
    name: String,
    at: Span,
    /// The node is the name as written (a php-parser `Name`, which UseAliasVisitor marks), not e.g. the parameter.
    written: bool,
}

impl Pair {
    /// A pair whose node is the written name at `at`.
    pub fn name(name: String, at: Span) -> Pair {
        Pair {
            name,
            at,
            written: true,
        }
    }

    /// A pair whose node is another node (an attribute, a parameter, a property).
    pub fn node(name: String, at: Span) -> Pair {
        Pair {
            name,
            at,
            written: false,
        }
    }
}

/// UseAliasVisitor: the explicit `use X as Alias;` aliases of the current namespace, keyed by lowercase alias.
#[derive(Default)]
pub struct State {
    explicit_aliases: HashMap<String, String>,
}

pub fn enter_namespace(c: &mut Checker) {
    c.class_names.explicit_aliases.clear();
}

pub fn use_statement(c: &mut Checker, u: &Use) {
    // `use function` and `use const` aliases are not class names
    let items: Vec<&UseItem> = match &u.items {
        UseItems::Sequence(s) => s.items.iter().collect(),
        UseItems::MixedList(l) => l
            .items
            .iter()
            .filter(|i| i.r#type.is_none())
            .map(|i| &i.item)
            .collect(),
        UseItems::TypedSequence(_) | UseItems::TypedList(_) => Vec::new(),
    };
    for alias in items.iter().filter_map(|i| i.alias.as_ref()) {
        let alias = String::from_utf8_lossy(alias.identifier.value).into_owned();
        c.class_names
            .explicit_aliases
            .insert(alias.to_ascii_lowercase(), alias);
    }
}

/// ClassNameCheck::checkClassNames(). `case` is its `$checkClassCaseSensitivity`: `true` for the rules that call it
/// with the default, [`Checker::case_sensitivity`] for the ones passing the `checkClassCaseSensitivity` parameter.
pub fn check(c: &mut Checker, pairs: &[Pair], case: bool) {
    if case {
        case_sensitivity(c, pairs);
    }
    forbidden(c, pairs);
    // ponytail: the RestrictedClassNameUsageExtension loop (RestrictedInternalClassNameUsageExtension's `*.internal`
    // errors) belongs to the internal-usage rules
}

/// The core classes PHPStan reflects from its own stubs (stubs/runtime/Attribute84.php) rather than as built-ins, so
/// their case is checked like a user class's.
const STUBBED: [&str; 4] = [
    "allowdynamicproperties",
    "attribute",
    "returntypewillchange",
    "sensitiveparameter",
];

/// ClassCaseSensitivityCheck::checkClassNames(), with `checkInternalClassCaseSensitivity` off (built-in classes are
/// skipped).
fn case_sensitivity(c: &mut Checker, pairs: &[Pair]) {
    for pair in pairs {
        let Some(k) = known(c, &pair.name) else {
            continue;
        };
        if let Known::Builtin(b) = k
            && !STUBBED.contains(&b.name.to_ascii_lowercase().as_str())
        {
            continue;
        }
        let real = k.display();
        // a class_alias() of another name, an explicit `use` alias written as declared, or the right case
        if !real.eq_ignore_ascii_case(&pair.name)
            || (pair.written && explicit_alias(c, pair.at))
            || real == pair.name
        {
            continue;
        }
        let kind = k.type_description();
        c.report(
            pair.at,
            &format!("{}.nameCase", kind.to_ascii_lowercase()),
            format!(
                "{kind} {real} referenced with incorrect case: {}.",
                pair.name
            ),
            None,
        );
    }
}

/// UseAliasVisitor's `isExplicitUseAlias`: the name at `at` is one part written exactly as an explicit alias.
fn explicit_alias(c: &Checker, at: Span) -> bool {
    let written = c.written(at);
    c.class_names
        .explicit_aliases
        .get(&written.to_ascii_lowercase())
        .is_some_and(|alias| *alias == written)
}

/// ClassForbiddenNameCheck::INTERNAL_CLASS_PREFIXES (no ForbiddenClassNameExtension adds any).
const INTERNAL_CLASS_PREFIXES: [(&str, &str); 5] = [
    ("PHPStan", "_PHPStan_"),
    ("Rector", "RectorPrefix"),
    ("PHP-Scoper", "_PhpScoper"),
    ("PHPUnit", "PHPUnitPHAR"),
    ("Box", "_HumbugBox"),
];

/// ClassForbiddenNameCheck::checkClassNames().
fn forbidden(c: &mut Checker, pairs: &[Pair]) {
    for pair in pairs {
        let name = pair.name.as_str();
        let mut found = None;
        for (project, prefix) in INTERNAL_CLASS_PREFIXES {
            let Some(rest) = name.strip_prefix(prefix) else {
                continue;
            };
            found = Some((project, rest.find('\\').map_or(rest, |pos| &rest[pos..])));
        }
        let Some((project, without_prefix)) = found else {
            continue;
        };
        c.report_non_ignorable(
            pair.at,
            "class.prefixed",
            format!("Referencing prefixed {project} class: {name}."),
            Some(&format!(
                "This is most likely unintentional. Did you mean to type {without_prefix}?"
            )),
        );
    }
}

/// FunctionDefinitionCheck::getOriginalClassNamePairsFromTypeNode() for one name: the resolved name's namespace
/// parts with the parts as written (`Foo\bar` for `bar` imported by `use Foo\Bar;`).
pub fn original_case(c: &Checker, at: Span) -> String {
    let resolved = c.resolved_span(at);
    let written = c.written(at);
    let written = written.trim_start_matches('\\');
    // `namespace\Foo`'s original name is `Foo`
    let written = match written.split_once('\\') {
        Some((first, rest)) if first.eq_ignore_ascii_case("namespace") => rest,
        _ => written,
    };
    let original: Vec<&str> = written.split('\\').collect();
    let parts: Vec<&str> = resolved.split('\\').collect();
    parts[..parts.len().saturating_sub(original.len())]
        .iter()
        .chain(&original)
        .copied()
        .collect::<Vec<_>>()
        .join("\\")
}
