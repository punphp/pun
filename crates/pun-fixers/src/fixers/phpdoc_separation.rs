//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocSeparationFixer`.
use crate::analyzers::docblock::{Annotation, DocBlock, replace_all};
use crate::analyzers::fixer_configuration_resolver::{php_array, resolve};
use crate::analyzers::preg::{Preg, preg_quote};
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::{Kind, Token, Tokens};
use serde_json::Value;
use std::path::Path;

const OPTION_GROUPS_DEFAULT: &[&[&str]] = &[
    &["author", "copyright", "license"],
    &["category", "package", "subpackage"],
    &["property", "property-read", "property-write"],
    &["deprecated", "link", "see", "since"],
];

struct PhpdocSeparationFixer {
    groups: Vec<Vec<String>>,
    skip_unlisted_annotations: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(PhpdocSeparationFixer {
        groups: vec![],
        skip_unlisted_annotations: false,
    })
}

impl Fixer for PhpdocSeparationFixer {
    fn name(&self) -> &'static str {
        "phpdoc_separation"
    }
    fn priority(&self) -> i32 {
        -3
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(
            self.name(),
            options,
            &["groups", "skip_unlisted_annotations"],
        )?;
        r.check_types("groups", &["string[][]"])?;
        let groups: Vec<Vec<String>> = match r.get("groups") {
            None => OPTION_GROUPS_DEFAULT
                .iter()
                .map(|g| g.iter().map(|s| s.to_string()).collect())
                .collect(),
            Some(v) => php_array(v)
                .into_iter()
                .map(|(_, g)| {
                    php_array(g)
                        .into_iter()
                        .map(|(_, s)| s.as_str().unwrap().to_string())
                        .collect()
                })
                .collect(),
        };
        let mut tags: Vec<(&str, usize)> = vec![];
        for (gi, group) in groups.iter().enumerate() {
            for member in group {
                if let Some(&(_, g)) = tags.iter().find(|(m, _)| *m == member) {
                    if g == gi {
                        bail!(
                            "[phpdoc_separation] Invalid configuration: The option \"groups\" value is invalid. The \"{member}\" tag is specified more than once."
                        );
                    }
                    bail!(
                        "[phpdoc_separation] Invalid configuration: The option \"groups\" value is invalid. The \"{member}\" tag belongs to more than one group."
                    );
                }
                tags.push((member, gi));
            }
        }
        self.skip_unlisted_annotations = r.bool("skip_unlisted_annotations", false)?;
        self.groups = groups;
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
            let doc = DocBlock::new(&tokens[index].content);
            fix_description(&doc);
            self.fix_annotations(&doc);
            tokens.set(index, Token::new(Kind::T_DOC_COMMENT, doc.get_content()));
        }
    }
}

fn fix_description(doc: &DocBlock) {
    for (index, line) in doc.get_lines().iter().enumerate() {
        if line.borrow().contains_a_tag() {
            break;
        }
        if line.borrow().contains_useful_content()
            && let Some(next) = doc.get_line(index + 1)
            && next.borrow().contains_a_tag()
        {
            line.borrow_mut().add_blank();
            break;
        }
    }
}

impl PhpdocSeparationFixer {
    fn fix_annotations(&self, doc: &DocBlock) {
        let annotations = doc.get_annotations();
        for (index, annotation) in annotations.iter().enumerate() {
            let Some(next) = doc.get_annotation(index + 1) else {
                break;
            };
            match self.should_be_together(annotation, next) {
                Some(true) => ensure_are_together(doc, annotation, next),
                s if s == Some(false) || !self.skip_unlisted_annotations => {
                    ensure_are_separate(doc, annotation, next)
                }
                _ => {}
            }
        }
    }

    fn should_be_together(&self, first: &Annotation, second: &Annotation) -> Option<bool> {
        let first_name = tag_name(first)?;
        let second_name = tag_name(second)?;
        if first_name == second_name {
            return Some(true);
        }
        for group in &self.groups {
            let a = is_in_group(&first_name, group);
            let b = is_in_group(&second_name, group);
            if a {
                return Some(b);
            }
            if b {
                return Some(false);
            }
        }
        None
    }
}

fn ensure_are_together(doc: &DocBlock, first: &Annotation, second: &Annotation) {
    for pos in first.get_end() + 1..second.get_start() {
        doc.get_line(pos).unwrap().borrow_mut().remove();
    }
}

fn ensure_are_separate(doc: &DocBlock, first: &Annotation, second: &Annotation) {
    let pos = first.get_end();
    let fin = second.get_start() - 1;
    if pos == fin {
        doc.get_line(pos).unwrap().borrow_mut().add_blank();
        return;
    }
    for p in pos + 1..fin {
        doc.get_line(p).unwrap().borrow_mut().remove();
    }
}

fn tag_name(a: &Annotation) -> Option<Vec<u8>> {
    let content = a.get_content();
    Preg::match_groups(r"/@([a-zA-Z0-9_\\\-]+(?=\s|$|\())/", &content, 0)
        .and_then(|m| m.get(1).map(<[u8]>::to_vec))
}

fn is_in_group(tag: &[u8], group: &[String]) -> bool {
    group.iter().any(|t| {
        let t = replace_all(t.as_bytes(), b"*", br"\*");
        let t = preg_quote(&t, Some(b'/'));
        let t = replace_all(&t, br"\\\*", b".*?");
        Preg::r#match([&b"/^"[..], &t, b"$/"].concat(), tag)
    })
}
