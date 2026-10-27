use std::rc::Rc;

use mago_word::WordMap;
use mago_word::WordSet;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::union::TUnion;
use mago_span::Span;

use crate::context::scope::control_action::ControlActionSet;
use crate::context::block::VariableScope;

#[derive(Clone, Debug)]
pub struct LoopScope {
    pub span: Span,
    pub iteration_count: usize,
    /// Product of constant-array entries on the speculative foreach chain.
    pub foreach_unroll_factor: usize,
    pub parent_context_variables: WordMap<Rc<TUnion>>,
    pub redefined_loop_variables: WordMap<Rc<TUnion>>,
    pub possibly_redefined_loop_variables: WordMap<Rc<TUnion>>,
    pub possibly_redefined_loop_parent_variables: WordMap<Rc<TUnion>>,
    pub possibly_defined_loop_parent_variables: WordMap<Rc<TUnion>>,
    /// Actual states at loop exits, separate from the back edge and the entry state.
    pub break_locals: Option<WordMap<Rc<TUnion>>>,
    pub break_scopes: Vec<VariableScope>,
    pub continue_locals: Option<WordMap<Rc<TUnion>>>,
    pub by_reference_loop_mutations: WordMap<Rc<TUnion>>,
    pub assignment_targets: WordSet,
    pub tracks_assignment_targets: bool,
    pub variables_possibly_in_scope: WordSet,
    pub final_actions: ControlActionSet,
    pub truthy_pre_conditions: bool,
    pub condition_always_false: bool,
    /// Actual first header evaluation, before truthy reconciliation; for/while only.
    pub entry_scope: Option<VariableScope>,
    pub first_falsey_scope: Option<VariableScope>,
    pub pre_update_scope: Option<VariableScope>,
    pub back_edge_dead: bool,
    pub parent_loop: Option<Box<LoopScope>>,
}

impl LoopScope {
    pub fn new(span: Span, parent_context_vars: WordMap<Rc<TUnion>>, parent_loop: Option<Box<LoopScope>>) -> Self {
        Self {
            span,
            parent_context_variables: parent_context_vars,
            iteration_count: 0,
            foreach_unroll_factor: 1,
            redefined_loop_variables: WordMap::default(),
            possibly_redefined_loop_variables: WordMap::default(),
            possibly_redefined_loop_parent_variables: WordMap::default(),
            possibly_defined_loop_parent_variables: WordMap::default(),
            break_locals: None,
            break_scopes: Vec::new(),
            continue_locals: None,
            by_reference_loop_mutations: WordMap::default(),
            assignment_targets: WordSet::default(),
            tracks_assignment_targets: false,
            final_actions: ControlActionSet::new(),
            variables_possibly_in_scope: WordSet::default(),
            parent_loop,
            truthy_pre_conditions: true,
            condition_always_false: false,
            entry_scope: None,
            first_falsey_scope: None,
            pre_update_scope: None,
            back_edge_dead: false,
        }
    }

    pub fn with_parent_loop(self, parent_loop: Option<Box<LoopScope>>) -> Self {
        Self { parent_loop, ..self }
    }
}

/// ScopeOps::mergeVariableHolders(): a local absent on one incoming edge is only possibly defined.
/// Unlike the loop's mutation maps, this retains both new locals and their incoming-edge certainty.
pub(crate) fn merge_exit_locals(
    merged: &mut Option<WordMap<Rc<TUnion>>>,
    incoming: &WordMap<Rc<TUnion>>,
    codebase: &CodebaseMetadata,
) {
    let Some(existing) = merged else {
        *merged = Some(incoming.clone());
        return;
    };
    for (name, ty) in existing.iter_mut() {
        let joined = match incoming.get(name) {
            Some(other) => combine_union_types(ty, other, codebase, CombinerOptions::default()),
            None => {
                let mut optional = (**ty).clone();
                optional.set_possibly_undefined(true, None);
                optional
            }
        };
        *ty = Rc::new(joined);
    }
    for (name, ty) in incoming {
        if !existing.contains_key(name) {
            let mut optional = (**ty).clone();
            optional.set_possibly_undefined(true, None);
            existing.insert(*name, Rc::new(optional));
        }
    }
}
