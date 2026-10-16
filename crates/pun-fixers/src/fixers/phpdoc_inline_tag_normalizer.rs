//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocInlineTagNormalizerFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::{preg_quote, Preg};
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::{php_compat, Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

const DEFAULT_TAGS: &[&str] = &["example", "id", "internal", "inheritdoc", "inheritdocs", "link", "source", "toc", "tutorial"];

struct PhpdocInlineTagNormalizerFixer {
    tags: Vec<String>,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocInlineTagNormalizerFixer { tags: DEFAULT_TAGS.iter().map(|s| s.to_string()).collect() })
}

impl Fixer for PhpdocInlineTagNormalizerFixer {
    fn name(&self) -> &'static str {
        "phpdoc_inline_tag_normalizer"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["tags"])?;
        self.tags = o.strings("tags", DEFAULT_TAGS, &[])?;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        if self.tags.is_empty() {
            return;
        }
        let pattern = format!(
            "#(?:@{{+|{{+\\h*@)\\h*({})\\b([^}}]*)(?:}}+)#i",
            self.tags.iter().map(|t| String::from_utf8_lossy(&preg_quote(t.as_bytes(), Some(b'/'))).into_owned()).collect::<Vec<_>>().join("|")
        );
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            // Move `@` inside tag, for example @{tag} -> {@tag}, replace multiple curly brackets,
            // remove spaces between '{' and '@', remove white space between end
            // of text and closing bracket and between the tag and inline comment.
            let content = Preg::replace_callback(
                &pattern,
                |m| {
                    let doc = php_compat::trim(m.str(2));
                    if doc.is_empty() {
                        return [&b"{@"[..], m.str(1), b"}"].concat();
                    }
                    [&b"{@"[..], m.str(1), b" ", doc, b"}"].concat()
                },
                &tokens[index].content,
            );
            tokens.set(index, Token::new(T_DOC_COMMENT, content));
        }
    }
}
