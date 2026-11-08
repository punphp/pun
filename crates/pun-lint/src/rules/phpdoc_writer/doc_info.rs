//! Rector's `PhpDocInfo` of a doc comment and its `PhpDocInfoPrinter`: the doc read as `BetterPhpDocParser` reads
//! it (phpdoc-parser 2.3's tokens, each child with its `START_AND_END` token positions), the typed tags it changes or
//! adds (`addTagValueNode()`, `PhpDocTypeChanger`), and the doc printed back: `printNew()` for a doc made from
//! nothing, `printFormatPreserving()` for one that had tokens (changed tags reprinted, the rest copied token by
//! token).

use std::sync::LazyLock;

use pun_phpstan::phpdoc::{self, Node};
use regex::Regex;

use super::type_node::TypeNode;

/// phpdoc-parser's token types, as far as the printer and the child boundaries tell them apart.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Kind {
    /// `TOKEN_OPEN_PHPDOC`: `/**` and one space after it.
    Open,
    /// `TOKEN_CLOSE_PHPDOC`: `*/`.
    Close,
    /// `TOKEN_PHPDOC_EOL`: a line break, the next line's indent, its `*` and one space after it.
    Eol,
    /// `TOKEN_HORIZONTAL_WS`.
    Ws,
    /// Any other token; a run of other characters is one.
    Other,
    /// `TOKEN_END`, empty.
    End,
}

/// A typed tag's value: phpdoc-parser's `ParamTagValueNode`, `VarTagValueNode`, `ReturnTagValueNode`.
#[derive(Clone, Debug)]
pub struct TypedValue {
    /// The type node as Rector prints it.
    pub ty: TypeNode,
    /// The type as phpdoc-parser parsed it, for PHPStan's type of it; `None` for a type Pun made.
    pub parsed: Option<Node>,
    /// `$name` of a `@param` or `@var` (`""` for a `@var` without one).
    pub variable: String,
    pub is_reference: bool,
    pub is_variadic: bool,
    /// The description, its line breaks followed by ` * ` (`BetterPhpDocParser::parseTagValue()`).
    pub description: String,
}

/// A tag's value.
#[derive(Clone, Debug)]
pub enum TagValue {
    /// `@param`, `@var` or `@return` with a type.
    Typed(TypedValue),
    /// Any other value, never reprinted: an untyped tag (a typeless `@param $a`).
    Other,
    /// A `@param`, `@var` or `@return` value its tag's parser rejects (`InvalidTagValueNode`), never reprinted.
    Invalid,
}

#[derive(Clone, Debug)]
pub enum ChildNode {
    /// `PhpDocTextNode`.
    Text,
    /// `PhpDocTagNode`.
    Tag { name: String, value: TagValue },
}

#[derive(Clone, Debug)]
struct Child {
    /// `START_AND_END`: the first token and the token after the last; `None` for a child Rector added.
    start_end: Option<(usize, usize)>,
    node: ChildNode,
    /// A node inside has no `ORIG_NODE` (`ChangedPhpDocNodeVisitor`).
    changed: bool,
}

impl Child {
    /// The tag's name and typed value, when it is a typed tag.
    fn typed(&self) -> Option<(&str, &TypedValue)> {
        match &self.node {
            ChildNode::Tag {
                name,
                value: TagValue::Typed(v),
            } => Some((name, v)),
            _ => None,
        }
    }

    /// `(string) $phpDocChildNode` of a reprinted child: only typed tags are ever changed or added (an untyped one is
    /// printed from its text).
    fn print(&self) -> String {
        let (name, v) = self.typed().expect("a changed or added child is a typed tag");
        let value = match name {
            "@param" => format!(
                "{} {}{}{} {}",
                v.ty,
                if v.is_reference { "&" } else { "" },
                if v.is_variadic { "..." } else { "" },
                v.variable,
                v.description
            ),
            "@var" => format!("{} {}", v.ty, format!("{} {}", v.variable, v.description).trim()),
            _ => format!("{} {}", v.ty, v.description),
        };
        format!("{name} {}", value.trim()).trim().to_string()
    }
}

