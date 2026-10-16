//! Port of `PhpCsFixer\Fixer\Phpdoc\GeneralPhpdocTagRenameFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, php_compat::{strtolower, trim}};
use serde_json::Value;
use std::path::Path;

struct GeneralPhpdocTagRenameFixer {
    fix_annotation: bool,
    fix_inline: bool,
    /// Normalized `replacements`, in PHP array order.
    replacements: Vec<(Vec<u8>, Vec<u8>)>,
    case_sensitive: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(GeneralPhpdocTagRenameFixer { fix_annotation: true, fix_inline: true, replacements: Vec::new(), case_sensitive: false })
}

/// PHP turns canonical decimal-integer string keys into int keys.
fn is_int_key(s: &str) -> bool {
    s == "0" || (!s.starts_with('0') && s.trim_start_matches('-').bytes().all(|b| b.is_ascii_digit()) && !s.trim_start_matches('-').is_empty() && !s.starts_with("-0") && s.len() < 20)
}

impl Fixer for GeneralPhpdocTagRenameFixer {
    fn name(&self) -> &'static str {
        "general_phpdoc_tag_rename"
    }
    fn priority(&self) -> i32 {
        // Must run before PhpdocAddMissingParamAnnotationFixer, PhpdocAlignFixer.
        // Must run after AlignMultilineCommentFixer, CommentToPhpdocFixer, PhpdocIndentFixer, PhpdocScalarFixer,
        // PhpdocToCommentFixer, PhpdocTypesFixer.
        11
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["fix_annotation", "fix_inline", "replacements", "case_sensitive"])?;
        let fix_annotation = r.bool("fix_annotation", true)?;
        let fix_inline = r.bool("fix_inline", true)?;
        let case_sensitive = r.bool("case_sensitive", false)?;
        let mut normalized: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        let entries: Vec<(String, &Value)> = match r.get("replacements") {
            None => Vec::new(),
            Some(Value::Array(a)) => a.iter().enumerate().map(|(i, v)| (i.to_string(), v)).collect(),
            Some(Value::Object(o)) => o.iter().map(|(k, v)| (k.clone(), v)).collect(),
            Some(v) => bail!("[general_phpdoc_tag_rename] Invalid configuration: The option \"replacements\" with value {v} is expected to be of type \"array<string, string>\"."),
        };
        for (from, to) in entries {
            let Value::String(to) = to else {
                bail!("[general_phpdoc_tag_rename] Invalid configuration: The option \"replacements\" with value {to} is expected to be of type \"array<string, string>\".");
            };
            if is_int_key(&from) {
                bail!("[general_phpdoc_tag_rename] Invalid configuration: Tag to replace must be a string.");
            }
            if !Preg::r#match("#^\\S+$#", to) || to.contains("*/") {
                bail!("[general_phpdoc_tag_rename] Invalid configuration: Tag \"{from}\" cannot be replaced by invalid tag \"{to}\".");
            }
            let mut from = trim(from.as_bytes()).to_vec();
            let to = trim(to.as_bytes()).to_vec();
            if !case_sensitive {
                let lowercase_from = strtolower(&from);
                if normalized.iter().any(|(f, t)| *f == lowercase_from && *t != to) {
                    bail!("[general_phpdoc_tag_rename] Invalid configuration: Tag \"{}\" cannot be configured to be replaced with several different tags when case sensitivity is off.", String::from_utf8_lossy(&from));
                }
                from = lowercase_from;
            }
            match normalized.iter_mut().find(|(f, _)| *f == from) {
                Some(e) => e.1 = to,
                None => normalized.push((from, to)),
            }
        }
        for (from, to) in &normalized {
            if let Some((_, to2)) = normalized.iter().find(|(f, _)| f == to) {
                if to2 != to {
                    bail!(
                        "[general_phpdoc_tag_rename] Invalid configuration: Cannot change tag \"{}\" to tag \"{}\", as the tag \"{}\" is configured to be replaced to \"{}\".",
                        String::from_utf8_lossy(from), String::from_utf8_lossy(to), String::from_utf8_lossy(to), String::from_utf8_lossy(to2)
                    );
                }
            }
        }
        *self = GeneralPhpdocTagRenameFixer { fix_annotation, fix_inline, replacements: normalized, case_sensitive };
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        if self.replacements.is_empty() {
            return;
        }
        let regex: &[u8] = if self.fix_annotation {
            if self.fix_inline {
                b"/([\"'])[^\\1]*\\1(*SKIP)(*FAIL)|\\b(?<=@)(?P<tag>%s)\\b/"
            } else {
                b"/([\"'])[^\\1]*\\1(*SKIP)(*FAIL)|(?<!\\{@)(?<=@)(?P<tag>%s)(?!\\})/"
            }
        } else {
            b"/(?<={@)(?P<tag>%s)(?=[ \\t}])/"
        };
        let case_insensitive = !self.case_sensitive;
        let keys = self.replacements.iter().map(|(f, _)| f.clone()).collect::<Vec<_>>().join(&b'|');
        let at = regex.windows(2).position(|w| w == b"%s").unwrap();
        let mut regex = [&regex[..at], &keys, &regex[at + 2..]].concat();
        if case_insensitive {
            regex.push(b'i');
        }
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            let content = Preg::replace_callback(
                &regex,
                |m| {
                    let mut tag = m.name_str("tag").to_vec();
                    if case_insensitive {
                        tag = strtolower(&tag);
                    }
                    self.replacements.iter().find(|(f, _)| *f == tag).map(|(_, t)| t.clone()).unwrap()
                },
                tokens[index].get_content(),
            );
            tokens.set(index, Token::new(T_DOC_COMMENT, content));
        }
    }
}
