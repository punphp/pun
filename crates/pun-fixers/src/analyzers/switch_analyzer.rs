//! Port of `PhpCsFixer\Tokenizer\Analyzer\SwitchAnalyzer`.
use super::control_case_structures_analyzer::ControlCaseStructuresAnalyzer;
use pun_tokens::Tokens;
use std::cell::RefCell;

thread_local! {
    /// Upstream caches per collection hash (the code); we keep the last collection's code + result.
    static CACHE: RefCell<Option<(Vec<u8>, Vec<usize>)>> = const { RefCell::new(None) };
}

pub struct SwitchAnalyzer;

fn same_code(tokens: &Tokens, code: &[u8]) -> bool {
    let mut at = 0;
    for t in tokens {
        let end = at + t.content.len();
        if end > code.len() || code[at..end] != t.content[..] {
            return false;
        }
        at = end;
    }
    at == code.len()
}

impl SwitchAnalyzer {
    pub fn belongs_to_switch(tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].equals(":", true) {
            return false;
        }
        CACHE.with(|c| {
            let mut c = c.borrow_mut();
            if !c.as_ref().is_some_and(|(code, _)| same_code(tokens, code)) {
                *c = Some((tokens.generate_code(), Self::get_colon_indices_for_switch(tokens)));
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
