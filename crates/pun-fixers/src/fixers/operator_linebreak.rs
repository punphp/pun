//! Port of `PhpCsFixer\Fixer\Operator\OperatorLinebreakFixer`.
use crate::analyzers::alternative_syntax_analyzer::AlternativeSyntaxAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::goto_label_analyzer::GotoLabelAnalyzer;
use crate::analyzers::preg::Preg;
use crate::analyzers::reference_analyzer::ReferenceAnalyzer;
use crate::analyzers::switch_analyzer::SwitchAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct OperatorLinebreakFixer {
    beginning: bool,
    operators: Vec<Proto<'static>>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(OperatorLinebreakFixer {
        beginning: false,
        operators: vec![],
    })
}

const BOOLEAN_OPERATORS: [Proto<'static>; 5] = [
    Proto::Kind(T_BOOLEAN_AND),
    Proto::Kind(T_BOOLEAN_OR),
    Proto::Kind(T_LOGICAL_AND),
    Proto::Kind(T_LOGICAL_OR),
    Proto::Kind(T_LOGICAL_XOR),
];

/// `getNonBooleanOperators()`, with `Token::getObjectOperatorKinds()` at the end.
const NON_BOOLEAN_OPERATORS: [Proto<'static>; 42] = [
    Proto::Char(b"%"),
    Proto::Char(b"&"),
    Proto::Char(b"*"),
    Proto::Char(b"+"),
    Proto::Char(b"-"),
    Proto::Char(b"."),
    Proto::Char(b"/"),
    Proto::Char(b":"),
    Proto::Char(b"<"),
    Proto::Char(b"="),
    Proto::Char(b">"),
    Proto::Char(b"?"),
    Proto::Char(b"^"),
    Proto::Char(b"|"),
    Proto::Kind(T_AND_EQUAL),
    Proto::Kind(T_CONCAT_EQUAL),
    Proto::Kind(T_DIV_EQUAL),
    Proto::Kind(T_DOUBLE_ARROW),
    Proto::Kind(T_IS_EQUAL),
    Proto::Kind(T_IS_GREATER_OR_EQUAL),
    Proto::Kind(T_IS_IDENTICAL),
    Proto::Kind(T_IS_NOT_EQUAL),
    Proto::Kind(T_IS_NOT_IDENTICAL),
    Proto::Kind(T_IS_SMALLER_OR_EQUAL),
    Proto::Kind(T_MINUS_EQUAL),
    Proto::Kind(T_MOD_EQUAL),
    Proto::Kind(T_MUL_EQUAL),
    Proto::Kind(T_OR_EQUAL),
    Proto::Kind(T_DOUBLE_COLON),
    Proto::Kind(T_PLUS_EQUAL),
    Proto::Kind(T_POW),
    Proto::Kind(T_POW_EQUAL),
    Proto::Kind(T_SL),
    Proto::Kind(T_SL_EQUAL),
    Proto::Kind(T_SR),
    Proto::Kind(T_SR_EQUAL),
    Proto::Kind(T_XOR_EQUAL),
    Proto::Kind(T_COALESCE),
    Proto::Kind(T_SPACESHIP),
    Proto::Kind(T_PIPE),
    Proto::Kind(T_OBJECT_OPERATOR),
    Proto::Kind(T_NULLSAFE_OBJECT_OPERATOR),
];

