//! Port of `PhpCsFixer\Fixer\Comment\HeaderCommentFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::{Preg, RawPreg};
use anyhow::{Result, bail};
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::Kind::*;
use pun_tokens::php_compat::{rtrim, substr_count, trim};
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

#[derive(Clone, Copy, PartialEq)]
enum Location {
    AfterOpen,
    AfterDeclareStrict,
}

#[derive(Clone, Copy, PartialEq)]
enum Separate {
    Both,
    Top,
    Bottom,
    None,
}

struct HeaderCommentFixer {
    header: Vec<u8>,
    validator: Option<String>,
    phpdoc: bool,
    location: Location,
    separate: Separate,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    // `header` is required, so the fixer has no default configuration: `configure` sets every field.
    Box::new(HeaderCommentFixer {
        header: vec![],
        validator: None,
        phpdoc: false,
        location: Location::AfterDeclareStrict,
        separate: Separate::Both,
        whitespaces_config: WhitespacesFixerConfig::default(),
    })
}

impl Fixer for HeaderCommentFixer {
    fn name(&self) -> &'static str {
        "header_comment"
    }
    fn priority(&self) -> i32 {
        // When this fixer is configured with ["separate" => "bottom", "comment_type" => "PHPDoc"]
        // and the target file has no namespace or declare() construct,
        // the fixed header comment gets trimmed by NoBlankLinesAfterPhpdocFixer if we run before it.
        -30
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_monolithic_php() && !tokens.is_token_kind_found(T_OPEN_TAG_WITH_ECHO)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        if options.is_null() {
            bail!("[header_comment] Configuration is required.");
        }
        let o = resolve(
            self.name(),
            options,
            &[
                "header",
                "validator",
                "comment_type",
                "location",
                "separate",
            ],
        )?;
        let header = match o.get("header") {
            None => bail!(
                "[header_comment] Missing required configuration: The required option \"header\" is missing."
            ),
            Some(Value::String(s)) => s.as_bytes(),
            Some(v) => return Err(o.type_error("header", v, "\"string\"")),
        };
        // The `header` normalizer.
        let header = if trim(header).is_empty() {
            vec![]
        } else if header.windows(2).any(|w| w == b"*/") {
            bail!("[header_comment] Cannot use '*/' in header.");
        } else {
            header.to_vec()
        };
        let validator = match o.get("validator") {
            None | Some(Value::Null) => None,
            Some(Value::String(s)) => {
                // The `validator` normalizer: `Preg::match($value, '')` must not throw.
                if RawPreg::replace(s, "", "").is_err() {
                    bail!("[header_comment] Provided RegEx is not valid.");
                }
                Some(s.clone())
            }
            Some(v) => return Err(o.type_error("validator", v, "\"string\" or \"null\"")),
        };
        self.phpdoc = o.choice(
            "comment_type",
            "comment",
            &[("PHPDoc", true), ("comment", false)],
        )?;
        self.location = o.choice(
            "location",
            "after_declare_strict",
            &[
                ("after_open", Location::AfterOpen),
                ("after_declare_strict", Location::AfterDeclareStrict),
            ],
        )?;
        self.separate = o.choice(
            "separate",
            "both",
            &[
                ("both", Separate::Both),
                ("top", Separate::Top),
                ("bottom", Separate::Bottom),
                ("none", Separate::None),
            ],
        )?;
        self.header = header;
        self.validator = validator;
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut header_as_comment = self.get_header_as_comment();
        // `$locationIndices[$locationIndex] = $possibleLocation`, in insertion order.
        let mut location_indices: Vec<(usize, Location)> = vec![];
        for possible in [Location::AfterOpen, Location::AfterDeclareStrict] {
            let index = find_header_comment_insertion_index(tokens, possible);
            match location_indices.iter_mut().find(|(i, _)| *i == index) {
                None => location_indices.push((index, possible)),
                Some(e) if possible == self.location => e.1 = possible,
                Some(_) => {}
            }
        }

        // pre-run to find existing comment, if dynamic content is allowed
        if self.validator.is_some() {
            for &(_, possible) in &location_indices {
                let new_index = find_header_comment_insertion_index(tokens, possible);
                if let Some(current) = self.find_header_comment_current_index(
                    tokens,
                    &header_as_comment,
                    new_index - 1,
                ) && self.does_token_fulfill_validator(&tokens[current])
                {
                    header_as_comment = tokens[current].get_content().to_vec();
                }
            }
        }

        for &(_, possible) in &location_indices {
            let new_index = find_header_comment_insertion_index(tokens, possible);
            let Some(current) =
                self.find_header_comment_current_index(tokens, &header_as_comment, new_index - 1)
            else {
                if !self.header.is_empty() && possible == self.location {
                    self.insert_header(tokens, &header_as_comment, new_index);
                }
                continue;
            };
            let same_comment = header_as_comment == tokens[current].get_content();
            let expected_location = possible == self.location;
            if same_comment && expected_location {
                self.fix_white_space_around_header(tokens, current);
                continue;
            }
            if expected_location != same_comment {
                remove_header(tokens, current);
            }
            if !self.header.is_empty() && expected_location {
                self.insert_header(tokens, &header_as_comment, new_index);
            }
        }
    }
}

