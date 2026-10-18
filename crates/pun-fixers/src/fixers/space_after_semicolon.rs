//! Port of `PhpCsFixer\Fixer\Semicolon\SpaceAfterSemicolonFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct SpaceAfterSemicolonFixer {
    remove_in_empty_for_expressions: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(SpaceAfterSemicolonFixer::default())
}

impl Fixer for SpaceAfterSemicolonFixer {
    fn name(&self) -> &'static str {
        "space_after_semicolon"
    }
    fn priority(&self) -> i32 {
        -1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(";")
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["remove_in_empty_for_expressions"])?;
        self.remove_in_empty_for_expressions = o.bool("remove_in_empty_for_expressions", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut inside_for_parentheses_until: Option<usize> = None;
        let mut max = tokens.count() - 1;
        let mut index = 0;
        while index < max {
            if self.remove_in_empty_for_expressions {
                if tokens[index].is_kind(T_FOR) {
                    index = tokens.get_next_meaningful_token(index).unwrap();
                    inside_for_parentheses_until =
                        Some(tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index));
                    index += 1;
                    continue;
                }
                if Some(index) == inside_for_parentheses_until {
                    inside_for_parentheses_until = None;
                    index += 1;
                    continue;
                }
            }

            if !tokens[index].equals(";", true) {
                index += 1;
                continue;
            }

            if !tokens[index + 1].is_whitespace(None) {
                if !tokens[index + 1].equals_any(&[")".into(), T_INLINE_HTML.into()], true)
                    && (!self.remove_in_empty_for_expressions
                        || !tokens[index + 1].equals(";", true))
                {
                    tokens.insert_at(index + 1, vec![Token::new(T_WHITESPACE, " ")]);
                    max += 1;
                }
                index += 1;
                continue;
            }

            if let Some(until) = inside_for_parentheses_until
                && (tokens[index + 2].equals(";", true) || index + 2 == until)
                && !Preg::r#match("/\\R/", &tokens[index + 1].content)
            {
                tokens.clear_at(index + 1);
                index += 1;
                continue;
            }

            if index + 2 < tokens.count()
                && !tokens[index + 1].equals((T_WHITESPACE, " "), true)
                && tokens[index + 1].is_whitespace(Some(b" \t"))
                && !tokens[index + 2].is_comment()
                && !tokens[index + 2].equals(")", true)
            {
                tokens.set(index + 1, Token::new(T_WHITESPACE, " "));
            }
            index += 1;
        }
    }
}
