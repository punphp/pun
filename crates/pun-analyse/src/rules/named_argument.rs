//! MethodCallWithPossiblyRenamedNamedArgumentRule. The collectors are emitted by
//! FunctionCallParametersCheck and MethodParameterComparisonHelper, not the placeholder Collector classes.
//! Records travel with the existing per-file results; only analysed files (including walked trait contexts)
//! contribute. The join precedes local ignores: a collected call is not itself an error.

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::comments::{LinesToIgnore, lines_to_ignore};
use pun_phpstan::ignore::{Error, ignores_without_comments, local_ignores};
use pun_phpstan::types::{NeverKind, Type};

use super::{Checker, declared, members, params};

pub(super) const ID: &str = "argument.parameterRenamedInSubtype";
const ENTRY: &str = "\u{1e}named-argument\u{1f}";

pub(super) fn rename(class: &str, method: &str, subtype: &str, old: &str, new: &str) -> String {
    format!("{ENTRY}rename\u{1f}{class}\u{1f}{method}\u{1f}{subtype}\u{1f}{old}\u{1f}{new}")
}

/// PhpMethodReflection::acceptsNamedArguments(): the declaring class's own tag and the merged method PHPDoc.
pub(super) fn accepts(c: &Checker, class: &ClassLikeMetadata, f: &FunctionLikeMetadata) -> bool {
    !super::declarations::has_tag(&c.class_tags(class), &["@no-named-arguments"])
        && method_doc(c, class, f, &mut HashSet::new()).unwrap_or(true)
}

/// PhpDocInheritanceResolver: the first parent method with PHPDoc (parent, interface, abstract trait), merged
/// with the method's own PHPDoc. A parent's class-level tag is not part of the method PHPDoc.
fn method_doc(
    c: &Checker,
    class: &ClassLikeMetadata,
    f: &FunctionLikeMetadata,
    seen: &mut HashSet<Span>,
) -> Option<bool> {
    if !seen.insert(f.span) {
        return None;
    }
    let own = declared::doc_at(c, f.span).map(|(doc, _)| {
        !super::declarations::has_tag(
            &super::declarations::doc_tags(&doc),
            &["@no-named-arguments"],
        )
    });
    let name = f.name.as_bytes();
    let parents = class
        .direct_parent_class
        .iter()
        .chain(&class.direct_parent_interfaces);
    for parent in parents.filter_map(|p| c.codebase.get_class_like(p.as_bytes())) {
        if let Some(members::Method::User { declaring, f, .. }) = members::method(c, parent, name)
            && !f
                .method_metadata
                .as_ref()
                .is_some_and(|m| m.visibility.is_private())
            && let Some(parent) = method_doc(c, declaring, f, seen)
        {
            return Some(own.unwrap_or(true) && parent);
        }
    }
    for trait_ in class
        .used_traits
        .iter()
        .filter_map(|t| c.codebase.get_class_like(t.as_bytes()))
    {
        if let Some(f) = c.codebase.get_method(trait_.name.as_bytes(), name)
            && f.method_metadata.as_ref().is_some_and(|m| m.is_abstract)
            && let Some(parent) = declared::doc_at(c, f.span)
        {
            return Some(
                own.unwrap_or(true)
                    && !super::declarations::has_tag(
                        &super::declarations::doc_tags(&parent.0),
                        &["@no-named-arguments"],
                    ),
            );
        }
    }
    own
}

struct Method {
    class: String,
    name: String,
    acceptor: params::Acceptor,
    accepts: bool,
    private: bool,
    final_class: bool,
}

