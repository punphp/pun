//! MethodCallWithPossiblyRenamedNamedArgumentRule. The collectors are emitted by
//! FunctionCallParametersCheck and MethodParameterComparisonHelper, not the placeholder Collector classes.
//! Records travel with the existing per-file results; only analysed files (including walked trait contexts)
//! contribute. The join precedes local ignores: a collected call is not itself an error.

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use mago_analyzer::observations::CallScopeRequest;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
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
    let own = declared::doc_at(c, f.span)
        .map(|(_, entry, _)| !super::declarations::has_tag(&entry.names, &["@no-named-arguments"]));
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
            && let Some((_, parent, _)) = declared::doc_at(c, f.span)
        {
            return Some(
                own.unwrap_or(true) && !super::declarations::has_tag(&parent.names, &["@no-named-arguments"]),
            );
        }
    }
    own
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
                Type::ConstString(s) if arrays.iter().all(|a| a.keys[i] == *key) => Some(String::from_utf8_lossy(s).into_owned()),
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

/// Named collectors and constant dynamic method checks share owned operand-position scopes.
pub(super) fn requests(program: &Program) -> Vec<CallScopeRequest> {
    let mut calls = Requests(Vec::new());
    calls.walk_program(program, &mut ());
    calls.0
}

struct Requests(Vec<CallScopeRequest>);
impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Requests {
    fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, _: &mut ()) {
        let (object, selector, args) = match expression {
            Expression::Call(Call::Method(call)) => (call.object, &call.method, &call.argument_list),
            Expression::Call(Call::NullSafeMethod(call)) => (call.object, &call.method, &call.argument_list),
            _ => return,
        };
        if matches!(selector, ClassLikeMemberSelector::Identifier(_))
            && !args.arguments.iter().any(|arg| matches!(arg, Argument::Named(_))
            || matches!(arg, Argument::Positional(arg) if arg.ellipsis.is_some()))
        {
            return;
        }
        let mut expressions = Expressions(Vec::new());
        expressions.walk_expression(object, &mut ());
        expressions.walk_class_like_member_selector(selector, &mut ());
        expressions.walk_argument_list(args, &mut ());
        expressions.0.sort_unstable_by_key(|span| (span.start.offset, span.end.offset));
        expressions.0.dedup();
        self.0.push(CallScopeRequest { call: expression.span(), expressions: expressions.0 });
    }
}

struct Expressions(Vec<Span>);
impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Expressions {
    fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, _: &mut ()) {
        self.0.push(expression.span());
    }
    fn walk_in_variable(&mut self, variable: &'ast Variable<'arena>, _: &mut ()) {
        self.0.push(variable.span());
    }
}

/// CallMethodsRule's instance calls only. Static calls, constructors and first-class callables do not emit.
pub(super) fn call(
    c: &mut Checker,
    span: Span,
    object: &Expression,
    args: &ArgumentList,
    nullsafe: bool,
) {
    // FunctionCallParametersCheck returns before emitting only when both independent argument checks are off.
    if (!c.check_argument_types && !c.check_arguments_passed_by_reference) || !c.checks_expression(object) {
        return;
    }
    let counts = params::count(args);
    let object = object.unparenthesized();
    let mut records = Vec::new();
    for scope in c.named_call_scopes.get(&span).into_iter().flatten() {
        // Missing observations are not mixed: later names cannot bind against an unknown unpack width.
        if counts.args.iter().any(|arg| arg.unpack.is_some() && !scope.types.contains_key(&arg.value)) {
            continue;
        }
        let name = scope.name;
        let short_circuit = scope.types.get(&object.span()).is_some_and(|t| t.has_nullsafe_null());
        // Both null removal and RuleLevelHelper recombine alternatives using declared generic variance.
        let ty = declared::with_classes(c, || {
            let ty = c.phpstan_type_in(object, &scope.types).map(|t| {
                // An earlier nullsafe operator does not erase null returned by the method itself.
                if nullsafe || (short_circuit && super::assign::contains_nullsafe(object)) {
                    t.remove_null()
                } else {
                    t
                }
            });
            c.find_type_to_check_of(ty, &|t| matches!(c.reflected_method(t, name.as_bytes()), Ok(Some(_))))
        });
        let Some(m) = ty
            .as_ref()
            .and_then(|t| c.reflected_method(t, name.as_bytes()).ok().flatten())
        else {
            continue;
        };
        if m.private || m.final_class || !m.accepts {
            continue;
        }
        let acceptor = m.select(c, &counts, Some(scope));
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
                    .phpstan_type_in(value, &scope.types)
                    .and_then(|t| unpacked_names(&t, position == 0))
                    .unwrap_or_else(|| vec![None]),
            };
            for name in names {
                let Some(name) = name else {
                    if position >= acceptor.names.len()
                        && !acceptor.variadics.last().copied().unwrap_or(false)
                    {
                        stopped = true;
                        break;
                    }
                    position += 1;
                    continue;
                };
                position += 1;
                let parameter = acceptor
                    .names
                    .iter()
                    .position(|p| *p == name)
                    .or_else(|| {
                        (!m.builtin() && acceptor.variadics.last() == Some(&true))
                            .then(|| acceptor.names.len() - 1)
                    });
                if let Some(parameter) = parameter {
                    records.push((
                        arg.span,
                        format!(
                            "{ENTRY}call\u{1f}{}\u{1f}{}\u{1f}{}",
                            m.declaring.declaring_name(), m.name, acceptor.names[parameter]
                        ),
                    ));
                }
            }
            if stopped {
                break;
            }
        }
    }
    for (span, record) in records {
        c.report(span, ID, record, None);
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
    enabled: bool,
) -> Vec<Error> {
    let (mut records, mut errors): (Vec<_>, Vec<_>) = errors
        .into_iter()
        .partition(|e| e.message.starts_with(ENTRY));
    // An unregistered CollectedDataNode rule never joins or consumes provisional unmatched ignores.
    if !enabled {
        return errors;
    }
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
                            && e.message
                                == format!(
                                    "No error with identifier {ID} is reported on line {}.",
                                    ignored.line.unwrap()
                                )
                    }
            }) {
                errors.remove(i);
            }
        }
        errors.extend(kept);
    }
    errors
}
