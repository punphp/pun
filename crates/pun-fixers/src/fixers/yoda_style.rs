//! Port of `PhpCsFixer\Fixer\ControlStructure\YodaStyleFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use pun_format::Fixer;
use pun_tokens::Kind::{self, *};
use pun_tokens::{Proto, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

/// A key of `$candidateTypes`: a token kind, or the content `'<'` / `'>'`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Candidate {
    Kind(Kind),
    Char(&'static str),
}

#[derive(Default)]
struct YodaStyleFixer {
    always_move_variable: bool,
    /// `$candidateTypesConfiguration` (key => yoda), insertion order kept.
    candidate_types: Vec<(Candidate, bool)>,
    /// `$candidatesMap`.
    candidates_map: Vec<(Candidate, Token)>,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(YodaStyleFixer::default())
}

impl Fixer for YodaStyleFixer {
    fn name(&self) -> &'static str {
        "yoda_style"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(
            self.name(),
            options,
            &[
                "equal",
                "identical",
                "less_and_greater",
                "always_move_variable",
            ],
        )?;
        let equal = c.opt_bool("equal", Some(true))?;
        let identical = c.opt_bool("identical", Some(true))?;
        let less_and_greater = c.opt_bool("less_and_greater", None)?;
        self.always_move_variable = c.bool("always_move_variable", false)?;
        self.resolve_configuration(equal, identical, less_and_greater);
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        self.candidate_types.iter().any(|(c, _)| match c {
            Candidate::Kind(k) => tokens.is_token_kind_found(*k),
            Candidate::Char(s) => tokens.is_token_kind_found(*s),
        })
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        self.fix_tokens(tokens);
    }
}

const LOWER_PRECEDENCE_KINDS: &[Kind] = &[
    T_BOOLEAN_AND,
    T_BOOLEAN_OR,
    T_CASE,
    T_DOUBLE_ARROW,
    T_ECHO,
    T_GOTO,
    T_LOGICAL_AND,
    T_LOGICAL_OR,
    T_LOGICAL_XOR,
    T_OPEN_TAG,
    T_OPEN_TAG_WITH_ECHO,
    T_PRINT,
    T_RETURN,
    T_THROW,
    T_COALESCE,
    T_YIELD,
    T_YIELD_FROM,
    T_REQUIRE,
    T_REQUIRE_ONCE,
    T_INCLUDE,
    T_INCLUDE_ONCE,
];

const LOWER_PRECEDENCE_ASSIGNMENT_KINDS: &[Kind] = &[
    T_AND_EQUAL,
    T_CONCAT_EQUAL,
    T_DIV_EQUAL,
    T_MINUS_EQUAL,
    T_MOD_EQUAL,
    T_MUL_EQUAL,
    T_OR_EQUAL,
    T_PLUS_EQUAL,
    T_POW_EQUAL,
    T_SL_EQUAL,
    T_SR_EQUAL,
    T_XOR_EQUAL,
    T_COALESCE_EQUAL,
];

const INDEX_OPEN: [Proto<'static>; 2] = [
    Proto::Char(b"["),
    Proto::KindContent(CT_ARRAY_INDEX_BRACE_OPEN, b"{"),
];

fn is_of_lower_precedence(token: &Token) -> bool {
    is_of_lower_precedence_assignment(token)
        || token.is_given_kind(LOWER_PRECEDENCE_KINDS)
        || token.equals_any(
            &[
                "&".into(),
                "|".into(),
                "^".into(),
                "?".into(),
                ":".into(),
                ",".into(),
                ";".into(),
            ],
            true,
        )
}

fn is_of_lower_precedence_assignment(token: &Token) -> bool {
    token.equals("=", true) || token.is_given_kind(LOWER_PRECEDENCE_ASSIGNMENT_KINDS)
}

