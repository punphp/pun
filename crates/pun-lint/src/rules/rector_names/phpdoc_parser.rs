//! phpdoc-parser 2.3 as Rector's `BetterPhpDocParser` runs it, where the name handling needs it token by token: the
//! lexer with its token types, `TokenIterator` (and `BetterTokenIterator`), the children `parseWithNode()` reads with
//! their `START_AND_END`, the Doctrine tag values (`parseDoctrineTagValue()` and the description after one), and
//! Rector's `StaticDoctrineAnnotationParser` with its `PlainValueParser` and `ArrayParser`, which read an annotation's
//! arguments into the `ArrayItemNode`s Rector prints.
//!
//! A tag in `parseTagValue()`'s list (`@param`, `@method`...) is left to the caller ([`Value::Known`]): only where it
//! ends is read here.

use std::sync::OnceLock;

/// `Lexer::TOKEN_*`.
pub(crate) mod t {
    pub const REFERENCE: u8 = 0;
    pub const UNION: u8 = 1;
    pub const INTERSECTION: u8 = 2;
    pub const NULLABLE: u8 = 3;
    pub const OPEN_PARENTHESES: u8 = 4;
    pub const CLOSE_PARENTHESES: u8 = 5;
    pub const OPEN_ANGLE_BRACKET: u8 = 6;
    pub const CLOSE_ANGLE_BRACKET: u8 = 7;
    pub const OPEN_SQUARE_BRACKET: u8 = 8;
    pub const CLOSE_SQUARE_BRACKET: u8 = 9;
    pub const COMMA: u8 = 10;
    pub const VARIADIC: u8 = 11;
    pub const DOUBLE_COLON: u8 = 12;
    pub const DOUBLE_ARROW: u8 = 13;
    pub const EQUAL: u8 = 14;
    pub const OPEN_PHPDOC: u8 = 15;
    pub const CLOSE_PHPDOC: u8 = 16;
    pub const PHPDOC_TAG: u8 = 17;
    pub const DOCTRINE_TAG: u8 = 18;
    pub const FLOAT: u8 = 19;
    pub const INTEGER: u8 = 20;
    pub const SINGLE_QUOTED_STRING: u8 = 21;
    pub const DOUBLE_QUOTED_STRING: u8 = 22;
    pub const DOCTRINE_ANNOTATION_STRING: u8 = 23;
    pub const IDENTIFIER: u8 = 24;
    pub const THIS_VARIABLE: u8 = 25;
    pub const VARIABLE: u8 = 26;
    pub const HORIZONTAL_WS: u8 = 27;
    pub const PHPDOC_EOL: u8 = 28;
    pub const OTHER: u8 = 29;
    pub const END: u8 = 30;
    pub const COLON: u8 = 31;
    pub const WILDCARD: u8 = 32;
    pub const OPEN_CURLY_BRACKET: u8 = 33;
    pub const CLOSE_CURLY_BRACKET: u8 = 34;
    pub const NEGATED: u8 = 35;
    pub const ARROW: u8 = 36;
    pub const COMMENT: u8 = 37;
}

/// `Lexer::generateRegexp()`'s alternatives in order (PCRE alternation: the first that matches wins).
const PATTERNS: &[(u8, &str)] = &[
    (t::HORIZONTAL_WS, r"[\x09\x20]++"),
    (
        t::IDENTIFIER,
        r"(?:[\\]?+[a-z_\x80-\xFF][0-9a-z_\x80-\xFF-]*+)++",
    ),
    (t::THIS_VARIABLE, r"\$this(?![0-9a-z_\x80-\xFF])"),
    (t::VARIABLE, r"\$[a-z_\x80-\xFF][0-9a-z_\x80-\xFF]*+"),
    (
        t::REFERENCE,
        r"&(?=\s*+(?:[.,=)]|(?:\$(?!this(?![0-9a-z_\x80-\xFF])))))",
    ),
    (t::UNION, r"\|"),
    (t::INTERSECTION, "&"),
    (t::NULLABLE, r"\?"),
    (t::NEGATED, "!"),
    (t::OPEN_PARENTHESES, r"\("),
    (t::CLOSE_PARENTHESES, r"\)"),
    (t::OPEN_ANGLE_BRACKET, "<"),
    (t::CLOSE_ANGLE_BRACKET, ">"),
    (t::OPEN_SQUARE_BRACKET, r"\["),
    (t::CLOSE_SQUARE_BRACKET, r"\]"),
    (t::OPEN_CURLY_BRACKET, r"\{"),
    (t::CLOSE_CURLY_BRACKET, r"\}"),
    (t::COMMA, ","),
    (t::COMMENT, r"\/\/[^\r\n]*(?=\n|\r|\*/)"),
    (t::VARIADIC, r"\.\.\."),
    (t::DOUBLE_COLON, "::"),
    (t::DOUBLE_ARROW, "=>"),
    (t::ARROW, "->"),
    (t::EQUAL, "="),
    (t::COLON, ":"),
    (t::OPEN_PHPDOC, r"/\*\*(?=\s)\x20?+"),
    (t::CLOSE_PHPDOC, r"\*/"),
    (t::PHPDOC_TAG, r"@(?:[a-z][a-z0-9-\\]+:)?[a-z][a-z0-9-\\]*+"),
    (t::DOCTRINE_TAG, r"@[a-z_\\][a-z0-9_\:\\]*[a-z_][a-z0-9_]*"),
    (t::PHPDOC_EOL, r"\r?+\n[\x09\x20]*+(?:\*(?!/)\x20?+)?"),
    (
        t::FLOAT,
        r"[+\-]?(?:(?:[0-9]++(?:_[0-9]++)*\.[0-9]*+(?:_[0-9]++)*(?:e[+\-]?[0-9]++(?:_[0-9]++)*)?)|(?:[0-9]*+(?:_[0-9]++)*\.[0-9]++(?:_[0-9]++)*(?:e[+\-]?[0-9]++(?:_[0-9]++)*)?)|(?:[0-9]++(?:_[0-9]++)*e[+\-]?[0-9]++(?:_[0-9]++)*))",
    ),
    (
        t::INTEGER,
        r"[+\-]?(?:(?:0b[0-1]++(?:_[0-1]++)*)|(?:0o[0-7]++(?:_[0-7]++)*)|(?:0x[0-9a-f]++(?:_[0-9a-f]++)*)|(?:[0-9]++(?:_[0-9]++)*))",
    ),
    (t::SINGLE_QUOTED_STRING, r"'(?:\\[^\r\n]|[^'\r\n\\])*+'"),
    (t::DOUBLE_QUOTED_STRING, r#""(?:\\[^\r\n]|[^"\r\n\\])*+""#),
    (t::DOCTRINE_ANNOTATION_STRING, r#""(?:""|[^"])*+""#),
    (t::WILDCARD, r"\*"),
    (t::OTHER, r"(?:(?!\*/)[^\s])++"),
];

/// A token: its type and its bytes `[start, end)` of the lexed text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Tok {
    pub ty: u8,
    pub start: usize,
    pub end: usize,
}

/// `Lexer::tokenize()`: the tokens, then `TOKEN_END`. Like `preg_match_all` with the `A` modifier, lexing stops at the
/// first byte no alternative matches (a lone `\r`, a vertical tab).
pub(crate) fn tokenize(input: &str) -> Vec<Tok> {
    static RE: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        let alternatives: Vec<String> = PATTERNS.iter().map(|(_, p)| format!("({p})")).collect();
        pcre2::bytes::RegexBuilder::new()
            .caseless(true)
            .dotall(true)
            .build(&format!(r"\G(?:{})", alternatives.join("|")))
            .expect("phpdoc lexer regex")
    });
    let mut locations = re.capture_locations();
    let mut tokens = Vec::new();
    let mut offset = 0;
    while let Some(found) = re
        .captures_read_at(&mut locations, input.as_bytes(), offset)
        .ok()
        .flatten()
    {
        let group = (1..=PATTERNS.len())
            .find(|g| locations.get(*g).is_some())
            .unwrap_or_default();
        tokens.push(Tok {
            ty: PATTERNS[group - 1].0,
            start: found.start(),
            end: found.end(),
        });
        offset = found.end();
    }
    tokens.push(Tok {
        ty: t::END,
        start: offset,
        end: offset,
    });
    tokens
}

