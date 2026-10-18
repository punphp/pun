//! Port of `PhpCsFixer\Tokenizer\Analyzer\ReferenceAnalyzer`. Its only caller (operator_linebreak) passes
//! `'&'` tokens, so upstream's leading `CT::T_RETURN_REF` check is left out.
use pun_tokens::{Kind::*, Proto, Tokens};

pub struct ReferenceAnalyzer;

impl ReferenceAnalyzer {
    pub fn is_reference(&self, tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].equals("&", true) {
            return false;
        }
        let mut index = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[index].equals_any(
            &[
                Proto::Char(b"="),
                Proto::Kind(T_AS),
                Proto::Kind(T_CALLABLE),
                Proto::Kind(T_DOUBLE_ARROW),
                Proto::Kind(CT_ARRAY_TYPEHINT),
            ],
            true,
        ) {
            return true;
        }
        if tokens[index].is_kind(T_STRING) {
            index = tokens.get_prev_meaningful_token(index).unwrap();
        }
        tokens[index].equals_any(
            &[
                Proto::Char(b"("),
                Proto::Char(b","),
                Proto::Kind(T_NS_SEPARATOR),
                Proto::Kind(CT_NULLABLE_TYPE),
            ],
            true,
        )
    }
}
