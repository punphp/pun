//! Port of `PhpCsFixer\Tokenizer\Analyzer\ReferenceAnalyzer`.
use pun_tokens::{Kind::*, Tokens};

pub struct ReferenceAnalyzer;

impl ReferenceAnalyzer {
    pub fn is_reference(&self, tokens: &Tokens, index: usize) -> bool {
        if tokens[index].is_kind(CT_RETURN_REF) {
            return true;
        }
        if !tokens[index].equals("&", true) {
            return false;
        }
        let mut index = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[index].equals_any(&["=".into(), T_AS.into(), T_CALLABLE.into(), T_DOUBLE_ARROW.into(), CT_ARRAY_TYPEHINT.into()], true) {
            return true;
        }
        if tokens[index].is_kind(T_STRING) {
            index = tokens.get_prev_meaningful_token(index).unwrap();
        }
        tokens[index].equals_any(&["(".into(), ",".into(), T_NS_SEPARATOR.into(), CT_NULLABLE_TYPE.into()], true)
    }
}
