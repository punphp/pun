//! CallTo{Function,Method,StaticMethod,Constructor}StatementWithoutImpurePointsRule: a call statement to a user
//! function, method or constructor whose body has no impure points (transitively through the calls it makes) and no
//! explicit throw points. The collectors' data travels as encoded errors that [`resolve()`] turns into the reports
//! once every file is analysed (PossiblyPureCallTransitivePurityResolver).
//! ponytail: a trait's methods declare nothing; a method call is resolved on one class only.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::ignore::Error;

use super::super::Checker;
use super::super::types::flow::unparen;

/// The prefix of an encoded entry: a declaration without impure points or a possibly pure call statement.
const ENTRY: &str = "\u{1e}impure-points\u{1f}";

pub(super) fn function_key(name: &str) -> String {
    format!("f\0{}", name.to_ascii_lowercase())
}

pub(super) fn method_key(class: &str, method: &str) -> String {
    format!("m\0{}\0{}", class.to_ascii_lowercase(), method.to_ascii_lowercase())
}

/// The {Function,Method,Constructor}WithoutImpurePointsCollector conditions on a declaration's reflection.
fn collectable(c: &Checker, f: &FunctionLikeMetadata) -> bool {
    super::noop::possibly_pure(c, f) && !f.parameters.iter().any(|p| p.flags.is_by_reference())
}

/// A declaration's entry: its kind (`f` function, `m` method, `c` constructor), key, display name and dependencies.
fn declare(c: &mut Checker, at: Span, f: &FunctionLikeMetadata, kind: &str, key: String, display: String, body: &[Statement]) {
    if c.level < 4 || !collectable(c, f) {
        return;
    }
    let Some(dependencies) = c.body_dependencies(body) else {
        return;
    };
    let message = format!("{ENTRY}{kind}\u{1f}{key}\u{1f}{display}\u{1f}{}", dependencies.join("\u{1d}"));
    c.report(at, "function.resultUnused", message, None);
}

pub(in super::super) fn function(c: &mut Checker, f: &Function) {
    let name = c.resolved_span(f.name.span());
    let Some(meta) = c.codebase.get_function(name.as_bytes()) else { return };
    let display = String::from_utf8_lossy(meta.original_name.as_bytes()).into_owned();
    declare(c, f.span(), meta, "f", function_key(&display), display, f.body.statements.as_slice());
}

pub(in super::super) fn method(c: &mut Checker, m: &Method) {
    let (Some(k), MethodBody::Concrete(body)) = (c.class_scope(), &m.body) else { return };
    if c.trait_context.is_some() || k.kind.is_trait() {
        return;
    }
    let lower = m.name.value.to_ascii_lowercase();
    let Some(meta) = c.codebase.get_method(k.name.as_bytes(), &lower) else { return };
    let class = String::from_utf8_lossy(k.original_name.as_bytes()).into_owned();
    let name = String::from_utf8_lossy(m.name.value).into_owned();
    let (kind, display) = if lower == b"__construct" {
        ("c", class.clone())
    } else {
        ("m", format!("{}::{name}", c.display(k)))
    };
    declare(c, m.span(), meta, kind, method_key(&class, &name), display, body.statements.as_slice());
}

/// The Possibly Pure{FuncCall,MethodCall,StaticCall,New}Collector entry of an expression statement.
pub(super) fn statement(c: &mut Checker, s: &ExpressionStatement) {
    if c.level < 4 {
        return;
    }
    let e = match unparen(s.expression) {
        // a pipe into a first-class callable calls it, into an arrow function evaluates its body
        Expression::Pipe(p) => match unparen(p.callable) {
            Expression::ArrowFunction(f) => unparen(f.expression),
            Expression::Call(_) => return,
            callable => callable,
        },
        Expression::PartialApplication(_) => return,
        e => e,
    };
    let Some((identifier, keys)) = c.possibly_pure_call(e) else { return };
    let message = format!("{ENTRY}call\u{1f}{identifier}\u{1f}{}\u{1f}", keys.join("\u{1d}"));
    c.report(s, identifier, message, None);
}

/// getPureCallableKeys() over every declaration, then the call statements to pure ones.
pub(crate) fn resolve(errors: Vec<Error>) -> Vec<Error> {
    let (entries, mut errors): (Vec<Error>, Vec<Error>) = errors.into_iter().partition(|e| e.message.starts_with(ENTRY));
    let mut declarations: BTreeMap<String, (String, String, Vec<String>)> = BTreeMap::new();
    let mut calls = Vec::new();
    for e in entries {
        let fields: Vec<String> = e.message[ENTRY.len()..].split('\u{1f}').map(str::to_string).collect();
        let [kind, key, display, dependencies] = <[String; 4]>::try_from(fields).expect("an entry has four fields");
        if kind == "call" {
            // (a call's identifier and keys)
            calls.push((key, display.split('\u{1d}').map(str::to_string).collect::<Vec<_>>(), e));
        } else {
            let dependencies = dependencies.split('\u{1d}').filter(|d| !d.is_empty()).map(str::to_string).collect();
            declarations.insert(key, (kind, display, dependencies));
        }
    }
    let mut pure = BTreeSet::new();
    loop {
        let found: Vec<String> = declarations
            .iter()
            .filter(|(key, (_, _, dependencies))| !pure.contains(*key) && dependencies.iter().all(|d| pure.contains(d)))
            .map(|(key, _)| key.clone())
            .collect();
        if found.is_empty() {
            break;
        }
        pure.extend(found);
    }
    for (identifier, keys, mut e) in calls {
        let kind = match identifier.as_str() {
            "function.resultUnused" => "f",
            "new.resultUnused" => "c",
            _ => "m",
        };
        let mut display = None;
        for key in &keys {
            display = declarations.get(key).filter(|(k, _, _)| k == kind && pure.contains(key)).map(|(_, d, _)| d);
            if display.is_none() {
                break;
            }
        }
        let Some(display) = display else { continue };
        e.message = match kind {
            "f" => format!("Call to function {display}() on a separate line has no effect."),
            "c" => format!("Call to new {display}() on a separate line has no effect."),
            _ if identifier == "method.resultUnused" => format!("Call to method {display}() on a separate line has no effect."),
            _ => format!("Call to {display}() on a separate line has no effect."),
        };
        errors.push(e);
    }
    errors
}