/// A doc comment as Rector's `PhpDocInfo` holds it.
#[derive(Clone, Debug)]
pub struct PhpDocInfo {
    text: String,
    tokens: Vec<(Kind, usize, usize)>,
    children: Vec<Child>,
    single_line: bool,
    /// `LAST_PHP_DOC_TOKEN_POSITION`: where the last child ended when the doc was read.
    last_token_position: Option<usize>,
}

/// `PhpDocInfoPrinter::CALLABLE_REGEX`.
static CALLABLE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"callable\s+\(").unwrap());

/// The typed tag names Rector reads and writes.
const TYPED: [&str; 3] = ["@param", "@var", "@return"];

impl PhpDocInfo {
    /// `PhpDocInfoFactory::createEmpty()` (and `createFromNode()` of a node with comments but no doc comment).
    pub fn empty() -> PhpDocInfo {
        PhpDocInfo {
            text: String::new(),
            tokens: Vec::new(),
            children: Vec::new(),
            single_line: false,
            last_token_position: None,
        }
    }

    /// `PhpDocInfoFactory::createFromNode()` of a node whose doc comment is `text`.
    pub fn parse(text: &str) -> PhpDocInfo {
        let tokens = lex(text);
        let children = child_ranges(text, &tokens)
            .into_iter()
            .map(|(start, end)| Child {
                start_end: Some((start, end)),
                node: child_node(text, &tokens, start, end),
                changed: false,
            })
            .collect::<Vec<_>>();
        PhpDocInfo {
            text: text.to_string(),
            single_line: !tokens.iter().any(|t| t.0 == Kind::Eol),
            last_token_position: children.last().and_then(|c| c.start_end).map(|(_, e)| e),
            tokens,
            children,
        }
    }

    /// The typed values of the tags named `name`, in order (`PhpDocNode::getReturnTagValues()` and its kin).
    fn typed(&self, name: &str) -> impl Iterator<Item = &TypedValue> {
        self.children
            .iter()
            .filter_map(move |c| c.typed().filter(|(n, _)| *n == name).map(|(_, v)| v))
    }

    /// `hasInvalidTag()`: a tag named `name` whose value its parser rejects.
    pub fn has_invalid_tag(&self, name: &str) -> bool {
        (self.children.iter()).any(|c| matches!(&c.node, ChildNode::Tag { name: n, value: TagValue::Invalid } if n == name))
    }

    /// `getReturnTagValue()`.
    pub fn return_tag(&self) -> Option<&TypedValue> {
        self.typed("@return").next()
    }

    /// `getVarTagValueNode()`.
    pub fn var_tag(&self) -> Option<&TypedValue> {
        self.typed("@var").next()
    }

    /// `getParamTagValueByName()`: the first `@param` of `$name`.
    pub fn param_tag(&self, name: &str) -> Option<&TypedValue> {
        let wanted = format!("${}", name.trim_start_matches('$'));
        self.typed("@param").find(|v| v.variable == wanted)
    }

    /// The type of the tag named `name` (and naming `$variable`) set to `ty`, as `PhpDocTypeChanger` does: an existing
    /// tag gets the new type and is reprinted; else a new tag is added (`addTagValueNode()`).
    fn set_type(&mut self, name: &str, variable: &str, is_variadic: bool, ty: TypeNode) {
        let existing = self.children.iter_mut().find(|c| match &c.node {
            ChildNode::Tag {
                name: n,
                value: TagValue::Typed(v),
            } => n == name && (name != "@param" || v.variable == variable),
            _ => false,
        });
        if let Some(child) = existing {
            if let ChildNode::Tag {
                value: TagValue::Typed(v),
                ..
            } = &mut child.node
            {
                v.ty = ty;
                v.parsed = None;
            }
            child.changed = true;
            return;
        }
        self.children.push(Child {
            start_end: None,
            node: ChildNode::Tag {
                name: name.to_string(),
                value: TagValue::Typed(TypedValue {
                    ty,
                    parsed: None,
                    variable: variable.to_string(),
                    is_reference: false,
                    is_variadic,
                    description: String::new(),
                }),
            },
            changed: true,
        });
        // `addPhpDocTagNode()`: to give node more space
        self.single_line = false;
    }

