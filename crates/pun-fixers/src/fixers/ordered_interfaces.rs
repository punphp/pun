//! Port of `PhpCsFixer\Fixer\ClassNotation\OrderedInterfacesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::php_compat::{str_replace, strcasecmp};
use pun_tokens::{Kind, Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
struct OrderedInterfacesFixer {
    order_length: bool,
    descend: bool,
    case_sensitive: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(OrderedInterfacesFixer::default())
}

impl Fixer for OrderedInterfacesFixer {
    fn name(&self) -> &'static str {
        "ordered_interfaces"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(
            self.name(),
            options,
            &["order", "direction", "case_sensitive"],
        )?;
        self.order_length = r.string("order", "alpha", &["alpha", "length"])? == "length";
        self.descend = r.string("direction", "ascend", &["ascend", "descend"])? == "descend";
        self.case_sensitive = r.bool("case_sensitive", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_IMPLEMENTS)
            || tokens.is_all_token_kinds_found(&[Kind::T_INTERFACE, Kind::T_EXTENDS])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(Kind::T_IMPLEMENTS) {
                if !tokens[index].is_kind(Kind::T_EXTENDS) {
                    continue;
                }
                let name_token_index = tokens.get_prev_meaningful_token(index).unwrap();
                let interface_token_index =
                    tokens.get_prev_meaningful_token(name_token_index).unwrap();
                if !tokens[interface_token_index].is_kind(Kind::T_INTERFACE) {
                    continue;
                }
            }
            let implements_start = index + 1;
            let implements_end = tokens
                .get_prev_meaningful_token(
                    tokens
                        .get_next_token_of_kind(implements_start, &["{".into()], true)
                        .unwrap(),
                )
                .unwrap();
            let interfaces_tokens = get_interfaces(tokens, implements_start, implements_end);
            if interfaces_tokens.len() == 1 {
                continue;
            }
            struct I {
                tokens: Vec<Token>,
                normalized: Vec<u8>,
                original_index: usize,
            }
            let mut interfaces: Vec<I> = interfaces_tokens
                .into_iter()
                .enumerate()
                .map(|(original_index, interface)| {
                    let mut normalized = Vec::new();
                    // every chunk has a name: an empty one (`implements A, B, {`) is a parse error
                    let start = interface
                        .iter()
                        .position(|t| {
                            !matches!(
                                t.id,
                                Some(Kind::T_WHITESPACE | Kind::T_COMMENT | Kind::T_DOC_COMMENT)
                            ) && !t.content.is_empty()
                        })
                        .unwrap();
                    for t in &interface[start..] {
                        if t.is_comment() || t.is_whitespace(None) {
                            break;
                        }
                        normalized.extend(str_replace(b"\\", b" ", &t.content));
                    }
                    I {
                        tokens: interface,
                        normalized,
                        original_index,
                    }
                })
                .collect();
            interfaces.sort_by(|a, b| {
                let score = if self.order_length {
                    a.normalized.len().cmp(&b.normalized.len())
                } else if self.case_sensitive {
                    a.normalized.cmp(&b.normalized)
                } else {
                    strcasecmp(&a.normalized, &b.normalized).cmp(&0)
                };
                if self.descend { score.reverse() } else { score }
            });
            if interfaces
                .iter()
                .enumerate()
                .all(|(i, x)| x.original_index == i)
            {
                continue;
            }
            let mut iter = interfaces.into_iter();
            let mut new_tokens = iter.next().unwrap().tokens;
            for i in iter {
                new_tokens.push(Token::char(","));
                new_tokens.extend(i.tokens);
            }
            tokens.override_range(implements_start, implements_end, new_tokens);
        }
    }
}

fn get_interfaces(tokens: &Tokens, start: usize, end: usize) -> Vec<Vec<Token>> {
    let mut interfaces = vec![vec![]];
    for i in start..=end {
        if tokens[i].equals(",", true) {
            interfaces.push(vec![]);
            continue;
        }
        interfaces.last_mut().unwrap().push(tokens[i].clone());
    }
    interfaces
}
