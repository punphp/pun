//! Port of `PhpCsFixer\Fixer\ClassNotation\OrderedTraitsFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::php_compat::{ltrim_chars, strcasecmp};
use pun_tokens::{Kind, Tokens};
use serde_json::Value;

use std::path::Path;

struct OrderedTraitsFixer {
    case_sensitive: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(OrderedTraitsFixer {
        case_sensitive: false,
    })
}

impl Fixer for OrderedTraitsFixer {
    fn name(&self) -> &'static str {
        "ordered_traits"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["case_sensitive"])?;
        self.case_sensitive = r.bool("case_sensitive", false)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::CT_USE_TRAIT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // upstream's generator yields a group only after it is complete and sorting keeps the token
        // count, so collecting the groups first is equivalent
        for uses in find_use_statements_groups(tokens) {
            self.sort_use_statements(tokens, uses);
        }
    }
}

fn find_use_statements_groups(tokens: &Tokens) -> Vec<Vec<(usize, Tokens)>> {
    let mut groups = vec![];
    let mut uses: Vec<(usize, Tokens)> = vec![];
    let max = tokens.count();
    let mut index = 1;
    while index < max {
        let token = &tokens[index];
        if token.is_whitespace(None) || token.is_comment() {
            index += 1;
            continue;
        }
        if !token.is_kind(Kind::CT_USE_TRAIT) {
            if !uses.is_empty() {
                groups.push(std::mem::take(&mut uses));
            }
            index += 1;
            continue;
        }
        let start_index = tokens
            .get_next_non_whitespace(tokens.get_prev_meaningful_token(index).unwrap(), None)
            .unwrap();
        let mut end_index = tokens
            .get_next_token_of_kind(index, &[";".into(), "{".into()], true)
            .unwrap();
        if tokens[end_index].equals("{", true) {
            end_index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, end_index);
        }
        uses.push((
            start_index,
            Tokens::from_array(tokens.as_slice()[start_index..=end_index].to_vec()),
        ));
        index = end_index + 1;
    }
    groups
}

impl OrderedTraitsFixer {
    fn sort_use_statements(&self, tokens: &mut Tokens, mut uses: Vec<(usize, Tokens)>) {
        for (_, u) in &mut uses {
            self.sort_multiple_traits_in_statement(u);
        }
        self.sort(tokens, uses);
    }

    fn sort_multiple_traits_in_statement(&self, u: &mut Tokens) {
        let mut traits: Vec<(usize, Tokens)> = vec![];
        let mut index_of_name: Option<usize> = None;
        let mut name = vec![];
        let max = u.count();
        let mut index = 0;
        while index < max {
            let token = &u[index];
            if token.is_given_kind(&[Kind::T_STRING, Kind::T_NS_SEPARATOR]) {
                name.push(token.clone());
                index_of_name.get_or_insert(index);
                index += 1;
                continue;
            }
            if token.equals_any(&[",".into(), ";".into(), "{".into()], true) {
                traits.push((
                    index_of_name.unwrap(),
                    Tokens::from_array(std::mem::take(&mut name)),
                ));
                index_of_name = None;
            }
            if u[index].equals("{", true) {
                index = u.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
            }
            index += 1;
        }
        self.sort(u, traits);
    }

    fn sort(&self, tokens: &mut Tokens, elements: Vec<(usize, Tokens)>) {
        let to_trait_name = |u: &Tokens| {
            let mut s = vec![];
            for t in u.iter() {
                if t.equals_any(&[";".into(), "{".into()], true) {
                    break;
                }
                if t.is_given_kind(&[Kind::T_NS_SEPARATOR, Kind::T_STRING]) {
                    s.extend_from_slice(&t.content);
                }
            }
            ltrim_chars(&s, b"\\").to_vec()
        };
        let mut order: Vec<usize> = (0..elements.len()).collect();
        order.sort_by(|&a, &b| {
            let (x, y) = (to_trait_name(&elements[a].1), to_trait_name(&elements[b].1));
            if self.case_sensitive {
                x.cmp(&y)
            } else {
                strcasecmp(&x, &y).cmp(&0)
            }
        });
        let before_override_count = tokens.count();
        for pos in (0..elements.len()).rev() {
            let index = elements[pos].0;
            let insert = elements[order[pos]].1.as_slice().to_vec();
            tokens.override_range(index, index + elements[pos].1.count() - 1, insert);
        }
        if before_override_count < tokens.count() {
            tokens.clear_empty_tokens();
        }
    }
}
