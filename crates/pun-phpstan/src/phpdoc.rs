//! phpstan/phpdoc-parser 2.3's `Lexer`, `TokenIterator`, `ConstExprParser`, `TypeParser` and `PhpDocParser`, with
//! the `ParserException` messages PHPStan reports (InvalidPhpDocTagValueRule). The type AST is the part of
//! phpdoc-parser's that Pun resolves: call-site variance and a callable's own templates are dropped.
//!
//! Pun's own extension, used by the generated symbol data (scripts/export-phpstan-symbols.php) for types PHPStan's
//! `toPhpDocNode()` cannot write: a whole type string `*ERROR*` or `*NEVER*` ([`parse`] only).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tok {
    Reference,
    Union,
    Inter,
    Nullable,
    Negated,
    OpenParen,
    CloseParen,
    OpenAngle,
    CloseAngle,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    Comma,
    Comment,
    Variadic,
    DoubleColon,
    DoubleArrow,
    Arrow,
    Equal,
    Colon,
    OpenPhpdoc,
    ClosePhpdoc,
    PhpdocTag,
    DoctrineTag,
    Eol,
    Float,
    Int,
    SingleQuoted,
    DoubleQuoted,
    DoctrineString,
    Ident,
    This,
    Var,
    Ws,
    Other,
    End,
    Wildcard,
}

/// `Lexer::TOKEN_LABELS`: how a ParserException names the token it expected.
const LABELS: &[(Tok, &str)] = &[
    (Tok::Reference, "'&'"),
    (Tok::Inter, "'&'"),
    (Tok::Union, "'|'"),
    (Tok::Nullable, "'?'"),
    (Tok::Negated, "'!'"),
    (Tok::OpenParen, "'('"),
    (Tok::CloseParen, "')'"),
    (Tok::OpenAngle, "'<'"),
    (Tok::CloseAngle, "'>'"),
    (Tok::OpenSquare, "'['"),
    (Tok::CloseSquare, "']'"),
    (Tok::OpenCurly, "'{'"),
    (Tok::CloseCurly, "'}'"),
    (Tok::Comma, "','"),
    (Tok::Comment, "'//'"),
    (Tok::Colon, "':'"),
    (Tok::Variadic, "'...'"),
    (Tok::DoubleColon, "'::'"),
    (Tok::DoubleArrow, "'=>'"),
    (Tok::Arrow, "'->'"),
    (Tok::Equal, "'='"),
    (Tok::OpenPhpdoc, "'/**'"),
    (Tok::ClosePhpdoc, "'*/'"),
    (Tok::PhpdocTag, "TOKEN_PHPDOC_TAG"),
    (Tok::DoctrineTag, "TOKEN_DOCTRINE_TAG"),
    (Tok::Eol, "TOKEN_PHPDOC_EOL"),
    (Tok::Float, "TOKEN_FLOAT"),
    (Tok::Int, "TOKEN_INTEGER"),
    (Tok::SingleQuoted, "TOKEN_SINGLE_QUOTED_STRING"),
    (Tok::DoubleQuoted, "TOKEN_DOUBLE_QUOTED_STRING"),
    (Tok::DoctrineString, "TOKEN_DOCTRINE_ANNOTATION_STRING"),
    (Tok::Ident, "type"),
    (Tok::This, "'$this'"),
    (Tok::Var, "variable"),
    (Tok::Ws, "TOKEN_HORIZONTAL_WS"),
    (Tok::Other, "TOKEN_OTHER"),
    (Tok::End, "TOKEN_END"),
    (Tok::Wildcard, "*"),
];

impl Tok {
    fn label(self) -> &'static str {
        LABELS
            .iter()
            .find(|(kind, _)| *kind == self)
            .map_or("", |(_, label)| label)
    }
}

