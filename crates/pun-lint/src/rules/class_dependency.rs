//! Constructor injection as Rector 2.8.4 writes it, shared by `lint/refactor/replace-static-with-method`
//! (`StaticCallToMethodCallRector`) and `lint/laravel/prefer-constructor-injection` (rector-laravel's
//! `ArgumentFuncCallToMethodCallRector`). Both visit a class (named or anonymous), walk each of its methods in order
//! ([`inject`]) and ask [`Injector::provide`] (`FuncCallStaticCallToMethodCallAnalyzer::matchTypeProvidingExpr`) for
//! an expression of the configured class to call the new method on:
//!
//! - `TypeProvidingExprFromClassResolver`: the first method, then the first property, whose type is the class (its
//!   PHPDoc type read, `null` aside), in the order PHPStan's native reflection (BetterReflection) lists them: the
//!   class's own methods in declaration order, then its parent's, its traits', its interfaces'; the parent's
//!   non-private properties, then the interfaces', the traits', and the class's own (declared, then promoted), an own
//!   one taking the place of an inherited one of the same name. In the constructor, else, a new parameter.
//! - `ClassDependencyManipulator::addConstructorDependency` (with `ClassInsertManipulator`,
//!   `ClassMethodAssignManipulator`, `PropertyPresenceChecker` and `AutowiredClassMethodOrPropertyAnalyzer`): a
//!   promoted property (PHP 8.0+, `readonly` from 8.1) added to the class's constructor, to a new one, or to a copy of
//!   the nearest parent constructor that calls it (`parent::__construct(...)`); below PHP 8.0, a private property
//!   (typed from 7.4, else with a `@var` tag) and a parameter assigned to it; in a public `@required` / `@inject`
//!   (Symfony, Nette) method if the class has one. A new member goes before the first method, or after the first run
//!   of properties.
//!
//! The code is printed as php-parser's format-preserving printer writes it: a new member on its own line at the
//! indentation of the member next to it; a new parameter after the last one (`, ` or, in a list with a line break
//! between every two parameters, `,` and a line break); a new statement on its own line after the last one; a method
//! whose body was empty printed anew; the copy of a parent constructor printed anew from its own file (names fully
//! qualified, literals as php-parser writes them); a rewritten call with its arguments joined by `, `, or one a line
//! when one of them has comments.
//!
//! Pun leaves alone what Rector breaks (each a `Pun/` fixture): a call in a nested class, a static closure or a
//! function (Rector writes `$this` where it means another object or none); a new constructor parameter used in a
//! closure that does not capture it; a parameter name the constructor has already, or a dependency the constructor
//! needs both as a parameter and as a property (Rector declares the parameter twice or reads a property it never
//! declares); a parent's private method and a static property (not readable through `$this->`); a constructor that
//! comes from a trait or an interface, or an abstract or variadic parent constructor (Rector calls
//! `parent::__construct()` where it does not exist, or adds a parameter PHP rejects); an abstract constructor or
//! `@required` method. A class among its own parents is left alone too, as Rector stops there with an error.

use std::rc::Rc;
use std::sync::Arc;

use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_codex::ttype::union::TUnion;
use mago_codex::visibility::Visibility;
use mago_names::ResolvedNames;
use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::class_method::{is_magic, qualified, walk};
use super::laravel::property_attribute::ClassView;
use super::no_redundant_readonly_property::print_modifiers;
use super::php_parser::{Token, block_list, previous_token_end, tokens};
use super::prefer_absolute_include::print_string;
use super::prefer_catch_name_from_type::fqn_to_variable_name;
use super::rector_batch5::{doc_comment, has_attribute, member_from, merge, resolve_class_name};
use super::rector_conditions::{comments_between, indentation, print_comments, reprint};
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report};

/// `PhpVersionFeature::PROPERTY_PROMOTION`, `READONLY_PROPERTY` and `TYPED_PROPERTIES`.
const PROMOTION: u32 = 80000;
const READONLY: u32 = 80100;
const TYPED_PROPERTIES: u32 = 70400;

/// Where a call stands in its method: whether `$this` is there (not in a static closure or a function), and whether
/// the method's variables are (not in a closure, which captures only what its `use` names).
#[derive(Clone, Copy)]
pub(crate) struct Scope {
    pub this: bool,
    pub captured: bool,
    /// Whether the node is in a class declared inside the method, where Rector's `$this` means the other class.
    pub nested: bool,
}

/// The arguments of a call written anew: the spans of its arguments, or a first-class callable's `...`.
pub(crate) struct Args {
    pub items: Vec<Span>,
    pub placeholder: bool,
}

/// What a rule makes of a node of a method: nothing, the call at `span` written anew as `head` and its arguments,
/// or no change to the class at all (where Rector stops with an error).
pub(crate) enum Outcome {
    Keep,
    Rewrite {
        span: Span,
        head: String,
        args: Option<Args>,
    },
    Fail,
}

/// The arguments of `list` (a call's).
pub(crate) fn call_args(list: &ArgumentList<'_>) -> Args {
    Args {
        items: list.arguments.iter().map(|a| node_span(a)).collect(),
        placeholder: false,
    }
}

/// Whether a first-class callable's arguments are `(...)`.
pub(crate) fn is_first_class(list: &PartialArgumentList<'_>) -> bool {
    matches!(
        list.arguments.iter().collect::<Vec<_>>()[..],
        [PartialArgument::VariadicPlaceholder(_)]
    )
}

/// The first-class callable's `(...)`.
pub(crate) fn placeholder() -> Args {
    Args {
        items: Vec::new(),
        placeholder: true,
    }
}

/// A rule's `refactor()` on the class `node`: each method `visit_method` accepts walked in order (nested classes
/// left out), `call` asked about each node, and the class's changes reported as one fix.
pub(crate) fn inject<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    ancestors: &[Node<'ast, 'arena>],
    ctx: &Context<'_>,
    message: &str,
    visit_method: &dyn Fn(&Method<'_>) -> bool,
    call: &dyn Fn(
        &mut Injector<'_, 'ast, 'arena>,
        Node<'ast, 'arena>,
        &[Node<'ast, 'arena>],
        &'ast Method<'arena>,
        Scope,
    ) -> Outcome,
    out: &mut Vec<Report>,
) {
    let Some(class) = ClassView::of(node) else {
        return;
    };
    if in_visited_method(ancestors, visit_method) {
        return;
    }
    let mut injector = Injector::new(ctx, &class, ancestors);
    let mut rewrites: Vec<(Span, String, Option<Args>)> = Vec::new();
    let mut failed = false;
    for member in class.members.iter() {
        let ClassLikeMember::Method(method) = member else {
            continue;
        };
        if !visit_method(method) {
            continue;
        }
        walk(Node::Method(method), &mut |n, path| {
            if failed {
                return false;
            }
            let mut full = ancestors.to_vec();
            full.push(node);
            full.extend_from_slice(path);
            match call(&mut injector, n, &full, method, scope_of(path)) {
                Outcome::Keep => {}
                Outcome::Rewrite { span, head, args } => rewrites.push((span, head, args)),
                Outcome::Fail => failed = true,
            }
            true
        });
    }
    if failed || injector.failed || rewrites.is_empty() {
        return;
    }
    let all = tokens(ctx);
    let mut edits = rewritten(ctx.source, &all, &rewrites);
    edits.extend(injector.finish(&all));
    out.push(Report {
        span: node_span(&class.node),
        message: message.to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits: merge(edits),
        }),
    });
}

