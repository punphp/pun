//! Port of `PhpCsFixer\Fixer\ControlStructure\EmptyLoopConditionFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Kind, Proto, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct EmptyLoopConditionFixer {
    while_style: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(EmptyLoopConditionFixer { while_style: false })
}

const TOKEN_LOOP_KINDS: [Kind; 2] = [T_FOR, T_WHILE];

impl Fixer for EmptyLoopConditionFixer {
    fn name(&self) -> &'static str {
        "empty_loop_condition"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["style"])?;
        self.while_style = c.string("style", "while", &["while", "for"])? == "while";
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&TOKEN_LOOP_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let candidate_loop_kinds: &[Kind] = if self.while_style {
            &[T_FOR, T_WHILE]
        } else {
            &[T_WHILE]
        };
        for index in (1..tokens.count()).rev() {
            if tokens[index].is_given_kind(candidate_loop_kinds) {
                let open_index = tokens
                    .get_next_token_of_kind(index, &[Proto::Char(b"(")], true)
                    .unwrap();
                let end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index);
                if self.while_style {
                    fix_loop_while(tokens, index, open_index, end_index);
                } else {
                    fix_loop_for(tokens, index, open_index, end_index);
                }
            }
        }
    }
}

fn while_replacement() -> Vec<Token> {
    vec![
        Token::new(T_WHILE, "while"),
        Token::new(T_WHITESPACE, " "),
        Token::char("("),
        Token::new(T_STRING, "true"),
        Token::char(")"),
    ]
}

fn for_replacement() -> Vec<Token> {
    vec![
        Token::new(T_FOR, "for"),
        Token::char("("),
        Token::char(";"),
        Token::char(";"),
        Token::char(")"),
    ]
}

fn fix_loop_while(tokens: &mut Tokens, index: usize, open_index: usize, end_index: usize) {
    if is_for_loop_with_empty_condition(tokens, index, open_index, end_index) {
        clear_not_comments_in_range(tokens, index, end_index);
        tokens.insert_at(index, while_replacement());
    } else if is_while_loop_with_empty_condition(tokens, index, open_index, end_index)
        && let Some(do_index) = get_do_index(tokens, index)
    {
        let semicolon = tokens.get_next_meaningful_token(end_index).unwrap();
        clear_not_comments_in_range(tokens, index, semicolon); // clear including `;`
        tokens.clear_at(do_index);
        tokens.insert_at(do_index, while_replacement());
    }
}

fn fix_loop_for(tokens: &mut Tokens, index: usize, open_index: usize, end_index: usize) {
    if !is_while_loop_with_empty_condition(tokens, index, open_index, end_index) {
        return;
    }
    match get_do_index(tokens, index) {
        None => {
            clear_not_comments_in_range(tokens, index, end_index);
            tokens.insert_at(index, for_replacement());
        }
        Some(do_index) => {
            let semicolon = tokens.get_next_meaningful_token(end_index).unwrap();
            clear_not_comments_in_range(tokens, index, semicolon); // clear including `;`
            tokens.clear_at(do_index);
            tokens.insert_at(do_index, for_replacement());
        }
    }
}

fn clear_not_comments_in_range(tokens: &mut Tokens, index_start: usize, index_end: usize) {
    for i in index_start..=index_end {
        if !tokens[i].is_comment() {
            tokens.clear_token_and_merge_surrounding_whitespace(i);
        }
    }
}

fn get_do_index(tokens: &Tokens, index: usize) -> Option<usize> {
    let end_index = tokens.get_prev_meaningful_token(index).unwrap();
    if !tokens[end_index].equals("}", true) {
        return None;
    }
    let start_index = tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, end_index);
    let index = tokens.get_prev_meaningful_token(start_index)?;
    tokens[index].is_kind(T_DO).then_some(index)
}

fn is_for_loop_with_empty_condition(
    tokens: &Tokens,
    index: usize,
    open_index: usize,
    end_index: usize,
) -> bool {
    if !tokens[index].is_kind(T_FOR) {
        return false;
    }
    let index = tokens.get_next_meaningful_token(open_index).unwrap();
    if !tokens[index].equals(";", true) {
        return false;
    }
    let index = tokens.get_next_meaningful_token(index).unwrap();
    tokens[index].equals(";", true) && Some(end_index) == tokens.get_next_meaningful_token(index)
}

fn is_while_loop_with_empty_condition(
    tokens: &Tokens,
    index: usize,
    open_index: usize,
    end_index: usize,
) -> bool {
    if !tokens[index].is_kind(T_WHILE) {
        return false;
    }
    let index = tokens.get_next_meaningful_token(open_index).unwrap();
    tokens[index].equals((T_STRING, "true"), true)
        && Some(end_index) == tokens.get_next_meaningful_token(index)
}
