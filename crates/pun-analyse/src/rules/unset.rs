//! PHPStan 2.3.5's UnsetRule. Property checks precede the recursive offset/variable check; each argument
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
            && let Some((class, meta, native_readonly, phpdoc_readonly, private)) =
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
                && !private
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
        // Private read visibility implies private writes, not PHPStan's asymmetric `private(set)` finality.
        || (p.write_visibility == Visibility::Private && p.read_visibility != Visibility::Private)
        || c.declaration(p)
            .and_then(|(span, _)| declared::doc_at(c, span))
            .is_some_and(|(_, entry, _)| super::declarations::has_tag(&entry.names, &["@final"]))
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
            .map(|s| s.get(&name));
        if certainty == Some(Tri::No) {
            return c.phpstan_type(e);
        }
        let entry = c.variables.unset_entries.get(&e.span())?;
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

/// The result queried by LateResolvableTypeTrait. Resolve the receiver, not templates stored in its values.
/// Resolving the whole offset to bounds would erase TemplateMixedType and TemplateTypeTrait::toArrayKey.
pub(super) fn late_offset(on: &Type, index: &Type) -> Option<Type> {
    let result = match on {
        Type::Template(t) => {
            let bound = match &t.bound {
                Type::SubtractedMixed { .. } => pun_phpstan::types::mixed(),
                Type::ConstArray(a) => {
                    let mut parent = a.clone();
                    parent.unsealed = None;
                    Type::ConstArray(parent)
                }
                bound => bound.clone(),
            };
            return late_offset(&bound, index);
        }
        Type::Conditional(c) => return late_offset(&c.resolve(), index),
        Type::Array(key, value) => {
            let index = array_key(index);
            if is_super_type_of(key, &index).no() && (index.is_string().no() || constant(&index)) {
                Type::Error
            } else {
                if value.as_ref() == &Type::Error { pun_phpstan::types::mixed() } else { (**value).clone() }
            }
        }
        Type::ConstArray(a) => {
            fn scalar(t: &Type) -> Tri {
                match t {
                    Type::Template(t) => scalar(&t.bound),
                    Type::Union(ts) | Type::Benevolent(ts) => Tri::extreme_identity(ts.iter().map(scalar)),
                    Type::ConstInt(_) | Type::ConstString(_) | Type::ConstFloat(_) | Type::ConstBool(_) | Type::Never(_) => Tri::Yes,
                    Type::Mixed { .. } | Type::SubtractedMixed { .. } | Type::Error => Tri::Maybe,
                    _ => Tri::No,
                }
            }
            if a.keys.is_empty() && a.unsealed.is_none() {
                return Some(Type::Error);
            }
            let index = array_key(index);
            let scalar_index = scalar(&index);
            let mut values = Vec::new();
            let mut maybe_all = true;
            for (key, value) in a.keys.iter().zip(&a.values) {
                if is_super_type_of(key, &index).no() {
                    if matches!(key, Type::ConstInt(_)) && !index.is_string().no() && scalar_index.no() {
                        continue;
                    }
                    maybe_all = false;
                } else {
                    values.push(value.clone());
                }
            }
            if let Some((key, value)) = &a.unsealed
                && !is_super_type_of(&union(a.keys.clone()), &index).yes()
                && !is_super_type_of(key, &index).no()
            {
                values.push(value.clone());
            }
            if values.is_empty() {
                if !maybe_all {
                    return Some(Type::Error);
                }
                values.extend(a.values.iter().cloned());
                values.extend(a.unsealed.iter().map(|(_, value)| value.clone()));
            }
            match union(values) {
                Type::Error => pun_phpstan::types::mixed(),
                value => value,
            }
        }
        _ => {
            let late = Type::OffsetAccess(Box::new(on.clone()), Box::new(index.clone()));
            let resolved = pun_phpstan::resolve::resolve_late(&late);
            return (resolved != late).then_some(resolved);
        }
    };
    Some(result)
}

