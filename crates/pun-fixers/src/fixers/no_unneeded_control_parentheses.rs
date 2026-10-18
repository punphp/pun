//! Port of `PhpCsFixer\Fixer\ControlStructure\NoUnneededControlParenthesesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::{BlockType, Proto, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct NoUnneededControlParenthesesFixer {
    statements: Vec<String>,
}

const CONFIG_OPTIONS: &[&str] = &[
    "break",
    "clone",
    "continue",
    "echo_print",
    "negative_instanceof",
    "others",
    "return",
    "switch_case",
    "yield",
    "yield_from",
];
const DEFAULT_STATEMENTS: &[&str] = &[
    "break",
    "clone",
    "continue",
    "echo_print",
    "return",
    "switch_case",
    "yield",
];

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(NoUnneededControlParenthesesFixer::default())
}

const BLOCK_TYPES: &[BlockType] = &[
    Tokens::BLOCK_TYPE_INDEX_BRACE,
    Tokens::BLOCK_TYPE_ARRAY_BRACKET,
    Tokens::BLOCK_TYPE_BRACE,
    Tokens::BLOCK_TYPE_DESTRUCTURING_BRACKET,
    Tokens::BLOCK_TYPE_DYNAMIC_PROP_BRACE,
    Tokens::BLOCK_TYPE_DYNAMIC_VAR_BRACE,
    Tokens::BLOCK_TYPE_INDEX_BRACKET,
    Tokens::BLOCK_TYPE_PARENTHESIS,
];

const BEFORE_TYPES: &[Proto<'static>] = &[
    Proto::Char(b";"),
    Proto::Char(b"{"),
    Proto::Kind(T_OPEN_TAG),
    Proto::Kind(T_OPEN_TAG_WITH_ECHO),
    Proto::Kind(T_ECHO),
    Proto::Kind(T_PRINT),
    Proto::Kind(T_RETURN),
    Proto::Kind(T_THROW),
    Proto::Kind(T_YIELD),
    Proto::Kind(T_YIELD_FROM),
    Proto::Kind(T_BREAK),
    Proto::Kind(T_CONTINUE),
    // won't be fixed, but true in concept, helpful for fast check
    Proto::Kind(T_REQUIRE),
    Proto::Kind(T_REQUIRE_ONCE),
    Proto::Kind(T_INCLUDE),
    Proto::Kind(T_INCLUDE_ONCE),
];

const TOKEN_TYPE_CONFIG_MAP: &[(Kind, &str)] = &[
    (T_BREAK, "break"),
    (T_CASE, "switch_case"),
    (T_CONTINUE, "continue"),
    (T_ECHO, "echo_print"),
    (T_PRINT, "echo_print"),
    (T_RETURN, "return"),
    (T_YIELD, "yield"),
    (T_YIELD_FROM, "yield_from"),
];

// handled by the `include` rule

const KNOWN_NEGATIVE_PRE_TYPES: &[Kind] = &[
    CT_CLASS_CONSTANT,
    CT_DYNAMIC_VAR_BRACE_CLOSE,
    CT_RETURN_REF,
    CT_USE_LAMBDA,
    T_ARRAY,
    T_CATCH,
    T_CLASS,
    T_DECLARE,
    T_ELSEIF,
    T_EMPTY,
    T_EXIT,
    T_EVAL,
    T_FN,
    T_FOREACH,
    T_FOR,
    T_FUNCTION,
    T_HALT_COMPILER,
    T_IF,
    T_ISSET,
    T_LIST,
    T_STRING,
    T_SWITCH,
    T_STATIC,
    T_UNSET,
    T_VARIABLE,
    T_WHILE,
    T_REQUIRE,
    T_REQUIRE_ONCE,
    T_INCLUDE,
    T_INCLUDE_ONCE,
    T_MATCH,
];

