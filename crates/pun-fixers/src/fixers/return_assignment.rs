//! Port of `PhpCsFixer\Fixer\ReturnNotation\ReturnAssignmentFixer`.
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

struct ReturnAssignmentFixer {
    skip_named_var_tags: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(ReturnAssignmentFixer {
        skip_named_var_tags: false,
    })
}

fn prev(tokens: &Tokens, index: usize) -> usize {
    tokens.get_prev_meaningful_token(index).unwrap()
}

fn next(tokens: &Tokens, index: usize) -> usize {
    tokens.get_next_meaningful_token(index).unwrap()
}

impl ReturnAssignmentFixer {
    /// Returns the number (>= 0) of tokens inserted into the collection.
    fn fix_function(
        &self,
        tokens: &mut Tokens,
        function_index: usize,
        function_open_index: usize,
        mut function_close_index: usize,
    ) -> usize {
        let mut inserted = 0;
        let mut candidates = Vec::new();
        let mut is_risky = tokens[next(tokens, function_index)].is_kind(CT_RETURN_REF);
        // go through the function declaration and check if references are passed
        // - check if it will be risky to fix return statements of this function
        if (function_index + 1..function_open_index).any(|i| tokens[i].equals("&", true)) {
            is_risky = true;
        }
        // go through all the tokens of the body of the function:
        // - check if it will be risky to fix return statements of this function
        // - check nested functions; fix when found and update the upper limit + number of inserted token
        // - check for return statements that might be fixed (based on if fixing will be risky, which is only know after analyzing the whole function)
        let mut index = function_open_index + 1;
        while index < function_close_index {
            if tokens[index].is_kind(T_FUNCTION) {
                let nested_function_open_index = tokens
                    .get_next_token_of_kind(index, &["{".into(), ";".into()], true)
                    .unwrap();
                if tokens[nested_function_open_index].equals(";", true) {
                    // abstract function
                    index = nested_function_open_index;
                    continue;
                }
                let nested_function_close_index =
                    tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, nested_function_open_index);
                let tokens_added = self.fix_function(
                    tokens,
                    index,
                    nested_function_open_index,
                    nested_function_close_index,
                );
                index = nested_function_close_index + tokens_added;
                function_close_index += tokens_added;
                inserted += tokens_added;
            }
            let i = index;
            index += 1;
            if is_risky {
                continue; // don't bother to look into anything else than nested functions as the current is risky already
            }
            let token = &tokens[i];
            if token.equals("&", true) {
                is_risky = true;
                continue;
            }
            if token.is_kind(T_RETURN) {
                candidates.push(i);
                continue;
            }
            // test if there is anything in the function body that might
            // change global state or indirect changes (like through references, eval, etc.)
            if token.is_given_kind(&[
                CT_DYNAMIC_VAR_BRACE_OPEN, // "$h = ${$g};" case
                T_EVAL,                    // "$c = eval('return $this;');" case
                T_GLOBAL,
                T_INCLUDE, // loading additional symbols we cannot analyse here
                T_INCLUDE_ONCE,
                T_REQUIRE,
                T_REQUIRE_ONCE,
            ]) || (token.is_kind(T_STATIC) && !tokens[next(tokens, i)].is_kind(T_FUNCTION)) // "static $a" case
                || (token.equals("$", true) && tokens[next(tokens, i)].is_kind(T_VARIABLE)) // "$$a" case
                || TokensAnalyzer::new(tokens).is_super_global(i)
            {
                is_risky = true;
            }
        }
        if is_risky {
            return inserted;
        }
        // fix the candidates in reverse order when applicable
        for &index in candidates.iter().rev() {
            let return_var_index = next(tokens, index);
            if !tokens[return_var_index].is_kind(T_VARIABLE) {
                continue; // example: "return 1;"
            }
            let end_return_var_index = next(tokens, return_var_index);
            if !tokens[end_return_var_index].equals_any(&[";".into(), T_CLOSE_TAG.into()], true) {
                continue; // example: "return $a + 1;"
            }
            let mut assign_var_end_index = prev(tokens, index);
            if !tokens[assign_var_end_index].equals(";", true) {
                continue; // example: "? return $a;"
            }
            // Note: here we are @ "; return $a;" (or "; return $a ? >")
            loop {
                let prev_meaningful = prev(tokens, assign_var_end_index);
                if !tokens[prev_meaningful].equals(")", true) {
                    break;
                }
                assign_var_end_index =
                    tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, prev_meaningful);
            }
            let mut assign_var_operator_index = tokens
                .get_prev_token_of_kind(
                    assign_var_end_index,
                    &[
                        "=".into(),
                        ";".into(),
                        "{".into(),
                        "}".into(),
                        T_OPEN_TAG.into(),
                        T_OPEN_TAG_WITH_ECHO.into(),
                    ],
                    true,
                )
                .unwrap();
            if tokens[assign_var_operator_index].equals("}", true) {
                // test for `anonymous class`, `lambda` and `match`
                let Some(start_index) =
                    is_close_brace_part_of_definition(tokens, assign_var_operator_index)
                else {
                    continue;
                };
                assign_var_operator_index = prev(tokens, start_index);
            }
            if !tokens[assign_var_operator_index].equals("=", true) {
                continue;
            }
            // Note: here we are @ "= [^;{<? ? >] ; return $a;"
            let assign_var_index = prev(tokens, assign_var_operator_index);
            if !tokens[assign_var_index].equals(Proto::from(&tokens[return_var_index]), false) {
                continue;
            }
            // Note: here we are @ "$a = [^;{<? ? >] ; return $a;"
            let before_assign_var_index = prev(tokens, assign_var_index);
            if !tokens[before_assign_var_index]
                .equals_any(&[";".into(), "{".into(), "}".into()], true)
            {
                continue;
            }
            if self.skip_named_var_tags
                && has_named_var_tag(tokens, assign_var_index, function_open_index)
            {
                continue;
            }
            if is_used_in_catch_or_finally(
                tokens,
                return_var_index,
                function_open_index,
                function_close_index,
            ) {
                continue;
            }
            // Note: here we are @ "[;{}] $a = [^;{<? ? >] ; return $a;"
            inserted += simplify_return_statement(
                tokens,
                assign_var_index,
                assign_var_operator_index,
                index,
                end_return_var_index,
            );
        }
        inserted
    }
}

