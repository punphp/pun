//! Port of `PhpCsFixer\Fixer\Operator\BinaryOperatorSpacesFixer`.
use pun_tokens::php_compat::{str_replace, strpos};
use crate::analyzers::fixer_configuration_resolver::{PhpKey, resolve};
use crate::analyzers::preg::Preg;
use crate::analyzers::utils::{mb_strwidth, natural_language_join};
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::{
    Kind::*,
    Token, Tokens, TokensAnalyzer,
    php_compat::{rtrim, strtolower},
};
use serde_json::Value;
use std::cell::Cell;
use std::path::Path;
use std::sync::Mutex;

const SINGLE_SPACE: &str = "single_space";
const AT_LEAST_SINGLE_SPACE: &str = "at_least_single_space";
const NO_SPACE: &str = "no_space";
const ALIGN: &str = "align";
const ALIGN_BY_SCOPE: &str = "align_by_scope";
const ALIGN_SINGLE_SPACE: &str = "align_single_space";
const ALIGN_SINGLE_SPACE_BY_SCOPE: &str = "align_single_space_by_scope";
const ALIGN_SINGLE_SPACE_MINIMAL: &str = "align_single_space_minimal";
const ALIGN_SINGLE_SPACE_MINIMAL_BY_SCOPE: &str = "align_single_space_minimal_by_scope";

/// Placeholder used as anchor for right alignment (`"\x2 ALIGNABLE%d \x3"`).
fn align_placeholder(level: usize) -> Vec<u8> {
    format!("\x02 ALIGNABLE{level} \x03").into_bytes()
}

const SUPPORTED_OPERATORS: &[&str] = &[
    "=", "*", "/", "%", "<", ">", "|", "^", "+", "-", "&", "&=", "&&", "||", ".=", "/=", "=>",
    "==", ">=", "===", "!=", "<>", "!==", "<=", "and", "or", "xor", "-=", "%=", "*=", "|=", "+=",
    "<<", "<<=", ">>", ">>=", "^=", "**", "**=", "<=>", "??", "??=",
];

const ALLOWED_VALUES: &[&str] = &[
    ALIGN,
    ALIGN_BY_SCOPE,
    ALIGN_SINGLE_SPACE,
    ALIGN_SINGLE_SPACE_MINIMAL,
    ALIGN_SINGLE_SPACE_BY_SCOPE,
    ALIGN_SINGLE_SPACE_MINIMAL_BY_SCOPE,
    SINGLE_SPACE,
    NO_SPACE,
    AT_LEAST_SINGLE_SPACE,
];

struct BinaryOperatorSpacesFixer {
    /// Resolved `operator => strategy`, in PHP array order.
    operators: Vec<(Vec<u8>, &'static str)>,
    /// `$this->alignOperatorTokens`: never reset between files upstream (instance state).
    // ponytail: shared across files like upstream's instance property; Pint's parallel workers make
    // the cross-file effect order-dependent there too.
    align_operator_tokens: Mutex<Vec<(Vec<u8>, &'static str)>>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(BinaryOperatorSpacesFixer {
        operators: Vec::new(),
        align_operator_tokens: Mutex::new(Vec::new()),
    })
}

/// `gettype($value).'#'.$value` for a decoded JSON value other than null, as the "operators" validator prints it.
fn typed_value(v: &Value) -> String {
    match v {
        Value::String(s) => format!("string#{s}"),
        Value::Number(n) if n.is_i64() => format!("integer#{n}"),
        Value::Number(n) => format!(
            "double#{}",
            pun_tokens::php_compat::float_to_string(n.as_f64().unwrap())
        ),
        Value::Bool(b) => format!("boolean#{}", if *b { "1" } else { "" }),
        _ => "array#Array".into(),
    }
}

fn strategy(s: &str) -> &'static str {
    ALLOWED_VALUES.iter().copied().find(|v| *v == s).unwrap()
}

/// Per-`fix` state (`deepestLevel`, `currentLevel`).
struct Levels {
    deepest_level: Cell<usize>,
    current_level: Cell<usize>,
}

/// The spaces `fix` adds around operators, by the index they go before. The loop walks backwards and looks only
/// at an operator's own neighbours, so it never reads a place where a space is still to come, and one
/// `insert_slices` at the end gives what an insert per operator would, without rebuilding the list each time.
type Spaces = std::collections::BTreeMap<usize, Vec<Token>>;