const NOOP_KINDS: &[Kind] = &[
    T_CONSTANT_ENCAPSED_STRING,
    T_DNUMBER,
    T_DOUBLE_COLON,
    T_LNUMBER,
    T_NS_SEPARATOR,
    T_STRING,
    T_VARIABLE,
    T_STATIC,
    T_CLASS_C,
    T_DIR,
    T_FILE,
    T_FUNC_C,
    T_LINE,
    T_METHOD_C,
    T_NS_C,
    T_TRAIT_C,
    T_OBJECT_OPERATOR,
    T_NULLSAFE_OBJECT_OPERATOR,
];

fn is_noop(t: &Token) -> bool {
    t.equals("$", true) || t.is_given_kind(NOOP_KINDS)
}

fn next(tokens: &Tokens, i: usize) -> usize {
    tokens.get_next_meaningful_token(i).unwrap()
}
fn prev(tokens: &Tokens, i: usize) -> usize {
    tokens.get_prev_meaningful_token(i).unwrap()
}

impl Fixer for NoUnneededControlParenthesesFixer {
    fn name(&self) -> &'static str {
        "no_unneeded_control_parentheses"
    }
    fn priority(&self) -> i32 {
        30
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(self.name(), options, &["statements"])?;
        self.statements = c.strings("statements", DEFAULT_STATEMENTS, CONFIG_OPTIONS)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("(")
            || tokens.is_token_kind_found(CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut open_index = 0;
        while open_index < tokens.len() {
            self.fix_at(tokens, open_index);
            open_index += 1;
        }
    }
}