/// A parse error (`ParserException`).
#[derive(Debug)]
pub(crate) struct Fail;

type Parsed<T> = Result<T, Fail>;

/// `TokenIterator` with `BetterTokenIterator`'s additions.
pub(crate) struct Iter<'a> {
    text: &'a str,
    toks: &'a [Tok],
    pub index: usize,
    saves: Vec<usize>,
    skip_eol: bool,
    newline: Option<&'static str>,
}

impl<'a> Iter<'a> {
    pub fn new(text: &'a str, toks: &'a [Tok]) -> Iter<'a> {
        let mut iter = Iter {
            text,
            toks,
            index: 0,
            saves: Vec::new(),
            skip_eol: false,
            newline: None,
        };
        iter.skip_irrelevant();
        iter
    }

    fn token(&self, i: usize) -> Option<&Tok> {
        self.toks.get(i)
    }

    /// `currentTokenType()`; past the end (where PHP reads `null`), `TOKEN_END`'s.
    pub fn ty(&self) -> u8 {
        self.token(self.index).map_or(t::END, |t| t.ty)
    }

    /// `currentTokenValue()`; past the end, `""`.
    pub fn value(&self) -> &'a str {
        self.token(self.index)
            .map_or("", |t| &self.text[t.start..t.end])
    }

    pub fn is(&self, types: &[u8]) -> bool {
        types.contains(&self.ty())
    }

    fn skipped(&self, ty: u8) -> bool {
        ty == t::HORIZONTAL_WS || (self.skip_eol && ty == t::PHPDOC_EOL)
    }

    fn skip_irrelevant(&mut self) {
        while self.index + 1 < self.toks.len() && self.skipped(self.toks[self.index].ty) {
            self.index += 1;
        }
    }

    pub fn next(&mut self) {
        self.index += 1;
        self.skip_irrelevant();
    }

    fn detect_newline(&mut self) {
        if self.newline.is_none() {
            let value = self.value();
            self.newline = match value.starts_with("\r\n") {
                true => Some("\r\n"),
                false => value.starts_with('\n').then_some("\n"),
            };
        }
    }

    pub fn consume(&mut self, ty: u8) -> Parsed<()> {
        match self.try_consume(ty) {
            true => Ok(()),
            false => Err(Fail),
        }
    }

    pub fn try_consume(&mut self, ty: u8) -> bool {
        if self.ty() != ty {
            return false;
        }
        if ty == t::PHPDOC_EOL {
            self.detect_newline();
        }
        self.next();
        true
    }

    fn try_consume_value(&mut self, value: &str) -> bool {
        let found = self.value() == value;
        if found {
            self.next();
        }
        found
    }

    pub fn preceded_by_ws(&self) -> bool {
        self.index > 0 && self.toks[self.index - 1].ty == t::HORIZONTAL_WS
    }

    /// `getSkippedHorizontalWhiteSpaceIfAny()`.
    fn skipped_ws(&self) -> &'a str {
        match self.preceded_by_ws() {
            true => {
                let ws = &self.toks[self.index - 1];
                &self.text[ws.start..ws.end]
            }
            false => "",
        }
    }

    /// `joinUntil()`.
    fn join_until(&mut self, types: &[u8]) -> String {
        let mut joined = String::new();
        while !types.contains(&self.ty()) {
            joined.push_str(self.value());
            self.index += 1;
        }
        joined
    }

    pub fn push(&mut self) {
        self.saves.push(self.index);
    }

    pub fn drop_save(&mut self) {
        self.saves.pop();
    }

    pub fn rollback(&mut self) {
        self.index = self.saves.pop().unwrap_or_default();
    }

    /// `BetterTokenIterator::isNextTokenType()`.
    pub fn is_next(&mut self, types: &[u8]) -> bool {
        if self.index + 1 >= self.toks.len() {
            return false;
        }
        let at = self.index;
        self.next();
        let next = self.ty();
        self.index = at;
        types.contains(&next)
    }

    /// `getDetectedNewline() ?? "\n"`.
    fn newline(&self) -> &'static str {
        self.newline.unwrap_or("\n")
    }
}

/// A child of the doc as `BetterPhpDocParser::parseWithNode()` reads it, with its `START_AND_END`.
#[derive(Clone, Debug)]
pub(crate) struct RawChild {
    pub start: usize,
    pub end: usize,
    pub kind: Kind,
}

#[derive(Clone, Debug)]
pub(crate) enum Kind {
    /// `PhpDocTextNode`.
    Text(String),
    /// `PhpDocTagNode`: the tag's name as `resolveTag()` joins it, and its value.
    Tag { name: String, value: Value },
}

#[derive(Clone, Debug)]
pub(crate) enum Value {
    /// A tag in `parseTagValue()`'s list, which the caller reads.
    Known,
    /// `GenericTagValueNode`: its value, runs of line breaks made one (`MULTI_NEW_LINES_REGEX`).
    Generic(String),
    /// `DoctrineTagValueNode`: the arguments as phpdoc-parser prints them, and the description.
    Doctrine {
        args: Vec<Printed>,
        description: String,
    },
    /// `InvalidTagValueNode`: the description read after it.
    Invalid(String),
}