/// The scope of a node below a method, `path` the nodes from the method down.
fn scope_of(path: &[Node<'_, '_>]) -> Scope {
    let mut scope = Scope {
        this: true,
        captured: true,
        nested: false,
    };
    for node in path {
        match node {
            n if is_class_like(n) => scope.nested = true,
            Node::Closure(c) => {
                scope.captured = false;
                scope.this &= c.r#static.is_none();
            }
            Node::ArrowFunction(a) => scope.this &= a.r#static.is_none(),
            Node::Function(_) => {
                scope.captured = false;
                scope.this = false;
            }
            _ => {}
        }
    }
    scope
}

fn is_class_like(node: &Node<'_, '_>) -> bool {
    matches!(
        node,
        Node::Class(_)
            | Node::AnonymousClass(_)
            | Node::Interface(_)
            | Node::Trait(_)
            | Node::Enum(_)
    )
}

/// Whether the class sits in a method of a class (named or anonymous) that the rule walks, Rector having rewritten
/// its calls from the outer class already.
fn in_visited_method(
    ancestors: &[Node<'_, '_>],
    visit_method: &dyn Fn(&Method<'_>) -> bool,
) -> bool {
    let Some((at, method)) = (ancestors.iter().enumerate().rev()).find_map(|(i, n)| match n {
        Node::Method(m) => Some((i, *m)),
        _ => None,
    }) else {
        return false;
    };
    let outer = ancestors[..at].iter().rev().find(|n| is_class_like(n));
    matches!(outer, Some(Node::Class(_) | Node::AnonymousClass(_))) && visit_method(method)
}

/// The edits of the rewritten calls: each printed with the rewrites inside its arguments, the outermost kept.
fn rewritten(source: &str, all: &[Token], rewrites: &[(Span, String, Option<Args>)]) -> Vec<Edit> {
    let mut done: Vec<Edit> = Vec::new();
    for (span, head, args) in rewrites.iter().rev() {
        let text = match args {
            None => head.clone(),
            Some(args) => format!(
                "{head}({})",
                print_args(source, all, args, indentation(source, span.start), &done)
            ),
        };
        done.push(Edit::new(*span, text));
    }
    outermost(&done, Span::new(0, source.len()))
}

/// The edits within `span` that no other edit within it holds.
fn outermost(edits: &[Edit], span: Span) -> Vec<Edit> {
    let within = |s: Span, outer: Span| outer.start <= s.start && s.end <= outer.end;
    let inside: Vec<&Edit> = edits.iter().filter(|e| within(e.span, span)).collect();
    inside
        .iter()
        .filter(|e| {
            !inside
                .iter()
                .any(|o| o.span != e.span && within(e.span, o.span))
        })
        .map(|e| (*e).clone())
        .collect()
}

/// The text at `span` with the edits within it applied.
fn edited(source: &str, span: Span, edits: &[Edit]) -> String {
    let mut inner = outermost(edits, span);
    inner.sort_by_key(|e| e.span.start);
    let mut out = String::new();
    let mut pos = span.start;
    for e in inner {
        out.push_str(&source[pos..e.span.start]);
        out.push_str(&e.text);
        pos = e.span.end;
    }
    out.push_str(&source[pos..span.end]);
    out
}

/// `pMaybeMultiline` over the arguments: joined by `, `, or each on a line of its own after its comments, one level
/// deeper than `level`, when one of them has comments.
fn print_args(source: &str, all: &[Token], args: &Args, level: usize, done: &[Edit]) -> String {
    if args.placeholder {
        return "...".to_string();
    }
    let printed: Vec<(Vec<Span>, String)> = (args.items.iter())
        .map(|s| {
            let before = previous_token_end(source, all, s.start);
            (
                comments_between(all, before, s.start),
                edited(source, *s, done),
            )
        })
        .collect();
    if printed.iter().all(|(c, _)| c.is_empty()) {
        let texts: Vec<&str> = printed.iter().map(|(_, t)| t.as_str()).collect();
        return texts.join(", ");
    }
    let inner = format!("\n{}", " ".repeat(level + 4));
    let lines: Vec<String> = printed
        .iter()
        .map(|(comments, text)| {
            let mut line = String::new();
            for c in comments {
                line.push_str(&inner);
                line.push_str(&source[c.start..c.end]);
            }
            line.push_str(&inner);
            line.push_str(text);
            line
        })
        .collect();
    format!("{}\n{}", lines.join(","), " ".repeat(level))
}

/// A parameter the injection adds: its name, its code and whether it is a promoted property.
struct Added {
    name: String,
    text: String,
    promoted: bool,
}

/// The class's constructor as the injection changes it.
enum Ctor {
    /// The class declares one.
    Own,
    /// None yet.
    Missing,
    /// A new one, empty.
    Fresh,
    /// A new one, a copy of the parent's.
    FromParent(Rc<ParentCtor>),
}

/// The nearest parent constructor (`ClassDependencyManipulator::resolveConstruct` / `hasFinalParentConstructor`).
#[derive(Clone)]
enum Nearest {
    None,
    Final,
    /// One Pun does not follow Rector to.
    Fail,
    Parent(Rc<ParentCtor>),
}

/// A member of a class as reflection lists it: its name, the class-like that declares it, and whether it came from
/// the parent.
#[derive(Clone)]
struct Member {
    name: String,
    declaring: String,
    from_parent: bool,
    private: bool,
    is_static: bool,
}

/// The parts of a class-like declaration reflection reads: its parent, interfaces and traits, and its own methods and
/// properties (declared, then promoted) in order.
#[derive(Default)]
struct Shape {
    parent: Option<String>,
    interfaces: Vec<String>,
    traits: Vec<String>,
    methods: Vec<String>,
    properties: Vec<Member>,
}

/// A property the injection adds: its name and whether it has a `@var` tag instead of a type.
struct NewProperty {
    name: String,
    class: String,
    doc: bool,
}

/// `ClassDependencyManipulator` and `TypeProvidingExprFromClassResolver` on one class.
pub(crate) struct Injector<'c, 'ast, 'arena> {
    ctx: &'c Context<'c>,
    facts: Rc<Facts>,
    class: &'c ClassView<'ast, 'arena>,
    ancestors: &'c [Node<'ast, 'arena>],
    name: String,
    own: Option<&'ast Method<'arena>>,
    autowire: Option<&'ast Method<'arena>>,
    methods: Option<Vec<Member>>,
    properties: Option<Vec<Member>>,
    nearest: Option<Nearest>,
    ctor: Ctor,
    ctor_params: Vec<Added>,
    ctor_stmts: Vec<String>,
    autowire_params: Vec<Added>,
    autowire_stmts: Vec<String>,
    new_properties: Vec<NewProperty>,
    /// The parameters added to the constructor for calls in it.
    ctor_vars: Vec<String>,
    /// The properties calls read.
    deps: Vec<String>,
    failed: bool,
}

impl<'c, 'ast, 'arena> Injector<'c, 'ast, 'arena> {
    fn new(
        ctx: &'c Context<'c>,
        class: &'c ClassView<'ast, 'arena>,
        ancestors: &'c [Node<'ast, 'arena>],
    ) -> Self {
        let own = class.method("__construct").map(|(_, m)| m);
        let mut injector = Injector {
            ctx,
            facts: facts(ctx),
            class,
            ancestors,
            name: class.name(ctx, ancestors),
            own,
            autowire: None,
            methods: None,
            properties: None,
            nearest: None,
            ctor: if own.is_some() {
                Ctor::Own
            } else {
                Ctor::Missing
            },
            ctor_params: Vec::new(),
            ctor_stmts: Vec::new(),
            autowire_params: Vec::new(),
            autowire_stmts: Vec::new(),
            new_properties: Vec::new(),
            ctor_vars: Vec::new(),
            deps: Vec::new(),
            failed: false,
        };
        injector.autowire = injector.autowired_method();
        injector
    }

