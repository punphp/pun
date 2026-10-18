//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocOrderFixer`.
use crate::analyzers::docblock::DocBlock;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::utils::natural_language_join;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::{Kind, Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct PhpdocOrderFixer {
    configuration_order: Vec<String>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocOrderFixer {
        configuration_order: vec![],
    })
}

impl Fixer for PhpdocOrderFixer {
    fn name(&self) -> &'static str {
        "phpdoc_order"
    }
    fn priority(&self) -> i32 {
        -2
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["order"])?;
        let order = r.strings("order", &["param", "throws", "return"], &[])?;
        if order.len() < 2 {
            bail!(
                "[phpdoc_order] Invalid configuration: The option \"order\" value is invalid. Minimum two tags are required."
            );
        }
        let mut dups: Vec<&String> = vec![];
        for (i, t) in order.iter().enumerate() {
            if order[..i].contains(t) && !dups.contains(&t) {
                dups.push(t);
            }
        }
        if !dups.is_empty() {
            let joined = natural_language_join(&dups);
            let many = dups.len() > 1;
            bail!(
                "[phpdoc_order] Invalid configuration: The option \"order\" value is invalid. Tag{} {joined} {} duplicated.",
                if many { "s" } else { "" },
                if many { "are" } else { "is" }
            );
        }
        self.configuration_order.clear();
        for t in &order {
            self.configuration_order.push(t.clone());
            for p in ["phpstan-", "psalm-"] {
                let n = format!("{p}{t}");
                if !order.contains(&n) {
                    self.configuration_order.push(n);
                }
            }
        }
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(Kind::T_DOC_COMMENT) {
                continue;
            }
            let mut content = tokens[index].content.clone();
            let mut successors: Vec<&str> = self
                .configuration_order
                .iter()
                .map(String::as_str)
                .collect();
            while successors.len() >= 3 {
                let predecessor = successors.remove(0);
                content = move_annotations_before(predecessor, &successors, &content);
            }
            let mut predecessors: Vec<&str> = self
                .configuration_order
                .iter()
                .map(String::as_str)
                .collect();
            let last = predecessors.pop().unwrap();
            content = move_annotations_after(last, &predecessors, &content);
            tokens.set(index, Token::new(Kind::T_DOC_COMMENT, content));
        }
    }
}

fn move_annotations_before(mv: &str, before: &[&str], content: &[u8]) -> Vec<u8> {
    let doc = DocBlock::new(content);
    let to_be_moved = doc.get_annotations_of_type(&[mv]);
    if to_be_moved.is_empty() {
        return content.to_vec();
    }
    let others = doc.get_annotations_of_type(before);
    if others.is_empty() {
        return content.to_vec();
    }
    let end = to_be_moved.last().unwrap().get_end();
    let line = doc.get_line(end).unwrap().clone();
    for other in others {
        if other.get_start() < end {
            let new = [line.borrow().get_content(), &other.get_content()[..]].concat();
            line.borrow_mut().set_content(new);
            other.remove();
        }
    }
    doc.get_content()
}

fn move_annotations_after(mv: &str, after: &[&str], content: &[u8]) -> Vec<u8> {
    let doc = DocBlock::new(content);
    let to_be_moved = doc.get_annotations_of_type(&[mv]);
    if to_be_moved.is_empty() {
        return content.to_vec();
    }
    let others = doc.get_annotations_of_type(after);
    if others.is_empty() {
        return content.to_vec();
    }
    let start = to_be_moved[0].get_start();
    let line = doc.get_line(start).unwrap().clone();
    for other in others.iter().rev() {
        if other.get_end() > start {
            let new = [&other.get_content()[..], line.borrow().get_content()].concat();
            line.borrow_mut().set_content(new);
            other.remove();
        }
    }
    doc.get_content()
}