    /// `changeReturnTypeNode()`.
    pub fn set_return_type(&mut self, ty: TypeNode) {
        self.set_type("@return", "", false, ty);
    }

    /// `changeVarTypeNode()`.
    pub fn set_var_type(&mut self, ty: TypeNode) {
        self.set_type("@var", "", false, ty);
    }

    /// `changeParamTypeNode()`: `$name` (`ParamPhpDocNodeFactory` marks a new tag of a variadic parameter
    /// variadic).
    pub fn set_param_type(&mut self, name: &str, is_variadic: bool, ty: TypeNode) {
        self.set_type("@param", &format!("${name}"), is_variadic, ty);
    }

    /// `DocBlockUpdater::printPhpDocInfoToString()`: the doc as Rector prints it after the changes.
    pub fn print(&self) -> String {
        // `isNewNode()`
        if !self.children.is_empty() && self.tokens.is_empty() {
            return self.print_new();
        }
        // `printFormatPreserving()`: a tag was added to a single-line docblock, printed fresh
        if !self.single_line && !self.tokens.iter().any(|t| t.0 == Kind::Eol) {
            return self.print_new();
        }
        self.print_format_preserving()
    }

    /// `PhpDocInfoPrinter::printNew()`: `(string) $phpDocNode`.
    fn print_new(&self) -> String {
        let children: Vec<String> = (self.children.iter())
            .map(|c| match (c.typed(), c.start_end) {
                // an untyped child of a single-line doc the new tag made multi-line: its own text
                (None, Some((start, end))) => {
                    format!(" {}", self.text[self.tokens[start].1..self.tokens[end - 1].2].trim())
                }
                _ => format!(" {}", c.print()),
            })
            .collect();
        // (never inlined: `addPhpDocTagNode()` made it multi-line)
        format!("/**\n *{}\n */", children.join("\n *"))
    }

    /// `PhpDocInfoPrinter::printFormatPreserving()`.
    fn print_format_preserving(&self) -> String {
        let mut printer = Printer {
            info: self,
            current: 0,
        };
        // hotfix of extra space with callable ()
        CALLABLE.replace_all(&printer.print_doc(), "callable(").into_owned()
    }
}

/// `PhpDocInfoPrinter`'s state while it prints one doc.
struct Printer<'a> {
    info: &'a PhpDocInfo,
    current: usize,
}

