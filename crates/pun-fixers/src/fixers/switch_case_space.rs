//! Port of `PhpCsFixer\Fixer\ControlStructure\SwitchCaseSpaceFixer`.
use crate::analyzers::control_case_structures_analyzer::ControlCaseStructuresAnalyzer;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Tokens};
use std::path::Path;

struct SwitchCaseSpaceFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SwitchCaseSpaceFixer)
}

impl Fixer for SwitchCaseSpaceFixer {
    fn name(&self) -> &'static str {
        "switch_case_space"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_SWITCH)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        'analysis: for analysis in ControlCaseStructuresAnalyzer::find_switch_structures(tokens) {
            if let Some(default) = analysis.get_default_analysis() {
                let index = default.get_index();
                if !tokens[index + 1].is_whitespace(None)
                    || !tokens[index + 2].equals_any(&[":".into(), ";".into()], true)
                {
                    continue 'analysis;
                }
                tokens.clear_at(index + 1);
            }
            for case_analysis in analysis.get_cases() {
                let colon_index = case_analysis.get_colon_index();
                let value_index = tokens.get_prev_non_whitespace(colon_index, None).unwrap();
                // skip if there is no space between the colon and previous token or is space after comment
                if value_index == colon_index - 1 || tokens[value_index].is_comment() {
                    continue;
                }
                tokens.clear_at(value_index + 1);
            }
        }
    }
}