impl YodaStyleFixer {
    fn resolve_configuration(
        &mut self,
        equal: Option<bool>,
        identical: Option<bool>,
        less_and_greater: Option<bool>,
    ) {
        let mut candidate_types = Vec::new();
        self.candidates_map.clear();
        if let Some(e) = equal {
            candidate_types.push((Candidate::Kind(T_IS_EQUAL), e));
            candidate_types.push((Candidate::Kind(T_IS_NOT_EQUAL), e));
        }
        if let Some(i) = identical {
            candidate_types.push((Candidate::Kind(T_IS_IDENTICAL), i));
            candidate_types.push((Candidate::Kind(T_IS_NOT_IDENTICAL), i));
        }
        if let Some(l) = less_and_greater {
            candidate_types.push((Candidate::Kind(T_IS_SMALLER_OR_EQUAL), l));
            self.candidates_map.push((
                Candidate::Kind(T_IS_SMALLER_OR_EQUAL),
                Token::new(T_IS_GREATER_OR_EQUAL, ">="),
            ));
            candidate_types.push((Candidate::Kind(T_IS_GREATER_OR_EQUAL), l));
            self.candidates_map.push((
                Candidate::Kind(T_IS_GREATER_OR_EQUAL),
                Token::new(T_IS_SMALLER_OR_EQUAL, "<="),
            ));
            candidate_types.push((Candidate::Char("<"), l));
            self.candidates_map
                .push((Candidate::Char("<"), Token::char(">")));
            candidate_types.push((Candidate::Char(">"), l));
            self.candidates_map
                .push((Candidate::Char(">"), Token::char("<")));
        }
        self.candidate_types = candidate_types;
    }

    fn yoda_for(&self, token: &Token) -> Option<bool> {
        for &(c, yoda) in &self.candidate_types {
            match c {
                Candidate::Kind(k) if token.is_kind(k) => return Some(yoda),
                Candidate::Char(s) if token.equals(s, true) => return Some(yoda),
                _ => {}
            }
        }
        None
    }

    fn find_comparison_end(&self, tokens: &Tokens, mut index: usize) -> usize {
        index += 1;
        let count = tokens.count();
        while index < count {
            let token = &tokens[index];
            if token.is_given_kind(&[T_WHITESPACE, T_COMMENT, T_DOC_COMMENT]) {
                index += 1;
                continue;
            }
            if is_of_lower_precedence(token) {
                break;
            }
            match Tokens::detect_block_type(token) {
                None => {
                    index += 1;
                    continue;
                }
                Some((_, false)) => break,
                Some((block_type, true)) => index = tokens.find_block_end(block_type, index) + 1,
            }
        }
        let prev = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[prev].is_kind(T_CLOSE_TAG) {
            tokens.get_prev_meaningful_token(prev).unwrap()
        } else {
            prev
        }
    }

    fn find_comparison_start(&self, tokens: &Tokens, index: usize) -> usize {
        let mut index = index as isize - 1;
        let mut non_block_found = false;
        while 0 <= index {
            let token = &tokens[index as usize];
            if token.is_given_kind(&[T_WHITESPACE, T_COMMENT, T_DOC_COMMENT]) {
                index -= 1;
                continue;
            }
            if token.is_kind(CT_NAMED_ARGUMENT_COLON) {
                break;
            }
            if is_of_lower_precedence(token) {
                break;
            }
            match Tokens::detect_block_type(token) {
                None => {
                    index -= 1;
                    non_block_found = true;
                    continue;
                }
                Some((block_type, is_start)) => {
                    // closing of structure not related to the comparison
                    if is_start || (non_block_found && block_type == Tokens::BLOCK_TYPE_BRACE) {
                        break;
                    }
                    index = tokens.find_block_start(block_type, index as usize) as isize - 1;
                }
            }
        }
        // The scan always stops at the file's T_OPEN_TAG(_WITH_ECHO) (lower precedence), so `index` never reaches -1.
        tokens.get_next_meaningful_token(index as usize).unwrap()
    }

    fn fix_tokens(&self, tokens: &mut Tokens) {
        let mut i = tokens.count() as isize - 1;
        while i > 1 {
            let iu = i as usize;
            let Some(yoda) = self.yoda_for(&tokens[iu]) else {
                i -= 1;
                continue;
            };
            if let Some((left, right)) = self.get_compare_fixable_info(tokens, iu, yoda) {
                i = self.fix_tokens_compare(tokens, left.0, left.1, iu, right.0, right.1) as isize;
            }
            i -= 1;
        }
    }

