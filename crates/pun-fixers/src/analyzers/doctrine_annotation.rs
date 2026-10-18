//! Ports of `PhpCsFixer\Doctrine\Annotation\{DocLexer, Token, Tokens}` and `PhpCsFixer\AbstractDoctrineAnnotationFixer`,
//! shared by the four doctrine_annotation_* fixers.
use crate::analyzers::fixer_configuration_resolver::Resolved;
use crate::analyzers::preg::{PREG_SPLIT_DELIM_CAPTURE, PREG_SPLIT_NO_EMPTY, Preg};
use anyhow::Result;
use pun_tokens::Kind::{self, *};
use pun_tokens::tokens_analyzer::{ClassyElement, TokensAnalyzer};
use pun_tokens::{Token, Tokens};
use std::collections::BTreeMap;

pub(crate) const T_NONE: u8 = 1;
pub(crate) const T_INTEGER: u8 = 2;
pub(crate) const T_STRING: u8 = 3;
pub(crate) const T_FLOAT: u8 = 4;
pub(crate) const T_IDENTIFIER: u8 = 100;
pub(crate) const T_AT: u8 = 101;
pub(crate) const T_CLOSE_CURLY_BRACES: u8 = 102;
pub(crate) const T_CLOSE_PARENTHESIS: u8 = 103;
pub(crate) const T_COMMA: u8 = 104;
pub(crate) const T_EQUALS: u8 = 105;
pub(crate) const T_NAMESPACE_SEPARATOR: u8 = 107;
pub(crate) const T_OPEN_CURLY_BRACES: u8 = 108;
pub(crate) const T_OPEN_PARENTHESIS: u8 = 109;
pub(crate) const T_COLON: u8 = 112;
pub(crate) const T_MINUS: u8 = 113;

/// `Doctrine\Annotation\Token`.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct DocToken {
    pub kind: u8,
    pub content: Vec<u8>,
    pub position: usize,
}

impl DocToken {
    pub(crate) fn new(kind: u8, content: impl Into<Vec<u8>>) -> Self {
        DocToken {
            kind,
            content: content.into(),
            position: 0,
        }
    }

    pub(crate) fn is_type(&self, types: &[u8]) -> bool {
        types.contains(&self.kind)
    }

    pub(crate) fn clear(&mut self) {
        self.content.clear();
    }
}

const NO_CASE: &[(&[u8], u8)] = &[
    (b"@", T_AT),
    (b",", T_COMMA),
    (b"(", T_OPEN_PARENTHESIS),
    (b")", T_CLOSE_PARENTHESIS),
    (b"{", T_OPEN_CURLY_BRACES),
    (b"}", T_CLOSE_CURLY_BRACES),
    (b"=", T_EQUALS),
    (b":", T_COLON),
    (b"-", T_MINUS),
    (b"\\", T_NAMESPACE_SEPARATOR),
];

const REGEX: &str = concat!(
    r#"/([a-z_\\][a-z0-9_\:\\]*[a-z_][a-z0-9_]*)|((?:[+-]?[0-9]+(?:[\.][0-9]+)*)(?:[eE][+-]?[0-9]+)?)|("(?:""|[^"])*+")"#,
    r"|\s+|\*+|(.)/iu",
);

/// `DocLexer::getType`, which also unquotes a string's value.
fn get_type(value: &mut Vec<u8>) -> u8 {
    if value[0] == b'"' {
        // `substr($value, 1, strlen($value) - 2)`: a lone `"` (matched by `(.)`) gives an empty string.
        let inner = value.get(1..value.len() - 1).unwrap_or_default();
        *value = pun_tokens::php_compat::str_replace(b"\"\"", b"\"", inner);
        return T_STRING;
    }
    if let Some(&(_, kind)) = NO_CASE.iter().find(|(c, _)| *c == &value[..]) {
        return kind;
    }
    if value[0] == b'_' || value[0] == b'\\' || value[0].is_ascii_alphabetic() {
        return T_IDENTIFIER;
    }
    // `is_numeric`: the number pattern without a second `.` part.
    if Preg::r#match(
        r"/^[+-]?[0-9]+(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?$/D",
        &value[..],
    ) {
        return if value.contains(&b'.') || value.iter().any(|c| c.eq_ignore_ascii_case(&b'e')) {
            T_FLOAT
        } else {
            T_INTEGER
        };
    }
    T_NONE
}

/// `DocLexer::setInput` + `scan`: every token of `input`, whitespace and `*` runs dropped.
fn scan(input: &[u8]) -> Vec<DocToken> {
    Preg::split(
        REGEX,
        input,
        -1,
        PREG_SPLIT_NO_EMPTY | PREG_SPLIT_DELIM_CAPTURE,
    )
    .into_iter()
    .map(|piece| {
        let mut content = piece.text;
        let kind = get_type(&mut content);
        DocToken {
            kind,
            content,
            position: piece.offset,
        }
    })
    .collect()
}

