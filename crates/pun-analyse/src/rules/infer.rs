//! Mago's inferred types of the expressions the rules ask about (`$scope->getType()`), from one analyser run per
//! file, only over the declarations that hold such an expression.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use mago_allocator::{Arena, LocalArena};
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::context::InvocationInfo;
use mago_analyzer::plugin::{
    ExpressionHook, ExpressionHookResult, HookContext, HookResult, Provider, ProviderMeta,
};
use mago_analyzer::settings::Settings;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::reference::SymbolReferences;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::union::TUnion;
use mago_database::file::File;
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::Tri;

use super::{Checker, assign, dead_code, variables};

#[path = "type_origins.rs"]
mod origins;
pub(super) use origins::{Origin, Origins};

/// The expressions whose type a rule reads: the class of `new $expr`, the object or class expression of a call whose
/// by-reference parameters matter, the expression of an `instanceof` naming a trait.
struct TypedExpressions<'n, 'arena> {
    spans: Vec<Span>,
    names: &'n ResolvedNames<'arena>,
    codebase: &'n CodebaseMetadata,
    level: u8,
    php_version: u32,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for TypedExpressions<'_, 'arena> {
    fn walk_in_variable(&mut self, v: &'ast Variable<'arena>, _: &mut ()) {
        if self.level >= 1 {
            self.spans.extend(dead_code::typed_variable(v));
        }
    }
    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        self.spans
            .extend(dead_code::typed_arguments(call, self.names, self.level));
    }
    fn walk_in_statement_expression(&mut self, s: &'ast ExpressionStatement<'arena>, _: &mut ()) {
        self.spans
            .extend(dead_code::typed_statement(s, self.level, self.php_version));
    }
    fn walk_in_binary(&mut self, b: &'ast Binary<'arena>, _: &mut ()) {
        // `$this` is described without its type
        if matches!(b.operator, BinaryOperator::Instanceof(_))
            && !matches!(b.lhs, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
            && let Expression::Identifier(id) = b.rhs
            && self
                .codebase
                .get_class_like(self.names.get(&id.span()))
                .is_some_and(|k| k.kind == SymbolKind::Trait)
        {
            self.spans.push(b.lhs.span());
        }
    }
    fn walk_in_instantiation(&mut self, new: &'ast Instantiation<'arena>, _: &mut ()) {
        if !matches!(
            new.class,
            Expression::Identifier(_)
                | Expression::Static(_)
                | Expression::Self_(_)
                | Expression::Parent(_)
                | Expression::AnonymousClass(_)
        ) {
            self.spans.push(new.class.span());
        }
    }
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        self.spans
            .extend(assign::typed_object(call.object, &call.argument_list).map(HasSpan::span));
    }
    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        if let Some(fetch) = assign::referenced_property(a)
            && !matches!(fetch.object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
        {
            self.spans.push(fetch.object.span());
        }
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.spans
            .extend(assign::typed_object(call.class, &call.argument_list).map(HasSpan::span));
    }
}

/// Runs Mago's analyzer over the file when a rule needs an expression's type (most files need none).
#[allow(clippy::too_many_arguments)]
pub(super) fn infer<'arena>(
    arena: &'arena LocalArena,
    file: &File,
    program: &'arena Program<'arena>,
    names: &ResolvedNames<'arena>,
    codebase: &CodebaseMetadata,
    php_version: u32,
    level: u8,
    extra: Vec<Span>,
    variables: &mut variables::Observations,
) -> (HashMap<Span, TUnion>, HashMap<Span, Tri>, Origins) {
    let mut wanted = TypedExpressions {
        spans: Vec::new(),
        names,
        codebase,
        level,
        php_version,
    };
    wanted.walk_program(program, &mut ());
    let mut wanted = wanted.spans;
    wanted.extend(extra);
    wanted.extend(variables::typed_expressions(program));
    wanted.extend(super::unset::expressions(program).0);
    wanted.extend(dead_code::flow_types(program, level));
    let (typed, watched) = dead_code::coalesce_types(program, level);
    wanted.extend(typed);
    analyse_observed(arena, file, program, names, codebase, php_version, wanted, watched, Some(variables))
}

/// Mago's types of the `wanted` expressions, analysing only the declarations that hold them.
pub(super) fn analyse<'arena>(
    arena: &'arena LocalArena,
    file: &File,
    program: &'arena Program<'arena>,
    names: &ResolvedNames<'arena>,
    codebase: &CodebaseMetadata,
    php_version: u32,
    wanted: Vec<Span>,
    watched: Vec<Span>,
) -> (HashMap<Span, TUnion>, HashMap<Span, Tri>, Origins) {
    analyse_observed(arena, file, program, names, codebase, php_version, wanted, watched, None)
}

