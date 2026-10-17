//! Port of `PhpCsFixer\Fixer\Whitespace\StatementIndentationFixer`.
use crate::analyzers::alternative_syntax_analyzer::AlternativeSyntaxAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::indentation_trait::{
    compute_new_line_content, extract_indent, is_new_line_token,
};
use crate::analyzers::preg::{Preg, preg_quote};
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind, Kind::*, Proto, Token, Tokens};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::Path;

const BLOCK_SIGNATURE_FIRST_TOKENS: &[Kind] = &[
    T_USE,
    T_IF,
    T_ELSE,
    T_ELSEIF,
    T_FOR,
    T_FOREACH,
    T_WHILE,
    T_DO,
    T_SWITCH,
    T_CASE,
    T_DEFAULT,
    T_TRY,
    T_CLASS,
    T_INTERFACE,
    T_TRAIT,
    T_EXTENDS,
    T_IMPLEMENTS,
    T_CONST,
    T_MATCH,
    T_ENUM,
];
const CONTROL_STRUCTURE_POSSIBIBLY_WITHOUT_BRACES_TOKENS: &[Kind] =
    &[T_IF, T_ELSE, T_ELSEIF, T_FOR, T_FOREACH, T_WHILE, T_DO];
const BLOCK_FIRST_TOKENS: &[Proto<'static>] = &[
    Proto::Char(b"{"),
    Proto::Kind(CT_DESTRUCTURING_BRACKET_OPEN),
    Proto::Kind(CT_USE_TRAIT),
    Proto::Kind(CT_GROUP_IMPORT_BRACE_OPEN),
    Proto::Kind(CT_PROPERTY_HOOK_BRACE_OPEN),
    Proto::Kind(T_ATTRIBUTE),
];
const PROPERTY_KEYWORDS: &[Kind] = &[
    T_VAR,
    T_PUBLIC,
    T_PROTECTED,
    T_PRIVATE,
    T_STATIC,
    T_READONLY,
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScopeType {
    Block,
    BlockSignature,
    Statement,
}

struct Scope {
    type_: ScopeType,
    end_index: usize,
    end_index_inclusive: bool,
    initial_indent: Vec<u8>,
    new_indent: Vec<u8>,
    is_indented_block: bool,
}

pub struct StatementIndentationFixer {
    stick_comment_to_next_continuous_control_statement: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    // Upstream's `bracesFixerCompatibility` mode is only used by the deprecated `braces` fixer, which Pun refuses.
    Box::new(StatementIndentationFixer {
        stick_comment_to_next_continuous_control_statement: false,
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

impl Fixer for StatementIndentationFixer {
    fn name(&self) -> &'static str {
        "statement_indentation"
    }
    fn priority(&self) -> i32 {
        -3
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &["stick_comment_to_next_continuous_control_statement"],
        )?;
        self.stick_comment_to_next_continuous_control_statement =
            o.bool("stick_comment_to_next_continuous_control_statement", false)?;
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        self.apply_fix(tokens);
    }
}

impl StatementIndentationFixer {
    fn apply_fix(&self, tokens: &mut Tokens) {
        let alternative_syntax_analyzer = AlternativeSyntaxAnalyzer;

        // PHP keeps `$endIndex` across loop iterations; some branches rely on its stale value.
        let mut end_index = tokens.count() - 1;
        if tokens[end_index].is_whitespace(None) {
            end_index -= 1;
        }

        let mut last_indent = extract_indent(&compute_new_line_content(tokens, 0));

        let mut scopes = vec![Scope {
            type_: ScopeType::Block,
            end_index,
            end_index_inclusive: true,
            initial_indent: last_indent.clone(),
            new_indent: Vec::new(),
            is_indented_block: false,
        }];

        let mut previous_line_initial_indent: Vec<u8> = Vec::new();
        let mut previous_line_new_indent: Vec<u8> = Vec::new();
        let mut no_braces_block_starts: HashSet<usize> = HashSet::new();
        let mut alternative_block_starts: HashMap<usize, usize> = HashMap::new();
        let mut case_block_starts: HashSet<usize> = HashSet::new();

        let mut index = 0;
        while index < tokens.count() {
            let token = tokens[index].clone();

            if no_braces_block_starts.contains(&index) {
                let initial_indent = last_indent.clone();
                scopes.push(Scope {
                    type_: ScopeType::Block,
                    end_index: find_statement_end_index(tokens, index, tokens.count() - 1) + 1,
                    end_index_inclusive: true,
                    initial_indent,
                    new_indent: Vec::new(),
                    is_indented_block: true,
                });
            }

            let prev_is_array = || {
                tokens
                    .get_prev_meaningful_token(index)
                    .map(|p| tokens[p].is_kind(T_ARRAY))
                    .expect("prev meaningful token")
            };

            if token.equals_any(BLOCK_FIRST_TOKENS, true)
                || (token.equals("(", true) && !prev_is_array())
                || alternative_block_starts.contains_key(&index)
                || case_block_starts.contains(&index)
            {
                let mut end_index_inclusive = true;

                // upstream first checks T_EXTENDS/T_IMPLEMENTS, which never start a block here
                if token.is_kind(CT_USE_TRAIT) {
                    end_index = tokens
                        .get_next_token_of_kind(index, &[";".into()], true)
                        .unwrap();
                } else if token.equals(":", true) {
                    if case_block_starts.contains(&index) {
                        (end_index, end_index_inclusive) =
                            find_case_block_end(&alternative_syntax_analyzer, tokens, index);
                    } else if alternative_syntax_analyzer
                        .belongs_to_alternative_syntax(tokens, index)
                    {
                        end_index = alternative_syntax_analyzer.find_alternative_syntax_block_end(
                            tokens,
                            alternative_block_starts[&index],
                        );
                    }
                } else if token.is_kind(CT_DESTRUCTURING_BRACKET_OPEN) {
                    end_index = tokens
                        .get_next_token_of_kind(
                            index,
                            &[CT_DESTRUCTURING_BRACKET_CLOSE.into()],
                            true,
                        )
                        .unwrap();
                } else if token.is_kind(CT_GROUP_IMPORT_BRACE_OPEN) {
                    end_index = tokens
                        .get_next_token_of_kind(index, &[CT_GROUP_IMPORT_BRACE_CLOSE.into()], true)
                        .unwrap();
                } else if token.equals("{", true) {
                    end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
                } else if token.equals("(", true) {
                    end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
                } else if token.is_kind(CT_PROPERTY_HOOK_BRACE_OPEN) {
                    end_index = tokens
                        .get_next_token_of_kind(index, &[CT_PROPERTY_HOOK_BRACE_CLOSE.into()], true)
                        .unwrap();
                } else {
                    end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, index);
                }

                let current = scopes.last().unwrap();
                let initial_indent = if current.type_ == ScopeType::BlockSignature {
                    current.initial_indent.clone()
                } else {
                    last_indent.clone()
                };

                scopes.push(Scope {
                    type_: ScopeType::Block,
                    end_index,
                    end_index_inclusive,
                    initial_indent,
                    new_indent: Vec::new(),
                    is_indented_block: true,
                });

                while scopes.last().is_some_and(|s| index >= s.end_index) {
                    scopes.pop();
                }
                index += 1;
                continue;
            }

            if token.is_kind(CT_ARRAY_BRACKET_OPEN) || (token.equals("(", true) && prev_is_array())
            {
                let block_type = if token.equals("(", true) {
                    Tokens::BLOCK_TYPE_PARENTHESIS
                } else {
                    Tokens::BLOCK_TYPE_ARRAY_BRACKET
                };
                scopes.push(Scope {
                    type_: ScopeType::Statement,
                    end_index: tokens.find_block_end(block_type, index),
                    end_index_inclusive: true,
                    initial_indent: previous_line_initial_indent.clone(),
                    new_indent: previous_line_new_indent.clone(),
                    is_indented_block: false,
                });
                index += 1;
                continue;
            }

            let is_property_start = is_property_start(tokens, index);
            if is_property_start || token.is_given_kind(BLOCK_SIGNATURE_FIRST_TOKENS) {
                let mut last_whitespace_index: Option<usize> = None;
                let mut closing_parenthesis_index: Option<usize> = None;

                end_index = index + 1;
                let max = tokens.count();
                while end_index < max {
                    let end_token = &tokens[end_index];
                    if end_token.equals("(", true) {
                        let c = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, end_index);
                        closing_parenthesis_index = Some(c);
                        end_index = c + 1;
                        continue;
                    }
                    if end_token.is_kind(CT_ARRAY_BRACKET_OPEN) {
                        end_index =
                            tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, end_index) + 1;
                        continue;
                    }
                    if end_token.equals_any(
                        &[
                            "{".into(),
                            ";".into(),
                            T_DOUBLE_ARROW.into(),
                            T_IMPLEMENTS.into(),
                        ],
                        true,
                    ) {
                        break;
                    }
                    if end_token.equals(":", true) {
                        if token.is_given_kind(&[T_CASE, T_DEFAULT]) {
                            case_block_starts.insert(end_index);
                        } else {
                            alternative_block_starts.insert(end_index, index);
                        }
                        break;
                    }
                    if !token.is_given_kind(CONTROL_STRUCTURE_POSSIBIBLY_WITHOUT_BRACES_TOKENS) {
                        end_index += 1;
                        continue;
                    }
                    if end_token.is_whitespace(None) {
                        last_whitespace_index = Some(end_index);
                        end_index += 1;
                        continue;
                    }
                    if !end_token.is_comment() {
                        let no_brace_block_start_index = last_whitespace_index.unwrap_or(end_index);
                        no_braces_block_starts.insert(no_brace_block_start_index);
                        end_index = closing_parenthesis_index.unwrap_or(index);
                        break;
                    }
                    end_index += 1;
                }

                let initial_indent = last_indent.clone();
                scopes.push(Scope {
                    type_: ScopeType::BlockSignature,
                    end_index,
                    end_index_inclusive: true,
                    initial_indent,
                    new_indent: Vec::new(),
                    is_indented_block: is_property_start
                        || token.is_given_kind(&[T_EXTENDS, T_IMPLEMENTS, T_CONST, T_CASE]),
                });
                index += 1;
                continue;
            }

            if token.is_kind(T_FUNCTION) {
                end_index = index + 1;
                let max = tokens.count();
                while end_index < max {
                    if tokens[end_index].equals("(", true) {
                        end_index =
                            tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, end_index) + 1;
                        continue;
                    }
                    if tokens[end_index].equals_any(&["{".into(), ";".into()], true) {
                        break;
                    }
                    end_index += 1;
                }
                let initial_indent = last_indent.clone();
                scopes.push(Scope {
                    type_: ScopeType::BlockSignature,
                    end_index,
                    end_index_inclusive: true,
                    initial_indent,
                    new_indent: Vec::new(),
                    is_indented_block: false,
                });
                index += 1;
                continue;
            }

            if token.is_whitespace(None) || (index > 0 && tokens[index - 1].is_kind(T_OPEN_TAG)) {
                let previous_open_tag_content = if tokens[index - 1].is_kind(T_OPEN_TAG) {
                    Preg::replace("/\\S/", "", &tokens[index - 1].content)
                } else {
                    Vec::new()
                };
                let mut content = [
                    &previous_open_tag_content[..],
                    if token.is_whitespace(None) {
                        &token.content[..]
                    } else {
                        b""
                    },
                ]
                .concat();

                if !Preg::r#match("/\\R/", &content) {
                    index += 1;
                    continue;
                }

                // Only used when it is a comment.
                let next_token = tokens
                    .as_slice()
                    .get(index + 1)
                    .filter(|t| t.is_comment())
                    .cloned();

                let current = scopes.last().unwrap();
                if current.type_ == ScopeType::Block || current.type_ == ScopeType::BlockSignature {
                    let mut indent = false;

                    if current.is_indented_block {
                        let mut first_non_whitespace_token_index: Option<usize> = None;
                        let mut next_newline_index: Option<usize> = None;
                        for search_index in index + 1..tokens.count() {
                            let search_token = &tokens[search_index];
                            if !search_token.is_whitespace(None) {
                                if first_non_whitespace_token_index.is_none() {
                                    first_non_whitespace_token_index = Some(search_index);
                                }
                                continue;
                            }
                            if Preg::r#match("/\\R/", &search_token.content) {
                                next_newline_index = Some(search_index);
                                break;
                            }
                        }

                        end_index = current.end_index;
                        if !current.end_index_inclusive {
                            end_index += 1;
                        }

                        if first_non_whitespace_token_index.is_some_and(|f| f < end_index)
                            || next_newline_index.is_some_and(|n| n < end_index)
                        {
                            let first = first_non_whitespace_token_index
                                .expect("first non-whitespace token");
                            if
                            // do we touch whitespace directly before comment...
                            tokens[first].is_kind(T_COMMENT)
                                // ...and afterwards, there is only comment or `}`
                                && tokens[tokens.get_next_meaningful_token(first).unwrap()].equals("}", true)
                            {
                                if
                                // ... and the comment was only content in docblock
                                tokens[tokens.get_prev_meaningful_token(first).unwrap()]
                                    .equals("{", true)
                                {
                                    indent = true;
                                } else {
                                    // or it was dedicated comment for next control loop
                                    // ^^ we need to check if there is a control group afterwards, and in that case don't make extra indent level
                                    let next_index =
                                        tokens.get_next_meaningful_token(first).unwrap();
                                    let next_next_index =
                                        tokens.get_next_meaningful_token(next_index);
                                    if next_next_index.is_some_and(|n| {
                                        tokens[n].is_given_kind(&[T_ELSE, T_ELSEIF])
                                    }) {
                                        indent = !self
                                            .stick_comment_to_next_continuous_control_statement;
                                    } else {
                                        indent = true;
                                    }
                                }
                            } else {
                                indent = true;
                            }
                        }
                    }

                    previous_line_initial_indent = extract_indent(&content);

                    // Only array statement scopes (never reaching here) and bracesFixerCompatibility blocks are `skip`.
                    let whitespaces = [
                        &current.initial_indent[..],
                        if indent {
                            self.whitespaces_config.get_indent().as_bytes()
                        } else {
                            b""
                        },
                    ]
                    .concat();

                    content = Preg::replace(
                        "/(\\R+)\\h*$/",
                        [&b"$1"[..], &whitespaces].concat(),
                        &content,
                    );

                    previous_line_new_indent = extract_indent(&content);
                } else {
                    let pattern = [&b"/(\\R)"[..], &current.initial_indent, b"(\\h*)$/D"].concat();
                    content = Preg::replace(
                        pattern,
                        [&b"$1"[..], &current.new_indent, b"$2"].concat(),
                        &content,
                    );
                }

                last_indent = extract_indent(&content);

                if !previous_open_tag_content.is_empty() {
                    content = Preg::replace(
                        [&b"/^"[..], &previous_open_tag_content, b"/"].concat(),
                        "",
                        &content,
                    );
                }

                if !content.is_empty() {
                    tokens.ensure_whitespace_at_index(index, 0, &content);
                } else if token.is_whitespace(None) {
                    tokens.clear_at(index);
                }

                if let Some(next_token) = next_token.filter(|t| t.is_comment()) {
                    let pattern = [
                        &b"/(\\R)"[..],
                        &preg_quote(&previous_line_initial_indent, Some(b'/')),
                        b"(\\h*\\S+.*)/",
                    ]
                    .concat();
                    let replaced = Preg::replace(
                        pattern,
                        [&b"$1"[..], &previous_line_new_indent, b"$2"].concat(),
                        &next_token.content,
                    );
                    tokens.set(
                        index + 1,
                        Token {
                            id: next_token.id,
                            content: replaced,
                        },
                    );
                }

                if token.is_whitespace(None) {
                    index += 1;
                    continue;
                }
            }

            if is_new_line_token(tokens, index) {
                last_indent = extract_indent(&compute_new_line_content(tokens, index));
            }

            while scopes.last().is_some_and(|s| index >= s.end_index) {
                scopes.pop();
                if scopes.is_empty() {
                    return;
                }
            }

            if token.is_comment()
                || token.equals_any(
                    &[
                        ";".into(),
                        ",".into(),
                        "}".into(),
                        T_OPEN_TAG.into(),
                        T_CLOSE_TAG.into(),
                        CT_ATTRIBUTE_CLOSE.into(),
                    ],
                    true,
                )
            {
                index += 1;
                continue;
            }

            let current = scopes.last().unwrap();
            if current.type_ != ScopeType::Statement && current.type_ != ScopeType::BlockSignature {
                end_index = find_statement_end_index(tokens, index, current.end_index);
                if end_index == index {
                    index += 1;
                    continue;
                }
                scopes.push(Scope {
                    type_: ScopeType::Statement,
                    end_index,
                    end_index_inclusive: false,
                    initial_indent: previous_line_initial_indent.clone(),
                    new_indent: previous_line_new_indent.clone(),
                    is_indented_block: true,
                });
            }
            index += 1;
        }
    }
}

