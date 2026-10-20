//! MissingReturnRule (`return.missing`, `return.never`) on each function-like body's ExecutionEndNodes.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use pun_phpstan::types::{Level, NeverKind, Type, intersect, is_super_type_of, union};

use super::super::{Checker, declared};
use super::flow::list;

/// What a function-like declares: its description in messages, its return type and whether it is native.
struct Declared {
    description: String,
    ret: Type,
    native: bool,
}

/// A function-like body, its own start (an empty body reports there) and how it is described.
pub(in super::super) fn function(c: &mut Checker, f: &Function) {
    let name = String::from_utf8_lossy(c.names.get(&f.name.span())).into_owned();
    let native = f.return_type_hint.is_some();
    let ret = declared::with_classes(c, || late(c, &declared::function(c, f).ret, None, !native));
    let d = Declared {
        description: format!("Function {name}()"),
        ret,
        native: f.return_type_hint.is_some(),
    };
    check(c, &f.body, f.span(), d);
}

pub(in super::super) fn method(c: &mut Checker, m: &Method) {
    let MethodBody::Concrete(b) = &m.body else {
        return;
    };
    // (a method is walked in its class, a trait's in the using class)
    let Some(context) = c.class_scope() else { return };
    let Some(declaring) = declaring(c, m.span()) else { return };
    let name = method_name(c, m);
    let native = m.return_type_hint.is_some();
    let ret = declared::with_classes(c, || late(c, &declared::method(c, m, declaring, context).ret, Some(context), !native));
    let d = Declared {
        description: format!("Method {}::{name}()", c.display(context)),
        ret,
        native: m.return_type_hint.is_some(),
    };
    check(c, b, m.span(), d);
}

pub(in super::super) fn closure(c: &mut Checker, f: &Closure) {
    // `getAnonymousFunctionReturnType()` is checked only with a native return type
    let Some(hint) = &f.return_type_hint else {
        return;
    };
    let class = c.class_scope();
    let ret = declared::with_classes(c, || late(c, &declared::native(c, &hint.hint, class), class, false));
    let d = Declared {
        description: "Anonymous function".to_string(),
        ret,
        native: true,
    };
    check(c, &f.body, f.span(), d);
}

/// The `get` hooks with a statement body of property `name` (with its `$`), returning the property's type.
pub(in super::super) fn get_hooks(
    c: &mut Checker,
    name: &[u8],
    hint: Option<&Hint>,
    at: Span,
    hooks: &PropertyHookList,
) {
    for h in hooks.hooks.iter() {
        if !h.name.value.eq_ignore_ascii_case(b"get") {
            continue;
        }
        let PropertyHookBody::Concrete(PropertyHookConcreteBody::Block(b)) = &h.body else {
            continue;
        };
        // (a property is walked in its class)
        let Some(context) = c.class_scope() else { return };
        let ret = declared::with_classes(c, || {
            late(c, &declared::property(c, hint, at, &name[1..], context), Some(context), hint.is_none())
        });
        let d = Declared {
            description: format!(
                "Get hook for property {}::{}",
                c.display(context),
                String::from_utf8_lossy(name)
            ),
            ret,
            native: hint.is_some(),
        };
        check(c, b, h.span(), d);
    }
}

/// A method's name as analysed (a trait method under the alias the class gives it).
pub(super) fn method_name(c: &Checker, m: &Method) -> String {
    c.renamed
        .as_ref()
        .filter(|(span, _)| *span == m.span())
        .map_or_else(|| String::from_utf8_lossy(m.name.value).into_owned(), |(_, n)| n.clone())
}

pub(super) fn declaring<'a>(
    c: &Checker<'a, '_>,
    span: Span,
) -> Option<&'a mago_codex::metadata::class_like::ClassLikeMetadata> {
    let inside = |k: &&mago_codex::metadata::class_like::ClassLikeMetadata| {
        k.span.file_id == span.file_id
            && k.span.start.offset <= span.start.offset
            && span.end.offset <= k.span.end.offset
    };
    let class = c.class_scope()?;
    if inside(&class) {
        return Some(class);
    }
    class
        .used_traits
        .iter()
        .filter_map(|t| c.codebase.get_class_like(t.as_bytes()))
        .find(inside)
}

