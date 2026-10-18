//! Port of `PhpCsFixer\Fixer\Basic\BracesPositionFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::indentation_trait::get_line_indentation;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind, Kind::*, Token, Tokens, TokensAnalyzer, token::CLASSY_KINDS};
use serde_json::Value;
use std::path::Path;

const NEXT_LINE_UNLESS_NEWLINE_AT_SIGNATURE_END: &str = "next_line_unless_newline_at_signature_end";
const SAME_LINE: &str = "same_line";
const POSITIONS: &[(&str, Position)] = &[
    (
        NEXT_LINE_UNLESS_NEWLINE_AT_SIGNATURE_END,
        Position::NextLineUnlessNewlineAtSignatureEnd,
    ),
    (SAME_LINE, Position::SameLine),
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Position {
    NextLineUnlessNewlineAtSignatureEnd,
    SameLine,
}

/// Which `*_opening_brace` option applies to a brace.
#[derive(Clone, Copy)]
enum Brace {
    ControlStructures,
    Functions,
    AnonymousFunctions,
    Classes,
    AnonymousClasses,
}
const CONTROL_STRUCTURE_TOKENS: &[Kind] = &[
    T_DECLARE, T_DO, T_ELSE, T_ELSEIF, T_FINALLY, T_FOR, T_FOREACH, T_IF, T_WHILE, T_TRY, T_CATCH,
    T_SWITCH, T_MATCH,
];

struct BracesPositionFixer {
    control_structures_opening_brace: Position,
    functions_opening_brace: Position,
    anonymous_functions_opening_brace: Position,
    classes_opening_brace: Position,
    anonymous_classes_opening_brace: Position,
    allow_single_line_empty_anonymous_classes: bool,
    allow_single_line_anonymous_functions: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(BracesPositionFixer {
        control_structures_opening_brace: Position::SameLine,
        functions_opening_brace: Position::SameLine,
        anonymous_functions_opening_brace: Position::SameLine,
        classes_opening_brace: Position::SameLine,
        anonymous_classes_opening_brace: Position::SameLine,
        allow_single_line_empty_anonymous_classes: true,
        allow_single_line_anonymous_functions: true,
        whitespaces_config: Default::default(),
    })
}

impl BracesPositionFixer {
    fn find_parenthesis_end(tokens: &Tokens, structure_token_index: usize) -> usize {
        let next_index = tokens
            .get_next_meaningful_token(structure_token_index)
            .unwrap();
        if !tokens[next_index].equals("(", true) {
            return structure_token_index;
        }
        tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, next_index)
    }

    fn is_followed_by_new_line(tokens: &Tokens, mut index: usize) -> bool {
        index += 1;
        let max = tokens.count() - 1;
        while index < max {
            let token = &tokens[index];
            if !token.is_comment() {
                return token.is_whitespace(None) && Preg::r#match("/\\R/", token.get_content());
            }
            index += 1;
        }
        false
    }

    fn has_comment_on_same_line(tokens: &Tokens, index: usize) -> bool {
        let mut token = &tokens[index + 1];
        if token.is_whitespace(None) && !Preg::r#match("/\\R/", token.get_content()) {
            token = &tokens[index + 2];
        }
        token.is_comment()
    }

    fn position(&self, brace: Brace) -> Position {
        match brace {
            Brace::ControlStructures => self.control_structures_opening_brace,
            Brace::Functions => self.functions_opening_brace,
            Brace::AnonymousFunctions => self.anonymous_functions_opening_brace,
            Brace::Classes => self.classes_opening_brace,
            Brace::AnonymousClasses => self.anonymous_classes_opening_brace,
        }
    }
}

