//! Port of `PhpCsFixer\Fixer\Strict\DeclareStrictTypesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::{Result, bail};
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, php_compat};
use serde_json::Value;
use std::path::Path;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Strategy {
    Remove,
    AddWhenMissing,
    Enforce,
}

struct DeclareStrictTypesFixer {
    strategy: Strategy,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    // `configure` sets the strategy.
    crate::configured(DeclareStrictTypesFixer {
        strategy: Strategy::Remove,
        whitespaces_config: Default::default(),
    })
}

struct Declaration {
    declare_index: usize,
    open_parenthesis: usize,
    close_parenthesis: usize,
    /// index => token at the time of the lookup
    sequence: Vec<(usize, Token)>,
}

impl Fixer for DeclareStrictTypesFixer {
    fn name(&self) -> &'static str {
        "declare_strict_types"
    }
    fn priority(&self) -> i32 {
        2
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_monolithic_php() && !tokens.is_token_kind_found(T_OPEN_TAG_WITH_ECHO)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &["preserve_existing_declaration", "strategy"],
        )?;
        let strategy = match o
            .string(
                "strategy",
                "enforce",
                &["remove", "add_when_missing", "enforce"],
            )?
            .as_str()
        {
            "remove" => Strategy::Remove,
            "add_when_missing" => Strategy::AddWhenMissing,
            _ => Strategy::Enforce,
        };
        let preserve = o.bool("preserve_existing_declaration", false)?;
        if preserve && strategy != Strategy::Enforce {
            bail!(
                "[declare_strict_types] Cannot configure \"strategy\" in parallel to configuring \"preserve_existing_declaration\"."
            );
        }
        self.strategy = if strategy == Strategy::Enforce && preserve {
            Strategy::AddWhenMissing
        } else {
            strategy
        };
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let open_tag_index = if tokens[0].is_kind(T_INLINE_HTML) {
            1
        } else {
            0
        };
        let declaration = get_strict_types_declaration(tokens);
        if self.strategy == Strategy::Remove {
            if let Some(declaration) = declaration {
                remove_strict_types_declaration(tokens, &declaration);
            }
            return;
        }
        let Some(declaration) = declaration else {
            self.insert_sequence(open_tag_index, tokens);
            return;
        };
        self.fix_strict_types_casing_and_value(tokens, &declaration.sequence);
    }
}

fn get_strict_types_declaration(tokens: &Tokens) -> Option<Declaration> {
    for index in tokens.find_given_kind(T_DECLARE, 0, None) {
        let open_parenthesis = tokens.get_next_meaningful_token(index).unwrap();
        let close_parenthesis =
            tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis);
        let Some(seq) = tokens.find_sequence(
            &[
                (T_STRING, "strict_types").into(),
                "=".into(),
                T_LNUMBER.into(),
            ],
            open_parenthesis,
            Some(close_parenthesis),
            |_| false,
        ) else {
            continue;
        };
        return Some(Declaration {
            declare_index: index,
            open_parenthesis,
            close_parenthesis,
            sequence: seq.into_iter().map(|i| (i, tokens[i].clone())).collect(),
        });
    }
    None
}

fn remove_strict_types_declaration(tokens: &mut Tokens, declaration: &Declaration) {
    let sequence_indices: Vec<usize> = declaration.sequence.iter().map(|(i, _)| *i).collect();
    let sequence_start = sequence_indices[0];
    let sequence_end = *sequence_indices.last().unwrap();

    if !has_other_declare_options(
        tokens,
        declaration.open_parenthesis,
        declaration.close_parenthesis,
        &sequence_indices,
    ) {
        remove_declare_statement(
            tokens,
            declaration.declare_index,
            declaration.close_parenthesis,
        );
        return;
    }

    // Backwards, clearing a token only ever merges away whitespace after it, so nothing ahead is empty yet.
    for index in (sequence_start..=sequence_end).rev() {
        tokens.clear_token_and_merge_surrounding_whitespace(index);
    }

    if let Some(prev_meaningful) = tokens
        .get_prev_meaningful_token(sequence_start)
        .filter(|&p| tokens[p].equals(",", true))
    {
        tokens.clear_token_and_merge_surrounding_whitespace(prev_meaningful);
        if let Some(next_meaningful) = tokens.get_next_meaningful_token(sequence_end) {
            tokens.remove_leading_whitespace(next_meaningful, Some(b" \t"));
        }
        return;
    }

    // Another option follows, so the next meaningful token is its `,` and something comes after it.
    let next_meaningful = tokens.get_next_meaningful_token(sequence_end).unwrap();
    tokens.clear_token_and_merge_surrounding_whitespace(next_meaningful);
    let after_comma = tokens.get_next_meaningful_token(next_meaningful).unwrap();
    tokens.remove_leading_whitespace(after_comma, Some(b" \t"));
}

fn has_other_declare_options(
    tokens: &Tokens,
    open_parenthesis: usize,
    close_parenthesis: usize,
    sequence: &[usize],
) -> bool {
    let mut index = tokens.get_next_meaningful_token(open_parenthesis);
    while let Some(i) = index.filter(|&i| i < close_parenthesis) {
        if tokens[i].is_kind(T_STRING) && !sequence.contains(&i) {
            return true;
        }
        index = tokens.get_next_meaningful_token(i);
    }
    false
}

fn remove_declare_statement(tokens: &mut Tokens, declare_index: usize, close_parenthesis: usize) {
    // upstream returns when there is none, which parsed code rules out (`;` or `?>` ends the statement)
    let semicolon_index = tokens.get_next_meaningful_token(close_parenthesis).unwrap();
    for index in (declare_index..=semicolon_index).rev() {
        tokens.clear_token_and_merge_surrounding_whitespace(index);
    }
}

impl DeclareStrictTypesFixer {
    fn fix_strict_types_casing_and_value(&self, tokens: &mut Tokens, sequence: &[(usize, Token)]) {
        for (index, token) in sequence {
            if token.is_kind(T_STRING) {
                tokens.set(
                    *index,
                    Token::new(T_STRING, php_compat::strtolower(&token.content)),
                );
                continue;
            }
            if token.is_kind(T_LNUMBER) && self.strategy == Strategy::Enforce {
                tokens.set(*index, Token::new(T_LNUMBER, "1"));
                break;
            }
        }
    }

    fn insert_sequence(&self, open_tag_index: usize, tokens: &mut Tokens) {
        let sequence = vec![
            Token::new(T_DECLARE, "declare"),
            Token::char("("),
            Token::new(T_STRING, "strict_types"),
            Token::char("="),
            Token::new(T_LNUMBER, "1"),
            Token::char(")"),
            Token::char(";"),
        ];
        let next_index = open_tag_index + sequence.len() + 1;
        tokens.insert_at(open_tag_index + 1, sequence);

        let content = tokens[open_tag_index].content.clone();
        if !content.contains(&b' ') || content.contains(&b'\n') {
            let id = tokens[open_tag_index].id;
            tokens.set(
                open_tag_index,
                Token {
                    id,
                    content: [php_compat::trim(&content), b" "].concat(),
                },
            );
        }

        if tokens.count() == next_index {
            return; // no more tokens after sequence, single_blank_line_at_eof might add a line
        }

        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        if tokens[next_index].is_whitespace(None) {
            let content = php_compat::ltrim_chars(&tokens[next_index].content, b" \t").to_vec();
            tokens.set(
                next_index,
                Token::new(T_WHITESPACE, [line_ending, &content].concat()),
            );
        } else {
            tokens.insert_at(
                next_index,
                vec![Token::new(T_WHITESPACE, line_ending.to_vec())],
            );
        }
    }
}
