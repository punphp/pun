//! PHPStan 2.3.1's UnsetRule. Property checks precede the recursive offset/variable check; each argument
//! produces at most one error. Variable certainty is the statement's entry scope, not a syntactic read.

use std::collections::{HashMap, HashSet};

use mago_analyzer::plugin::HookContext;
use mago_codex::metadata::property::PropertyMetadata;
use mago_codex::ttype::union::TUnion;
use mago_codex::visibility::Visibility;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Accessory, ConstArray, Level, Tri, Type, is_super_type_of, union};

use super::types::flow::unparen;
use super::{Checker, declared, types};

/// Types of unset chains/indexes and target/root observations. The first target supplies statement-entry scope.
pub(super) fn expressions(program: &Program) -> (Vec<Span>, HashSet<Span>, HashMap<Span, Vec<(Span, Vec<u8>)>>) {
    struct Wanted {
        typed: Vec<Span>,
        scopes: HashSet<Span>,
        roots: HashMap<Span, Vec<(Span, Vec<u8>)>>,
    }
    impl Wanted {
        fn chain(&mut self, e: &Expression, entry: Span) {
            self.typed.push(e.span());
            match unparen(e) {
                Expression::Variable(Variable::Direct(v)) => {
                    self.scopes.insert(e.span());
                    self.roots.entry(entry).or_default().push((e.span(), v.name.to_vec()));
                }
                Expression::ArrayAccess(a) => {
                    self.chain(a.index, entry);
                    self.chain(a.array, entry);
                }
                Expression::Access(Access::Property(p)) => self.chain(p.object, entry),
                Expression::UnaryPostfix(u) => self.chain(u.operand, entry),
                _ => {}
            }
        }
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
        fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, _: &mut ()) {
            self.scopes.extend(u.values.iter().map(HasSpan::span));
            let Some(entry) = u.values.first().map(HasSpan::span) else { return };
            for e in &u.values {
                self.chain(e, entry);
            }
        }
    }
    let mut wanted = Wanted {
        typed: Vec::new(),
        scopes: HashSet::new(),
        roots: HashMap::new(),
    };
    wanted.walk_program(program, &mut ());
    (wanted.typed, wanted.scopes, wanted.roots)
}

/// A memoized property fetch need not visit its receiver. Retain its actual live root value too.
pub(super) fn observe(
    e: &Expression,
    context: &HookContext<'_, '_>,
    read: &mut HashMap<Span, TUnion>,
) {
    match unparen(e) {
        Expression::Variable(Variable::Direct(v)) => {
            if context.block_context().variable_certainty(mago_word::word(v.name)).is_absent() {
                read.insert(e.span(), mago_codex::ttype::get_error());
            } else if let Some(ty) = context.get_variable_type(v.name) {
                read.insert(e.span(), (**ty).clone());
            }
        }
        Expression::ArrayAccess(a) => observe(a.array, context, read),
        Expression::Access(Access::Property(p)) => observe(p.object, context, read),
        _ => {}
    }
}

pub(super) fn check(c: &mut Checker, u: &Unset) {
    let entry = u.values.first().map(HasSpan::span);
    for argument in &u.values {
        if let Expression::Access(Access::Property(p)) = unparen(argument)
            && let ClassLikeMemberSelector::Identifier(name) = &p.property
            && let Some((class, meta, native_readonly, phpdoc_readonly)) =
                types::native_unset_property(c, argument)
        {
            let property = format!(
                "{}::${}",
                c.display(class),
                String::from_utf8_lossy(name.value)
            );
            if native_readonly || phpdoc_readonly {
                let (kind, id) = if native_readonly {
                    ("readonly", "unset.readOnlyProperty")
                } else {
                    ("@readonly", "unset.readOnlyPropertyByPhpDoc")
                };
                c.report(
                    argument.span(),
                    id,
                    format!("Cannot unset {kind} {property} property."),
                    None,
                );
                continue;
            }
            let hooks = c.property_hooks(
                class,
                super::overriding_property::hook_names(meta),
                meta.name.0.as_bytes(),
            );
            if hooks.0 || hooks.1 {
                c.report(
                    argument.span(),
                    "unset.hookedProperty",
                    format!("Cannot unset hooked {property} property."),
                    None,
                );
                continue;
            }
            if c.php_version >= 80400
                && meta.read_visibility != Visibility::Private
                && !property_final(c, meta)
                && !class.flags.is_final()
                && !class
                    .original_name
                    .as_bytes()
                    .starts_with(b"{anonymous-class:")
            {
                c.report(
                    argument.span(),
                    "unset.possiblyHookedProperty",
                    format!("Cannot unset property {property} because it might have hooks in a subclass."),
                    None,
                );
                continue;
            }
        }
        can_be_unset(c, argument, entry);
    }
}