/// The tags `PhpDocParser::parseTagValue()` gives a value node of their own.
const KNOWN: &[&str] = &[
    "@param",
    "@phpstan-param",
    "@psalm-param",
    "@phan-param",
    "@param-immediately-invoked-callable",
    "@phpstan-param-immediately-invoked-callable",
    "@param-later-invoked-callable",
    "@phpstan-param-later-invoked-callable",
    "@param-closure-this",
    "@phpstan-param-closure-this",
    "@pure-unless-callable-is-impure",
    "@phpstan-pure-unless-callable-is-impure",
    "@pure-unless-parameter-passed",
    "@phpstan-pure-unless-parameter-passed",
    "@var",
    "@phpstan-var",
    "@psalm-var",
    "@phan-var",
    "@return",
    "@phpstan-return",
    "@psalm-return",
    "@phan-return",
    "@phan-real-return",
    "@throws",
    "@phpstan-throws",
    "@mixin",
    "@phan-mixin",
    "@psalm-require-extends",
    "@phpstan-require-extends",
    "@psalm-require-implements",
    "@phpstan-require-implements",
    "@psalm-inheritors",
    "@phpstan-sealed",
    "@deprecated",
    "@property",
    "@property-read",
    "@property-write",
    "@phpstan-property",
    "@phpstan-property-read",
    "@phpstan-property-write",
    "@psalm-property",
    "@psalm-property-read",
    "@psalm-property-write",
    "@phan-property",
    "@phan-property-read",
    "@phan-property-write",
    "@method",
    "@phpstan-method",
    "@psalm-method",
    "@phan-method",
    "@template",
    "@phpstan-template",
    "@psalm-template",
    "@phan-template",
    "@template-covariant",
    "@phpstan-template-covariant",
    "@psalm-template-covariant",
    "@template-contravariant",
    "@phpstan-template-contravariant",
    "@psalm-template-contravariant",
    "@extends",
    "@phpstan-extends",
    "@phan-extends",
    "@phan-inherits",
    "@template-extends",
    "@implements",
    "@phpstan-implements",
    "@template-implements",
    "@use",
    "@phpstan-use",
    "@template-use",
    "@phpstan-type",
    "@psalm-type",
    "@phan-type",
    "@phpstan-import-type",
    "@psalm-import-type",
    "@phpstan-assert",
    "@phpstan-assert-if-true",
    "@phpstan-assert-if-false",
    "@psalm-assert",
    "@psalm-assert-if-true",
    "@psalm-assert-if-false",
    "@phan-assert",
    "@phan-assert-if-true",
    "@phan-assert-if-false",
    "@phpstan-this-out",
    "@phpstan-self-out",
    "@psalm-this-out",
    "@psalm-self-out",
    "@param-out",
    "@phpstan-param-out",
    "@psalm-param-out",
];

/// The doc parser: the doc's text and tokens, and `type_end`, which reads a type from a byte of the text (to the end
/// of its line) and gives where it ends, or where phpdoc-parser's `TypeParser` throws.
pub(crate) struct DocParser<'a> {
    pub iter: Iter<'a>,
    type_end: &'a dyn Fn(usize) -> Result<usize, usize>,
}