    /// `matchTypeProvidingExpr` for the class `class` in `method`: the code of the expression to call on, or `None`
    /// where the call stays.
    pub(crate) fn provide(
        &mut self,
        method: &Method<'_>,
        class: &str,
        scope: Scope,
    ) -> Option<String> {
        // Pun: Rector writes `$this` of the outer class in the nested one.
        if scope.nested {
            self.failed = true;
            return None;
        }
        // A. a method, B. a property
        if let Some(found) = self.method_providing(class) {
            return scope.this.then(|| format!("$this->{found}()"));
        }
        if let Some(found) = self.property_providing(class) {
            return scope.this.then(|| format!("$this->{found}"));
        }
        if self.failed {
            return None;
        }
        let name = fqn_to_variable_name(class);
        // C. a parameter of the constructor
        if method.name.value.eq_ignore_ascii_case(b"__construct") {
            if !scope.captured {
                return None;
            }
            if !self.ctor_vars.contains(&name) {
                if self.deps.contains(&name) {
                    self.failed = true;
                    return None;
                }
                if self.param_names().iter().any(|p| *p == name) {
                    return None;
                }
                self.ctor_params.push(Added {
                    text: format!("\\{class} ${name}"),
                    name: name.clone(),
                    promoted: false,
                });
                self.ctor_vars.push(name.clone());
            }
            return Some(format!("${name}"));
        }
        if !scope.this || matches!(self.nearest(), Nearest::Final) {
            return None;
        }
        if self.ctor_vars.contains(&name) {
            self.failed = true;
            return None;
        }
        self.add_constructor_dependency(&name, class);
        self.deps.push(name.clone());
        Some(format!("$this->{name}"))
    }

    // TypeProvidingExprFromClassResolver

    fn method_providing(&mut self, class: &str) -> Option<String> {
        if self.methods.is_none() {
            let methods = methods_of(
                &self.facts,
                self.class,
                self.ancestors,
                &self.name,
                &self.name,
                &mut Vec::new(),
            );
            self.failed |= methods.is_none();
            self.methods = Some(methods.unwrap_or_default());
        }
        let methods = self.methods.as_ref().unwrap();
        let documented = self.facts.documented();
        let (found, private) = methods.iter().find_map(|m| {
            let meta = documented.get_method(m.declaring.as_bytes(), m.name.as_bytes())?;
            let t = meta.return_type_metadata.as_ref()?;
            let private = (meta.method_metadata.as_ref())
                .is_some_and(|mm| mm.visibility == Visibility::Private);
            is_matching_type(&t.type_union, class)
                .then(|| (m.name.clone(), m.from_parent && private))
        })?;
        // Pun: a parent's private method is not callable here.
        self.failed |= private;
        (!private).then_some(found)
    }

    fn property_providing(&mut self, class: &str) -> Option<String> {
        if self.properties.is_none() {
            let properties = properties_of(
                &self.facts,
                self.class,
                self.ancestors,
                &self.name,
                &self.name,
                &mut Vec::new(),
            );
            self.failed |= properties.is_none();
            self.properties = Some(properties.unwrap_or_default());
        }
        let properties = self.properties.as_ref().unwrap();
        let documented = self.facts.documented();
        let found = properties.iter().find(|p| {
            documented
                .get_property(p.declaring.as_bytes(), format!("${}", p.name).as_bytes())
                .and_then(|meta| meta.type_metadata.as_ref())
                .is_some_and(|t| is_matching_type(&t.type_union, class))
        })?;
        // Pun: a static property is not readable through `$this->`.
        self.failed |= found.is_static;
        (!found.is_static).then(|| found.name.clone())
    }

    // ClassDependencyManipulator

    fn add_constructor_dependency(&mut self, name: &str, class: &str) {
        // hasClassPropertyAndDependency
        if let Some(autowired) = self.class_context_property(name) {
            match autowired {
                Some(true) => return,
                None if self.param_names().iter().any(|p| p == name) => return,
                _ => {}
            }
        }
        let autowire = self.autowire;
        if self.ctx.php_version < PROMOTION || autowire.is_some() {
            self.add_property_to_class(name, class);
        }
        if let Some(method) = autowire {
            // Pun: an abstract `@required` method takes no statement.
            if !matches!(method.body, MethodBody::Concrete(_)) {
                self.failed = true;
                return;
            }
            let existing = params_of(method);
            add_parameter_and_assign(
                &mut self.autowire_params,
                &mut self.autowire_stmts,
                &existing,
                name,
                class,
                &mut self.failed,
            );
            return;
        }
        if self.ctx.php_version >= PROMOTION && !self.has_property(name) {
            self.add_promoted_property(name, class);
            return;
        }
        self.add_with_custom_assign(name, class);
    }

    /// `PropertyPresenceChecker::getClassContextProperty`: the class's property or promoted constructor parameter
    /// named `name`, `Some(true)` when it is a property autowiring fills (`detect()`), `Some(false)` for a promoted
    /// parameter it marks, `None` for any other.
    fn class_context_property(&self, name: &str) -> Option<Option<bool>> {
        for (index, member) in self.class.members.iter().enumerate() {
            let (lists, items): (_, Vec<&PropertyItem<'_>>) = match member {
                ClassLikeMember::Property(Property::Plain(p)) => {
                    (&p.attribute_lists, p.items.iter().collect())
                }
                ClassLikeMember::Property(Property::Hooked(p)) => {
                    (&p.attribute_lists, vec![&p.item])
                }
                _ => continue,
            };
            if items
                .iter()
                .any(|item| &item.variable().name[1..] == name.as_bytes())
            {
                return Some(self.is_autowired_member(index, lists).then_some(true));
            }
        }
        if self.new_properties.iter().any(|p| p.name == name) {
            return Some(None);
        }
        if let Some(own) = self.own {
            for p in own.parameter_list.parameters.iter() {
                if !p.modifiers.is_empty() && &p.variable.name[1..] == name.as_bytes() {
                    return Some(
                        self.is_autowired_attributes(&p.attribute_lists)
                            .then_some(false),
                    );
                }
            }
        }
        self.ctor_params
            .iter()
            .any(|a| a.promoted && a.name == name)
            .then_some(None)
    }

    /// `Class_::getProperty()` of the class as changed.
    fn has_property(&self, name: &str) -> bool {
        self.class.property(name).is_some() || self.new_properties.iter().any(|p| p.name == name)
    }

    /// The names of the parameters of the class's constructor as changed (`isParamInConstructor`).
    fn param_names(&self) -> Vec<String> {
        let mut names: Vec<String> = match &self.ctor {
            Ctor::Own => self
                .own
                .map(params_of)
                .unwrap_or_default()
                .into_iter()
                .map(|(n, _)| n)
                .collect(),
            Ctor::Missing | Ctor::Fresh => Vec::new(),
            Ctor::FromParent(p) => p.params.iter().map(|p| p.name.clone()).collect(),
        };
        names.extend(self.ctor_params.iter().map(|a| a.name.clone()));
        names
    }

    fn add_property_to_class(&mut self, name: &str, class: &str) {
        if self.has_property(name) {
            return;
        }
        self.new_properties.push(NewProperty {
            name: name.to_string(),
            class: class.to_string(),
            doc: self.ctx.php_version < TYPED_PROPERTIES,
        });
    }