impl Printer<'_> {
    /// `printPhpDocNode()`.
    fn print_doc(&mut self) -> String {
        let info = self.info;
        // (`EmptyPhpDocDetector` never finds a changed doc empty: it has the changed tag)
        let mut output = String::new();
        let count = info.children.len();
        for (key, child) in info.children.iter().enumerate() {
            output.push_str(&self.print_child(child, key + 1, count));
        }
        output = self.print_end(output);
        if !["//", "/**", "/*", "#"].iter().any(|s| output.starts_with(s)) && !output.is_empty() {
            output.insert_str(0, "/**");
        }
        // (the doc's own `*/` is always printed)
        output.replace(" \r\n", "\r\n").replace(" \n", "\n")
    }

    /// `printDocChildNode()`.
    /// Only typed tags are changed, and a child Rector added is changed.
    fn print_child(&mut self, child: &Child, key: usize, count: usize) -> String {
        if child.changed {
            if let Some((_, end)) = child.start_end {
                self.current = end;
            }
            return self.standard_print(child);
        }
        let (_, end) = child.start_end.expect("an unchanged child was read");
        // (`correctPreviouslyReprintedFirstNode()` never moves back: every child before set the position)
        let output = self.add_tokens_from_to(String::new(), self.current, end, count == key);
        self.current = end;
        output.trim_end_matches([' ', '\t', '\n', '\r', '\0', '\x0B']).to_string()
    }

    /// `printEnd()`.
    fn print_end(&self, output: String) -> String {
        let last = self.info.last_token_position.unwrap_or(self.current);
        if last == 0 {
            return output + "\n */";
        }
        self.add_tokens_from_to(output, last, self.info.tokens.len(), true)
    }

    /// `addTokensFromTo()` (no child is removed, so nothing is jumped over).
    fn add_tokens_from_to(&self, mut output: String, from: usize, to: usize, skip_empty_lines_above: bool) -> String {
        let tokens = &self.info.tokens;
        let text = |i: usize| tokens.get(i).map_or("", |t| &self.info.text[t.1..t.2]);
        let mut from = from;
        // include also space before, in case of inlined docs
        if from > 0 && tokens[from - 1].0 == Kind::Ws {
            from -= 1;
        }
        // skip extra empty lines above if this is the last one
        if skip_empty_lines_above && text(from).contains('\n') && text(from + 1).contains('\n') {
            from += 1;
        }
        for i in from..to {
            output.push_str(text(i));
        }
        output
    }

    /// `standardPrintPhpDocChildNode()`.
    fn standard_print(&self, child: &Child) -> String {
        let printed = child.print();
        if self.info.single_line {
            return format!(" {printed}");
        }
        // (a typed tag never prints empty)
        format!("\n * {printed}")
    }
}

/// phpdoc-parser's `Lexer` over a doc comment, a run of other characters taken as one token.
pub(crate) fn lex(text: &str) -> Vec<(Kind, usize, usize)> {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 3 + usize::from(bytes.get(3) == Some(&b' '));
    tokens.push((Kind::Open, 0, i));
    while i < bytes.len() {
        let start = i;
        let kind = match bytes[i] {
            b'\r' | b'\n' => {
                i += if bytes[i] == b'\r' && bytes.get(i + 1) == Some(&b'\n') { 2 } else { 1 };
                while matches!(bytes.get(i), Some(b' ' | b'\t')) {
                    i += 1;
                }
                if bytes.get(i) == Some(&b'*') && bytes.get(i + 1) != Some(&b'/') {
                    i += 1 + usize::from(bytes.get(i + 1) == Some(&b' '));
                }
                Kind::Eol
            }
            b' ' | b'\t' => {
                while matches!(bytes.get(i), Some(b' ' | b'\t')) {
                    i += 1;
                }
                Kind::Ws
            }
            b'*' if bytes.get(i + 1) == Some(&b'/') => {
                i += 2;
                Kind::Close
            }
            _ => {
                while i < bytes.len()
                    && !matches!(bytes[i], b' ' | b'\t' | b'\r' | b'\n')
                    && !(bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/'))
                {
                    i += 1;
                }
                Kind::Other
            }
        };
        tokens.push((kind, start, i));
    }
    tokens.push((Kind::End, bytes.len(), bytes.len()));
    tokens
}

/// The tag name a token starts with (`TOKEN_PHPDOC_TAG`, or `TOKEN_DOCTRINE_TAG`), if it starts with one.
pub(crate) fn tag_name(word: &str) -> Option<&str> {
    let b = word.as_bytes();
    if b.first() != Some(&b'@') {
        return None;
    }
    let name_char = |c: u8| c.is_ascii_alphanumeric() || c == b'-' || c == b'\\';
    // `@(?:[a-z][a-z0-9-\\]+:)?[a-z][a-z0-9-\\]*+`
    let mut i = 1;
    if b.get(i).is_some_and(u8::is_ascii_alphabetic) {
        let mut j = i + 1;
        while j < b.len() && name_char(b[j]) {
            j += 1;
        }
        if b.get(j) == Some(&b':') && j > i + 1 && b.get(j + 1).is_some_and(u8::is_ascii_alphabetic) {
            i = j + 1;
        }
        let mut j = i + 1;
        while j < b.len() && name_char(b[j]) {
            j += 1;
        }
        return Some(&word[..j]);
    }
    // `@[a-z_\\][a-z0-9_\:\\]*[a-z_][a-z0-9_]*`
    if b.get(1).is_some_and(|c| *c == b'_' || *c == b'\\') {
        let end = b.iter().skip(1).take_while(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b':' | b'\\')).count() + 1;
        let tail = b[..end].iter().rposition(|c| c.is_ascii_alphabetic() || *c == b'_')?;
        return (tail > 1).then(|| &word[..end]);
    }
    None
}