impl<'a> DocParser<'a> {
    pub fn new(
        text: &'a str,
        toks: &'a [Tok],
        type_end: &'a dyn Fn(usize) -> Result<usize, usize>,
    ) -> DocParser<'a> {
        DocParser {
            iter: Iter::new(text, toks),
            type_end,
        }
    }

    /// `BetterPhpDocParser::parseWithNode()` before the decorators, and the position it stops at.
    pub fn parse_with_node(&mut self) -> Vec<RawChild> {
        let it = &mut self.iter;
        it.try_consume(t::OPEN_PHPDOC);
        it.try_consume(t::PHPDOC_EOL);
        let mut children = Vec::new();
        if self.iter.is(&[t::CLOSE_PHPDOC]) {
            return children;
        }
        children.push(self.parse_child_and_store_its_positions());
        while self.iter.try_consume(t::PHPDOC_EOL) && !self.iter.is(&[t::CLOSE_PHPDOC]) {
            children.push(self.parse_child_and_store_its_positions());
        }
        children
    }

    /// `parseChildAndStoreItsPositions()`: a child that throws is an empty text.
    fn parse_child_and_store_its_positions(&mut self) -> RawChild {
        let start = self.iter.index;
        let kind = self
            .parse_child()
            .unwrap_or_else(|_| Kind::Text(String::new()));
        RawChild {
            start,
            end: self.iter.index,
            kind,
        }
    }

    /// `PhpDocParser::parseChild()`.
    fn parse_child(&mut self) -> Parsed<Kind> {
        if self.iter.is(&[t::PHPDOC_TAG]) {
            return Ok(self.parse_tag());
        }
        if self.iter.is(&[t::DOCTRINE_TAG]) {
            let name = self.iter.value().to_string();
            self.iter.next();
            let (args, description) = self.parse_doctrine_tag_value()?;
            return Ok(Kind::Tag {
                name,
                value: Value::Doctrine { args, description },
            });
        }
        Ok(Kind::Text(self.parse_text()))
    }

    /// `BetterPhpDocParser::parseTag()`: `resolveTag()` joins an identifier written right after the tag's name.
    fn parse_tag(&mut self) -> Kind {
        let mut name = self.iter.value().to_string();
        self.iter.next();
        if !self.iter.preceded_by_ws() && self.iter.is(&[t::IDENTIFIER]) {
            name.push_str(self.iter.value());
            self.iter.next();
        }
        let value = self.parse_tag_value(&name);
        Kind::Tag { name, value }
    }

    /// `parseTagValue()`: where a value of a tag in its list ends, and the value of any other tag.
    fn parse_tag_value(&mut self, tag: &str) -> Value {
        self.iter.push();
        let parsed = match tag {
            "@phpstan-type" | "@psalm-type" | "@phan-type" => {
                self.parse_type_alias().map(|()| Value::Known)
            }
            "@phpstan-import-type" | "@psalm-import-type" => {
                self.parse_type_alias_import().map(|()| Value::Known)
            }
            _ if KNOWN.contains(&tag) => {
                self.parse_text();
                Ok(Value::Known)
            }
            _ if self.iter.is(&[t::OPEN_PARENTHESES]) => self
                .parse_doctrine_tag_value()
                .map(|(args, description)| Value::Doctrine { args, description }),
            _ => Ok(Value::Generic(collapse_newlines(
                &self.parse_optional_description_after_doctrine_tag(),
            ))),
        };
        match parsed {
            Ok(value) => {
                self.iter.drop_save();
                value
            }
            Err(Fail) => {
                self.iter.rollback();
                match KNOWN.contains(&tag) {
                    true => {
                        self.parse_text();
                        Value::Known
                    }
                    false => Value::Invalid(self.parse_text()),
                }
            }
        }
    }

    /// `parseTypeAliasTagValue()`: a type the line ends after, else the description read where the type stops.
    fn parse_type_alias(&mut self) -> Parsed<()> {
        self.iter.consume(t::IDENTIFIER)?;
        self.iter.try_consume(t::EQUAL);
        let start = self.iter.toks[self.iter.index].start;
        let end = (self.type_end)(start);
        while self.iter.ty() != t::END
            && self.iter.toks[self.iter.index].start < end.unwrap_or_else(|at| at)
        {
            self.iter.next();
        }
        if end.is_ok() && self.iter.is(&[t::CLOSE_PHPDOC, t::PHPDOC_EOL]) {
            return Ok(());
        }
        // the `ParserException`, at the token it throws at: `parseOptionalDescription()`
        self.parse_text();
        Ok(())
    }

    /// `parseTypeAliasImportTagValue()`.
    fn parse_type_alias_import(&mut self) -> Parsed<()> {
        self.iter.consume(t::IDENTIFIER)?;
        if !(self.iter.ty() == t::IDENTIFIER && self.iter.try_consume_value("from")) {
            return Err(Fail);
        }
        self.iter.consume(t::IDENTIFIER)?;
        if self.iter.try_consume_value("as") {
            self.iter.consume(t::IDENTIFIER)?;
        }
        Ok(())
    }

    /// `parseText()`.
    pub fn parse_text(&mut self) -> String {
        let it = &mut self.iter;
        let ends = [t::CLOSE_PHPDOC, t::END];
        let mut text = String::new();
        let mut savepoint = false;
        loop {
            let piece = format!(
                "{}{}",
                it.skipped_ws(),
                it.join_until(&[t::PHPDOC_EOL, t::CLOSE_PHPDOC, t::END])
            );
            text.push_str(&piece);
            // (at a line break or the `*/` a doc ends with: phpdoc-parser's stop at the end never comes)
            if !savepoint {
                it.push();
                savepoint = true;
            } else if !piece.is_empty() {
                it.drop_save();
                it.push();
            }
            it.push();
            it.next();
            if it.is(&[t::PHPDOC_TAG, t::DOCTRINE_TAG]) || it.is(&ends) {
                it.rollback();
                break;
            }
            it.drop_save();
            text.push_str(it.newline());
        }
        it.rollback();
        text = text
            .trim_end_matches(|c| it.newline().contains(c))
            .to_string();
        text.trim_matches([' ', '\t']).to_string()
    }

    /// `parseOptionalDescriptionAfterDoctrineTag()`.
    fn parse_optional_description_after_doctrine_tag(&mut self) -> String {
        let ends = [t::CLOSE_PHPDOC, t::END];
        let mut text = String::new();
        let mut savepoint = false;
        loop {
            let piece = format!(
                "{}{}",
                self.iter.skipped_ws(),
                self.iter.join_until(&[
                    t::PHPDOC_TAG,
                    t::DOCTRINE_TAG,
                    t::PHPDOC_EOL,
                    t::CLOSE_PHPDOC,
                    t::END
                ])
            );
            text.push_str(&piece);
            if !self.iter.is(&[t::PHPDOC_EOL, t::CLOSE_PHPDOC]) {
                if !self.iter.preceded_by_ws() {
                    let rest = self.parse_text();
                    return format!("{text}{rest}")
                        .trim_matches([' ', '\t'])
                        .to_string();
                }
                if self.iter.is(&[t::PHPDOC_TAG]) {
                    self.iter.push();
                    let child = self.parse_tag();
                    self.iter.rollback();
                    let stops = match child {
                        Kind::Tag {
                            value: Value::Generic(_) | Value::Doctrine { .. },
                            ..
                        } => true,
                        Kind::Tag {
                            value: Value::Invalid(_),
                            ..
                        } => {
                            self.iter.push();
                            self.iter.next();
                            let open = self.iter.is(&[t::OPEN_PARENTHESES]);
                            self.iter.rollback();
                            open
                        }
                        _ => false,
                    };
                    if !stops {
                        let rest = self.parse_text();
                        return format!("{text}{rest}")
                            .trim_matches([' ', '\t'])
                            .to_string();
                    }
                }
                break;
            }
            if !savepoint {
                self.iter.push();
                savepoint = true;
            } else if !piece.is_empty() {
                self.iter.drop_save();
                self.iter.push();
            }
            self.iter.push();
            self.iter.next();
            if self.iter.is(&[t::PHPDOC_TAG, t::DOCTRINE_TAG]) || self.iter.is(&ends) {
                self.iter.rollback();
                break;
            }
            self.iter.drop_save();
            text.push_str(self.iter.newline());
        }
        if savepoint {
            self.iter.rollback();
            let newline = self.iter.newline();
            text = text.trim_end_matches(|c| newline.contains(c)).to_string();
        }
        text.trim_matches([' ', '\t']).to_string()
    }

    /// `parseDoctrineTagValue()`: the arguments as phpdoc-parser prints them, and the description.
    fn parse_doctrine_tag_value(&mut self) -> Parsed<(Vec<Printed>, String)> {
        let args = parse_doctrine_arguments(&mut self.iter, false)?;
        Ok((args, self.parse_optional_description_after_doctrine_tag()))
    }
}

/// `StringUnescaper::unescapeString()` of the string token `token`, its quotes included.
pub(crate) fn unescape_string(token: &str) -> Vec<u8> {
    let inner = &token[1..token.len() - 1];
    if token.starts_with('\'') {
        return inner.replace("\\\\", "\\").replace("\\'", "'").into_bytes();
    }
    // `parseEscapeSequences()`
    static RE: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        pcre2::bytes::Regex::new(
            r"\\([\\nrtfve]|[xX][0-9a-fA-F]{1,2}|[0-7]{1,3}|u\{([0-9a-fA-F]+)\})",
        )
        .expect("escapes")
    });
    let value = inner.replace("\\\"", "\"").into_bytes();
    let mut out = Vec::new();
    let mut last = 0;
    for caps in re.captures_iter(&value).flatten() {
        let whole = caps.get(0).expect("match");
        let group = |g: usize| {
            caps.get(g)
                .map(|m| &value[m.start()..m.end()])
                .expect("group")
        };
        let escape = group(1);
        out.extend_from_slice(&value[last..whole.start()]);
        let number = |digits: &[u8], radix| {
            u32::from_str_radix(std::str::from_utf8(digits).expect("ascii"), radix)
                .unwrap_or(u32::MAX)
        };
        match escape[0] {
            b'\\' => out.push(b'\\'),
            b'n' => out.push(b'\n'),
            b'r' => out.push(b'\r'),
            b't' => out.push(b'\t'),
            b'f' => out.push(0x0c),
            b'v' => out.push(0x0b),
            b'e' => out.push(0x1b),
            b'x' | b'X' => out.push(number(&escape[1..], 16) as u8),
            // `codePointToUtf8()`
            b'u' => {
                let n = number(group(2), 16);
                match n {
                    0..=0x7f => out.push(n as u8),
                    0x80..=0x7ff => out.extend([(n >> 6) as u8 + 0xc0, (n & 0x3f) as u8 + 0x80]),
                    0x800..=0xffff => out.extend([
                        (n >> 12) as u8 + 0xe0,
                        (n >> 6 & 0x3f) as u8 + 0x80,
                        (n & 0x3f) as u8 + 0x80,
                    ]),
                    0x10000..=0x1fffff => out.extend([
                        (n >> 18) as u8 + 0xf0,
                        (n >> 12 & 0x3f) as u8 + 0x80,
                        (n >> 6 & 0x3f) as u8 + 0x80,
                        (n & 0x3f) as u8 + 0x80,
                    ]),
                    _ => out.extend("\u{fffd}".as_bytes()),
                }
            }
            // `chr(octdec())`, modulo 256
            _ => out.push(number(escape, 8) as u8),
        }
        last = whole.end();
    }
    out.extend_from_slice(&value[last..]);
    out
}

