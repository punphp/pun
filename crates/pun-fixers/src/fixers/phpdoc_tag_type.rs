//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocTagTypeFixer`.
use crate::analyzers::preg::{Preg, PREG_SPLIT_DELIM_CAPTURE};
use anyhow::{bail, Result};
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

const TAG_REGEX: &str = "/^(?:
        (?<tag>
            (?:@(?<tag_name>.+?)(?:\\s.+)?)
        )
        |
        {(?<inlined_tag>
            (?:@(?<inlined_tag_name>.+?)(?:\\s.+)?)
        )}
    )$/x";

const DEFAULT_TAGS: &[&str] = &[
    "api", "author", "copyright", "deprecated", "example", "global", "inheritDoc", "internal", "license", "method",
    "package", "param", "property", "return", "see", "since", "throws", "todo", "uses", "var", "version",
];

struct PhpdocTagTypeFixer {
    /// lowercased tag => is_inline, in configuration order
    tags: Vec<(String, bool)>,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocTagTypeFixer { tags: DEFAULT_TAGS.iter().map(|t| (t.to_ascii_lowercase(), false)).collect() })
}

/// PHP `preg_quote($str, $delimiter)`.
fn preg_quote(s: &str, delimiter: u8) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c == '\0' {
            out.push_str("\\000");
            continue;
        }
        if ".\\+*?[^]$(){}=!<>|:-#".contains(c) || (c.is_ascii() && c as u8 == delimiter) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

impl Fixer for PhpdocTagTypeFixer {
    fn name(&self) -> &'static str {
        "phpdoc_tag_type"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = crate::analyzers::fixer_configuration_resolver::resolve(self.name(), options, &["tags"])?;
        let Some(v) = o.get("tags") else {
            *self = PhpdocTagTypeFixer { tags: DEFAULT_TAGS.iter().map(|t| (t.to_ascii_lowercase(), false)).collect() };
            return Ok(());
        };
        let map = match v {
            Value::Object(m) => m.clone(),
            Value::Array(a) if a.is_empty() => Default::default(),
            _ => bail!("[phpdoc_tag_type] Invalid configuration: The option \"tags\" with value {v} is expected to be of type \"array<string, 'annotation'|'inline'>\"."),
        };
        let mut tags: Vec<(String, bool)> = Vec::new();
        for (tag, ty) in &map {
            let inline = match ty.as_str() {
                Some("annotation") => false,
                Some("inline") => true,
                _ => bail!("[phpdoc_tag_type] Invalid configuration: Unknown tag type \"{}\".", ty.as_str().map_or(ty.to_string(), String::from)),
            };
            // normalizer: strtolower keys, later duplicates overwrite earlier ones in place
            let key = tag.to_ascii_lowercase();
            match tags.iter_mut().find(|(k, _)| *k == key) {
                Some(slot) => slot.1 = inline,
                None => tags.push((key, inline)),
            }
        }
        self.tags = tags;
        Ok(())
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        if self.tags.is_empty() {
            return;
        }
        let regular_expression = format!(
            "/({{?@(?:{}).*?(?:(?=\\s\\*\\/)|(?=\\n)}}?))/i",
            self.tags.iter().map(|(t, _)| preg_quote(t, b'/')).collect::<Vec<_>>().join("|")
        );
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            let mut parts: Vec<Vec<u8>> = Preg::split(&regular_expression, &tokens[index].content, -1, PREG_SPLIT_DELIM_CAPTURE)
                .into_iter()
                .map(|p| p.text)
                .collect();
            let max = parts.len() - 1;
            let mut i = 1;
            while i < max {
                let part = parts[i].clone();
                let Some(matches) = Preg::match_groups(TAG_REGEX, &part, 0) else {
                    i += 2;
                    continue;
                };
                let (tag, tag_name) = if !matches.name_str("tag").is_empty() {
                    (matches.name_str("tag").to_vec(), matches.name_str("tag_name"))
                } else {
                    (matches.name_str("inlined_tag").to_vec(), matches.name_str("inlined_tag_name"))
                };
                let tag_name = String::from_utf8_lossy(&tag_name.to_ascii_lowercase()).into_owned();
                let Some(&(_, inline)) = self.tags.iter().find(|(k, _)| *k == tag_name) else {
                    i += 2;
                    continue;
                };
                if inline {
                    parts[i] = [&b"{"[..], &tag, b"}"].concat();
                    i += 2;
                    continue;
                }
                if !tag_is_surrounded_by_text(&parts, i) {
                    parts[i] = tag;
                }
                i += 2;
            }
            tokens.set(index, Token::new(T_DOC_COMMENT, parts.concat()));
        }
    }
}

fn tag_is_surrounded_by_text(parts: &[Vec<u8>], index: usize) -> bool {
    Preg::r#match("/(^|\\R)\\h*[^@\\s]\\N*/", clean_comment(&parts[index - 1]))
        || Preg::r#match("/^.*?\\R\\s*[^@\\s]/", clean_comment(&parts[index + 1]))
}

fn clean_comment(comment: &[u8]) -> Vec<u8> {
    let comment = Preg::replace("/^\\/\\*\\*|\\*\\/$/", "", comment);
    Preg::replace("/(\\R)(\\h*\\*)?\\h*/", "$1", comment)
}
