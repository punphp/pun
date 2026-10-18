//! Port of `PhpCsFixer\Tokenizer\TokensAnalyzer::isConstantInvocation` (lives here, not in pun-tokens,
//! because it needs AttributeAnalyzer and GotoLabelAnalyzer).
use super::attribute_analyzer::AttributeAnalyzer;
use super::goto_label_analyzer::GotoLabelAnalyzer;
use pun_tokens::{Kind::*, Tokens, token::CLASSY_KINDS};

pub(crate) fn is_constant_invocation(tokens: &Tokens, index: usize) -> bool {
    assert!(
        tokens[index].is_kind(T_STRING),
        "No T_STRING at given index {index}, got \"{}\".",
        tokens[index].get_name().unwrap_or_default()
    );
    let prev = |i: usize| tokens.get_prev_meaningful_token(i).unwrap();
    let next_index = tokens.get_next_meaningful_token(index).unwrap();
    if tokens[next_index].equals_any(&["(".into(), "{".into()], true)
        || tokens[next_index].is_given_kind(&[
            T_DOUBLE_COLON,
            T_ELLIPSIS,
            T_NS_SEPARATOR,
            CT_RETURN_REF,
            CT_TYPE_ALTERNATION,
            CT_TYPE_INTERSECTION,
            T_VARIABLE,
        ])
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
    if tokens[prev_index].is_given_kind(&[
        T_AS,
        T_CONST,
        T_DOUBLE_COLON,
        T_FUNCTION,
        T_GOTO,
        CT_GROUP_IMPORT_BRACE_OPEN,
        CT_TYPE_COLON,
        CT_TYPE_ALTERNATION,
        CT_TYPE_INTERSECTION,
    ]) || tokens[prev_index].is_object_operator()
    {
        return false;
    }
    if tokens[prev_index].is_kind(T_CASE) && tokens.is_all_token_kinds_found(&[T_ENUM]) {
        let enum_switch_index = tokens
            .get_prev_token_of_kind(index, &[T_SWITCH.into(), T_ENUM.into()], true)
            .unwrap();
        if !tokens[enum_switch_index].is_kind(T_SWITCH) {
            return false;
        }
    }
    while tokens[prev_index].is_given_kind(&[
        CT_NAMESPACE_OPERATOR,
        T_NS_SEPARATOR,
        T_STRING,
        CT_ARRAY_TYPEHINT,
    ]) {
        prev_index = prev(prev_index);
    }
    if tokens[prev_index].is_given_kind(&[
        CT_CONST_IMPORT,
        T_EXTENDS,
        CT_FUNCTION_IMPORT,
        T_IMPLEMENTS,
        T_INSTANCEOF,
        T_INSTEADOF,
        T_NAMESPACE,
        T_NEW,
        CT_NULLABLE_TYPE,
        CT_TYPE_COLON,
        T_USE,
        CT_USE_TRAIT,
        CT_TYPE_INTERSECTION,
        CT_TYPE_ALTERNATION,
        T_CONST,
        CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
    ]) {
        return false;
    }
    // `FOO & $bar` could be a function reference parameter or a bit operator
    if tokens[next_index].equals("&", true)
        && tokens[tokens.get_next_meaningful_token(next_index).unwrap()].is_kind(T_VARIABLE)
    {
        let check_index = tokens
            .get_prev_token_of_kind(
                prev_index,
                &[
                    ";".into(),
                    "{".into(),
                    "}".into(),
                    T_FUNCTION.into(),
                    T_OPEN_TAG.into(),
                    T_OPEN_TAG_WITH_ECHO.into(),
                ],
                true,
            )
            .unwrap();
        if tokens[check_index].is_kind(T_FUNCTION) {
            return false;
        }
    }
    // check for `extends`/`implements`/`use` list
    if tokens[prev_index].equals(",", true) {
        let mut check_index = prev_index;
        while tokens[check_index].equals_any(
            &[
                ",".into(),
                T_AS.into(),
                CT_NAMESPACE_OPERATOR.into(),
                T_NS_SEPARATOR.into(),
                T_STRING.into(),
            ],
            true,
        ) {
            check_index = prev(check_index);
        }
        if tokens[check_index].is_given_kind(&[
            T_EXTENDS,
            CT_GROUP_IMPORT_BRACE_OPEN,
            T_IMPLEMENTS,
            T_USE,
            CT_USE_TRAIT,
        ]) {
            return false;
        }
    }
    // check for array in double quoted string: `"..$foo[bar].."`
    if tokens[prev_index].equals("[", true) && tokens[next_index].equals("]", true) {
        let check_token = &tokens[tokens.get_next_meaningful_token(next_index).unwrap()];
        if check_token.equals("\"", true)
            || check_token.is_given_kind(&[
                T_CURLY_OPEN,
                T_DOLLAR_OPEN_CURLY_BRACES,
                T_ENCAPSED_AND_WHITESPACE,
                T_VARIABLE,
            ])
        {
            return false;
        }
    }
    if AttributeAnalyzer::is_attribute(tokens, index) {
        return false;
    }
    if tokens[next_index].equals(":", true)
        && GotoLabelAnalyzer.belongs_to_go_to_label(tokens, next_index)
    {
        return false;
    }
    // check for non-capturing catches (upstream re-skips name tokens and `|` here first; that loop never
    // runs: the loop above already skipped names, and a `|` before them returned false above)
    if tokens[prev_index].equals("(", true) {
        let prev_prev_index = prev(prev_index);
        if tokens[prev_prev_index].is_kind(T_CATCH) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE: &str = r#"<?php
namespace N\M;
use const A\B, C;
use function D\e;
use F\{G, H as I};
#[Attr(X1)]
class K extends L implements M1, M2 {
    use T1, T2 { T1::f insteadof T2; T2::f as g; }
    const Q = 1;
    function f(A1 & $a, int|B2 $b, ?C2 $c): D2 { return Q2 & $a; }
}
enum En: string { case Ca = 'a'; const Z = self::Ca; }
switch ($x) { case CS: break; }
interface J1 extends J2, J3 {}
echo "$a[FOO] $a[$b] {$a[BAR]}", FOO_BAR, new K(), K::class, $o->p, $o?->q, \X\Y, X2\Y2;
foreach (CON as $v) {}
foreach ($v as $k => [CON2]) {}
try {} catch (E1 | E2) {} catch (\E3 $e) {} catch (E4) {}
lbl: goto lbl;
$x = A3 ? B3 : C3;
fn(): E5 => E6;
$y = [Z1 => Z2];
function &rr() {}
class Z { use T3, T4; }
use X1, X2;
"#;

    // Expected: the real TokensAnalyzer::isConstantInvocation on every T_STRING of CODE.
    #[test]
    fn matches_php() {
        let tokens = Tokens::from_code(CODE.as_bytes());
        let got: Vec<(usize, String, bool)> = (0..tokens.count())
            .filter(|&i| tokens[i].is_kind(T_STRING))
            .map(|i| {
                (
                    i,
                    String::from_utf8_lossy(tokens[i].get_content()).into_owned(),
                    is_constant_invocation(&tokens, i),
                )
            })
            .collect();
        let expected: Vec<(usize, String, bool)> = [
            (3, "N", false),
            (5, "M", false),
            (12, "A", false),
            (14, "B", false),
            (17, "C", true),
            (24, "D", false),
            (26, "e", false),
            (31, "F", false),
            (34, "G", false),
            (37, "H", false),
            (41, "I", false),
            (46, "Attr", false),
            (48, "X1", true),
            (54, "K", false),
            (58, "L", false),
            (62, "M1", false),
            (65, "M2", false),
            (71, "T1", false),
            (74, "T2", false),
            (78, "T1", false),
            (80, "f", false),
            (84, "T2", false),
            (87, "T2", false),
            (89, "f", false),
            (93, "g", false),
            (100, "Q", false),
            (109, "f", false),
            (111, "A1", false),
            (118, "int", false),
            (120, "B2", false),
            (126, "C2", false),
            (132, "D2", false),
            (138, "Q2", true),
            (151, "En", false),
            (154, "string", false),
            (160, "Ca", false),
            (169, "Z", false),
            (173, "self", false),
            (175, "Ca", false),
            (190, "CS", true),
            (200, "J1", false),
            (204, "J2", false),
            (207, "J3", false),
            (217, "FOO", false),
            (228, "BAR", true),
            (234, "FOO_BAR", true),
            (239, "K", false),
            (244, "K", false),
            (251, "p", false),
            (256, "q", false),
            (260, "X", false),
            (262, "Y", true),
            (265, "X2", false),
            (267, "Y2", true),
            (273, "CON", true),
            (295, "CON2", true),
            (310, "E1", false),
            (314, "E2", false),
            (324, "E3", false),
            (335, "E4", false),
            (341, "lbl", false),
            (346, "lbl", false),
            (353, "A3", true),
            (357, "B3", true),
            (361, "C3", true),
            (369, "E5", false),
            (373, "E6", true),
            (381, "Z1", true),
            (385, "Z2", true),
            (392, "rr", false),
            (401, "Z", false),
            (407, "T3", false),
            (410, "T4", false),
            (417, "X1", false),
            (420, "X2", false),
        ]
        .into_iter()
        .map(|(i, c, b)| (i, c.to_string(), b))
        .collect();
        assert_eq!(got, expected);
    }

    // PHP: LogicException 'No T_STRING at given index 4, got "".' (a token without an id has a null name).
    #[test]
    #[should_panic(expected = "No T_STRING at given index 4, got \"\".")]
    fn rejects_non_string() {
        is_constant_invocation(&Tokens::from_code(b"<?php new Foo;"), 4);
    }
}