/// `ConstExprStringNode::__toString()` of `value`, the string `written`: single-quoted with `addcslashes($value,
/// '\'\\')`, else `escapeDoubleQuotedString()`. Where that leaves bytes that are no UTF-8 (a `\u{...}` escape of
/// a surrogate or beyond U+10FFFF, which the regex lets through), the string stays as written.
pub(crate) fn print_string(value: &[u8], written: &str) -> String {
    if written.starts_with('\'') {
        let value = String::from_utf8_lossy(value);
        return format!("'{}'", value.replace('\\', "\\\\").replace('\'', "\\'"));
    }
    // `addcslashes($value, "\n\r\t\f\v\$\"\\")`
    let mut escaped = Vec::new();
    for &b in value {
        match b {
            b'\n' => escaped.extend(b"\\n"),
            b'\r' => escaped.extend(b"\\r"),
            b'\t' => escaped.extend(b"\\t"),
            0x0c => escaped.extend(b"\\f"),
            0x0b => escaped.extend(b"\\v"),
            b'$' | b'"' | b'\\' => escaped.extend([b'\\', b]),
            _ => escaped.push(b),
        }
    }
    // control characters and bytes that are no UTF-8, each as `\xhh`
    static RE: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        pcre2::bytes::RegexBuilder::new()
            .extended(true)
            .build(
                r"(
                  [\x00-\x08\x0E-\x1F]
                | [\xC0-\xC1]
                | [\xF5-\xFF]
                | \xE0(?=[\x80-\x9F])
                | \xF0(?=[\x80-\x8F])
                | [\xC2-\xDF](?![\x80-\xBF])
                | [\xE0-\xEF](?![\x80-\xBF]{2})
                | [\xF0-\xF4](?![\x80-\xBF]{3})
                | (?<=[\x00-\x7F\xF5-\xFF])[\x80-\xBF]
                | (?<![\xC2-\xDF]|[\xE0-\xEF]|[\xE0-\xEF][\x80-\xBF]|[\xF0-\xF4]|[\xF0-\xF4][\x80-\xBF]|[\xF0-\xF4][\x80-\xBF]{2})[\x80-\xBF]
                | (?<=[\xE0-\xEF])[\x80-\xBF](?![\x80-\xBF])
                | (?<=[\xF0-\xF4])[\x80-\xBF](?![\x80-\xBF]{2})
                | (?<=[\xF0-\xF4][\x80-\xBF])[\x80-\xBF](?![\x80-\xBF])
            )",
            )
            .expect("utf-8 regex")
    });
    let mut out = Vec::new();
    let mut last = 0;
    for m in re.find_iter(&escaped).flatten() {
        out.extend_from_slice(&escaped[last..m.start()]);
        out.extend(format!("\\x{:02x}", escaped[m.start()]).bytes());
        last = m.end();
    }
    out.extend_from_slice(&escaped[last..]);
    String::from_utf8(out).map_or_else(|_| written.to_string(), |out| format!("\"{out}\""))
}

/// `MULTI_NEW_LINES_REGEX`: a run of two or more line breaks made its last one.
fn collapse_newlines(value: &str) -> String {
    static RE: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re =
        RE.get_or_init(|| pcre2::bytes::Regex::new(r"(\r\n|\n){2,}").expect("new lines regex"));
    replace_all(re, value, |caps| caps[caps.len() - 1].to_string())
}

/// `preg_replace_callback()` with `re`, `f` given the match and its groups.
pub(crate) fn replace_all(
    re: &pcre2::bytes::Regex,
    value: &str,
    f: impl Fn(&[&str]) -> String,
) -> String {
    let mut out = String::new();
    let mut last = 0;
    for caps in re.captures_iter(value.as_bytes()).flatten() {
        let whole = caps.get(0).expect("match");
        let groups: Vec<&str> = (0..caps.len())
            .filter_map(|g| caps.get(g).map(|m| &value[m.start()..m.end()]))
            .collect();
        out.push_str(&value[last..whole.start()]);
        out.push_str(&f(&groups));
        last = whole.end();
    }
    out.push_str(&value[last..]);
    out
}

/// A Doctrine argument, array item or value as phpdoc-parser prints it: its pieces, each with whether it is an
/// `IdentifierTypeNode` (a key, or a value that is a bare name).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Printed(pub Vec<(String, bool)>);

impl Printed {
    fn text(text: impl Into<String>) -> Printed {
        Printed(vec![(text.into(), false)])
    }

    fn ident(name: &str) -> Printed {
        Printed(vec![(name.to_string(), true)])
    }

    /// `self`, then `separator` and `other`.
    fn then(mut self, separator: &str, other: Printed) -> Printed {
        self.0.push((separator.to_string(), false));
        self.0.extend(other.0);
        self
    }

    /// `implode(', ', $items)` between `open` and `close`.
    fn wrap(open: &str, items: Vec<Printed>, close: &str) -> Printed {
        let mut printed = Printed::text(open);
        for (i, item) in items.into_iter().enumerate() {
            printed = printed.then(if i == 0 { "" } else { ", " }, item);
        }
        printed.then(close, Printed::default())
    }
}

impl std::fmt::Display for Printed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|(piece, _)| f.write_str(piece))
    }
}

/// `parseDoctrineArguments()`.
fn parse_doctrine_arguments(it: &mut Iter<'_>, deep: bool) -> Parsed<Vec<Printed>> {
    if !it.is(&[t::OPEN_PARENTHESES]) {
        return Ok(Vec::new());
    }
    let skip_eol = it.skip_eol;
    if !deep {
        it.skip_eol = true;
    }
    let mut arguments = Vec::new();
    let mut inner = || -> Parsed<()> {
        it.consume(t::OPEN_PARENTHESES)?;
        loop {
            if it.is(&[t::CLOSE_PARENTHESES]) {
                break;
            }
            arguments.push(parse_doctrine_argument(it)?);
            if !it.try_consume(t::COMMA) {
                break;
            }
        }
        Ok(())
    };
    let result = inner();
    if !deep {
        it.skip_eol = skip_eol;
    }
    result?;
    it.consume(t::CLOSE_PARENTHESES)?;
    Ok(arguments)
}