    /// `addPromotedProperty`.
    fn add_promoted_property(&mut self, name: &str, class: &str) {
        let readonly = if self.ctx.php_version >= READONLY {
            "readonly "
        } else {
            ""
        };
        let param = Added {
            name: name.to_string(),
            text: format!("private {readonly}\\{class} ${name}"),
            promoted: true,
        };
        if let Ctor::Missing = self.ctor {
            match self.nearest().clone() {
                Nearest::Fail => self.failed = true,
                Nearest::None | Nearest::Final => {
                    self.ctor = Ctor::Fresh;
                    self.ctor_params.push(param);
                }
                Nearest::Parent(parent) => {
                    match parent.params.iter().find(|p| p.name == name) {
                        // A private promoted property of the parent: a new constructor on the class.
                        Some(p) if p.promoted == Some(true) => {
                            self.ctor = Ctor::Fresh;
                            self.ctor_params.push(param);
                        }
                        Some(p) if p.promoted.is_some() => {}
                        // Pun: a parameter the parent does not keep as a property.
                        Some(_) => self.failed = true,
                        None => {
                            self.ctor = Ctor::FromParent(parent);
                            self.ctor_params.push(param);
                        }
                    }
                }
            }
            return;
        }
        if !self.ctor_has_body() {
            self.failed = true;
            return;
        }
        match self.ctor_param(name) {
            // Pun: a parameter of that name that is no property.
            Some(false) => self.failed = true,
            Some(true) => {}
            None => self.ctor_params.push(param),
        }
    }

    /// Whether the constructor parameter `name` exists, and then whether it is a promoted property.
    fn ctor_param(&self, name: &str) -> Option<bool> {
        let found = match &self.ctor {
            Ctor::Own => self
                .own
                .map(params_of)
                .unwrap_or_default()
                .into_iter()
                .find(|(n, _)| n == name)
                .map(|(_, p)| p),
            Ctor::FromParent(p) => p
                .params
                .iter()
                .find(|p| p.name == name)
                .map(|p| p.promoted.is_some()),
            Ctor::Missing | Ctor::Fresh => None,
        };
        found.or_else(|| {
            self.ctor_params
                .iter()
                .find(|a| a.name == name)
                .map(|a| a.promoted)
        })
    }

    /// Whether the class's own constructor has a body to add to.
    fn ctor_has_body(&self) -> bool {
        !matches!(self.ctor, Ctor::Own)
            || self
                .own
                .is_some_and(|m| matches!(m.body, MethodBody::Concrete(_)))
    }

    /// `addConstructorDependencyWithCustomAssign` with `$this->name = $name`.
    fn add_with_custom_assign(&mut self, name: &str, class: &str) {
        if let Ctor::Missing = self.ctor {
            match self.nearest().clone() {
                Nearest::Fail => {
                    self.failed = true;
                    return;
                }
                Nearest::None | Nearest::Final => self.ctor = Ctor::Fresh,
                Nearest::Parent(parent) => self.ctor = Ctor::FromParent(parent),
            }
        }
        if !self.ctor_has_body() {
            self.failed = true;
            return;
        }
        let existing: Vec<(String, bool)> = match &self.ctor {
            Ctor::Own => self.own.map(params_of).unwrap_or_default(),
            Ctor::FromParent(p) => p
                .params
                .iter()
                .map(|p| (p.name.clone(), p.promoted.is_some()))
                .collect(),
            _ => Vec::new(),
        };
        add_parameter_and_assign(
            &mut self.ctor_params,
            &mut self.ctor_stmts,
            &existing,
            name,
            class,
            &mut self.failed,
        );
    }

    /// The nearest parent constructor, worked out once.
    fn nearest(&mut self) -> &Nearest {
        if self.nearest.is_none() {
            let nearest = if self.own.is_some() {
                Nearest::None
            } else {
                nearest_parent(&self.facts, &self.name)
            };
            self.nearest = Some(nearest);
        }
        self.nearest.as_ref().unwrap()
    }

    // AutowiredClassMethodOrPropertyAnalyzer

    /// `matchAutowiredMethodInClass`: the first public method, not a magic one, that autowiring calls.
    fn autowired_method(&self) -> Option<&'ast Method<'arena>> {
        self.class.members.iter().enumerate().find_map(|(i, m)| {
            let ClassLikeMember::Method(method) = m else {
                return None;
            };
            let public = !(method.modifiers.iter())
                .any(|m| matches!(m, Modifier::Private(_) | Modifier::Protected(_)));
            (public
                && !is_magic(method.name.value)
                && self.is_autowired_member(i, &method.attribute_lists))
            .then_some(method)
        })
    }

    /// `detect()` on the member at `index`, whose attributes are `lists`: an `@required` or `@inject` tag in its doc
    /// comment, or Symfony's `Required` or Nette's `Inject` attribute.
    fn is_autowired_member(&self, index: usize, lists: &Sequence<'_, AttributeList<'_>>) -> bool {
        let member = self.class.members.get(index).unwrap();
        let from = member_from(self.class.members, self.class.left_brace, index);
        let tagged =
            doc_comment(self.ctx.source, from, node_span(member).start).is_some_and(|doc| {
                let text = self.ctx.source[doc.start..doc.end].to_ascii_lowercase();
                ["@required", "@inject"].iter().any(|tag| {
                    text.match_indices(tag).any(|(at, _)| {
                        !text[at + tag.len()..]
                            .starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
                    })
                })
            });
        tagged || self.is_autowired_attributes(lists)
    }

    fn is_autowired_attributes(&self, lists: &Sequence<'_, AttributeList<'_>>) -> bool {
        has_attribute(
            self.ancestors,
            lists,
            "Symfony\\Contracts\\Service\\Attribute\\Required",
        ) || has_attribute(self.ancestors, lists, "Nette\\DI\\Attributes\\Inject")
    }

    // Printing

    /// The edits for the members, parameters and statements the injection added.
    fn finish(&self, all: &[Token]) -> Vec<Edit> {
        let source = self.ctx.source;
        let mut edits = Vec::new();
        let new_ctor = matches!(self.ctor, Ctor::Fresh | Ctor::FromParent(_));
        if !self.new_properties.is_empty() || new_ctor {
            edits.push(self.insert_members(all, new_ctor));
        }
        if let (Ctor::Own, Some(own)) = (&self.ctor, self.own)
            && let MethodBody::Concrete(block) = &own.body
        {
            edits.extend(method_edits(
                source,
                all,
                own,
                block,
                &self.ctor_params,
                &self.ctor_stmts,
            ));
        }
        if let Some(method) = self.autowire
            && let MethodBody::Concrete(block) = &method.body
        {
            edits.extend(method_edits(
                source,
                all,
                method,
                block,
                &self.autowire_params,
                &self.autowire_stmts,
            ));
        }
        edits
    }

    /// `addAsFirstMethod` for the new properties, then the new constructor: before the first method, or after the
    /// first run of properties.
    fn insert_members(&self, all: &[Token], new_ctor: bool) -> Edit {
        let source = self.ctx.source;
        let members: Vec<&ClassLikeMember<'_>> = self.class.members.iter().collect();
        let mut after = None;
        for (i, m) in members.iter().enumerate() {
            match m {
                ClassLikeMember::Method(_) => {
                    after = i.checked_sub(1);
                    break;
                }
                ClassLikeMember::Property(_)
                    if !matches!(members.get(i + 1), Some(ClassLikeMember::Property(_))) =>
                {
                    after = Some(i);
                    break;
                }
                _ => {}
            }
        }
        let anchor = after.unwrap_or(0);
        let level = indentation(source, node_span(members[anchor]).start);
        let nl = format!("\n{}", " ".repeat(level));
        let mut printed: Vec<(Option<String>, String)> = (self.new_properties.iter())
            .map(|p| print_property(p, &nl))
            .collect();
        if new_ctor {
            printed.push(self.print_new_ctor(level));
        }
        let mut text = String::new();
        match after {
            Some(i) => {
                for (comments, node) in printed {
                    text.push_str(&nl);
                    if let Some(c) = comments {
                        text.push_str(&c);
                        text.push_str(&nl);
                    }
                    text.push_str(&node);
                }
                let at = node_span(members[i]).end;
                Edit::new(Span::new(at, at), text)
            }
            None => {
                for (comments, node) in printed {
                    if let Some(c) = comments {
                        text.push_str(&c);
                        text.push_str(&nl);
                    }
                    text.push_str(&node);
                    text.push_str(&nl);
                }
                let start = node_span(members[0]).start;
                let from = self.class.left_brace.end.offset as usize;
                let at = comments_between(all, from, start)
                    .first()
                    .map_or(start, |c| c.start);
                Edit::new(Span::new(at, at), text)
            }
        }
    }