/// UnionTypeMethodReflection / IntersectionTypeMethodReflection, retaining their distinct declaring-class,
/// parameter combination, private and acceptsNamedArguments contracts. No receiver types are inferred here.
fn method(c: &Checker, ty: &Type, name: &[u8], counts: &params::Counts) -> Option<Method> {
    match ty {
        Type::Union(ts) | Type::Benevolent(ts) => {
            let methods: Vec<_> = ts
                .iter()
                .map(|t| method(c, t, name, counts))
                .collect::<Option<_>>()?;
            let first = methods.first()?;
            Some(Method {
                class: first.class.clone(),
                name: String::from_utf8_lossy(name).into_owned(),
                acceptor: params::combine(&methods.iter().map(|m| &m.acceptor).collect::<Vec<_>>()),
                accepts: methods.iter().all(|m| m.accepts),
                private: methods.iter().any(|m| m.private),
                final_class: first.final_class,
            })
        }
        Type::Intersection(ts) => {
            let methods: Vec<_> = ts
                .iter()
                .filter_map(|t| method(c, t, name, counts))
                .collect();
            let mut selected = methods.first()?;
            for candidate in &methods[1..] {
                if (candidate.acceptor.variadic, candidate.acceptor.names.len())
                    > (selected.acceptor.variadic, selected.acceptor.names.len())
                {
                    selected = candidate;
                }
            }
            Some(Method {
                class: selected.class.clone(),
                name: String::from_utf8_lossy(name).into_owned(),
                acceptor: selected.acceptor.clone(),
                accepts: methods.iter().any(|m| m.accepts),
                private: methods.iter().all(|m| m.private),
                final_class: selected.final_class,
            })
        }
        Type::Template(t) => method(c, &t.bound, name, counts),
        Type::Object(n, _) | Type::Static(n, _) | Type::This(n) | Type::EnumCase(n, _) => {
            let m = c.method_of(n, name).ok().flatten()?;
            let (accepts, final_class) = match &m {
                members::Method::User { declaring, f, .. } => {
                    (accepts(c, declaring, f), declaring.flags.is_final())
                }
                members::Method::Builtin { declaring, .. } => (true, declaring.is_final),
            };
            Some(Method {
                class: m.declaring_name(),
                name: m.name(),
                acceptor: params::select(c, m.variants(), counts),
                accepts,
                private: m.visibility().is_private(),
                final_class,
            })
        }
        _ => None,
    }
}

fn strings(ty: &Type) -> Vec<String> {
    match ty {
        Type::ConstString(s) => vec![s.clone()],
        Type::Union(ts) | Type::Benevolent(ts) => {
            let parts: Vec<_> = ts.iter().map(strings).collect();
            if parts.iter().any(Vec::is_empty) {
                Vec::new()
            } else {
                parts.concat()
            }
        }
        Type::Intersection(ts) => ts.iter().flat_map(strings).collect(),
        Type::Template(t) => strings(&t.bound),
        _ => Vec::new(),
    }
}

/// The part of a ConstantArrayType that FunctionCallParametersCheck reads when binding names.
struct ArrayArguments {
    keys: Vec<Type>,
    optional: Vec<usize>,
    unsealed: bool,
}

/// Type::getConstantArrays(): a union contributes only when every alternative is a constant array.
/// TemplateConstantArrayType copies the bound's keys and optionality, but its constructor omits unsealed extras.
fn arrays(ty: &Type) -> Vec<ArrayArguments> {
    match ty {
        Type::ConstArray(a) => vec![ArrayArguments {
            keys: a.keys.clone(),
            optional: a.optional.clone(),
            unsealed: a
                .unsealed
                .as_ref()
                .is_some_and(|(key, _)| *key != Type::Never(NeverKind::Explicit)),
        }],
        Type::Union(ts) | Type::Benevolent(ts) => {
            let parts: Vec<_> = ts.iter().map(arrays).collect();
            if parts.iter().any(Vec::is_empty) {
                Vec::new()
            } else {
                parts.into_iter().flatten().collect()
            }
        }
        Type::Intersection(_) if ty.is_callable().yes() && ty.is_array().yes() => {
            vec![ArrayArguments {
                // IntersectionType::getConstantArrays builds the two required positional offsets of a callable array.
                keys: vec![Type::ConstInt(0), Type::ConstInt(1)],
                optional: Vec::new(),
                unsealed: false,
            }]
        }
        Type::Intersection(ts) => ts.iter().flat_map(arrays).collect(),
        Type::Template(t) => {
            let mut arrays = arrays(&t.bound);
            if matches!(t.bound, Type::ConstArray(_)) {
                arrays[0].unsealed = false;
            }
            arrays
        }
        _ => Vec::new(),
    }
}