/// `Doctrine\Annotation\Tokens`.
#[derive(Debug, Default)]
pub(crate) struct DocTokens(pub Vec<DocToken>);

impl DocTokens {
    /// `Tokens::createFromDocComment`.
    pub(crate) fn from_doc_comment(content: &[u8], ignored_tags: &[String]) -> Self {
        let mut tokens = vec![];
        let mut ignored_text_position = 0;
        let mut current_position = 0;
        while let Some(next_at) = pun_tokens::php_compat::strpos(content, b"@", current_position) {
            if next_at != 0 && !pun_tokens::php_compat::isspace(content[next_at - 1]) {
                current_position = next_at + 1;
                continue;
            }
            let scanned_all = scan(&content[next_at..]);
            let mut scanned: Vec<&DocToken> = vec![];
            let mut use_count = 0;
            let mut scopes = 0;
            // The scan starts at the `@`, so its first token is always T_AT (upstream checks it).
            for (index, token) in scanned_all.iter().enumerate() {
                if index == 1 {
                    if token.kind != T_IDENTIFIER
                        || ignored_tags
                            .iter()
                            .any(|t| t.as_bytes() == &token.content[..])
                    {
                        break;
                    }
                    use_count = 2;
                }
                if index >= 2 && scopes == 0 && !token.is_type(&[T_NONE, T_OPEN_PARENTHESIS]) {
                    break;
                }
                scanned.push(token);
                if token.kind == T_OPEN_PARENTHESIS {
                    scopes += 1;
                } else if token.kind == T_CLOSE_PARENTHESIS {
                    scopes -= 1;
                    if scopes == 0 {
                        use_count = scanned.len();
                        break;
                    }
                }
            }
            if scopes != 0 {
                break;
            }
            if use_count == 0 {
                current_position = next_at + 1;
                continue;
            }
            if next_at > ignored_text_position {
                tokens.push(DocToken::new(
                    T_NONE,
                    &content[ignored_text_position..next_at],
                ));
            }
            let mut last_token_end = 0;
            let mut end = 0;
            for scanned in &scanned[..use_count] {
                let token_content = if scanned.kind == T_STRING {
                    [
                        &b"\""[..],
                        &pun_tokens::php_compat::str_replace(b"\"", b"\"\"", &scanned.content),
                        b"\"",
                    ]
                    .concat()
                } else {
                    scanned.content.clone()
                };
                if scanned.position > last_token_end {
                    tokens.push(DocToken::new(
                        T_NONE,
                        &content[next_at + last_token_end..next_at + scanned.position],
                    ));
                }
                last_token_end = scanned.position + token_content.len();
                end = last_token_end;
                tokens.push(DocToken::new(scanned.kind, token_content));
            }
            current_position = next_at + end;
            ignored_text_position = current_position;
        }
        if ignored_text_position < content.len() {
            tokens.push(DocToken::new(T_NONE, &content[ignored_text_position..]));
        }
        DocTokens(tokens)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get_next_meaningful_token(&self, mut index: usize) -> Option<usize> {
        loop {
            index += 1;
            if self.0.get(index)?.kind != T_NONE {
                return Some(index);
            }
        }
    }

    /// `getAnnotationEnd`: the index of the `)` closing the annotation at `index`, or the tag's name.
    ///
    /// Upstream returns null when the parentheses never close; `from_doc_comment` keeps only balanced annotations and
    /// the docblock's closing `*/` always follows the last one, so the end always exists here.
    pub(crate) fn get_annotation_end(&self, index: usize) -> usize {
        let t = &self.0;
        let token = &t[index + 2];
        let start = if token.kind == T_OPEN_PARENTHESIS {
            index + 2
        } else if token.kind == T_NONE
            && t.get(index + 3).is_some_and(|t| t.kind == T_OPEN_PARENTHESIS)
            && Preg::r#match(r"/^(\R\s*\*\s*)*\s*$/", &token.content)
        {
            index + 3
        } else {
            return index + 1;
        };
        let mut level = 0;
        (start..t.len())
            .find(|&i| {
                match t[i].kind {
                    T_OPEN_PARENTHESIS => level += 1,
                    T_CLOSE_PARENTHESIS => level -= 1,
                    _ => {}
                }
                level == 0
            })
            .unwrap()
    }

    pub(crate) fn insert_at(&mut self, index: usize, token: DocToken) {
        self.0.insert(index, token);
    }

    pub(crate) fn get_code(&self) -> Vec<u8> {
        self.0
            .iter()
            .flat_map(|t| t.content.iter().copied())
            .collect()
    }
}

impl std::ops::Index<usize> for DocTokens {
    type Output = DocToken;
    fn index(&self, i: usize) -> &DocToken {
        &self.0[i]
    }
}

impl std::ops::IndexMut<usize> for DocTokens {
    fn index_mut(&mut self, i: usize) -> &mut DocToken {
        &mut self.0[i]
    }
}

/// `AbstractDoctrineAnnotationFixer`'s `ignored_tags` default.
const IGNORED_TAGS: &[&str] = &[
    // PHPDocumentor 1
    "abstract",
    "access",
    "code",
    "deprec",
    "encode",
    "exception",
    "final",
    "ingroup",
    "inheritdoc",
    "inheritDoc",
    "magic",
    "name",
    "toc",
    "tutorial",
    "private",
    "static",
    "staticvar",
    "staticVar",
    "throw",
    // PHPDocumentor 2
    "api",
    "author",
    "category",
    "copyright",
    "deprecated",
    "example",
    "filesource",
    "global",
    "ignore",
    "internal",
    "license",
    "link",
    "method",
    "package",
    "param",
    "property",
    "property-read",
    "property-write",
    "return",
    "see",
    "since",
    "source",
    "subpackage",
    "throws",
    "todo",
    "TODO",
    "usedBy",
    "uses",
    "var",
    "version",
    // PHPUnit
    "after",
    "afterClass",
    "backupGlobals",
    "backupStaticAttributes",
    "before",
    "beforeClass",
    "codeCoverageIgnore",
    "codeCoverageIgnoreStart",
    "codeCoverageIgnoreEnd",
    "covers",
    "coversDefaultClass",
    "coversNothing",
    "dataProvider",
    "depends",
    "expectedException",
    "expectedExceptionCode",
    "expectedExceptionMessage",
    "expectedExceptionMessageRegExp",
    "group",
    "large",
    "medium",
    "preserveGlobalState",
    "requires",
    "runTestsInSeparateProcesses",
    "runInSeparateProcess",
    "small",
    "test",
    "testdox",
    "ticket",
    "uses",
    // PHPCheckStyle
    "SuppressWarnings",
    // PHPStorm
    "noinspection",
    // PEAR
    "package_version",
    // PlantUML
    "enduml",
    "startuml",
    // Psalm
    "psalm",
    // PHPStan
    "phpstan",
    "template",
    // other
    "fix",
    "FIXME",
    "fixme",
    "override",
];

/// The `ignored_tags` option every doctrine fixer shares.
pub(crate) fn ignored_tags(o: &Resolved) -> Result<Vec<String>> {
    o.strings("ignored_tags", IGNORED_TAGS, &[])
}

const CLASS_MODIFIERS: &[Kind] = &[T_ABSTRACT, T_FINAL, T_READONLY];
const MODIFIER_KINDS: &[Kind] = &[
    T_PUBLIC,
    T_PROTECTED,
    T_PRIVATE,
    T_FINAL,
    T_ABSTRACT,
    T_NS_SEPARATOR,
    Kind::T_STRING,
    CT_NULLABLE_TYPE,
    T_READONLY,
    T_PRIVATE_SET,
    T_PROTECTED_SET,
    T_PUBLIC_SET,
];

/// `AbstractDoctrineAnnotationFixer::applyFix`: `fix_annotations` on the annotations of every docblock in front of a
/// class or a class member.
pub(crate) fn apply_fix(
    tokens: &mut Tokens,
    ignored_tags: &[String],
    fix_annotations: impl Fn(&mut DocTokens),
) {
    // fetch indices one time, this is safe as we never add or remove a token during fixing
    let elements = TokensAnalyzer::new(tokens).get_classy_elements();
    for index in tokens.find_given_kind(T_DOC_COMMENT, 0, None) {
        if !next_element_accepts_doctrine_annotations(tokens, index, &elements) {
            continue;
        }
        let mut doc_tokens = DocTokens::from_doc_comment(tokens[index].get_content(), ignored_tags);
        fix_annotations(&mut doc_tokens);
        tokens.set(index, Token::new(T_DOC_COMMENT, doc_tokens.get_code()));
    }
}

fn next_element_accepts_doctrine_annotations(
    tokens: &Tokens,
    mut index: usize,
    elements: &BTreeMap<usize, ClassyElement>,
) -> bool {
    loop {
        let Some(next) = tokens.get_next_meaningful_token(index) else {
            return false;
        };
        index = next;
        if !tokens[index].is_given_kind(CLASS_MODIFIERS) {
            break;
        }
    }
    if tokens[index].is_kind(T_CLASS) {
        return true;
    }
    while tokens[index].is_given_kind(MODIFIER_KINDS) {
        index = tokens.get_next_meaningful_token(index).unwrap();
    }
    // interface, enums and traits cannot have doctrine annotations
    elements
        .get(&index)
        .is_some_and(|e| tokens[e.class_index].is_kind(T_CLASS))
}
