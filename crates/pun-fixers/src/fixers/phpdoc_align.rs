//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocAlignFixer`.
use crate::analyzers::docblock::{DocBlock, type_expression::REGEX_TYPES};
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::{Result, bail};
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::php_compat::trim;
use pun_tokens::{Kind, Token, Tokens};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

const DEFAULT_TAGS: &[&str] = &[
    "method", "param", "property", "return", "throws", "type", "var",
];
const TAGS_WITH_NAME: &[&str] = &[
    "param",
    "property",
    "property-read",
    "property-write",
    "phpstan-param",
    "phpstan-property",
    "phpstan-property-read",
    "phpstan-property-write",
    "phpstan-assert",
    "phpstan-assert-if-true",
    "phpstan-assert-if-false",
    "psalm-param",
    "psalm-param-out",
    "psalm-property",
    "psalm-property-read",
    "psalm-property-write",
    "psalm-assert",
    "psalm-assert-if-true",
    "psalm-assert-if-false",
];
const TAGS_WITH_METHOD_SIGNATURE: &[&str] = &["method", "phpstan-method", "psalm-method"];
const DEFAULT_SPACING: i64 = 1;

enum Spacing {
    Int(i64),
    Map(BTreeMap<String, i64>),
}

struct PhpdocAlignFixer {
    regex: String,
    regex_comment_line: String,
    align_left: bool,
    spacing: Spacing,
    line_ending: Vec<u8>,
}

pub fn fixer() -> Box<dyn Fixer> {
    let mut f = PhpdocAlignFixer {
        regex: String::new(),
        regex_comment_line: String::new(),
        align_left: false,
        spacing: Spacing::Int(1),
        line_ending: b"\n".to_vec(),
    };
    f.configure(&Value::Null).unwrap();
    Box::new(f)
}

#[derive(Clone)]
struct Item {
    indent: Vec<u8>,
    tag: Option<Vec<u8>>,
    hint: Vec<u8>,
    var: Vec<u8>,
    static_: Vec<u8>,
    desc: Vec<u8>,
}

fn mb_strlen(s: &[u8]) -> usize {
    match std::str::from_utf8(s) {
        Ok(s) => s.chars().count(),
        Err(_) => s.iter().filter(|&&b| !(0x80..0xC0).contains(&b)).count(),
    }
}

impl Fixer for PhpdocAlignFixer {
    fn name(&self) -> &'static str {
        "phpdoc_align"
    }
    fn priority(&self) -> i32 {
        -42
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.line_ending = config.get_line_ending().as_bytes().to_vec();
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["tags", "align", "spacing"])?;
        let tags = r.strings("tags", DEFAULT_TAGS, &[])?;
        let align = r.string("align", "vertical", &["left", "vertical"])?;
        r.check_types("spacing", &["int", "int[]"])?;
        let spacing = match r.get("spacing") {
            None => Spacing::Int(DEFAULT_SPACING),
            Some(Value::Object(o)) => Spacing::Map(
                o.iter()
                    .map(|(k, v)| (k.clone(), v.as_i64().unwrap()))
                    .collect(),
            ),
            // a JSON list is a PHP array keyed 0..n
            Some(Value::Array(a)) => Spacing::Map(
                a.iter()
                    .enumerate()
                    .map(|(k, v)| (k.to_string(), v.as_i64().unwrap()))
                    .collect(),
            ),
            Some(v) => Spacing::Int(v.as_i64().unwrap()),
        };
        let vals: Vec<i64> = match &spacing {
            Spacing::Int(i) => vec![*i],
            Spacing::Map(m) => m.values().copied().collect(),
        };
        if vals.iter().any(|&v| v <= 0) {
            bail!(
                "[phpdoc_align] Invalid configuration: The option \"spacing\" is invalid. All spacings must be greater than zero."
            );
        }

        let with_name: Vec<&str> = tags
            .iter()
            .map(String::as_str)
            .filter(|t| TAGS_WITH_NAME.contains(t))
            .collect();
        let with_sig: Vec<&str> = tags
            .iter()
            .map(String::as_str)
            .filter(|t| TAGS_WITH_METHOD_SIGNATURE.contains(t))
            .collect();
        let without: Vec<&str> = tags
            .iter()
            .map(String::as_str)
            .filter(|t| !with_name.contains(t) && !with_sig.contains(t))
            .collect();
        let indent_regex = r"^(?P<indent>(?:\ {2}|\t)*)\ ?";
        let mut types = vec![];
        if !with_name.is_empty() {
            types.push(format!(
                r"(?P<tag>{})\s+(?P<hint>(?:{REGEX_TYPES})?)\s*(?P<var>(?:&|\.{{3}})?\$\S+)",
                with_name.join("|")
            ));
        }
        if !without.is_empty() {
            types.push(format!(
                r"(?P<tag2>{})\s+(?P<hint2>(?:{REGEX_TYPES})?)",
                without.join("|")
            ));
        }
        if !with_sig.is_empty() {
            types.push(format!(
                r"(?P<tag3>{})(\s+(?P<static>static))?(\s+(?P<hint3>(?:{REGEX_TYPES})?))\s+(?P<signature>.+\))",
                with_sig.join("|")
            ));
        }
        let desc = r"(?:\s+(?P<desc>\V*))";
        self.regex = format!(
            r"/{indent_regex}\*\h*@(?J)(?:{}){desc}\h*\r?$/",
            types.join("|")
        );
        self.regex_comment_line =
            format!(r"/{indent_regex}\*(?!\h?+@)(?:\s+(?P<desc>\V+))(?<!\*\/)\r?$/");
        self.align_left = align == "left";
        self.spacing = spacing;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            // Only a line with `@` can start an aligned group; without one the docblock is left as is.
            if !tokens[index].is_kind(Kind::T_DOC_COMMENT) || !tokens[index].content.contains(&b'@')
            {
                continue;
            }
            let content = tokens[index].content.clone();
            let doc = DocBlock::new(&content);
            self.fix_doc_block(&doc);
            let new = doc.get_content();
            if new != content {
                tokens.set(index, Token::new(Kind::T_DOC_COMMENT, new));
            }
        }
    }
}