/// FunctionCallParametersCheck's unpack expansion: the shortest alternative, common string keys, and optional
/// positional keys omitted. Named optional keys still emit. Every expanded argument uses the unpack's line.
fn unpacked_names(ty: &Type, first: bool) -> Option<Vec<Option<String>>> {
    let arrays = arrays(ty);
    let max = arrays
        .iter()
        .map(|a| if a.unsealed { 0 } else { a.keys.len() })
        .min()?;
    let mut names: Vec<_> = (0..max)
        .filter_map(|i| {
            let key = &arrays[0].keys[i];
            let name = match key {
                Type::ConstString(s) if arrays.iter().all(|a| a.keys[i] == *key) => Some(s.clone()),
                _ => None,
            };
            if name.is_none() && arrays.last().unwrap().optional.contains(&i) {
                None
            } else {
                Some(name)
            }
        })
        .collect();
    if first && names.is_empty() && ty.is_iterable_at_least_once().yes() {
        names.push(None);
    }
    Some(names)
}

/// CallMethodsRule's instance calls only. Static calls, constructors and first-class callables do not emit.
pub(super) fn call(
    c: &mut Checker,
    object: &Expression,
    selector: &ClassLikeMemberSelector,
    args: &ArgumentList,
    nullsafe: bool,
) {
    // FunctionCallParametersCheck returns before emitting when both argument checks are disabled (levels 0–4).
    if c.level < 5 {
        return;
    }
    let names = match selector {
        ClassLikeMemberSelector::Identifier(n) => {
            vec![String::from_utf8_lossy(n.value).into_owned()]
        }
        _ => super::types::dynamic_names::member(selector)
            .and_then(|span| c.types.get(&span))
            .map_or_else(Vec::new, |t| strings(&c.to_phpstan(t))),
    };
    let counts = params::count(args);
    let mut object = object;
    while let Expression::Parenthesized(p) = object {
        object = p.expression;
    }
    let exact = c.conditions.exact.contains(&object.span())
        || matches!(object, Expression::Instantiation(_));
    for name in names {
        let ty = c.phpstan_type(object).map(|t| {
            if nullsafe || super::assign::contains_nullsafe(object) {
                t.remove_null()
            } else {
                t
            }
        });
        let ty = c.find_type_to_check_of(ty, &|t| method(c, t, name.as_bytes(), &counts).is_some());
        let Some(m) = ty
            .as_ref()
            .and_then(|t| method(c, t, name.as_bytes(), &counts))
        else {
            continue;
        };
        // NewHandler's asFinal() applies to the called-on class, not the class of an inherited declaration.
        let final_called_on =
            exact && ty.as_ref().unwrap().object_class_names() == vec![m.class.clone()];
        if m.private || m.final_class || final_called_on || !m.accepts {
            continue;
        }
        // processArguments(): names bind the actual parameter (including a user variadic's unknown names).
        // Positional arguments beyond the signature stop processing, even if a named argument follows.
        let mut position = 0;
        let mut stopped = false;
        for (arg, written) in counts.args.iter().zip(args.arguments.iter()) {
            let value = match written {
                Argument::Named(n) => n.value,
                Argument::Positional(p) => p.value,
            };
            let names = match &arg.unpack {
                None => vec![arg.name.clone()],
                Some(_) => c
                    .phpstan_type(value)
                    .and_then(|t| unpacked_names(&t, position == 0))
                    .unwrap_or_else(|| vec![None]),
            };
            for name in names {
                let Some(name) = name else {
                    if position >= m.acceptor.names.len()
                        && !m.acceptor.variadics.last().copied().unwrap_or(false)
                    {
                        stopped = true;
                        break;
                    }
                    position += 1;
                    continue;
                };
                position += 1;
                let parameter = m
                    .acceptor
                    .names
                    .iter()
                    .position(|p| *p == name)
                    .or_else(|| {
                        (!m.acceptor.builtin && m.acceptor.variadics.last() == Some(&true))
                            .then(|| m.acceptor.names.len() - 1)
                    });
                if let Some(parameter) = parameter {
                    c.report(
                        arg.span,
                        ID,
                        format!(
                            "{ENTRY}call\u{1f}{}\u{1f}{}\u{1f}{}",
                            m.class, m.name, m.acceptor.names[parameter]
                        ),
                        None,
                    );
                }
            }
            if stopped {
                break;
            }
        }
    }
}