fn is_binary(tokens: &Tokens, index: usize) -> bool {
    TokensAnalyzer::new(tokens).is_binary_operator(index)
}

impl BinaryOperatorSpacesFixer {
    fn operator(&self, content: &[u8]) -> Option<&'static str> {
        self.operators
            .iter()
            .find(|(o, _)| o == content)
            .map(|&(_, s)| s)
    }

    fn fix_white_space_around_operator(&self, tokens: &mut Tokens, index: usize, spaces: &mut Spaces) {
        let token_content = strtolower(tokens[index].get_content());
        let Some(op) = self.operator(&token_content) else {
            return;
        };
        if op == SINGLE_SPACE {
            Self::fix_white_space_around_operator_to_single_space(tokens, index, spaces);
            return;
        }
        if op == AT_LEAST_SINGLE_SPACE {
            Self::fix_white_space_around_operator_to_at_least_single_space(tokens, index, spaces);
            return;
        }
        if op == NO_SPACE {
            Self::fix_white_space_around_operator_to_no_space(tokens, index);
            return;
        }
        {
            let mut align = self.align_operator_tokens.lock().unwrap();
            match align.iter_mut().find(|(c, _)| *c == token_content) {
                Some(e) => e.1 = op,
                None => align.push((token_content.clone(), op)),
            }
        }
        if op == ALIGN || op == ALIGN_BY_SCOPE {
            return;
        }
        if tokens[index + 1].is_whitespace(None) {
            if op == ALIGN_SINGLE_SPACE_MINIMAL || op == ALIGN_SINGLE_SPACE_MINIMAL_BY_SCOPE {
                tokens.set(index + 1, Token::new(T_WHITESPACE, " "));
            }
            return;
        }
        spaces.insert(index + 1, vec![Token::new(T_WHITESPACE, " ")]);
    }

    fn fix_white_space_around_operator_to_single_space(
        tokens: &mut Tokens,
        index: usize,
        spaces: &mut Spaces,
    ) {
        if tokens[index + 1].is_whitespace(None) {
            let content = tokens[index + 1].get_content();
            if content != b" "
                && !content.contains(&b'\n')
                && !tokens[tokens.get_next_non_whitespace(index + 1, None).unwrap()].is_comment()
            {
                tokens.set(index + 1, Token::new(T_WHITESPACE, " "));
            }
        } else {
            spaces.insert(index + 1, vec![Token::new(T_WHITESPACE, " ")]);
        }
        if tokens[index - 1].is_whitespace(None) {
            let content = tokens[index - 1].get_content();
            if content != b" "
                && !content.contains(&b'\n')
                && !tokens[tokens.get_prev_non_whitespace(index - 1, None).unwrap()].is_comment()
            {
                tokens.set(index - 1, Token::new(T_WHITESPACE, " "));
            }
        } else {
            spaces.insert(index, vec![Token::new(T_WHITESPACE, " ")]);
        }
    }

    fn fix_white_space_around_operator_to_at_least_single_space(
        tokens: &mut Tokens,
        index: usize,
        spaces: &mut Spaces,
    ) {
        if !tokens[index + 1].is_whitespace(None) {
            spaces.insert(index + 1, vec![Token::new(T_WHITESPACE, " ")]);
        }
        if !tokens[index - 1].is_whitespace(None) {
            spaces.insert(index, vec![Token::new(T_WHITESPACE, " ")]);
        }
    }

    fn fix_white_space_around_operator_to_no_space(tokens: &mut Tokens, index: usize) {
        if tokens[index + 1].is_whitespace(None) {
            let content = tokens[index + 1].get_content();
            if !content.contains(&b'\n')
                && !tokens[tokens.get_next_non_whitespace(index + 1, None).unwrap()].is_comment()
            {
                tokens.clear_at(index + 1);
            }
        }
        if tokens[index - 1].is_whitespace(None) {
            let content = tokens[index - 1].get_content();
            if !content.contains(&b'\n')
                && !tokens[tokens.get_prev_non_whitespace(index - 1, None).unwrap()].is_comment()
            {
                tokens.clear_at(index - 1);
            }
        }
    }

    /// Index of T_DECLARE where the `=` belongs to.
    fn is_equal_part_of_declare_statement(tokens: &Tokens, index: usize) -> Option<usize> {
        let mut prev_meaningful_index = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[prev_meaningful_index].is_kind(T_STRING) {
            prev_meaningful_index = tokens
                .get_prev_meaningful_token(prev_meaningful_index)
                .unwrap();
            if tokens[prev_meaningful_index].equals("(", true) {
                prev_meaningful_index = tokens
                    .get_prev_meaningful_token(prev_meaningful_index)
                    .unwrap();
                // `( T_STRING =` only parses as `declare(...)`
                return tokens[prev_meaningful_index]
                    .is_kind(T_DECLARE)
                    .then_some(prev_meaningful_index);
            }
        }
        None
    }


    fn fix_alignment(&self, tokens: &mut Tokens, to_align: &[(Vec<u8>, &'static str)]) {
        let levels = Levels {
            deepest_level: Cell::new(0),
            current_level: Cell::new(0),
        };
        for (token_content, align_strategy) in to_align {
            let align_strategy = *align_strategy;
            // Work on a clone, then override the original collection with the fixed code.
            let mut tokens_clone = tokens.clone();
            let count = tokens.count();
            if token_content == b"=>" {
                Self::inject_alignment_placeholders_for_arrow(
                    &levels,
                    tokens,
                    &mut tokens_clone,
                    0,
                    count,
                );
            } else {
                Self::inject_alignment_placeholders_default(
                    &levels,
                    tokens,
                    &mut tokens_clone,
                    0,
                    count,
                    token_content,
                );
            }
            // for all tokens that should be aligned but do not have anything to align with, fix spacing if needed
            if [
                ALIGN_SINGLE_SPACE,
                ALIGN_SINGLE_SPACE_MINIMAL,
                ALIGN_SINGLE_SPACE_BY_SCOPE,
                ALIGN_SINGLE_SPACE_MINIMAL_BY_SCOPE,
            ]
            .contains(&align_strategy)
            {
                let mut index = tokens.count() as isize - 2;
                while index > 0 {
                    let i = index as usize;
                    let hit = if token_content == b"=>" {
                        // always binary operator, never part of declare statement
                        tokens[i].is_kind(T_DOUBLE_ARROW)
                    } else if token_content == b"=" {
                        tokens[i].get_content() == b"="
                            && Self::is_equal_part_of_declare_statement(tokens, i).is_none()
                            && is_binary(tokens, i)
                    } else {
                        // never part of declare statement
                        strtolower(tokens[i].get_content()) == *token_content
                            && is_binary(tokens, i)
                    };
                    if hit {
                        Self::fix_white_space_before_operator(&mut tokens_clone, i, align_strategy);
                    }
                    index -= 1;
                }
            }
            let code = Self::replace_placeholders(&levels, &tokens_clone, align_strategy);
            tokens.set_code(&code);
        }
    }

    /// `analyzed` is the collection the upstream TokensAnalyzer wraps (the original, not the clone).
    fn inject_alignment_placeholders_default(
        levels: &Levels,
        analyzed: &Tokens,
        tokens: &mut Tokens,
        start_at: usize,
        end_at: usize,
        token_content: &[u8],
    ) {
        let mut new_line_found_since_last_placeholder = true;
        let mut index = start_at;
        while index < end_at {
            // `tokens[index]` is upstream's `$token` below: nothing edits it before the checks.
            let content = tokens[index].get_content();
            if content.contains(&b'\n') {
                new_line_found_since_last_placeholder = true;
            }
            // `token_content` is lowercase, so this is `strtolower($content) === $tokenContent`.
            if content.eq_ignore_ascii_case(token_content)
                && is_binary(analyzed, index)
                && (content != b"="
                    || Self::is_equal_part_of_declare_statement(tokens, index).is_none())
                && new_line_found_since_last_placeholder
            {
                let content = [
                    align_placeholder(levels.current_level.get()),
                    tokens[index].get_content().to_vec(),
                ]
                .concat();
                tokens.set(index, Token::char(content));
                new_line_found_since_last_placeholder = false;
                index += 1;
                continue;
            }
            if tokens[index].is_kind(T_FN) {
                let from = tokens.get_next_meaningful_token(index).unwrap();
                let until =
                    TokensAnalyzer::new(analyzed).get_last_token_index_of_arrow_function(index);
                Self::inject_alignment_placeholders(
                    levels,
                    analyzed,
                    tokens,
                    from + 1,
                    until - 1,
                    token_content,
                );
                index = until + 1;
                continue;
            }
            if tokens[index].is_given_kind(&[T_FUNCTION, T_CLASS]) {
                index = tokens
                    .get_next_token_of_kind(index, &["{".into(), ";".into(), "(".into()], true)
                    .unwrap();
                // We don't align `=` on multi-line definition of function parameters with default values
                if tokens[index].equals("(", true) {
                    index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index) + 1;
                    continue;
                }
                // (upstream's `;` arm is dead: `::class` is CT_CLASS_CONSTANT, and T_FUNCTION / T_CLASS always
                // reach `(` or `{` before any `;`)
                // `index` is now the `{`, so the checks below look at it, like upstream's `$token = $tokens[$index]`
            }
            if tokens[index].equals("{", true) {
                let until = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
                Self::inject_alignment_placeholders(
                    levels,
                    analyzed,
                    tokens,
                    index + 1,
                    until - 1,
                    token_content,
                );
                index = until + 1;
                continue;
            }
            if tokens[index].equals("(", true) {
                let until = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
                Self::inject_alignment_placeholders(
                    levels,
                    analyzed,
                    tokens,
                    index + 1,
                    until - 1,
                    token_content,
                );
                index = until + 1;
                continue;
            }
            if tokens[index].equals("[", true) {
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_INDEX_BRACKET, index) + 1;
                continue;
            }
            if tokens[index].is_kind(CT_ARRAY_BRACKET_OPEN) {
                let until = tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, index);
                Self::inject_alignment_placeholders(
                    levels,
                    analyzed,
                    tokens,
                    index + 1,
                    until - 1,
                    token_content,
                );
                index = until + 1;
                continue;
            }
            index += 1;
        }
    }

    fn inject_alignment_placeholders(
        levels: &Levels,
        analyzed: &Tokens,
        tokens: &mut Tokens,
        from: usize,
        until: usize,
        token_content: &[u8],
    ) {
        // Only inject placeholders for multi-line code
        if tokens.is_partial_code_multiline(from, until) {
            levels.deepest_level.set(levels.deepest_level.get() + 1);
            let current_level = levels.current_level.get();
            levels.current_level.set(levels.deepest_level.get());
            Self::inject_alignment_placeholders_default(
                levels,
                analyzed,
                tokens,
                from,
                until,
                token_content,
            );
            levels.current_level.set(current_level);
        }
    }

    fn inject_alignment_placeholders_for_arrow(
        levels: &Levels,
        analyzed: &Tokens,
        tokens: &mut Tokens,
        start_at: usize,
        end_at: usize,
    ) {
        let mut new_line_found_since_last_placeholder = true;
        let mut yield_found_since_last_placeholder = false;
        let mut index = start_at;
        while index < end_at {
            // `tokens[index]` is upstream's `$token`: nothing edits it before the checks.
            if tokens[index].get_content().contains(&b'\n') {
                new_line_found_since_last_placeholder = true;
            }
            if tokens[index].is_kind(T_YIELD) {
                yield_found_since_last_placeholder = true;
            }
            if tokens[index].is_kind(T_FN) {
                yield_found_since_last_placeholder = false;
                let from = tokens.get_next_meaningful_token(index).unwrap();
                let until =
                    TokensAnalyzer::new(analyzed).get_last_token_index_of_arrow_function(index);
                Self::inject_array_alignment_placeholders(
                    levels,
                    analyzed,
                    tokens,
                    from + 1,
                    until - 1,
                );
                index = until + 1;
                continue;
            }
            if tokens[index].is_kind(T_ARRAY) {
                // don't use "$tokens->isArray()" here, short arrays are handled in the next case
                yield_found_since_last_placeholder = false;
                let from = tokens.get_next_meaningful_token(index).unwrap();
                let until = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, from);
                index = until;
                Self::inject_array_alignment_placeholders(
                    levels,
                    analyzed,
                    tokens,
                    from + 1,
                    until - 1,
                );
                index += 1;
                continue;
            }
            if tokens[index].is_kind(CT_ARRAY_BRACKET_OPEN) {
                yield_found_since_last_placeholder = false;
                let from = index;
                let until = tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, from);
                index = until;
                Self::inject_array_alignment_placeholders(
                    levels,
                    analyzed,
                    tokens,
                    from + 1,
                    until - 1,
                );
                index += 1;
                continue;
            }
            // no need to analyse for `isBinaryOperator` (always true), nor if part of declare statement (not valid PHP)
            // there is also no need to analyse the second arrow of a line
            if tokens[index].is_kind(T_DOUBLE_ARROW) && new_line_found_since_last_placeholder {
                if yield_found_since_last_placeholder {
                    levels.deepest_level.set(levels.deepest_level.get() + 1);
                    levels.current_level.set(levels.deepest_level.get());
                }
                let mut token_content = [
                    align_placeholder(levels.current_level.get()),
                    tokens[index].get_content().to_vec(),
                ]
                .concat();
                let next_token = &tokens[index + 1];
                if !next_token.is_whitespace(None) {
                    token_content.push(b' ');
                } else if next_token.is_whitespace(Some(b" \t")) {
                    tokens.set(index + 1, Token::new(T_WHITESPACE, " "));
                }
                tokens.set(index, Token::new(T_DOUBLE_ARROW, token_content));
                new_line_found_since_last_placeholder = false;
                yield_found_since_last_placeholder = false;
                index += 1;
                continue;
            }
            if tokens[index].equals(";", true) {
                levels.deepest_level.set(levels.deepest_level.get() + 1);
                levels.current_level.set(levels.current_level.get() + 1);
                index += 1;
                continue;
            }
            if tokens[index].equals(",", true) {
                let mut i = index;
                while i < end_at - 1 {
                    if tokens[i].get_content().contains(&b'\n') {
                        new_line_found_since_last_placeholder = true;
                        break;
                    }
                    if tokens[i + 1].is_kind(T_DOUBLE_ARROW) {
                        break;
                    }
                    if tokens[i + 1].is_given_kind(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN]) {
                        let array_start_index = if tokens[i + 1].is_kind(T_ARRAY) {
                            tokens.get_next_meaningful_token(i + 1).unwrap()
                        } else {
                            i + 1
                        };
                        let (block_type, _) =
                            Tokens::detect_block_type(&tokens[array_start_index]).unwrap();
                        let array_end_index = tokens.find_block_end(block_type, array_start_index);
                        if tokens.is_partial_code_multiline(array_start_index, array_end_index) {
                            break;
                        }
                    }
                    index += 1;
                    i += 1;
                }
            }
            if tokens[index].equals("{", true) {
                let until = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
                Self::inject_array_alignment_placeholders(
                    levels,
                    analyzed,
                    tokens,
                    index + 1,
                    until - 1,
                );
                index = until + 1;
                continue;
            }
            if tokens[index].equals("(", true) {
                let until = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
                Self::inject_array_alignment_placeholders(
                    levels,
                    analyzed,
                    tokens,
                    index + 1,
                    until - 1,
                );
                index = until + 1;
                continue;
            }
            index += 1;
        }
    }

    fn inject_array_alignment_placeholders(
        levels: &Levels,
        analyzed: &Tokens,
        tokens: &mut Tokens,
        from: usize,
        until: usize,
    ) {
        // Only inject placeholders for multi-line arrays
        if tokens.is_partial_code_multiline(from, until) {
            levels.deepest_level.set(levels.deepest_level.get() + 1);
            let current_level = levels.current_level.get();
            levels.current_level.set(levels.deepest_level.get());
            Self::inject_alignment_placeholders_for_arrow(levels, analyzed, tokens, from, until);
            levels.current_level.set(current_level);
        }
    }

    fn fix_white_space_before_operator(tokens: &mut Tokens, index: usize, align_strategy: &str) {
        // fix white space after operator is not needed as BinaryOperatorSpacesFixer took care of this (if strategy is _not_ ALIGN)
        if !tokens[index - 1].is_whitespace(None) {
            tokens.insert_at(index, vec![Token::new(T_WHITESPACE, " ")]);
            return;
        }
        if (align_strategy != ALIGN_SINGLE_SPACE_MINIMAL
            && align_strategy != ALIGN_SINGLE_SPACE_MINIMAL_BY_SCOPE)
            || tokens[tokens.get_prev_non_whitespace(index - 1, None).unwrap()].is_comment()
        {
            return;
        }
        let content = tokens[index - 1].get_content();
        if content != b" " && !content.contains(&b'\n') {
            tokens.set(index - 1, Token::new(T_WHITESPACE, " "));
        }
    }

    /// Look for group of placeholders and provide vertical alignment.
    fn replace_placeholders(levels: &Levels, tokens: &Tokens, align_strategy: &str) -> Vec<u8> {
        let mut tmp_code = tokens.generate_code();
        for j in 0..=levels.deepest_level.get() {
            let placeholder = align_placeholder(j);
            if strpos(&tmp_code, &placeholder, 0).is_none() {
                continue;
            }
            let mut lines: Vec<Vec<u8>> = tmp_code
                .split(|&b| b == b'\n')
                .map(|l| l.to_vec())
                .collect();
            let mut groups: Vec<Vec<usize>> = vec![Vec::new()];
            let by_scope = [
                ALIGN_BY_SCOPE,
                ALIGN_SINGLE_SPACE_BY_SCOPE,
                ALIGN_SINGLE_SPACE_MINIMAL_BY_SCOPE,
            ]
            .contains(&align_strategy);
            for (index, line) in lines.iter().enumerate() {
                if strpos(line, &placeholder, 0).is_some() {
                    groups.last_mut().unwrap().push(index);
                } else if !by_scope {
                    groups.push(Vec::new());
                }
            }
            for group in &groups {
                if group.is_empty() {
                    continue;
                }
                if align_strategy != ALIGN {
                    for &index in group {
                        let current_position = strpos(&lines[index], &placeholder, 0).unwrap();
                        let mut before = lines[index][..current_position].to_vec();
                        if align_strategy == ALIGN_SINGLE_SPACE
                            || align_strategy == ALIGN_SINGLE_SPACE_BY_SCOPE
                        {
                            if !before.ends_with(b" ") {
                                before.push(b' ');
                            }
                        } else if (align_strategy == ALIGN_SINGLE_SPACE_MINIMAL
                            || align_strategy == ALIGN_SINGLE_SPACE_MINIMAL_BY_SCOPE)
                            && !Preg::r#match("/^\\h+$/", &before)
                        {
                            // if indent; do not move, leave to other fixer
                            before = [rtrim(&before), b" "].concat();
                        }
                        lines[index] = [&before[..], &lines[index][current_position..]].concat();
                    }
                }
                let width = |line: &[u8]| mb_strwidth(&line[..strpos(line, &placeholder, 0).unwrap()]);
                let rightmost_symbol = group.iter().map(|&i| width(&lines[i])).max().unwrap_or(0);
                for &index in group {
                    let current_symbol = width(&lines[index]);
                    let delta = rightmost_symbol.abs_diff(current_symbol);
                    if delta > 0 {
                        let rep = [" ".repeat(delta).into_bytes(), placeholder.clone()].concat();
                        lines[index] = str_replace(&placeholder, &rep, &lines[index]);
                    }
                }
            }
            tmp_code = str_replace(&placeholder, b"", &lines.join(&b'\n'));
        }
        tmp_code
    }
}

