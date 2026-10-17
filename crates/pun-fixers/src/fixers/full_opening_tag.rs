//! Port of `PhpCsFixer\Fixer\PhpTag\FullOpeningTagFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, php_compat::strtolower};
use std::path::Path;

struct FullOpeningTagFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(FullOpeningTagFixer)
}

/// `explode($delimiter, $s)`.
fn explode<'a>(delimiter: &[u8], s: &'a [u8]) -> Vec<&'a [u8]> {
    let mut parts = Vec::new();
    let mut rest = s;
    while let Some(p) = rest.windows(delimiter.len()).position(|w| w == delimiter) {
        parts.push(&rest[..p]);
        rest = &rest[p + delimiter.len()..];
    }
    parts.push(rest);
    parts
}

/// `substr($s, $start, $len)` for non-negative arguments.
fn substr(s: &[u8], start: usize, len: usize) -> &[u8] {
    if start >= s.len() {
        b""
    } else {
        &s[start..(start + len).min(s.len())]
    }
}

impl Fixer for FullOpeningTagFixer {
    fn name(&self) -> &'static str {
        "full_opening_tag"
    }
    fn priority(&self) -> i32 {
        // must run before all Token-based fixers
        98
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let content = tokens.generate_code();
        // replace all <? with <?php to replace all short open tags even without short_open_tag option enabled
        let (new_content, count) = Preg::replace_limit(
            "/<\\?(?:phP|pHp|pHP|Php|PhP|PHp|PHP)?(\\s|$)/",
            "<?php$1",
            &content,
            -1,
        );
        if count == 0 {
            return;
        }
        // the following code is magic to revert previous replacements which should NOT be replaced, for
        // example incorrectly replacing `echo '<? ';` with `echo '<?php ';`
        let mut new_tokens = Tokens::from_code(&new_content);
        let mut tokens_old_content_length = 0;
        for index in 0..new_tokens.count() {
            let token = new_tokens[index].clone();
            if token.is_kind(T_OPEN_TAG) {
                let mut token_content = token.get_content().to_vec();
                let possible_open_content = substr(&content, tokens_old_content_length, 5);
                if strtolower(possible_open_content) != b"<?php" {
                    token_content = b"<? ".to_vec();
                }
                tokens_old_content_length += token_content.len();
                continue;
            }
            let mut len = token.get_content().len();
            if token.is_given_kind(&[
                T_COMMENT,
                T_DOC_COMMENT,
                T_CONSTANT_ENCAPSED_STRING,
                T_ENCAPSED_AND_WHITESPACE,
                T_STRING,
            ]) {
                let mut token_content = Vec::new();
                let mut token_content_length = 0;
                let parts = explode(b"<?php", token.get_content());
                let i_last = parts.len() - 1;
                for (i, part) in parts.iter().enumerate() {
                    token_content.extend_from_slice(part);
                    token_content_length += part.len();
                    if i != i_last {
                        let original_token_content = substr(
                            &content,
                            tokens_old_content_length + token_content_length,
                            5,
                        );
                        if strtolower(original_token_content) == b"<?php" {
                            token_content.extend_from_slice(original_token_content);
                            token_content_length += 5;
                        } else {
                            token_content.extend_from_slice(b"<?");
                            token_content_length += 2;
                        }
                    }
                }
                len = token_content.len();
                new_tokens.set(
                    index,
                    Token {
                        id: token.get_id(),
                        content: token_content,
                    },
                );
            }
            tokens_old_content_length += len;
        }
        let count = tokens.count();
        tokens.override_range(0, count - 1, new_tokens.iter().cloned().collect());
    }
}
