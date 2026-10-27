//! PHPStan's DefinedVariableRule and CompactVariablesRule. Certainty comes from the running inference block,
//! not the checker's syntax walk; a missing observation is unknown, never an undefined variable.

use std::collections::{HashMap, HashSet};

use mago_analyzer::plugin::HookContext;
use mago_analyzer::plugin::context::InvocationInfo;
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Level, Tri, Type};

use super::{Checker, declared};

/// Only genuinely observed reads occur here. The larger scope snapshots are needed for dynamic names and
/// compact(), whose names come from the arguments' inferred types.
#[derive(Default)]
pub(super) struct Observations {
    pub reads: HashMap<Span, (String, Tri, bool)>,
    pub scopes: HashMap<Span, Certainty>,
    pub compact_scopes: HashMap<Span, Certainty>,
    pub unset_entries: HashMap<Span, TUnion>,
    pub property_selectors: HashSet<Span>,
}

pub(super) struct Certainty {
    variables: HashMap<String, Tri>,
    otherwise: Tri,
    main: bool,
}

/// A reference read permits an undefined variable, but PHPStan does not define its source. Keep the source's
/// running-block state across Mago's synthetic null assignment, without reconstructing assignment flow.
pub(super) struct ReferenceSource {
    name: mago_word::Word,
    local: Option<TUnion>,
    possible: bool,
    assigned: Option<u32>,
    possibly_assigned: bool,
}

impl ReferenceSource {
    pub fn of(a: &Assignment, context: &HookContext<'_, '_>) -> Option<Self> {
        let Expression::UnaryPrefix(u) = a.rhs else {
            return None;
        };
        let Expression::Variable(Variable::Direct(v)) = u.operand else {
            return None;
        };
        if !u.operator.is_reference()
            || defined(context, &String::from_utf8_lossy(v.name)).yes()
            || matches!(a.lhs, Expression::Variable(Variable::Direct(lhs)) if lhs.name == v.name)
        {
            return None;
        }
        let block = context.block_context();
        let name = mago_word::word(v.name);
        Some(Self {
            name,
            local: block.locals.get(&name).map(|t| (**t).clone()),
            possible: block.variables_possibly_in_scope.contains(&name),
            assigned: block.assigned_variable_ids.get(&name).copied(),
            possibly_assigned: block.possibly_assigned_variable_ids.contains(&name),
        })
    }

    pub fn restore(self, context: &mut HookContext<'_, '_>) {
        let block = context.block_context_mut();
        match self.local {
            Some(ty) => {
                block.locals.insert(self.name, std::rc::Rc::new(ty));
            }
            None => {
                block.locals.remove(&self.name);
            }
        }
        if self.possible {
            block.variables_possibly_in_scope.insert(self.name);
        } else {
            block.variables_possibly_in_scope.remove(&self.name);
        }
        match self.assigned {
            Some(offset) => {
                block.assigned_variable_ids.insert(self.name, offset);
            }
            None => {
                block.assigned_variable_ids.remove(&self.name);
            }
        }
        if self.possibly_assigned {
            block.possibly_assigned_variable_ids.insert(self.name);
        } else {
            block.possibly_assigned_variable_ids.remove(&self.name);
        }
    }
}

impl Certainty {
    pub fn of(context: &HookContext<'_, '_>) -> Self {
        let block = context.block_context();
        let variables = block
            .locals
            .keys()
            .chain(block.variables_possibly_in_scope.iter())
            .chain(block.variable_state.absent.iter())
            .map(|name| {
                let name = String::from_utf8_lossy(name.as_bytes()).into_owned();
                let certainty = defined(context, &name);
                (name, certainty)
            })
            .collect();
        Self {
            variables,
            otherwise: default_certainty(context),
            main: is_main(context),
        }
    }