    fn fix_tokens_compare(
        &self,
        tokens: &mut Tokens,
        start_left: usize,
        end_left: usize,
        compare_operator_index: usize,
        start_right: usize,
        end_right: usize,
    ) -> usize {
        let op = &tokens[compare_operator_index];
        let replacement = self
            .candidates_map
            .iter()
            .find(|(c, _)| match c {
                Candidate::Kind(k) => op.is_kind(*k),
                Candidate::Char(s) => op.id.is_none() && op.content == s.as_bytes(),
            })
            .map(|(_, t)| t.clone());
        if let Some(t) = replacement {
            tokens.set(compare_operator_index, t);
        }
        let right = self.fix_tokens_compare_part(tokens, start_right, end_right);
        let left = self.fix_tokens_compare_part(tokens, start_left, end_left);
        for i in start_right..=end_right {
            tokens.clear_at(i);
        }
        for i in start_left..=end_left {
            tokens.clear_at(i);
        }
        tokens.insert_at(start_right, left);
        tokens.insert_at(start_left, right);
        start_left
    }

    fn fix_tokens_compare_part(&self, tokens: &Tokens, start: usize, end: usize) -> Vec<Token> {
        let code = [
            &b"<?php "[..],
            &tokens.generate_partial_code(start, end),
            b";",
        ]
        .concat();
        let mut new_tokens = Tokens::from_code(&code);
        self.fix_tokens(&mut new_tokens);
        let last = new_tokens.count() - 1;
        new_tokens.clear_at(last);
        new_tokens.clear_at(0);
        new_tokens.clear_empty_tokens();
        new_tokens.iter().cloned().collect()
    }

    fn get_compare_fixable_info(
        &self,
        tokens: &Tokens,
        index: usize,
        yoda: bool,
    ) -> Option<((usize, usize), (usize, usize))> {
        let right = (
            tokens.get_next_meaningful_token(index).unwrap(),
            self.find_comparison_end(tokens, index),
        );
        if !yoda
            && tokens
                .get_next_meaningful_token(right.1)
                .is_some_and(|n| is_of_lower_precedence_assignment(&tokens[n]))
        {
            return None;
        }
        let left = (
            self.find_comparison_start(tokens, index),
            tokens.get_prev_meaningful_token(index).unwrap(),
        );
        if is_list_statement(tokens, left.0, left.1) || is_list_statement(tokens, right.0, right.1)
        {
            return None; // do not fix lists assignment inside statements
        }
        let strict = self.always_move_variable;
        let mut left_side_is_variable = is_variable(tokens, left.0, left.1, strict);
        let mut right_side_is_variable = is_variable(tokens, right.0, right.1, strict);
        if left_side_is_variable == right_side_is_variable {
            return None; // both are (not) variables, do not touch
        }
        if !strict {
            // special handling for braces with not "always_move_variable"
            left_side_is_variable = left_side_is_variable && !tokens[left.0].equals("(", true);
            right_side_is_variable = right_side_is_variable && !tokens[right.0].equals("(", true);
        }
        if (yoda && !left_side_is_variable) || (!yoda && !right_side_is_variable) {
            None
        } else {
            Some((left, right))
        }
    }
}

fn is_list_statement(tokens: &Tokens, index: usize, end: usize) -> bool {
    (index..=end).any(|i| {
        tokens[i].is_given_kind(&[
            T_LIST,
            CT_DESTRUCTURING_BRACKET_OPEN,
            CT_DESTRUCTURING_BRACKET_CLOSE,
        ])
    })
}

