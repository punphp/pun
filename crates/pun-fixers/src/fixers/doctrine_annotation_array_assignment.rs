//! Port of `PhpCsFixer\Fixer\DoctrineAnnotation\DoctrineAnnotationArrayAssignmentFixer`.
use crate::analyzers::doctrine_annotation::{self as doctrine, *};
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Kind::T_DOC_COMMENT;
use pun_tokens::Tokens;
use serde_json::Value;
use std::path::Path;

struct DoctrineAnnotationArrayAssignmentFixer {
    ignored_tags: Vec<String>,
    operator: &'static [u8],
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(DoctrineAnnotationArrayAssignmentFixer {
        ignored_tags: vec![],
        operator: b"=",
    })
}

impl Fixer for DoctrineAnnotationArrayAssignmentFixer {
    fn name(&self) -> &'static str {
        "doctrine_annotation_array_assignment"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["ignored_tags", "operator"])?;
        self.ignored_tags = doctrine::ignored_tags(&o)?;
        self.operator = o.choice("operator", "=", &[("=", &b"="[..]), (":", &b":"[..])])?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        doctrine::apply_fix(tokens, &self.ignored_tags, |doc| {
            let mut scopes = vec![];
            for token in &mut doc.0 {
                if token.kind == T_OPEN_PARENTHESIS {
                    scopes.push(false);
                } else if token.kind == T_OPEN_CURLY_BRACES {
                    scopes.push(true);
                } else if token.is_type(&[T_CLOSE_PARENTHESIS, T_CLOSE_CURLY_BRACES]) {
                    scopes.pop();
                } else if scopes.last() == Some(&true) && token.is_type(&[T_EQUALS, T_COLON]) {
                    token.content = self.operator.to_vec();
                }
            }
        });
    }
}