#[allow(clippy::too_many_arguments)]
fn analyse_observed<'arena>(
    arena: &'arena LocalArena,
    file: &File,
    program: &'arena Program<'arena>,
    names: &ResolvedNames<'arena>,
    codebase: &CodebaseMetadata,
    php_version: u32,
    wanted: Vec<Span>,
    watched: Vec<Span>,
    variables: Option<&mut variables::Observations>,
) -> (HashMap<Span, TUnion>, HashMap<Span, Tri>, Origins) {
    if wanted.is_empty() {
        return Default::default();
    }
    let origins = origins::collect(file, program, names, codebase);
    let mut wanted = wanted;
    wanted.extend(origins::dependencies(&origins, &wanted));
    // Array conversion needs each key/value's actual inferred type, not just the enclosing wanted span.
    // Retain only literals inside requested expressions; unrelated declarations must stay prunable.
    let mut children = ArrayChildren { wanted: &wanted, spans: Vec::new() };
    children.walk_program(program, &mut ());
    let children = children.spans;
    wanted.extend(children);
    wanted.extend(origins::dependencies(&origins, &wanted));
    let mut registry = super::return_types::registry(&file.contents);
    let found = Arc::new(Mutex::new(Found {
        allowed: variables.as_ref().map(|_| variables::allowed_expressions(program)).unwrap_or_default(),
        ..Found::default()
    }));
    if !watched.is_empty() || variables.is_some() {
        let ids = watched
            .into_iter()
            .map(|s| {
                let text = &file.contents[s.start.offset as usize..s.end.offset as usize];
                (s, text.iter().filter(|b| !b.is_ascii_whitespace()).copied().collect())
            })
            .collect();
        registry.register_expression_hook(Definedness {
            ids,
            builtins: super::reflection::builtins(php_version),
            unset_scopes: super::unset::expressions(program).1,
            found: Arc::clone(&found),
        });
    }
    let settings = Settings::new(crate::codebase::php_version(php_version));
    let analyzer = Analyzer::new(arena, file, names, codebase, &registry, settings);
    let mut result = AnalysisResult::new(SymbolReferences::new());
    // Only the declarations holding a wanted expression: a method's types come from its own body and the codebase,
    // so the file's other functions and methods would only cost time (most of the rules' time on large classes).
    let program = arena.alloc(Program {
        statements: Prune {
            arena,
            wanted: &wanted,
        }
        .statements(&program.statements),
        ..program.clone()
    });
    // ponytail: Mago's analyzer trips debug assertions on some code (an empty TUnion::from_vec); no types then
    let analysed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        analyzer.analyze_with_artifacts(program, &mut result)
    }));
    let Ok(Ok(artifacts)) = analysed else { return Default::default() };
    let mut types: HashMap<Span, TUnion> = wanted
        .into_iter()
        .filter_map(|span| Some((span, artifacts.get_expression_type(&span)?.clone())))
        .collect();
    let mut found = std::mem::take(&mut *found.lock().expect("no hook panics while holding the lock"));
    types.extend(found.read);
    if let Some(variables) = variables {
        *variables = std::mem::take(&mut found.variables);
    }
    (types, found.defined, origins)
}

struct ArrayChildren<'s> {
    wanted: &'s [Span],
    spans: Vec<Span>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for ArrayChildren<'_> {
    fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, _: &mut ()) {
        let elements = match e {
            Expression::Array(a) => &a.elements,
            Expression::LegacyArray(a) => &a.elements,
            _ => return,
        };
        let span = e.span();
        if !self.wanted.iter().any(|w| w.start.offset <= span.start.offset && w.end.offset >= span.end.offset) {
            return;
        }
        self.spans.push(span);
        for element in elements.iter() {
            if let ArrayElement::KeyValue(kv) = element {
                self.spans.push(kv.key.span());
            }
            self.spans.extend(element.get_value().map(HasSpan::span));
        }
    }
}

/// `$scope->hasVariableType()` of the watched direct variables, and `hasExpressionType()` of the other watched
/// expressions (by Mago's expression id, their text without whitespace), from Mago's block context where it reads
/// them. A loop body is analysed more than once: answers that differ are a maybe.
struct Definedness {
    ids: HashMap<Span, Vec<u8>>,
    builtins: &'static pun_phpstan::symbols::Symbols,
    unset_scopes: HashSet<Span>,
    found: Arc<Mutex<Found>>,
}