    /// The new constructor at the indent `level`, and its comments.
    fn print_new_ctor(&self, level: usize) -> (Option<String>, String) {
        let nl = format!("\n{}", " ".repeat(level));
        let nl4 = format!("\n{}", " ".repeat(level + 4));
        let added: Vec<&str> = self.ctor_params.iter().map(|a| a.text.as_str()).collect();
        let mut stmts: Vec<String> = Vec::new();
        let (comments, head, params) = match &self.ctor {
            Ctor::FromParent(parent) => {
                let args: Vec<String> = parent
                    .params
                    .iter()
                    .map(|p| format!("${}", p.name))
                    .collect();
                stmts.push(format!("parent::__construct({});", args.join(", ")));
                let comments = (!parent.comments.is_empty())
                    .then(|| print_comments(&parent.source, &parent.comments, &nl));
                let mut head: String = parent
                    .attributes
                    .iter()
                    .map(|a| format!("{a}{nl}"))
                    .collect();
                head.push_str(&parent.modifiers);
                let multiline = (parent.params.iter())
                    .any(|p| !p.comments.is_empty() || !p.attributes.is_empty());
                let params = if multiline {
                    let mut items: Vec<String> = parent
                        .params
                        .iter()
                        .map(|p| {
                            let mut item = String::new();
                            if !p.comments.is_empty() {
                                item.push_str(&nl4);
                                item.push_str(&print_comments(&parent.source, &p.comments, &nl4));
                            }
                            item.push_str(&nl4);
                            for a in &p.attributes {
                                item.push_str(a);
                                item.push_str(&nl4);
                            }
                            item.push_str(&p.text);
                            item
                        })
                        .collect();
                    items.extend(added.iter().map(|a| format!("{nl4}{a}")));
                    format!("{}{nl}", items.join(","))
                } else {
                    let mut all: Vec<&str> =
                        parent.params.iter().map(|p| p.text.as_str()).collect();
                    all.extend(added);
                    all.join(", ")
                };
                (comments, head, params)
            }
            _ => (None, "public ".to_string(), added.join(", ")),
        };
        stmts.extend(self.ctor_stmts.iter().cloned());
        let body: String = stmts.iter().map(|s| format!("{nl4}{s}")).collect();
        (
            comments,
            format!("{head}function __construct({params}){nl}{{{body}{nl}}}"),
        )
    }
}

/// `ClassMethodAssignManipulator::addParameterAndAssignToMethod`: the parameter and `$this->name = $name;`, unless
/// the method has the parameter. Pun: one it had before the injection is not the dependency.
fn add_parameter_and_assign(
    params: &mut Vec<Added>,
    stmts: &mut Vec<String>,
    existing: &[(String, bool)],
    name: &str,
    class: &str,
    failed: &mut bool,
) {
    if existing.iter().any(|(n, _)| n == name) {
        *failed = true;
        return;
    }
    if params.iter().any(|a| a.name == name) {
        return;
    }
    params.push(Added {
        name: name.to_string(),
        text: format!("\\{class} ${name}"),
        promoted: false,
    });
    stmts.push(format!("$this->{name} = ${name};"));
}

/// The parameters of `method`: each name and whether it is a promoted property.
fn params_of(method: &Method<'_>) -> Vec<(String, bool)> {
    (method.parameter_list.parameters.iter())
        .map(|p| {
            (
                String::from_utf8_lossy(&p.variable.name[1..]).into_owned(),
                !p.modifiers.is_empty() || p.hooks.is_some(),
            )
        })
        .collect()
}

/// `isMatchingType`: not `mixed`, and without `null` exactly the object type of `class`.
fn is_matching_type(t: &TUnion, class: &str) -> bool {
    if t.is_mixed() {
        return false;
    }
    let kept: Vec<&TAtomic> = t.types.iter().filter(|a| !a.is_null()).collect();
    match kept[..] {
        [TAtomic::Object(TObject::Named(n))] => {
            n.name.as_bytes() == class.as_bytes()
                && n.type_parameters.is_none()
                && !n.is_static
                && !n.is_this
                && n.intersection_types.is_none()
        }
        // a class the codebase does not know
        [
            TAtomic::Reference(TReference::Symbol {
                name,
                parameters: None,
                intersection_types: None,
                ..
            }),
        ] => name.as_bytes() == class.as_bytes(),
        _ => false,
    }
}

/// The new property, and its doc comment, at the line break `nl`.
fn print_property(p: &NewProperty, nl: &str) -> (Option<String>, String) {
    if p.doc {
        (
            Some(format!("/**{nl} * @var \\{}{nl} */", p.class)),
            format!("private ${};", p.name),
        )
    } else {
        (None, format!("private \\{} ${};", p.class, p.name))
    }
}

/// The edits for the parameters and statements added to the class's method `method`: after its last parameter and
/// statement, or the whole method printed anew when its body was empty.
fn method_edits(
    source: &str,
    all: &[Token],
    method: &Method<'_>,
    block: &Block<'_>,
    params: &[Added],
    stmts: &[String],
) -> Vec<Edit> {
    let list = block_list(all, block);
    if !stmts.is_empty() && list.is_empty() {
        return vec![Edit::new(
            node_span(method),
            reprint_method(source, all, method, params, stmts),
        )];
    }
    let mut edits = Vec::new();
    let texts: Vec<&str> = params.iter().map(|a| a.text.as_str()).collect();
    let existing: Vec<Span> = method
        .parameter_list
        .parameters
        .iter()
        .map(|p| node_span(p))
        .collect();
    if !texts.is_empty() {
        match existing.last() {
            None => {
                let at = method.parameter_list.left_parenthesis.end.offset as usize;
                edits.push(Edit::new(Span::new(at, at), texts.join(", ")));
            }
            Some(last) => {
                let sep = if is_multiline(source, &existing) {
                    format!(",\n{}", " ".repeat(indentation(source, last.start)))
                } else {
                    ", ".to_string()
                };
                let text: String = texts.iter().map(|t| format!("{sep}{t}")).collect();
                edits.push(Edit::new(Span::new(last.end, last.end), text));
            }
        }
    }
    if !stmts.is_empty() {
        let after = list
            .stmts
            .last()
            .map_or(list.inner.start, |s| node_span(*s).end);
        let (start, end) = match list.nop {
            true => {
                let comments = comments_between(all, after, list.inner.end);
                (comments[0].start, comments[comments.len() - 1].end)
            }
            false => (node_span(*list.stmts.last().unwrap()).start, after),
        };
        let nl = format!("\n{}", " ".repeat(indentation(source, start)));
        let text: String = stmts.iter().map(|s| format!("{nl}{s}")).collect();
        edits.push(Edit::new(Span::new(end, end), text));
    }
    edits
}

/// `isMultiline`: two items or more, with a line break before the end of each after the first.
fn is_multiline(source: &str, items: &[Span]) -> bool {
    items.len() >= 2
        && items
            .windows(2)
            .all(|w| source[w[0].end..w[1].end].contains('\n'))
}

