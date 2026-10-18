//! Port of `PhpCsFixer\Tokenizer\Analyzer\SwitchAnalyzer`.
use super::control_case_structures_analyzer::ControlCaseStructuresAnalyzer;
use pun_tokens::Tokens;

pub struct SwitchAnalyzer;

impl SwitchAnalyzer {
    pub fn belongs_to_switch(tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].equals(":", true) {
            return false;
        }
        // Upstream caches per collection hash; Tokens keeps the indices until the next edit, sorted for the lookup.
        tokens
            .switch_colon_indices(|t| {
                let mut indices = Self::get_colon_indices_for_switch(t);
                indices.sort_unstable();
                indices
            })
            .binary_search(&index)
            .is_ok()
    }

    fn get_colon_indices_for_switch(tokens: &Tokens) -> Vec<usize> {
        let mut colon_indices = Vec::new();
        for analysis in ControlCaseStructuresAnalyzer::find_switch_structures(tokens) {
            if tokens[analysis.get_open_index()].equals(":", true) {
                colon_indices.push(analysis.get_open_index());
            }
            for case in analysis.get_cases() {
                colon_indices.push(case.get_colon_index());
            }
            if let Some(default_analysis) = analysis.get_default_analysis() {
                colon_indices.push(default_analysis.get_colon_index());
            }
        }
        colon_indices
    }
}