    /// DefinedVariableRule filters each constant name's scope by `name-expression === name`.
    pub fn for_name(guard: Option<&Variable>, context: &HookContext<'_, '_>) -> Self {
        let mut result = Self::of(context);
        if let Some(Variable::Direct(guard)) = guard
            && let Some(ty) = context.get_variable_type(guard.name)
        {
            for value in ty
                .types
                .iter()
                .filter_map(|atomic| atomic.get_literal_string_value())
            {
                let mut filtered = context.block_context().clone();
                filtered.locals.insert(
                    mago_word::word(guard.name),
                    std::rc::Rc::new(mago_codex::ttype::get_literal_string(mago_word::word(
                        value,
                    ))),
                );
                filtered.apply_variable_conditions(context.codebase());
                let target = format!("${}", String::from_utf8_lossy(value));
                let certainty = filtered.variable_certainty(mago_word::word(&target));
                if certainty
                    != context
                        .block_context()
                        .variable_certainty(mago_word::word(&target))
                {
                    // A conditional holder only refines the merged Maybe to a definite Yes or No.
                    result
                        .variables
                        .insert(target, Tri::from(certainty.is_defined()));
                }
            }
        }
        result
    }

    pub fn get(&self, name: &str) -> Tri {
        if super::functions::SUPERGLOBALS.contains(&name.as_bytes()) {
            return Tri::Yes;
        }
        self.variables
            .get(&format!("${name}"))
            .copied()
            .unwrap_or(self.otherwise)
    }

    pub fn captured(&self, name: &str) -> Tri {
        // An arrow's function scope does not inherit main's canAnyVariableExist().
        self.variables
            .get(&format!("${name}"))
            .copied()
            .unwrap_or(Tri::No)
    }
}

fn default_certainty(context: &HookContext<'_, '_>) -> Tri {
    if context.block_context().can_any_variable_exist() {
        Tri::Maybe
    } else {
        Tri::No
    }
}

pub(super) fn is_main(context: &HookContext<'_, '_>) -> bool {
    context.scope().get_function_like().is_none() && context.scope().get_class_like().is_none()
}

/// Scope::hasVariableType(). Mago registers argc/argv as superglobals even inside functions; PHPStan doesn't.
pub(super) fn defined(context: &HookContext<'_, '_>, name: &str) -> Tri {
    // ScopeOps checks the unconditional superglobal names before looking at expression holders, even after unset.
    if super::functions::SUPERGLOBALS.contains(&&name.as_bytes()[1..]) {
        return Tri::Yes;
    }
    let block = context.block_context();
    let id = mago_word::word(name);
    if matches!(name, "$argc" | "$argv")
        && !block.assigned_variable_ids.contains_key(&id)
        && !block.references_to_external_scope.contains(&id)
        && !block
            .scope
            .get_function_like()
            .is_some_and(|f| f.parameters.iter().any(|p| p.get_name().0 == id))
    {
        return default_certainty(context);
    }
    let certainty = block.variable_certainty(id);
    if certainty.is_defined() {
        Tri::Yes
    } else if certainty.is_absent() {
        Tri::No
    } else {
        Tri::Maybe
    }
}