impl NoUnneededControlParenthesesFixer {
    fn fix_at(&self, tokens: &mut Tokens, open_index: usize) {
        let close_index = if tokens[open_index].equals("(", true) {
            tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index)
        } else if tokens[open_index].is_kind(CT_CLASS_INSTANTIATION_PARENTHESIS_OPEN) {
            tokens.find_block_end(
                Tokens::BLOCK_TYPE_CLASS_INSTANTIATION_PARENTHESIS,
                open_index,
            )
        } else {
            return;
        };
        let before_open_index = prev(tokens, open_index);
        let after_close_index = next(tokens, close_index);
        if next(tokens, open_index) == close_index {
            if tokens[before_open_index].is_kind(T_EXIT) {
                self.remove_useless_parenthesis_pair(
                    tokens,
                    before_open_index,
                    after_close_index,
                    open_index,
                    close_index,
                    Some("others"),
                );
            }
            return;
        }
        if tokens[before_open_index].is_given_kind(KNOWN_NEGATIVE_PRE_TYPES) {
            return;
        }
        if self.is_useless_wrapped(tokens, before_open_index, after_close_index) {
            let t = get_config_type(tokens, before_open_index);
            self.remove_useless_parenthesis_pair(
                tokens,
                before_open_index,
                after_close_index,
                open_index,
                close_index,
                t,
            );
            return;
        }
        if tokens[before_open_index].is_kind(T_CLONE) {
            if self.is_wrapped_clone_argument(
                tokens,
                before_open_index,
                open_index,
                close_index,
                after_close_index,
            ) {
                self.remove_useless_parenthesis_pair(
                    tokens,
                    before_open_index,
                    after_close_index,
                    open_index,
                    close_index,
                    Some("clone"),
                );
            }
            return;
        }
        let instance_of = tokens.find_given_kind(T_INSTANCEOF, open_index, Some(close_index));
        if instance_of.len() == 1 {
            if self.is_wrapped_instance_of(
                tokens,
                instance_of[0],
                before_open_index,
                open_index,
                close_index,
                after_close_index,
            ) {
                let t = if tokens[before_open_index].equals("!", true) {
                    "negative_instanceof"
                } else {
                    "others"
                };
                self.remove_useless_parenthesis_pair(
                    tokens,
                    before_open_index,
                    after_close_index,
                    open_index,
                    close_index,
                    Some(t),
                );
            }
            return;
        }
        // last checks deal with operators, do not swap around
        if self.is_wrapped_part_of_operation(
            tokens,
            before_open_index,
            open_index,
            close_index,
            after_close_index,
        ) {
            let t = get_config_type(tokens, before_open_index);
            self.remove_useless_parenthesis_pair(
                tokens,
                before_open_index,
                after_close_index,
                open_index,
                close_index,
                t,
            );
        }
    }

    fn is_useless_wrapped(&self, tokens: &Tokens, before: usize, after: usize) -> bool {
        is_single_statement(tokens, before, after)
            || is_wrapped_fn_body(tokens, before, after)
            || is_wrapped_for_element(tokens, before, after)
            || is_wrapped_language_construct_argument(tokens, before, after)
            || is_wrapped_sequence_element(tokens, before, after)
    }

    fn is_wrapped_clone_argument(
        &self,
        tokens: &Tokens,
        before: usize,
        mut open: usize,
        close: usize,
        after: usize,
    ) -> bool {
        let before = prev(tokens, before);
        if !(tokens[before].equals("?", true) // For BC reasons
            || is_simple_assignment(tokens, before, after)
            || is_single_statement(tokens, before, after)
            || is_wrapped_fn_body(tokens, before, after)
            || is_wrapped_for_element(tokens, before, after)
            || is_wrapped_sequence_element(tokens, before, after))
        {
            return false;
        }
        let new_candidate_index = next(tokens, open);
        if tokens[new_candidate_index].is_kind(T_NEW) {
            open = new_candidate_index; // `clone (new X)`, `clone (new X())`, clone (new X(Y))`
        }
        !contains_operation(tokens, open, close)
    }

    fn is_wrapped_instance_of(
        &self,
        tokens: &Tokens,
        instance_of: usize,
        mut before: usize,
        open: usize,
        close: usize,
        after: usize,
    ) -> bool {
        if contains_operation(tokens, open, instance_of)
            || contains_operation(tokens, instance_of, close)
        {
            return false;
        }
        if tokens[before].equals("!", true) {
            before = prev(tokens, before);
        }
        is_simple_assignment(tokens, before, after)
            || is_single_statement(tokens, before, after)
            || is_wrapped_fn_body(tokens, before, after)
            || is_wrapped_for_element(tokens, before, after)
            || is_wrapped_sequence_element(tokens, before, after)
    }

    fn is_wrapped_part_of_operation(
        &self,
        tokens: &Tokens,
        mut before: usize,
        open: usize,
        close: usize,
        mut after: usize,
    ) -> bool {
        if contains_operation(tokens, open, close) {
            return false;
        }
        let ta = TokensAnalyzer::new(tokens);
        let mut boundaries_moved = false;
        if is_pre_unary_operation(tokens, before) {
            before = get_before_pre_unary_operation(tokens, before);
            boundaries_moved = true;
        }
        if is_access(tokens, after) {
            after = get_after_access(tokens, after);
            boundaries_moved = true;
            if ta.is_unary_successor_operator(after) {
                // post unary operation are only valid here
                after = next(tokens, after);
            }
        }
        if boundaries_moved {
            if tokens[before].is_given_kind(KNOWN_NEGATIVE_PRE_TYPES) {
                return false;
            }
            if self.is_useless_wrapped(tokens, before, after) {
                return true;
            }
        }
        let before_is_binary_operation = ta.is_binary_operator(before);
        let after_is_binary_operation = ta.is_binary_operator(after);
        if before_is_binary_operation && after_is_binary_operation {
            return true; // `+ (x) +`
        }
        let before_token = &tokens[before];
        let after_token = &tokens[after];
        let before_is_block_open_or_comma =
            before_token.equals(",", true) || get_block(tokens, before, true).is_some();
        let after_is_block_end_or_comma =
            after_token.equals(",", true) || get_block(tokens, after, false).is_some();
        if (before_is_block_open_or_comma && after_is_binary_operation)
            || (before_is_binary_operation && after_is_block_end_or_comma)
        {
            return true;
        }
        let before_is_statement_open = if before_token.equals("}", true) {
            !close_curly_belongs_to_dynamic_element(tokens, before)
        } else {
            before_token.equals_any(BEFORE_TYPES, true) || before_token.is_kind(T_CASE)
        };
        let after_is_statement_end =
            after_token.equals_any(&[";".into(), T_CLOSE_TAG.into()], true);
        (before_is_statement_open && after_is_binary_operation)
            || (before_is_binary_operation && after_is_statement_end)
    }

    fn remove_useless_parenthesis_pair(
        &self,
        tokens: &mut Tokens,
        before: usize,
        after: usize,
        open: usize,
        close: usize,
        config_type: Option<&str>,
    ) {
        if !config_type.is_some_and(|c| self.statements.iter().any(|s| s == c)) {
            return;
        }
        let needs_space_after = !is_access(tokens, after)
            && !tokens[after].equals_any(&[";".into(), ",".into(), T_CLOSE_TAG.into()], true)
            && get_block(tokens, after, false).is_none()
            && !(tokens[after].equals_any(&[":".into(), ";".into()], true)
                && tokens[before].is_kind(T_CASE));
        let needs_space_before = !is_pre_unary_operation(tokens, before)
            && !tokens[before].equals_any(&["}".into(), T_EXIT.into(), T_OPEN_TAG.into()], true)
            && get_block(tokens, before, true).is_none();
        remove_brace(tokens, close, needs_space_after);
        remove_brace(tokens, open, needs_space_before);
    }
}