/// What [`Definedness`] found: the certainties, and the watched variables' types where they are read (Mago keeps a
/// `??=` target's type after the assignment).
#[derive(Default)]
struct Found {
    defined: HashMap<Span, Tri>,
    read: HashMap<Span, TUnion>,
    variables: variables::Observations,
    allowed: HashSet<Span>,
    arrows: HashMap<Span, variables::Certainty>,
    reference_sources: HashMap<Span, variables::ReferenceSource>,
}

impl Provider for Definedness {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new("pun-definedness", "definedness", "variable certainty");
        &META
    }
}

impl ExpressionHook for Definedness {
    fn is_observer(&self) -> bool {
        true
    }

    fn before_arguments(
        &self,
        invocation: &InvocationInfo<'_, '_, '_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let mut found = self.found.lock().expect("no hook panics while holding the lock");
        let mut non_creating = HashSet::new();
        variables::allow_arguments(invocation, self.builtins, &mut found.allowed, &mut non_creating);
        context.block_context_mut().variable_state.non_creating_references.extend(non_creating);
        Ok(())
    }

    fn before_expression(&self, expr: &Expression<'_>, context: &mut HookContext<'_, '_>) -> HookResult<ExpressionHookResult> {
        let span = expr.span();
        {
            let mut found = self.found.lock().expect("no hook panics while holding the lock");
            if self.unset_scopes.contains(&span) {
                found.variables.scopes.insert(span, variables::Certainty::of(context));
                super::unset::observe(expr, context, &mut found.read);
            }
            match expr {
                Expression::Variable(v) => observe_variable(v, context, &mut found),
                Expression::Access(Access::StaticProperty(p)) => {
                    if let Variable::Nested(v) = &p.property {
                        observe_variable(v.variable, context, &mut found);
                    }
                }
                Expression::Call(Call::Function(f)) if variables::is_compact(f) => {
                    found.variables.scopes.insert(span, variables::Certainty::of(context));
                }
                Expression::Closure(c) => {
                    if let Some(uses) = &c.use_clause {
                        for capture in &uses.variables {
                            if capture.ampersand.is_none() {
                                observe_direct(&capture.variable, context, &mut found);
                            }
                        }
                    }
                }
                Expression::ArrowFunction(f) => {
                    found.arrows.insert(f.span(), variables::Certainty::of(context));
                }
                Expression::Assignment(a) => {
                    if let Some(source) = variables::ReferenceSource::of(a, context) {
                        found.reference_sources.insert(span, source);
                    }
                }
                _ => {}
            }
        }
        if let Some(id) = self.ids.get(&span) {
            let block = context.block_context();
            let name = mago_word::word(id);
            let variable = matches!(expr, Expression::Variable(_));
            let local = block.locals.get(&name);
            let now = if variable {
                let certainty = block.variable_certainty(name);
                if certainty.is_defined() {
                    Tri::Yes
                } else if certainty.is_absent() {
                    Tri::No
                } else {
                    Tri::Maybe
                }
            } else {
                match local {
                    Some(t) if t.possibly_undefined() || t.possibly_undefined_from_try() => Tri::Maybe,
                    Some(_) => Tri::Yes,
                    None => Tri::No,
                }
            };
            let mut found = self.found.lock().expect("no hook panics while holding the lock");
            if let Some(t) = local.filter(|_| variable) {
                found.read.insert(span, (**t).clone());
            }
            let entry = found.defined.entry(span).or_insert(now);
            if *entry != now {
                *entry = Tri::Maybe;
            }
        }
        Ok(ExpressionHookResult::Continue)
    }

    fn after_expression(&self, expr: &Expression<'_>, context: &mut HookContext<'_, '_>) -> HookResult<()> {
        // A compound assignment later rewrites its target's artifacts, including the holder
        // offsets. Keep the completed source read before those virtual writes take place.
        if self.ids.contains_key(&expr.span()) && !matches!(expr, Expression::Variable(_))
            && let Some(ty) = context.get_expression_type(expr)
        {
            self.found.lock().expect("no hook panics while holding the lock").read.insert(expr.span(), ty.clone());
        }
        {
            if let Some(source) = self.found.lock().expect("no hook panics while holding the lock").reference_sources.remove(&expr.span()) {
                source.restore(context);
            }
            let (dynamic, guard) = match expr {
                Expression::Variable(Variable::Nested(n)) => (true, Some(n.variable)),
                Expression::Variable(Variable::Indirect(i)) => (true, match super::types::flow::unparen(i.expression) {
                    Expression::Variable(v) => Some(v),
                    _ => None,
                }),
                _ => (false, None),
            };
            if dynamic {
                let mut found = self.found.lock().expect("no hook panics while holding the lock");
                if !found.allowed.contains(&expr.span()) {
                    found.variables.scopes.insert(expr.span(), variables::Certainty::for_name(guard, context));
                }
            }
        }
        Ok(())
    }
}