/// The index of the first token at or after `i` that is not white space (`TokenIterator::next()`'s skipping).
fn skip_ws(tokens: &[(Kind, usize, usize)], i: usize) -> usize {
    let mut i = i;
    while i + 1 < tokens.len() && tokens[i].0 == Kind::Ws {
        i += 1;
    }
    i
}

/// `BetterPhpDocParser::parseWithNode()`'s children, each as its first token and the token after its last.
pub(crate) fn child_ranges(text: &str, tokens: &[(Kind, usize, usize)]) -> Vec<(usize, usize)> {
    let ends = |i: usize| matches!(tokens[i].0, Kind::Close | Kind::End);
    let mut pos = skip_ws(tokens, 1);
    if tokens[pos].0 == Kind::Eol {
        pos = skip_ws(tokens, pos + 1);
    }
    let mut ranges = Vec::new();
    if ends(pos) {
        return ranges;
    }
    loop {
        let end = child_end(text, tokens, pos);
        ranges.push((pos, end));
        if tokens[end].0 != Kind::Eol {
            break;
        }
        pos = skip_ws(tokens, end + 1);
        if ends(pos) {
            break;
        }
    }
    ranges
}

/// Where the child that starts at `start` ends (`parseText()`'s save points): at the line end after its last line
/// that is not blank, the lines after it running up to the next line that starts with a tag.
fn child_end(text: &str, tokens: &[(Kind, usize, usize)], start: usize) -> usize {
    let is_tag = |i: usize| tokens[i].0 == Kind::Other && tag_name(&text[tokens[i].1..tokens[i].2]).is_some();
    let mut last = None;
    let mut p = start;
    loop {
        let mut q = p;
        while !matches!(tokens[q].0, Kind::Eol | Kind::Close | Kind::End) {
            q += 1;
        }
        let non_empty = q > p || (p > 0 && tokens[p - 1].0 == Kind::Ws);
        if tokens[q].0 != Kind::Eol {
            if non_empty || last.is_none() {
                last = Some(q);
            }
            break;
        }
        if non_empty || last.is_none() {
            last = Some(q);
        }
        let next = skip_ws(tokens, q + 1);
        if is_tag(next) || matches!(tokens[next].0, Kind::Close | Kind::End) {
            break;
        }
        p = next;
    }
    last.unwrap_or(start)
}

/// The child node of the tokens `[start, end)`.
fn child_node(text: &str, tokens: &[(Kind, usize, usize)], start: usize, end: usize) -> ChildNode {
    let first = &text[tokens[start].1..tokens[start].2];
    let Some(name) = tokens
        .get(start)
        .filter(|t| t.0 == Kind::Other)
        .and_then(|_| tag_name(first))
    else {
        return ChildNode::Text;
    };
    let value = if TYPED.contains(&name) {
        let value_start = tokens[start].1 + name.len();
        typed_value(text, tokens, name, value_start, tokens[end].1)
    } else {
        TagValue::Other
    };
    ChildNode::Tag {
        name: name.to_string(),
        value,
    }
}

