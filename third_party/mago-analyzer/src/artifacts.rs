use std::rc::Rc;
use std::sync::Arc;

use foldhash::HashMap;
use foldhash::HashSet;
use mago_word::Word;
use mago_word::WordMap;
use mago_word::WordSet;

use mago_algebra::assertion_set::AssertionSet;
use mago_algebra::clause::Clause;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::reference::SymbolReferences;
use mago_codex::ttype::combine_owned_union_types_rc;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::Node;

use crate::artifacts::editor::EditorArtifacts;
use crate::context::block::BlockContext;
use crate::context::block::ReferenceConstraintSource;
use crate::context::block::VariableScope;
use crate::context::block::VariableState;
use crate::context::scope::case_scope::CaseScope;
use crate::context::scope::loop_scope::LoopScope;
use crate::readonly::PendingReadonlyPropertyWrite;

pub use self::expression_result::ExpressionResult;
use self::expression_result::OwnExpressionType;
pub(crate) use self::expression_types::CheckpointAction;
pub(crate) use self::expression_types::ExpressionTypeCheckpoints;

pub(crate) type DetachedExpressionTypeCheckpoints = (
    ExpressionTypeCheckpoints,
    Option<ExpressionTypeCheckpoints>,
    ExpressionTypeCheckpoints<bool>,
    ExpressionTypeCheckpoints<OwnExpressionType>,
);

#[derive(Debug, Clone, Default)]
struct NativeExpressionTypes {
    types: HashMap<(u32, u32), Rc<TUnion>>,
    checkpoints: ExpressionTypeCheckpoints,
}

pub mod editor;
mod expression_result;
mod expression_types;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum VariableDefinedness {
    Defined = 1,
    PossiblyDefined = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum VariableThrowKind {
    Expression,
    Declared,
    Implicit,
}

/// Represents scope information extracted from a `Closure::bind()` or `Closure::bindTo()` call.
/// This is used to pass the bound class scope to closure/arrow function analysis.
#[derive(Debug, Clone)]
pub struct ClosureBindScope {
    /// The class name for the bound scope (from the newScope argument).
    pub class_name: Option<Word>,
    /// Whether the closure has `$this` bound (newThis argument is non-null object).
    pub has_this: bool,
    /// Runtime `$this` is independent of the class used for member access.
    pub this_type: Option<Rc<TUnion>>,
    /// The same binding's native holder, not its PHPDoc-selected full type.
    pub native_this_type: Option<Rc<TUnion>>,
}

/// One semantic receiver and method target retained for a source-level call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedMethodCall {
    pub span: (u32, u32),
    pub class: Word,
    pub method: Word,
}

pub(crate) struct BranchFlow {
    loop_scope: Option<LoopScope>,
    case_scopes: Vec<CaseScope>,
    throw_scopes: HashMap<(Span, VariableThrowKind), WordMap<VariableScope>>,
    property_abrupt_scopes: HashMap<Span, crate::property_initialization::Markers>,
}

/// PHPStan ExpressionResult's evaluated-operand scopes, distinct from branch specifications.
#[derive(Debug, Clone)]
pub(crate) struct BooleanExpressionScopes {
    pub post: BooleanExpressionScope,
    pub truthy: BooleanExpressionScope,
    pub falsey: BooleanExpressionScope,
}

/// A cached type does not prove that its variable was assigned on every incoming edge.
#[derive(Debug, Clone)]
pub(crate) struct BooleanExpressionScope {
    pub locals: WordMap<Rc<TUnion>>,
    pub variable_state: VariableState,
    pub variables_possibly_in_scope: WordSet,
    pub stable_method_calls: WordSet,
    pub stable_method_call_assertions: WordMap<AssertionSet>,
}

