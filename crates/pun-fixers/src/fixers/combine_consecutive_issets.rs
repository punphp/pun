//! Port of `PhpCsFixer\Fixer\LanguageConstruct\CombineConsecutiveIssetsFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use std::path::Path;

struct CombineConsecutiveIssetsFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(CombineConsecutiveIssetsFixer)
}

const BEFORE_ISSET: [Proto<'static>; 7] = [
    Proto::Char(b"("),
    Proto::Char(b"{"),
    Proto::Char(b";"),
    Proto::Char(b"="),
    Proto::Kind(T_OPEN_TAG),
    Proto::Kind(T_BOOLEAN_AND),
    Proto::Kind(T_BOOLEAN_OR),
];
const AFTER_ISSET: [Proto<'static>; 6] = [
    Proto::Char(b")"),
    Proto::Char(b"}"),
    Proto::Char(b";"),
    Proto::Kind(T_CLOSE_TAG),
    Proto::Kind(T_BOOLEAN_AND),
    Proto::Kind(T_BOOLEAN_OR),
];

impl Fixer for CombineConsecutiveIssetsFixer {
    fn name(&self) -> &'static str {
        "combine_consecutive_issets"
    }
    fn priority(&self) -> i32 {
        4
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_ISSET, T_BOOLEAN_AND])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut token_count = tokens.count();
        let mut index = 1;
        while index < token_count {
            if !tokens[index].is_kind(T_ISSET)
                || !tokens[tokens.get_prev_meaningful_token(index).unwrap()].equals_any(&BEFORE_ISSET, true)
            {
                index += 1;
                continue;
            }
            let isset_info = get_isset_info(tokens, index);
            let mut isset_close_brace_index = isset_info[isset_info.len() - 1]; // ')' token
            // one index after the previous meaningful of ')'
            let mut insert_location = isset_info[isset_info.len() - 2] + 1;
            let mut boolean_and_index = tokens.get_next_meaningful_token(isset_close_brace_index).unwrap();

            while tokens[boolean_and_index].is_kind(T_BOOLEAN_AND) {
                let isset_index = tokens.get_next_meaningful_token(boolean_and_index).unwrap();
                if !tokens[isset_index].is_kind(T_ISSET) {
                    index = isset_index;
                    break;
                }
                let next_isset_info = get_isset_info(tokens, isset_index);
                let next_meaningful_index = tokens.get_next_meaningful_token(next_isset_info[next_isset_info.len() - 1]).unwrap();
                if !tokens[next_meaningful_index].equals_any(&AFTER_ISSET, true) {
                    index = next_meaningful_index;
                    break;
                }
                // clone what we want to move, not the '(' and ')' of the 'isset' statement we're merging
                let mut clones = vec![Token::char(","), Token::new(T_WHITESPACE, " ")];
                clones.extend(next_isset_info[1..next_isset_info.len() - 1].iter().map(|&i| tokens[i].clone()));
                for i in next_isset_info.iter().copied().chain([isset_index, boolean_and_index]) {
                    tokens.clear_token_and_merge_surrounding_whitespace(i);
                }
                let inserted = clones.len();
                tokens.insert_at(insert_location, clones);
                token_count += inserted;
                isset_close_brace_index += inserted;
                insert_location += inserted;
                boolean_and_index = tokens.get_next_meaningful_token(isset_close_brace_index).unwrap();
            }
            index += 1;
        }
    }
}

/// Indices of the meaningful tokens of `isset(...)`, from its '(' to its ')'.
fn get_isset_info(tokens: &Tokens, index: usize) -> Vec<usize> {
    let open_index = tokens.get_next_meaningful_token(index).unwrap();
    let mut brace_open_count = 1;
    let mut indices = vec![open_index];
    let mut i = open_index;
    loop {
        i += 1;
        if tokens[i].is_whitespace(None) || tokens[i].is_comment() {
            continue;
        }
        indices.push(i);
        if tokens[i].equals(")", true) {
            brace_open_count -= 1;
            if brace_open_count == 0 {
                return indices;
            }
        } else if tokens[i].equals("(", true) {
            brace_open_count += 1;
        }
    }
}
