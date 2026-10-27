//! Level 3's ThrowsVoidFunctionWithExplicitThrowPointRule, ThrowsVoidMethodWithExplicitThrowPointRule and
//! ThrowsVoidPropertyHookWithExplicitThrowPointRule (`throws.void`): an explicit throw point of a function-like whose
//! PHPDoc says `@throws void`, with the default `missingCheckedExceptionInThrows` (off). Also
//! CatchWithUnthrownExceptionRule's `catch.alreadyCaught`: a catch whose types the catches above take.

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Level, Type, is_super_type_of};

use super::super::doc::{doc_before, tags};
use super::super::members::{self, Method as Member};
use super::super::{Checker, declared};
use super::flow::unparen;
use super::missing_return::method_name;

pub const IDENTIFIERS: &[&str] = &["throws.void", "catch.alreadyCaught"];

/// `getThrowType()->isVoid()->yes()` of the function-like at `at`.
fn throws_void(c: &Checker, at: Span) -> bool {
    let Some(doc) = doc_before(&c.file.contents, at.start.offset as usize) else { return false };
    let throws: Vec<String> = tags(doc)
        .into_iter()
        .filter(|(t, _)| t == "@throws" || t == "@phpstan-throws")
        .map(|(_, v)| v.split_whitespace().next().unwrap_or_default().to_ascii_lowercase())
        .collect();
    !throws.is_empty() && throws.iter().all(|t| t == "void")
}

/// The `@throws` types of a user function-like (built-in ones and `@throws void` give none).
// ponytail: built-in functions' throw types (stubs, dynamic throw type extensions) are not read
fn declared_throws(c: &Checker, f: &FunctionLikeMetadata) -> Vec<Type> {
    if f.flags.is_built_in() {
        return Vec::new();
    }
    f.thrown_types
        .iter()
        .filter(|t| t.from_docblock && !t.type_union.is_void())
        .map(|t| c.to_phpstan(&t.type_union))
        .collect()
}

/// `TypeUtils::flattenTypes()`.
fn flatten(t: Type) -> Vec<Type> {
    match t {
        Type::Union(ts) => ts,
        t => vec![t],
    }
}

/// The explicit throw points of a body (not of the functions and classes in it), as each thrown type and where.
struct Points<'c, 'a, 'arena> {
    c: &'c Checker<'a, 'arena>,
    points: Vec<(Span, Type)>,
}