/// `pStmt_ClassMethod` for a method whose body was empty: its attributes and modifiers, its parameters (kept as
/// written) with the new ones, and the new statements.
fn reprint_method(
    source: &str,
    all: &[Token],
    method: &Method<'_>,
    params: &[Added],
    stmts: &[String],
) -> String {
    let level = indentation(source, node_span(method).start);
    let nl = format!("\n{}", " ".repeat(level));
    let nl4 = format!("\n{}", " ".repeat(level + 4));
    let text = |s: Span| &source[s.start..s.end];
    let mut out: String = method
        .attribute_lists
        .iter()
        .map(|a| format!("{}{nl}", text(node_span(a))))
        .collect();
    out.push_str(&print_modifiers(&method.modifiers, false));
    out.push_str("function ");
    if method.ampersand.is_some() {
        out.push('&');
    }
    out.push_str(&String::from_utf8_lossy(method.name.value));
    let originals: Vec<(Vec<Span>, Span, bool)> = (method.parameter_list.parameters.iter())
        .map(|p| {
            let s = node_span(p);
            let before = previous_token_end(source, all, s.start);
            (
                comments_between(all, before, s.start),
                s,
                !p.attribute_lists.is_empty(),
            )
        })
        .collect();
    let moved = |s: Span, to: usize| {
        reprint(
            source,
            all,
            s,
            to as isize - indentation(source, s.start) as isize,
        )
    };
    out.push('(');
    if originals.iter().all(|(c, _, a)| c.is_empty() && !a) {
        let mut items: Vec<String> = originals.iter().map(|(_, s, _)| moved(*s, level)).collect();
        items.extend(params.iter().map(|a| a.text.clone()));
        out.push_str(&items.join(", "));
    } else {
        let mut items: Vec<String> = originals
            .iter()
            .map(|(comments, s, _)| {
                let mut item = String::new();
                if !comments.is_empty() {
                    item.push_str(&nl4);
                    item.push_str(&print_comments(source, comments, &nl4));
                }
                item.push_str(&nl4);
                item.push_str(&moved(*s, level + 4));
                item
            })
            .collect();
        items.extend(params.iter().map(|a| format!("{nl4}{}", a.text)));
        out.push_str(&items.join(","));
        out.push_str(&nl);
    }
    out.push(')');
    if let Some(r) = &method.return_type_hint {
        out.push_str(": ");
        out.push_str(text(node_span(&r.hint)));
    }
    out.push_str(&nl);
    out.push('{');
    for s in stmts {
        out.push_str(&nl4);
        out.push_str(s);
    }
    out.push_str(&nl);
    out.push('}');
    out
}

// Reflection order

/// The shape of the class-like `name`: the class being changed from its own syntax, any other from its file (a
/// built-in one from its metadata, in a fixed order).
fn shape_of(
    facts: &Facts,
    class: &ClassView<'_, '_>,
    ancestors: &[Node<'_, '_>],
    current: &str,
    name: &str,
) -> Option<Shape> {
    if name.eq_ignore_ascii_case(current) {
        let resolve = |id: &Identifier<'_>| resolve_class_name(ancestors, id);
        let types = |list: &Option<Implements<'_>>| -> Vec<String> {
            list.iter()
                .flat_map(|i| i.types.iter().map(resolve))
                .collect()
        };
        let parent = class
            .extends
            .as_ref()
            .and_then(|e| e.types.iter().next())
            .map(resolve);
        return Some(shape_from(
            class.members,
            parent,
            types(class.implements),
            &resolve,
        ));
    }
    let meta = facts.codebase.get_class_like(name.as_bytes())?;
    let Some(source) = facts.source_of(meta.span.file_id) else {
        let mut interfaces: Vec<String> = meta
            .direct_parent_interfaces
            .iter()
            .map(|w| w.to_string())
            .collect();
        interfaces.sort();
        let mut methods: Vec<String> = meta.methods.iter().map(|w| w.to_string()).collect();
        methods.sort();
        return Some(Shape {
            parent: meta.direct_parent_class.as_ref().map(|w| w.to_string()),
            interfaces,
            methods,
            ..Shape::default()
        });
    };
    let arena = mago_allocator::LocalArena::new();
    let file = mago_database::file::File::ephemeral(
        std::borrow::Cow::Borrowed(b"class.php"),
        std::borrow::Cow::Owned(source.as_bytes().to_vec()),
    );
    let program = mago_syntax::parser::parse_file(&arena, &file);
    let names = mago_names::resolver::NameResolver::new(&arena).resolve(program);
    let resolve = |id: &Identifier<'_>| String::from_utf8_lossy(names.get(id)).into_owned();
    let wanted = meta.name.to_string();
    let mut shape = None;
    walk(Node::Program(program), &mut |n, path| {
        if shape.is_some() {
            return false;
        }
        let names_of = |types: Option<&TokenSeparatedSequence<'_, Identifier<'_>>>| -> Vec<String> {
            types
                .into_iter()
                .flat_map(|t| t.iter().map(resolve))
                .collect()
        };
        let found = match n {
            Node::Class(c) if qualified(path, c.name.value).eq_ignore_ascii_case(&wanted) => {
                Some(shape_from(
                    &c.members,
                    c.extends
                        .as_ref()
                        .and_then(|e| e.types.iter().next())
                        .map(resolve),
                    names_of(c.implements.as_ref().map(|i| &i.types)),
                    &resolve,
                ))
            }
            Node::Interface(i) if qualified(path, i.name.value).eq_ignore_ascii_case(&wanted) => {
                Some(shape_from(
                    &i.members,
                    None,
                    names_of(i.extends.as_ref().map(|e| &e.types)),
                    &resolve,
                ))
            }
            Node::Trait(t) if qualified(path, t.name.value).eq_ignore_ascii_case(&wanted) => {
                Some(shape_from(&t.members, None, Vec::new(), &resolve))
            }
            _ => None,
        };
        if found.is_some() {
            shape = found;
            return false;
        }
        true
    });
    shape
}

