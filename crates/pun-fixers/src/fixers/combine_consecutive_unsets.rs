//! Port of `PhpCsFixer\Fixer\LanguageConstruct\CombineConsecutiveUnsetsFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct CombineConsecutiveUnsetsFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(CombineConsecutiveUnsetsFixer)
}

impl Fixer for CombineConsecutiveUnsetsFixer {
    fn name(&self) -> &'static str {
        "combine_consecutive_unsets"
    }
    fn priority(&self) -> i32 {
        24
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_UNSET)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() as isize - 1;
        while index >= 0 {
            let i = index as usize;
            index -= 1;
            if !tokens[i].is_kind(T_UNSET) {
                continue;
            }
            let (previous_unset, previous_unset_brace_end) =
                match get_previous_unset_call(tokens, i) {
                    Err(skip_to) => {
                        index = skip_to as isize - 1;
                        continue;
                    }
                    Ok(call) => call,
                };
            // Merge the tokens inside the 'unset' call into the previous one 'unset' call.
            let next_unset_content_start = tokens
                .get_next_token_of_kind(i, &["(".into()], true)
                .unwrap();
            let next_unset_content_end =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, next_unset_content_start);
            let mut tokens_add_count = move_tokens(
                tokens,
                next_unset_content_start,
                next_unset_content_end,
                previous_unset_brace_end - 1,
            );

            if !tokens[previous_unset_brace_end].is_whitespace(None) {
                tokens.insert_at(
                    previous_unset_brace_end,
                    vec![Token::new(T_WHITESPACE, " ")],
                );
                tokens_add_count += 1;
            }
            let before_brace_end = tokens
                .get_prev_meaningful_token(previous_unset_brace_end)
                .unwrap();
            if !tokens[before_brace_end].equals(",", true) {
                tokens.insert_at(previous_unset_brace_end, vec![Token::char(",")]);
                tokens_add_count += 1;
            } else if tokens[before_brace_end + 1].is_whitespace(None) {
                // keeping trailing comma from previous `unset`, but tokens moved: clean up two whitespace tokens in a row
                tokens.clear_token_and_merge_surrounding_whitespace(before_brace_end + 1);
            }

            // Remove 'unset', '(', ')' and (possibly) ';' from the merged 'unset' call.
            for at in [i, next_unset_content_start, next_unset_content_end] {
                tokens.clear_token_and_merge_surrounding_whitespace(at + tokens_add_count);
            }
            if let Some(semicolon) = tokens.get_next_meaningful_token(next_unset_content_end)
                && tokens[semicolon].equals(";", true)
            {
                tokens.clear_token_and_merge_surrounding_whitespace(semicolon);
            }
            index = previous_unset as isize; // upstream sets `$previousUnset + 1`, then the loop decrements
        }
    }
}

/// `getPreviousUnsetCall`: `Ok((unset, brace end))`, or `Err(index)` to continue the scan from.
fn get_previous_unset_call(tokens: &Tokens, index: usize) -> Result<(usize, usize), usize> {
    // upstream's `null === getPrevMeaningfulToken` exits are unreachable: the open tag (or inline HTML) comes first
    let semicolon = tokens.get_prev_meaningful_token(index).unwrap();
    if !tokens[semicolon].equals(";", true) {
        return Err(semicolon);
    }
    let brace_end = tokens.get_prev_meaningful_token(semicolon).unwrap();
    if !tokens[brace_end].equals(")", true) {
        return Err(brace_end);
    }
    let brace_start = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, brace_end);
    let previous_unset = tokens.get_prev_meaningful_token(brace_start).unwrap();
    if !tokens[previous_unset].is_kind(T_UNSET) {
        return Err(previous_unset);
    }
    Ok((previous_unset, brace_end))
}

/// `moveTokens`. Upstream merges a moved whitespace into a whitespace at `$to + 1`, but that slot always holds the
/// previous call's `)` (each insert lands just before it), so every token is cloned in.
fn move_tokens(tokens: &mut Tokens, start: usize, mut end: usize, mut to: usize) -> usize {
    let mut added = 0;
    let mut i = start + 1;
    while i < end {
        to += 1;
        let clone = tokens[i].clone();
        tokens.insert_at(to, vec![clone]);
        end += 1;
        added += 1;
        tokens.clear_at(i + 1);
        i += 2;
    }
    added
}