impl<'a> Points<'_, 'a, '_> {
    fn class_of(&self, e: &Expression) -> Option<&'a mago_codex::metadata::class_like::ClassLikeMetadata> {
        match unparen(e) {
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => self.c.class_scope(),
            Expression::Self_(_) | Expression::Static(_) => self.c.class_scope(),
            Expression::Identifier(id) => self.c.codebase.get_class_like(self.c.names.get(id)),
            // (a nullsafe call's holder without its null)
            e => match self.c.type_of(e)?.types.iter().filter(|t| !t.is_null()).collect::<Vec<_>>().as_slice() {
                [TAtomic::Object(TObject::Named(o))] => self.c.codebase.get_class_like(o.name.as_bytes()),
                _ => None,
            },
        }
    }

    fn method(&mut self, at: Span, on: &Expression, name: &ClassLikeMemberSelector) {
        let ClassLikeMemberSelector::Identifier(name) = name else { return };
        let Some(class) = self.class_of(on) else { return };
        if let Some(Member::User { f, .. }) = members::method(self.c, class, name.value) {
            let types = declared_throws(self.c, f);
            self.points.extend(types.into_iter().map(|t| (at, t)));
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Points<'_, '_, '_> {
    fn walk_in_throw(&mut self, t: &'ast Throw<'arena>, _: &mut ()) {
        if let Some(ty) = self.c.phpstan_type(t.exception) {
            self.points.push((t.span(), ty));
        }
    }
    fn walk_in_function_call(&mut self, f: &'ast FunctionCall<'arena>, _: &mut ()) {
        let Expression::Identifier(id) = unparen(f.function) else { return };
        if let Some(meta) = self.c.codebase.get_function(self.c.names.get(id)) {
            let types = declared_throws(self.c, meta);
            self.points.extend(types.into_iter().map(|t| (f.span(), t)));
        }
    }
    fn walk_in_method_call(&mut self, m: &'ast MethodCall<'arena>, _: &mut ()) {
        self.method(m.span(), m.object, &m.method);
    }
    fn walk_in_null_safe_method_call(&mut self, m: &'ast NullSafeMethodCall<'arena>, _: &mut ()) {
        self.method(m.span(), m.object, &m.method);
    }
    fn walk_in_static_method_call(&mut self, m: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.method(m.span(), m.class, &m.method);
    }
    fn walk_in_instantiation(&mut self, n: &'ast Instantiation<'arena>, _: &mut ()) {
        let Some(class) = self.class_of(n.class).filter(|_| matches!(unparen(n.class), Expression::Identifier(_)))
        else {
            return;
        };
        if let Some(Member::User { f, .. }) = members::method(self.c, class, b"__construct") {
            let types = declared_throws(self.c, f);
            self.points.extend(types.into_iter().map(|t| (n.span(), t)));
        }
    }
    /// The try block's points a catch catches are gone.
    fn walk_try(&mut self, t: &'ast Try<'arena>, _: &mut ()) {
        let mut inner = Points { c: self.c, points: Vec::new() };
        inner.walk_block(&t.block, &mut ());
        let class = self.c.class_scope();
        let caught: Vec<Type> = t
            .catch_clauses
            .iter()
            .map(|k| declared::with_classes(self.c, || declared::native(self.c, &k.hint, class)))
            .collect();
        for (at, ty) in inner.points {
            let left: Vec<Type> = flatten(ty)
                .into_iter()
                .filter(|t| !declared::with_classes(self.c, || caught.iter().any(|k| is_super_type_of(k, t).yes())))
                .collect();
            self.points.extend(left.into_iter().map(|t| (at, t)));
        }
        for k in t.catch_clauses.iter() {
            self.walk_block(&k.block, &mut ());
        }
        if let Some(f) = &t.finally_clause {
            self.walk_block(&f.block, &mut ());
        }
    }
    fn walk_closure(&mut self, _: &'ast Closure<'arena>, _: &mut ()) {}
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {}
    fn walk_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {}
    fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
    fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {}
}

fn check(c: &mut Checker, at: Span, of: &str, walk: impl FnOnce(&mut Points)) {
    if c.level < 3 || !throws_void(c, at) {
        return;
    }
    let mut p = Points { c, points: Vec::new() };
    walk(&mut p);
    let points = p.points;
    for (at, ty) in points {
        for t in flatten(ty) {
            let described = declared::with_classes(c, || t.describe(Level::TypeOnly));
            let message = format!("{of} throws exception {described} but the PHPDoc contains @throws void.");
            c.report(at, "throws.void", message, None);
        }
    }
}

pub(in super::super) fn function(c: &mut Checker, f: &Function) {
    let name = String::from_utf8_lossy(c.names.get(&f.name.span())).into_owned();
    check(c, f.span(), &format!("Function {name}()"), |p| p.walk_block(&f.body, &mut ()));
}

pub(in super::super) fn method(c: &mut Checker, m: &Method) {
    let (MethodBody::Concrete(b), Some(context)) = (&m.body, c.class_scope()) else { return };
    let of = format!("Method {}::{}()", c.display(context), method_name(c, m));
    check(c, m.span(), &of, |p| p.walk_block(b, &mut ()));
}

pub(in super::super) fn hooks(c: &mut Checker, name: &[u8], list: &PropertyHookList) {
    let Some(context) = c.class_scope() else { return };
    let display = c.display(context);
    for h in list.hooks.iter() {
        let PropertyHookBody::Concrete(body) = &h.body else { continue };
        let hook = String::from_utf8_lossy(h.name.value).to_ascii_lowercase();
        let mut chars = hook.chars();
        let hook: String = chars.next().map(|f| f.to_ascii_uppercase()).into_iter().chain(chars).collect();
        let of = format!("{hook} hook for property {display}::{}", String::from_utf8_lossy(name));
        check(c, h.span(), &of, |p| match body {
            PropertyHookConcreteBody::Block(b) => p.walk_block(b, &mut ()),
            PropertyHookConcreteBody::Expression(e) => p.walk_expression(e.expression, &mut ()),
        });
    }
}

/// `catch.alreadyCaught`: a catch type `TypeCombinator::remove()` of the types the catches above catch leaves nothing.
/// A `Throwable` one matches every throw point of the try block instead.
// ponytail: an empty try block (no throw points) still counts as throwing for a `Throwable` catch; `catch.neverThrown`
// needs the try block's implicit throw points, not ported
pub(in super::super) fn catches(c: &mut Checker, t: &Try) {
    if c.level < 4 {
        return;
    }
    let class = c.class_scope();
    let throwable = Type::Object("Throwable".into(), vec![], false);
    let mut past: Vec<Type> = Vec::new();
    for k in t.catch_clauses.iter() {
        let items = flatten(declared::with_classes(c, || declared::native(c, &k.hint, class)));
        for item in &items {
            let dead = declared::with_classes(c, || {
                !is_super_type_of(item, &throwable).yes() && past.iter().any(|p| is_super_type_of(p, item).yes())
            });
            if dead {
                let described = declared::with_classes(c, || item.describe(Level::TypeOnly));
                let message = format!("Dead catch - {described} is already caught above.");
                c.report(k.r#catch.span(), "catch.alreadyCaught", message, None);
            }
        }
        past.extend(items);
    }
}