impl PhpdocAlignFixer {
    fn fix_doc_block(&self, doc: &DocBlock) {
        let line_ending = &self.line_ending;
        let l = doc.get_lines().len();
        let mut i = 0;
        'outer: while i < l {
            let Some(m) = self.get_matches(doc.get_line(i).unwrap().borrow().get_content(), false)
            else {
                i += 1;
                continue;
            };
            let current = i;
            let mut items = vec![m];
            loop {
                i += 1;
                let Some(line) = doc.get_line(i) else {
                    break 'outer;
                };
                let Some(m) = self.get_matches(line.borrow().get_content(), true) else {
                    break;
                };
                items.push(m);
            }

            let (mut has_static, mut tag_max, mut hint_max, mut var_max) =
                (false, 0usize, 0usize, 0usize);
            for item in &items {
                let Some(tag) = &item.tag else { continue };
                has_static = has_static || !item.static_.is_empty();
                tag_max = tag_max.max(tag.len());
                hint_max = hint_max.max(item.hint.len());
                var_max = var_max.max(item.var.len());
            }
            let mut item_opening_line: Option<&Item> = None;
            let mut spacing_for_tag = self.spacing_for_tag(None);
            for (j, item) in items.iter().enumerate() {
                let line_ref = doc.get_line(current + j).unwrap();
                let Some(tag) = &item.tag else {
                    if item.desc.first() == Some(&b'@') {
                        let line = [&item.indent[..], b" * ", &item.desc, line_ending].concat();
                        line_ref.borrow_mut().set_content(line);
                        continue;
                    }
                    let opening = item_opening_line.unwrap();
                    let otag = std::str::from_utf8(opening.tag.as_deref().unwrap()).unwrap_or("");
                    let mut extra_indent = 2 * spacing_for_tag;
                    if TAGS_WITH_NAME.contains(&otag) || TAGS_WITH_METHOD_SIGNATURE.contains(&otag)
                    {
                        extra_indent += var_max as i64 + spacing_for_tag;
                    }
                    if has_static {
                        extra_indent += 7;
                    }
                    let line = [
                        &item.indent[..],
                        b" * ",
                        if !opening.hint.is_empty() { b" " } else { b"" },
                        &self.get_indent(
                            tag_max as i64 + hint_max as i64 + extra_indent,
                            self.get_left_aligned_description_indent(&items, j),
                        ),
                        &item.desc,
                        line_ending,
                    ]
                    .concat();
                    line_ref.borrow_mut().set_content(line);
                    continue;
                };
                let tag_s = String::from_utf8_lossy(tag).into_owned();
                spacing_for_tag = self.spacing_for_tag(Some(&tag_s));
                item_opening_line = Some(item);
                let mut line = [&item.indent[..], b" * @", tag].concat();
                let hint_vertical_align_indent;
                if has_static {
                    line.extend(self.get_indent(
                        tag_max as i64 - tag.len() as i64 + spacing_for_tag,
                        if !item.static_.is_empty() {
                            spacing_for_tag
                        } else {
                            0
                        },
                    ));
                    if !item.static_.is_empty() {
                        line.extend_from_slice(&item.static_);
                    } else {
                        line.extend(self.get_indent(6, 0));
                    }
                    hint_vertical_align_indent = spacing_for_tag;
                } else {
                    hint_vertical_align_indent =
                        tag_max as i64 - tag.len() as i64 + spacing_for_tag;
                }
                line.extend(self.get_indent(
                    hint_vertical_align_indent,
                    if !item.hint.is_empty() {
                        spacing_for_tag
                    } else {
                        0
                    },
                ));
                line.extend_from_slice(&item.hint);
                if !item.var.is_empty() {
                    let hm = if hint_max != 0 { hint_max as i64 } else { -1 };
                    line.extend(self.get_indent(
                        hm - mb_strlen(&item.hint) as i64 + spacing_for_tag,
                        spacing_for_tag,
                    ));
                    line.extend_from_slice(&item.var);
                    if !item.desc.is_empty() {
                        line.extend(self.get_indent(
                            var_max as i64 - item.var.len() as i64 + spacing_for_tag,
                            spacing_for_tag,
                        ));
                        line.extend_from_slice(&item.desc);
                    }
                } else if !item.desc.is_empty() {
                    line.extend(self.get_indent(
                        hint_max as i64 - item.hint.len() as i64 + spacing_for_tag,
                        spacing_for_tag,
                    ));
                    line.extend_from_slice(&item.desc);
                }
                line.extend_from_slice(line_ending);
                line_ref.borrow_mut().set_content(line);
            }
            i += 1;
        }
    }

    fn spacing_for_tag(&self, tag: Option<&str>) -> i64 {
        match &self.spacing {
            Spacing::Int(i) => *i,
            Spacing::Map(m) => m
                .get(tag.unwrap_or(""))
                .or_else(|| m.get("_default"))
                .copied()
                .unwrap_or(DEFAULT_SPACING),
        }
    }

    fn get_matches(&self, line: &[u8], match_comment_only: bool) -> Option<Item> {
        if let Some(m) = line
            .contains(&b'@')
            .then(|| Preg::match_groups(&self.regex, line, 0))
            .flatten()
        {
            let g = |n: &str| m.name_str(n).to_vec();
            let has = |n: &str| m.name_exists(n);
            let mut item = Item {
                indent: g("indent"),
                tag: Some(g("tag")),
                hint: g("hint"),
                var: g("var"),
                static_: g("static"),
                desc: g("desc"),
            };
            if has("tag2") && !m.name_str("tag2").is_empty() {
                item.tag = Some(g("tag2"));
                item.hint = g("hint2");
                item.var = vec![];
            }
            if has("tag3") && !m.name_str("tag3").is_empty() {
                item.tag = Some(g("tag3"));
                item.hint = g("hint3");
                item.var = g("signature");
                if item.hint.is_empty() && !item.static_.is_empty() {
                    item.hint = std::mem::take(&mut item.static_);
                }
            }
            item.hint = trim(&item.hint).to_vec();
            return Some(item);
        }
        if match_comment_only {
            if let Some(m) = Preg::match_groups(&self.regex_comment_line, line, 0) {
                return Some(Item {
                    indent: m.name_str("indent").to_vec(),
                    tag: None,
                    hint: vec![],
                    var: vec![],
                    static_: vec![],
                    desc: m.name_str("desc").to_vec(),
                });
            }
        }
        None
    }

    fn get_indent(&self, vertical_align_indent: i64, left_align_indent: i64) -> Vec<u8> {
        let n = if self.align_left {
            left_align_indent
        } else {
            vertical_align_indent
        };
        vec![
            b' ';
            usize::try_from(n)
                .expect("str_repeat(): Argument #2 ($times) must be greater than or equal to 0")
        ]
    }

    fn get_left_aligned_description_indent(&self, items: &[Item], mut index: usize) -> i64 {
        if !self.align_left {
            return 0;
        }
        while items[index].tag.is_none() && index > 0 {
            index -= 1;
        }
        let item = &items[index];
        let tag = item
            .tag
            .as_ref()
            .map(|t| String::from_utf8_lossy(t).into_owned());
        let s = self.spacing_for_tag(tag.as_deref());
        let si =
            |x: Option<&[u8]>| x.map_or(0, |x| if x.is_empty() { 0 } else { x.len() as i64 + s });
        si(Some(&item.static_))
            + si(item.tag.as_deref())
            + si(Some(&item.hint))
            + si(Some(&item.var))
    }
}

#[cfg(test)]
mod tests {
    /// Non-UTF-8 hint (JSON cases can't hold it): PHP's mb_strlen counts "F\u{e9}\xff" as 3 characters, not 4 bytes.
    /// Expected bytes are PHP-CS-Fixer's output for this input.
    #[test]
    fn invalid_utf8_width() {
        let input =
            b"<?php\n/**\n * @param F\xc3\xa9\xff $a desc\n * @param string $bb other\n */\n";
        let expected =
            b"<?php\n/**\n * @param F\xc3\xa9\xff    $a  desc\n * @param string $bb other\n */\n";
        let mut tokens = pun_tokens::Tokens::from_code(input);
        super::fixer().fix(std::path::Path::new("t.php"), &mut tokens);
        assert_eq!(
            String::from_utf8_lossy(&tokens.generate_code()),
            String::from_utf8_lossy(expected)
        );
        assert_eq!(tokens.generate_code(), expected);
    }
}
