//! PHPStan's DefinedVariableRule and CompactVariablesRule. Certainty comes from the running inference block,
//! not the checker's syntax walk; a missing observation is unknown, never an undefined variable.

use std::collections::{HashMap, HashSet};

use mago_analyzer::plugin::HookContext;
use mago_analyzer::plugin::context::InvocationInfo;
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::get_literal_string;
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
    /// A dim fetch's direct-variable receiver (by span): its type in the scope before the fetch, which
    /// NonexistentOffsetInArrayDimFetchRule reads. Mago analyses the dimension first, so the receiver's own type
    /// already sees the dimension's assignments.
    pub dim_receivers: HashMap<Span, TUnion>,
    pub property_selectors: HashSet<Span>,
    /// The by-reference parameters' types around their bodies' statements and writes.
    pub param_out: super::types::param_out::Observed,
    /// A plain assignment's target (by span): its root variable's full and native types once the assigned value is
    /// evaluated, when the variable is certainly defined (UnusedVariableRule's redundant assignments).
    pub assigned_roots: HashMap<Span, (TUnion, Option<TUnion>)>,
    /// `isUndefinedExpressionAllowed()` as the inference saw it: the targets of `isset()`, `??` and the like, and
    /// the arguments of by-reference parameters that create a variable.
    pub allowed: HashSet<Span>,
    /// The allowed direct variables (by span) that are certainly undefined there: their ErrorType skips the
    /// argument check of a by-reference parameter that creates the variable.
    pub undefined_allowed: HashSet<Span>,
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
    pub(super) fn of(a: &Assignment, context: &HookContext<'_, '_>) -> Option<Self> {
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

    pub(super) fn restore(self, context: &mut HookContext<'_, '_>) {
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
    pub(super) fn of(context: &HookContext<'_, '_>) -> Self {
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
    pub(super) fn for_name(guard: &Expression<'_>, context: &HookContext<'_, '_>) -> Self {
        let mut result = Self::of(context);
        let guard = super::types::flow::unparen(guard);
        let ty = match guard {
            Expression::Variable(Variable::Direct(v)) => {
                context.get_variable_type(v.name).map(std::rc::Rc::as_ref)
            }
            _ => context.get_expression_type(guard),
        };
        if let Some(ty) = ty {
            for value in ty
                .types
                .iter()
                .filter_map(|atomic| atomic.get_literal_string_value())
            {
                let literal = get_literal_string(mago_word::word(value));
                let Some(name) = OffsetName::of(guard) else {
                    continue;
                };
                let mut filtered = context.block_context().clone();
                filtered.locals.insert(name.id, std::rc::Rc::new(literal));
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

    pub(super) fn get(&self, name: &str) -> Tri {
        if super::functions::SUPERGLOBALS.contains(&name.as_bytes()) {
            return Tri::Yes;
        }
        self.variables
            .get(&format!("${name}"))
            .copied()
            .unwrap_or(self.otherwise)
    }

    pub(super) fn captured(&self, name: &str) -> Tri {
        // An arrow's function scope does not inherit main's canAnyVariableExist().
        self.variables
            .get(&format!("${name}"))
            .copied()
            .unwrap_or(Tri::No)
    }
}

/// Only literal offsets of local arrays have stable identities without selector dependencies.
/// Their observed types participate in Mago's existing branch merge and conditional-holder invalidation.
#[derive(PartialEq, Eq, Hash)]
pub(super) struct OffsetName {
    root: mago_word::Word,
    id: mago_word::Word,
}

impl OffsetName {
    fn of(guard: &Expression<'_>) -> Option<Self> {
        match super::types::flow::unparen(guard) {
            Expression::Variable(Variable::Direct(v)) => {
                let id = mago_word::word(v.name);
                Some(Self { root: id, id })
            }
            Expression::ArrayAccess(access) => {
                let mut name = Self::of(access.array)?;
                let key = match super::types::flow::unparen(access.index) {
                    Expression::Literal(Literal::String(s)) => {
                        ArrayKey::from_string(mago_word::word(s.value?))
                    }
                    Expression::Literal(Literal::Integer(i)) => {
                        ArrayKey::Integer(i64::try_from(i.value?).ok()?)
                    }
                    _ => return None,
                };
                name.id = mago_word::concat_word!(
                    name.id.as_bytes(),
                    b"[",
                    key.id_word().as_bytes(),
                    b"]"
                );
                Some(name)
            }
            _ => None,
        }
    }

    /// Materialize only selected name offsets after a real write, never by replaying statement syntax.
    pub(super) fn assigned(&self, assignment: &Assignment<'_>, context: &mut HookContext<'_, '_>) {
        let mut lhs = super::types::flow::unparen(assignment.lhs);
        while let Expression::ArrayAccess(access) = lhs {
            lhs = super::types::flow::unparen(access.array);
        }
        if !matches!(lhs, Expression::Variable(Variable::Direct(v)) if v.name == self.root.as_bytes())
        {
            return;
        }
        // Whole-root and unstable-offset writes invalidate the cached expression too. Do not create
        // correlations from a whole array's fields: PHPStan records the actual assigned expression.
        let ty = Self::of(assignment.lhs)
            .filter(|name| name.id == self.id)
            .and_then(|_| context.get_expression_type(assignment).cloned());
        let block = context.block_context_mut();
        match ty {
            Some(ty) => {
                block.locals.insert(self.id, std::rc::Rc::new(ty));
            }
            None => {
                block.locals.remove(&self.id);
            }
        }
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
) -> (Vec<Span>, HashSet<Span>, HashSet<OffsetName>) {
    struct Wanted<'n, 'arena> {
        spans: Vec<Span>,
        compact_calls: HashSet<Span>,
        offset_names: HashSet<OffsetName>,
        names: &'n mago_names::ResolvedNames<'arena>,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted<'_, '_> {
        fn walk_in_variable(&mut self, v: &'ast Variable<'arena>, (): &mut ()) {
            self.spans.push(v.span());
            self.spans
                .extend(super::types::dynamic_names::static_property(v));
            if let Variable::Indirect(v) = v
                && let Some(name) = OffsetName::of(v.expression)
                && name.id != name.root
            {
                self.offset_names.insert(name);
            }
        }
        fn walk_in_closure(&mut self, c: &'ast Closure<'arena>, (): &mut ()) {
            self.spans.push(c.span());
        }
        fn walk_in_function_call(&mut self, f: &'ast FunctionCall<'arena>, (): &mut ()) {
            // FileAssertRule's assertVariableCertainty() reads the same scope before its arguments
            if is_compact(f, self.names) || super::debug::is_variable_certainty(f, self.names) {
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
        offset_names: HashSet::new(),
        names,
    };
    wanted.walk_program(program, &mut ());
    (wanted.spans, wanted.compact_calls, wanted.offset_names)
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

/// MutatingScope::isUndefinedExpressionAllowed() compares node keys: while the target chain of `expr` (and of the
/// earlier operands in `names`) is allowed, an equal variable elsewhere in `expr`, such as an offset, is too.
fn allow_named<'arena>(expr: &Expression<'arena>, names: &mut Vec<&'arena [u8]>, out: &mut HashSet<Span>) {
    struct Named<'n, 'arena>(&'n [&'arena [u8]], &'n mut HashSet<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Named<'_, 'arena> {
        fn walk_in_direct_variable(&mut self, v: &'ast DirectVariable<'arena>, (): &mut ()) {
            if self.0.contains(&v.name) {
                self.1.insert(v.span());
            }
        }
    }
    let mut link = expr;
    loop {
        link = match link {
            Expression::Parenthesized(p) => p.expression,
            Expression::ArrayAccess(a) => a.array,
            Expression::Access(Access::Property(p)) => p.object,
            Expression::Access(Access::NullSafeProperty(p)) => p.object,
            Expression::Access(Access::StaticProperty(p)) => p.class,
            Expression::Call(Call::NullSafeMethod(m)) => m.object,
            Expression::Variable(Variable::Direct(v)) => {
                names.push(v.name);
                break;
            }
            _ => break,
        };
    }
    if !names.is_empty() {
        Named(names, out).walk_expression(expr, &mut ());
    }
}

pub(super) fn allowed_expressions(program: &Program) -> HashSet<Span> {
    struct Allowed(HashSet<Span>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Allowed {
        fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, (): &mut ()) {
            match e {
                Expression::Construct(Construct::Isset(i)) => {
                    // IssetHandler unsets the allowed expressions only after its last operand.
                    let mut names = Vec::new();
                    for value in &i.values {
                        allow_target(value, &mut self.0);
                        allow_named(value, &mut names, &mut self.0);
                    }
                }
                Expression::Construct(Construct::Empty(e)) => {
                    allow_target(e.value, &mut self.0);
                    allow_named(e.value, &mut Vec::new(), &mut self.0);
                }
                Expression::Binary(b) if matches!(b.operator, BinaryOperator::NullCoalesce(_)) => {
                    allow_target(b.lhs, &mut self.0);
                    allow_named(b.lhs, &mut Vec::new(), &mut self.0);
                }
                Expression::Assignment(a)
                    if matches!(a.operator, AssignmentOperator::Coalesce(_)) =>
                {
                    allow_target(a.lhs, &mut self.0);
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
                    allow_target(u.operand, &mut self.0);
                }
                _ => {}
            }
        }
        fn walk_in_unset(&mut self, u: &'ast Unset<'arena>, (): &mut ()) {
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
        if c.on(crate::rules::Switches::NON_STRINGABLE_NAMES)
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
    if c.on(crate::rules::Switches::CLI_ARGUMENTS_VARIABLES_REGISTERED) && matches!(name, "argc" | "argv") && main {
        return;
    }
    let message = match certainty {
        Tri::No => format!("Undefined variable: ${name}"),
        Tri::Maybe if c.on(crate::rules::Switches::CHECK_MAYBE_UNDEFINED_VARIABLES) => {
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
    if !is_compact(call, c.names) {
        return;
    }
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
                Tri::Maybe if c.on(crate::rules::Switches::CHECK_MAYBE_UNDEFINED_VARIABLES) => "possibly undefined",
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