// bounded `print|yield|yield from|require|require_once|include|include_once (X)`
fn is_wrapped_language_construct_argument(tokens: &Tokens, before: usize, after: usize) -> bool {
    if !tokens[before].is_given_kind(&[
        T_PRINT,
        T_YIELD,
        T_YIELD_FROM,
        T_REQUIRE,
        T_REQUIRE_ONCE,
        T_INCLUDE,
        T_INCLUDE_ONCE,
    ]) {
        return false;
    }
    is_wrapped_sequence_element(tokens, prev(tokens, before), after)
}

// any of `<?php|<?|<?=|;|throw|return|... (X) ;|T_CLOSE`
fn is_single_statement(tokens: &Tokens, before: usize, after: usize) -> bool {
    if tokens[before].is_kind(T_CASE) {
        return tokens[after].equals_any(&[":".into(), ";".into()], true); // `switch case`
    }
    if !tokens[after].equals_any(&[";".into(), T_CLOSE_TAG.into()], true) {
        return false;
    }
    if tokens[before].equals("}", true) {
        return !close_curly_belongs_to_dynamic_element(tokens, before);
    }
    tokens[before].equals_any(BEFORE_TYPES, true)
}

fn is_simple_assignment(tokens: &Tokens, before: usize, after: usize) -> bool {
    tokens[before].equals("=", true)
        && tokens[after].equals_any(&[";".into(), T_CLOSE_TAG.into()], true)
}

fn is_wrapped_sequence_element(tokens: &Tokens, start: usize, end: usize) -> bool {
    let start_is_comma = tokens[start].equals(",", true);
    let end_is_comma = tokens[end].equals(",", true);
    if start_is_comma && end_is_comma {
        return true; // `,(X),`
    }
    let block_type_start = get_block(tokens, start, true);
    let block_type_end = get_block(tokens, end, false);
    (start_is_comma && block_type_end.is_some())
        || (end_is_comma && block_type_start.is_some())
        || (block_type_end.is_some() && block_type_start.is_some())
}

// any of `for( (X); ;(X)) ;`
fn is_wrapped_for_element(tokens: &Tokens, before: usize, after: usize) -> bool {
    let for_candidate_index = if tokens[before].equals("(", true) && tokens[after].equals(";", true)
    {
        Some(prev(tokens, before))
    } else if tokens[after].equals(")", true) && tokens[before].equals(";", true) {
        Some(prev(
            tokens,
            tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, after),
        ))
    } else {
        None
    };
    for_candidate_index.is_some_and(|i| tokens[i].is_kind(T_FOR))
}

