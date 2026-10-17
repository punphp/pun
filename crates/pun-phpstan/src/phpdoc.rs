//! PHPDoc type syntax: phpstan/phpdoc-parser 2.x's `Lexer` + `TypeParser` (+ `ConstExprParser`), the subset
//! PHPStan's `TypeStringResolver` uses on one-line type strings (no comments, no newlines).
//!
//! Pun's own extensions, used by the generated symbol data (scripts/export-phpstan-symbols.php) for types PHPStan's
//! `toPhpDocNode()` cannot write: `*ERROR*`, `*NEVER*`; `__template<...>` is an ordinary generic.

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tok {
    Ws,
    Ident,
    This,
    Var,
    Reference,
    Union,
    Inter,
    Nullable,
    OpenParen,
    CloseParen,
    OpenAngle,
    CloseAngle,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    Comma,
    Variadic,
    DoubleColon,
    Equal,
    Colon,
    Float,
    Int,
    SingleQuoted,
    DoubleQuoted,
    Wildcard,
    Other,
    End,
}

/// One alternative of the lexer's regex, tried in this order (PCRE alternation: first match wins, not longest).
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
    (Tok::OpenParen, r"\("),
    (Tok::CloseParen, r"\)"),
    (Tok::OpenAngle, "<"),
    (Tok::CloseAngle, ">"),
    (Tok::OpenSquare, r"\["),
    (Tok::CloseSquare, r"\]"),
    (Tok::OpenCurly, r"\{"),
    (Tok::CloseCurly, r"\}"),
    (Tok::Comma, ","),
    (Tok::Variadic, r"\.\.\."),
    (Tok::DoubleColon, "::"),
    (Tok::Equal, "="),
    (Tok::Colon, ":"),
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
    (Tok::Wildcard, r"\*"),
    (Tok::Other, r"(?:(?!\*/)[^\s])++"),
];