/// The receivers the body's method calls are resolved on.
pub(super) fn typed(program: &Program) -> Vec<Span> {
    struct Receivers(Vec<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Receivers {
        fn walk_in_method_call(&mut self, m: &'ast MethodCall<'arena>, _: &mut ()) {
            self.0.push(unparen(m.object).span());
        }
        fn walk_in_null_safe_method_call(&mut self, m: &'ast NullSafeMethodCall<'arena>, _: &mut ()) {
            self.0.push(unparen(m.object).span());
        }
    }
    let mut r = Receivers(Vec::new());
    r.walk_program(program, &mut ());
    r.0
}

/// NewHandler's `asFinal()`: the occurrences of a scope's local variables that are only ever assigned a `new` of a
/// named class. A closure inherits the ones its `use` list copies.
pub(super) fn exact_variables(program: &Program) -> HashSet<Span> {
    /// One function-like body: arrow functions share it, other function-likes and classes have their own.
    #[derive(Default)]
    struct Scope<'ast, 'arena> {
        assigned: BTreeMap<&'arena [u8], bool>,
        occurrences: Vec<(&'arena [u8], Span)>,
        closures: Vec<&'ast Closure<'arena>>,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Scope<'ast, 'arena> {
        fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
            let new = matches!(a.operator, AssignmentOperator::Assign(_))
                && matches!(unparen(a.rhs), Expression::Instantiation(Instantiation { class: Expression::Identifier(_), .. }));
            mark(&mut self.assigned, a.lhs, new);
        }
        fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
            for e in [f.target.key(), Some(f.target.value())].into_iter().flatten() {
                mark(&mut self.assigned, e, false);
            }
        }
        fn walk_in_global(&mut self, g: &'ast Global<'arena>, _: &mut ()) {
            for v in g.variables.iter() {
                if let Variable::Direct(v) = v {
                    self.assigned.insert(v.name, false);
                }
            }
        }
        fn walk_in_static_item(&mut self, i: &'ast StaticItem<'arena>, _: &mut ()) {
            self.assigned.insert(i.variable().name, false);
        }
        fn walk_in_direct_variable(&mut self, v: &'ast DirectVariable<'arena>, _: &mut ()) {
            self.occurrences.push((v.name, v.span));
        }
        fn walk_closure(&mut self, k: &'ast Closure<'arena>, _: &mut ()) {
            self.closures.push(k);
        }
        fn walk_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {}
        fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
        fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {}
    }
    fn mark<'arena>(assigned: &mut BTreeMap<&'arena [u8], bool>, e: &Expression<'arena>, new: bool) {
        match unparen(e) {
            Expression::Variable(Variable::Direct(v)) => {
                let entry = assigned.entry(v.name).or_insert(new);
                *entry &= new;
            }
            Expression::Array(a) => a.elements.iter().for_each(|e| element(assigned, e)),
            Expression::List(a) => a.elements.iter().for_each(|e| element(assigned, e)),
            _ => {}
        }
    }
    fn element<'arena>(assigned: &mut BTreeMap<&'arena [u8], bool>, e: &ArrayElement<'arena>) {
        match e {
            ArrayElement::KeyValue(kv) => mark(assigned, kv.value, false),
            ArrayElement::Value(v) => mark(assigned, v.value, false),
            _ => {}
        }
    }
    /// A scope's statements, with the parameters (never exact) and the variables it inherits as exact.
    fn scope<'ast, 'arena>(
        exact: &mut HashSet<Span>,
        parameters: Option<&'ast FunctionLikeParameterList<'arena>>,
        inherited: &[&'arena [u8]],
        statements: &'ast [Statement<'arena>],
    ) {
        let mut body = Scope::default();
        for name in inherited {
            body.assigned.insert(name, true);
        }
        for p in parameters.iter().flat_map(|l| l.parameters.iter()) {
            body.assigned.insert(p.variable.name, false);
        }
        for s in statements {
            body.walk_statement(s, &mut ());
        }
        let is_exact = |name: &[u8]| body.assigned.get(name) == Some(&true);
        exact.extend(body.occurrences.iter().filter(|(name, _)| is_exact(name)).map(|(_, span)| *span));
        for k in &body.closures {
            let inherited: Vec<&[u8]> = k
                .use_clause
                .iter()
                .flat_map(|u| u.variables.iter())
                .map(|u| u.variable.name)
                .filter(|name| is_exact(name))
                .collect();
            scope(exact, Some(&k.parameter_list), &inherited, k.body.statements.as_slice());
        }
    }
    struct Bodies(HashSet<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Bodies {
        fn walk_in_function(&mut self, f: &'ast Function<'arena>, _: &mut ()) {
            scope(&mut self.0, Some(&f.parameter_list), &[], f.body.statements.as_slice());
        }
        fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
            if let MethodBody::Concrete(b) = &m.body {
                scope(&mut self.0, Some(&m.parameter_list), &[], b.statements.as_slice());
            }
        }
    }
    let mut bodies = Bodies(HashSet::new());
    bodies.walk_program(program, &mut ());
    scope(&mut bodies.0, None, &[], program.statements.as_slice());
    bodies.0
}