/// This walk only selects expressions for the inference run; it never guesses their control-flow state.
pub(super) fn typed_expressions(
    program: &Program,
    names: &mago_names::ResolvedNames,
) -> (Vec<Span>, HashSet<Span>) {
    struct Wanted<'n, 'arena> {
        spans: Vec<Span>,
        compact_calls: HashSet<Span>,
        names: &'n mago_names::ResolvedNames<'arena>,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted<'_, '_> {
        fn walk_in_variable(&mut self, v: &'ast Variable<'arena>, _: &mut ()) {
            self.spans.push(v.span());
            self.spans
                .extend(super::types::dynamic_names::static_property(v));
        }
        fn walk_in_closure(&mut self, c: &'ast Closure<'arena>, _: &mut ()) {
            self.spans.push(c.span());
        }
        fn walk_in_function_call(&mut self, f: &'ast FunctionCall<'arena>, _: &mut ()) {
            if is_compact(f, self.names) {
                self.compact_calls.insert(f.span());
                self.spans.push(f.span());
                self.spans
                    .extend(f.argument_list.arguments.iter().map(|a| a.value().span()));
            }
        }
    }
    let mut wanted = Wanted {
        spans: Vec::new(),
        compact_calls: HashSet::new(),
        names,
    };
    wanted.walk_program(program, &mut ());
    (wanted.spans, wanted.compact_calls)
}

/// NodeScopeResolver::lookForExpressionCallback() selects only the target chain, not name/index expressions.
pub(super) fn allow_target(expr: &Expression, out: &mut HashSet<Span>) {
    out.insert(expr.span());
    match expr {
        Expression::Parenthesized(p) => allow_target(p.expression, out),
        Expression::ArrayAccess(a) => allow_target(a.array, out),
        Expression::ArrayAppend(a) => allow_target(a.array, out),
        Expression::Access(Access::Property(p)) => allow_target(p.object, out),
        Expression::Access(Access::NullSafeProperty(p)) => allow_target(p.object, out),
        Expression::Access(Access::StaticProperty(p)) => allow_target(p.class, out),
        Expression::Call(Call::NullSafeMethod(m)) => allow_target(m.object, out),
        _ => {}
    }
}

pub(super) fn allowed_expressions(program: &Program) -> HashSet<Span> {
    struct Allowed(HashSet<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Allowed {
        fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, _: &mut ()) {
            match e {
                Expression::Construct(Construct::Isset(i)) => {
                    for value in &i.values {
                        allow_target(value, &mut self.0);
                    }
                }
                Expression::Construct(Construct::Empty(e)) => allow_target(e.value, &mut self.0),
                Expression::Binary(b) if matches!(b.operator, BinaryOperator::NullCoalesce(_)) => {
                    allow_target(b.lhs, &mut self.0)
                }
                Expression::Assignment(a)
                    if matches!(a.operator, AssignmentOperator::Coalesce(_)) =>
                {
                    allow_target(a.lhs, &mut self.0)
                }
                Expression::Assignment(a)
                    if matches!(a.operator, AssignmentOperator::Assign(_)) =>
                {
                    let mut root = super::types::flow::unparen(a.lhs);
                    loop {
                        root = match root {
                            Expression::ArrayAccess(a) => super::types::flow::unparen(a.array),
                            Expression::ArrayAppend(a) => super::types::flow::unparen(a.array),
                            _ => break,
                        };
                    }
                    if matches!(root, Expression::Variable(_)) {
                        self.0.insert(root.span());
                    }
                }
                Expression::UnaryPrefix(u) if u.operator.is_reference() => {
                    allow_target(u.operand, &mut self.0)
                }
                _ => {}
            }
        }
        fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, _: &mut ()) {
            for value in &u.values {
                allow_target(value, &mut self.0);
            }
        }
    }
    let mut allowed = Allowed(HashSet::new());
    allowed.walk_program(program, &mut ());
    allowed.0
}