fn observe_variable(v: &Variable<'_>, context: &HookContext<'_, '_>, found: &mut Found) {
    match v {
        Variable::Direct(v) => observe_direct(v, context, found),
        // NestedVariable::analyze() calls Variable::analyze(), bypassing the expression hook for the name.
        Variable::Nested(v) => {
            if let Variable::Direct(name) = v.variable
                && let Some(ty) = context.get_variable_type(name.name)
            {
                found.read.insert(name.span(), (**ty).clone());
            }
            observe_variable(v.variable, context, found);
        }
        Variable::Indirect(_) => {}
    }
}

fn observe_direct(v: &DirectVariable<'_>, context: &HookContext<'_, '_>, found: &mut Found) {
    if found.allowed.contains(&v.span()) { return; }
    let name = String::from_utf8_lossy(v.name);
    // Mago places every implicit arrow capture in possibly_in_scope, even one absent in its parent. Read the
    // parent's real block observation for those names, not a syntactically reconstructed parent scope.
    let certainty = context.scope().get_function_like()
        .filter(|_| v.name != b"$this" && context.get_variable_type(v.name).is_none())
        .and_then(|f| found.arrows.get(&f.span))
        .map_or_else(|| variables::defined(context, &name), |parent| parent.captured(&name[1..]));
    let entry = found.variables.reads.entry(v.span()).or_insert_with(|| (name[1..].to_string(), certainty, variables::is_main(context)));
    if entry.1 != certainty { entry.1 = Tri::Maybe; }
}

/// A copy of a program's statements without the functions, class-likes and methods that hold none of `wanted`. Every
/// other statement and member stays, so the kept code's scope is unchanged.
struct Prune<'w, 'arena> {
    arena: &'arena LocalArena,
    wanted: &'w [Span],
}

impl<'arena> Prune<'_, 'arena> {
    fn holds(&self, span: Span) -> bool {
        self.wanted
            .iter()
            .any(|w| span.start.offset <= w.start.offset && w.end.offset <= span.end.offset)
    }

    fn statements(
        &self,
        statements: &Sequence<'arena, Statement<'arena>>,
    ) -> Sequence<'arena, Statement<'arena>> {
        let kept = statements.iter().filter_map(|s| match s {
            Statement::Function(_)
            | Statement::Class(_)
            | Statement::Interface(_)
            | Statement::Trait(_)
            | Statement::Enum(_)
                if !self.holds(s.span()) =>
            {
                None
            }
            Statement::Namespace(n) => Some(Statement::Namespace(Namespace {
                body: match &n.body {
                    NamespaceBody::Implicit(b) => NamespaceBody::Implicit(NamespaceImplicitBody {
                        statements: self.statements(&b.statements),
                        ..b.clone()
                    }),
                    NamespaceBody::BraceDelimited(b) => NamespaceBody::BraceDelimited(Block {
                        statements: self.statements(&b.statements),
                        ..b.clone()
                    }),
                },
                ..n.clone()
            })),
            Statement::Class(c) => Some(Statement::Class(Class {
                members: self.members(&c.members),
                ..c.clone()
            })),
            Statement::Trait(t) => Some(Statement::Trait(Trait {
                members: self.members(&t.members),
                ..t.clone()
            })),
            Statement::Enum(e) => Some(Statement::Enum(Enum {
                members: self.members(&e.members),
                ..e.clone()
            })),
            s => Some(s.clone()),
        });
        Sequence::from_slice(self.arena.alloc_slice_fill_iter(kept))
    }

    fn members(
        &self,
        members: &Sequence<'arena, ClassLikeMember<'arena>>,
    ) -> Sequence<'arena, ClassLikeMember<'arena>> {
        let kept = members
            .iter()
            .filter(|m| !matches!(m, ClassLikeMember::Method(_)) || self.holds(m.span()))
            .cloned();
        Sequence::from_slice(self.arena.alloc_slice_fill_iter(kept))
    }
}

impl<'a, 'arena> Checker<'a, 'arena> {
    /// `$scope->getType($expr)`, as Mago infers it, for the expressions [`typed_expressions()`] lists.
    pub(super) fn type_of(&self, expr: &Expression) -> Option<&TUnion> {
        self.types.get(&expr.span())
    }
}
