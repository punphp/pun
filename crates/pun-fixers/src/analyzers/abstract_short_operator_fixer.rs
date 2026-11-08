//! Port of `PhpCsFixer\Fixer\AbstractShortOperatorFixer`: the shared `applyFix` of the fixers that turn
//! `$a = $a <op> $b` into `$a <op>= $b`.
use super::alternative_syntax_analyzer::AlternativeSyntaxAnalyzer;
use super::range_analyzer::{Range, range_equals_range};
use pun_tokens::{BlockType, Kind::*, Proto, Token, Tokens};

pub(crate) trait ShortOperator {
    fn get_replacement_token(&self, token: &Token) -> Token;
    fn is_operator_token_candidate(&self, tokens: &Tokens, index: usize) -> bool;
}

pub(crate) fn apply_fix(fixer: &impl ShortOperator, tokens: &mut Tokens) {
    let alternative_syntax_analyzer = AlternativeSyntaxAnalyzer;
    let mut index = tokens.count();
    while index > 4 {
        index -= 1;
        if !fixer.is_operator_token_candidate(tokens, index) {
            continue;
        }
        // get what is before the operator
        let before_range = get_before_operator_range(tokens, index);
        let equals_index = tokens
            .get_prev_meaningful_token(before_range.start)
            .unwrap();
        // make sure that before that is '='
        if !tokens[equals_index].equals("=", true) {
            continue;
        }
        // get and check what is before '='
        let assign_range = get_before_operator_range(tokens, equals_index);
        let before_assignment_index = tokens
            .get_prev_meaningful_token(assign_range.start)
            .unwrap();
        if tokens[before_assignment_index].equals(":", true) {
            if !belongs_to_switch_or_alternative_syntax(
                &alternative_syntax_analyzer,
                tokens,
                before_assignment_index,
            ) {
                continue;
            }
        } else if !tokens[before_assignment_index].equals_any(
            &[
                ";".into(),
                "{".into(),
                "}".into(),
                "(".into(),
                ")".into(),
                ",".into(),
                T_OPEN_TAG.into(),
                T_RETURN.into(),
            ],
            true,
        ) {
            continue;
        }
        // check if "assign" and "before" the operator are (functionally) the same
        if range_equals_range(tokens, assign_range, before_range) {
            shorten_operation(
                fixer,
                tokens,
                equals_index,
                index,
                assign_range,
                before_range,
            );
            continue;
        }
        if !is_operator_commutative(&tokens[index]) {
            continue;
        }
        let after_range = get_after_operator_range(tokens, index);
        // check if "assign" and "after" the operator are (functionally) the same
        if !range_equals_range(tokens, assign_range, after_range) {
            continue;
        }
        shorten_operation(
            fixer,
            tokens,
            equals_index,
            index,
            assign_range,
            after_range,
        );
    }
}

fn shorten_operation(
    fixer: &impl ShortOperator,
    tokens: &mut Tokens,
    equals_index: usize,
    operator_index: usize,
    assign_range: Range,
    operator_range: Range,
) {
    let replacement = fixer.get_replacement_token(&tokens[operator_index]);
    tokens.set(equals_index, replacement);
    tokens.clear_token_and_merge_surrounding_whitespace(operator_index);
    clear_meaningful_from_range(tokens, operator_range);
    for i in [equals_index, assign_range.end] {
        let i = tokens.get_non_empty_sibling(i, 1).unwrap();
        if tokens[i].is_whitespace(Some(b" \t")) {
            tokens.set(i, Token::new(T_WHITESPACE, " "));
        } else if !tokens[i].is_whitespace(None) {
            tokens.insert_at(i, vec![Token::new(T_WHITESPACE, " ")]);
        }
    }
}

fn get_after_operator_range(tokens: &Tokens, index: usize) -> Range {
    let mut index = tokens.get_next_meaningful_token(index).unwrap();
    let start = index;
    loop {
        // `is_operator_token_candidate` found the statement's end, so a next token exists (upstream also stops at null).
        let next_index = tokens.get_next_meaningful_token(index).unwrap();
        if tokens[next_index].equals_any(&[";".into(), ",".into(), T_CLOSE_TAG.into()], true) {
            break;
        }
        match Tokens::detect_block_type(&tokens[next_index]) {
            None => index = next_index,
            Some((_, false)) => break,
            Some((block_type, true)) => index = tokens.find_block_end(block_type, next_index),
        }
    }
    Range { start, end: index }
}