/// Whether `text` starts with the variable `$this` (`TOKEN_THIS_VARIABLE`).
fn is_this_variable(text: &str) -> bool {
    text.strip_prefix("$this")
        .is_some_and(|after| !after.bytes().next().is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_' || c >= 0x80))
}

/// A `@param`, `@var` or `@return` value, from `from` to `to` (bytes of the doc).
fn typed_value(
    text: &str,
    tokens: &[(Kind, usize, usize)],
    name: &str,
    from: usize,
    to: usize,
) -> TagValue {
    let value = &text[from..to];
    let leading = value.len() - value.trim_start_matches([' ', '\t']).len();
    let rest = &value[leading..];
    // a typeless `@param $a`
    if name == "@param" && ((rest.starts_with('$') && !is_this_variable(rest)) || rest.starts_with('&') || rest.starts_with("...")) {
        return TagValue::Other;
    }
    let Ok((parsed, offset)) = phpdoc::parse_prefix(rest) else {
        return TagValue::Invalid;
    };
    let type_text = rest[..offset].trim_end_matches([' ', '\t']);
    let ty = TypeNode::from_parsed(&parsed).unwrap_or_else(|| TypeNode::Raw(type_text.to_string()));
    let mut at = leading + type_text.len();
    let preceded_by_ws = |at: usize| at > 0 && matches!(value.as_bytes()[at - 1], b' ' | b'\t');
    let skip = |at: usize| at + value[at..].len() - value[at..].trim_start_matches([' ', '\t']).len();
    let mut variable = String::new();
    let (mut is_reference, mut is_variadic) = (false, false);
    let variable_len = |s: &str| {
        let b = s.as_bytes();
        (b.first() == Some(&b'$') && b.get(1).is_some_and(|c| c.is_ascii_alphabetic() || *c == b'_' || *c >= 0x80))
            .then(|| 1 + b[1..].iter().take_while(|c| c.is_ascii_alphanumeric() || **c == b'_' || **c >= 0x80).count())
    };
    let mut limit_start = name == "@return";
    match name {
        "@param" => {
            at = skip(at);
            if value[at..].starts_with('&') {
                is_reference = true;
                at = skip(at + 1);
            }
            if value[at..].starts_with("...") {
                is_variadic = true;
                at = skip(at + 3);
            }
            // `parseRequiredVariableName()`: a `TOKEN_VARIABLE`, which `$this` is not
            let Some(len) = variable_len(&value[at..]).filter(|len| &value[at..at + len] != "$this") else {
                return TagValue::Invalid;
            };
            variable = value[at..at + len].to_string();
            at += len;
        }
        "@var" => {
            let next = skip(at);
            if let Some(len) = variable_len(&value[next..]) {
                variable = value[next..next + len].to_string();
                at = next + len;
            } else {
                limit_start = true;
            }
        }
        _ => {}
    }
    // `parseOptionalDescription($tokens, $limitStartToken)`
    let next = skip(at);
    if limit_start {
        let starts = &value[next..];
        // (a `TOKEN_UNION` or `TOKEN_INTERSECTION` never follows the type: the type parser takes it or fails)
        if next < value.len() && !starts.starts_with(['\r', '\n']) && !preceded_by_ws(next) {
            return TagValue::Invalid;
        }
    }
    // the description's lines joined by "\n" (each line break with the next line's `*` and one space), then
    // `BetterPhpDocParser` puts ` * ` after each line break
    let mut description = String::new();
    let start = from + at;
    for t in tokens.iter().filter(|t| t.1 >= start && t.2 <= to) {
        match t.0 {
            Kind::Eol => description.push('\n'),
            _ => description.push_str(&text[t.1..t.2]),
        }
    }
    if let Some(t) = tokens.iter().find(|t| t.1 < start && t.2 > start) {
        description.insert_str(0, &text[start..t.2]);
    }
    let description = description
        .trim_matches([' ', '\t'])
        .trim_end_matches('\n')
        .replace('\n', "\n * ");
    TagValue::Typed(TypedValue {
        ty,
        parsed: Some(parsed),
        variable,
        is_reference,
        is_variadic,
        description,
    })
}
