//! Port of `PhpCsFixer\Fixer\Operator\TernaryOperatorSpacesFixer`.
use crate::analyzers::alternative_syntax_analyzer::AlternativeSyntaxAnalyzer;
use crate::analyzers::goto_label_analyzer::GotoLabelAnalyzer;
use crate::analyzers::switch_analyzer::SwitchAnalyzer;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

struct TernaryOperatorSpacesFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(TernaryOperatorSpacesFixer)
}

impl Fixer for TernaryOperatorSpacesFixer {
    fn name(&self) -> &'static str {
        "ternary_operator_spaces"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("?") && tokens.is_token_kind_found(":")
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let alternative_syntax_analyzer = AlternativeSyntaxAnalyzer;
        let goto_label_analyzer = GotoLabelAnalyzer;
        let mut ternary_operator_indices = Vec::new();

        for index in 0..tokens.count() {
            if !tokens[index].equals_any(&["?".into(), ":".into()], true) {
                continue;
            }
            if SwitchAnalyzer::belongs_to_switch(tokens, index) {
                continue;
            }
            if alternative_syntax_analyzer.belongs_to_alternative_syntax(tokens, index) {
                continue;
            }
            if goto_label_analyzer.belongs_to_go_to_label(tokens, index) {
                continue;
            }
            ternary_operator_indices.push(index);
        }

        for &index in ternary_operator_indices.iter().rev() {
            if tokens[index].equals("?", true) {
                let next_non_whitespace_index =
                    tokens.get_next_non_whitespace(index, None).unwrap();
                if tokens[next_non_whitespace_index].equals(":", true) {
                    // for `$a ?: $b` remove spaces between `?` and `:`
                    tokens.ensure_whitespace_at_index(index + 1, 0, b"");
                } else {
                    // for `$a ? $b : $c` ensure space after `?`
                    ensure_whitespace_existence(tokens, index + 1, true);
                }
                // for `$a ? $b : $c` ensure space before `?`
                ensure_whitespace_existence(tokens, index - 1, false);
                continue;
            }
            // only `:` is left
            // for `$a ? $b : $c` ensure space after `:`
            ensure_whitespace_existence(tokens, index + 1, true);
            let prev = tokens.get_prev_non_whitespace(index, None).unwrap();
            if !tokens[prev].equals("?", true) {
                // for `$a ? $b : $c` ensure space before `:`
                ensure_whitespace_existence(tokens, index - 1, false);
            }
        }
    }
}

fn ensure_whitespace_existence(tokens: &mut Tokens, mut index: usize, after: bool) {
    if tokens[index].is_whitespace(None) {
        if !tokens[index].content.contains(&b'\n') && !tokens[index - 1].is_comment() {
            tokens.set(index, Token::new(T_WHITESPACE, " "));
        }
        return;
    }
    index += if after { 0 } else { 1 };
    tokens.insert_at(index, vec![Token::new(T_WHITESPACE, " ")]);
}