/// `Lexer::generateRegexp()`'s alternatives in order (PCRE alternation: the first that matches wins).
const PATTERNS: &[(Tok, &str)] = &[
    (Tok::Ws, r"[\x09\x20]++"),
    (
        Tok::Ident,
        r"(?:[\\]?+[a-z_\x80-\xFF][0-9a-z_\x80-\xFF-]*+)++",
    ),
    (Tok::This, r"\$this(?![0-9a-z_\x80-\xFF])"),
    (Tok::Var, r"\$[a-z_\x80-\xFF][0-9a-z_\x80-\xFF]*+"),
    (
        Tok::Reference,
        r"&(?=\s*+(?:[.,=)]|(?:\$(?!this(?![0-9a-z_\x80-\xFF])))))",
    ),
    (Tok::Union, r"\|"),
    (Tok::Inter, "&"),
    (Tok::Nullable, r"\?"),
    (Tok::Negated, "!"),
    (Tok::OpenParen, r"\("),
    (Tok::CloseParen, r"\)"),
    (Tok::OpenAngle, "<"),
    (Tok::CloseAngle, ">"),
    (Tok::OpenSquare, r"\["),
    (Tok::CloseSquare, r"\]"),
    (Tok::OpenCurly, r"\{"),
    (Tok::CloseCurly, r"\}"),
    (Tok::Comma, ","),
    (Tok::Comment, r"\/\/[^\r\n]*(?=\n|\r|\*/)"),
    (Tok::Variadic, r"\.\.\."),
    (Tok::DoubleColon, "::"),
    (Tok::DoubleArrow, "=>"),
    (Tok::Arrow, "->"),
    (Tok::Equal, "="),
    (Tok::Colon, ":"),
    (Tok::OpenPhpdoc, r"/\*\*(?=\s)\x20?+"),
    (Tok::ClosePhpdoc, r"\*/"),
    (
        Tok::PhpdocTag,
        r"@(?:[a-z][a-z0-9-\\]+:)?[a-z][a-z0-9-\\]*+",
    ),
    (Tok::DoctrineTag, r"@[a-z_\\][a-z0-9_\:\\]*[a-z_][a-z0-9_]*"),
    (Tok::Eol, r"\r?+\n[\x09\x20]*+(?:\*(?!/)\x20?+)?"),
    (
        Tok::Float,
        r"[+\-]?(?:(?:[0-9]++(?:_[0-9]++)*\.[0-9]*+(?:_[0-9]++)*(?:e[+\-]?[0-9]++(?:_[0-9]++)*)?)|(?:[0-9]*+(?:_[0-9]++)*\.[0-9]++(?:_[0-9]++)*(?:e[+\-]?[0-9]++(?:_[0-9]++)*)?)|(?:[0-9]++(?:_[0-9]++)*e[+\-]?[0-9]++(?:_[0-9]++)*))",
    ),
    (
        Tok::Int,
        r"[+\-]?(?:(?:0b[0-1]++(?:_[0-1]++)*)|(?:0o[0-7]++(?:_[0-7]++)*)|(?:0x[0-9a-f]++(?:_[0-9a-f]++)*)|(?:[0-9]++(?:_[0-9]++)*))",
    ),
    (Tok::SingleQuoted, r"'(?:\\[^\r\n]|[^'\r\n\\])*+'"),
    (Tok::DoubleQuoted, r#""(?:\\[^\r\n]|[^"\r\n\\])*+""#),
    (Tok::DoctrineString, r#""(?:""|[^"])*+""#),
    (Tok::Wildcard, r"\*"),
    (Tok::Other, r"(?:(?!\*/)[^\s])++"),
];

/// `Lexer::tokenize()`: (value, type, line). Like `preg_match_all` with the `A` modifier, lexing stops at the first
/// byte no alternative matches (a lone `\r`, a vertical tab).
fn tokenize(input: &str) -> Vec<(Tok, String, usize)> {
    use std::sync::OnceLock;
    static RE: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        let alternatives: Vec<String> = PATTERNS.iter().map(|(_, p)| format!("({p})")).collect();
        pcre2::bytes::RegexBuilder::new()
            .caseless(true)
            .dotall(true)
            .build(&format!(r"\G(?:{})", alternatives.join("|")))
            .expect("phpdoc lexer regex")
    });
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut offset = 0;
    let mut line = 1;
    let mut locs = re.capture_locations();
    while offset < bytes.len()
        && re
            .captures_read_at(&mut locs, bytes, offset)
            .ok()
            .flatten()
            .is_some()
    {
        let (start, end) = locs.get(0).expect("whole match");
        let group = (1..=PATTERNS.len())
            .find(|&g| locs.get(g).is_some())
            .expect("one alternative");
        let kind = PATTERNS[group - 1].0;
        tokens.push((kind, input[start..end].to_string(), line));
        if kind == Tok::Eol {
            line += 1;
        }
        offset = end;
    }
    tokens.push((Tok::End, String::new(), line));
    tokens
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Ident(String),
    This,
    /// `*` (a generic argument), `NEVER` / `ERROR` for `*NEVER*` / `*ERROR*`
    Star(String),
    Nullable(Box<Node>),
    Union(Vec<Node>),
    Inter(Vec<Node>),
    Conditional {
        subject: Box<Node>,
        target: Box<Node>,
        then: Box<Node>,
        otherwise: Box<Node>,
        negated: bool,
    },
    ConditionalForParameter {
        param: String,
        target: Box<Node>,
        then: Box<Node>,
        otherwise: Box<Node>,
        negated: bool,
    },
    Array(Box<Node>),
    /// `T[K]`
    OffsetAccess(Box<Node>, Box<Node>),
    Generic {
        base: String,
        args: Vec<Node>,
    },
    Callable {
        ident: String,
        /// `callable<T>(T): T`'s templates
        templates: Vec<TemplateTag>,
        params: Vec<CallableParam>,
        ret: Box<Node>,
    },
    ArrayShape {
        kind: String,
        items: Vec<ShapeItem>,
        sealed: bool,
        unsealed: Option<(Option<Box<Node>>, Box<Node>)>,
    },
    ObjectShape(Vec<(String, bool, Node)>),
    Const(ConstExpr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallableParam {
    pub ty: Node,
    pub by_ref: bool,
    pub variadic: bool,
    pub name: String,
    pub optional: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShapeItem {
    pub key: Option<ShapeKey>,
    pub optional: bool,
    pub value: Node,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeKey {
    Int(String),
    Str(String),
    Ident(String),
    /// `Class::NAME`
    Fetch(String, String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstExpr {
    Int(String),
    Float(String),
    Str(String),
    True,
    False,
    Null,
    Array(Vec<(Option<ConstExpr>, ConstExpr)>),
    /// `Class::NAME`, `Class::PREFIX_*`; a global constant has an empty class
    Fetch(String, String),
}

/// phpdoc-parser's `ParserException`: the token found and the one expected.
#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    value: String,
    offset: usize,
    expected: Tok,
    expected_value: Option<String>,
    line: usize,
}

impl ParseError {
    /// `ParserException::getMessage()`
    pub fn message(&self) -> String {
        format!(
            "Unexpected token {}, expected {}{} at offset {} on line {}",
            json_string(&self.value),
            self.expected.label(),
            self.expected_value
                .as_ref()
                .map(|v| format!(" ({})", json_string(v)))
                .unwrap_or_default(),
            self.offset,
            self.line
        )
    }
}

/// `json_encode($s, JSON_UNESCAPED_UNICODE | JSON_UNESCAPED_SLASHES)`: serde_json escapes the same characters.
fn json_string(s: &str) -> String {
    serde_json::to_string(s).expect("a string")
}

type R<T> = Result<T, ParseError>;

/// `TokenIterator`
struct Tokens {
    tokens: Vec<(Tok, String, usize)>,
    /// the byte offset of each token
    offsets: Vec<usize>,
    index: usize,
    saves: Vec<usize>,
    /// `addEndOfLineToSkippedTokens()` is in effect
    skip_eol: bool,
    newline: Option<&'static str>,
}

impl Tokens {
    fn new(input: &str) -> Self {
        let tokens = tokenize(input);
        let offsets = tokens
            .iter()
            .scan(0, |sum, (_, v, _)| {
                let at = *sum;
                *sum += v.len();
                Some(at)
            })
            .collect();
        let mut t = Tokens {
            tokens,
            offsets,
            index: 0,
            saves: Vec::new(),
            skip_eol: false,
            newline: None,
        };
        t.skip_irrelevant();
        t
    }
    fn skipped(&self, kind: Tok) -> bool {
        kind == Tok::Ws || (self.skip_eol && kind == Tok::Eol)
    }
    fn skip_irrelevant(&mut self) {
        while self.skipped(self.tokens[self.index].0) && self.index + 1 < self.tokens.len() {
            self.index += 1;
        }
    }
    fn kind(&self) -> Tok {
        self.tokens[self.index].0
    }
    fn value(&self) -> &str {
        &self.tokens[self.index].1
    }
    fn line(&self) -> usize {
        self.tokens[self.index].2
    }
    fn is(&self, kind: Tok) -> bool {
        self.kind() == kind
    }
    fn is_any(&self, kinds: &[Tok]) -> bool {
        kinds.contains(&self.kind())
    }
    fn is_value(&self, value: &str) -> bool {
        self.value() == value
    }
    fn preceded_by_ws(&self) -> bool {
        self.index > 0 && self.tokens[self.index - 1].0 == Tok::Ws
    }
    /// `getSkippedHorizontalWhiteSpaceIfAny()`
    fn skipped_ws(&self) -> &str {
        if self.preceded_by_ws() {
            &self.tokens[self.index - 1].1
        } else {
            ""
        }
    }
    fn next(&mut self) {
        // ponytail: PHP reads past the TOKEN_END (null); nothing reads after moving past it here
        self.index = (self.index + 1).min(self.tokens.len() - 1);
        self.skip_irrelevant();
    }
    fn error(&self, expected: Tok, expected_value: Option<&str>) -> ParseError {
        ParseError {
            value: self.value().to_string(),
            offset: self.offsets[self.index],
            expected,
            expected_value: expected_value.map(str::to_string),
            line: self.line(),
        }
    }
    fn consume(&mut self, kind: Tok) -> R<()> {
        if !self.is(kind) {
            return Err(self.error(kind, None));
        }
        self.detect_newline(kind);
        self.next();
        Ok(())
    }
    fn consume_value(&mut self, kind: Tok, value: &str) -> R<()> {
        if !self.is(kind) || !self.is_value(value) {
            return Err(self.error(kind, Some(value)));
        }
        self.next();
        Ok(())
    }
    fn try_consume(&mut self, kind: Tok) -> bool {
        if !self.is(kind) {
            return false;
        }
        self.detect_newline(kind);
        self.next();
        true
    }
    fn try_consume_value(&mut self, value: &str) -> bool {
        if !self.is_value(value) {
            return false;
        }
        self.next();
        true
    }
    fn detect_newline(&mut self, kind: Tok) {
        if kind == Tok::Eol && self.newline.is_none() {
            if self.value().starts_with("\r\n") {
                self.newline = Some("\r\n");
            } else if self.value().starts_with('\n') {
                self.newline = Some("\n");
            }
        }
    }
    /// `skipNewLineTokensAndConsumeComments()` (the comments are not kept)
    fn skip_nl(&mut self) {
        if self.is(Tok::Comment) {
            self.next();
        }
        while self.try_consume(Tok::Eol) {
            if self.is(Tok::Comment) {
                self.next();
            }
        }
    }
    fn join_until(&mut self, kinds: &[Tok]) -> String {
        let mut s = String::new();
        while !self.is_any(kinds) {
            s.push_str(&self.tokens[self.index].1);
            self.index += 1;
        }
        s
    }
    fn push(&mut self) {
        self.saves.push(self.index);
    }
    fn drop_save(&mut self) {
        self.saves.pop();
    }
    fn rollback(&mut self) {
        self.index = self.saves.pop().expect("a save point");
    }
    /// The byte offset of the current token.
    fn offset(&self) -> usize {
        self.offsets[self.index]
    }
}

/// `TypeStringResolver::resolve()`'s parse step: one type, then the end of input.
pub fn parse(input: &str) -> R<Node> {
    if let Some(name) = input
        .strip_prefix('*')
        .and_then(|s| s.strip_suffix('*'))
        .filter(|s| matches!(*s, "NEVER" | "ERROR"))
    {
        return Ok(Node::Star(name.to_string()));
    }
    let mut t = Tokens::new(input);
    let node = parse_type(&mut t)?;
    t.consume(Tok::End)?;
    Ok(node)
}

/// phpdoc-parser's `TypeParser::parse()` at the start of a PHPDoc tag's value: the type and the byte offset where
/// the rest of the value starts.
pub fn parse_prefix(input: &str) -> R<(Node, usize)> {
    let mut t = Tokens::new(input);
    let node = parse_type(&mut t)?;
    Ok((node, t.offset()))
}

/// `TypeParser::parse()`
fn parse_type(t: &mut Tokens) -> R<Node> {
    if t.is(Tok::Nullable) {
        return parse_nullable(t);
    }
    let ty = parse_atomic(t)?;
    t.push();
    t.skip_nl();
    match enrich(t, &ty) {
        Ok(Some(enriched)) => {
            t.drop_save();
            Ok(enriched)
        }
        _ => {
            t.rollback();
            Ok(enrich(t, &ty)?.unwrap_or(ty))
        }
    }
}

/// `enrichTypeOnUnionOrIntersection()`
fn enrich(t: &mut Tokens, ty: &Node) -> R<Option<Node>> {
    for (sep, wrap) in [
        (Tok::Union, Node::Union as fn(Vec<Node>) -> Node),
        (Tok::Inter, Node::Inter),
    ] {
        if t.is(sep) {
            let mut types = vec![ty.clone()];
            while t.try_consume(sep) {
                types.push(parse_atomic(t)?);
                t.push();
                t.skip_nl();
                if !t.is(sep) {
                    t.rollback();
                    break;
                }
                t.drop_save();
            }
            return Ok(Some(wrap(types)));
        }
    }
    Ok(None)
}

fn sub_parse(t: &mut Tokens) -> R<Node> {
    if t.is(Tok::Nullable) {
        return parse_nullable(t);
    }
    if t.is(Tok::Var) {
        let param = t.value().to_string();
        t.consume(Tok::Var)?;
        t.consume_value(Tok::Ident, "is")?;
        let (target, then, otherwise, negated) = parse_conditional_rest(t)?;
        return Ok(Node::ConditionalForParameter {
            param,
            target,
            then,
            otherwise,
            negated,
        });
    }
    let ty = parse_atomic(t)?;
    if t.is_value("is") {
        t.consume(Tok::Ident)?;
        let (target, then, otherwise, negated) = parse_conditional_rest(t)?;
        return Ok(Node::Conditional {
            subject: Box::new(ty),
            target,
            then,
            otherwise,
            negated,
        });
    }
    t.skip_nl();
    for (sep, wrap) in [
        (Tok::Union, Node::Union as fn(Vec<Node>) -> Node),
        (Tok::Inter, Node::Inter),
    ] {
        if t.is(sep) {
            let mut types = vec![ty];
            while t.try_consume(sep) {
                t.skip_nl();
                types.push(parse_atomic(t)?);
                t.skip_nl();
            }
            return Ok(wrap(types));
        }
    }
    Ok(ty)
}

type Conditional = (Box<Node>, Box<Node>, Box<Node>, bool);

fn parse_conditional_rest(t: &mut Tokens) -> R<Conditional> {
    let negated = t.is_value("not");
    if negated {
        t.consume(Tok::Ident)?;
    }
    let target = parse_type(t)?;
    t.skip_nl();
    t.consume(Tok::Nullable)?;
    t.skip_nl();
    let then = parse_type(t)?;
    t.skip_nl();
    t.consume(Tok::Colon)?;
    t.skip_nl();
    let otherwise = sub_parse(t)?;
    Ok((
        Box::new(target),
        Box::new(then),
        Box::new(otherwise),
        negated,
    ))
}

fn parse_nullable(t: &mut Tokens) -> R<Node> {
    t.consume(Tok::Nullable)?;
    Ok(Node::Nullable(Box::new(parse_atomic(t)?)))
}

const SHAPE_KINDS: &[&str] = &[
    "array",
    "list",
    "non-empty-array",
    "non-empty-list",
    "object",
];

fn parse_atomic(t: &mut Tokens) -> R<Node> {
    if t.try_consume(Tok::OpenParen) {
        t.skip_nl();
        let ty = sub_parse(t)?;
        t.skip_nl();
        t.consume(Tok::CloseParen)?;
        return Ok(array_or_offset(t, ty));
    }
    if t.try_consume(Tok::This) {
        return Ok(array_or_offset(t, Node::This));
    }
    let name = t.value().to_string();
    t.push();
    if t.try_consume(Tok::Ident) {
        if !t.is(Tok::DoubleColon) {
            t.drop_save();
            let mut ty = Node::Ident(name.clone());
            if t.is(Tok::OpenAngle) {
                t.push();
                let html = is_html(t);
                t.rollback();
                if html {
                    return Ok(ty);
                }
                ty = match try_parse_callable(t, &name, true) {
                    Some(callable) => callable,
                    None => {
                        let generic = parse_generic(t, name)?;
                        array_or_offset(t, generic)
                    }
                };
            } else if t.is(Tok::OpenParen) {
                ty = try_parse_callable(t, &name, false).unwrap_or(ty);
            } else if t.is(Tok::OpenSquare) {
                ty = array_or_offset(t, ty);
            } else if SHAPE_KINDS.contains(&name.as_str())
                && t.is(Tok::OpenCurly)
                && !t.preceded_by_ws()
            {
                let shape = if name == "object" {
                    parse_object_shape(t)?
                } else {
                    parse_array_shape(t, name)?
                };
                ty = array_or_offset(t, shape);
            }
            return Ok(ty);
        }
        t.rollback();
    } else {
        t.drop_save();
    }
    const_type(t)
}

/// A constant expression as a type (`ConstTypeNode`); an array is no type.
fn const_type(t: &mut Tokens) -> R<Node> {
    let at = t.error(Tok::Ident, None);
    let constant = parse_const_expr(t, false)?;
    if matches!(constant, ConstExpr::Array(_)) {
        return Err(at);
    }
    Ok(array_or_offset(t, Node::Const(constant)))
}

/// `isHtml()`: `<tag>` ... `</tag>` (a description, not generics).
fn is_html(t: &mut Tokens) -> bool {
    t.next();
    if !t.is(Tok::Ident) {
        return false;
    }
    let tag = t.value().to_string();
    t.next();
    if !t.try_consume(Tok::CloseAngle) {
        return false;
    }
    let end_tag = format!("</{tag}>");
    let closing = format!("/{tag}>");
    while !t.is(Tok::End) {
        if (t.try_consume(Tok::OpenAngle) && t.value().contains(&closing))
            || t.value().ends_with(&end_tag)
        {
            return true;
        }
        t.next();
    }
    false
}

/// `parseGeneric()`
fn parse_generic(t: &mut Tokens, base: String) -> R<Node> {
    t.consume(Tok::OpenAngle)?;
    t.skip_nl();
    let mut args = Vec::new();
    let mut first = true;
    while first || t.try_consume(Tok::Comma) {
        t.skip_nl();
        if !first && t.is(Tok::CloseAngle) {
            break;
        }
        first = false;
        // `parseGenericTypeArgument()`; ponytail: the variance is dropped (described as the invariant argument)
        if t.try_consume(Tok::Wildcard) {
            args.push(Node::Star("*".to_string()));
        } else {
            let _ = t.try_consume_value("contravariant") || t.try_consume_value("covariant");
            args.push(parse_type(t)?);
        }
        t.skip_nl();
    }
    t.consume(Tok::CloseAngle)?;
    Ok(Node::Generic { base, args })
}

/// `TypeParser::parseTemplateTagValue()`
fn parse_template(t: &mut Tokens, description: bool) -> R<TemplateTag> {
    let name = t.value().to_string();
    t.consume(Tok::Ident)?;
    let bound = if t.try_consume_value("of") || t.try_consume_value("as") {
        Some(parse_type(t)?)
    } else {
        None
    };
    let lower = if t.try_consume_value("super") {
        Some(parse_type(t)?)
    } else {
        None
    };
    let default = if t.try_consume_value("=") {
        Some(parse_type(t)?)
    } else {
        None
    };
    if description {
        parse_optional_description(t, true)?;
    }
    Ok(TemplateTag {
        name,
        bound,
        lower,
        default,
    })
}

fn try_parse_callable(t: &mut Tokens, ident: &str, templates: bool) -> Option<Node> {
    t.push();
    match parse_callable(t, ident, templates) {
        Ok(node) => {
            t.drop_save();
            Some(node)
        }
        Err(_) => {
            t.rollback();
            None
        }
    }
}

fn parse_callable(t: &mut Tokens, ident: &str, templates: bool) -> R<Node> {
    let mut tags = Vec::new();
    if templates {
        // `parseCallableTemplates()`
        t.consume(Tok::OpenAngle)?;
        let mut first = true;
        while first || t.try_consume(Tok::Comma) {
            t.skip_nl();
            if !first && t.is(Tok::CloseAngle) {
                break;
            }
            first = false;
            tags.push(parse_template(t, false)?);
            t.skip_nl();
        }
        t.consume(Tok::CloseAngle)?;
    }
    t.consume(Tok::OpenParen)?;
    t.skip_nl();
    let mut params = Vec::new();
    if !t.is(Tok::CloseParen) {
        params.push(parse_callable_param(t)?);
        t.skip_nl();
        while t.try_consume(Tok::Comma) {
            t.skip_nl();
            if t.is(Tok::CloseParen) {
                break;
            }
            params.push(parse_callable_param(t)?);
            t.skip_nl();
        }
    }
    t.consume(Tok::CloseParen)?;
    t.consume(Tok::Colon)?;
    let ret = parse_callable_return(t)?;
    Ok(Node::Callable {
        ident: ident.to_string(),
        templates: tags,
        params,
        ret: Box::new(ret),
    })
}

fn parse_callable_param(t: &mut Tokens) -> R<CallableParam> {
    let ty = parse_type(t)?;
    let by_ref = t.try_consume(Tok::Reference);
    let variadic = t.try_consume(Tok::Variadic);
    let name = if t.is(Tok::Var) {
        let name = t.value().to_string();
        t.next();
        name
    } else {
        String::new()
    };
    let optional = t.try_consume(Tok::Equal);
    Ok(CallableParam {
        ty,
        by_ref,
        variadic,
        name,
        optional,
    })
}

fn parse_callable_return(t: &mut Tokens) -> R<Node> {
    if t.is(Tok::Nullable) {
        return parse_nullable(t);
    }
    if t.try_consume(Tok::OpenParen) {
        let ty = sub_parse(t)?;
        t.consume(Tok::CloseParen)?;
        return Ok(array_or_offset(t, ty));
    }
    if t.try_consume(Tok::This) {
        return Ok(array_or_offset(t, Node::This));
    }
    let name = t.value().to_string();
    t.push();
    if t.try_consume(Tok::Ident) {
        if !t.is(Tok::DoubleColon) {
            t.drop_save();
            let mut ty = Node::Ident(name.clone());
            if t.is(Tok::OpenAngle) {
                let generic = parse_generic(t, name)?;
                ty = array_or_offset(t, generic);
            } else if t.is(Tok::OpenSquare) {
                ty = array_or_offset(t, ty);
            } else if SHAPE_KINDS.contains(&name.as_str())
                && t.is(Tok::OpenCurly)
                && !t.preceded_by_ws()
            {
                let shape = if name == "object" {
                    parse_object_shape(t)?
                } else {
                    parse_array_shape(t, name)?
                };
                ty = array_or_offset(t, shape);
            }
            return Ok(ty);
        }
        t.rollback();
    } else {
        t.drop_save();
    }
    const_type(t)
}

/// `tryParseArrayOrOffsetAccess()`: `T[]` and `T[K]` suffixes.
fn array_or_offset(t: &mut Tokens, mut ty: Node) -> Node {
    while t.is(Tok::OpenSquare) {
        t.push();
        let offset_access = !t.preceded_by_ws();
        t.next();
        let suffix = if offset_access && !t.is(Tok::CloseSquare) {
            parse_type(t).and_then(|offset| {
                t.consume(Tok::CloseSquare)?;
                Ok(Node::OffsetAccess(Box::new(ty.clone()), Box::new(offset)))
            })
        } else {
            t.consume(Tok::CloseSquare)
                .map(|()| Node::Array(Box::new(ty.clone())))
        };
        match suffix {
            Ok(node) => {
                t.drop_save();
                ty = node;
            }
            Err(_) => {
                t.rollback();
                break;
            }
        }
    }
    ty
}

fn parse_array_shape(t: &mut Tokens, kind: String) -> R<Node> {
    t.consume(Tok::OpenCurly)?;
    let mut items = Vec::new();
    let mut sealed = true;
    let mut unsealed = None;
    loop {
        t.skip_nl();
        if t.try_consume(Tok::CloseCurly) {
            return Ok(Node::ArrayShape {
                kind,
                items,
                sealed: true,
                unsealed: None,
            });
        }
        if t.try_consume(Tok::Variadic) {
            sealed = false;
            t.skip_nl();
            if t.is(Tok::OpenAngle) {
                t.consume(Tok::OpenAngle)?;
                t.skip_nl();
                let mut value = parse_type(t)?;
                t.skip_nl();
                let mut key = None;
                if kind == "array" && t.try_consume(Tok::Comma) {
                    t.skip_nl();
                    key = Some(Box::new(value));
                    value = parse_type(t)?;
                    t.skip_nl();
                }
                t.consume(Tok::CloseAngle)?;
                unsealed = Some((key, Box::new(value)));
                t.skip_nl();
            }
            t.try_consume(Tok::Comma);
            break;
        }
        items.push(parse_shape_item(t)?);
        t.skip_nl();
        let done = !t.try_consume(Tok::Comma);
        if t.is(Tok::Comment) {
            t.next();
        }
        if done {
            break;
        }
    }
    t.skip_nl();
    t.consume(Tok::CloseCurly)?;
    Ok(Node::ArrayShape {
        kind,
        items,
        sealed,
        unsealed,
    })
}

fn parse_shape_item(t: &mut Tokens) -> R<ShapeItem> {
    t.skip_nl();
    t.push();
    let keyed = parse_shape_key(t).and_then(|key| {
        let optional = t.try_consume(Tok::Nullable);
        t.consume(Tok::Colon)?;
        Ok((key, optional, parse_type(t)?))
    });
    match keyed {
        Ok((key, optional, value)) => {
            t.drop_save();
            Ok(ShapeItem {
                key: Some(key),
                optional,
                value,
            })
        }
        Err(_) => {
            t.rollback();
            Ok(ShapeItem {
                key: None,
                optional: false,
                value: parse_type(t)?,
            })
        }
    }
}

fn parse_shape_key(t: &mut Tokens) -> R<ShapeKey> {
    let key = match t.kind() {
        Tok::Int => ShapeKey::Int(t.value().replace('_', "")),
        Tok::SingleQuoted | Tok::DoubleQuoted => ShapeKey::Str(unescape(t.value())),
        _ => {
            let ident = t.value().to_string();
            t.consume(Tok::Ident)?;
            if t.try_consume(Tok::DoubleColon) {
                let name = t.value().to_string();
                t.consume(Tok::Ident)?;
                return Ok(ShapeKey::Fetch(ident, name));
            }
            return Ok(ShapeKey::Ident(ident));
        }
    };
    t.next();
    Ok(key)
}

fn parse_object_shape(t: &mut Tokens) -> R<Node> {
    t.consume(Tok::OpenCurly)?;
    let mut items = Vec::new();
    loop {
        t.skip_nl();
        if t.try_consume(Tok::CloseCurly) {
            return Ok(Node::ObjectShape(items));
        }
        t.skip_nl();
        let key = match t.kind() {
            Tok::SingleQuoted | Tok::DoubleQuoted => {
                let key = unescape(t.value());
                t.next();
                key
            }
            _ => {
                let key = t.value().to_string();
                t.consume(Tok::Ident)?;
                key
            }
        };
        let optional = t.try_consume(Tok::Nullable);
        t.consume(Tok::Colon)?;
        items.push((key, optional, parse_type(t)?));
        t.skip_nl();
        if !t.try_consume(Tok::Comma) {
            break;
        }
    }
    t.skip_nl();
    t.consume(Tok::CloseCurly)?;
    Ok(Node::ObjectShape(items))
}

/// `ConstExprParser::parse()`; `doctrine`: the parser for Doctrine annotation arguments (`toDoctrine()`).
fn parse_const_expr(t: &mut Tokens, doctrine: bool) -> R<ConstExpr> {
    match t.kind() {
        Tok::Float => {
            let value = t.value().replace('_', "");
            t.next();
            return Ok(ConstExpr::Float(value));
        }
        Tok::Int => {
            let value = t.value().replace('_', "");
            t.next();
            return Ok(ConstExpr::Int(value));
        }
        Tok::DoctrineString if doctrine => {
            let value = t.value().to_string();
            t.next();
            return Ok(ConstExpr::Str(value));
        }
        Tok::SingleQuoted if doctrine => return Err(t.error(Tok::DoubleQuoted, None)),
        Tok::DoubleQuoted if doctrine => {
            let value = t.value().to_string();
            t.next();
            return Ok(ConstExpr::Str(doctrine_string(t, value)));
        }
        Tok::SingleQuoted | Tok::DoubleQuoted => {
            let value = unescape(t.value());
            t.next();
            return Ok(ConstExpr::Str(value));
        }
        Tok::Ident => {
            let ident = t.value().to_string();
            t.next();
            match ident.to_ascii_lowercase().as_str() {
                "true" => return Ok(ConstExpr::True),
                "false" => return Ok(ConstExpr::False),
                "null" => return Ok(ConstExpr::Null),
                "array" => {
                    t.consume(Tok::OpenParen)?;
                    return parse_const_array(t, Tok::CloseParen, doctrine);
                }
                _ => {}
            }
            if t.try_consume(Tok::DoubleColon) {
                let mut name = String::new();
                let mut last = None;
                loop {
                    if last != Some(Tok::Ident) && t.is(Tok::Ident) {
                        name.push_str(t.value());
                        t.next();
                        last = Some(Tok::Ident);
                        continue;
                    }
                    if last != Some(Tok::Wildcard) && t.try_consume(Tok::Wildcard) {
                        name.push('*');
                        last = Some(Tok::Wildcard);
                        if !t.skipped_ws().is_empty() {
                            break;
                        }
                        continue;
                    }
                    if last.is_none() {
                        t.consume(Tok::Wildcard)?;
                    }
                    break;
                }
                return Ok(ConstExpr::Fetch(ident, name));
            }
            return Ok(ConstExpr::Fetch(String::new(), ident));
        }
        _ => {}
    }
    if t.try_consume(Tok::OpenSquare) {
        return parse_const_array(t, Tok::CloseSquare, doctrine);
    }
    Err(t.error(Tok::Ident, None))
}

fn parse_const_array(t: &mut Tokens, end: Tok, doctrine: bool) -> R<ConstExpr> {
    let mut items = Vec::new();
    if !t.try_consume(end) {
        loop {
            let expr = parse_const_expr(t, doctrine)?;
            items.push(if t.try_consume(Tok::DoubleArrow) {
                (Some(expr), parse_const_expr(t, doctrine)?)
            } else {
                (None, expr)
            });
            if !(t.try_consume(Tok::Comma) && !t.is(end)) {
                break;
            }
        }
        t.consume(end)?;
    }
    Ok(ConstExpr::Array(items))
}

/// `parseDoctrineString()`: a Doctrine string continues over the double-quoted and annotation strings after it
/// (the value is kept as written).
fn doctrine_string(t: &mut Tokens, mut text: String) -> String {
    while t.is_any(&[Tok::DoubleQuoted, Tok::DoctrineString]) {
        text.push_str(t.value());
        t.next();
    }
    text
}

/// A PHPDoc tag (`PhpDocTagNode`) and the line it starts on, counted from the doc comment's first line.
#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub line: usize,
    pub value: TagValue,
}

/// A `@template` tag's value (`TemplateTagValueNode`).
#[derive(Clone, Debug, PartialEq)]
pub struct TemplateTag {
    pub name: String,
    pub bound: Option<Node>,
    pub lower: Option<Node>,
    pub default: Option<Node>,
}

/// A `@method` tag's parameter (`MethodTagValueParameterNode`).
#[derive(Clone, Debug, PartialEq)]
pub struct MethodParam {
    pub ty: Option<Node>,
    pub by_ref: bool,
    pub variadic: bool,
    pub name: String,
    pub default: Option<ConstExpr>,
}

/// A tag's value: phpdoc-parser's `PhpDocTagValueNode` subclasses, the descriptions left out.
#[derive(Clone, Debug, PartialEq)]
pub enum TagValue {
    /// `@param` (a typeless one has no type)
    Param {
        ty: Option<Node>,
        by_ref: bool,
        variadic: bool,
        name: String,
    },
    /// `@param-immediately-invoked-callable`, `@param-later-invoked-callable`, `@pure-unless-callable-is-impure`,
    /// `@pure-unless-parameter-passed`
    ParamName(String),
    /// `@param-closure-this`, `@param-out`, `@property`
    TypedName(Node, String),
    Var(Node, String),
    /// `@return`, `@throws`, `@mixin`, `@require-extends`, `@require-implements`, `@sealed`, `@self-out`
    Type(Node),
    Deprecated,
    Method {
        is_static: bool,
        ret: Option<Node>,
        name: String,
        params: Vec<MethodParam>,
        templates: Vec<TemplateTag>,
    },
    Template(TemplateTag),
    /// `@extends`, `@implements`, `@use`
    Extends(Node),
    /// `@phpstan-type`: the type, or why it does not parse (`InvalidTypeNode`)
    TypeAlias(String, Result<Node, ParseError>),
    /// `@phpstan-import-type A from B as C`
    TypeAliasImport(String, String, Option<String>),
    Assert {
        ty: Node,
        parameter: String,
        /// `->prop` or `->method()`
        member: Option<(String, bool)>,
        negated: bool,
        equality: bool,
    },
    /// Any other tag: its text
    Generic(String),
    Doctrine,
    /// A value the tag's parser rejects: its text and the error
    Invalid(String, ParseError),
}

/// `PhpDocParser::parse()` on a doc comment: its tags. `None` where PHPStan's parser throws (an argument of a
/// `@\Doctrine` tag that does not parse).
pub fn tags(doc: &str) -> Option<Vec<Tag>> {
    let mut t = Tokens::new(doc);
    t.consume(Tok::OpenPhpdoc).ok()?;
    t.try_consume(Tok::Eol);
    let mut children: Vec<Option<Tag>> = Vec::new();
    if !t.is(Tok::ClosePhpdoc) {
        children.push(parse_child(&mut t).ok()?);
        while !t.is(Tok::ClosePhpdoc) {
            let free_form = matches!(
                children.last(),
                Some(Some(Tag {
                    value: TagValue::Doctrine | TagValue::Generic(_),
                    ..
                }))
            );
            if free_form {
                t.try_consume(Tok::Eol);
            } else if !t.try_consume(Tok::Eol) {
                break;
            }
            if t.is(Tok::ClosePhpdoc) {
                break;
            }
            children.push(parse_child(&mut t).ok()?);
        }
    }
    if let Err(e) = t.consume(Tok::ClosePhpdoc) {
        let name = match children.last() {
            Some(Some(tag)) => tag.name.clone(),
            _ => String::new(),
        };
        return Some(vec![Tag {
            name,
            line: t.line(),
            value: TagValue::Invalid(e.message(), e),
        }]);
    }
    Some(children.into_iter().flatten().collect())
}

/// `parseChild()`: a tag, or `None` for text.
fn parse_child(t: &mut Tokens) -> R<Option<Tag>> {
    let line = t.line();
    if t.is(Tok::PhpdocTag) {
        let name = t.value().to_string();
        t.next();
        let value = parse_tag_value(t, &name);
        return Ok(Some(Tag { name, line, value }));
    }
    if t.is(Tok::DoctrineTag) {
        let name = t.value().to_string();
        t.next();
        parse_doctrine_tag_value(t)?;
        return Ok(Some(Tag {
            name,
            line,
            value: TagValue::Doctrine,
        }));
    }
    parse_text(t);
    Ok(None)
}

/// `parseText()`: the text up to the next tag or the end, its lines joined with the doc's newline.
fn parse_text(t: &mut Tokens) -> String {
    let mut text = String::new();
    let mut savepoint = false;
    loop {
        let tmp =
            t.skipped_ws().to_string() + &t.join_until(&[Tok::Eol, Tok::ClosePhpdoc, Tok::End]);
        text.push_str(&tmp);
        if !t.is_any(&[Tok::Eol, Tok::ClosePhpdoc]) {
            break;
        }
        if !savepoint {
            t.push();
            savepoint = true;
        } else if !tmp.is_empty() {
            t.drop_save();
            t.push();
        }
        t.push();
        t.next();
        if t.is_any(&[Tok::PhpdocTag, Tok::DoctrineTag, Tok::ClosePhpdoc, Tok::End]) {
            t.rollback();
            break;
        }
        t.drop_save();
        text.push_str(t.newline.unwrap_or("\n"));
    }
    if savepoint {
        t.rollback();
        let newline: Vec<char> = t.newline.unwrap_or("\n").chars().collect();
        text.truncate(text.trim_end_matches(newline.as_slice()).len());
    }
    text.trim_matches([' ', '\t']).to_string()
}

fn parse_optional_description_after_doctrine_tag(t: &mut Tokens) -> R<String> {
    let mut text = String::new();
    let mut savepoint = false;
    let rest = |t: &mut Tokens, text: &str| {
        (text.to_string() + &parse_text(t))
            .trim_matches([' ', '\t'])
            .to_string()
    };
    loop {
        let tmp = t.skipped_ws().to_string()
            + &t.join_until(&[
                Tok::PhpdocTag,
                Tok::DoctrineTag,
                Tok::Eol,
                Tok::ClosePhpdoc,
                Tok::End,
            ]);
        text.push_str(&tmp);
        if !t.is_any(&[Tok::Eol, Tok::ClosePhpdoc]) {
            if !t.preceded_by_ws() {
                return Ok(rest(t, &text));
            }
            if t.is(Tok::PhpdocTag) {
                // parseChild() of a PHPDoc tag
                t.push();
                let name = t.value().to_string();
                t.next();
                let value = parse_tag_value(t, &name);
                if matches!(value, TagValue::Generic(_) | TagValue::Doctrine) {
                    t.rollback();
                    break;
                }
                if matches!(value, TagValue::Invalid(..)) {
                    t.rollback();
                    t.push();
                    t.next();
                    let paren = t.is(Tok::OpenParen);
                    t.rollback();
                    if paren {
                        break;
                    }
                    return Ok(rest(t, &text));
                }
                t.rollback();
                return Ok(rest(t, &text));
            }
            break;
        }
        if !savepoint {
            t.push();
            savepoint = true;
        } else if !tmp.is_empty() {
            t.drop_save();
            t.push();
        }
        t.push();
        t.next();
        if t.is_any(&[Tok::PhpdocTag, Tok::DoctrineTag, Tok::ClosePhpdoc, Tok::End]) {
            t.rollback();
            break;
        }
        t.drop_save();
        text.push_str(t.newline.unwrap_or("\n"));
    }
    if savepoint {
        t.rollback();
        let newline: Vec<char> = t.newline.unwrap_or("\n").chars().collect();
        text.truncate(text.trim_end_matches(newline.as_slice()).len());
    }
    Ok(text.trim_matches([' ', '\t']).to_string())
}

/// `parseTagValue()`: a value the tag's parser rejects becomes `Invalid` with the text from the value's start.
fn parse_tag_value(t: &mut Tokens, tag: &str) -> TagValue {
    t.push();
    match tag_value(t, tag) {
        Ok(value) => {
            t.drop_save();
            value
        }
        Err(e) => {
            t.rollback();
            TagValue::Invalid(parse_text(t), e)
        }
    }
}

fn tag_value(t: &mut Tokens, tag: &str) -> R<TagValue> {
    Ok(match tag {
        "@param" | "@phpstan-param" | "@psalm-param" | "@phan-param" => {
            let ty = if t.is_any(&[Tok::Reference, Tok::Variadic, Tok::Var]) {
                None
            } else {
                Some(parse_type(t)?)
            };
            let by_ref = t.try_consume(Tok::Reference);
            let variadic = t.try_consume(Tok::Variadic);
            let name = required_variable(t)?;
            parse_optional_description(t, false)?;
            TagValue::Param {
                ty,
                by_ref,
                variadic,
                name,
            }
        }
        "@param-immediately-invoked-callable"
        | "@phpstan-param-immediately-invoked-callable"
        | "@param-later-invoked-callable"
        | "@phpstan-param-later-invoked-callable"
        | "@pure-unless-callable-is-impure"
        | "@phpstan-pure-unless-callable-is-impure"
        | "@pure-unless-parameter-passed"
        | "@phpstan-pure-unless-parameter-passed" => {
            let name = required_variable(t)?;
            parse_optional_description(t, false)?;
            TagValue::ParamName(name)
        }
        "@param-closure-this"
        | "@phpstan-param-closure-this"
        | "@property"
        | "@property-read"
        | "@property-write"
        | "@phpstan-property"
        | "@phpstan-property-read"
        | "@phpstan-property-write"
        | "@psalm-property"
        | "@psalm-property-read"
        | "@psalm-property-write"
        | "@phan-property"
        | "@phan-property-read"
        | "@phan-property-write"
        | "@param-out"
        | "@phpstan-param-out"
        | "@psalm-param-out" => {
            let ty = parse_type(t)?;
            let name = required_variable(t)?;
            parse_optional_description(t, false)?;
            TagValue::TypedName(ty, name)
        }
        "@var" | "@phpstan-var" | "@psalm-var" | "@phan-var" => {
            let ty = parse_type(t)?;
            let name = if t.is(Tok::Var) {
                let name = t.value().to_string();
                t.next();
                name
            } else if t.try_consume(Tok::This) {
                "$this".to_string()
            } else {
                String::new()
            };
            parse_optional_description(t, name.is_empty())?;
            TagValue::Var(ty, name)
        }
        "@return"
        | "@phpstan-return"
        | "@psalm-return"
        | "@phan-return"
        | "@phan-real-return"
        | "@throws"
        | "@phpstan-throws"
        | "@mixin"
        | "@phan-mixin"
        | "@psalm-require-extends"
        | "@phpstan-require-extends"
        | "@psalm-require-implements"
        | "@phpstan-require-implements"
        | "@psalm-inheritors"
        | "@phpstan-sealed"
        | "@phpstan-this-out"
        | "@phpstan-self-out"
        | "@psalm-this-out"
        | "@psalm-self-out" => {
            let ty = parse_type(t)?;
            parse_optional_description(t, true)?;
            TagValue::Type(ty)
        }
        "@deprecated" => {
            parse_optional_description(t, false)?;
            TagValue::Deprecated
        }
        "@method" | "@phpstan-method" | "@psalm-method" | "@phan-method" => parse_method_tag(t)?,
        "@template"
        | "@phpstan-template"
        | "@psalm-template"
        | "@phan-template"
        | "@template-covariant"
        | "@phpstan-template-covariant"
        | "@psalm-template-covariant"
        | "@template-contravariant"
        | "@phpstan-template-contravariant"
        | "@psalm-template-contravariant" => TagValue::Template(parse_template(t, true)?),
        "@extends"
        | "@phpstan-extends"
        | "@phan-extends"
        | "@phan-inherits"
        | "@template-extends"
        | "@implements"
        | "@phpstan-implements"
        | "@template-implements"
        | "@use"
        | "@phpstan-use"
        | "@template-use" => {
            let base = t.value().to_string();
            t.consume(Tok::Ident)?;
            let ty = parse_generic(t, base)?;
            parse_optional_description(t, true)?;
            TagValue::Extends(ty)
        }
        "@phpstan-type" | "@psalm-type" | "@phan-type" => {
            let alias = t.value().to_string();
            t.consume(Tok::Ident)?;
            t.try_consume(Tok::Equal);
            let ty = parse_type(t).and_then(|ty| {
                if !t.is_any(&[Tok::ClosePhpdoc, Tok::Eol]) {
                    return Err(t.error(Tok::Eol, None));
                }
                Ok(ty)
            });
            if ty.is_err() {
                parse_optional_description(t, false)?;
            }
            TagValue::TypeAlias(alias, ty)
        }
        "@phpstan-import-type" | "@psalm-import-type" => {
            let alias = t.value().to_string();
            t.consume(Tok::Ident)?;
            t.consume_value(Tok::Ident, "from")?;
            let from = t.value().to_string();
            t.consume(Tok::Ident)?;
            let mut alias_as = None;
            if t.try_consume_value("as") {
                alias_as = Some(t.value().to_string());
                t.consume(Tok::Ident)?;
            }
            TagValue::TypeAliasImport(alias, from, alias_as)
        }
        "@phpstan-assert"
        | "@phpstan-assert-if-true"
        | "@phpstan-assert-if-false"
        | "@psalm-assert"
        | "@psalm-assert-if-true"
        | "@psalm-assert-if-false"
        | "@phan-assert"
        | "@phan-assert-if-true"
        | "@phan-assert-if-false" => {
            let negated = t.try_consume(Tok::Negated);
            let equality = t.try_consume(Tok::Equal);
            let ty = parse_type(t)?;
            let parameter = if t.try_consume(Tok::This) {
                "$this".to_string()
            } else {
                required_variable(t)?
            };
            let mut member = None;
            if t.try_consume(Tok::Arrow) {
                let name = t.value().to_string();
                t.consume(Tok::Ident)?;
                let method = t.try_consume(Tok::OpenParen);
                if method {
                    t.consume(Tok::CloseParen)?;
                }
                member = Some((name, method));
            }
            parse_optional_description(t, false)?;
            TagValue::Assert {
                ty,
                parameter,
                member,
                negated,
                equality,
            }
        }
        _ if t.is(Tok::OpenParen) => {
            parse_doctrine_tag_value(t)?;
            TagValue::Doctrine
        }
        _ => TagValue::Generic(parse_optional_description_after_doctrine_tag(t)?),
    })
}

fn required_variable(t: &mut Tokens) -> R<String> {
    let name = t.value().to_string();
    t.consume(Tok::Var)?;
    Ok(name)
}

/// `parseOptionalDescription()`; `limit`: right after a type, where `|`, `&` or no space is an error.
fn parse_optional_description(t: &mut Tokens, limit: bool) -> R<String> {
    if limit {
        if t.is_any(&[Tok::Union, Tok::Inter]) {
            t.consume(Tok::Other)?;
        }
        if !t.is_any(&[Tok::Eol, Tok::ClosePhpdoc, Tok::End]) && !t.preceded_by_ws() {
            t.consume(Tok::Ws)?;
        }
    }
    Ok(parse_text(t))
}

fn parse_method_tag(t: &mut Tokens) -> R<TagValue> {
    let first = parse_type(t)?;
    let (mut is_static, ret_or_name) = if first == Node::Ident("static".to_string()) {
        (true, parse_type(t)?)
    } else {
        (false, first.clone())
    };
    let (ret, name) = if t.is(Tok::Ident) {
        let name = t.value().to_string();
        t.next();
        (Some(ret_or_name), name)
    } else if let Node::Ident(name) = ret_or_name {
        let ret = is_static.then_some(first);
        is_static = false;
        (ret, name)
    } else {
        return Err(t.error(Tok::Ident, None));
    };
    let mut templates = Vec::new();
    if t.try_consume(Tok::OpenAngle) {
        loop {
            templates.push(parse_template(t, false)?);
            if !t.try_consume(Tok::Comma) {
                break;
            }
        }
        t.consume(Tok::CloseAngle)?;
    }
    let mut params = Vec::new();
    t.consume(Tok::OpenParen)?;
    if !t.is(Tok::CloseParen) {
        params.push(parse_method_param(t)?);
        while t.try_consume(Tok::Comma) {
            params.push(parse_method_param(t)?);
        }
    }
    t.consume(Tok::CloseParen)?;
    parse_optional_description(t, false)?;
    Ok(TagValue::Method {
        is_static,
        ret,
        name,
        params,
        templates,
    })
}

fn parse_method_param(t: &mut Tokens) -> R<MethodParam> {
    let ty = if t.is_any(&[Tok::Ident, Tok::OpenParen, Tok::Nullable]) {
        Some(parse_type(t)?)
    } else {
        None
    };
    let by_ref = t.try_consume(Tok::Reference);
    let variadic = t.try_consume(Tok::Variadic);
    let name = required_variable(t)?;
    let default = if t.try_consume(Tok::Equal) {
        Some(parse_const_expr(t, false)?)
    } else {
        None
    };
    Ok(MethodParam {
        ty,
        by_ref,
        variadic,
        name,
        default,
    })
}

/// `parseDoctrineTagValue()`: the arguments, then the description.
fn parse_doctrine_tag_value(t: &mut Tokens) -> R<()> {
    parse_doctrine_arguments(t, false)?;
    parse_optional_description_after_doctrine_tag(t)?;
    Ok(())
}

fn parse_doctrine_arguments(t: &mut Tokens, deep: bool) -> R<()> {
    if !t.is(Tok::OpenParen) {
        return Ok(());
    }
    if !deep {
        t.skip_eol = true;
    }
    let args = (|| {
        t.consume(Tok::OpenParen)?;
        loop {
            if t.is(Tok::CloseParen) {
                break;
            }
            parse_doctrine_argument(t)?;
            if !t.try_consume(Tok::Comma) {
                break;
            }
        }
        Ok(())
    })();
    if !deep {
        t.skip_eol = false;
    }
    args?;
    t.consume(Tok::CloseParen)
}

fn parse_doctrine_argument(t: &mut Tokens) -> R<()> {
    if !t.is(Tok::Ident) {
        return parse_doctrine_value(t);
    }
    t.push();
    let named = t
        .consume(Tok::Ident)
        .and_then(|()| t.consume(Tok::Equal))
        .and_then(|()| parse_doctrine_value(t));
    if named.is_ok() {
        t.drop_save();
        return Ok(());
    }
    t.rollback();
    parse_doctrine_value(t)
}

fn parse_doctrine_value(t: &mut Tokens) -> R<()> {
    if t.is_any(&[Tok::PhpdocTag, Tok::DoctrineTag]) {
        t.next();
        return parse_doctrine_arguments(t, true);
    }
    if t.try_consume(Tok::OpenCurly) {
        loop {
            if t.is(Tok::CloseCurly) {
                break;
            }
            parse_doctrine_array_item(t)?;
            if !t.try_consume(Tok::Comma) {
                break;
            }
        }
        return t.consume(Tok::CloseCurly);
    }
    t.push();
    if t.try_consume(Tok::Ident) {
        if !t.is(Tok::DoubleColon) {
            t.drop_save();
            return Ok(());
        }
        t.rollback();
    } else {
        t.drop_save();
    }
    let at = t.error(Tok::Ident, None);
    match parse_const_expr(t, true)? {
        ConstExpr::Array(_) => Err(at),
        _ => Ok(()),
    }
}

fn parse_doctrine_array_item(t: &mut Tokens) -> R<()> {
    t.push();
    let keyed = parse_doctrine_array_key(t).and_then(|()| {
        if !t.try_consume(Tok::Equal) && !t.try_consume(Tok::Colon) {
            t.consume(Tok::Equal)?;
        }
        parse_doctrine_value(t)
    });
    if keyed.is_ok() {
        t.drop_save();
        return Ok(());
    }
    t.rollback();
    parse_doctrine_value(t)
}

fn parse_doctrine_array_key(t: &mut Tokens) -> R<()> {
    match t.kind() {
        Tok::Int | Tok::SingleQuoted => t.next(),
        Tok::DoctrineString => {
            let value = t.value().to_string();
            doctrine_string(t, value);
            t.next();
        }
        Tok::DoubleQuoted => {
            let value = t.value().to_string();
            t.next();
            doctrine_string(t, value);
        }
        _ => {
            t.push();
            if !t.try_consume(Tok::Ident) {
                t.drop_save();
                return Err(t.error(Tok::Ident, None));
            }
            if !t.is(Tok::DoubleColon) {
                t.drop_save();
                return Ok(());
            }
            t.rollback();
            if !matches!(parse_const_expr(t, true)?, ConstExpr::Fetch(..)) {
                return Err(t.error(Tok::Ident, None));
            }
        }
    }
    Ok(())
}

/// `StringUnescaper::unescapeString()`.
fn unescape(quoted: &str) -> String {
    let inner = &quoted[1..quoted.len() - 1];
    if quoted.starts_with('\'') {
        return inner.replace("\\\\", "\\").replace("\\'", "'");
    }
    let s = inner.replace("\\\"", "\"");
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' && i + 1 < b.len() {
            let c = b[i + 1];
            let simple = match c {
                b'\\' => Some(b'\\'),
                b'n' => Some(b'\n'),
                b'r' => Some(b'\r'),
                b't' => Some(b'\t'),
                b'f' => Some(0x0c),
                b'v' => Some(0x0b),
                b'e' => Some(0x1b),
                _ => None,
            };
            if let Some(byte) = simple {
                out.push(byte);
                i += 2;
                continue;
            }
            let hex = |from: usize, max: usize| {
                b[from..]
                    .iter()
                    .take(max)
                    .take_while(|d| d.is_ascii_hexdigit())
                    .count()
            };
            let digits =
                |from: usize, len: usize| std::str::from_utf8(&b[from..from + len]).expect("ascii");
            if matches!(c, b'x' | b'X') && hex(i + 2, 2) > 0 {
                let len = hex(i + 2, 2);
                out.push(u8::from_str_radix(digits(i + 2, len), 16).expect("hex"));
                i += 2 + len;
                continue;
            }
            if c == b'u' && b.get(i + 2) == Some(&b'{') {
                let len = hex(i + 3, usize::MAX);
                if len > 0 && b.get(i + 3 + len) == Some(&b'}') {
                    let code = u32::from_str_radix(digits(i + 3, len), 16).unwrap_or(u32::MAX);
                    let mut buf = [0; 4];
                    out.extend_from_slice(
                        char::from_u32(code)
                            .unwrap_or('\u{fffd}')
                            .encode_utf8(&mut buf)
                            .as_bytes(),
                    );
                    i += 4 + len;
                    continue;
                }
            }
            let octal = b[i + 1..]
                .iter()
                .take(3)
                .take_while(|d| (b'0'..=b'7').contains(d))
                .count();
            if octal > 0 {
                out.push(u32::from_str_radix(digits(i + 1, octal), 8).expect("octal") as u8);
                i += 1 + octal;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}