/// Returns the number (>= 0) of tokens inserted into the collection.
fn simplify_return_statement(
    tokens: &mut Tokens,
    assign_var_index: usize,
    assign_var_operator_index: usize,
    return_index: usize,
    return_var_end_index: usize,
) -> usize {
    let mut inserted = 0;
    let original_indent = tokens[assign_var_index - 1]
        .is_whitespace(None)
        .then(|| tokens[assign_var_index - 1].get_content().to_vec());
    // remove the return statement
    if tokens[return_var_end_index].equals(";", true) {
        // do not remove PHP close tags
        tokens.clear_token_and_merge_surrounding_whitespace(return_var_end_index);
    }
    for i in return_index..return_var_end_index {
        clear_if_save(tokens, i);
    }
    // remove no longer needed indentation of the old/remove return statement
    if tokens[return_index - 1].is_whitespace(None) {
        let content = tokens[return_index - 1].get_content();
        let content = match content.iter().rposition(|&b| b == b'\n') {
            None => b" ".to_vec(),
            Some(pos) => content[pos..].to_vec(),
        };
        tokens.set(return_index - 1, Token::new(T_WHITESPACE, content));
    }
    // remove the variable and the assignment
    for i in assign_var_index..=assign_var_operator_index {
        clear_if_save(tokens, i);
    }
    // insert new return statement
    tokens.insert_at(assign_var_index, vec![Token::new(T_RETURN, "return")]);
    inserted += 1;
    // use the original indent of the var assignment for the new return statement
    if let Some(original_indent) = original_indent
        && tokens[assign_var_index - 1].is_whitespace(None)
        && original_indent != tokens[assign_var_index - 1].get_content()
    {
        tokens.set(
            assign_var_index - 1,
            Token::new(T_WHITESPACE, original_indent),
        );
    }
    // remove trailing space after the new return statement which might be added during the cleanup process
    let next_index = tokens.get_non_empty_sibling(assign_var_index, 1).unwrap();
    if !tokens[next_index].is_whitespace(None) {
        tokens.insert_at(next_index, vec![Token::new(T_WHITESPACE, " ")]);
        inserted += 1;
    }
    inserted
}

fn clear_if_save(tokens: &mut Tokens, index: usize) {
    if tokens[index].is_comment() {
        return;
    }
    if tokens[index].is_whitespace(None)
        && tokens[tokens.get_prev_non_whitespace(index, None).unwrap()].is_comment()
    {
        return;
    }
    tokens.clear_token_and_merge_surrounding_whitespace(index);
}

/// The index of the first token of the lambda, anonymous class or match whose body the `}` at
/// `index` closes, or `None` when it closes something else.
fn is_close_brace_part_of_definition(tokens: &Tokens, index: usize) -> Option<usize> {
    let index = tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, index);
    is_open_brace_of_lambda(tokens, index)
        .or_else(|| is_open_brace_of_anonymous_class(tokens, index))
        .or_else(|| is_open_brace_of_match(tokens, index))
}

/// The index of `T_NEW` of an anonymous class whose `{` is at `index`.
fn is_open_brace_of_anonymous_class(tokens: &Tokens, mut index: usize) -> Option<usize> {
    loop {
        index = prev(tokens, index);
        if !tokens[index].equals_any(
            &[
                ",".into(),
                T_STRING.into(),
                T_IMPLEMENTS.into(),
                T_EXTENDS.into(),
                T_NS_SEPARATOR.into(),
            ],
            true,
        ) {
            break;
        }
    }
    if tokens[index].equals(")", true) {
        // skip constructor braces and content within
        index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, index);
        index = prev(tokens, index);
    }
    if !tokens[index].is_kind(T_CLASS) || !TokensAnalyzer::new(tokens).is_anonymous_class(index) {
        return None;
    }
    tokens.get_prev_token_of_kind(index, &[T_NEW.into()], true)
}

