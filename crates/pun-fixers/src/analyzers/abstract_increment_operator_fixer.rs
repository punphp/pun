//! Port of `PhpCsFixer\Fixer\AbstractIncrementOperatorFixer`.
use pun_tokens::Kind::*;
use pun_tokens::Tokens;

/// `AbstractIncrementOperatorFixer::findStart`.
pub(crate) fn find_start(tokens: &Tokens, mut index: usize) -> usize {
    loop {
        index = tokens.get_prev_meaningful_token(index).unwrap();
        if let Some((block_type, false)) = Tokens::detect_block_type(&tokens[index]) {
            index = tokens.find_block_start(block_type, index);
        }
        if tokens[index].equals_any(&["$".into(), T_VARIABLE.into()], true) {
            break;
        }
    }
    let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
    let prev_token = &tokens[prev_index];
    if prev_token.equals("$", true) {
        return find_start(tokens, index);
    }
    if prev_token.is_object_operator() {
        return find_start(tokens, prev_index);
    }
    if prev_token.is_kind(T_DOUBLE_COLON) {
        let prev_prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
        if !tokens[prev_prev_index].is_given_kind(&[T_STATIC, T_STRING]) {
            return find_start(tokens, prev_index);
        }
        index = tokens
            .get_token_not_of_kinds_sibling(prev_index, -1, &[T_NS_SEPARATOR, T_STATIC, T_STRING])
            .unwrap();
        index = tokens.get_next_meaningful_token(index).unwrap();
    }
    index
}
