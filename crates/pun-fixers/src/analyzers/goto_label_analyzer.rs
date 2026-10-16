//! Port of `PhpCsFixer\Tokenizer\Analyzer\GotoLabelAnalyzer`.
use pun_tokens::{Kind::*, Tokens};

pub struct GotoLabelAnalyzer;

impl GotoLabelAnalyzer {
    pub fn belongs_to_go_to_label(&self, tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].equals(":", true) {
            return false;
        }
        let Some(prev) = tokens.get_prev_meaningful_token(index) else { return false };
        if !tokens[prev].is_kind(T_STRING) {
            return false;
        }
        let Some(prev) = tokens.get_prev_meaningful_token(prev) else { return false };
        tokens[prev].equals_any(&[":".into(), ";".into(), "{".into(), "}".into(), T_OPEN_TAG.into()], true)
    }
}