fn find_statement_end_index(tokens: &Tokens, index: usize, parent_scope_end_index: usize) -> usize {
    let mut end_index: Option<usize> = None;
    let mut if_level = 0;
    let mut do_while_level = 0;
    let mut search_end_index = index;
    while search_end_index <= parent_scope_end_index {
        let mut search_end_token = &tokens[search_end_index];
        if search_end_token.is_kind(T_IF)
            && !tokens[tokens.get_prev_meaningful_token(search_end_index).unwrap()].is_kind(T_ELSE)
        {
            if_level += 1;
            search_end_index += 1;
            continue;
        }
        if search_end_token.is_kind(T_DO) {
            do_while_level += 1;
            search_end_index += 1;
            continue;
        }
        if search_end_token.equals_any(
            &["(".into(), "{".into(), CT_ARRAY_BRACKET_OPEN.into()],
            true,
        ) {
            let block_type = if search_end_token.equals("(", true) {
                Tokens::BLOCK_TYPE_PARENTHESIS
            } else if search_end_token.equals("{", true) {
                Tokens::BLOCK_TYPE_BRACE
            } else {
                Tokens::BLOCK_TYPE_ARRAY_BRACKET
            };
            search_end_index = tokens.find_block_end(block_type, search_end_index);
            search_end_token = &tokens[search_end_index];
        }
        if !search_end_token.equals_any(
            &[";".into(), ",".into(), "}".into(), T_CLOSE_TAG.into()],
            true,
        ) {
            search_end_index += 1;
            continue;
        }
        let control_structure_continuation_index =
            tokens.get_next_meaningful_token(search_end_index);
        if if_level > 0
            && let Some(c) = control_structure_continuation_index
                .filter(|&c| tokens[c].is_given_kind(&[T_ELSE, T_ELSEIF]))
        {
            if tokens[c].is_kind(T_ELSE)
                && !tokens[tokens.get_next_meaningful_token(c).unwrap()].is_kind(T_IF)
            {
                if_level -= 1;
            }
            search_end_index = c + 1;
            continue;
        }
        if do_while_level > 0
            && let Some(c) =
                control_structure_continuation_index.filter(|&c| tokens[c].is_kind(T_WHILE))
        {
            do_while_level -= 1;
            search_end_index = c + 1;
            continue;
        }
        end_index = tokens.get_prev_non_whitespace(search_end_index, None);
        break;
    }
    end_index.unwrap_or_else(|| {
        tokens
            .get_prev_meaningful_token(parent_scope_end_index)
            .unwrap()
    })
}