/// ArgumentsHandler::processArgs(): output parameters allow undefined target chains. A named user parameter
/// with a non-nullable native type and a built-in's read-reference parameter do not. Closure/callable
/// parameters have no separate native reflection. The engine's reference flag cannot distinguish these cases.
pub(super) fn allow_arguments(
    invocation: &InvocationInfo<'_, '_, '_>,
    builtins: &pun_phpstan::symbols::Symbols,
    allowed: &mut HashSet<Span>,
    non_creating: &mut HashSet<Span>,
) {
    let args = invocation.arguments();
    let invocation = invocation.inner();
    let metadata = invocation.target.get_function_like_metadata();
    let builtin = match invocation.target.get_function_like_identifier() {
        Some(FunctionLikeIdentifier::Function(name)) => builtins
            .functions
            .get(&name.to_string().to_ascii_lowercase()),
        Some(FunctionLikeIdentifier::Method(class, name)) => builtins
            .classes
            .get(&class.to_string().to_ascii_lowercase())
            .and_then(|class| class.methods.get(&name.to_string().to_ascii_lowercase())),
        _ => None,
    };
    let builtin = builtin.filter(|_| metadata.is_none_or(|m| m.flags.is_built_in()));
    for (i, arg) in args.iter().enumerate() {
        let Some(value) = arg.value() else { continue };
        let permits = if let Some(builtin) = builtin {
            let variant = builtin
                .variants
                .iter()
                .find(|v| {
                    v.params
                        .iter()
                        .filter(|p| !p.optional && !p.variadic)
                        .count()
                        <= args.len()
                        && (v.params.iter().any(|p| p.variadic) || args.len() <= v.params.len())
                })
                .or(builtin.variants.first());
            variant
                .and_then(|v| match arg.get_parameter_name() {
                    Some(name) => v
                        .params
                        .iter()
                        .find(|p| p.name.as_bytes() == name)
                        .or_else(|| v.params.last().filter(|p| p.variadic)),
                    _ => v
                        .params
                        .get(i)
                        .or_else(|| v.params.last().filter(|p| p.variadic)),
                })
                .is_some_and(|p| {
                    // Some Mago builtin signatures use a reference where PHPStan reads a value.
                    // Mark the real argument transfer, not a pre-call variable snapshot: later
                    // arguments may assign that same input before the invocation completes.
                    if p.by_ref != pun_phpstan::symbols::ByRef::CreatesNewVariable {
                        allow_target(value, non_creating);
                    }
                    p.by_ref == pun_phpstan::symbols::ByRef::CreatesNewVariable
                })
        } else {
            let parameter = match arg.get_parameter_name() {
                Some(name) => invocation
                    .target
                    .iter_parameters()
                    .find(|p| p.get_name().is_some_and(|n| &n.0.as_bytes()[1..] == name)),
                _ => invocation.target.get_parameter(i),
            }
            .or_else(|| {
                invocation
                    .target
                    .iter_parameters()
                    .last()
                    .filter(|p| p.is_variadic())
            });
            parameter.is_some_and(|p| {
                p.is_by_reference()
                    && (invocation
                        .target
                        .get_function_like_identifier()
                        .is_some_and(|id| id.is_closure())
                        || metadata
                            .and_then(|m| {
                                m.parameters
                                    .iter()
                                    .find(|original| Some(original.get_name()) == p.get_name())
                            })
                            .and_then(|p| p.type_declaration_metadata.as_ref())
                            .is_none_or(|t| t.type_union.has_null() || t.type_union.is_mixed()))
            })
        };
        if permits {
            allow_target(value, allowed);
        }
    }
}

/// CompactVariablesRule reads php-parser's post-NameResolver name, not the runtime function reflection.
/// Only an unimported local name stays written; imports and qualified/relative names are resolved.
fn is_compact(call: &FunctionCall, names: &mago_names::ResolvedNames) -> bool {
    let Expression::Identifier(id) = call.function else {
        return false;
    };
    let name = match id {
        Identifier::Local(local) if !names.is_imported(&local.span) => local.value,
        _ => names.get(&id.span()),
    };
    // Mago retains a leading separator for `namespace\compact` in an empty namespace.
    name.strip_prefix(b"\\")
        .unwrap_or(name)
        .eq_ignore_ascii_case(b"compact")
}