impl Fixer for BinaryOperatorSpacesFixer {
    fn name(&self) -> &'static str {
        "binary_operator_spaces"
    }
    fn priority(&self) -> i32 {
        // Must run after ArrayIndentationFixer, ArraySyntaxFixer, AssignNullCoalescingToCoalesceEqualFixer, ListSyntaxFixer,
        // LongToShorthandOperatorFixer, ModernizeStrposFixer, NoMultilineWhitespaceAroundDoubleArrowFixer, NoUnsetCastFixer,
        // PowToExponentiationFixer, StandardizeNotEqualsFixer, StrictComparisonFixer.
        -32
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["default", "operators"])?;
        let default = match r.get("default") {
            None => Some(SINGLE_SPACE),
            Some(Value::Null) => None,
            Some(Value::String(s)) if ALLOWED_VALUES.contains(&s.as_str()) => Some(strategy(s)),
            Some(v) => return Err(r.invalid_or_null("default", v, ALLOWED_VALUES)),
        };
        // `array<string, ?string>`, which the resolver checks as `array`.
        r.check_types("operators", &["array"])?;
        let mut configured: Vec<(String, Option<&'static str>)> = Vec::new();
        for (operator, value) in r.php_array("operators").unwrap_or_default() {
            if !matches!(operator, PhpKey::Str(o) if SUPPORTED_OPERATORS.contains(&o)) {
                bail!(
                    "[binary_operator_spaces] Invalid configuration: Unexpected \"operators\" key, expected any of {}, got \"{}\".",
                    natural_language_join(SUPPORTED_OPERATORS),
                    operator.typed()
                );
            }
            match value {
                Value::Null => configured.push((operator.to_string(), None)),
                Value::String(s) if ALLOWED_VALUES.contains(&s.as_str()) => {
                    configured.push((operator.to_string(), Some(strategy(s))));
                }
                v => bail!(
                    "[binary_operator_spaces] Invalid configuration: Unexpected value for operator \"{operator}\", expected any of {}, got \"{}\".",
                    natural_language_join(
                        ALLOWED_VALUES
                            .iter()
                            .map(|a| format!("'{a}'"))
                            .chain(["null".to_string()])
                    ),
                    typed_value(v)
                ),
            }
        }
        // resolveOperatorsFromConfig
        let mut operators: Vec<(Vec<u8>, &'static str)> = Vec::new();
        if let Some(d) = default {
            operators = SUPPORTED_OPERATORS
                .iter()
                .map(|o| (o.as_bytes().to_vec(), d))
                .collect();
        }
        for (operator, value) in configured {
            let key = operator.into_bytes();
            match value {
                None => operators.retain(|(o, _)| *o != key),
                Some(v) => match operators.iter_mut().find(|(o, _)| *o == key) {
                    Some(e) => e.1 = v,
                    None => operators.push((key, v)),
                },
            }
        }
        self.operators = operators;
        Ok(())
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut spaces = Spaces::new();
        // last and first tokens cannot be an operator
        let mut index = tokens.count() as isize - 2;
        while index > 0 {
            let i = index as usize;
            if !is_binary(tokens, i) {
                index -= 1;
                continue;
            }
            if tokens[i].get_content() == b"=" {
                match Self::is_equal_part_of_declare_statement(tokens, i) {
                    None => self.fix_white_space_around_operator(tokens, i, &mut spaces),
                    Some(d) => index = d as isize, // skip `declare(foo ==bar)`, see `declare_equal_normalize`
                }
            } else {
                self.fix_white_space_around_operator(tokens, i, &mut spaces);
            }
            // previous of binary operator is now never an operator / previous of declare statement cannot be an operator
            index -= 2;
        }
        tokens.insert_slices(spaces);
        let to_align = self.align_operator_tokens.lock().unwrap().clone();
        if !to_align.is_empty() {
            self.fix_alignment(tokens, &to_align);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Accept/reject and messages from the real fixer's configure() (PHP-CS-Fixer 3.95).
    #[test]
    fn configure_validation() {
        assert_eq!(
            fixer()
                .configure(&json!({"operators": {"~~": "align"}}))
                .unwrap_err()
                .to_string(),
            "[binary_operator_spaces] Invalid configuration: Unexpected \"operators\" key, expected any of \"=\", \"*\", \"/\", \"%\", \"<\", \">\", \"|\", \"^\", \"+\", \"-\", \"&\", \"&=\", \"&&\", \"||\", \".=\", \"/=\", \"=>\", \"==\", \">=\", \"===\", \"!=\", \"<>\", \"!==\", \"<=\", \"and\", \"or\", \"xor\", \"-=\", \"%=\", \"*=\", \"|=\", \"+=\", \"<<\", \"<<=\", \">>\", \">>=\", \"^=\", \"**\", \"**=\", \"<=>\", \"??\" and \"??=\", got \"string#~~\"."
        );
        assert!(fixer().configure(&json!({"operators": []})).is_ok());
        assert!(
            fixer()
                .configure(&json!({"default": null, "operators": {"=": null}}))
                .is_ok()
        );
    }
}