// `fn() => (X);`
fn is_wrapped_fn_body(tokens: &Tokens, before: usize, after: usize) -> bool {
    if !tokens[before].is_kind(T_DOUBLE_ARROW) {
        return false;
    }
    let mut before = prev(tokens, before);
    if tokens[before].is_kind(T_STRING) {
        loop {
            before = prev(tokens, before);
            if !tokens[before].is_given_kind(&[T_STRING, CT_TYPE_INTERSECTION, CT_TYPE_ALTERNATION])
            {
                break;
            }
        }
        if !tokens[before].is_kind(CT_TYPE_COLON) {
            return false;
        }
        before = prev(tokens, before);
    }
    if !tokens[before].equals(")", true) {
        return false;
    }
    before = prev(
        tokens,
        tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, before),
    );
    if tokens[before].is_kind(CT_RETURN_REF) {
        before = prev(tokens, before);
    }
    if !tokens[before].is_kind(T_FN) {
        return false;
    }
    tokens[after].equals_any(&[";".into(), ",".into(), T_CLOSE_TAG.into()], true)
}

fn is_pre_unary_operation(tokens: &Tokens, index: usize) -> bool {
    TokensAnalyzer::new(tokens).is_unary_predecessor_operator(index) || tokens[index].is_cast()
}

fn get_before_pre_unary_operation(tokens: &Tokens, mut index: usize) -> usize {
    loop {
        index = prev(tokens, index);
        if !is_pre_unary_operation(tokens, index) {
            return index;
        }
    }
}

// array access `(X)[` or `(X){` or object access `(X)->` or `(X)?->`
fn is_access(tokens: &Tokens, index: usize) -> bool {
    let token = &tokens[index];
    token.is_object_operator()
        || token.equals("[", true)
        || token.is_kind(CT_ARRAY_INDEX_BRACE_OPEN)
}

fn get_after_access(tokens: &Tokens, mut index: usize) -> usize {
    loop {
        if let Some(bt) = get_block(tokens, index, true) {
            index = next(tokens, tokens.find_block_end(bt, index));
            continue;
        }
        if tokens[index].is_object_operator()
            || tokens[index].equals_any(
                &[
                    "$".into(),
                    T_DOUBLE_COLON.into(),
                    T_STRING.into(),
                    T_VARIABLE.into(),
                ],
                true,
            )
        {
            index = next(tokens, index);
            continue;
        }
        return index;
    }
}

fn get_block(tokens: &Tokens, index: usize, is_start: bool) -> Option<BlockType> {
    match Tokens::detect_block_type(&tokens[index]) {
        Some((bt, s)) if s == is_start && BLOCK_TYPES.contains(&bt) => Some(bt),
        _ => None,
    }
}

fn contains_operation(tokens: &Tokens, mut start: usize, end: usize) -> bool {
    loop {
        start = next(tokens, start);
        if start == end {
            return false;
        }
        if let Some((bt, true)) = Tokens::detect_block_type(&tokens[start]) {
            start = tokens.find_block_end(bt, start);
            continue;
        }
        if !is_noop(&tokens[start]) {
            return true;
        }
    }
}

/// Upstream returns null for require/include(_once); those are KNOWN_NEGATIVE_PRE_TYPES, rejected before any call.
fn get_config_type(tokens: &Tokens, before: usize) -> Option<&'static str> {
    for &(k, item) in TOKEN_TYPE_CONFIG_MAP {
        if tokens[before].is_kind(k) {
            return Some(item);
        }
    }
    Some("others")
}

fn remove_brace(tokens: &mut Tokens, index: usize, mut needs_space: bool) {
    if needs_space {
        for direction in [-1, 1] {
            let sibling_index = tokens.get_non_empty_sibling(index, direction).unwrap();
            if tokens[sibling_index].is_whitespace(None) || tokens[sibling_index].is_comment() {
                needs_space = false;
                break;
            }
        }
    }
    if needs_space {
        tokens.set(index, Token::new(T_WHITESPACE, " "));
    } else {
        tokens.clear_token_and_merge_surrounding_whitespace(index);
    }
}

fn close_curly_belongs_to_dynamic_element(tokens: &Tokens, before: usize) -> bool {
    let index = prev(
        tokens,
        tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, before),
    );
    if tokens[index].is_kind(T_DOUBLE_COLON) {
        return true;
    }
    if tokens[index].equals(":", true) {
        let index = tokens
            .get_prev_token_of_kind(index, &[T_CASE.into(), "?".into()], true)
            .unwrap();
        return !tokens[index].is_kind(T_CASE);
    }
    false
}
