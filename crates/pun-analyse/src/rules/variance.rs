//! Level 2: VarianceCheck, for FunctionSignatureVarianceRule, MethodSignatureVarianceRule, PropertyVarianceRule
//! and GenericAncestorsCheck.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::symbols;
use pun_phpstan::types::{Subject, Template, Type};

use super::Checker;
use super::declared;
use super::doc::{Doc, doc_before};
use super::properties::PropertyNode;

pub const IDENTIFIERS: &[&str] = &["generics.variance", "function.variance", "method.variance"];

/// TemplateTypeVariance
#[derive(Clone, Copy, PartialEq)]
pub enum Variance {
    Invariant,
    Covariant,
    Contravariant,
    Bivariant,
    Static,
}

use Variance::*;

impl Variance {
    fn of(name: &str) -> Self {
        match name {
            "covariant" => Covariant,
            "contravariant" => Contravariant,
            "bivariant" => Bivariant,
            _ => Invariant,
        }
    }

    /// (a bivariant or static position is always valid, and no template is declared so, so neither is described)
    fn describe(self) -> &'static str {
        match self {
            Invariant => "invariant",
            Covariant => "covariant",
            _ => "contravariant",
        }
    }

    fn compose(self, other: Self) -> Self {
        match (self, other) {
            (Contravariant, Contravariant) | (Covariant, Covariant) => Covariant,
            (Contravariant, Covariant) | (Covariant, Contravariant) => Contravariant,
            (Contravariant | Covariant, Bivariant) => Bivariant,
            (Contravariant | Covariant, _) => Invariant,
            (Static, other) => other,
            (this, _) => this,
        }
    }

    fn valid_position(self, other: Self) -> bool {
        other == self || other == Invariant || matches!(self, Bivariant | Static)
    }
}

/// A template's declared variance: a class's from its reflection, a function's or method's from `variances`.
fn declared_variance(t: &Template) -> Variance {
    let Some(class) = t.scope.strip_prefix("class ") else {
        return Invariant;
    };
    symbols::lookup(class)
        .and_then(|k| k.templates.iter().find(|x| x.name == t.name).map(|x| Variance::of(x.variance)))
        .unwrap_or(Invariant)
}

/// `getReferencedTemplateTypes()`
fn referenced<'t>(ty: &'t Type, position: Variance, out: &mut Vec<(&'t Template, Variance)>) {
    let co = position.compose(Covariant);
    match ty {
        Type::Template(t) => out.push((t, position)),
        Type::Object(class, args) | Type::Static(class, args) => {
            // `key-of<T>` / `value-of<T>` of a template
            if class == "key-of" || class == "value-of" {
                return args.iter().for_each(|a| referenced(a, position, out));
            }
            let templates: Vec<Variance> = symbols::lookup(class)
                .map(|k| k.templates.iter().map(|t| Variance::of(t.variance)).collect())
                .unwrap_or_default();
            for (i, arg) in args.iter().enumerate() {
                let (effective, arg) = match arg {
                    Type::Projection(covariant, inner) => {
                        (if *covariant { Covariant } else { Contravariant }, &**inner)
                    }
                    Type::Wildcard => (Bivariant, arg),
                    _ => (templates.get(i).copied().unwrap_or(Invariant), arg),
                };
                referenced(arg, position.compose(effective), out);
            }
        }
        Type::Array(k, v) | Type::Iterable(k, v) => {
            referenced(k, co, out);
            referenced(v, co, out);
        }
        Type::ConstArray(a) => {
            a.keys.iter().chain(&a.values).for_each(|t| referenced(t, co, out));
            if let Some((k, v)) = &a.unsealed {
                referenced(k, co, out);
                referenced(v, co, out);
            }
        }
        Type::ClassString(Some(t)) => referenced(t, co, out),
        Type::ObjectShape(props) => props.iter().for_each(|(_, t, _)| referenced(t, co, out)),
        Type::Callable(Some(s)) | Type::Closure(s) => {
            referenced(&s.ret, co, out);
            let contra = position.compose(Contravariant);
            s.params.iter().for_each(|p| referenced(&p.ty, contra, out));
        }
        Type::Union(types) | Type::Benevolent(types) | Type::Intersection(types) => {
            types.iter().for_each(|t| referenced(t, position, out))
        }
        Type::Conditional(c) => {
            if let Subject::Type(s) = &c.subject {
                referenced(s, position, out);
            }
            for t in [&c.target, &c.then, &c.otherwise] {
                referenced(t, position, out);
            }
        }
        Type::OffsetAccess(t, o) => {
            referenced(t, position, out);
            referenced(o, position, out);
        }
        _ => {}
    }
}

fn function_scope(t: &Template) -> bool {
    t.scope.starts_with("function ") || t.scope.starts_with("method ")
}

/// VarianceCheck::check()
pub(super) fn check(position: Variance, ty: &Type, context: &str, out: &mut Vec<String>) {
    let mut refs = Vec::new();
    referenced(ty, position, &mut refs);
    for (t, at) in refs {
        let declared = declared_variance(t);
        if (function_scope(t) && declared != Invariant) || at.valid_position(declared) {
            continue;
        }
        out.push(format!(
            "Template type {} is declared as {}, but occurs in {} position {context}.",
            t.name,
            declared.describe(),
            at.describe()
        ));
    }
}