fn find_case_block_end(
    alternative_syntax_analyzer: &AlternativeSyntaxAnalyzer,
    tokens: &Tokens,
    mut index: usize,
) -> (usize, bool) {
    // A case block always ends at the next case/default or the switch's closing `}` / `endswitch`.
    loop {
        if tokens[index].is_kind(T_SWITCH) {
            let brace_index = tokens
                .get_next_meaningful_token(tokens.find_block_end(
                    Tokens::BLOCK_TYPE_PARENTHESIS,
                    tokens.get_next_meaningful_token(index).unwrap(),
                ))
                .unwrap();
            if tokens[brace_index].equals(":", true) {
                index =
                    alternative_syntax_analyzer.find_alternative_syntax_block_end(tokens, index);
            } else {
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, brace_index);
            }
            index += 1;
            continue;
        }
        if tokens[index].equals("{", true) {
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index) + 1;
            continue;
        }
        if tokens[index].is_given_kind(&[T_CASE, T_DEFAULT]) {
            return (index, true);
        }
        if tokens[index].equals_any(&["}".into(), T_ENDSWITCH.into()], true) {
            return (tokens.get_prev_non_whitespace(index, None).unwrap(), false);
        }
        index += 1;
    }
}

fn is_property_start(tokens: &Tokens, mut index: usize) -> bool {
    let next_index = tokens.get_next_meaningful_token(index);
    match next_index {
        None => return false,
        Some(n)
            if tokens[n].is_given_kind(PROPERTY_KEYWORDS)
                || tokens[n].is_given_kind(&[T_CONST, T_FUNCTION]) =>
        {
            return false;
        }
        _ => {}
    }
    while tokens[index].is_given_kind(PROPERTY_KEYWORDS) {
        if tokens[index].is_given_kind(&[T_VAR, T_PUBLIC, T_PROTECTED, T_PRIVATE]) {
            return true;
        }
        index = tokens.get_prev_meaningful_token(index).unwrap();
    }
    false
}
