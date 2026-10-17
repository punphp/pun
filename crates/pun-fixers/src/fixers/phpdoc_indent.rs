//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocIndentFixer`.
use crate::analyzers::preg::Preg;
use crate::analyzers::utils::Utils;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, php_compat};
use std::path::Path;

struct PhpdocIndentFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocIndentFixer)
}

impl Fixer for PhpdocIndentFixer {
    fn name(&self) -> &'static str {
        "phpdoc_indent"
    }
    fn priority(&self) -> i32 {
        20
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            let token = tokens[index].clone();
            let next_index = tokens.get_next_meaningful_token(index);
            // skip if there is no next token or if next token is block end `}`
            let Some(next_index) = next_index.filter(|&n| !tokens[n].equals("}", true)) else {
                continue;
            };

            let prev_index = index - 1;
            let prev_token = tokens[prev_index].clone();

            if prev_token.is_kind(T_OPEN_TAG)
                || (prev_token.is_whitespace(Some(b" \t"))
                    && !tokens[index - 2].is_kind(T_OPEN_TAG))
                || prev_token.equals_any(&[";".into(), ",".into(), "{".into(), "(".into()], true)
            {
                continue;
            }

            let indent = if tokens[next_index - 1].is_whitespace(None) {
                Utils::calculate_trailing_whitespace_indent(&tokens[next_index - 1])
            } else {
                Vec::new()
            };

            let new_prev_content = fix_whitespace_before_docblock(&prev_token.content, &indent);

            tokens.set(
                index,
                Token::new(T_DOC_COMMENT, fix_doc_block(&token.content, &indent)),
            );

            if !prev_token.is_whitespace(None) {
                if !indent.is_empty() {
                    tokens.insert_at(index, vec![Token::new(T_WHITESPACE, indent)]);
                }
            } else if !new_prev_content.is_empty() {
                tokens.set(
                    prev_index,
                    Token {
                        id: prev_token.id,
                        content: new_prev_content,
                    },
                );
            } else {
                tokens.clear_at(prev_index);
            }
        }
    }
}

fn fix_doc_block(content: &[u8], indent: &[u8]) -> Vec<u8> {
    let replacement = [indent, b" *"].concat();
    // indent is whitespace only, so it needs no escaping as a preg_replace replacement
    let replaced = Preg::replace("/^\\h*\\*/m", replacement, content);
    php_compat::ltrim_chars(&replaced, php_compat::TRIM_CHARS).to_vec()
}

fn fix_whitespace_before_docblock(content: &[u8], indent: &[u8]) -> Vec<u8> {
    [php_compat::rtrim_chars(content, b" \t"), indent].concat()
}
