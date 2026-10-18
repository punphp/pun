//! Port of `PhpCsFixer\Fixer\DoctrineAnnotation\DoctrineAnnotationBracesFixer`.
use crate::analyzers::doctrine_annotation::{self as doctrine, *};
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::T_DOC_COMMENT;
use pun_tokens::Tokens;
use serde_json::Value;
use std::path::Path;

struct DoctrineAnnotationBracesFixer {
    ignored_tags: Vec<String>,
    with_braces: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(DoctrineAnnotationBracesFixer {
        ignored_tags: vec![],
        with_braces: false,
    })
}

impl Fixer for DoctrineAnnotationBracesFixer {
    fn name(&self) -> &'static str {
        "doctrine_annotation_braces"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["ignored_tags", "syntax"])?;
        self.ignored_tags = doctrine::ignored_tags(&o)?;
        self.with_braces = o.choice(
            "syntax",
            "without_braces",
            &[("with_braces", true), ("without_braces", false)],
        )?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        doctrine::apply_fix(tokens, &self.ignored_tags, |doc| {
            if self.with_braces {
                add_braces_to_annotations(doc);
            } else {
                removes_braces_from_annotations(doc);
            }
        });
    }
}

fn add_braces_to_annotations(tokens: &mut DocTokens) {
    // `foreach` over the live list: the inserted braces are visited too.
    let mut index = 0;
    while index < tokens.len() {
        if tokens[index].kind == T_AT {
            let brace = tokens.get_next_meaningful_token(index + 1);
            if !brace.is_some_and(|b| tokens[b].kind == T_OPEN_PARENTHESIS) {
                tokens.insert_at(index + 2, DocToken::new(T_OPEN_PARENTHESIS, "("));
                tokens.insert_at(index + 3, DocToken::new(T_CLOSE_PARENTHESIS, ")"));
            }
        }
        index += 1;
    }
}

fn removes_braces_from_annotations(tokens: &mut DocTokens) {
    for index in 0..tokens.len() {
        if tokens[index].kind != T_AT {
            continue;
        }
        let Some(open) = tokens.get_next_meaningful_token(index + 1) else {
            continue;
        };
        if tokens[open].kind != T_OPEN_PARENTHESIS {
            continue;
        }
        // An annotation's parentheses are balanced, so a `(` is always followed by a meaningful token.
        let close = tokens.get_next_meaningful_token(open).unwrap();
        if tokens[close].kind != T_CLOSE_PARENTHESIS {
            continue;
        }
        for current in index + 2..=close {
            tokens[current].clear();
        }
    }
}
