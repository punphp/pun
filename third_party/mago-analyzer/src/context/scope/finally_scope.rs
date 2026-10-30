use mago_span::Span;
use crate::context::block::VariableScope;

#[derive(Clone, Debug)]
pub(crate) struct FinallyExit {
    pub span: Span,
    pub target: Option<usize>,
    pub scope: VariableScope,
}

#[derive(Clone, Debug)]
pub struct FinallyScope {
    pub(crate) break_depth: usize,
    pub(crate) exits: Vec<FinallyExit>,
}

impl FinallyScope {
    pub fn new() -> Self {
        Self { break_depth: 0, exits: Vec::new() }
    }

    /// A break inside this try is not an exit through its finally block.
    pub(crate) fn remember(&mut self, span: Span, target: Option<usize>, scope: VariableScope) {
        if !scope.state.impossible && target.is_none_or(|target| target < self.break_depth) {
            self.exits.push(FinallyExit { span, target, scope });
        }
    }
}

impl Default for FinallyScope {
    fn default() -> Self {
        Self::new()
    }
}