fn get_before_operator_range(tokens: &Tokens, index: usize) -> Range {
    // `','` is not a true "block type", but speeds up things
    let block_open_types: Vec<Proto<'static>> = std::iter::once(Proto::Char(b","))
        .chain(BlockType::ALL.iter().map(|&t| Tokens::block_edges(t).0))
        .collect();
    let control_structure_without_braces_types =
        [T_IF, T_ELSE, T_ELSEIF, T_FOR, T_FOREACH, T_WHILE];
    let mut previous_index = tokens.get_prev_meaningful_token(index).unwrap();
    if tokens[previous_index].equals_any(&block_open_types, true) {
        return Range {
            start: index,
            end: index,
        };
    }
    let end = previous_index;
    let mut index = previous_index;
    while tokens[previous_index].equals_any(
        &[
            "$".into(),
            "]".into(),
            ")".into(),
            CT_ARRAY_INDEX_BRACE_CLOSE.into(),
            CT_DYNAMIC_PROP_BRACE_CLOSE.into(),
            CT_DYNAMIC_VAR_BRACE_CLOSE.into(),
            T_NS_SEPARATOR.into(),
            T_STRING.into(),
            T_VARIABLE.into(),
        ],
        true,
    ) {
        if let Some((block_type, _)) = Tokens::detect_block_type(&tokens[previous_index]) {
            let block_start = tokens.find_block_start(block_type, previous_index);
            if tokens[previous_index].equals(")", true)
                && tokens[tokens.get_prev_meaningful_token(block_start).unwrap()]
                    .is_given_kind(&control_structure_without_braces_types)
            {
                break; // we went too far back
            }
            previous_index = block_start;
        }
        index = previous_index;
        previous_index = tokens.get_prev_meaningful_token(previous_index).unwrap();
    }
    if tokens[previous_index].is_kind(T_OBJECT_OPERATOR) {
        index = get_before_operator_range(tokens, previous_index).start;
    } else if tokens[previous_index].is_kind(T_DOUBLE_COLON) {
        let prev = tokens.get_prev_meaningful_token(previous_index).unwrap();
        index = get_before_operator_range(tokens, prev).start;
    }
    Range { start: index, end }
}

/// `$range['end']` must be meaningful!
fn clear_meaningful_from_range(tokens: &mut Tokens, range: Range) {
    let mut i = Some(range.end);
    while let Some(index) = i.filter(|&i| i >= range.start) {
        tokens.clear_token_and_merge_surrounding_whitespace(index);
        i = tokens.get_prev_meaningful_token(index);
    }
}

fn is_operator_commutative(operator_token: &Token) -> bool {
    // `??` is not commutative; note that for arrays in PHP `+` is not commutative
    let commutative = !operator_token.is_kind(T_COALESCE)
        && operator_token.equals_any(&["*".into(), "|".into(), "&".into(), "^".into()], true);
    assert!(
        commutative
            || operator_token.is_kind(T_COALESCE)
            || operator_token.equals_any(
                &["-".into(), "/".into(), ".".into(), "%".into(), "+".into()],
                true
            ),
        "Not supported operator."
    );
    commutative
}

fn belongs_to_switch_or_alternative_syntax(
    alternative_syntax_analyzer: &AlternativeSyntaxAnalyzer,
    tokens: &Tokens,
    index: usize,
) -> bool {
    let candidate = index;
    let index = tokens.get_prev_meaningful_token(candidate).unwrap();
    if tokens[index].is_kind(T_DEFAULT) {
        return true;
    }
    let index = tokens.get_prev_meaningful_token(index).unwrap();
    if tokens[index].is_kind(T_CASE) {
        return true;
    }
    alternative_syntax_analyzer.belongs_to_alternative_syntax(tokens, candidate)
}