impl HeaderCommentFixer {
    fn does_token_fulfill_validator(&self, token: &Token) -> bool {
        let lines: Vec<&[u8]> = token.get_content().split(|&c| c == b'\n').collect();
        let inner = if lines.len() < 2 {
            &[][..]
        } else {
            &lines[1..lines.len() - 1]
        };
        let lines: Vec<&[u8]> = (inner.iter())
            .map(|&line| {
                if line == b" *" {
                    &b""[..]
                } else {
                    line.strip_prefix(b" * ").unwrap_or(line)
                }
            })
            .collect();
        Preg::r#match(self.validator.as_ref().unwrap(), lines.join(&b'\n'))
    }

    /// Enclose the given text in a comment block.
    fn get_header_as_comment(&self) -> Vec<u8> {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let mut comment = [if self.phpdoc { &b"/**"[..] } else { b"/*" }, line_ending].concat();
        let header: Vec<u8> = self
            .header
            .iter()
            .copied()
            .filter(|&c| c != b'\r')
            .collect();
        for line in header.split(|&c| c == b'\n') {
            comment.extend_from_slice(rtrim(&[b" * ", line].concat()));
            comment.extend_from_slice(line_ending);
        }
        comment.extend_from_slice(b" */");
        comment
    }

    fn find_header_comment_current_index(
        &self,
        tokens: &Tokens,
        header_as_comment: &[u8],
        index: usize,
    ) -> Option<usize> {
        let index = tokens.get_next_non_whitespace(index, None)?;
        if !tokens[index].is_comment() {
            return None;
        }
        let mut next = index + 1;
        if next >= tokens.count()
            || matches!(self.separate, Separate::Top | Separate::None)
            || !tokens[index].is_kind(T_DOC_COMMENT)
        {
            return Some(index);
        }
        if tokens[next].is_whitespace(None) {
            if !Preg::r#match(r"/^\h*\R\h*$/D", tokens[next].get_content()) {
                return Some(index);
            }
            next += 1;
        }
        if next >= tokens.count() || !tokens[next].is_classy() && !tokens[next].is_kind(T_FUNCTION)
        {
            return Some(index);
        }
        if header_as_comment == tokens[index].get_content()
            || (self.validator.is_some() && self.does_token_fulfill_validator(&tokens[index]))
        {
            return Some(index);
        }
        None
    }

    fn fix_white_space_around_header(&self, tokens: &mut Tokens, header_index: usize) {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();

        // fix lines after header comment
        let expected = if matches!(self.separate, Separate::Both | Separate::Bottom)
            && tokens.get_next_meaningful_token(header_index).is_some()
        {
            2
        } else {
            1
        };
        if header_index == tokens.count() - 1 {
            tokens.insert_at(
                header_index + 1,
                vec![Token::new(T_WHITESPACE, line_ending.repeat(expected))],
            );
        } else {
            let line_break_count = get_line_break_count(tokens, header_index, 1);
            if line_break_count < expected {
                let missing = line_ending.repeat(expected - line_break_count);
                if tokens[header_index + 1].is_whitespace(None) {
                    let content = [&missing[..], tokens[header_index + 1].get_content()].concat();
                    tokens.set(header_index + 1, Token::new(T_WHITESPACE, content));
                } else {
                    tokens.insert_at(header_index + 1, vec![Token::new(T_WHITESPACE, missing)]);
                }
            } else if line_break_count > expected && tokens[header_index + 1].is_whitespace(None) {
                let pattern = format!("/^\\R{{{}}}/", line_break_count - expected);
                let content = Preg::replace(pattern, "", tokens[header_index + 1].get_content());
                tokens.set(header_index + 1, Token::new(T_WHITESPACE, content));
            }
        }

        // fix lines before header comment
        let expected = if matches!(self.separate, Separate::Both | Separate::Top) {
            2
        } else {
            1
        };
        let prev = tokens.get_prev_non_whitespace(header_index, None).unwrap();
        if tokens[prev].is_kind(T_OPEN_TAG) && Preg::r#match(r"/\h$/", tokens[prev].get_content()) {
            let content = Preg::replace(r"/\h$/", line_ending, tokens[prev].get_content());
            tokens.set(prev, Token::new(T_OPEN_TAG, content));
        }
        let line_break_count = get_line_break_count(tokens, header_index, -1);
        if line_break_count < expected {
            // because of the way the insert index was determined for header comment there cannot be an empty token here
            tokens.insert_at(
                header_index,
                vec![Token::new(
                    T_WHITESPACE,
                    line_ending.repeat(expected - line_break_count),
                )],
            );
        }
    }

    fn insert_header(&self, tokens: &mut Tokens, header_as_comment: &[u8], index: usize) {
        let kind = if self.phpdoc {
            T_DOC_COMMENT
        } else {
            T_COMMENT
        };
        tokens.insert_at(index, vec![Token::new(kind, header_as_comment)]);
        self.fix_white_space_around_header(tokens, index);
    }
}