/// PhpPropertyReflection::isFinal(): the keyword, private(set), or its own @final tag.
fn property_final(c: &Checker, p: &PropertyMetadata) -> bool {
    p.flags.is_final()
        || p.write_visibility == Visibility::Private
        || c.declaration(p)
            .and_then(|(span, _)| declared::doc_at(c, span))
            .is_some_and(|(doc, _)| {
                super::declarations::has_tag(&super::declarations::doc_tags(&doc), &["@final"])
            })
}

fn can_be_unset(c: &mut Checker, node: &Expression, entry: Option<Span>) {
    match unparen(node) {
        Expression::Variable(Variable::Direct(v)) => {
            let name = String::from_utf8_lossy(&v.name[1..]);
            let certainty = entry
                .and_then(|s| c.variables.scopes.get(&s))
                .map(|s| s.get(&name));
            if certainty == Some(Tri::No) {
                c.report(
                    node.span(),
                    "unset.variable",
                    format!("Call to function unset() contains undefined variable ${name}."),
                    None,
                );
            }
        }
        Expression::ArrayAccess(a) => {
            if let (Some(ty), Some(index)) = (scope_type(c, a.array), scope_type(c, a.index))
                && declared::with_classes(c, || {
                    types::offset_accessible(&ty).no() || has_offset(c, &ty, &index).no()
                })
            {
                c.report(
                    node.span(),
                    "unset.offset",
                    format!(
                        "Cannot unset offset {} on {}.",
                        index.describe(Level::Value),
                        ty.describe(Level::Value)
                    ),
                    None,
                );
                return;
            }
            can_be_unset(c, a.array, entry);
        }
        _ => {}
    }
}

/// Scope::getType() uses the unset statement's entry scope, while its index expressions have already run.
/// Definitely absent variables are ErrorType, not Mago's synthetic null.
fn scope_type(c: &Checker, e: &Expression) -> Option<Type> {
    if let Expression::UnaryPostfix(u) = unparen(e) {
        // A postfix result is its input value, even on a counterfactual entry scope.
        // Its real increment was already applied by the authoritative walk.
        return scope_type(c, u.operand);
    }
    if let Expression::Variable(Variable::Direct(v)) = unparen(e) {
        let name = String::from_utf8_lossy(&v.name[1..]);
        let certainty = c
            .variables
            .scopes
            .get(&e.span())
            .map(|s| s.get(&name))
            .or_else(|| {
                c.variables
                    .reads
                    .get(&v.span)
                    .map(|(_, certainty, _)| *certainty)
            });
        if certainty == Some(Tri::No) {
            return c.phpstan_type(e);
        }
        if let Some(entry) = c.variables.unset_entries.get(&e.span()) {
            let entry = c.phpstan_type_from(e, entry);
            let actual = c.phpstan_type(e)?;
            // NodeCallbackScope keeps stored reads that are narrower than the asking
            // entry scope. An incompatible virtual mutation must be priced on entry.
            return Some(if declared::with_classes(c, || is_super_type_of(&entry, &actual).yes()) {
                actual
            } else {
                entry
            });
        }
    }
    if let Expression::ArrayAccess(a) = unparen(e)
        && let (Some(base), Some(index)) = (scope_type(c, a.array), scope_type(c, a.index))
    {
        if declared::with_classes(c, || {
            matches!(base, Type::Error)
                || types::offset_accessible(&base).no()
                || has_offset(c, &base, &index).no()
        }) {
            return Some(Type::Error);
        }
        let offset = Type::OffsetAccess(Box::new(base), Box::new(index));
        let value = declared::with_classes(c, || pun_phpstan::resolve::resolve_late(&offset));
        if value != offset {
            return Some(value);
        }
    }
    c.phpstan_type(e)
}

/// `TrinaryLogic::extremeIdentity()` for union receivers and finite union offsets.
fn identity(results: impl Iterator<Item = Tri>) -> Tri {
    results
        .reduce(|a, b| if a == b { a } else { Tri::Maybe })
        .unwrap_or(Tri::No)
}