/// `parseDoctrineArgument()`.
fn parse_doctrine_argument(it: &mut Iter<'_>) -> Parsed<Printed> {
    if !it.is(&[t::IDENTIFIER]) {
        return parse_doctrine_argument_value(it);
    }
    it.push();
    let key = it.value().to_string();
    it.next();
    let keyed = it
        .consume(t::EQUAL)
        .and_then(|()| parse_doctrine_argument_value(it));
    match keyed {
        Ok(value) => {
            it.drop_save();
            Ok(Printed::ident(&key).then("=", value))
        }
        Err(Fail) => {
            it.rollback();
            parse_doctrine_argument_value(it)
        }
    }
}

/// `parseDoctrineArgumentValue()`, printed.
fn parse_doctrine_argument_value(it: &mut Iter<'_>) -> Parsed<Printed> {
    if it.is(&[t::PHPDOC_TAG, t::DOCTRINE_TAG]) {
        let name = it.value().to_string();
        it.next();
        let args = parse_doctrine_arguments(it, true)?;
        return Ok(Printed::text(name).then("", Printed::wrap("(", args, ")")));
    }
    if it.try_consume(t::OPEN_CURLY_BRACKET) {
        let mut items = Vec::new();
        loop {
            if it.is(&[t::CLOSE_CURLY_BRACKET]) {
                break;
            }
            items.push(parse_doctrine_array_item(it)?);
            if !it.try_consume(t::COMMA) {
                break;
            }
        }
        it.consume(t::CLOSE_CURLY_BRACKET)?;
        return Ok(Printed::wrap("{", items, "}"));
    }
    if it.is(&[t::IDENTIFIER]) {
        let identifier = it.value().to_string();
        it.push();
        it.next();
        if !it.is(&[t::DOUBLE_COLON]) {
            it.drop_save();
            return Ok(Printed::ident(&identifier));
        }
        it.rollback();
    }
    match parse_const_expr(it)? {
        Const::Array => Err(Fail),
        Const::Printed(printed) | Const::Fetch(printed) => Ok(Printed::text(printed)),
    }
}

/// `parseDoctrineArrayItem()`.
fn parse_doctrine_array_item(it: &mut Iter<'_>) -> Parsed<Printed> {
    it.push();
    let keyed = parse_doctrine_array_key(it).and_then(|key| {
        if !it.try_consume(t::EQUAL) {
            it.consume(t::COLON)?;
        }
        Ok(key.then("=", parse_doctrine_argument_value(it)?))
    });
    match keyed {
        Ok(item) => {
            it.drop_save();
            Ok(item)
        }
        Err(Fail) => {
            it.rollback();
            parse_doctrine_argument_value(it)
        }
    }
}

/// `parseDoctrineArrayKey()`, printed.
fn parse_doctrine_array_key(it: &mut Iter<'_>) -> Parsed<Printed> {
    let value = it.value().to_string();
    match it.ty() {
        t::INTEGER => {
            it.next();
            Ok(Printed::text(value.replace('_', "")))
        }
        t::DOCTRINE_ANNOTATION_STRING | t::DOUBLE_QUOTED_STRING => {
            it.next();
            Ok(Printed::text(parse_doctrine_string(it, value)))
        }
        t::SINGLE_QUOTED_STRING => {
            it.next();
            // `ConstExprStringNode` single-quoted: `StringUnescaper::unescapeString()`, printed with `addcslashes()`
            let unescaped = value[1..value.len() - 1]
                .replace("\\\\", "\\")
                .replace("\\'", "'");
            Ok(Printed::text(format!(
                "'{}'",
                unescaped.replace('\\', "\\\\").replace('\'', "\\'")
            )))
        }
        t::IDENTIFIER => {
            it.push();
            it.next();
            if !it.is(&[t::DOUBLE_COLON]) {
                it.drop_save();
                return Ok(Printed::ident(&value));
            }
            it.rollback();
            match parse_const_expr(it)? {
                Const::Fetch(printed) => Ok(Printed::text(printed)),
                _ => Err(Fail),
            }
        }
        _ => Err(Fail),
    }
}

/// A `ConstExprNode`, printed: a `ConstFetchNode` with a class, an array, or any other.
enum Const {
    Printed(String),
    Fetch(String),
    Array,
}

/// `ConstExprParser::parse()` with Doctrine strings, printed (a `ConstFetchNode` is the only node an identifier
/// followed by `::` gives here).
fn parse_const_expr(it: &mut Iter<'_>) -> Parsed<Const> {
    let value = it.value().to_string();
    match it.ty() {
        t::FLOAT | t::INTEGER => {
            it.next();
            Ok(Const::Printed(value.replace('_', "")))
        }
        t::DOCTRINE_ANNOTATION_STRING | t::DOUBLE_QUOTED_STRING => {
            it.next();
            Ok(Const::Printed(parse_doctrine_string(it, value)))
        }
        t::IDENTIFIER => {
            it.next();
            match value.to_ascii_lowercase().as_str() {
                "true" | "false" | "null" => return Ok(Const::Printed(value.to_ascii_lowercase())),
                "array" => {
                    it.consume(t::OPEN_PARENTHESES)?;
                    return parse_const_array(it, t::CLOSE_PARENTHESES);
                }
                _ => {}
            }
            if !it.try_consume(t::DOUBLE_COLON) {
                return Ok(Const::Printed(value));
            }
            let mut constant = String::new();
            let mut last = None;
            loop {
                if last != Some(t::IDENTIFIER) && it.ty() == t::IDENTIFIER {
                    constant.push_str(it.value());
                    it.next();
                    last = Some(t::IDENTIFIER);
                    continue;
                }
                if last != Some(t::WILDCARD) && it.try_consume(t::WILDCARD) {
                    constant.push('*');
                    last = Some(t::WILDCARD);
                    if !it.skipped_ws().is_empty() {
                        break;
                    }
                    continue;
                }
                if last.is_none() {
                    return Err(Fail);
                }
                break;
            }
            Ok(Const::Fetch(format!("{value}::{constant}")))
        }
        t::OPEN_SQUARE_BRACKET => {
            it.next();
            parse_const_array(it, t::CLOSE_SQUARE_BRACKET)
        }
        _ => Err(Fail),
    }
}

/// `ConstExprParser::parseArray()`: its items `expr` or `expr => expr`.
fn parse_const_array(it: &mut Iter<'_>, end: u8) -> Parsed<Const> {
    if !it.try_consume(end) {
        loop {
            parse_const_expr(it)?;
            if it.try_consume(t::DOUBLE_ARROW) {
                parse_const_expr(it)?;
            }
            if !it.try_consume(t::COMMA) || it.is(&[end]) {
                break;
            }
        }
        it.consume(end)?;
    }
    Ok(Const::Array)
}

