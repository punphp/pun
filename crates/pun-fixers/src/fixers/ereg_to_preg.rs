//! Port of `PhpCsFixer\Fixer\Alias\EregToPregFixer`.
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{preg_compile, substr_count};
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct EregToPregFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(EregToPregFixer)
}

/// `FUNCTIONS`: the ereg function, its preg replacement and the modifier it adds.
const FUNCTIONS: &[(&str, &str, &str)] = &[
    ("ereg", "preg_match", ""),
    ("eregi", "preg_match", "i"),
    ("ereg_replace", "preg_replace", ""),
    ("eregi_replace", "preg_replace", "i"),
    ("split", "preg_split", ""),
    ("spliti", "preg_split", "i"),
];

const DELIMITERS: &[u8] = b"/#!";

impl Fixer for EregToPregFixer {
    fn name(&self) -> &'static str {
        "ereg_to_preg"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let end = tokens.count() - 1;
        let functions_analyzer = FunctionsAnalyzer::new();
        for &(function, replacement, modifier) in FUNCTIONS {
            let seq = [
                (T_STRING, function).into(),
                "(".into(),
                T_CONSTANT_ENCAPSED_STRING.into(),
            ];
            let mut curr_index = 0;
            while let Some(found) = tokens.find_sequence(&seq, curr_index, Some(end), |_| false) {
                let (name_index, regex_index) = (found[0], found[2]);
                curr_index = regex_index;
                if !functions_analyzer.is_global_function_call(tokens, name_index) {
                    continue;
                }
                let next = tokens.get_next_meaningful_token(regex_index);
                if !next
                    .is_some_and(|next| tokens[next].equals_any(&[",".into(), ")".into()], true))
                {
                    continue;
                }
                let content = tokens[regex_index].get_content();
                let (quote, prefix, string) = if content[0] == b'b' || content[0] == b'B' {
                    (content[1], &content[..1], &content[2..content.len() - 1])
                } else {
                    (content[0], &b""[..], &content[1..content.len() - 1])
                };
                let delim = get_best_delimiter(string);
                let mut preg = vec![delim];
                for &c in string {
                    if c == delim {
                        preg.push(b'\\');
                    }
                    preg.push(c);
                }
                preg.push(delim);
                preg.push(b'D');
                preg.extend_from_slice(modifier.as_bytes());
                if preg_compile(&String::from_utf8_lossy(&preg)).is_err() {
                    continue;
                }
                let mut literal = prefix.to_vec();
                literal.push(quote);
                literal.extend_from_slice(&preg);
                literal.push(quote);
                tokens.set(name_index, Token::new(T_STRING, replacement));
                tokens.set(regex_index, Token::new(T_CONSTANT_ENCAPSED_STRING, literal));
            }
        }
    }
}

/// The first delimiter not in `pattern`, else the one it holds least often (the earlier one on a tie).
fn get_best_delimiter(pattern: &[u8]) -> u8 {
    if let Some(&d) = DELIMITERS.iter().find(|&&d| !pattern.contains(&d)) {
        return d;
    }
    *DELIMITERS
        .iter()
        .min_by_key(|&&d| substr_count(pattern, &[d]))
        .unwrap()
}