/// Collector payloads must bypass LocalIgnoresProcessor; declarations are not diagnostics and a call might never
/// join. Ordinary errors and their unmatched-ignore diagnostics retain the existing per-file processing.
pub(super) fn deferred_local_ignores(
    file: &str,
    errors: Vec<Error>,
    lines: &mut LinesToIgnore,
    unmatched: &mut LinesToIgnore,
) -> (Vec<Error>, Vec<Error>) {
    let (records, errors): (Vec<_>, Vec<_>) = errors
        .into_iter()
        .partition(|e| e.message.starts_with(ENTRY));
    let (mut kept, ignored) = local_ignores(file, errors, lines, unmatched);
    kept.extend(records);
    (kept, ignored)
}

/// CollectedDataNode's file-keyed iteration is lexical, independent of the project's path ordering or Rayon.
pub(crate) fn resolve(
    errors: Vec<Error>,
    sources: &crate::codebase::Sources,
    explain_ignores: bool,
) -> Vec<Error> {
    let (mut records, mut errors): (Vec<_>, Vec<_>) = errors
        .into_iter()
        .partition(|e| e.message.starts_with(ENTRY));
    records.sort_by(|a, b| a.file_path.cmp(&b.file_path));
    let mut calls: BTreeMap<(&str, &str, &str), Vec<&Error>> = BTreeMap::new();
    for e in &records {
        let fields: Vec<_> = e.message[ENTRY.len()..].split('\u{1f}').collect();
        if let ["call", class, method, parameter] = fields.as_slice() {
            calls.entry((class, method, parameter)).or_default().push(e);
        }
    }
    let mut joined: BTreeMap<String, Vec<Error>> = BTreeMap::new();
    for e in &records {
        let fields: Vec<_> = e.message[ENTRY.len()..].split('\u{1f}').collect();
        let ["rename", class, method, subtype, old, new] = fields.as_slice() else {
            continue;
        };
        let Some(calls) = calls.get(&(class, method, old)) else {
            continue;
        };
        for call in calls {
            let mut error = Error::new(
                format!(
                    "Call to {class}::{method}() uses named argument for parameter ${old}, but {subtype} renames it to ${new}."
                ),
                &call.file_path,
                call.line,
            );
            error.identifier = Some(ID.into());
            joined.entry(error.file.clone()).or_default().push(error);
        }
    }
    for (file, joined) in joined {
        let source = sources
            .get(Path::new(&file))
            .expect("collector calls belong to analysed files");
        let (mut lines, _) = lines_to_ignore(&source.contents);
        let original = lines.clone();
        let mut unmatched = lines.clone();
        if explain_ignores {
            ignores_without_comments(&file, &mut lines, &mut unmatched);
        }
        let (kept, ignored) = local_ignores(&file, joined, &mut lines, &mut unmatched);
        // The file driver could not know these errors would join. Consume its provisional unmatched entries,
        // once per identifier occurrence (or once for an ignore-all), only when a real joined error was ignored.
        for ignored in ignored {
            let all = original
                .get(&ignored.line.unwrap())
                .is_some_and(Option::is_none);
            if let Some(i) = errors.iter().position(|e| {
                e.file == file
                    && e.line == ignored.line
                    && if all {
                        e.identifier.as_deref() == Some("ignore.unmatchedLine")
                    } else {
                        e.identifier.as_deref() == Some("ignore.unmatchedIdentifier")
                            && e.message.contains(ID)
                    }
            }) {
                errors.remove(i);
            }
        }
        errors.extend(kept);
    }
    errors
}