/// `parseDoctrineString()`: the strings that follow joined, as a `DoctrineConstExprStringNode` prints.
fn parse_doctrine_string(it: &mut Iter<'_>, mut text: String) -> String {
    while it.is(&[t::DOUBLE_QUOTED_STRING, t::DOCTRINE_ANNOTATION_STRING]) {
        text.push_str(it.value());
        it.next();
    }
    let unescaped = text[1..text.len() - 1].replace("\"\"", "\"");
    format!("\"{}\"", unescaped.replace('"', "\"\""))
}

/// A value of `ArrayItemNode` (or its key) as Rector's annotation parsers make it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Val {
    /// A PHP string, printed as it is.
    Str(String),
    /// `StringNode`: printed in `"..."`, a `"` doubled.
    StringNode(String),
    /// `null`, printed empty.
    Null,
    /// `ConstExprTrueNode`, `ConstExprFalseNode`, `ConstExprIntegerNode`: printed.
    Const(String),
    /// `CurlyListNode`.
    Curly(Vec<ArrayItem>),
    /// A PHP array of `ArrayItemNode`s, printed one after the other.
    List(Vec<ArrayItem>),
    /// A nested `DoctrineAnnotationTagValueNode`.
    Annotation(Box<Nested>),
}

/// A key of `ArrayItemNode`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Key {
    /// An integer key, never printed.
    Int(i64),
    Val(Val),
}

/// `ArrayItemNode`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ArrayItem {
    pub key: Option<Key>,
    pub value: Val,
}

/// A nested `DoctrineAnnotationTagValueNode`: its identifier, the class it resolves to, its values.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Nested {
    pub identifier: String,
    pub class: String,
    pub values: Vec<ArrayItem>,
}

impl ArrayItem {
    /// `AbstractValuesAwareNode::isValueKeyEquals()`.
    pub fn key_is(&self, key: &str) -> bool {
        matches!(&self.key, Some(Key::Val(Val::StringNode(k) | Val::Str(k))) if k == key)
    }
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Str(s) | Val::Const(s) => f.write_str(s),
            Val::StringNode(s) => write!(f, "\"{}\"", s.replace('"', "\"\"")),
            Val::Null => Ok(()),
            // `CurlyListNode::implode()`
            Val::Curly(items) => write!(f, "{{{}}}", join(items)),
            Val::List(items) => items.iter().try_for_each(|item| write!(f, "{item}")),
            // `ArrayItemNode::__toString()` of a nested annotation
            Val::Annotation(nested) => {
                write!(f, "@{}", nested.identifier.trim_start_matches('@'))?;
                match nested.values.is_empty() {
                    true => Ok(()),
                    false => write!(f, "({})", join(&nested.values)),
                }
            }
        }
    }
}

impl std::fmt::Display for ArrayItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(Key::Val(key)) = &self.key {
            write!(f, "{key}=")?;
        }
        write!(f, "{}", self.value)
    }
}