/// DefinedVariableRule::processNode(). Name castability is checked even in assignment/undefined-allowed contexts.
pub(super) fn variable(c: &mut Checker, variable: &Variable) {
    let span = variable.span();
    // A static-property selector shares this CST shape, but is not a variable expression.
    if c.variables.property_selectors.contains(&span) {
        return;
    }
    if let Some(name) = super::types::dynamic_names::static_property(variable) {
        if c.non_stringable_names
            && let Some((ty, _)) = super::types::dynamic_names::not_string(c, name, true)
        {
            let given = declared::with_classes(c, || ty.describe(Level::Precise));
            c.report(
                name,
                "variable.nameNotString",
                format!("Variable variable name must be a string, but {given} was given."),
                None,
            );
        }
        let Some(scope) = c.variables.scopes.get(&span) else {
            return;
        };
        let Some(ty) = c.types.get(&name).map(|t| c.to_phpstan(t)) else {
            return;
        };
        let names = ty.constant_strings();
        let mut seen = HashSet::new();
        let main = scope.main;
        let names: Vec<_> = names
            .into_iter()
            .map(|name| String::from_utf8_lossy(&name).into_owned())
            .filter(|name| seen.insert(name.clone()))
            .map(|name| {
                let certainty = scope.get(&name);
                (name, certainty)
            })
            .collect();
        for (name, certainty) in names {
            single_variable(c, variable, &name, certainty, main);
        }
    } else if let Some((name, certainty, main)) = c.variables.reads.get(&span).cloned() {
        single_variable(c, variable, &name, certainty, main);
    }
}

pub(super) fn capture(c: &mut Checker, capture: &ClosureUseClauseVariable) {
    if let Some((name, certainty, main)) = c.variables.reads.get(&capture.variable.span()).cloned()
    {
        single_variable(c, &capture.variable, &name, certainty, main);
    }
}

/// DefinedVariableRule::processSingleVariable(). Suppressed reads have no observation.
fn single_variable(
    c: &mut Checker,
    variable: &impl HasSpan,
    name: &str,
    certainty: Tri,
    main: bool,
) {
    // Explicit Closure::bind() keeps PHPStan's inherited $this even when Mago removes its null-bound local.
    if name == "this" && c.this_defined() {
        return;
    }
    if c.cli_arguments_variables_registered && matches!(name, "argc" | "argv") && main {
        return;
    }
    let message = match certainty {
        Tri::No => format!("Undefined variable: ${name}"),
        Tri::Maybe if c.check_maybe_undefined_variables => {
            format!("Variable ${name} might not be defined.")
        }
        _ => return,
    };
    c.report(variable, "variable.undefined", message, None);
}

/// CompactVariablesRule::processNode(). Its scope precedes argument evaluation, even though argument types can
/// describe assignments made by their evaluation.
pub(super) fn compact(c: &mut Checker, call: &FunctionCall) {
    // Only calls selected by the name-resolution pass can have a compact scope observation.
    let Some(scope) = c.variables.compact_scopes.get(&call.span()) else {
        return;
    };
    let mut errors = Vec::new();
    for arg in &call.argument_list.arguments {
        let Some(ty) = c.phpstan_type(arg.value()) else {
            continue;
        };
        let mut names = Vec::new();
        declared::with_classes(c, || find_constant_strings(&ty, &mut names));
        for name in names {
            let adjective = match scope.get(&name) {
                Tri::No => "undefined",
                Tri::Maybe if c.check_maybe_undefined_variables => "possibly undefined",
                _ => continue,
            };
            errors.push((
                arg.span(),
                format!("Call to function compact() contains {adjective} variable ${name}."),
            ));
        }
    }
    for (span, message) in errors {
        c.report_at_line(c.line_of(span), "variable.undefined", message, None);
    }
}

/// CompactVariablesRule::findConstantStrings(): constant strings first, otherwise recurse through constant arrays.
fn find_constant_strings(ty: &Type, out: &mut Vec<String>) {
    let strings = ty.constant_strings();
    if !strings.is_empty() {
        out.extend(strings.into_iter().map(|s| String::from_utf8_lossy(&s).into_owned()));
        return;
    }
    for array in ty.constant_arrays() {
        for value in &array.values {
            find_constant_strings(value, out);
        }
    }
}