/// VarianceCheck::checkParametersAcceptor(): `subject` is `function f()` / `method C::m()`.
fn parameters_acceptor(
    c: &mut Checker,
    at: Span,
    kind: &'static str,
    subject: &str,
    signature: impl FnOnce(&Checker) -> declared::Signature,
    skip: bool,
) {
    let mut errors: Vec<(&'static str, String)> = Vec::new();
    if let Some(doc) = doc_before(&c.file.contents, at.start.offset as usize) {
        let doc = Doc::parse(doc);
        for (t, v) in doc.templates.iter().zip(&doc.template_variances) {
            if *v != "invariant" {
                errors.push((
                    if kind == "function" { "function.variance" } else { "method.variance" },
                    format!(
                        "Variance annotation is only allowed for type parameters of classes and interfaces, but occurs in template type {} in in {subject}.",
                        t.name
                    ),
                ));
            }
        }
    }
    if !skip {
        let mut found = Vec::new();
        declared::with_classes(c, || {
            let s = signature(c);
            for (name, ty, out_ty, _) in &s.params {
                check(Contravariant, ty, &format!("in parameter {name} of {subject}"), &mut found);
                if let Some(out_ty) = out_ty {
                    let context = format!("in param-out type of parameter {name} of {subject}");
                    check(Covariant, out_ty, &context, &mut found);
                }
            }
            check(Covariant, &s.ret, &format!("in return type of {subject}"), &mut found);
        });
        errors.extend(found.into_iter().map(|m| ("generics.variance", m)));
    }
    for (identifier, message) in errors {
        c.report(at, identifier, message, None);
    }
}

/// FunctionSignatureVarianceRule
pub fn function(c: &mut Checker, f: &Function) {
    if c.level < 2 {
        return;
    }
    let subject = format!("function {}()", c.resolved_span(f.name.span));
    parameters_acceptor(c, f.span(), "function", &subject, |c| declared::function(c, f), false);
}

/// MethodSignatureVarianceRule (`method` is the name the method is analysed under).
pub fn method(c: &mut Checker, m: &Method, method: &str) {
    if c.level < 2 {
        return;
    }
    let (k, display) = c.member_scope();
    let declaring = super::missing_types::declaring(c, m.span()).expect("a method is declared by its class or a used trait");
    let skip = m.modifiers.iter().any(super::declarations::is_private) || method.eq_ignore_ascii_case("__construct");
    let subject = format!("method {display}::{method}()");
    parameters_acceptor(c, m.span(), "method", &subject, |c| declared::method(c, m, declaring, k), skip);
}

/// `hasRestrictedWriteAccess()`: a read-only property (natively or by PHPDoc), a `private(set)` one, or one with
/// hooks but no `set` hook.
fn restricted(c: &Checker, modifiers: &[Modifier], hooks: &[PropertyHook], by_phpdoc: bool) -> bool {
    by_phpdoc
        || modifiers.iter().any(|m| matches!(m, Modifier::Readonly(_) | Modifier::PrivateSet(_)))
        || c.decls.last().is_some_and(|d| d.is_readonly)
        || (!hooks.is_empty() && !hooks.iter().any(|h| h.name.value == b"set"))
}

fn property_check(c: &mut Checker, at: Span, name: &[u8], position: Variance, ty: impl FnOnce(&Checker) -> Type) {
    let (_, display) = c.member_scope();
    let mut found = Vec::new();
    declared::with_classes(c, || {
        let context = format!("in property {display}::${}", String::from_utf8_lossy(name));
        check(position, &ty(c), &context, &mut found);
    });
    for message in found {
        c.report(at, "generics.variance", message, None);
    }
}

fn position(restricted: bool) -> Variance {
    if restricted { Covariant } else { Invariant }
}

/// PropertyVarianceRule of a declared property (`decl` is where its PHPDoc is read from).
pub fn property(c: &mut Checker, node: &PropertyNode, decl: Span, by_phpdoc: bool) {
    if c.level < 2 || node.has(super::declarations::is_private) {
        return;
    }
    let position = position(restricted(c, node.modifiers, node.hooks, by_phpdoc));
    let name = &node.name[1..];
    property_check(c, node.at, name, position, |c| {
        let (context, _) = c.member_scope();
        declared::property(c, node.hint, decl, name, context)
    });
}

/// PropertyVarianceRule of constructor `m`'s promoted parameters, typed as its signature types them.
pub fn promoted(c: &mut Checker, m: &Method) {
    if c.level < 2 {
        return;
    }
    let by_phpdoc = c.decls.last().is_some_and(|d| d.immutable_doc)
        || c.decls.last().and_then(|d| d.parent.clone()).is_some_and(|p| c.class_immutable(&p, 0));
    let (k, _) = c.member_scope();
    let declaring = super::missing_types::declaring(c, m.span()).expect("a method is declared by its class or a used trait");
    for (i, p) in m.parameter_list.parameters.iter().enumerate() {
        let hooks = p.hooks.as_ref().map_or(&[][..], |h| h.hooks.as_slice());
        if (p.modifiers.is_empty() && hooks.is_empty()) || p.modifiers.iter().any(super::declarations::is_private) {
            continue;
        }
        let position = position(restricted(c, p.modifiers.as_slice(), hooks, by_phpdoc));
        property_check(c, p.span(), &p.variable.name[1..], position, |c| {
            declared::method(c, m, declaring, k).params.swap_remove(i).1
        });
    }
}
