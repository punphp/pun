//! Port of `PhpCsFixer\Fixer\ControlStructure\SwitchCaseSemicolonToColonFixer`.
use crate::analyzers::control_case_structures_analyzer::ControlCaseStructuresAnalyzer;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

struct SwitchCaseSemicolonToColonFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(SwitchCaseSemicolonToColonFixer)
}

impl Fixer for SwitchCaseSemicolonToColonFixer {
    fn name(&self) -> &'static str {
        "switch_case_semicolon_to_colon"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_SWITCH)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for analysis in ControlCaseStructuresAnalyzer::find_switch_structures(tokens) {
            if let Some(default) = analysis.get_default_analysis() {
                fix_token_if_needed(tokens, default.get_colon_index());
            }
            for case_analysis in analysis.get_cases() {
                fix_token_if_needed(tokens, case_analysis.get_colon_index());
            }
        }
    }
}

fn fix_token_if_needed(tokens: &mut Tokens, index: usize) {
    if tokens[index].equals(";", true) {
        tokens.set(index, Token::char(":"));
    }
}