fn lex(input: &str) -> Vec<(Tok, String)> {
    use std::sync::OnceLock;
    static RE: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        let alternatives: Vec<String> = PATTERNS.iter().map(|(_, p)| format!("({p})")).collect();
        pcre2::bytes::RegexBuilder::new()
            .caseless(true)
            .build(&format!(r"\G(?:{})", alternatives.join("|")))
            .expect("phpdoc lexer regex")
    });
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut offset = 0;
    let mut locs = re.capture_locations();
    // The alternation ends with TOKEN_OTHER (any non-space run), so only stray whitespace other than tab/space,
    // which one-line type strings don't contain, would stop the loop early.
    while re
        .captures_read_at(&mut locs, bytes, offset)
        .ok()
        .flatten()
        .is_some()
    {
        let (start, end) = locs.get(0).expect("whole match");
        let group = (1..=PATTERNS.len())
            .find(|&g| locs.get(g).is_some())
            .expect("one alternative");
        tokens.push((PATTERNS[group - 1].0, input[start..end].to_string()));
        offset = end;
        if offset == bytes.len() {
            break;
        }
    }
    tokens.push((Tok::End, String::new()));
    tokens
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Ident(String),
    This,
    /// Pun's `*ERROR*` / `*NEVER*`.
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
    OffsetAccess(Box<Node>, Box<Node>),
    Generic {
        base: String,
        args: Vec<Node>,
    },
    Callable {
        ident: String,
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstExpr {
    Int(String),
    Float(String),
    Str(String),
    /// `Class::NAME`, `Class::PREFIX_*`
    Fetch(String, String),
}

/// Parse error: the input is not a type phpdoc-parser accepts.
#[derive(Debug, PartialEq)]
pub struct ParseError;

type R<T> = Result<T, ParseError>;

struct Tokens {
    tokens: Vec<(Tok, String)>,
    index: usize,
}

impl Tokens {
    fn new(input: &str) -> Self {
        let mut t = Tokens {
            tokens: lex(input),
            index: 0,
        };
        t.skip_ws();
        t
    }
    fn skip_ws(&mut self) {
        while self.tokens[self.index].0 == Tok::Ws && self.index + 1 < self.tokens.len() {
            self.index += 1;
        }
    }
    fn kind(&self) -> Tok {
        self.tokens[self.index].0
    }
    fn value(&self) -> &str {
        &self.tokens[self.index].1
    }
    fn is(&self, kind: Tok) -> bool {
        self.kind() == kind
    }
    fn preceded_by_ws(&self) -> bool {
        self.index > 0 && self.tokens[self.index - 1].0 == Tok::Ws
    }
    fn next(&mut self) {
        self.index = (self.index + 1).min(self.tokens.len() - 1);
        self.skip_ws();
    }
    fn consume(&mut self, kind: Tok) -> R<()> {
        if self.kind() != kind {
            return Err(ParseError);
        }
        self.next();
        Ok(())
    }
    fn try_consume(&mut self, kind: Tok) -> bool {
        self.consume(kind).is_ok()
    }
    fn try_consume_value(&mut self, value: &str) -> bool {
        if self.value() != value {
            return false;
        }
        self.next();
        true
    }
    fn take(&mut self) -> String {
        let v = self.value().to_string();
        self.next();
        v
    }
}

/// `TypeStringResolver::resolve()`'s parse step: one type, then the end of input.
pub fn parse(input: &str) -> R<Node> {
    let mut t = Tokens::new(input);
    let node = parse_type(&mut t)?;
    t.consume(Tok::End)?;
    Ok(node)
}

fn parse_type(t: &mut Tokens) -> R<Node> {
    if t.is(Tok::Nullable) {
        return parse_nullable(t);
    }
    let ty = parse_atomic(t)?;
    if t.is(Tok::Union) {
        return parse_list(t, ty, Tok::Union).map(Node::Union);
    }
    if t.is(Tok::Inter) {
        return parse_list(t, ty, Tok::Inter).map(Node::Inter);
    }
    Ok(ty)
}

fn parse_list(t: &mut Tokens, first: Node, sep: Tok) -> R<Vec<Node>> {
    let mut types = vec![first];
    while t.try_consume(sep) {
        types.push(parse_atomic(t)?);
    }
    Ok(types)
}

fn sub_parse(t: &mut Tokens) -> R<Node> {
    if t.is(Tok::Nullable) {
        return parse_nullable(t);
    }
    if t.is(Tok::Var) {
        let param = t.take();
        if !t.try_consume_value("is") {
            return Err(ParseError);
        }
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
    if t.value() == "is" {
        t.next();
        let (target, then, otherwise, negated) = parse_conditional_rest(t)?;
        return Ok(Node::Conditional {
            subject: Box::new(ty),
            target,
            then,
            otherwise,
            negated,
        });
    }
    if t.is(Tok::Union) {
        return parse_list(t, ty, Tok::Union).map(Node::Union);
    }
    if t.is(Tok::Inter) {
        return parse_list(t, ty, Tok::Inter).map(Node::Inter);
    }
    Ok(ty)
}

type Conditional = (Box<Node>, Box<Node>, Box<Node>, bool);

fn parse_conditional_rest(t: &mut Tokens) -> R<Conditional> {
    let negated = t.try_consume_value("not");
    let target = parse_type(t)?;
    t.consume(Tok::Nullable)?;
    let then = parse_type(t)?;
    t.consume(Tok::Colon)?;
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

fn parse_atomic(t: &mut Tokens) -> R<Node> {
    if t.try_consume(Tok::OpenParen) {
        let ty = sub_parse(t)?;
        t.consume(Tok::CloseParen)?;
        return Ok(array_or_offset(t, ty));
    }
    if t.try_consume(Tok::This) {
        return Ok(array_or_offset(t, Node::This));
    }
    if t.is(Tok::Wildcard) {
        let save = t.index;
        t.next();
        if t.is(Tok::Ident) && matches!(t.value(), "ERROR" | "NEVER") {
            let name = t.take();
            if t.try_consume(Tok::Wildcard) {
                return Ok(Node::Star(name));
            }
        }
        t.index = save;
        return Err(ParseError);
    }
    if t.is(Tok::Ident) {
        let save = t.index;
        let name = t.take();
        if !t.is(Tok::DoubleColon) {
            return parse_after_identifier(t, name);
        }
        t.index = save;
    }
    let constant = parse_const_expr(t)?;
    Ok(array_or_offset(t, Node::Const(constant)))
}

fn parse_after_identifier(t: &mut Tokens, name: String) -> R<Node> {
    if t.is(Tok::OpenAngle) {
        let generic = parse_generic(t, name)?;
        return Ok(array_or_offset(t, generic));
    }
    if t.is(Tok::OpenParen) {
        return Ok(try_parse_callable(t, name));
    }
    if t.is(Tok::OpenSquare) {
        return Ok(array_or_offset(t, Node::Ident(name)));
    }
    if matches!(
        name.as_str(),
        "array" | "list" | "non-empty-array" | "non-empty-list" | "object"
    ) && t.is(Tok::OpenCurly)
        && !t.preceded_by_ws()
    {
        let shape = if name == "object" {
            parse_object_shape(t)?
        } else {
            parse_array_shape(t, name)?
        };
        return Ok(array_or_offset(t, shape));
    }
    Ok(Node::Ident(name))
}

fn parse_generic(t: &mut Tokens, base: String) -> R<Node> {
    t.consume(Tok::OpenAngle)?;
    let mut args = Vec::new();
    loop {
        // `*`: a bivariant `mixed` argument
        let save = t.index;
        if t.try_consume(Tok::Wildcard) && (t.is(Tok::Comma) || t.is(Tok::CloseAngle)) {
            args.push(Node::Star("*".to_string()));
        } else {
            t.index = save;
            args.push(parse_type(t)?);
        }
        if !t.try_consume(Tok::Comma) {
            break;
        }
    }
    t.consume(Tok::CloseAngle)?;
    Ok(Node::Generic { base, args })
}

fn try_parse_callable(t: &mut Tokens, ident: String) -> Node {
    let save = t.index;
    match parse_callable(t, ident.clone()) {
        Ok(node) => node,
        Err(ParseError) => {
            t.index = save;
            Node::Ident(ident)
        }
    }
}

fn parse_callable(t: &mut Tokens, ident: String) -> R<Node> {
    t.consume(Tok::OpenParen)?;
    let mut params = Vec::new();
    if !t.is(Tok::CloseParen) {
        loop {
            params.push(parse_callable_param(t)?);
            if !t.try_consume(Tok::Comma) {
                break;
            }
        }
    }
    t.consume(Tok::CloseParen)?;
    t.consume(Tok::Colon)?;
    let ret = parse_callable_return(t)?;
    Ok(Node::Callable {
        ident,
        params,
        ret: Box::new(ret),
    })
}

fn parse_callable_param(t: &mut Tokens) -> R<CallableParam> {
    let ty = parse_type(t)?;
    let by_ref = t.try_consume(Tok::Reference);
    let variadic = t.try_consume(Tok::Variadic);
    let name = if t.is(Tok::Var) {
        t.take()
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
        return Ok(ty);
    }
    let name = t.value().to_string();
    t.consume(Tok::Ident)?;
    if t.is(Tok::OpenAngle) {
        return parse_generic(t, name);
    }
    if matches!(name.as_str(), "array" | "list") && t.is(Tok::OpenCurly) {
        return parse_array_shape(t, name);
    }
    Ok(Node::Ident(name))
}

fn array_or_offset(t: &mut Tokens, mut ty: Node) -> Node {
    while t.is(Tok::OpenSquare) {
        let save = t.index;
        t.next();
        if t.try_consume(Tok::CloseSquare) {
            ty = Node::Array(Box::new(ty));
            continue;
        }
        let offset = parse_type(t).and_then(|offset| t.consume(Tok::CloseSquare).map(|()| offset));
        match offset {
            Ok(offset) => ty = Node::OffsetAccess(Box::new(ty), Box::new(offset)),
            Err(ParseError) => {
                t.index = save;
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
    while !t.is(Tok::CloseCurly) {
        if t.try_consume(Tok::Variadic) {
            sealed = false;
            if t.try_consume(Tok::OpenAngle) {
                let mut value = parse_type(t)?;
                let mut key = None;
                if kind == "array" && t.try_consume(Tok::Comma) {
                    key = Some(Box::new(value));
                    value = parse_type(t)?;
                }
                t.consume(Tok::CloseAngle)?;
                unsealed = Some((key, Box::new(value)));
            }
            t.try_consume(Tok::Comma);
            break;
        }
        items.push(parse_shape_item(t)?);
        if !t.try_consume(Tok::Comma) {
            break;
        }
    }
    t.consume(Tok::CloseCurly)?;
    Ok(Node::ArrayShape {
        kind,
        items,
        sealed,
        unsealed,
    })
}

fn parse_shape_item(t: &mut Tokens) -> R<ShapeItem> {
    let save = t.index;
    let key = match t.kind() {
        Tok::Int => Some(ShapeKey::Int(t.take().replace('_', ""))),
        Tok::SingleQuoted | Tok::DoubleQuoted => Some(ShapeKey::Str(unescape(&t.take()))),
        Tok::Ident => Some(ShapeKey::Ident(t.take())),
        _ => None,
    };
    if let Some(key) = key {
        let optional = t.try_consume(Tok::Nullable);
        if t.try_consume(Tok::Colon) {
            return Ok(ShapeItem {
                key: Some(key),
                optional,
                value: parse_type(t)?,
            });
        }
    }
    t.index = save;
    Ok(ShapeItem {
        key: None,
        optional: false,
        value: parse_type(t)?,
    })
}

fn parse_object_shape(t: &mut Tokens) -> R<Node> {
    t.consume(Tok::OpenCurly)?;
    let mut items = Vec::new();
    loop {
        let key = match t.kind() {
            Tok::SingleQuoted | Tok::DoubleQuoted => unescape(&t.take()),
            _ => {
                let key = t.value().to_string();
                t.consume(Tok::Ident)?;
                key
            }
        };
        let optional = t.try_consume(Tok::Nullable);
        t.consume(Tok::Colon)?;
        items.push((key, optional, parse_type(t)?));
        if !t.try_consume(Tok::Comma) {
            break;
        }
    }
    t.consume(Tok::CloseCurly)?;
    Ok(Node::ObjectShape(items))
}

fn parse_const_expr(t: &mut Tokens) -> R<ConstExpr> {
    match t.kind() {
        Tok::Float => Ok(ConstExpr::Float(t.take().replace('_', ""))),
        Tok::Int => Ok(ConstExpr::Int(t.take().replace('_', ""))),
        Tok::SingleQuoted | Tok::DoubleQuoted => Ok(ConstExpr::Str(unescape(&t.take()))),
        Tok::Ident => {
            // Only reached for `Class::NAME`: a bare identifier is parsed as a type before this.
            let class = t.take();
            t.consume(Tok::DoubleColon)?;
            let mut name = String::new();
            let mut last = None;
            loop {
                if last != Some(Tok::Ident) && t.is(Tok::Ident) {
                    name.push_str(&t.take());
                    last = Some(Tok::Ident);
                } else if last != Some(Tok::Wildcard) && t.is(Tok::Wildcard) {
                    t.next();
                    name.push('*');
                    last = Some(Tok::Wildcard);
                    if t.preceded_by_ws() {
                        break;
                    }
                } else {
                    break;
                }
            }
            if last.is_none() {
                return Err(ParseError);
            }
            Ok(ConstExpr::Fetch(class, name))
        }
        _ => Err(ParseError),
    }
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
            let octal = b[i + 1..]
                .iter()
                .take(3)
                .take_while(|d| (b'0'..=b'7').contains(d))
                .count();
            if octal > 0 {
                let digits = std::str::from_utf8(&b[i + 1..i + 1 + octal]).expect("ascii");
                out.push(u32::from_str_radix(digits, 8).expect("octal") as u8);
                i += 1 + octal;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}
