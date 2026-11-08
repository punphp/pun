//! NEON, ported from nette/neon v3.3.4 (the version bundled in PHPStan 2.3.5): `Lexer`, `TokenStream`, `Parser`, the
//! node `toValue()`s and `Encoder`. Error messages and positions are nette/neon's.

use std::sync::LazyLock;

use pcre2::bytes::{Regex, RegexBuilder};

use crate::value::{Array, Entity, Key, Value, base_dec, float_json, numeric};

pub const CHAIN: &str = "!!chain";

/// nette/neon's `Exception` message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

const STRING: &str = r#"
			'''\n (?:(?: [^\n] | \n(?![\t\ ]*+''') )*+ \n)?[\t\ ]*+''' |
			"""\n (?:(?: [^\n] | \n(?![\t\ ]*+""") )*+ \n)?[\t\ ]*+""" |
			' (?: '' | [^'\n] )*+ ' |
			" (?: \\. | [^"\\\n] )*+ "
		"#;
const LITERAL: &str = r#"
			(?: [^#"',:=[\]{}()\n\t\ `-] | (?<!["']) [:-] [^"',=[\]{}()\n\t\ ] )
			(?:
				[^,:=\]})(\n\t\ ]++ |
				:(?! [\n\t\ ,\]})] | $ ) |
				[\ \t]++ [^#,:=\]})(\n\t\ ]
			)*+
		"#;
const CHAR: &str = r"[,:=[\]{}()-]";
const COMMENT: &str = r"\#.*+";
const NEWLINE: &str = r"\n++";
const WHITESPACE: &str = r"[\t\ ]++";

fn build(pattern: &str, caseless: bool, multi_line: bool, extended: bool, utf: bool) -> Regex {
    RegexBuilder::new()
        .caseless(caseless)
        .multi_line(multi_line)
        .extended(extended)
        .utf(utf)
        .ucp(utf)
        .build(pattern)
        .expect("static regex")
}

/// `~(String)|(Literal)|(Char)|(Comment)|(Newline)|(Whitespace)~Amixu`, anchored with `\G`. Matched on bytes, without
/// `u`: every character the patterns name is ASCII, so a multi-byte character's bytes fall in the same negated
/// classes and the tokens are the same, while PCRE2's UTF mode would re-validate the whole input at each token.
static TOKENS: LazyLock<Regex> = LazyLock::new(|| {
    let p =
        format!(r"\G(?:({STRING})|({LITERAL})|({CHAR})|({COMMENT})|({NEWLINE})|({WHITESPACE}))");
    build(&p, true, true, true, false)
});
static REQUIRES_DELIMITERS: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new()
        .caseless(true)
        .build(r"[\x00-\x1F]|^[+-.]?\d|^(true|false|yes|no|on|off|null)$(?!\n)")
        .unwrap()
});
static LITERAL_ONLY: LazyLock<Regex> =
    LazyLock::new(|| build(&format!(r"^{LITERAL}$(?!\n)"), false, false, true, false));
static DATETIME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\G\d\d\d\d-\d\d?-\d\d?(?:(?:[Tt]| ++)\d\d?:\d\d:\d\d(?:\.\d*+)? *+(?:Z|[-+]\d\d?(?::?\d\d)?)?)?$(?!\n)",
    )
    .unwrap()
});

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum T {
    String,
    Literal,
    Comment,
    Newline,
    Whitespace,
    Char(u8),
}

struct Token {
    value: String,
    kind: T,
}

struct Stream {
    tokens: Vec<Token>,
    pos: usize,
}

impl Stream {
    fn kind_at(&self, pos: usize) -> Option<T> {
        self.tokens.get(pos).map(|t| t.kind)
    }

    fn is_next(&mut self, types: &[T]) -> bool {
        while matches!(self.kind_at(self.pos), Some(T::Comment | T::Whitespace)) {
            self.pos += 1;
        }
        match self.kind_at(self.pos) {
            None => false,
            Some(k) => types.is_empty() || types.contains(&k),
        }
    }

    fn has_next(&mut self) -> bool {
        self.is_next(&[])
    }

    fn consume(&mut self, types: &[T]) -> Option<String> {
        if self.is_next(types) {
            self.pos += 1;
            Some(self.tokens[self.pos - 1].value.clone())
        } else {
            None
        }
    }

    fn consume_newlines(&mut self) {
        while self.consume(&[T::Newline]).is_some() {}
    }

    fn indentation(&self) -> String {
        let before = self.pos.checked_sub(2).and_then(|p| self.kind_at(p));
        let prev = self.pos.checked_sub(1).and_then(|p| self.kind_at(p));
        if matches!(before, Some(T::Newline) | None) && prev == Some(T::Whitespace) {
            self.tokens[self.pos - 1].value.clone()
        } else {
            String::new()
        }
    }

    fn error_at<R>(&self, message: Option<&str>, pos: usize) -> Result<R, Error> {
        let input: String = self
            .tokens
            .iter()
            .take(pos)
            .map(|t| t.value.as_str())
            .collect();
        let line = input.matches('\n').count() + 1;
        let col = input.len() - input.rfind('\n').map_or(0, |i| i + 1) + 1;
        let message = match message {
            Some(m) => m.to_string(),
            None => match self.tokens.get(pos) {
                None => "Unexpected end".to_string(),
                Some(t) => format!(
                    "Unexpected '{}'",
                    substr_bytes(&t.value, 40).replace('\n', "<new line>")
                ),
            },
        };
        Err(Error(format!("{message} on line {line}, column {col}.")))
    }

    fn error<R>(&self) -> Result<R, Error> {
        self.error_at(None, self.pos)
    }
}

/// `substr($s, 0, $n)` on a UTF-8 string (cut at a byte boundary, lossily if mid-character).
fn substr_bytes(s: &str, n: usize) -> String {
    String::from_utf8_lossy(&s.as_bytes()[..s.len().min(n)]).into_owned()
}

fn tokenize(input: &str) -> Result<Stream, Error> {
    let input = input.replace('\r', "");
    let bytes = input.as_bytes();
    let mut locs = TOKENS.capture_locations();
    let mut tokens = Vec::new();
    let mut offset = 0;
    while offset < bytes.len() {
        let Ok(Some(m)) = TOKENS.captures_read_at(&mut locs, bytes, offset) else {
            break;
        };
        let (start, end) = (m.start(), m.end());
        let group = (1..=6)
            .find(|&g| locs.get(g).is_some())
            .expect("one group matches");
        let value = input[start..end].to_string();
        let kind = match group {
            1 => T::String,
            2 => T::Literal,
            3 => T::Char(bytes[start]),
            4 => T::Comment,
            5 => T::Newline,
            _ => T::Whitespace,
        };
        tokens.push(Token { value, kind });
        offset = end;
    }
    let stream = Stream { tokens, pos: 0 };
    if offset != bytes.len() {
        let s = substr_bytes(&input[offset..], 40).replace('\n', "\\n");
        return stream.error_at(Some(&format!("Unexpected '{s}'")), stream.tokens.len());
    }
    Ok(stream)
}

enum Node {
    Literal(Value),
    Str(String),
    Array(Vec<Item>),
    Entity(Box<Node>, Vec<Item>),
    Chain(Vec<Node>),
}

struct Item {
    key: Option<Node>,
    value: Node,
    /// `startTokenPos`
    start: usize,
}

struct Parser {
    s: Stream,
}

impl Parser {
    fn parse(&mut self) -> Result<Node, Error> {
        self.s.consume_newlines();
        let indent = self.s.indentation();
        let node = self.parse_block(&indent, false)?;
        self.s.consume_newlines();
        if self.s.has_next() {
            return self.s.error();
        }
        Ok(node)
    }

    fn parse_block(&mut self, indent: &str, only_bullets: bool) -> Result<Node, Error> {
        let mut items: Vec<Item> = Vec::new();
        let mut keys = Vec::new();
        loop {
            let start = self.s.pos;
            let mut key = None;
            if self.s.consume(&[T::Char(b'-')]).is_some() {
            } else if !self.s.has_next() || only_bullets {
                return Ok(if items.is_empty() {
                    Node::Literal(Value::Null)
                } else {
                    Node::Array(items)
                });
            } else {
                let pos = self.s.pos;
                let value = self.parse_value()?;
                if self.s.consume(&[T::Char(b':'), T::Char(b'=')]).is_some() {
                    self.check_key(&value, pos, &mut keys)?;
                    key = Some(value);
                } else {
                    if !items.is_empty() {
                        return self.s.error();
                    }
                    return Ok(value);
                }
            }

            let mut value = Node::Literal(Value::Null);
            let mut close = false;
            if self.s.consume(&[T::Newline]).is_some() {
                self.s.consume_newlines();
                let next = self.s.indentation();
                if !prefix_compatible(&next, indent) {
                    return self
                        .s
                        .error_at(Some("Invalid combination of tabs and spaces"), self.s.pos);
                } else if next.len() > indent.len() {
                    value = self.parse_block(&next, false)?;
                } else if next.len() < indent.len() {
                    close = true;
                } else if key.is_some() && self.s.is_next(&[T::Char(b'-')]) {
                    value = self.parse_block(indent, true)?;
                }
            } else if key.is_none() {
                value = self.parse_block(&format!("{indent}  "), false)?;
            } else if self.s.has_next() {
                value = self.parse_value()?;
                if self.s.has_next() && !self.s.is_next(&[T::Newline]) {
                    return self.s.error();
                }
            }
            items.push(Item { key, value, start });
            if close {
                return Ok(Node::Array(items));
            }

            self.s.consume_newlines();
            if !self.s.has_next() {
                return Ok(Node::Array(items));
            }
            let next = self.s.indentation();
            if !prefix_compatible(&next, indent) {
                return self
                    .s
                    .error_at(Some("Invalid combination of tabs and spaces"), self.s.pos);
            } else if next.len() > indent.len() {
                return self.s.error_at(Some("Bad indentation"), self.s.pos);
            } else if next.len() < indent.len() {
                return Ok(Node::Array(items));
            }
        }
    }

    fn parse_value(&mut self) -> Result<Node, Error> {
        let node = if let Some(s) = self.s.consume(&[T::String]) {
            match parse_string(&s) {
                Ok(v) => Node::Str(v),
                Err(m) => return self.s.error_at(Some(&m), self.s.pos - 1),
            }
        } else if let Some(s) = self.s.consume(&[T::Literal]) {
            let is_key = self.s.is_next(&[T::Char(b':'), T::Char(b'=')]);
            Node::Literal(parse_literal(&s, is_key))
        } else if self
            .s
            .is_next(&[T::Char(b'['), T::Char(b'('), T::Char(b'{')])
        {
            Node::Array(self.parse_braces()?)
        } else {
            return self.s.error();
        };
        self.parse_entity(node)
    }

    fn parse_entity(&mut self, node: Node) -> Result<Node, Error> {
        if !self.s.is_next(&[T::Char(b'(')]) {
            return Ok(node);
        }
        let attributes = self.parse_braces()?;
        let mut entities = vec![Node::Entity(Box::new(node), attributes)];
        while let Some(s) = self.s.consume(&[T::Literal]) {
            let value = Node::Literal(parse_literal(&s, false));
            if self.s.is_next(&[T::Char(b'(')]) {
                let attributes = self.parse_braces()?;
                entities.push(Node::Entity(Box::new(value), attributes));
            } else {
                entities.push(Node::Entity(Box::new(value), Vec::new()));
                break;
            }
        }
        Ok(if entities.len() == 1 {
            entities.pop().expect("one")
        } else {
            Node::Chain(entities)
        })
    }

    fn parse_braces(&mut self) -> Result<Vec<Item>, Error> {
        let open = self.s.consume(&[]).expect("brace");
        let end = T::Char(match open.as_str() {
            "[" => b']',
            "{" => b'}',
            _ => b')',
        });
        let mut items = Vec::new();
        let mut keys = Vec::new();
        loop {
            self.s.consume_newlines();
            if self.s.consume(&[end]).is_some() {
                return Ok(items);
            }
            let pos = self.s.pos;
            let value = self.parse_value()?;
            let item = if self.s.consume(&[T::Char(b':'), T::Char(b'=')]).is_some() {
                self.check_key(&value, pos, &mut keys)?;
                let v = if self.s.is_next(&[T::Newline, T::Char(b','), end]) {
                    Node::Literal(Value::Null)
                } else {
                    self.parse_value()?
                };
                Item {
                    key: Some(value),
                    value: v,
                    start: pos,
                }
            } else {
                Item {
                    key: None,
                    value,
                    start: pos,
                }
            };
            items.push(item);
            if self.s.consume(&[T::Char(b','), T::Newline]).is_some() {
                continue;
            }
            self.s.consume_newlines();
            if !self.s.is_next(&[end]) {
                return self.s.error();
            }
        }
    }

    /// `pos` is `$key->startTokenPos` (the stream position is never on whitespace or a comment).
    fn check_key(&mut self, key: &Node, pos: usize, keys: &mut Vec<String>) -> Result<(), Error> {
        let k = match key {
            Node::Str(s) => s.clone(),
            Node::Literal(v) if v.is_scalar() => v.to_php_string().expect("scalar"),
            _ => return self.s.error_at(Some("Unacceptable key"), pos),
        };
        if keys.contains(&k) {
            return self.s.error_at(Some(&format!("Duplicated key '{k}'")), pos);
        }
        keys.push(k);
        Ok(())
    }
}

/// `strncmp($next, $indent, min(strlen($next), strlen($indent))) === 0`.
fn prefix_compatible(a: &str, b: &str) -> bool {
    let n = a.len().min(b.len());
    a.as_bytes()[..n] == b.as_bytes()[..n]
}

/// `LiteralNode::parse()`.
fn parse_literal(value: &str, is_key: bool) -> Value {
    if !is_key {
        match value {
            "true" | "True" | "TRUE" | "yes" | "Yes" | "YES" | "on" | "On" | "ON" => {
                return Value::Bool(true);
            }
            "false" | "False" | "FALSE" | "no" | "No" | "NO" | "off" | "Off" | "OFF" => {
                return Value::Bool(false);
            }
            "null" | "Null" | "NULL" => return Value::Null,
            _ => {}
        }
    }
    if let Some(n) = numeric(value) {
        return n;
    }
    for (prefix, base, digits) in [
        ("0x", 16, "0123456789abcdefABCDEF"),
        ("0o", 8, "01234567"),
        ("0b", 2, "01"),
    ] {
        if let Some(rest) = value.strip_prefix(prefix)
            && !rest.is_empty()
            && rest.chars().all(|c| digits.contains(c))
        {
            return base_dec(rest, base);
        }
    }
    if !is_key && DATETIME.is_match(value.as_bytes()).unwrap_or(false) {
        return Value::DateTime(value.to_string());
    }
    Value::Str(value.to_string())
}

/// `StringNode::parse()`; `Err` carries the message (positioned by the caller).
fn parse_string(s: &str) -> Result<String, String> {
    let b = s.as_bytes();
    let multiline = b.len() > 3 && b[3] == b'\n';
    let res = if multiline {
        let after = s[3..].trim_start_matches('\n');
        let indent: String = after
            .chars()
            .take_while(|c| *c == '\t' || *c == ' ')
            .collect();
        let mut res = s[3..s.len() - 3].replace(&format!("\n{indent}"), "\n");
        if res.starts_with('\n') {
            res.remove(0);
        }
        if let Some(nl) = res.rfind('\n')
            && res[nl + 1..].chars().all(|c| c == '\t' || c == ' ')
        {
            res.truncate(nl);
        }
        res
    } else {
        let inner = &s[1..s.len() - 1];
        if b[0] == b'\'' {
            inner.replace("''", "'")
        } else {
            inner.to_string()
        }
    };
    if b[0] == b'\'' {
        return Ok(res);
    }
    unescape(&res)
}

/// `preg_replace_callback('#\\(?:ud[89ab][0-9a-f]{2}\\ud[c-f][0-9a-f]{2}|u[0-9a-f]{4}|x[0-9a-f]{2}|.)#i', ...)`.
fn unescape(s: &str) -> Result<String, String> {
    let b = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'\\' || i + 1 >= b.len() || b[i + 1] == b'\n' {
            out.push(b[i]);
            i += 1;
            continue;
        }
        let rest = &b[i + 1..];
        let is_u = |r: &[u8]| {
            r.len() >= 5 && (r[0] | 0x20) == b'u' && r[1..5].iter().all(u8::is_ascii_hexdigit)
        };
        let pair = is_u(rest)
            && (rest[1] | 0x20) == b'd'
            && matches!(rest[2] | 0x20, b'8' | b'9' | b'a' | b'b')
            && rest.len() >= 11
            && rest[5] == b'\\'
            && is_u(&rest[6..])
            && (rest[7] | 0x20) == b'd'
            && matches!(rest[8] | 0x20, b'c'..=b'f');
        let len = if pair {
            12
        } else if is_u(rest) {
            6
        } else if rest.len() >= 3
            && (rest[0] | 0x20) == b'x'
            && rest[1].is_ascii_hexdigit()
            && rest[2].is_ascii_hexdigit()
        {
            4
        } else {
            2 // one byte, as PCRE without /u: the message may show half a character
        };
        let sq = String::from_utf8_lossy(&s.as_bytes()[i..i + len]);
        let c = sq.as_bytes()[1];
        match c {
            b't' => out.push(b'\t'),
            b'n' => out.push(b'\n'),
            b'r' => out.push(b'\r'),
            b'f' => out.push(0x0C),
            b'b' => out.push(0x08),
            b'"' | b'\\' | b'/' => out.push(c),
            b'_' => out.extend_from_slice("\u{A0}".as_bytes()),
            b'u' if len >= 6 => {
                let unit = |h: &str| u32::from_str_radix(h, 16).expect("hex");
                let ch = if len == 12 {
                    let (hi, lo) = (unit(&sq[2..6]), unit(&sq[8..12]));
                    char::from_u32(0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00))
                } else {
                    char::from_u32(unit(&sq[2..6]))
                };
                match ch {
                    Some(ch) => out.extend_from_slice(ch.to_string().as_bytes()),
                    None => return Err(format!("Invalid UTF-8 sequence {sq}")),
                }
            }
            b'x' if len == 4 => out.push(u8::from_str_radix(&sq[2..4], 16).expect("hex")),
            _ => return Err(format!("Invalid escaping sequence {sq}")),
        }
        i += len;
    }
    // ponytail: PHP strings are bytes; a deprecated `\xHH` >= 0x80 is replaced by U+FFFD here.
    Ok(String::from_utf8(out)
        .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned()))
}

fn items_to_array(items: Vec<Item>) -> Array {
    let mut res = Array::new();
    for item in items {
        let value = to_value(item.value);
        match item.key {
            None => res.push(value),
            Some(k) => {
                let k = to_value(k).to_php_string().expect("checked key");
                res.set(Key::from_string(k), value);
            }
        }
    }
    res
}

fn to_value(node: Node) -> Value {
    match node {
        Node::Literal(v) => v,
        Node::Str(s) => Value::Str(s),
        Node::Array(items) => Value::Array(items_to_array(items)),
        Node::Entity(value, attributes) => Value::Entity(Box::new(Entity {
            value: to_value(*value),
            attributes: items_to_array(attributes),
        })),
        Node::Chain(chain) => Value::Entity(Box::new(Entity {
            value: Value::str(CHAIN),
            attributes: Array::list(chain.into_iter().map(to_value)),
        })),
    }
}

/// `Neon::decode()`.
pub fn decode(input: &str) -> Result<Value, Error> {
    let mut parser = Parser {
        s: tokenize(input)?,
    };
    Ok(to_value(parser.parse()?))
}

/// IgnoreErrorsConfigFixer::findNeonEntries(): each item of the `parameters: ignoreErrors:` list as decoded, with its
/// `startLine`. None when the file does not decode or the list is not an array.
pub fn ignore_errors_items(input: &[u8]) -> Option<Vec<(Value, usize)>> {
    let mut parser = Parser {
        s: tokenize(std::str::from_utf8(input).ok()?).ok()?,
    };
    let root = parser.parse().ok()?;
    // Parser::initLines()
    let mut lines = Vec::with_capacity(parser.s.tokens.len() + 1);
    let mut line = 1;
    for token in &parser.s.tokens {
        lines.push(line);
        line += token.value.matches('\n').count();
    }
    lines.push(line);
    // findNeonItem(): the first item with the key
    let find = |node: Node, key: &str| match node {
        Node::Array(items) => items.into_iter().find(|i| {
            i.key.as_ref().is_some_and(|k| match k {
                Node::Str(s) => s == key,
                Node::Literal(v) => v.to_php_string().as_deref() == Some(key),
                _ => false,
            })
        }),
        _ => None,
    };
    let parameters = find(root, "parameters")?;
    let Node::Array(items) = find(parameters.value, "ignoreErrors")?.value else {
        return None;
    };
    Some(
        items
            .into_iter()
            .map(|i| (to_value(i.value), lines[i.start]))
            .collect(),
    )
}

/// `Neon::decode()` on bytes: nette/neon's lexer (`/u`) rejects invalid UTF-8 up front.
pub fn decode_bytes(input: &[u8]) -> Result<Value, Error> {
    let s = std::str::from_utf8(input).map_err(|_| Error("Invalid UTF-8 sequence.".into()))?;
    decode(s)
}

/// `Lexer::requiresDelimiters()`.
fn requires_delimiters(s: &str) -> bool {
    REQUIRES_DELIMITERS.is_match(s.as_bytes()).expect("regex")
        || !LITERAL_ONLY.is_match(s.as_bytes()).expect("regex")
}

/// `Neon::encode($value, $blockMode, $indentation)`.
pub fn encode(value: &Value, block: bool, indentation: &str) -> String {
    Encoder { indentation }.node(value, block).to_string()
}

struct Encoder<'a> {
    indentation: &'a str,
}

/// The encoder's node tree (`toString()` side of nette/neon's nodes).
enum Out {
    Text(String),
    Block(String, Vec<(Option<Out>, Out)>),
    Inline(char, Vec<(Option<Out>, Out)>),
    Entity(Box<Out>, Vec<(Option<Out>, Out)>),
    Chain(Vec<Out>),
}

impl Encoder<'_> {
    fn node(&self, v: &Value, block: bool) -> Out {
        match v {
            Value::Entity(e) if e.value == Value::str(CHAIN) => {
                Out::Chain(e.attributes.values().map(|x| self.node(x, false)).collect())
            }
            Value::Entity(e) => Out::Entity(
                Box::new(self.node(&e.value, false)),
                self.items(&e.attributes, false),
            ),
            Value::Array(a) => {
                if block {
                    Out::Block(String::new(), self.items(a, true))
                } else {
                    Out::Inline(if a.is_list() { '[' } else { '{' }, self.items(a, false))
                }
            }
            Value::Str(s) if requires_delimiters(s) => Out::Text(string_to_string(s)),
            other => Out::Text(literal_to_string(other)),
        }
    }

    fn items(&self, a: &Array, block: bool) -> Vec<(Option<Out>, Out)> {
        let mut res = Vec::new();
        let mut counter: i64 = 0;
        let mut hide = true;
        for (k, v) in a.iter() {
            let key = match k {
                Key::Int(i) if hide && *i == counter => None,
                Key::Int(i) => Some(self.node(&Value::Int(*i), false)),
                Key::Str(s) => Some(self.node(&Value::str(s.as_str()), false)),
            };
            let mut value = self.node(v, block);
            if let Out::Block(ind, _) = &mut value {
                *ind = self.indentation.to_string();
            }
            if hide && let Key::Int(i) = k {
                hide = *i == counter;
                counter = counter.max(i + 1);
            }
            res.push((key, value));
        }
        res
    }
}

impl std::fmt::Display for Out {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inline = |items: &[(Option<Out>, Out)]| {
            items
                .iter()
                .map(|(k, v)| match k {
                    Some(k) => format!("{k}: {v}"),
                    None => v.to_string(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        match self {
            Out::Text(s) => f.write_str(s),
            Out::Inline(b, items) => {
                let close = if *b == '[' { ']' } else { '}' };
                write!(f, "{b}{}{close}", inline(items))
            }
            Out::Entity(v, items) => write!(f, "{v}({})", inline(items)),
            Out::Chain(c) => c.iter().try_for_each(|e| write!(f, "{e}")),
            Out::Block(indentation, items) => {
                if items.is_empty() {
                    return f.write_str("[]");
                }
                let mut res = String::new();
                for (k, v) in items {
                    let s = v.to_string();
                    res.push_str(&match k {
                        Some(k) => format!("{k}:"),
                        None => "-".into(),
                    });
                    if matches!(v, Out::Block(_, items) if !items.is_empty()) {
                        res.push('\n');
                        res.push_str(&s);
                        if s.as_bytes().get(s.len().wrapping_sub(2)) != Some(&b'\n') {
                            res.push('\n');
                        }
                    } else {
                        res.push(' ');
                        res.push_str(&s);
                        res.push('\n');
                    }
                }
                f.write_str(&indent_lines(&res, indentation))
            }
        }
    }
}

/// `preg_replace('#^(?=.)#m', $indentation, $s)`: indent every non-empty line.
fn indent_lines(s: &str, indentation: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for (i, line) in s.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        if !line.is_empty() {
            out.push_str(indentation);
        }
        out.push_str(line);
    }
    out
}

fn literal_to_string(v: &Value) -> String {
    match v {
        Value::Float(f) => {
            let s = float_json(*f);
            if s.contains('.') { s } else { s + ".0" }
        }
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::DateTime(raw) => crate::value::datetime_format(raw, "Y-m-d H:i:s O"),
        other => other
            .to_php_string()
            .unwrap_or_else(|| format!("{other:?}")),
    }
}

/// `StringNode::toString()`.
fn string_to_string(s: &str) -> String {
    let control = |c: char| matches!(c, '\x00'..='\x08' | '\x0B'..='\x1F');
    if !s.contains('\n') {
        return if s.chars().any(control) {
            json_string(s)
        } else {
            format!("'{}'", s.replace('\'', "''"))
        };
    }
    let delim;
    let body = if s.chars().any(control) || has_indented_triple_quote(s) {
        let j = json_string(s);
        let mut out = String::new();
        let mut chars = j[1..j.len() - 1].chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                let n = chars.next().expect("escape");
                match n {
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    '"' => out.push('"'),
                    _ => {
                        out.push('\\');
                        out.push(n);
                    }
                }
            } else {
                out.push(c);
            }
        }
        delim = "\"\"\"";
        out.replace("\"\"\"", "\"\"\\\"")
    } else {
        delim = "'''";
        s.to_string()
    };
    format!("{delim}\n{}\n{delim}", indent_lines(&body, "\t"))
}

/// `\n[\t ]+'''`
fn has_indented_triple_quote(s: &str) -> bool {
    s.split('\n').skip(1).any(|line| {
        let t = line.trim_start_matches(['\t', ' ']);
        t.len() < line.len() && t.starts_with("'''")
    })
}

/// `json_encode($s, JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES)`.
pub fn json_string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x08' => out.push_str("\\b"),
            '\x0C' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