/// Find the index where the header comment must be inserted.
fn find_header_comment_insertion_index(tokens: &Tokens, location: Location) -> usize {
    let open_tag_index = usize::from(tokens[0].is_kind(T_INLINE_HTML));
    let fallback = open_tag_index + 1;
    if location == Location::AfterOpen {
        return fallback;
    }
    let Some(index) = tokens.get_next_meaningful_token(open_tag_index) else {
        return fallback; // file without meaningful tokens but an open tag, comment should always be placed directly after the open tag
    };
    if !tokens[index].is_kind(T_DECLARE) {
        return fallback;
    }
    let mut next = index;
    let steps: [&dyn Fn(&Token) -> bool; 6] = [
        &|t| t.equals("(", true),
        &|t| t.equals((T_STRING, "strict_types"), false),
        &|t| t.equals("=", true),
        &|t| t.is_kind(T_LNUMBER),
        &|t| t.equals(")", true),
        &|t| t.equals(";", true), // don't insert after close tag
    ];
    for step in steps {
        match tokens.get_next_meaningful_token(next) {
            Some(n) if step(&tokens[n]) => next = n,
            _ => return fallback,
        }
    }
    next + 1
}

fn get_line_break_count(tokens: &Tokens, index: usize, direction: isize) -> usize {
    let mut whitespace = vec![];
    let mut index = index as isize + direction;
    while index >= 0 && (index as usize) < tokens.count() {
        let token = &tokens[index as usize];
        if token.is_whitespace(None) {
            whitespace.extend_from_slice(token.get_content());
            index += direction;
            continue;
        }
        if direction == -1 && token.is_kind(T_OPEN_TAG) {
            whitespace.extend_from_slice(token.get_content());
        }
        // Upstream walks on over empty (cleared) tokens; none exist here: the runner clears them after every fixer and
        // this fixer clears a header only after its last whitespace fix.
        break;
    }
    substr_count(&whitespace, b"\n")
}

fn remove_header(tokens: &mut Tokens, index: usize) {
    let prev_index = index - 1;
    let mut newline_removed = false;
    if tokens[prev_index].is_whitespace(None) {
        let content = tokens[prev_index].get_content();
        if Preg::r#match(r"/\R/", content) {
            newline_removed = true;
        }
        let content = Preg::replace(r"/\R?\h*$/", "", content);
        tokens.ensure_whitespace_at_index(prev_index, 0, &content);
    }
    let next_index = index + 1;
    if !newline_removed && next_index < tokens.count() && tokens[next_index].is_whitespace(None) {
        let content = Preg::replace(r"/^\R/", "", tokens[next_index].get_content());
        tokens.ensure_whitespace_at_index(next_index, 0, &content);
    }
    tokens.clear_token_and_merge_surrounding_whitespace(index);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// HeaderCommentFixerTest::provideInvalidConfigurationCases: the messages (regexes) the real fixer's configure()
    /// throws, except the cases configuring an option with an object, which JSON can't express, and `[]`, which Pint's
    /// RuleSet refuses before configure() ("Configuration must be an array and may not be empty.").
    #[test]
    fn invalid_configuration() {
        let raw = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../pun-harness/fixtures/header_comment.json"
        ))
        .unwrap();
        let doc: Value = serde_json::from_slice(&raw).unwrap();
        let test = (doc["tests"].as_array().unwrap().iter())
            .find(|t| t["method"] == "testInvalidConfiguration")
            .unwrap();
        let mut checked = 0;
        for case in test["cases"].as_array().unwrap() {
            let (config, message) = (&case["args"][0], case["args"][1].as_str().unwrap());
            if config
                .as_object()
                .is_some_and(|o| o.values().any(Value::is_object))
                || config.as_array().is_some_and(Vec::is_empty)
            {
                continue;
            }
            let error = fixer().configure(config).unwrap_err().to_string();
            // The test prepends `[header_comment] ` to the expected message (a regex fragment).
            let pattern = format!("#^\\[header_comment\\] {message}$#");
            assert!(
                Preg::r#match(&pattern, &error),
                "{config}: {error} !~ {pattern}"
            );
            checked += 1;
        }
        assert_eq!(checked, 5);
    }
}