/// The return type as the function reflection gives it: `resolveLateResolvableTypes()` (a conditional type is
/// either branch), class names as declared, templates of an ancestor class (inherited PHPDoc whose generics the
/// class does not specify) resolved to their bounds, a nested intersection normalized and, for a PHPDoc type, a
/// `static` changed to the class (`transformStaticType()`, the class itself when final).
pub(super) fn late(c: &Checker, t: &Type, class: Option<&ClassLikeMetadata>, phpdoc: bool) -> Type {
    let go = |t: &Type| late(c, t, class, phpdoc);
    let own = |scope: &str| {
        !scope.starts_with("class ") || class.is_some_and(|k| scope == format!("class {}", declared::original_name(k)))
    };
    match t {
        Type::Conditional(cd) => union(vec![go(&cd.then), go(&cd.otherwise)]),
        Type::Template(tt) if !own(&tt.scope) => go(&tt.bound),
        Type::Intersection(ts) => intersect(ts.iter().map(go).collect()),
        // ponytail: a template's OffsetAccessType is not resolved to its bound's offset type
        Type::OffsetAccess(..) => Type::Error,
        // a KeyOfType / ValueOfType of a template, resolved now
        Type::Object(name, args) if (name == "key-of" || name == "value-of") && args.len() == 1 => {
            pun_phpstan::resolve::key_or_value_of(&go(&args[0]), name == "value-of")
        }
        Type::Object(name, args) => Type::Object(c.reflected_name(name.as_bytes()), args.iter().map(go).collect()),
        Type::Static(_, args) if phpdoc && let Some(k) = class => {
            let args = args.iter().map(go).collect();
            let name = c.reflected_name(k.name.as_bytes());
            if k.flags.is_final() || k.kind == SymbolKind::Enum {
                Type::Object(name, args)
            } else {
                Type::Static(name, args)
            }
        }
        Type::Static(name, args) => Type::Static(c.reflected_name(name.as_bytes()), args.iter().map(go).collect()),
        t => t.traverse(&go),
    }
}

fn is_mixed(t: &Type) -> bool {
    match t {
        Type::Mixed { .. } => true,
        Type::Template(t) => matches!(t.bound, Type::Mixed { .. }),
        _ => false,
    }
}

/// `TypeCombinator::containsNull()`.
fn contains_null(t: &Type) -> bool {
    match t {
        Type::Null => true,
        Type::Union(ts) | Type::Benevolent(ts) => ts.iter().any(contains_null),
        _ => false,
    }
}

/// `getTemplateType(Generator::class, 'TReturn')`, `None` for ErrorType.
pub(super) fn generator_return(t: &Type) -> Option<Type> {
    match t {
        Type::Object(name, args) if name.eq_ignore_ascii_case("Generator") => {
            Some(args.get(3).cloned().unwrap_or_else(pun_phpstan::types::mixed))
        }
        Type::Never(_) => Some(Type::Never(NeverKind::Implicit)),
        _ => None,
    }
}

fn check(c: &mut Checker, b: &Block, parent: Span, d: Declared) {
    let ret = d.ret;
    let results = super::flow::Results {
        types: &c.types,
        contents: &c.file.contents,
        codebase: Some(c.codebase),
    };
    let (_, ends) = results.execution_ends(&list(b.statements.as_slice()), parent, Some(b.right_brace));
    for end in ends {
        if end.flow.always {
            continue;
        }
        let mut ret = ret.clone();
        if is_super_type_of(&ret, &Type::Void).yes() && !is_mixed(&ret) {
            continue;
        }
        if end.flow.has_yield {
            if c.level >= 2
                && let Some(r) = generator_return(&ret)
            {
                ret = r;
                if ret.is_void().yes() {
                    continue;
                }
                if !is_mixed(&ret) {
                    c.report(
                        end.at,
                        "return.missing",
                        format!(
                            "{} should return {} but return statement is missing.",
                            d.description,
                            ret.describe(Level::TypeOnly)
                        ),
                        None,
                    );
                }
            }
            continue;
        }
        if !d.native && c.level < 2 && contains_null(&ret) {
            continue;
        }
        if matches!(ret, Type::Never(NeverKind::Explicit | NeverKind::NonAccepting)) {
            let message = format!(
                "{} should always throw an exception or terminate script execution but doesn't do that.",
                d.description
            );
            if d.native {
                c.report_non_ignorable(end.at, "return.never", message, None);
            } else {
                c.report(end.at, "return.never", message, None);
            }
            continue;
        }
        if matches!(ret, Type::Mixed { .. }) && !d.native {
            continue;
        }
        let message = format!(
            "{} should return {} but return statement is missing.",
            d.description,
            ret.describe(Level::TypeOnly)
        );
        if d.native {
            c.report_non_ignorable(end.at, "return.missing", message, None);
        } else {
            c.report(end.at, "return.missing", message, None);
        }
    }
}
