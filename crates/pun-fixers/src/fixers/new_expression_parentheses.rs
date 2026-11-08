//! Port of `PhpCsFixer\Fixer\Operator\NewExpressionParenthesesFixer`. Pun runs as PHP 8.5, so
//! `PHP_VERSION_ID >= 8_04_00` holds.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{BlockType, Proto, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct NewExpressionParenthesesFixer {
    use_parentheses: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(NewExpressionParenthesesFixer {
        use_parentheses: false,
    })
}

impl Fixer for NewExpressionParenthesesFixer {
    fn name(&self) -> &'static str {
        "new_expression_parentheses"
    }
    fn priority(&self) -> i32 {
        29
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_NEW)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["use_parentheses"])?;
        self.use_parentheses = o.bool("use_parentheses", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() as isize - 3;
        while index > 0 {
            let i = index as usize;
            index -= 1;
            if !tokens[i].is_kind(T_NEW) {
                continue;
            }
            // Upstream returns when `new` ends the file, which PHP's parser rejects before any fixer runs.
            let class_start_index = tokens.get_next_meaningful_token(i).unwrap();

            // anonymous class
            if tokens[class_start_index].is_kind(T_CLASS) {
                let mut next_index = tokens.get_next_meaningful_token(class_start_index).unwrap();
                if tokens[next_index].equals("(", true) {
                    next_index = tokens.find_block_end(BlockType::Parenthesis, next_index);
                } else {
                    next_index = class_start_index;
                }
                let body_start_index = tokens
                    .get_next_token_of_kind(next_index, &["{".into()], true)
                    .unwrap();
                let body_end_index = tokens.find_block_end(BlockType::Brace, body_start_index);
                self.ensure(tokens, i, body_end_index);
                continue;
            }

            // named class
            let Some(class_end_index) = find_class_end_index(tokens, class_start_index) else {
                continue;
            };
            let next_index = tokens.get_next_meaningful_token(class_end_index).unwrap();
            if !tokens[next_index].equals("(", true) {
                // If arguments' parentheses are absent then either this new expression is not further called
                // and does not need parentheses, or we cannot omit its parentheses due to the grammar rules.
                continue;
            }
            let args_end_index = tokens.find_block_end(BlockType::Parenthesis, next_index);
            self.ensure(tokens, i, args_end_index);
        }
    }
}

impl NewExpressionParenthesesFixer {
    fn ensure(&self, tokens: &mut Tokens, expr_start_index: usize, expr_end_index: usize) {
        if self.use_parentheses {
            ensure_wrapped_in_parentheses(tokens, expr_start_index, expr_end_index);
        } else {
            ensure_not_wrapped_in_parentheses(tokens, expr_start_index, expr_end_index);
        }
    }
}

fn is_call_operator(tokens: &Tokens, index: usize) -> bool {
    tokens[index].is_object_operator() || tokens[index].is_kind(T_DOUBLE_COLON)
}

fn ensure_wrapped_in_parentheses(
    tokens: &mut Tokens,
    expr_start_index: usize,
    expr_end_index: usize,
) {
    let prev_index = tokens.get_prev_meaningful_token(expr_start_index).unwrap();
    let next_index = tokens.get_next_meaningful_token(expr_end_index).unwrap();
    if tokens[prev_index].is_kind(CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN)
        && tokens[next_index].is_kind(CT_CLASS_INSTANTIATION_PARENTHESIS_CLOSE)
    {
        return;
    }
    if !is_call_operator(tokens, next_index) {
        return;
    }
    tokens.insert_at(
        expr_start_index,
        vec![Token::new(CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN, "(")],
    );
    tokens.insert_at(
        expr_end_index + 2,
        vec![Token::new(CT_CLASS_INSTANTIATION_PARENTHESIS_CLOSE, ")")],
    );
}

fn ensure_not_wrapped_in_parentheses(
    tokens: &mut Tokens,
    expr_start_index: usize,
    expr_end_index: usize,
) {
    let prev_index = tokens.get_prev_meaningful_token(expr_start_index).unwrap();
    let next_index = tokens.get_next_meaningful_token(expr_end_index).unwrap();
    if !tokens[prev_index].is_kind(CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN)
        || !tokens[next_index].is_kind(CT_CLASS_INSTANTIATION_PARENTHESIS_CLOSE)
    {
        return;
    }
    let operator_index = tokens.get_next_meaningful_token(next_index).unwrap();
    if !is_call_operator(tokens, operator_index) {
        return;
    }
    tokens.clear_token_and_merge_surrounding_whitespace(prev_index);
    tokens.clear_token_and_merge_surrounding_whitespace(next_index);
}

fn find_class_end_index(tokens: &Tokens, mut index: usize) -> Option<usize> {
    // (expression) class name
    if tokens[index].equals("(", true) {
        return Some(tokens.find_block_end(BlockType::Parenthesis, index));
    }

    // regular class name or $variable class name
    let next_tokens: [Proto; 10] = [
        T_STRING.into(),
        T_NS_SEPARATOR.into(),
        CT_NAMESPACE_OPERATOR.into(),
        T_VARIABLE.into(),
        "$".into(),
        CT_DYNAMIC_VAR_BRACE_OPEN.into(),
        "[".into(),
        T_OBJECT_OPERATOR.into(),
        T_NULLSAFE_OBJECT_OPERATOR.into(),
        T_DOUBLE_COLON.into(),
    ];
    if !tokens[index].equals_any(&next_tokens, true) {
        return None;
    }
    while tokens[index].equals_any(&next_tokens, true) {
        if let Some((block_type, _)) = Tokens::detect_block_type(&tokens[index]) {
            index = tokens.find_block_end(block_type, index);
        }
        index = tokens.get_next_meaningful_token(index).unwrap();
    }
    Some(index - 1)
}
