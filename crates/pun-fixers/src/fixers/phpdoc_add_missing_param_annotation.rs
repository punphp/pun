//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocAddMissingParamAnnotationFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::helpers::contains;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct PhpdocAddMissingParamAnnotationFixer {
    only_untyped: bool,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocAddMissingParamAnnotationFixer {
        only_untyped: false,
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

#[derive(Default)]
struct ArgumentInformation {
    default: Vec<u8>,
    name: Vec<u8>,
    type_: Vec<u8>,
}

impl Fixer for PhpdocAddMissingParamAnnotationFixer {
    fn name(&self) -> &'static str {
        "phpdoc_add_missing_param_annotation"
    }
    fn priority(&self) -> i32 {
        10
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["only_untyped"])?;
        self.only_untyped = o.bool("only_untyped", true)?;
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let arguments_analyzer = ArgumentsAnalyzer;
        let limit = tokens.count();
        let mut index = 0;
        while index < limit {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                index += 1;
                continue;
            }
            let token_content = tokens[index].get_content().to_vec();
            if contains(&strtolower(&token_content), b"inheritdoc") {
                index += 1;
                continue;
            }
            // ignore one-line phpdocs like `/** foo */`, as there is no place to put new annotations
            if !token_content.contains(&b'\n') {
                index += 1;
                continue;
            }
            let main_index = index;
            let Some(next) = tokens.get_next_meaningful_token(index) else {
                return;
            };
            index = next;
            while tokens[index].is_given_kind(&[
                T_ABSTRACT,
                T_FINAL,
                T_PRIVATE,
                T_PROTECTED,
                T_PUBLIC,
                T_STATIC,
            ]) {
                index = tokens.get_next_meaningful_token(index).unwrap();
            }
            if !tokens[index].is_kind(T_FUNCTION) {
                index += 1;
                continue;
            }
            let open_index = tokens
                .get_next_token_of_kind(index, &["(".into()], true)
                .unwrap();
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_index);

            // An ordered map keyed by name, as upstream's array: a repeated name keeps its first position.
            let mut arguments: Vec<ArgumentInformation> = Vec::new();
            for (start, end) in arguments_analyzer.get_arguments(tokens, open_index, index) {
                let info = prepare_argument_information(tokens, start, end);
                if !self.only_untyped || info.type_.is_empty() {
                    match arguments.iter_mut().find(|a| a.name == info.name) {
                        Some(a) => *a = info,
                        None => arguments.push(info),
                    }
                }
            }
            if arguments.is_empty() {
                index += 1;
                continue;
            }
            let doc = DocBlock::new(&token_content);
            let mut last_param_line: usize = 0;
            for annotation in doc.get_annotations_of_type(&["param"]) {
                if let Some(m) =
                    Preg::match_groups(r"/^[^$]+(\$\w+).*$/s", &annotation.get_content())
                {
                    let name = m.str(1).to_vec();
                    arguments.retain(|a| a.name != name);
                }
                last_param_line = last_param_line.max(annotation.get_end());
            }
            if arguments.is_empty() {
                index += 1;
                continue;
            }
            let mut lines: Vec<Vec<u8>> = doc
                .get_lines()
                .iter()
                .map(|l| l.borrow().get_content().to_vec())
                .collect();
            let lines_count = lines.len();
            let last = lines[lines_count - 1].clone();
            let indent = Preg::match_groups(r"/^(\s*).*$/", &last)
                .unwrap()
                .str(1)
                .to_vec();
            let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
            let new_lines: Vec<Vec<u8>> = arguments
                .iter()
                .map(|argument| {
                    let mut type_ = if argument.type_.is_empty() {
                        b"mixed".to_vec()
                    } else {
                        argument.type_.clone()
                    };
                    if !type_.starts_with(b"?") && strtolower(&argument.default) == b"null" {
                        type_ = [&b"null|"[..], &type_].concat();
                    }
                    [
                        &indent[..],
                        b"* @param ",
                        &type_,
                        b" ",
                        &argument.name,
                        line_ending,
                    ]
                    .concat()
                })
                .collect();
            let at = if last_param_line > 0 {
                last_param_line + 1
            } else {
                lines_count - 1
            };
            lines.splice(at..at, new_lines);
            tokens.set(main_index, Token::new(T_DOC_COMMENT, lines.concat()));
            index += 1;
        }
    }
}

fn prepare_argument_information(tokens: &Tokens, start: usize, end: usize) -> ArgumentInformation {
    let mut info = ArgumentInformation::default();
    let mut saw_name = false;
    let mut index = start;
    while index <= end {
        let token = &tokens[index];
        if token.is_kind(T_ATTRIBUTE) {
            index = tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, index) + 1;
            continue;
        }
        if token.is_comment()
            || token.is_whitespace(None)
            || token.is_given_kind(&[
                CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
                CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
                CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
                T_READONLY,
                T_PRIVATE_SET,
                T_PROTECTED_SET,
                T_PUBLIC_SET,
            ])
        {
            index += 1;
            continue;
        }
        if token.is_kind(T_VARIABLE) {
            saw_name = true;
            info.name = token.get_content().to_vec();
        } else if token.equals("=", true) {
        } else if saw_name {
            info.default.extend_from_slice(token.get_content());
        } else if !token.equals("&", true) {
            if token.is_kind(T_ELLIPSIS) {
                if info.type_.is_empty() {
                    info.type_ = b"array".to_vec();
                } else {
                    info.type_.extend_from_slice(b"[]");
                }
            } else {
                info.type_.extend_from_slice(token.get_content());
            }
        }
        index += 1;
    }
    info
}