impl BooleanExpressionScope {
    pub(crate) fn restore_certainty(&self, block_context: &mut BlockContext<'_>) {
        block_context.variable_state.clone_from(&self.variable_state);
        block_context.variables_possibly_in_scope.clone_from(&self.variables_possibly_in_scope);
        block_context.stable_method_calls.clone_from(&self.stable_method_calls);
        block_context.stable_method_call_assertions.clone_from(&self.stable_method_call_assertions);
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisArtifacts {
    /// Effective full values. Use get_raw_expression_type for a call's own unprojected return.
    pub expression_types: HashMap<(u32, u32), Rc<TUnion>>,
    own_expression_types: HashMap<(u32, u32), OwnExpressionType>,
    own_expression_type_checkpoints: ExpressionTypeCheckpoints<OwnExpressionType>,
    native_expression_types: Option<NativeExpressionTypes>,
    /// Whether every resolved call target explicitly cannot return, independently of dynamic return types.
    pub call_terminations: HashMap<(u32, u32), bool>,
    call_termination_checkpoints: ExpressionTypeCheckpoints<bool>,
    pub(crate) boolean_expression_scopes: HashMap<(u32, u32), BooleanExpressionScopes>,
    /// Walked-call specifications, consumed only by the current expression statement.
    /// Like BooleanExpressionScopes, these are derived results, not convergence inputs.
    pub(crate) null_context_clauses: HashMap<(u32, u32), Vec<Clause>>,
    pub if_true_assertions: HashMap<(u32, u32), WordMap<AssertionSet>>,
    pub if_false_assertions: HashMap<(u32, u32), WordMap<AssertionSet>>,
    pub true_branch_only_assertions: HashMap<(u32, u32), WordMap<AssertionSet>>,
    pub inferred_return_types: Vec<Rc<TUnion>>,
    pub inferred_yield_key_types: Vec<TUnion>,
    pub inferred_yield_value_types: Vec<TUnion>,
    pub symbol_references: SymbolReferences,
    pub loop_scope: Option<LoopScope>,
    pub case_scopes: Vec<CaseScope>,
    pub fully_matched_switch_offsets: HashSet<u32>,
    pub inferred_parameter_types: Option<HashMap<usize, TUnion>>,
    /// ArrayHandler's passed-to type: the value type an argument's parameter expects of the array literal analysed
    /// next, whose closure items take their parameter types from its callables.
    pub(crate) array_passed_to_type: Option<TUnion>,
    /// Results of providers' typed callback invocations, local to the asking call.
    pub(crate) callback_returns: Vec<(Span, Vec<TUnion>, TUnion)>,
    pub method_initialized_properties: HashMap<(Word, Word), WordSet>,
    pub method_calls_this_methods: HashMap<(Word, Word), HashSet<Word>>,
    pub method_calls_parent_constructor: HashMap<(Word, Word), bool>,
    pub method_calls_parent_initializer: HashMap<(Word, Word), Word>,
    pub closure_bind_scope: Option<ClosureBindScope>,
    pub resolved_method_calls: Vec<ResolvedMethodCall>,
    /// Requested, owned rule-facing answers; distinct from the actual walk's expression types.
    pub call_scope_observations: HashMap<Span, Vec<crate::observations::NamedCallScope>>,
    /// Owned live-before-operand virtual initialization certainties, only for requested spans/names.
    pub property_initialization_observations: HashMap<Span, crate::property_initialization::Markers>,
    /// Function-local raw method return snapshots; not propagated out of nested functions.
    pub(crate) constructor_property_returns: HashMap<Span, crate::property_initialization::Markers>,
    /// Live statement ends for CalledMethodProcessor, separate from raw returns and runtime exits.
    pub(crate) property_statement_ends: HashMap<Span, Vec<crate::property_initialization::Markers>>,
    /// Runtime abrupt scopes for finally entry/propagation; raw constructor returns stay separate.
    pub(crate) property_abrupt_scopes: HashMap<Span, crate::property_initialization::Markers>,
    /// NullsafeMethodCallExpressionNode's processed receiver, before shallow non-null specification.
    pub nullsafe_method_receiver_types: HashMap<Span, TUnion>,
    /// A property's whole new value after an offset write (`$this->p[] = $v`), by the root fetch's span.
    pub property_offset_writes: HashMap<Span, TUnion>,
    /// Body-boundary value-read/write answers, separate from type and definedness observations.
    pub variable_writes: HashMap<Span, crate::variable_flow::VariableWritesNode>,
    pub(crate) editor: Option<EditorArtifacts>,
    pub(crate) variable_definedness: HashMap<(u32, u32), WordMap<VariableDefinedness>>,
    pub(crate) variable_throw_scopes: HashMap<(Span, VariableThrowKind), WordMap<VariableScope>>,
    variable_definedness_targets: Option<Arc<[bool; u8::MAX as usize + 1]>>,
    pub(crate) pending_readonly_property_writes: Vec<PendingReadonlyPropertyWrite>,
    pub(crate) static_local_types: Option<WordMap<Rc<TUnion>>>,
    expression_type_checkpoints: ExpressionTypeCheckpoints,
    retain_completed_expression_types: bool,
}

impl Default for AnalysisArtifacts {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisArtifacts {
    #[must_use]
    pub fn new() -> Self {
        Self {
            expression_types: HashMap::default(),
            own_expression_types: HashMap::default(),
            own_expression_type_checkpoints: ExpressionTypeCheckpoints::default(),
            native_expression_types: None,
            call_terminations: HashMap::default(),
            call_termination_checkpoints: ExpressionTypeCheckpoints::default(),
            boolean_expression_scopes: HashMap::default(),
            null_context_clauses: HashMap::default(),
            inferred_return_types: Vec::new(),
            inferred_yield_key_types: Vec::new(),
            inferred_yield_value_types: Vec::new(),
            if_true_assertions: HashMap::default(),
            if_false_assertions: HashMap::default(),
            true_branch_only_assertions: HashMap::default(),
            symbol_references: SymbolReferences::new(),
            case_scopes: Vec::new(),
            loop_scope: None,
            fully_matched_switch_offsets: HashSet::default(),
            inferred_parameter_types: None,
            array_passed_to_type: None,
            callback_returns: Vec::new(),
            method_initialized_properties: HashMap::default(),
            method_calls_this_methods: HashMap::default(),
            method_calls_parent_constructor: HashMap::default(),
            method_calls_parent_initializer: HashMap::default(),
            closure_bind_scope: None,
            resolved_method_calls: Vec::new(),
            call_scope_observations: HashMap::default(),
            property_initialization_observations: HashMap::default(),
            constructor_property_returns: HashMap::default(),
            property_statement_ends: HashMap::default(),
            property_abrupt_scopes: HashMap::default(),
            nullsafe_method_receiver_types: HashMap::default(),
            property_offset_writes: HashMap::default(),
            variable_writes: HashMap::default(),
            editor: None,
            variable_definedness: HashMap::default(),
            variable_throw_scopes: HashMap::default(),
            variable_definedness_targets: None,
            pending_readonly_property_writes: Vec::new(),
            static_local_types: None,
            expression_type_checkpoints: ExpressionTypeCheckpoints::default(),
            retain_completed_expression_types: true,
        }
    }

    /// Impossible branches are still analysed for diagnostics and expression observers, but their
    /// break/continue/throw edges do not contribute to the enclosing statement's flow.
    pub(crate) fn branch_flow(&self) -> BranchFlow {
        BranchFlow {
            loop_scope: self.loop_scope.clone(),
            case_scopes: self.case_scopes.clone(),
            throw_scopes: self.variable_throw_scopes.clone(),
            property_abrupt_scopes: self.property_abrupt_scopes.clone(),
        }
    }

    pub(crate) fn restore_branch_flow(&mut self, flow: BranchFlow) {
        self.loop_scope = flow.loop_scope;
        self.case_scopes = flow.case_scopes;
        self.variable_throw_scopes = flow.throw_scopes;
        self.property_abrupt_scopes = flow.property_abrupt_scopes;
    }

    pub(crate) fn with_retained_expression_types(mut self, retain: bool) -> Self {
        self.retain_completed_expression_types = retain;
        self
    }

    pub(crate) fn with_native_types(mut self, enabled: bool) -> Self {
        self.native_expression_types = enabled.then(NativeExpressionTypes::default);
        self
    }

    pub(crate) fn should_retain_completed_expression_types(&self) -> bool {
        self.retain_completed_expression_types || !self.expression_type_checkpoints.is_empty()
    }

    pub(crate) fn record_static_local_types(
        &mut self,
        block_context: &BlockContext<'_>,
        codebase: &CodebaseMetadata,
        options: CombinerOptions,
    ) {
        let Some(static_local_types) = self.static_local_types.as_mut() else {
            return;
        };

        for variable in &block_context.static_locals {
            if block_context
                .by_reference_constraints
                .get(variable)
                .is_some_and(|constraint| constraint.source == ReferenceConstraintSource::Static)
            {
                continue;
            }

            let Some(variable_type) = block_context.locals.get(variable) else {
                continue;
            };

            if let Some(previous_type) = static_local_types.get_mut(variable) {
                let previous = std::mem::replace(previous_type, Rc::clone(variable_type));
                *previous_type = combine_owned_union_types_rc(previous, variable_type, codebase, options);
            } else {
                static_local_types.insert(*variable, Rc::clone(variable_type));
            }
        }
    }

    pub(crate) fn with_variable_definedness_targets(
        mut self,
        targets: Option<Arc<[bool; u8::MAX as usize + 1]>>,
    ) -> Self {
        self.variable_definedness_targets = targets;
        self
    }

    pub(crate) fn variable_definedness_targets(&self) -> Option<Arc<[bool; u8::MAX as usize + 1]>> {
        self.variable_definedness_targets.clone()
    }

    #[inline]
    pub(crate) fn record_variable_definedness(&mut self, node: Node<'_, '_>, block_context: &BlockContext<'_>) {
        let Some(targets) = self.variable_definedness_targets.as_deref() else {
            return;
        };

        let span = node.span();
        if !node_or_same_span_descendant_is_targeted(node, span, targets) {
            return;
        }

        let mut variables = WordMap::default();
        for (variable, variable_type) in &block_context.locals {
            if !is_plain_variable(*variable) {
                continue;
            }

            let definedness = if variable_type.possibly_undefined_from_try()
                || variable_type.possibly_undefined()
                    && block_context.possibly_undefined_variable_ids.contains(variable)
            {
                VariableDefinedness::PossiblyDefined
            } else {
                VariableDefinedness::Defined
            };
            variables.insert(*variable, definedness);
        }

        for variable in &block_context.variables_possibly_in_scope {
            if is_plain_variable(*variable) {
                variables.entry(*variable).or_insert(VariableDefinedness::PossiblyDefined);
            }
        }

        self.variable_definedness.insert((span.start.offset, span.end.offset), variables);
    }

    pub(crate) fn set_loop_scope(&mut self, loop_scope: LoopScope) {
        let previous_scope = self.loop_scope.take().map(Box::new);
        self.loop_scope = Some(loop_scope.with_parent_loop(previous_scope));
    }

    /// SAFETY: the caller must ensure that `self.loop_scope` is not `None`.
    pub(crate) unsafe fn take_loop_scope_unchecked(&mut self) -> LoopScope {
        let mut loop_scope = unsafe {
            // SAFETY: the caller must ensure that `self.loop_scope` is not `None`.
            self.loop_scope.take().unwrap_unchecked()
        };

        match loop_scope.parent_loop.take() {
            Some(parent_loop) => {
                self.loop_scope = Some(*parent_loop);
            }
            None => {
                self.loop_scope = None;
            }
        }

        loop_scope
    }

    pub(crate) fn get_loop_scope_mut(&mut self) -> Option<&mut LoopScope> {
        self.loop_scope.as_mut()
    }

    pub(crate) fn record_loop_assignment_target(&mut self, target: Word) {
        let mut loop_scope = self.loop_scope.as_mut();
        while let Some(scope) = loop_scope {
            if scope.tracks_assignment_targets {
                scope.assignment_targets.insert(target);
            }

            loop_scope = scope.parent_loop.as_deref_mut();
        }
    }

    /// Set the type of expression `expression` to `t`.
    #[inline]
    pub fn set_expression_type<T>(&mut self, expression: &T, t: TUnion)
    where
        T: HasSpan,
    {
        self.set_rc_expression_type(expression, Rc::new(t));
    }

    /// Get the type of expression `expression`.
    #[inline]
    pub fn get_expression_type<T>(&self, expression: &T) -> Option<&TUnion>
    where
        T: HasSpan,
    {
        let t = self.expression_types.get(&get_expression_range(expression))?;

        Some(&**t)
    }

    pub(crate) fn set_call_termination<T: HasSpan>(&mut self, expression: &T, terminates: bool) {
        self.call_termination_checkpoints.insert(
            &mut self.call_terminations,
            get_expression_range(expression),
            terminates,
        );
    }

    pub(crate) fn transfer_call_terminations(&mut self, child: &mut Self) {
        for (range, terminates) in child.call_terminations.drain() {
            self.call_termination_checkpoints.insert(&mut self.call_terminations, range, terminates);
        }
    }

    /// Native type from this same evaluation. None means disabled, unsupported, or not evaluated;
    /// it is never substituted with the full type or mixed. This is an explicitly partial API.
    pub fn get_native_expression_type<T: HasSpan>(&self, expression: &T) -> Option<&TUnion> {
        self.get_rc_native_expression_type(expression).map(Rc::as_ref)
    }

    pub(crate) fn get_rc_native_expression_type<T: HasSpan>(&self, expression: &T) -> Option<&Rc<TUnion>> {
        self.native_expression_types.as_ref()?.types.get(&get_expression_range(expression))
    }

    pub(crate) fn set_rc_native_expression_type<T: HasSpan>(&mut self, expression: &T, ty: Rc<TUnion>) {
        let range = get_expression_range(expression);
        if self.native_expression_types.is_some()
            && let Some(mut own) = self.own_expression_types.get(&range).cloned()
            && own.raw_native.is_none()
        {
            own.raw_native = Some(Rc::clone(&ty));
            self.own_expression_type_checkpoints.insert(&mut self.own_expression_types, range, own);
        }
        if let Some(native) = &mut self.native_expression_types {
            native.checkpoints.insert(&mut native.types, range, ty);
        }
    }

    pub(crate) fn remove_native_expression_type(&mut self, range: &(u32, u32)) {
        if let Some(native) = &mut self.native_expression_types {
            native.checkpoints.remove(&mut native.types, range);
        }
    }

    pub(crate) fn clear_native_expression_types(&mut self) {
        if let Some(native) = &mut self.native_expression_types {
            for range in native.types.keys().copied().collect::<Vec<_>>() {
                native.checkpoints.remove(&mut native.types, &range);
            }
        }
    }

    pub(crate) fn transfer_native_expression_types(&mut self, child: &mut Self) {
        if let (Some(parent), Some(child)) = (&mut self.native_expression_types, &mut child.native_expression_types) {
            for (range, ty) in child.types.drain() {
                parent.checkpoints.insert(&mut parent.types, range, ty);
            }
        }
    }

    /// Set the type of expression `expression` to `t`.
    #[inline]
    pub fn set_rc_expression_type<T>(&mut self, expression: &T, t: Rc<TUnion>)
    where
        T: HasSpan,
    {
        let range = get_expression_range(expression);
        if let Some(mut own) = self.own_expression_types.get(&range).cloned() {
            own.overridden = own.raw.is_some();
            own.raw.get_or_insert_with(|| Rc::clone(&t));
            self.own_expression_type_checkpoints.insert(&mut self.own_expression_types, range, own);
        }
        self.expression_type_checkpoints.insert(&mut self.expression_types, range, t);
        self.remove_native_expression_type(&range);
    }

    /// Get the type of expression `expression`.
    #[inline]
    pub fn get_rc_expression_type<T>(&self, expression: &T) -> Option<&Rc<TUnion>>
    where
        T: HasSpan,
    {
        self.expression_types.get(&get_expression_range(expression))
    }

    pub(crate) fn begin_expression_type_checkpoint(&mut self) {
        self.expression_type_checkpoints.begin_checkpoint();
        self.own_expression_type_checkpoints.begin_checkpoint();
        self.call_termination_checkpoints.begin_checkpoint();
        if let Some(native) = &mut self.native_expression_types {
            native.checkpoints.begin_checkpoint();
        }
        if let Some(editor) = &mut self.editor {
            editor.begin_checkpoint();
        }
    }

    pub(crate) fn expression_type_checkpoint_matches(&self, same: impl Fn(&TUnion, &TUnion) -> bool) -> bool {
        // Native state is passive: it must not change the full lane's loop iteration decisions.
        self.expression_type_checkpoints.checkpoint_matches(&self.expression_types, |a, b| same(a, b))
            && self.own_expression_type_checkpoints.checkpoint_matches(&self.own_expression_types, |a, b| {
                a.raw == b.raw
                    && a.holder == b.holder
                    && a.projects_void == b.projects_void
                    && a.resolved == b.resolved
                    && a.overridden == b.overridden
            })
            && self.call_termination_checkpoints.checkpoint_matches(&self.call_terminations, |a, b| a == b)
    }

    pub(crate) fn finish_expression_type_checkpoint(&mut self, action: CheckpointAction) {
        let restore = |range: &(u32, u32)| {
            self.expression_type_checkpoints.can_restore_related(range, &self.expression_types, Rc::ptr_eq)
        };
        self.own_expression_type_checkpoints.finish_checkpoint_where(&mut self.own_expression_types, action, restore);
        self.call_termination_checkpoints.finish_checkpoint_where(&mut self.call_terminations, action, restore);
        if let Some(native) = &mut self.native_expression_types {
            native.checkpoints.finish_checkpoint_where(&mut native.types, action, restore);
        }
        self.expression_type_checkpoints.finish_checkpoint(&mut self.expression_types, action);
        if let Some(editor) = &mut self.editor {
            editor.finish_checkpoint(action);
        }
    }

    pub(crate) fn remove_expression_type(&mut self, range: &(u32, u32)) {
        self.expression_type_checkpoints.remove(&mut self.expression_types, range);
        self.own_expression_type_checkpoints.remove(&mut self.own_expression_types, range);
        self.call_termination_checkpoints.remove(&mut self.call_terminations, range);
        self.remove_native_expression_type(range);
    }

    pub(crate) fn extend_expression_types(&mut self, types: impl IntoIterator<Item = ((u32, u32), Rc<TUnion>)>) {
        if self.expression_type_checkpoints.is_empty() && self.native_expression_types.is_none() {
            for (range, ty) in types {
                self.expression_types.insert(range, ty);
                self.own_expression_types.remove(&range);
            }
        } else {
            for (range, ty) in types {
                self.expression_type_checkpoints.insert(&mut self.expression_types, range, ty);
                self.own_expression_type_checkpoints.remove(&mut self.own_expression_types, &range);
                if let Some(native) = &mut self.native_expression_types {
                    native.checkpoints.remove(&mut native.types, &range);
                }
            }
        }
    }

    pub(crate) fn detach_expression_type_checkpoints(&mut self) -> Option<DetachedExpressionTypeCheckpoints> {
        if self.expression_type_checkpoints.is_empty() && self.native_expression_types.is_none() {
            return None;
        }

        Some((
            self.expression_type_checkpoints.detach(&self.expression_types),
            self.native_expression_types.as_mut().map(|native| native.checkpoints.detach(&native.types)),
            self.call_termination_checkpoints.detach(&self.call_terminations),
            self.own_expression_type_checkpoints.detach(&self.own_expression_types),
        ))
    }

    pub(crate) fn restore_expression_type_checkpoints(&mut self, checkpoints: DetachedExpressionTypeCheckpoints) {
        self.expression_type_checkpoints = checkpoints.0;
        self.call_termination_checkpoints = checkpoints.2;
        self.own_expression_type_checkpoints = checkpoints.3;
        if let Some(checkpoints) = checkpoints.1 {
            self.native_expression_types.get_or_insert_with(NativeExpressionTypes::default).checkpoints = checkpoints;
        } else if let Some(native) = &mut self.native_expression_types {
            // A transported result can introduce a native lane while a mutable hook
            // has detached the checkpoints. Every such native entry is new to them.
            for _ in 0..self.expression_type_checkpoints.depth() {
                native.checkpoints.begin_checkpoint();
            }
            for (range, ty) in std::mem::take(&mut native.types) {
                native.checkpoints.insert(&mut native.types, range, ty);
            }
        }
    }
}

fn node_or_same_span_descendant_is_targeted(
    node: Node<'_, '_>,
    span: Span,
    targets: &[bool; u8::MAX as usize + 1],
) -> bool {
    if targets[node.kind() as usize] {
        return true;
    }

    let mut targeted = false;
    node.visit_children(|child| {
        if !targeted && child.span() == span {
            targeted = node_or_same_span_descendant_is_targeted(child, span, targets);
        }
    });

    targeted
}

fn is_plain_variable(variable: Word) -> bool {
    let bytes = variable.as_bytes();

    bytes.starts_with(b"$")
        && !bytes.contains(&b'[')
        && !bytes.windows(2).any(|window| window == b"->" || window == b"::")
}

#[inline]
pub fn get_expression_range<T>(expression: &T) -> (u32, u32)
where
    T: HasSpan,
{
    let span = expression.span();

    (span.start.offset, span.end.offset)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::rc::Rc;

    use mago_codex::context::ScopeContext;
    use mago_codex::metadata::CodebaseMetadata;
    use mago_codex::reference::ReferenceOrigin;
    use mago_codex::ttype::combine_union_types;
    use mago_codex::ttype::combiner::CombinerOptions;
    use mago_codex::ttype::flags::UnionFlags;
    use mago_codex::ttype::get_int;
    use mago_codex::ttype::get_list;
    use mago_codex::ttype::get_string;
    use mago_codex::ttype::union::TUnion;
    use mago_span::Span;
    use mago_word::WordMap;
    use mago_word::word;

    use super::AnalysisArtifacts;
    use crate::context::block::BlockContext;
    use crate::context::block::ReferenceConstraint;
    use crate::context::block::ReferenceConstraintSource;

    fn static_context(variable_type: Rc<TUnion>) -> BlockContext<'static> {
        let mut context = BlockContext::new(ScopeContext::new(ReferenceOrigin::File(word("static.php"))), false);
        context.static_locals.insert(word("$state"));
        context.locals.insert(word("$state"), variable_type);
        context
    }

    #[test]
    fn static_local_records_share_unchanged_types_and_keep_independent_snapshots() {
        let variable = word("$state");
        let original = Rc::new(get_list(get_int()));
        let mut context = static_context(Rc::clone(&original));
        let mut artifacts = AnalysisArtifacts::new();
        artifacts.static_local_types = Some(WordMap::default());
        let codebase = CodebaseMetadata::new();
        let options = CombinerOptions::default();

        artifacts.record_static_local_types(&context, &codebase, options);
        assert!(Rc::ptr_eq(&artifacts.static_local_types.as_ref().unwrap()[&variable], &original));
        let saved = artifacts.clone();

        artifacts.record_static_local_types(&context, &codebase, options);
        assert!(Rc::ptr_eq(&artifacts.static_local_types.as_ref().unwrap()[&variable], &original));
        assert!(Rc::ptr_eq(&saved.static_local_types.as_ref().unwrap()[&variable], &original));

        *Rc::make_mut(context.locals.get_mut(&variable).unwrap()) = get_string();
        artifacts.record_static_local_types(&context, &codebase, options);
        let expected = combine_union_types(&original, &get_string(), &codebase, options);
        let actual = &artifacts.static_local_types.as_ref().unwrap()[&variable];
        assert_eq!(actual.types, expected.types);
        assert_eq!(actual.flags, expected.flags);
        assert!(Rc::ptr_eq(&saved.static_local_types.as_ref().unwrap()[&variable], &original));
        assert_eq!(original.as_ref(), &get_list(get_int()));
    }

    #[test]
    fn equal_static_local_records_keep_the_first_types_flags() {
        let variable = word("$state");
        let mut first = get_list(get_int());
        first.flags.insert(UnionFlags::POPULATED);
        let first = Rc::new(first);
        let mut next = get_list(get_int());
        next.flags.insert(UnionFlags::NULLSAFE_NULL);
        let next = Rc::new(next);
        assert_eq!(first, next);

        let context = static_context(next);
        let mut artifacts = AnalysisArtifacts::new();
        artifacts.static_local_types = Some(WordMap::from_iter([(variable, Rc::clone(&first))]));
        artifacts.record_static_local_types(&context, &CodebaseMetadata::new(), CombinerOptions::default());

        let actual = &artifacts.static_local_types.as_ref().unwrap()[&variable];
        assert!(Rc::ptr_eq(actual, &first));
        assert_eq!(actual.flags, UnionFlags::POPULATED);
    }

    #[test]
    fn explicit_static_constraints_do_not_enter_the_inferred_type_map() {
        let variable = word("$state");
        let mut context = static_context(Rc::new(get_int()));
        context.by_reference_constraints.insert(
            variable,
            ReferenceConstraint::new(Span::dummy(0, 0), ReferenceConstraintSource::Static, Some(Rc::new(get_int()))),
        );
        let mut artifacts = AnalysisArtifacts::new();
        artifacts.static_local_types = Some(WordMap::default());
        artifacts.record_static_local_types(&context, &CodebaseMetadata::new(), CombinerOptions::default());
        assert!(artifacts.static_local_types.as_ref().unwrap().is_empty());
    }
}