/// The items joined by `, ` (`printValuesContent()`, `CurlyListNode::implode()`).
pub(crate) fn join(items: &[ArrayItem]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

/// Why Rector's annotation parsers give no values: PHP throws (an object as an array key), or never returns (a
/// string left open).
#[derive(Debug)]
pub(crate) struct Unparsable;

/// Rector's `StaticDoctrineAnnotationParser` on the annotation content `content`; `resolve_tag` is
/// `ClassAnnotationMatcher::resolveTagFullyQualifiedName()`, for the nested annotations.
pub(crate) fn annotation_values(
    content: &str,
    resolve_tag: &dyn Fn(&str) -> String,
) -> Result<Vec<ArrayItem>, Unparsable> {
    let toks = tokenize(content);
    let mut parser = Static {
        it: Iter::new(content, &toks),
        resolve_tag,
    };
    parser.resolve_annotation_method_call()
}

struct Static<'a, 'r> {
    it: Iter<'a>,
    resolve_tag: &'r dyn Fn(&str) -> String,
}

/// A value `resolveAnnotationValue()` gives: one value, or a key and its value.
enum Resolved {
    Value(Val),
    Keyed(Val, Val),
}

/// A PHP array key.
#[derive(Clone, PartialEq)]
enum PhpKey {
    Int(i64),
    Str(String),
}

impl Static<'_, '_> {
    /// `resolveAnnotationMethodCall()`.
    fn resolve_annotation_method_call(&mut self) -> Result<Vec<ArrayItem>, Unparsable> {
        if !self.it.is(&[t::OPEN_PARENTHESES]) {
            return Ok(Vec::new());
        }
        self.it.next();
        if self.it.is(&[t::CLOSE_PARENTHESES]) {
            return Ok(Vec::new());
        }
        self.resolve_annotation_values()
    }

    /// `resolveAnnotationValue()`.
    fn resolve_annotation_value(&mut self) -> Result<Resolved, Unparsable> {
        self.it.try_consume(t::PHPDOC_EOL);
        if !self.it.is_next(&[t::EQUAL]) {
            return Ok(Resolved::Value(self.parse_value()?));
        }
        let key = self.parse_value()?;
        // `consumeTokenType(TOKEN_EQUAL)`: the key's parse may stop elsewhere, and PHP throws
        self.it.consume(t::EQUAL).map_err(|_| Unparsable)?;
        Ok(Resolved::Keyed(key, self.parse_value()?))
    }

    /// `resolveAnnotationValues()`: the values in a PHP array, `array_merge()`d, then `createArrayFromValues()`.
    fn resolve_annotation_values(&mut self) -> Result<Vec<ArrayItem>, Unparsable> {
        let mut values: Vec<(PhpKey, Val)> = Vec::new();
        let first = self.resolve_annotation_value()?;
        merge(&mut values, first)?;
        while self.it.is(&[t::COMMA]) {
            self.it.next();
            if self.it.is_next(&[t::CLOSE_PARENTHESES]) {
                continue;
            }
            match self.resolve_annotation_value()? {
                Resolved::Value(_) if self.it.is(&[t::END]) => break,
                resolved => merge(&mut values, resolved)?,
            }
        }
        Ok(values
            .into_iter()
            .enumerate()
            .map(|(natural, (key, value))| {
                let key = match key {
                    PhpKey::Int(k) if k == natural as i64 => None,
                    PhpKey::Int(k) => Some(Key::Int(k)),
                    PhpKey::Str(k) => Some(Key::Val(Val::Str(k))),
                };
                create_array_item(key, value)
            })
            .collect())
    }

    /// `StaticDoctrineAnnotationParser::parseValue()`.
    fn parse_value(&mut self) -> Result<Val, Unparsable> {
        if self.it.is(&[t::OPEN_CURLY_BRACKET]) {
            return Ok(Val::Curly(self.parse_curly_array()?));
        }
        self.parse_plain_value()
    }

    /// `PlainValueParser::parseValue()`.
    fn parse_plain_value(&mut self) -> Result<Val, Unparsable> {
        let current = self.it.value().to_string();
        if self.it.is(&[t::END]) {
            return Ok(Val::Str(current));
        }
        if self.it.is(&[t::OPEN_CURLY_BRACKET]) {
            return Ok(Val::List(self.parse_curly_array()?));
        }
        self.it.next();
        if let Some(constant) = match_constant_value(&current) {
            return Ok(constant);
        }
        let current = self.parse_string_value(current)?;
        if self.it.is(&[t::OPEN_PARENTHESES]) {
            return self.parse_nested_annotation(current);
        }
        // a lone `"`: Rector looks for the closing quote past the end and never returns
        (current != "\"").then_some(()).ok_or(Unparsable)?;
        Ok(Val::Str(current))
    }

    /// `parseStringValue()`.
    fn parse_string_value(&mut self, mut current: String) -> Result<String, Unparsable> {
        // `parseMultilineOrWhiteSpacedString()` on a `"` that no `"` closes: the lexer reads a string to the next `"` in
        // the doc, so none follows, and Rector reads past the end and never returns
        if current.starts_with('"') && !current.ends_with('"') {
            return Err(Unparsable);
        }
        while self.it.is(&[t::DOUBLE_COLON, t::IDENTIFIER]) {
            current.push_str(self.it.value());
            self.it.next();
        }
        Ok(current)
    }

    /// `parseNestedDoctrineAnnotationTagValueNode()`.
    fn parse_nested_annotation(&mut self, name: String) -> Result<Val, Unparsable> {
        let values = self.resolve_annotation_method_call()?;
        let class = (self.resolve_tag)(&name);
        self.it.try_consume(t::PHPDOC_EOL);
        if self.it.value() == ")" {
            self.it.next();
        }
        Ok(Val::Annotation(Box::new(Nested {
            identifier: name,
            class,
            values,
        })))
    }

    /// `ArrayParser::parseCurlyArray()`.
    fn parse_curly_array(&mut self) -> Result<Vec<ArrayItem>, Unparsable> {
        let mut values = Vec::new();
        self.it.next();
        if self.it.try_consume(t::CLOSE_CURLY_BRACKET) {
            return Ok(Vec::new());
        }
        values.push(self.resolve_array_item()?);
        while self.it.is(&[t::COMMA]) {
            self.it.next();
            self.it.try_consume(t::PHPDOC_EOL);
            if self.it.is(&[t::CLOSE_CURLY_BRACKET]) {
                break;
            }
            values.push(self.resolve_array_item()?);
            if self.it.is_next(&[t::CLOSE_CURLY_BRACKET]) {
                break;
            }
            self.it.try_consume(t::PHPDOC_EOL);
        }
        self.it.try_consume(t::PHPDOC_EOL);
        if !self.it.is(&[t::CLOSE_PARENTHESES]) {
            self.it.try_consume(t::CLOSE_CURLY_BRACKET);
        }
        Ok(values
            .into_iter()
            .map(|(key, value)| {
                // `createArrayFromValues()`: an integer node's key is its value, a string
                let key = key.map(|k| match k {
                    Val::Const(c) if c != "true" && c != "false" => Key::Val(Val::Str(c)),
                    k => Key::Val(k),
                });
                create_array_item(key, value)
            })
            .collect())
    }

    /// `ArrayParser::resolveArrayItem()`.
    fn resolve_array_item(&mut self) -> Result<(Option<Val>, Val), Unparsable> {
        self.it.try_consume(t::PHPDOC_EOL);
        let mut key = None;
        if self.it.is_next(&[t::DOUBLE_COLON]) {
            let mut k = self.it.value().to_string();
            self.it.next();
            k.push_str(self.it.value());
            self.it.next();
            k.push_str(self.it.value());
            self.it.next();
            key = Some(Val::Str(k));
        }
        self.it.try_consume(t::PHPDOC_EOL);
        if self.it.is(&[t::CLOSE_CURLY_BRACKET, t::COMMA]) {
            return Ok((None, key.unwrap_or(Val::Null)));
        }
        if self.it.is(&[t::EQUAL, t::COLON]) || self.it.is_next(&[t::EQUAL, t::COLON]) {
            self.it.try_consume(t::EQUAL);
            self.it.try_consume(t::COLON);
            if key.is_none() {
                if !self.it.is_next(&[t::IDENTIFIER]) {
                    self.it.try_consume(t::COMMA);
                }
                key = Some(self.parse_plain_value()?);
            }
            self.it.try_consume(t::EQUAL);
            self.it.try_consume(t::COLON);
        }
        Ok((key, self.parse_plain_value()?))
    }
}

/// `array_merge($values, [$key => $value])` or `$values[] = $value`: a string key already there keeps its place and
/// takes the new value, integer keys are numbered again; an object key throws.
fn merge(values: &mut Vec<(PhpKey, Val)>, resolved: Resolved) -> Result<(), Unparsable> {
    let next = values
        .iter()
        .filter(|(k, _)| matches!(k, PhpKey::Int(_)))
        .count() as i64;
    let (key, value) = match resolved {
        Resolved::Value(value) => (PhpKey::Int(next), value),
        // (an integer string is a `ConstExprIntegerNode` already, which `matchConstantValue()` gives)
        Resolved::Keyed(Val::Str(key), value) => (PhpKey::Str(key), value),
        Resolved::Keyed(..) => return Err(Unparsable),
    };
    match values
        .iter_mut()
        .find(|(k, _)| matches!(k, PhpKey::Str(_)) && *k == key)
    {
        Some(existing) => existing.1 = value,
        None => values.push((key, value)),
    }
    Ok(())
}

/// The integer a PHP array key `key` is cast to: a decimal integer written as PHP prints it.
fn php_int_key(key: &str) -> Option<i64> {
    let n: i64 = key.parse().ok()?;
    (n.to_string() == key).then_some(n)
}

/// `matchConstantValue()`.
fn match_constant_value(value: &str) -> Option<Val> {
    match value.to_ascii_lowercase().as_str() {
        "false" => return Some(Val::Const("false".into())),
        "true" => return Some(Val::Const("true".into())),
        _ => {}
    }
    php_int_key(value).map(|_| Val::Const(value.to_string()))
}

/// `createArrayItemFromKeyAndValue()`.
fn create_array_item(key: Option<Key>, value: Val) -> ArrayItem {
    let quoted = |v: &str, q: char| v.len() >= 2 && v.starts_with(q) && v.ends_with(q);
    let value = match value {
        Val::Str(v) if quoted(&v, '"') => Val::StringNode(v[1..v.len() - 1].replace("\"\"", "\"")),
        Val::Str(v) if quoted(&v, '\'') => Val::Str(v.trim_matches('\'').to_string()),
        // (`true` and `false` are `ConstExpr` nodes already)
        Val::Str(v) if v.eq_ignore_ascii_case("null") => Val::Null,
        value => value,
    };
    let key = match key {
        Some(Key::Val(Val::Str(k))) if quoted(&k, '"') => Some(Key::Val(Val::StringNode(
            k[1..k.len() - 1].replace("\"\"", "\""),
        ))),
        key => key,
    };
    ArrayItem { key, value }
}