impl Fixer for OperatorLinebreakFixer {
    fn name(&self) -> &'static str {
        "operator_linebreak"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["only_booleans", "position"])?;
        let only_booleans = c.bool("only_booleans", false)?;
        self.beginning = c.string("position", "beginning", &["beginning", "end"])? == "beginning";
        self.operators = BOOLEAN_OPERATORS.to_vec();
        if !only_booleans {
            self.operators.extend_from_slice(&NON_BOOLEAN_OPERATORS);
        }
        Ok(())
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let reference_analyzer = ReferenceAnalyzer;
        let goto_label_analyzer = GotoLabelAnalyzer;
        let alternative_syntax_analyzer = AlternativeSyntaxAnalyzer;

        let mut index = tokens.count();
        while index > 1 {
            index -= 1;
            if !tokens[index].equals_any(&self.operators, false) {
                continue;
            }
            if goto_label_analyzer.belongs_to_go_to_label(tokens, index)
                || reference_analyzer.is_reference(tokens, index)
                || alternative_syntax_analyzer.belongs_to_alternative_syntax(tokens, index)
                || SwitchAnalyzer::belongs_to_switch(tokens, index)
            {
                continue;
            }
            let mut operator_indices = vec![index];
            if tokens[index].equals(":", true) {
                let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
                if tokens[prev_index].equals("?", true) {
                    operator_indices = vec![prev_index, index];
                    index = prev_index;
                }
            }
            self.fix_operator_linebreak(tokens, &operator_indices);
        }
    }
}

impl OperatorLinebreakFixer {
    fn fix_operator_linebreak(&self, tokens: &mut Tokens, operator_indices: &[usize]) {
        let min = operator_indices[0];
        let max = *operator_indices.last().unwrap();
        let index_start = tokens.get_prev_meaningful_token(min).unwrap() + 1;
        let index_end = tokens.get_next_meaningful_token(max).unwrap() - 1;
        if !is_multiline(tokens, index_start, index_end) {
            return; // operator is not surrounded by multiline whitespaces, do not touch it
        }
        if self.beginning {
            if !is_multiline(tokens, max, index_end) {
                return; // operator already is placed correctly
            }
            fix_move_to_the_beginning(tokens, operator_indices);
            return;
        }
        if !is_multiline(tokens, index_start, min) {
            return; // operator already is placed correctly
        }
        fix_move_to_the_end(tokens, operator_indices);
    }
}

fn is_multiline_whitespace(token: &Token) -> bool {
    token.is_whitespace(None) && Preg::r#match(r"/\R/u", token.get_content())
}

fn fix_move_to_the_beginning(tokens: &mut Tokens, operator_indices: &[usize]) {
    let min = operator_indices[0];
    let max = *operator_indices.last().unwrap();
    let prev_index = tokens.get_non_empty_sibling(min, -1).unwrap();
    let next_index = tokens.get_next_meaningful_token(max).unwrap();
    for i in (max + 1..next_index).rev() {
        if is_multiline_whitespace(&tokens[i]) {
            let is_whitespace_before = tokens[prev_index].is_whitespace(None);
            let mut inserts = get_replacements_and_clear(tokens, operator_indices, -1);
            if is_whitespace_before {
                inserts.push(Token::new(T_WHITESPACE, " "));
            }
            tokens.insert_at(next_index, inserts);
            break;
        }
    }
}

fn fix_move_to_the_end(tokens: &mut Tokens, operator_indices: &[usize]) {
    let min = operator_indices[0];
    let max = *operator_indices.last().unwrap();
    let prev_index = tokens.get_prev_meaningful_token(min).unwrap();
    let next_index = tokens.get_non_empty_sibling(max, 1).unwrap();
    for i in prev_index + 1..max {
        if is_multiline_whitespace(&tokens[i]) {
            let is_whitespace_after = tokens[next_index].is_whitespace(None);
            let mut inserts = get_replacements_and_clear(tokens, operator_indices, 1);
            if is_whitespace_after {
                inserts.insert(0, Token::new(T_WHITESPACE, " "));
            }
            tokens.insert_at(prev_index + 1, inserts);
            break;
        }
    }
}

fn get_replacements_and_clear(
    tokens: &mut Tokens,
    indices: &[usize],
    direction: isize,
) -> Vec<Token> {
    indices
        .iter()
        .map(|&index| {
            let clone = tokens[index].clone();
            let neighbour = index.checked_add_signed(direction).unwrap();
            if tokens[neighbour].is_whitespace(None) {
                tokens.clear_at(neighbour);
            }
            tokens.clear_at(index);
            clone
        })
        .collect()
}

fn is_multiline(tokens: &Tokens, index_start: usize, index_end: usize) -> bool {
    (index_start..=index_end).any(|index| tokens[index].get_content().contains(&b'\n'))
}