/// A class-like's shape from its members.
fn shape_from(
    members: &Sequence<'_, ClassLikeMember<'_>>,
    parent: Option<String>,
    interfaces: Vec<String>,
    resolve: &dyn Fn(&Identifier<'_>) -> String,
) -> Shape {
    let mut shape = Shape {
        parent,
        interfaces,
        ..Shape::default()
    };
    let mut promoted = Vec::new();
    for member in members.iter() {
        match member {
            ClassLikeMember::TraitUse(t) => shape.traits.extend(t.trait_names.iter().map(resolve)),
            ClassLikeMember::Method(m) => {
                shape
                    .methods
                    .push(String::from_utf8_lossy(m.name.value).into_owned());
                if m.name.value.eq_ignore_ascii_case(b"__construct") {
                    for p in m
                        .parameter_list
                        .parameters
                        .iter()
                        .filter(|p| !p.modifiers.is_empty())
                    {
                        promoted.push(Member {
                            name: String::from_utf8_lossy(&p.variable.name[1..]).into_owned(),
                            declaring: String::new(),
                            from_parent: false,
                            private: p
                                .modifiers
                                .iter()
                                .any(|m| matches!(m, Modifier::Private(_))),
                            is_static: false,
                        });
                    }
                }
            }
            ClassLikeMember::Property(p) => {
                let (modifiers, items): (_, Vec<&PropertyItem<'_>>) = match p {
                    Property::Plain(p) => (&p.modifiers, p.items.iter().collect()),
                    Property::Hooked(p) => (&p.modifiers, vec![&p.item]),
                };
                for item in items {
                    shape.properties.push(Member {
                        name: String::from_utf8_lossy(&item.variable().name[1..]).into_owned(),
                        declaring: String::new(),
                        from_parent: false,
                        private: modifiers.iter().any(|m| matches!(m, Modifier::Private(_))),
                        is_static: modifiers.iter().any(|m| matches!(m, Modifier::Static(_))),
                    });
                }
            }
            _ => {}
        }
    }
    shape.properties.extend(promoted);
    shape
}

fn position(members: &[Member], name: &str, case_insensitive: bool) -> Option<usize> {
    members.iter().position(|m| match case_insensitive {
        true => m.name.eq_ignore_ascii_case(name),
        false => m.name == name,
    })
}

/// BetterReflection's `getMethods()` of `name`: its own methods, then its parent's, its traits' (one replacing an
/// inherited method of its name), its interfaces'.
fn methods_of(
    facts: &Facts,
    class: &ClassView<'_, '_>,
    ancestors: &[Node<'_, '_>],
    current: &str,
    name: &str,
    visited: &mut Vec<String>,
) -> Option<Vec<Member>> {
    // a class among its own ancestors: Rector stops with an error
    if visited.iter().any(|v| v.eq_ignore_ascii_case(name)) {
        return None;
    }
    visited.push(name.to_string());
    let Some(shape) = shape_of(facts, class, ancestors, current, name) else {
        return Some(Vec::new());
    };
    let mut out: Vec<Member> = (shape.methods.iter())
        .map(|m| Member {
            name: m.clone(),
            declaring: name.to_string(),
            from_parent: false,
            private: false,
            is_static: false,
        })
        .collect();
    if let Some(parent) = &shape.parent {
        for m in methods_of(facts, class, ancestors, current, parent, visited)? {
            if position(&out, &m.name, true).is_none() {
                out.push(Member {
                    from_parent: true,
                    ..m
                });
            }
        }
    }
    for t in &shape.traits {
        for m in methods_of(facts, class, ancestors, current, t, &mut visited.clone())? {
            match position(&out, &m.name, true) {
                None => out.push(m),
                Some(i) if out[i].from_parent => out[i] = m,
                Some(_) => {}
            }
        }
    }
    for i in &shape.interfaces {
        for m in methods_of(facts, class, ancestors, current, i, &mut visited.clone())? {
            if position(&out, &m.name, true).is_none() {
                out.push(m);
            }
        }
    }
    Some(out)
}

/// BetterReflection's `getProperties()` of `name`: its parent's non-private ones, its interfaces', its traits', then
/// its own, each in the place of an inherited one of its name.
fn properties_of(
    facts: &Facts,
    class: &ClassView<'_, '_>,
    ancestors: &[Node<'_, '_>],
    current: &str,
    name: &str,
    visited: &mut Vec<String>,
) -> Option<Vec<Member>> {
    // a class among its own ancestors: Rector stops with an error
    if visited.iter().any(|v| v.eq_ignore_ascii_case(name)) {
        return None;
    }
    visited.push(name.to_string());
    let Some(shape) = shape_of(facts, class, ancestors, current, name) else {
        return Some(Vec::new());
    };
    let mut out: Vec<Member> = Vec::new();
    if let Some(parent) = &shape.parent {
        out.extend(
            properties_of(facts, class, ancestors, current, parent, visited)?
                .into_iter()
                .filter(|p| !p.private),
        );
    }
    let own: Vec<Member> = (shape.properties.into_iter())
        .map(|p| Member {
            declaring: name.to_string(),
            ..p
        })
        .collect();
    for i in &shape.interfaces {
        for p in properties_of(facts, class, ancestors, current, i, &mut visited.clone())? {
            upsert(&mut out, p);
        }
    }
    for t in &shape.traits {
        for p in properties_of(facts, class, ancestors, current, t, &mut visited.clone())? {
            if position(&own, &p.name, false).is_none() && position(&out, &p.name, false).is_none()
            {
                out.push(p);
            }
        }
    }
    for p in own {
        upsert(&mut out, p);
    }
    Some(out)
}

/// `p` in the place of the property of its name in `out`, else after them.
fn upsert(out: &mut Vec<Member>, p: Member) {
    match position(out, &p.name, false) {
        Some(at) => out[at] = p,
        None => out.push(p),
    }
}

// The parent constructor

/// A parent constructor as the injection copies it: its comments (spans in `source`), attribute groups and
/// modifiers, and its parameters, each printed anew.
struct ParentCtor {
    source: Arc<str>,
    comments: Vec<Span>,
    attributes: Vec<String>,
    modifiers: String,
    params: Vec<ParentParam>,
}

/// A parameter of the parent constructor: its name, comments, attribute groups and the rest of its code, and whether
/// it is a promoted property (`Some(true)` a private one).
struct ParentParam {
    name: String,
    comments: Vec<Span>,
    attributes: Vec<String>,
    text: String,
    promoted: Option<bool>,
}

/// The nearest constructor among the class's ancestors (the class has none of its own).
fn nearest_parent(facts: &Facts, name: &str) -> Nearest {
    let codebase = &facts.codebase;
    let found = codebase.get_class_like(name.as_bytes()).and_then(|meta| {
        (meta.declaring_method_ids.iter())
            .find(|(m, _)| m.as_bytes().eq_ignore_ascii_case(b"__construct"))
            .map(|(_, id)| id.get_class_name().to_string())
    });
    let Some((id, method)) = found.and_then(|id| {
        codebase
            .get_method(id.as_bytes(), b"__construct")
            .map(|method| (id, method))
    }) else {
        return Nearest::None;
    };
    // Pun: a constructor from a trait or an interface has no `parent::__construct()` to call.
    let kind = codebase.get_class_like(id.as_bytes()).map(|c| c.kind);
    if id.eq_ignore_ascii_case(name) || !matches!(kind, Some(SymbolKind::Class)) {
        return Nearest::Fail;
    }
    let meta = method.method_metadata.clone().unwrap_or_default();
    if meta.is_final {
        return Nearest::Final;
    }
    // a private one stops the search; a built-in class's has no syntax to copy
    if meta.visibility == Visibility::Private {
        return Nearest::None;
    }
    let Some(source) = facts.source_of(method.span.file_id) else {
        return Nearest::None;
    };
    if meta.is_abstract {
        return Nearest::Fail;
    }
    match parent_ctor(source, method.span.start.offset as usize) {
        Some(parent) => Nearest::Parent(Rc::new(parent)),
        None => Nearest::Fail,
    }
}

/// The constructor of `source` that holds the offset `at`, as the injection copies it; `None` for a variadic or
/// hooked parameter.
fn parent_ctor(source: Arc<str>, at: usize) -> Option<ParentCtor> {
    let arena = mago_allocator::LocalArena::new();
    let file = mago_database::file::File::ephemeral(
        std::borrow::Cow::Borrowed(b"parent.php"),
        std::borrow::Cow::Owned(source.as_bytes().to_vec()),
    );
    let program = mago_syntax::parser::parse_file(&arena, &file);
    let names = mago_names::resolver::NameResolver::new(&arena).resolve(program);
    let mut found = None;
    walk(Node::Program(program), &mut |n, _| {
        if found.is_some() {
            return false;
        }
        if let Node::Method(m) = n {
            let s = node_span(m);
            if m.name.value.eq_ignore_ascii_case(b"__construct") && s.start <= at && at < s.end {
                found = Some(m);
            }
            return false;
        }
        true
    });
    let method = found?;
    let all = crate::rules::php_parser::lex(&source);
    let printer = Printer {
        source: &source,
        names: &names,
    };
    let start = node_span(method).start;
    let mut params = Vec::new();
    for p in method.parameter_list.parameters.iter() {
        if p.ellipsis.is_some() || p.hooks.is_some() {
            return None;
        }
        let s = node_span(p);
        let mut text = param_modifiers(&p.modifiers);
        if let Some(hint) = &p.hint {
            text.push_str(&printer.hint(hint));
            text.push(' ');
        }
        if p.ampersand.is_some() {
            text.push('&');
        }
        text.push_str(&String::from_utf8_lossy(p.variable.name));
        if let Some(default) = &p.default_value {
            text.push_str(" = ");
            text.push_str(&printer.expr(default.value));
        }
        params.push(ParentParam {
            name: String::from_utf8_lossy(&p.variable.name[1..]).into_owned(),
            comments: comments_between(&all, previous_token_end(&source, &all, s.start), s.start),
            attributes: p
                .attribute_lists
                .iter()
                .map(|l| printer.attribute_list(l))
                .collect(),
            text,
            promoted: (!p.modifiers.is_empty()).then(|| {
                p.modifiers
                    .iter()
                    .any(|m| matches!(m, Modifier::Private(_)))
            }),
        });
    }
    Some(ParentCtor {
        comments: comments_between(&all, previous_token_end(&source, &all, start), start),
        attributes: method
            .attribute_lists
            .iter()
            .map(|l| printer.attribute_list(l))
            .collect(),
        modifiers: print_modifiers(&method.modifiers, false),
        params,
        source: Arc::clone(&source),
    })
}

/// `pModifiers` for a promoted parameter's modifiers.
fn param_modifiers(modifiers: &Sequence<'_, Modifier<'_>>) -> String {
    let has = |f: fn(&Modifier<'_>) -> bool| modifiers.iter().any(f);
    let order: [(&str, bool); 7] = [
        ("public", has(|m| matches!(m, Modifier::Public(_)))),
        ("protected", has(|m| matches!(m, Modifier::Protected(_)))),
        ("private", has(|m| matches!(m, Modifier::Private(_)))),
        ("public(set)", has(|m| matches!(m, Modifier::PublicSet(_)))),
        (
            "protected(set)",
            has(|m| matches!(m, Modifier::ProtectedSet(_))),
        ),
        (
            "private(set)",
            has(|m| matches!(m, Modifier::PrivateSet(_))),
        ),
        ("readonly", has(|m| matches!(m, Modifier::Readonly(_)))),
    ];
    order
        .iter()
        .filter(|(_, on)| *on)
        .map(|(n, _)| format!("{n} "))
        .collect()
}

/// php-parser's pretty printer for the parts of another file's code the injection copies, its names resolved as
/// `NameResolver` resolves them. A node it does not print anew keeps its source.
struct Printer<'s, 'n, 'arena> {
    source: &'s str,
    names: &'n ResolvedNames<'arena>,
}

impl Printer<'_, '_, '_> {
    fn text(&self, node: &impl mago_span::HasSpan) -> String {
        let s = node_span(node);
        self.source[s.start..s.end].to_string()
    }

    fn class(&self, id: &Identifier<'_>) -> String {
        format!("\\{}", String::from_utf8_lossy(self.names.get(id)))
    }

    /// A type declaration: class names fully qualified, built-in types lowercased.
    fn hint(&self, hint: &Hint<'_>) -> String {
        match hint {
            Hint::Identifier(id) => self.class(id),
            Hint::Nullable(n) => format!("?{}", self.hint(n.hint)),
            Hint::Union(u) => format!("{}|{}", self.hint(u.left), self.hint(u.right)),
            Hint::Intersection(i) => format!("{}&{}", self.hint(i.left), self.hint(i.right)),
            // an intersection in a union
            Hint::Parenthesized(p) => format!("({})", self.hint(p.hint)),
            Hint::Self_(_) | Hint::Parent(_) => self.text(hint),
            other => self.text(other).to_ascii_lowercase(),
        }
    }

    fn attribute_list(&self, list: &AttributeList<'_>) -> String {
        let attributes: Vec<String> = (list.attributes.iter())
            .map(|a| {
                let args = match &a.argument_list {
                    Some(l) if !l.arguments.is_empty() => {
                        let args: Vec<String> = l
                            .arguments
                            .iter()
                            .map(|arg| self.partial_argument(arg))
                            .collect();
                        format!("({})", args.join(", "))
                    }
                    _ => String::new(),
                };
                format!("{}{args}", self.class(&a.name))
            })
            .collect();
        format!("#[{}]", attributes.join(", "))
    }

    fn partial_argument(&self, arg: &PartialArgument<'_>) -> String {
        match arg {
            PartialArgument::Positional(p) if p.ellipsis.is_none() => self.expr(p.value),
            PartialArgument::Named(n) => format!(
                "{}: {}",
                String::from_utf8_lossy(n.name.value),
                self.expr(n.value)
            ),
            other => self.text(other),
        }
    }

    /// A constant expression as php-parser prints it anew.
    fn expr(&self, expr: &Expression<'_>) -> String {
        match expr {
            Expression::Parenthesized(p) => format!("({})", self.expr(p.expression)),
            Expression::Literal(Literal::Integer(i)) => i
                .value
                .and_then(|v| print_int(i.raw, v))
                .unwrap_or_else(|| self.text(expr)),
            Expression::Literal(Literal::Float(f)) => {
                super::phpunit::print_float(f.value.into_inner())
            }
            Expression::Literal(Literal::String(s)) => s
                .value
                .and_then(|v| print_string(s.kind, v))
                .unwrap_or_else(|| self.text(expr)),
            Expression::Array(a) => format!("[{}]", self.elements(&a.elements)),
            Expression::LegacyArray(a) => format!("array({})", self.elements(&a.elements)),
            Expression::UnaryPrefix(u) => match &u.operator {
                UnaryPrefixOperator::Negation(_) | UnaryPrefixOperator::Plus(_) => {
                    let op = if matches!(&u.operator, UnaryPrefixOperator::Negation(_)) {
                        "-"
                    } else {
                        "+"
                    };
                    let operand = self.expr(u.operand);
                    match operand.starts_with(op) {
                        true => format!("{op}({operand})"),
                        false => format!("{op}{operand}"),
                    }
                }
                _ => self.text(expr),
            },
            Expression::Access(Access::ClassConstant(c)) => {
                format!("{}::{}", self.class_expr(c.class), self.text(&c.constant))
            }
            Expression::Instantiation(n) => {
                let args: Vec<String> = (n.argument_list.iter())
                    .flat_map(|l| l.arguments.iter())
                    .map(|a| match a {
                        Argument::Positional(p) if p.ellipsis.is_none() => self.expr(p.value),
                        Argument::Named(n) => format!(
                            "{}: {}",
                            String::from_utf8_lossy(n.name.value),
                            self.expr(n.value)
                        ),
                        other => self.text(other),
                    })
                    .collect();
                format!("new {}({})", self.class_expr(n.class), args.join(", "))
            }
            _ => self.text(expr),
        }
    }

    /// A class reference: a name fully qualified, `self` and the like as written.
    fn class_expr(&self, class: &Expression<'_>) -> String {
        match unparenthesize(class) {
            Expression::Identifier(id) => self.class(id),
            other => self.text(other),
        }
    }

    fn elements(&self, elements: &TokenSeparatedSequence<'_, ArrayElement<'_>>) -> String {
        let items: Vec<String> = (elements.iter())
            .map(|e| match e {
                ArrayElement::KeyValue(kv) => {
                    format!("{} => {}", self.expr(kv.key), self.expr(kv.value))
                }
                ArrayElement::Value(v) => self.expr(v.value),
                other => self.text(other),
            })
            .collect();
        items.join(", ")
    }
}

/// `pScalar_Int` of an integer literal written `raw` with the value `value`: in its kind (decimal, `0x` lowercase,
/// `0` octal, `0b` binary).
fn print_int(raw: &[u8], value: u64) -> Option<String> {
    let raw = String::from_utf8_lossy(raw).to_ascii_lowercase();
    Some(if raw.starts_with("0x") {
        format!("0x{value:x}")
    } else if raw.starts_with("0b") {
        format!("0b{value:b}")
    } else if raw.len() > 1 && raw.starts_with('0') {
        format!("0{value:o}")
    } else {
        value.to_string()
    })
}
