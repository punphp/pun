//! Port of `PhpCsFixer\AbstractNoUselessElseFixer` (shared by no_useless_else and no_superfluous_elseif).
use pun_tokens::{Kind::*, Tokens};

/// `getPriority()`: should be run before NoWhitespaceInBlankLineFixer, NoExtraBlankLinesFixer,
/// BracesFixer and after NoEmptyStatementFixer.
pub const PRIORITY: i32 = 39;

pub fn is_superfluous_else(tokens: &Tokens, index: usize) -> bool {
    let mut previous_block_start = index;
    loop {
        // Check if all 'if', 'else if ' and 'elseif' blocks above this 'else' always end,
        // if so this 'else' is overcomplete.
        let (start, previous_block_end) = get_previous_block(tokens, previous_block_start);
        previous_block_start = start;

        // short 'if' detection
        let mut previous = previous_block_end;
        if tokens[previous].equals("}", true) {
            previous = tokens.get_prev_meaningful_token(previous).unwrap();
        }
        if !tokens[previous].equals(";", true) // 'if' block doesn't end with semicolon, keep 'else'
            || tokens[tokens.get_prev_meaningful_token(previous).unwrap()].equals("{", true)
        // empty 'if' block, keep 'else'
        {
            return false;
        }

        let candidate_index = tokens.get_prev_token_of_kind(
            previous,
            &[
                ";".into(),
                T_BREAK.into(),
                T_CLOSE_TAG.into(),
                T_CONTINUE.into(),
                T_EXIT.into(),
                T_GOTO.into(),
                T_IF.into(),
                T_RETURN.into(),
                T_THROW.into(),
            ],
            true,
        );
        let Some(candidate_index) = candidate_index.filter(|&c| {
            !tokens[c].equals_any(&[";".into(), T_CLOSE_TAG.into(), T_IF.into()], true)
        }) else {
            return false;
        };

        if tokens[candidate_index].is_kind(T_THROW) {
            let previous_index = tokens.get_prev_meaningful_token(candidate_index).unwrap();
            if !tokens[previous_index].equals_any(&[";".into(), "{".into()], true) {
                return false;
            }
        }

        if is_in_conditional(tokens, candidate_index, previous_block_start)
            || is_in_condition_without_braces(tokens, candidate_index, previous_block_start)
        {
            return false;
        }

        // implicit continue, i.e. delete candidate
        if tokens[previous_block_start].is_kind(T_IF) {
            return true;
        }
    }
}

fn get_previous_block(tokens: &Tokens, index: usize) -> (usize, usize) {
    let close = tokens.get_prev_meaningful_token(index).unwrap();
    let mut previous = close;
    // short 'if' detection
    if tokens[close].equals("}", true) {
        previous = tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, close);
    }
    let mut open = tokens
        .get_prev_token_of_kind(
            previous,
            &[T_IF.into(), T_ELSE.into(), T_ELSEIF.into()],
            true,
        )
        .unwrap();
    if tokens[open].is_kind(T_IF) {
        let else_candidate = tokens.get_prev_meaningful_token(open).unwrap();
        if tokens[else_candidate].is_kind(T_ELSE) {
            open = else_candidate;
        }
    }
    (open, close)
}

fn is_in_conditional(tokens: &Tokens, index: usize, lower_limit_index: usize) -> bool {
    let candidate_index = tokens
        .get_prev_token_of_kind(index, &[")".into(), ";".into(), ":".into()], true)
        .unwrap();
    if tokens[candidate_index].equals(":", true) {
        return true;
    }
    if !tokens[candidate_index].equals(")", true) {
        return false; // token is ';' or close tag
    }
    // token is always ')' here.
    // If it is part of the condition the token is always in, return false.
    // If it is not it is a nested condition so return true
    let open = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, candidate_index);
    tokens
        .get_prev_meaningful_token(open)
        .is_some_and(|p| p > lower_limit_index)
}

fn is_in_condition_without_braces(
    tokens: &Tokens,
    mut index: usize,
    lower_limit_index: usize,
) -> bool {
    loop {
        if tokens[index].is_comment() || tokens[index].is_whitespace(None) {
            index = tokens.get_prev_meaningful_token(index).unwrap();
        }
        let token = &tokens[index];
        if token.is_given_kind(&[T_IF, T_ELSEIF, T_ELSE]) {
            return true;
        }
        if token.equals(";", true) {
            return false;
        }
        if token.equals("{", true) {
            // Upstream steps over a `{` owned by for/foreach/while/do, but walking back from the candidate meets
            // no ")" (else isInConditional held), so this "{" opens the if/elseif/else block itself: not OK.
            return false;
        }
        if token.equals(")", true) {
            let (t, _) = Tokens::detect_block_type(token).unwrap();
            index = tokens.find_block_start(t, index);
            index = tokens.get_prev_meaningful_token(index).unwrap();
        } else {
            index -= 1;
        }
        if index <= lower_limit_index {
            return false;
        }
    }
}
