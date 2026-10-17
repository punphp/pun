//! Port of `PhpCsFixer\Fixer\NamespaceNotation\BlankLinesBeforeNamespaceFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::{Result, bail};
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, php_compat::rtrim};
use serde_json::Value;
use std::path::Path;

struct BlankLinesBeforeNamespaceFixer {
    min_line_breaks: usize,
    max_line_breaks: usize,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(BlankLinesBeforeNamespaceFixer {
        min_line_breaks: 2,
        max_line_breaks: 2,
        whitespaces_config: Default::default(),
    })
}

impl BlankLinesBeforeNamespaceFixer {
    /// Make sure # of line breaks prefixing namespace is within given range.
    fn fix_lines_before_namespace(
        &self,
        tokens: &mut Tokens,
        index: usize,
        expected_min: usize,
        expected_max: usize,
    ) {
        // Let's determine the total numbers of new lines before the namespace and the opening token
        let mut opening_token_index = None;
        let mut preceding_newlines = 0;
        let mut newline_in_opening = false;
        let mut opening_token: Option<Token> = None;
        // `index - i` never underflows: a T_NAMESPACE at index 1 is preceded by the T_OPEN_TAG, which breaks at i = 1.
        for i in 1..=2 {
            let token = &tokens[index - i];
            if token.is_kind(T_OPEN_TAG) {
                opening_token = Some(token.clone());
                opening_token_index = Some(index - i);
                newline_in_opening = token.get_content().contains(&b'\n');
                if newline_in_opening {
                    preceding_newlines += 1;
                }
                break;
            }
            if !token.is_kind(T_WHITESPACE) {
                break;
            }
            preceding_newlines += token.get_content().iter().filter(|&&b| b == b'\n').count();
        }
        if preceding_newlines >= expected_min && preceding_newlines <= expected_max {
            return;
        }
        let previous_index = index - 1;
        let previous = tokens[previous_index].clone();
        if expected_max == 0 {
            if previous.is_whitespace(None) {
                tokens.clear_at(previous_index);
            }
            if newline_in_opening {
                let t = Token::new(
                    T_OPEN_TAG,
                    [rtrim(opening_token.as_ref().unwrap().get_content()), b" "].concat(),
                );
                tokens.set(opening_token_index.unwrap(), t);
            }
            return;
        }
        let line_ending = self.whitespaces_config.get_line_ending();
        let mut newlines_for_whitespace_token = if preceding_newlines >= expected_max {
            expected_max
        } else {
            preceding_newlines.max(expected_min)
        };
        if let Some(opening_token) = &opening_token {
            let new_content = [rtrim(opening_token.get_content()), line_ending.as_bytes()].concat();
            tokens.set(
                opening_token_index.unwrap(),
                Token::new(T_OPEN_TAG, new_content),
            );
            newlines_for_whitespace_token -= 1;
        }
        if newlines_for_whitespace_token == 0 {
            if previous.is_whitespace(None) {
                tokens.clear_at(previous_index);
            }
            return;
        }
        if previous.is_whitespace(None) {
            let content = previous.get_content();
            let content = match content.iter().rposition(|&b| b == b'\n') {
                None => &b""[..],
                Some(pos) => &content[pos + 1..],
            };
            let t = Token::new(
                T_WHITESPACE,
                [
                    line_ending.repeat(newlines_for_whitespace_token).as_bytes(),
                    content,
                ]
                .concat(),
            );
            tokens.set(previous_index, t);
        } else {
            tokens.insert_at(
                index,
                vec![Token::new(
                    T_WHITESPACE,
                    line_ending.repeat(newlines_for_whitespace_token),
                )],
            );
        }
    }
}

impl Fixer for BlankLinesBeforeNamespaceFixer {
    fn name(&self) -> &'static str {
        "blank_lines_before_namespace"
    }
    fn priority(&self) -> i32 {
        // Must run after BlankLineAfterOpeningTagFixer, HeaderCommentFixer.
        -31
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(
            self.name(),
            options,
            &["min_line_breaks", "max_line_breaks"],
        )?;
        let int = |key: &str| -> Result<i64> {
            match r.get(key) {
                None => Ok(2),
                Some(Value::Number(n)) if n.is_i64() => Ok(n.as_i64().unwrap()),
                Some(v) => bail!(
                    "[blank_lines_before_namespace] Invalid configuration: The option \"{key}\" with value {v} is expected to be of type \"int\"."
                ),
            }
        };
        let min = int("min_line_breaks")?;
        if min < 0 {
            bail!(
                "[blank_lines_before_namespace] Option `min_line_breaks` cannot be lower than 0."
            );
        }
        let max = int("max_line_breaks")?;
        if max < 0 {
            bail!(
                "[blank_lines_before_namespace] Option `max_line_breaks` cannot be lower than 0."
            );
        }
        if max < min {
            bail!(
                "[blank_lines_before_namespace] Option `max_line_breaks` cannot have lower value than `min_line_breaks`."
            );
        }
        self.min_line_breaks = min as usize;
        self.max_line_breaks = max as usize;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_NAMESPACE)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.count()).rev() {
            if tokens[index].is_kind(T_NAMESPACE) {
                self.fix_lines_before_namespace(
                    tokens,
                    index,
                    self.min_line_breaks,
                    self.max_line_breaks,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Messages from the real fixer's configure() (PHP-CS-Fixer 3.95).
    #[test]
    fn configure_validation() {
        let err = |c: Value| fixer().configure(&c).unwrap_err().to_string();
        assert_eq!(
            err(json!({"min_line_breaks": -1})),
            "[blank_lines_before_namespace] Option `min_line_breaks` cannot be lower than 0."
        );
        assert_eq!(
            err(json!({"max_line_breaks": -1})),
            "[blank_lines_before_namespace] Option `max_line_breaks` cannot be lower than 0."
        );
        assert_eq!(
            err(json!({"min_line_breaks": 2, "max_line_breaks": 1})),
            "[blank_lines_before_namespace] Option `max_line_breaks` cannot have lower value than `min_line_breaks`."
        );
        assert!(fixer().configure(&json!({"min_line_breaks": "1"})).is_err());
        assert!(fixer().configure(&json!({"min_line_breaks": 1.0})).is_err());
        assert!(
            fixer()
                .configure(&json!({"min_line_breaks": 0, "max_line_breaks": 0}))
                .is_ok()
        );
    }
}
