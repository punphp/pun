//! Port of `PhpCsFixer\Fixer\DoctrineAnnotation\DoctrineAnnotationIndentationFixer`.
use crate::analyzers::doctrine_annotation::{self as doctrine, *};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::T_DOC_COMMENT;
use pun_tokens::Tokens;
use serde_json::Value;
use std::path::Path;

struct DoctrineAnnotationIndentationFixer {
    ignored_tags: Vec<String>,
    indent_mixed_lines: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(DoctrineAnnotationIndentationFixer {
        ignored_tags: vec![],
        indent_mixed_lines: false,
    })
}

impl Fixer for DoctrineAnnotationIndentationFixer {
    fn name(&self) -> &'static str {
        "doctrine_annotation_indentation"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(
            self.name(),
            options,
            &["ignored_tags", "indent_mixed_lines"],
        )?;
        self.ignored_tags = doctrine::ignored_tags(&o)?;
        self.indent_mixed_lines = o.bool("indent_mixed_lines", false)?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        doctrine::apply_fix(tokens, &self.ignored_tags, |doc| self.fix_annotations(doc));
    }
}

fn is_newline(token: &DocToken) -> bool {
    token.kind == T_NONE && token.content.contains(&b'\n')
}

impl DoctrineAnnotationIndentationFixer {
    fn fix_annotations(&self, tokens: &mut DocTokens) {
        let mut annotation_positions = vec![];
        let mut index = 0;
        while index < tokens.len() {
            if tokens[index].kind == T_AT {
                let end = tokens.get_annotation_end(index);
                annotation_positions.push((index, end));
                index = end;
            }
            index += 1;
        }

        let mut indent_level: usize = 0;
        for index in 0..tokens.len() {
            if !is_newline(&tokens[index])
                || !indentation_can_be_fixed(tokens, index, &annotation_positions)
            {
                continue;
            }
            let (opening, closing) = get_line_braces_count(tokens, index);
            let delta = opening as isize - closing as isize;
            let mixed_braces = delta == 0 && opening > 0;
            let mut extra_indent_level = 0;
            if indent_level > 0 && (delta < 0 || mixed_braces) {
                indent_level -= 1;
                if self.indent_mixed_lines && is_closing_line_with_meaningful_content(tokens, index)
                {
                    extra_indent_level = 1;
                }
            }
            let replacement = [
                &b"$1"[..],
                &b" ".repeat(4 * (indent_level + extra_indent_level) + 1),
            ]
            .concat();
            tokens[index].content =
                Preg::replace(r"/(\n( +\*)?) *$/", replacement, &tokens[index].content);
            if delta > 0 || mixed_braces {
                indent_level += 1;
            }
        }
    }
}

fn get_line_braces_count(tokens: &DocTokens, index: usize) -> (usize, usize) {
    let (mut opening, mut closing) = (0, 0);
    for token in &tokens.0[index + 1..] {
        if is_newline(token) {
            break;
        }
        if token.is_type(&[T_OPEN_PARENTHESIS, T_OPEN_CURLY_BRACES]) {
            opening += 1;
        } else if token.is_type(&[T_CLOSE_PARENTHESIS, T_CLOSE_CURLY_BRACES]) {
            if opening > 0 {
                opening -= 1;
            } else {
                closing += 1;
            }
        }
    }
    (opening, closing)
}

/// Only called on a line whose braces don't balance, so a non-T_NONE token follows on the line.
fn is_closing_line_with_meaningful_content(tokens: &DocTokens, index: usize) -> bool {
    let token = tokens.0[index + 1..]
        .iter()
        .find(|t| t.kind != T_NONE || t.content.contains(&b'\n'));
    token.is_some_and(|t| {
        t.kind != T_NONE && !t.is_type(&[T_CLOSE_PARENTHESIS, T_CLOSE_CURLY_BRACES])
    })
}

fn indentation_can_be_fixed(
    tokens: &DocTokens,
    new_line_index: usize,
    positions: &[(usize, usize)],
) -> bool {
    if positions
        .iter()
        .any(|&(start, end)| new_line_index >= start && new_line_index <= end)
    {
        return true;
    }
    // Upstream's loop returns on its first iteration: only the token after the line break counts.
    tokens
        .0
        .get(new_line_index + 1)
        .is_some_and(|t| !t.content.contains(&b'\n') && t.kind == T_AT)
}
