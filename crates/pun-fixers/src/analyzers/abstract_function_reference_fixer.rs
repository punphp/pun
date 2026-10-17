//! Port of `PhpCsFixer\AbstractFunctionReferenceFixer`: `isCandidate` (`T_STRING` found), `isRisky`
//! (true) and `find()`. Fixers create one `FunctionReferenceFinder` per `fix()` call (it caches a
//! `FunctionsAnalyzer` for that token collection).
use super::functions_analyzer::FunctionsAnalyzer;
use pun_tokens::{Kind::*, Tokens};

pub fn is_candidate(tokens: &Tokens) -> bool {
    tokens.is_token_kind_found(T_STRING)
}

pub const IS_RISKY: bool = true;

#[derive(Default)]
pub struct FunctionReferenceFinder {
    functions_analyzer: FunctionsAnalyzer,
}

impl FunctionReferenceFinder {
    pub fn new() -> Self {
        Self::default()
    }

    /// `[function name index, open parenthesis, close parenthesis]` of the next global call to
    /// `function_name_to_search` (case-insensitive) in `start..=end` (`end` defaults to the token count).
    pub fn find(
        &self,
        function_name_to_search: &str,
        tokens: &Tokens,
        mut start: usize,
        end: Option<usize>,
    ) -> Option<(usize, usize, usize)> {
        // make interface consistent with findSequence
        let end = end.unwrap_or(tokens.count());
        loop {
            // find raw sequence which we can analyse for context
            let matches = tokens.find_sequence(
                &[(T_STRING, function_name_to_search).into(), "(".into()],
                start,
                Some(end),
                |_| false,
            )?;
            let (function_name, open_parenthesis) = (matches[0], matches[1]);
            if !self
                .functions_analyzer
                .is_global_function_call(tokens, function_name)
            {
                start = open_parenthesis;
                continue;
            }
            return Some((
                function_name,
                open_parenthesis,
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis),
            ));
        }
    }
}
