//! Port of `PhpCsFixer\Fixer\Phpdoc\GeneralPhpdocTagRenameFixer`, shared by the general_phpdoc_tag_rename fixer and
//! phpdoc_no_alias_tag (an AbstractProxyFixer over it), so each fixer builds on its own under `PUN_FIXERS_ONLY`.
use crate::analyzers::fixer_configuration_resolver::{PhpKey, resolve};
use crate::analyzers::preg::Preg;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::{
    Kind::*,
    Token, Tokens,
    php_compat::{strtolower, trim},
};
use serde_json::Value;
use std::path::Path;

#[derive(Default)]
pub struct GeneralPhpdocTagRenameFixer {
    fix_annotation: bool,
    fix_inline: bool,
    /// Normalized `replacements`, in PHP array order.
    replacements: Vec<(Vec<u8>, Vec<u8>)>,
    case_sensitive: bool,
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
        let r = resolve(
            self.name(),
            options,
            &[
                "fix_annotation",
                "fix_inline",
                "replacements",
                "case_sensitive",
            ],
        )?;
        let fix_annotation = r.bool("fix_annotation", true)?;
        let fix_inline = r.bool("fix_inline", true)?;
        let case_sensitive = r.bool("case_sensitive", false)?;
        let mut normalized: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        // `array<string, string>`, which the resolver checks as `string[]`.
        r.check_types("replacements", &["string[]"])?;
        for (from, to) in r.php_array("replacements").unwrap_or_default() {
            let PhpKey::Str(from) = from else {
                bail!(
                    "[general_phpdoc_tag_rename] Invalid configuration: Tag to replace must be a string."
                );
            };
            let to = to.as_str().unwrap();
            if !Preg::r#match("#^\\S+$#", to) || to.contains("*/") {
                bail!(
                    "[general_phpdoc_tag_rename] Invalid configuration: Tag \"{from}\" cannot be replaced by invalid tag \"{to}\"."
                );
            }
            let mut from = trim(from.as_bytes()).to_vec();
            let to = trim(to.as_bytes()).to_vec();
            if !case_sensitive {
                let lowercase_from = strtolower(&from);
                if normalized
                    .iter()
                    .any(|(f, t)| *f == lowercase_from && *t != to)
                {
                    bail!(
                        "[general_phpdoc_tag_rename] Invalid configuration: Tag \"{}\" cannot be configured to be replaced with several different tags when case sensitivity is off.",
                        String::from_utf8_lossy(&from)
                    );
                }
                from = lowercase_from;
            }
            match normalized.iter_mut().find(|(f, _)| *f == from) {
                Some(e) => e.1 = to,
                None => normalized.push((from, to)),
            }
        }
        for (from, to) in &normalized {
            if let Some((_, to2)) = normalized.iter().find(|(f, _)| f == to)
                && to2 != to
            {
                bail!(
                    "[general_phpdoc_tag_rename] Invalid configuration: Cannot change tag \"{}\" to tag \"{}\", as the tag \"{}\" is configured to be replaced to \"{}\".",
                    String::from_utf8_lossy(from),
                    String::from_utf8_lossy(to),
                    String::from_utf8_lossy(to),
                    String::from_utf8_lossy(to2)
                );
            }
        }
        *self = GeneralPhpdocTagRenameFixer {
            fix_annotation,
            fix_inline,
            replacements: normalized,
            case_sensitive,
        };
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
        let keys = self
            .replacements
            .iter()
            .map(|(f, _)| f.clone())
            .collect::<Vec<_>>()
            .join(&b'|');
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
                    self.replacements
                        .iter()
                        .find(|(f, _)| *f == tag)
                        .map(|(_, t)| t.clone())
                        .unwrap()
                },
                tokens[index].get_content(),
            );
            tokens.set(index, Token::new(T_DOC_COMMENT, content));
        }
    }
}
