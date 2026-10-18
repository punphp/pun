//! Port of `PhpCsFixer\Tokenizer\Analyzer\SwitchAnalyzer`.
use super::control_case_structures_analyzer::ControlCaseStructuresAnalyzer;
use pun_tokens::{Token, Tokens};
use std::cell::RefCell;

thread_local! {
    /// Upstream caches per collection hash (code, count and token ids); we keep the last collection's
    /// tokens + result, so a collection with the same code but other indices (cleared tokens) misses.
    static CACHE: RefCell<Option<(Vec<Token>, Vec<usize>)>> = const { RefCell::new(None) };
}

pub struct SwitchAnalyzer;

impl SwitchAnalyzer {
    pub fn belongs_to_switch(tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].equals(":", true) {
            return false;
        }
        CACHE.with(|c| {
            let mut c = c.borrow_mut();
            if !c
                .as_ref()
                .is_some_and(|(cached, _)| cached[..] == *tokens.as_slice())
            {
                *c = Some((
                    tokens.as_slice().to_vec(),
                    Self::get_colon_indices_for_switch(tokens),
                ));
            }
            c.as_ref().unwrap().1.contains(&index)
        })
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
