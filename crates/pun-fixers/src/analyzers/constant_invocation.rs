//! Port of `PhpCsFixer\Tokenizer\TokensAnalyzer::isConstantInvocation` (lives here, not in pun-tokens,
//! because it needs AttributeAnalyzer and GotoLabelAnalyzer).
use super::attribute_analyzer::AttributeAnalyzer;
use super::goto_label_analyzer::GotoLabelAnalyzer;
use pun_tokens::{Kind::*, Tokens, token::CLASSY_KINDS};

pub fn is_constant_invocation(tokens: &Tokens, index: usize) -> bool {
    assert!(tokens[index].is_kind(T_STRING), "No T_STRING at given index {index}, got \"{:?}\".", tokens[index].get_name());
    let prev = |i: usize| tokens.get_prev_meaningful_token(i).unwrap();
    let next_index = tokens.get_next_meaningful_token(index).unwrap();
    if tokens[next_index].equals_any(&["(".into(), "{".into()], true)
        || tokens[next_index].is_given_kind(&[T_DOUBLE_COLON, T_ELLIPSIS, T_NS_SEPARATOR, CT_RETURN_REF, CT_TYPE_ALTERNATION, CT_TYPE_INTERSECTION, T_VARIABLE])
    {
        return false;
    }
    // handle foreach( FOO as $_ ) {}
    if tokens[next_index].is_kind(T_AS) && !tokens[prev(index)].equals("(", true) {
        return false;
    }
    let mut prev_index = prev(index);
    if tokens[prev_index].is_given_kind(CLASSY_KINDS) {
        return false;
    }
    if tokens[prev_index].is_given_kind(&[T_AS, T_CONST, T_DOUBLE_COLON, T_FUNCTION, T_GOTO, CT_GROUP_IMPORT_BRACE_OPEN, CT_TYPE_COLON, CT_TYPE_ALTERNATION, CT_TYPE_INTERSECTION])
        || tokens[prev_index].is_object_operator()
    {
        return false;
    }
    if tokens[prev_index].is_kind(T_CASE) && tokens.is_all_token_kinds_found(&[T_ENUM]) {
        let enum_switch_index = tokens.get_prev_token_of_kind(index, &[T_SWITCH.into(), T_ENUM.into()], true).unwrap();
        if !tokens[enum_switch_index].is_kind(T_SWITCH) {
            return false;
        }
    }
    while tokens[prev_index].is_given_kind(&[CT_NAMESPACE_OPERATOR, T_NS_SEPARATOR, T_STRING, CT_ARRAY_TYPEHINT]) {
        prev_index = prev(prev_index);
    }
    if tokens[prev_index].is_given_kind(&[
        CT_CONST_IMPORT, T_EXTENDS, CT_FUNCTION_IMPORT, T_IMPLEMENTS, T_INSTANCEOF, T_INSTEADOF, T_NAMESPACE, T_NEW, CT_NULLABLE_TYPE,
        CT_TYPE_COLON, T_USE, CT_USE_TRAIT, CT_TYPE_INTERSECTION, CT_TYPE_ALTERNATION, T_CONST, CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
    ]) {
        return false;
    }
    // `FOO & $bar` could be a function reference parameter or a bit operator
    if tokens[next_index].equals("&", true) && tokens[tokens.get_next_meaningful_token(next_index).unwrap()].is_kind(T_VARIABLE) {
        let check_index = tokens
            .get_prev_token_of_kind(prev_index, &[";".into(), "{".into(), "}".into(), T_FUNCTION.into(), T_OPEN_TAG.into(), T_OPEN_TAG_WITH_ECHO.into()], true)
            .unwrap();
        if tokens[check_index].is_kind(T_FUNCTION) {
            return false;
        }
    }
    // check for `extends`/`implements`/`use` list
    if tokens[prev_index].equals(",", true) {
        let mut check_index = prev_index;
        while tokens[check_index].equals_any(&[",".into(), T_AS.into(), CT_NAMESPACE_OPERATOR.into(), T_NS_SEPARATOR.into(), T_STRING.into()], true) {
            check_index = prev(check_index);
        }
        if tokens[check_index].is_given_kind(&[T_EXTENDS, CT_GROUP_IMPORT_BRACE_OPEN, T_IMPLEMENTS, T_USE, CT_USE_TRAIT]) {
            return false;
        }
    }
    // check for array in double quoted string: `"..$foo[bar].."`
    if tokens[prev_index].equals("[", true) && tokens[next_index].equals("]", true) {
        let check_token = &tokens[tokens.get_next_meaningful_token(next_index).unwrap()];
        if check_token.equals("\"", true) || check_token.is_given_kind(&[T_CURLY_OPEN, T_DOLLAR_OPEN_CURLY_BRACES, T_ENCAPSED_AND_WHITESPACE, T_VARIABLE]) {
            return false;
        }
    }
    // check for attribute: `#[Foo]`
    if AttributeAnalyzer::is_attribute(tokens, index) {
        return false;
    }
    // check for goto label
    if tokens[next_index].equals(":", true) && GotoLabelAnalyzer.belongs_to_go_to_label(tokens, next_index) {
        return false;
    }
    // check for non-capturing catches
    while tokens[prev_index].is_given_kind(&[CT_NAMESPACE_OPERATOR, T_NS_SEPARATOR, T_STRING, CT_TYPE_ALTERNATION]) {
        prev_index = prev(prev_index);
    }
    if tokens[prev_index].equals("(", true) {
        let prev_prev_index = prev(prev_index);
        if tokens[prev_prev_index].is_kind(T_CATCH) {
            return false;
        }
    }
    true
}