impl Fixer for BracesPositionFixer {
    fn name(&self) -> &'static str {
        "braces_position"
    }
    fn priority(&self) -> i32 {
        // Must run before SingleLineEmptyBodyFixer, StatementIndentationFixer.
        // Must run after ControlStructureBracesFixer, MultilinePromotedPropertiesFixer, NoMultipleStatementsPerLineFixer.
        -2
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(
            self.name(),
            options,
            &[
                "control_structures_opening_brace",
                "functions_opening_brace",
                "anonymous_functions_opening_brace",
                "classes_opening_brace",
                "anonymous_classes_opening_brace",
                "allow_single_line_empty_anonymous_classes",
                "allow_single_line_anonymous_functions",
            ],
        )?;
        self.control_structures_opening_brace =
            r.choice("control_structures_opening_brace", SAME_LINE, POSITIONS)?;
        self.functions_opening_brace = r.choice(
            "functions_opening_brace",
            NEXT_LINE_UNLESS_NEWLINE_AT_SIGNATURE_END,
            POSITIONS,
        )?;
        self.anonymous_functions_opening_brace =
            r.choice("anonymous_functions_opening_brace", SAME_LINE, POSITIONS)?;
        self.classes_opening_brace = r.choice(
            "classes_opening_brace",
            NEXT_LINE_UNLESS_NEWLINE_AT_SIGNATURE_END,
            POSITIONS,
        )?;
        self.anonymous_classes_opening_brace =
            r.choice("anonymous_classes_opening_brace", SAME_LINE, POSITIONS)?;
        self.allow_single_line_empty_anonymous_classes =
            r.bool("allow_single_line_empty_anonymous_classes", true)?;
        // Future::getV4OrV3(false, true)
        self.allow_single_line_anonymous_functions =
            r.bool("allow_single_line_anonymous_functions", true)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("{")
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let mut allow_single_line_until: Option<usize> = None;
        let mut foreach_index = 0;
        while foreach_index < tokens.count() {
            let mut index = foreach_index;
            foreach_index += 1;
            let kind = tokens[index].id;
            let is = |kinds: &[Kind]| kind.is_some_and(|k| kinds.contains(&k));
            let mut allow_single_line = false;
            let mut allow_single_line_if_empty = false;
            let mut open_brace_index;
            let position_option;
            if is(CLASSY_KINDS) {
                open_brace_index = tokens
                    .get_next_token_of_kind(index, &["{".into()], true)
                    .unwrap();
                if TokensAnalyzer::new(tokens).is_anonymous_class(index) {
                    allow_single_line_if_empty = self.allow_single_line_empty_anonymous_classes;
                    position_option = Brace::AnonymousClasses;
                } else {
                    position_option = Brace::Classes;
                }
            } else if kind == Some(T_FUNCTION) {
                open_brace_index = tokens
                    .get_next_token_of_kind(
                        index,
                        &["{".into(), ";".into(), CT_PROPERTY_HOOK_BRACE_OPEN.into()],
                        true,
                    )
                    .unwrap();
                if !tokens[open_brace_index].equals("{", true) {
                    continue;
                }
                if TokensAnalyzer::new(tokens).is_lambda(index) {
                    allow_single_line = self.allow_single_line_anonymous_functions;
                    position_option = Brace::AnonymousFunctions;
                } else {
                    position_option = Brace::Functions;
                }
            } else if is(CONTROL_STRUCTURE_TOKENS) {
                let parenthesis_end_index = Self::find_parenthesis_end(tokens, index);
                open_brace_index = tokens
                    .get_next_meaningful_token(parenthesis_end_index)
                    .unwrap();
                if !tokens[open_brace_index].equals("{", true) {
                    continue;
                }
                position_option = Brace::ControlStructures;
            } else if kind == Some(T_VARIABLE) {
                // handle default value - explicitly skip array as default value
                let mut next_meaningful_index = tokens.get_next_meaningful_token(index).unwrap();
                if tokens[next_meaningful_index].equals("=", true) {
                    next_meaningful_index = tokens
                        .get_next_meaningful_token(next_meaningful_index)
                        .unwrap();
                    if tokens[next_meaningful_index].is_kind(CT_ARRAY_BRACKET_OPEN) {
                        index = tokens.find_block_end(
                            Tokens::BLOCK_TYPE_ARRAY_BRACKET,
                            next_meaningful_index,
                        );
                    } else if tokens[next_meaningful_index].is_kind(T_ARRAY) {
                        next_meaningful_index = tokens
                            .get_next_meaningful_token(next_meaningful_index)
                            .unwrap();
                        if tokens[next_meaningful_index].equals("(", true) {
                            index = tokens.find_block_end(
                                Tokens::BLOCK_TYPE_PARENTHESIS,
                                next_meaningful_index,
                            );
                        }
                    }
                }
                open_brace_index = match tokens.get_next_token_of_kind(
                    index,
                    &[
                        "{".into(),
                        ";".into(),
                        ".".into(),
                        CT_CURLY_CLOSE.into(),
                        CT_PROPERTY_HOOK_BRACE_OPEN.into(),
                        T_ENCAPSED_AND_WHITESPACE.into(),
                        T_CLOSE_TAG.into(),
                    ],
                    true,
                ) {
                    Some(o) if tokens[o].is_kind(CT_PROPERTY_HOOK_BRACE_OPEN) => o,
                    _ => continue,
                };
                let close_brace_index =
                    tokens.find_block_end(Tokens::BLOCK_TYPE_PROPERTY_HOOK, open_brace_index);
                if !tokens.is_partial_code_multiline(open_brace_index, close_brace_index) {
                    continue;
                }
                position_option = Brace::ControlStructures;
            } else if kind == Some(T_STRING) {
                let parenthesis_end_index = Self::find_parenthesis_end(tokens, index);
                open_brace_index = tokens
                    .get_next_meaningful_token(parenthesis_end_index)
                    .unwrap();
                if !tokens[open_brace_index].equals("{", true) {
                    continue;
                }
                let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
                if !tokens[prev_index].equals_any(
                    &[
                        "}".into(),
                        ";".into(),
                        CT_ATTRIBUTE_CLOSE.into(),
                        CT_PROPERTY_HOOK_BRACE_OPEN.into(),
                    ],
                    true,
                ) {
                    continue;
                }
                allow_single_line = self.allow_single_line_anonymous_functions;
                position_option = Brace::ControlStructures;
            } else {
                continue;
            }

            let mut close_brace_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, open_brace_index);
            let mut add_newlines_inside_braces = true;
            // `$index < null` is always false in PHP
            if allow_single_line
                || allow_single_line_if_empty
                || allow_single_line_until.is_some_and(|u| index < u)
            {
                add_newlines_inside_braces = false;
                for index_inside_braces in open_brace_index + 1..close_brace_index {
                    let token_inside_braces = &tokens[index_inside_braces];
                    if (allow_single_line_if_empty
                        && !token_inside_braces.is_whitespace(None)
                        && !token_inside_braces.is_comment())
                        || (token_inside_braces.is_whitespace(None)
                            && Preg::r#match("/\\R/", token_inside_braces.get_content()))
                    {
                        add_newlines_inside_braces = true;
                        break;
                    }
                }
                if !add_newlines_inside_braces && allow_single_line_until.is_none() {
                    allow_single_line_until = Some(close_brace_index);
                }
            }

            if add_newlines_inside_braces
                && !Self::is_followed_by_new_line(tokens, open_brace_index)
                && !Self::has_comment_on_same_line(tokens, open_brace_index)
                && !tokens[tokens.get_next_meaningful_token(open_brace_index).unwrap()]
                    .is_kind(T_CLOSE_TAG)
            {
                let whitespace =
                    [line_ending, &get_line_indentation(tokens, open_brace_index)].concat();
                if tokens.ensure_whitespace_at_index(open_brace_index + 1, 0, &whitespace) {
                    close_brace_index += 1;
                }
            }

            let mut whitespace: Vec<u8> = b" ".to_vec();
            if self.position(position_option) == Position::NextLineUnlessNewlineAtSignatureEnd {
                whitespace = [line_ending, &get_line_indentation(tokens, index)].concat();
                let mut previous_token_index = open_brace_index;
                loop {
                    previous_token_index = tokens
                        .get_prev_meaningful_token(previous_token_index)
                        .unwrap();
                    if !tokens[previous_token_index].is_given_kind(&[
                        CT_TYPE_COLON,
                        CT_NULLABLE_TYPE,
                        T_STRING,
                        T_NS_SEPARATOR,
                        CT_ARRAY_TYPEHINT,
                        T_STATIC,
                        CT_TYPE_ALTERNATION,
                        CT_TYPE_INTERSECTION,
                        T_CALLABLE,
                        CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_OPEN,
                        CT_DISJUNCTIVE_NORMAL_FORM_TYPE_PARENTHESIS_CLOSE,
                    ]) {
                        break;
                    }
                }
                if tokens[previous_token_index].equals(")", true) {
                    previous_token_index -= 1;
                    if tokens[previous_token_index].is_comment() {
                        previous_token_index -= 1;
                    }
                    if tokens[previous_token_index].is_whitespace(None)
                        && Preg::r#match("/\\R/", tokens[previous_token_index].get_content())
                    {
                        whitespace = b" ".to_vec();
                    }
                }
            }

            let mut move_brace_to_index: Option<usize> = None;
            if whitespace == b" " {
                let previous_meaningful_index =
                    tokens.get_prev_meaningful_token(open_brace_index).unwrap();
                let mut index_before_open_brace = open_brace_index - 1;
                while index_before_open_brace > previous_meaningful_index {
                    if !tokens[index_before_open_brace].is_comment() {
                        index_before_open_brace -= 1;
                        continue;
                    }
                    index_before_open_brace -= 1;
                    let token_before_open_brace = &tokens[index_before_open_brace];
                    if token_before_open_brace.is_whitespace(None) {
                        move_brace_to_index = Some(index_before_open_brace);
                    } else if index_before_open_brace == previous_meaningful_index {
                        move_brace_to_index = Some(previous_meaningful_index + 1);
                    }
                    index_before_open_brace -= 1; // >= previous_meaningful_index >= 1: no underflow
                }
            } else if !tokens[open_brace_index - 1].is_whitespace(None)
                || !Preg::r#match("/\\R/", tokens[open_brace_index - 1].get_content())
            {
                for index_after_open_brace in open_brace_index + 1..close_brace_index {
                    if tokens[index_after_open_brace].is_whitespace(None)
                        && Preg::r#match("/\\R/", tokens[index_after_open_brace].get_content())
                    {
                        break;
                    }
                    if tokens[index_after_open_brace].is_comment()
                        && !tokens[index_after_open_brace]
                            .get_content()
                            .starts_with(b"/*")
                    {
                        move_brace_to_index = Some(index_after_open_brace + 1);
                    }
                }
            }

            if let Some(move_to) = move_brace_to_index {
                let moved_token = tokens[open_brace_index].clone();
                let delta: isize = if open_brace_index < move_to { 1 } else { -1 };
                if tokens[open_brace_index.wrapping_add_signed(delta)].is_whitespace(None) {
                    if delta == -1 && Preg::r#match("/\\R/", tokens[open_brace_index - 1].get_content()) {
                        let content = Preg::replace(
                            "/^(\\h*?\\R)?\\h*/",
                            "",
                            tokens[open_brace_index + 1].get_content(),
                        );
                        if !content.is_empty() {
                            tokens.set(open_brace_index + 1, Token::new(T_WHITESPACE, content));
                        } else {
                            tokens.clear_at(open_brace_index + 1);
                        }
                    } else if tokens[open_brace_index - 1].is_whitespace(None) {
                        tokens.clear_at(open_brace_index - 1);
                    }
                }
                let mut i = open_brace_index;
                while i != move_to {
                    let sibling_token = tokens[i.wrapping_add_signed(delta)].clone();
                    tokens.set(i, sibling_token);
                    i = i.wrapping_add_signed(delta);
                }
                tokens.set(i, moved_token);
                if tokens[open_brace_index].is_whitespace(None)
                    && tokens[open_brace_index + 1].is_whitespace(None)
                {
                    let content = [
                        tokens[open_brace_index].get_content(),
                        tokens[open_brace_index + 1].get_content(),
                    ]
                    .concat();
                    tokens.set(open_brace_index, Token::new(T_WHITESPACE, content));
                    tokens.clear_at(open_brace_index + 1);
                }
                open_brace_index = move_to;
            }

            if tokens.ensure_whitespace_at_index(open_brace_index - 1, 1, &whitespace) {
                close_brace_index += 1;
                if let Some(u) = allow_single_line_until.as_mut() {
                    *u += 1;
                }
            }

            if !add_newlines_inside_braces
                || tokens[tokens.get_prev_meaningful_token(close_brace_index).unwrap()]
                    .is_kind(T_OPEN_TAG)
            {
                continue;
            }
            let mut prev_index = close_brace_index - 1;
            while tokens.is_empty_at(prev_index) {
                prev_index -= 1;
            }
            let prev_token = &tokens[prev_index];
            if prev_token.is_whitespace(None) && Preg::r#match("/\\R/", prev_token.get_content()) {
                continue;
            }
            let whitespace =
                [line_ending, &get_line_indentation(tokens, open_brace_index)].concat();
            tokens.ensure_whitespace_at_index(prev_index, 1, &whitespace);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Both rejected by the real fixer's configure() (undefined option / value not allowed).
    #[test]
    fn configure_rejects_invalid_options() {
        assert!(fixer().configure(&json!({"foo": true})).is_err());
        assert!(
            fixer()
                .configure(&json!({"classes_opening_brace": "next_line"}))
                .is_err()
        );
    }
}