fn is_variable(tokens: &Tokens, start: usize, end: usize, strict: bool) -> bool {
    let token_analyzer = TokensAnalyzer::new(tokens);
    if start == end {
        return tokens[start].is_kind(T_VARIABLE);
    }
    if tokens[start].equals("(", true) {
        return true;
    }
    if strict {
        for index in start..=end {
            if tokens[index].is_cast()
                || tokens[index].is_kind(T_INSTANCEOF)
                || tokens[index].equals("!", true)
                || token_analyzer.is_binary_operator(index)
            {
                return false;
            }
        }
    }
    // Upstream's "handle multiple braces around statement" loop never runs: a `(` start already returned true above.
    let mut index = start;
    let mut expect_string = false;
    while index <= end {
        // (Upstream skips comments/whitespace/empty tokens here; `index` only ever lands on meaningful tokens.)
        let current = &tokens[index];
        if index == end {
            return current.is_kind(if expect_string { T_STRING } else { T_VARIABLE });
        }
        // (Upstream also rejects T_LIST/destructuring brackets here; isListStatement already did.)
        let next_index = tokens.get_next_meaningful_token(index).unwrap();
        let next = &tokens[next_index];
        // self:: or ClassName::
        if current.is_kind(T_STRING) && next.is_kind(T_DOUBLE_COLON) {
            index = tokens.get_next_meaningful_token(next_index).unwrap();
            continue;
        }
        // \ClassName
        if current.is_kind(T_NS_SEPARATOR) && next.is_kind(T_STRING) {
            index = next_index;
            continue;
        }
        // ClassName\
        if current.is_kind(T_STRING) && next.is_kind(T_NS_SEPARATOR) {
            index = next_index;
            continue;
        }
        // $a-> or a-> (as in $b->a->c)
        if current.is_given_kind(&[T_STRING, T_VARIABLE]) && next.is_object_operator() {
            index = tokens.get_next_meaningful_token(next_index).unwrap();
            expect_string = true;
            continue;
        }
        // $a[...], a[...] (as in $c->a[$b]), $a{...} or a{...} (as in $c->a{$b})
        if current.is_kind(if expect_string { T_STRING } else { T_VARIABLE })
            && next.equals_any(&INDEX_OPEN, true)
        {
            let bt = if next.equals("[", true) {
                Tokens::BLOCK_TYPE_INDEX_BRACKET
            } else {
                Tokens::BLOCK_TYPE_INDEX_BRACE
            };
            index = tokens.find_block_end(bt, next_index);
            if index == end {
                return true;
            }
            index = tokens.get_next_meaningful_token(index).unwrap();
            if !tokens[index].equals_any(&INDEX_OPEN, true) && !tokens[index].is_object_operator() {
                return false;
            }
            index = tokens.get_next_meaningful_token(index).unwrap();
            expect_string = true;
            continue;
        }
        // $a(...) or $a->b(...)
        if strict && current.is_given_kind(&[T_STRING, T_VARIABLE]) && next.equals("(", true) {
            return false;
        }
        // {...} (as in $a->{$b})
        if expect_string && current.is_kind(CT_DYNAMIC_PROP_BRACE_OPEN) {
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_DYNAMIC_PROP_BRACE, index);
            if index == end {
                return true;
            }
            index = tokens.get_next_meaningful_token(index).unwrap();
            if !tokens[index].is_object_operator() {
                return false;
            }
            index = tokens.get_next_meaningful_token(index).unwrap();
            expect_string = true;
            continue;
        }
        break;
    }
    !is_constant(tokens, start, end)
}

fn is_constant(tokens: &Tokens, mut index: usize, end: usize) -> bool {
    let (mut expect_array_only, mut expect_number_only, mut expect_nothing) = (false, false, false);
    while index <= end {
        let token = &tokens[index];
        index += 1;
        if token.is_comment() || token.is_whitespace(None) {
            continue;
        }
        if expect_nothing {
            return false;
        }
        if expect_array_only {
            if token.equals_any(
                &["(".into(), ")".into(), CT_ARRAY_BRACKET_CLOSE.into()],
                true,
            ) {
                continue;
            }
            return false;
        }
        if token.is_given_kind(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN]) {
            expect_array_only = true;
            continue;
        }
        if expect_number_only && !token.is_given_kind(&[T_LNUMBER, T_DNUMBER]) {
            return false;
        }
        if token.equals("-", true) {
            expect_number_only = true;
            continue;
        }
        if token.is_given_kind(&[T_LNUMBER, T_DNUMBER, T_CONSTANT_ENCAPSED_STRING])
            || token.equals_any(
                &[
                    (T_STRING, "true").into(),
                    (T_STRING, "false").into(),
                    (T_STRING, "null").into(),
                ],
                true,
            )
        {
            expect_nothing = true;
            continue;
        }
        return false;
    }
    true
}