/// `hasOffsetValueType()`: no RuleLevelHelper filtering (in particular, no union member is discarded).
pub(super) fn has_offset(c: &Checker, ty: &Type, offset: &Type) -> Tri {
    match ty {
        Type::Union(ts) => Tri::extreme_identity(ts.iter().map(|t| has_offset(c, t, offset))),
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
        Type::Template(t) => match &t.bound {
            Type::SubtractedMixed { .. } => Tri::Maybe,
            Type::ConstArray(array) => {
                // TemplateConstantArrayType displays its bound, but its parent constructor is sealed.
                let mut parent = array.clone();
                parent.unsealed = None;
                shape_offset(&parent, &array_key(offset))
            }
            bound => has_offset(c, bound, offset),
        },
        Type::Conditional(conditional) => has_offset(c, &conditional.resolve(), offset),
        Type::OffsetAccess(on, index) => late_offset(on, index).map_or(Tri::Maybe, |resolved| has_offset(c, &resolved, offset)),
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
        Type::Accessory(Accessory::HasOffsetValue(key, _)) => {
            if key.as_ref() == &array_key(offset) { Tri::Yes } else { Tri::Maybe }
        }
        Type::ConstString(s) if is_super_type_of(&Type::Int, offset).yes() => {
            let len = s.len() as i64;
            is_super_type_of(&Type::IntRange(Some(-len), Some(len - 1)), offset)
        }
        t if t.is_string().yes() => is_super_type_of(&Type::Int, offset).and(Tri::Maybe),
        Type::Null => Tri::No,
        Type::Object(..) | Type::Static(..) | Type::This(_) | Type::EnumCase(..) => {
            if is_super_type_of(&Type::Object("ArrayAccess".into(), vec![], false), ty).yes()
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
        return Tri::extreme_identity(ts.iter().map(|t| shape_offset(array, t)));
    }
    let mut result = Tri::No;
    for (i, key) in array.keys.iter().enumerate() {
        if matches!(key, Type::ConstInt(_))
            && !offset.is_string().no()
            && !constant(offset)
            && !offset.is_decimal_int_string().no()
        {
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
        // MixedType::toArrayKey ignores exclusions: floats, booleans and null also cast to keys.
        // TemplateTypeTrait deliberately retains the template itself instead.
        Type::Mixed { .. } | Type::SubtractedMixed { .. } | Type::Error => Type::Benevolent(vec![Type::Int, Type::String]),
        Type::Conditional(conditional) => array_key(&conditional.resolve()),
        Type::OffsetAccess(on, index) => late_offset(on, index).map_or_else(|| array_key(&pun_phpstan::types::mixed()), |resolved| array_key(&resolved)),
        Type::ConstString(s) if std::str::from_utf8(s).ok().and_then(|s| s.parse::<i64>().ok()).is_some_and(|i| i.to_string().as_bytes() == s) => {
            Type::ConstInt(std::str::from_utf8(s).unwrap().parse().expect("canonical integer"))
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
        Type::Null => Type::ConstString(Vec::new()),
        t => t.clone(),
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_codex::metadata::CodebaseMetadata;
    use mago_database::file::{File, FileType};
    use mago_names::resolver::NameResolver;
    use mago_syntax::parser::parse_file;
    use pun_phpstan::types::{Conditional, Level, Subject, Template, Type, mixed};
    use serde_json::{Value, json};

    use super::{Checker, array_key, declared, has_offset, types};
    use crate::rules::{include_file, reflection};

    /// These late and fully excluded values cannot all be expressed by the current PHPDoc/flow producer.
    /// Query the exact PHPStan type objects, without broadening the public source-type restoration layer.
    #[test]
    fn offset_type_queries_match_pinned_phpstan() {
        let arena = LocalArena::new();
        let file = File::new(Cow::Borrowed(b"offset-queries.php"), FileType::Host, None, Cow::Borrowed(b"<?php"));
        let program = parse_file(&arena, &file);
        let names = NameResolver::new(&arena).resolve(program);
        let codebase = CodebaseMetadata::new();
        let project = crate::Project::new(Vec::new(), Vec::new());
        let settings = include_file::Settings {
            cwd: project.current_working_directory.clone(),
            include_paths: Vec::new(),
            magic_dir: false,
            path_constants: false,
        };
        let facts = super::super::Facts::of(&file.contents, program, true);
        let function_likes = std::collections::HashMap::new();
        let natives = super::super::driver::Natives::load(80400);
        let (_, include_state) =
            include_file::prepare(&arena, &file, program, &names, &codebase, 80400, true, true, true, &settings, &facts.comments, &[]);
        let exceptions = crate::Exceptions::default();
        let c = Checker {
            file: &file,
            facts: &facts,
            function_likes: &function_likes,
            anonymous: Default::default(),
            property_access: Default::default(),
            relative_path: &project.relative_path,
            types: Default::default(),
            keep_void_types: Default::default(),
            native_types: Default::default(),
            nullsafe_method_receiver_types: Default::default(),
            property_offset_writes: Default::default(),
            variable_writes: Default::default(),
            property_initializations: Default::default(),
            call_terminations: Default::default(),
            named_call_scopes: Default::default(),
            defined: Default::default(),
            variables: Default::default(),
            origins: Default::default(),
            names: &names,
            codebase: &codebase,
            classes: Vec::new(),
            guards: Vec::new(),
            marks: Vec::new(),
            vars: Default::default(),
            saved: Vec::new(),
            functions: Vec::new(),
            var_tag_functions: Vec::new(),
            in_exists_call: 0,
            top_level: Default::default(),
            piped: Default::default(),
            rebound: Default::default(),
            label: String::new(),
            trait_context: None,
            trait_uses: Vec::new(),
            renamed: None,
            php_version: 80400,
            builtins: reflection::builtins(80400),
            natives: &natives,
            resolving_aliases: Default::default(),
            trait_lines: Vec::new(),
            frames: Vec::new(),
            namespaces: Vec::new(),
            magic_defaults: Default::default(),
            walk: crate::rules::Walk::FIRST_STATEMENT,
            declares_not_first: Vec::new(),
            decls: Vec::new(),
            immutable: Default::default(),
            files: &Default::default(),
            sources: &Default::default(),
            patched_docs: &Default::default(),
            class_cache: &Default::default(),
            syntax: Default::default(),
            assign: Default::default(),
            signatures: Default::default(),
            class_names: Default::default(),
            arrays: Default::default(),
            standalone_throws: Default::default(),
            level: 0,
            switches: super::super::Switches::CHECK_THIS_ONLY | super::super::Switches::TREAT_PHP_DOC_TYPES_AS_CERTAIN | super::super::Switches::CLI_ARGUMENTS_VARIABLES_REGISTERED | super::super::Switches::POLLUTE_SCOPE_WITH_LOOP_INITIAL_ASSIGNMENTS | super::super::Switches::POLLUTE_SCOPE_WITH_ALWAYS_ITERABLE_FOREACH | super::super::Switches::POLLUTE_SCOPE_WITH_BLOCK,
            method_rules: project.method_rules,
            larastan: None,
            larastan_state: Default::default(),
            internal: None,
            unreachable: Default::default(),
            conditions: Default::default(),
            strict: Default::default(),
            exceptions: &exceptions,
            too_wide: Default::default(),
            include_settings: &settings,
            include_state,
            closure_bind: 0,
            toggled: &Default::default(),
            by_ref_foreach: Vec::new(),
            errors: Vec::new(),
        };
        let template = |bound| Type::Template(Box::new(Template {
            name: "T".into(), scope: "function offsetQueries()".into(), bound, default: None, argument: false,
        }));
        let conditional = |subject, then, otherwise| Type::Conditional(Box::new(Conditional {
            subject: Subject::Type(subject), target: Type::Int, then, otherwise, negated: false,
        }));
        let array = Type::array(mixed(), mixed());
        let excluded = Type::mixed_subtracted(false, Type::Union(vec![
            Type::String, array.clone(), Type::Object("ArrayAccess".into(), vec![], false),
        ]));
        let shape = pun_phpstan::resolve::resolve_str("array{0: int}", None);
        let open = Type::ConstArray(Box::new(pun_phpstan::types::ConstArray {
            keys: vec![], values: vec![], next_auto_indexes: vec![0], optional: vec![],
            is_list: pun_phpstan::types::Tri::No, unsealed: Some((Type::String, Type::Int)),
        }));
        let rows = [
            ("mixed", mixed()),
            ("not-zero", Type::mixed_subtracted(false, Type::ConstInt(0))),
            ("not-array-keys", Type::mixed_subtracted(false, Type::Union(vec![Type::Int, Type::String]))),
            ("not-string-array", Type::mixed_subtracted(false, Type::Union(vec![Type::String, array.clone()]))),
            ("not-offsets", excluded.clone()),
            ("template-not-offsets", template(excluded.clone())),
            ("union-no", Type::Union(vec![excluded.clone(), Type::Int])),
            ("union-maybe", Type::Union(vec![excluded.clone(), Type::String])),
            ("benevolent-no", Type::Benevolent(vec![excluded.clone(), Type::Int])),
            ("intersection-array", Type::Intersection(vec![excluded.clone(), array])),
            ("conditional-array", conditional(Type::Int, shape.clone(), Type::Int)),
            ("conditional-union", conditional(mixed(), shape.clone(), Type::String)),
            ("conditional-key", conditional(Type::Int, Type::ConstInt(1), Type::ConstInt(0))),
            ("late-array", Type::OffsetAccess(Box::new(template(Type::array(Type::String, shape.clone()))), Box::new(Type::ConstString("inner".into())))),
            ("late-key", Type::OffsetAccess(Box::new(template(Type::array(Type::String, Type::ConstInt(1)))), Box::new(Type::ConstString("inner".into())))),
            ("late-object-mixed", Type::OffsetAccess(Box::new(Type::Object("ArrayAccess".into(), vec![Type::Int, mixed()], false)), Box::new(Type::ConstInt(0)))),
            ("late-array-error", Type::OffsetAccess(Box::new(Type::array(Type::String, Type::Error)), Box::new(Type::ConstString("inner".into())))),
            ("late-array-invalid-key", Type::OffsetAccess(Box::new(Type::array(Type::Int, Type::ConstInt(1))), Box::new(Type::ConstString("inner".into())))),
            ("late-shape-error", Type::OffsetAccess(Box::new(Type::ConstArray(Box::new(pun_phpstan::types::ConstArray {
                keys: vec![Type::ConstInt(0)], values: vec![Type::Error], next_auto_indexes: vec![1], optional: vec![], is_list: pun_phpstan::types::Tri::Yes, unsealed: None,
            }))), Box::new(Type::ConstInt(0)))),
            ("late-shape-template-value", Type::OffsetAccess(Box::new(Type::ConstArray(Box::new(pun_phpstan::types::ConstArray {
                keys: vec![Type::ConstInt(0)], values: vec![template(Type::Float)], next_auto_indexes: vec![1], optional: vec![], is_list: pun_phpstan::types::Tri::Yes, unsealed: None,
            }))), Box::new(Type::ConstInt(0)))),
            ("late-array-template-value", Type::OffsetAccess(Box::new(template(Type::array(Type::String, template(Type::Float)))), Box::new(Type::ConstString("inner".into())))),
            ("late-array-template-mixed", Type::OffsetAccess(Box::new(Type::array(Type::String, template(excluded.clone()))), Box::new(Type::ConstString("inner".into())))),
            ("late-template-mixed-container", Type::OffsetAccess(Box::new(template(excluded)), Box::new(Type::ConstInt(0)))),
            ("late-conditional-container", Type::OffsetAccess(Box::new(conditional(Type::Int, Type::array(Type::String, Type::ConstInt(1)), Type::Int)), Box::new(Type::ConstString("inner".into())))),
            ("late-open", Type::OffsetAccess(Box::new(open.clone()), Box::new(Type::ConstString("inner".into())))),
            ("late-open-template", Type::OffsetAccess(Box::new(template(open.clone())), Box::new(Type::ConstString("inner".into())))),
            ("late-unknown-shape", Type::OffsetAccess(Box::new(pun_phpstan::resolve::resolve_str("array{inner: mixed}", None)), Box::new(Type::String))),
            ("late-shape-general", Type::OffsetAccess(Box::new(pun_phpstan::resolve::resolve_str("array{inner: int}", None)), Box::new(Type::String))),
            ("late-shape-numeric", Type::OffsetAccess(Box::new(shape.clone()), Box::new(Type::String))),
            ("late-shape-bad-union", Type::OffsetAccess(Box::new(shape.clone()), Box::new(Type::Union(vec![Type::ConstString("bad".into()), Type::ConstString("missing".into())])))),
            ("late-shape-mixed-key", Type::OffsetAccess(Box::new(shape.clone()), Box::new(template(mixed())))),
            ("late-shape-template-key", Type::OffsetAccess(Box::new(shape), Box::new(template(Type::ConstString("bad".into()))))),
            ("late-shape-nonstring", Type::OffsetAccess(Box::new(pun_phpstan::resolve::resolve_str("array{inner: int}", None)), Box::new(Type::Int))),
            ("late-open-incompatible", Type::OffsetAccess(Box::new(open), Box::new(Type::Int))),
            ("late-shape-covered-extra", Type::OffsetAccess(Box::new(Type::ConstArray(Box::new(pun_phpstan::types::ConstArray {
                keys: vec![Type::ConstInt(0)], values: vec![template(Type::Float)], next_auto_indexes: vec![1], optional: vec![], is_list: pun_phpstan::types::Tri::No, unsealed: Some((Type::String, Type::Int)),
            }))), Box::new(Type::ConstInt(0)))),
            ("template-key", template(Type::Int)),
        ];
        let expected: Value = serde_json::from_str(include_str!("../../tests/fixtures/unset/capability-types.json")).unwrap();
        let tri = |result| match result {
            pun_phpstan::types::Tri::Yes => "yes",
            pun_phpstan::types::Tri::No => "no",
            pun_phpstan::types::Tri::Maybe => "maybe",
        };
        declared::with_classes(&c, || {
            for (name, ty) in rows {
                let mut actual = json!({
                    "accessible": tri(types::offset_accessible(&ty)),
                    "hasOffset": tri(has_offset(&c, &ty, &Type::ConstInt(0))),
                });
                if expected[name].get("arrayKey").is_some() {
                    actual["arrayKey"] = array_key(&ty).describe(Level::Precise).into();
                }
                assert_eq!(actual, expected[name], "{name}: {ty:?}");
            }
        });
    }
}
