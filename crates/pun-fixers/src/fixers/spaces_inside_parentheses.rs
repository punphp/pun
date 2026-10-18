//! Port of `PhpCsFixer\Fixer\Whitespace\SpacesInsideParenthesesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct SpacesInsideParenthesesFixer {
    single: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(SpacesInsideParenthesesFixer::default())
}

impl Fixer for SpacesInsideParenthesesFixer {
    fn name(&self) -> &'static str {
        "spaces_inside_parentheses"
    }
    fn priority(&self) -> i32 {
        3
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("(")
            || tokens.is_token_kind_found(CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["space"])?;
        self.single = o.string("space", "none", &["none", "single"])? == "single";
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let open = ["(".into(), CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN.into()];
        if !self.single {
            for index in 0..tokens.count() {
                if !tokens[index].equals_any(&open, true) {
                    continue;
                }
                let prev_index = tokens.get_prev_meaningful_token(index);
                // ignore parenthesis for T_ARRAY
                if prev_index.is_some_and(|p| tokens[p].is_kind(T_ARRAY)) {
                    continue;
                }
                let (block_type, _) = Tokens::detect_block_type(&tokens[index]).unwrap();
                let end_index = tokens.find_block_end(block_type, index);
                if !tokens[tokens.get_next_non_whitespace(index, None).unwrap()].is_comment() {
                    remove_space_around_token(tokens, index + 1);
                }
                // remove space before closing `)` if it is not `list($a, $b, )` case
                if !tokens[tokens.get_prev_meaningful_token(end_index).unwrap()].equals(",", true) {
                    remove_space_around_token(tokens, end_index - 1);
                }
            }
        } else {
            let mut index = 0;
            while index < tokens.count() {
                if !tokens[index].equals_any(&open, true) {
                    index += 1;
                    continue;
                }
                let (block_type, _) = Tokens::detect_block_type(&tokens[index]).unwrap();
                let end_parenthesis_index = tokens.find_block_end(block_type, index);

                // if not other content than spaces in block remove spaces
                if end_parenthesis_index == index + 2 && tokens[index + 1].get_content() == b" " {
                    remove_space_around_token(tokens, index + 1);
                    index += 1;
                    continue;
                }

                // don't process if the next token is `)`
                let next_meaningful_token_index = tokens.get_next_meaningful_token(index).unwrap();
                if tokens[next_meaningful_token_index].get_content() == b")" {
                    index += 1;
                    continue;
                }

                let after_parenthesis_index =
                    tokens.get_next_non_whitespace(end_parenthesis_index, None);
                if let Some(after) =
                    after_parenthesis_index.filter(|&a| tokens[a].is_kind(CT_USE_LAMBDA))
                {
                    let use_start = tokens
                        .get_next_token_of_kind(after, &["(".into()], true)
                        .unwrap();
                    let use_end = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, use_start);
                    fix_parenthesis_inner_edge(tokens, use_start, use_end);
                }

                fix_parenthesis_inner_edge(tokens, index, end_parenthesis_index);
                index += 1;
            }
        }
    }
}

fn remove_space_around_token(tokens: &mut Tokens, index: usize) {
    let token = &tokens[index];
    if token.is_whitespace(None) && !token.get_content().contains(&b'\n') {
        tokens.clear_at(index);
    }
}

fn fix_parenthesis_inner_edge(tokens: &mut Tokens, start: usize, end: usize) {
    if tokens[end - 1].is_whitespace(None) {
        let content = tokens[end - 1].get_content();
        if content != b" "
            && !content.contains(&b'\n')
            && !tokens[tokens.get_prev_non_whitespace(end - 1, None).unwrap()].is_comment()
        {
            tokens.set(end - 1, Token::new(T_WHITESPACE, " "));
        }
    } else {
        tokens.insert_at(end, vec![Token::new(T_WHITESPACE, " ")]);
    }

    if tokens[start + 1].is_whitespace(None) {
        let content = tokens[start + 1].get_content();
        if content != b" "
            && !content.contains(&b'\n')
            && !tokens[tokens.get_next_non_whitespace(start + 1, None).unwrap()].is_comment()
        {
            tokens.set(start + 1, Token::new(T_WHITESPACE, " "));
        }
    } else {
        tokens.insert_at(start + 1, vec![Token::new(T_WHITESPACE, " ")]);
    }
}
