//! Port of `PhpCsFixer\Fixer\Casing\ConstantCaseFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct ConstantCaseFixer {
    upper: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(ConstantCaseFixer::default())
}

impl Fixer for ConstantCaseFixer {
    fn name(&self) -> &'static str {
        "constant_case"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["case"])?;
        self.upper = c.string("case", "lower", &["upper", "lower"])? == "upper";
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let forbidden_prev_kinds = [
            T_DOUBLE_COLON,
            T_EXTENDS,
            T_IMPLEMENTS,
            T_INSTANCEOF,
            T_NAMESPACE,
            T_NEW,
            T_NS_SEPARATOR,
            T_OBJECT_OPERATOR,
            T_NULLSAFE_OBJECT_OPERATOR,
        ];
        for index in 0..tokens.len() {
            let token = &tokens[index];
            if !token.equals_any(
                &[
                    (T_STRING, "true").into(),
                    (T_STRING, "false").into(),
                    (T_STRING, "null").into(),
                ],
                false,
            ) {
                continue;
            }
            let prev_index = tokens.get_prev_meaningful_token(index).unwrap_or(0);
            if tokens[prev_index].is_given_kind(&forbidden_prev_kinds) {
                continue;
            }
            // Upstream reads `$tokens[null]` (= token 0) when nothing follows; valid code always has `;` or `?>`.
            let next_index = tokens.get_next_meaningful_token(index).unwrap_or(0);
            if tokens[next_index].is_given_kind(&[T_DOUBLE_COLON, T_NS_SEPARATOR])
                || tokens[next_index].equals("=", false)
            {
                continue;
            }
            if tokens[prev_index].is_kind(T_CASE) && tokens[next_index].equals(";", true) {
                continue;
            }
            let token = &tokens[index];
            let content = if self.upper {
                token.content.to_ascii_uppercase()
            } else {
                token.content.to_ascii_lowercase()
            };
            tokens.set(index, Token::new(token.id.unwrap(), content));
        }
    }
}