/// The index of `T_FUNCTION` or `T_STATIC` of a lambda whose `{` is at `index`.
fn is_open_brace_of_lambda(tokens: &Tokens, index: usize) -> Option<usize> {
    let mut index = prev(tokens, index);
    if !tokens[index].equals(")", true) {
        return None;
    }
    index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, index);
    index = prev(tokens, index);
    if tokens[index].is_kind(CT_USE_LAMBDA) {
        index = tokens
            .get_prev_token_of_kind(index, &[")".into()], true)
            .unwrap();
        index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, index);
        index = prev(tokens, index);
    }
    if tokens[index].is_kind(CT_RETURN_REF) {
        index = prev(tokens, index);
    }
    if !tokens[index].is_kind(T_FUNCTION) {
        return None;
    }
    let static_candidate = prev(tokens, index);
    Some(if tokens[static_candidate].is_kind(T_STATIC) {
        static_candidate
    } else {
        index
    })
}

/// The index of `T_MATCH` of a match whose `{` is at `index`.
fn is_open_brace_of_match(tokens: &Tokens, index: usize) -> Option<usize> {
    if !tokens.is_token_kind_found(T_MATCH) {
        return None;
    }
    let mut index = prev(tokens, index);
    if !tokens[index].equals(")", true) {
        return None;
    }
    index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, index);
    index = prev(tokens, index);
    tokens[index].is_kind(T_MATCH).then_some(index)
}

fn has_named_var_tag(tokens: &Tokens, assign_var_index: usize, function_open_index: usize) -> bool {
    let Some(doc_index) = tokens
        .get_prev_token_of_kind(assign_var_index, &[T_DOC_COMMENT.into()], true)
        .filter(|&i| i > function_open_index)
    else {
        return false;
    };
    let doc = DocBlock::new(tokens[doc_index].get_content());
    doc.get_annotations_of_type(&["var", "psalm-var", "phpstan-var"])
        .iter()
        .any(|a| a.get_variable_name().is_some())
}

fn is_used_in_catch_or_finally(
    tokens: &Tokens,
    return_var_index: usize,
    function_open_index: usize,
    function_close_index: usize,
) -> bool {
    let Some(try_index) = tokens
        .get_prev_token_of_kind(return_var_index, &[T_TRY.into()], true)
        .filter(|&i| i > function_open_index)
    else {
        return false;
    };
    let var = Proto::from(&tokens[return_var_index]);
    let try_open_index = tokens
        .get_next_token_of_kind(try_index, &["{".into()], true)
        .unwrap();
    let try_close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, try_open_index);
    // Find catch or finally; the function's `}` follows the try block
    let mut next_index = next(tokens, try_close_index);
    while tokens[next_index].is_kind(T_CATCH) {
        let catch_open_index = tokens
            .get_next_token_of_kind(next_index, &["{".into()], true)
            .unwrap();
        let catch_close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, catch_open_index);
        // The try statement precedes the return inside the function body, so its catch blocks close before the function.
        debug_assert!(catch_close_index < function_close_index);
        if tokens
            .get_next_token_of_kind(catch_open_index, &[var], true)
            .is_some_and(|i| i < catch_close_index)
        {
            return true;
        }
        next_index = next(tokens, catch_close_index);
    }
    if !tokens[next_index].is_kind(T_FINALLY) {
        return false;
    }
    let finally_index = next_index;
    // Likewise its finally block.
    debug_assert!(finally_index < function_close_index);
    let finally_open_index = tokens
        .get_next_token_of_kind(finally_index, &["{".into()], true)
        .unwrap();
    let finally_close_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, finally_open_index);
    tokens
        .get_next_token_of_kind(finally_open_index, &[var], true)
        .is_some_and(|i| i < finally_close_index)
}

impl Fixer for ReturnAssignmentFixer {
    fn name(&self) -> &'static str {
        "return_assignment"
    }
    fn priority(&self) -> i32 {
        -15
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["skip_named_var_tags"])?;
        self.skip_named_var_tags = c.bool("skip_named_var_tags", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_FUNCTION, T_RETURN, T_VARIABLE])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut token_count = tokens.count();
        let mut index = 1;
        while index < token_count {
            if !tokens[index].is_kind(T_FUNCTION)
                || tokens[next(tokens, index)].is_kind(CT_RETURN_REF)
            {
                index += 1;
                continue;
            }
            let function_open_index = tokens
                .get_next_token_of_kind(index, &["{".into(), ";".into()], true)
                .unwrap();
            if tokens[function_open_index].equals(";", true) {
                // abstract function
                index = function_open_index;
                continue;
            }
            let mut function_close_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, function_open_index);
            let mut total_tokens_added = 0;
            loop {
                let tokens_added =
                    self.fix_function(tokens, index, function_open_index, function_close_index);
                function_close_index += tokens_added;
                total_tokens_added += tokens_added;
                if tokens_added == 0 {
                    break;
                }
            }
            index = function_close_index + 1;
            token_count += total_tokens_added;
        }
    }
}