/// `hasOffsetValueType()`: no RuleLevelHelper filtering (in particular, no union member is discarded).
fn has_offset(c: &Checker, ty: &Type, offset: &Type) -> Tri {
    match ty {
        Type::Union(ts) => identity(ts.iter().map(|t| has_offset(c, t, offset))),
        Type::Benevolent(ts) => ts
            .iter()
            .map(|t| has_offset(c, t, offset))
            .fold(Tri::No, Tri::or),
        Type::Intersection(ts) => {
            if ty.is_callable().yes() && ty.is_array().yes() {
                return is_super_type_of(
                    &union(vec![Type::ConstInt(0), Type::ConstInt(1)]),
                    &array_key(offset),
                );
            }
            if ty.is_list().yes() {
                let key = array_key(offset);
                if is_super_type_of(&Type::IntRange(None, Some(-1)), &key).yes() {
                    return Tri::No;
                }
                if ty.is_iterable_at_least_once().yes()
                    && is_super_type_of(&Type::ConstInt(0), &key).yes()
                {
                    return Tri::Yes;
                }
            }
            let results: Vec<_> = ts.iter().map(|t| has_offset(c, t, offset)).collect();
            if results.contains(&Tri::Yes) {
                Tri::Yes
            } else {
                results.into_iter().min().unwrap_or(Tri::No)
            }
        }
        Type::Template(t) => has_offset(c, &t.bound, offset),
        Type::ConstArray(a) => shape_offset(a, &array_key(offset)),
        Type::Array(key, _) => {
            let offset = array_key(offset);
            let incompatible = is_super_type_of(key, &offset).no()
                && (offset.is_string().no() || constant(&offset));
            if incompatible { Tri::No } else { Tri::Maybe }
        }
        Type::Iterable(key, _) => is_super_type_of(key, offset).and(Tri::Maybe),
        Type::Accessory(Accessory::List) => {
            is_super_type_of(&Type::IntRange(Some(0), None), offset).and(Tri::Maybe)
        }
        Type::ConstString(s) if is_super_type_of(&Type::Int, offset).yes() => {
            let len = s.len() as i64;
            is_super_type_of(&Type::IntRange(Some(-len), Some(len - 1)), offset)
        }
        t if t.is_string().yes() => is_super_type_of(&Type::Int, offset).and(Tri::Maybe),
        Type::Null => Tri::No,
        Type::Object(..) | Type::Static(..) | Type::This(_) | Type::EnumCase(..) => {
            if is_super_type_of(&Type::Object("ArrayAccess".into(), vec![]), ty).yes()
                && let Some((key, _)) = types::offset_types(c, ty)
            {
                return is_super_type_of(&key, offset).and(Tri::Maybe);
            }
            types::offset_accessible(ty).and(Tri::Maybe)
        }
        _ => types::offset_accessible(ty).and(Tri::Maybe),
    }
}

/// `ConstantArrayType::recursiveHasOffsetValueType()` after key conversion.
fn shape_offset(array: &ConstArray, offset: &Type) -> Tri {
    if let Type::Union(ts) | Type::Benevolent(ts) = offset {
        return identity(ts.iter().map(|t| shape_offset(array, t)));
    }
    let mut result = Tri::No;
    for (i, key) in array.keys.iter().enumerate() {
        if matches!(key, Type::ConstInt(_)) && !offset.is_string().no() && !constant(offset) {
            return Tri::Maybe;
        }
        let has = is_super_type_of(key, offset);
        if has.yes() {
            return if array.optional.contains(&i) {
                Tri::Maybe
            } else {
                Tri::Yes
            };
        }
        if has.maybe() {
            result = Tri::Maybe;
        }
    }
    if result.no()
        && array
            .unsealed
            .as_ref()
            .is_some_and(|(key, _)| !is_super_type_of(key, offset).no())
    {
        return Tri::Maybe;
    }
    result
}

fn constant(t: &Type) -> bool {
    matches!(t, Type::ConstInt(_) | Type::ConstString(_))
}

/// `toArrayKey()` for offsets: the original offset type is still used in the diagnostic.
fn array_key(t: &Type) -> Type {
    match t {
        Type::Union(ts) | Type::Benevolent(ts) => union(ts.iter().map(array_key).collect()),
        Type::ConstString(s) if s.parse::<i64>().is_ok_and(|i| i.to_string() == *s) => {
            Type::ConstInt(s.parse().expect("canonical integer"))
        }
        Type::ConstBool(b) => Type::ConstInt(i64::from(*b)),
        Type::Bool => union(vec![Type::ConstInt(0), Type::ConstInt(1)]),
        Type::Float => Type::Int,
        Type::ConstFloat(f) => {
            // PHP's float-to-int conversion truncates modulo 2^64, unlike Rust's saturating cast.
            let word = 18446744073709551616.0;
            let mut value = if f.is_finite() { f.trunc() % word } else { 0.0 };
            if value >= -(i64::MIN as f64) {
                value -= word;
            } else if value < i64::MIN as f64 {
                value += word;
            }
            Type::ConstInt(value as i64)
        }
        Type::Null => Type::ConstString(String::new()),
        t => t.clone(),
    }
}
