//! Port of `PhpCsFixer\Tokenizer\Token`.

use crate::kind::Kind::{self, *};

/// A token. `id == None` is upstream's non-array token (`'('`, `';'`, cleared `''`), whose
/// identity is its content. Content is bytes: PHP strings, possibly invalid UTF-8.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub id: Option<Kind>,
    pub content: Vec<u8>,
}

/// Upstream's "token prototype" for `equals`: `'('`, `[T_STRING]`, `[T_STRING, 'foo']`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proto<'a> {
    Char(&'a [u8]),
    Kind(Kind),
    KindContent(Kind, &'a [u8]),
}

impl<'a> From<&'a str> for Proto<'a> {
    fn from(s: &'a str) -> Self {
        Proto::Char(s.as_bytes())
    }
}
impl From<Kind> for Proto<'_> {
    fn from(k: Kind) -> Self {
        Proto::Kind(k)
    }
}
impl<'a> From<(Kind, &'a str)> for Proto<'a> {
    fn from((k, s): (Kind, &'a str)) -> Self {
        Proto::KindContent(k, s.as_bytes())
    }
}
impl<'a> From<&'a Token> for Proto<'a> {
    fn from(t: &'a Token) -> Self {
        match t.id {
            Some(k) => Proto::KindContent(k, &t.content),
            None => Proto::Char(&t.content),
        }
    }
}

impl Proto<'_> {
    /// `Tokens::extractTokenKind`: the key used by `isTokenKindFound`.
    pub fn found_key(&self) -> FoundKey<'_> {
        match *self {
            Proto::Char(c) => FoundKey::Content(c),
            Proto::Kind(k) | Proto::KindContent(k, _) => FoundKey::Kind(k),
        }
    }
}

/// A key of `Tokens::$foundTokenKinds`: a kind, or the content of a non-array token.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FoundKey<'a> {
    Kind(Kind),
    Content(&'a [u8]),
}
impl From<Kind> for FoundKey<'_> {
    fn from(k: Kind) -> Self {
        FoundKey::Kind(k)
    }
}
impl<'a> From<&'a str> for FoundKey<'a> {
    fn from(s: &'a str) -> Self {
        FoundKey::Content(s.as_bytes())
    }
}

pub const CAST_KINDS: &[Kind] =
    &[T_ARRAY_CAST, T_BOOL_CAST, T_DOUBLE_CAST, T_INT_CAST, T_OBJECT_CAST, T_STRING_CAST, T_UNSET_CAST, T_VOID_CAST];
pub const CLASSY_KINDS: &[Kind] = &[T_CLASS, T_TRAIT, T_INTERFACE, T_ENUM];
pub const OBJECT_OPERATOR_KINDS: &[Kind] = &[T_OBJECT_OPERATOR, T_NULLSAFE_OBJECT_OPERATOR];
pub const MAGIC_CONSTANTS: &[Kind] = &[T_CLASS_C, T_DIR, T_FILE, T_FUNC_C, T_LINE, T_METHOD_C, T_NS_C, T_TRAIT_C];
pub const KEYWORDS: &[Kind] = &[
    T_ABSTRACT, T_ARRAY, T_AS, T_BREAK, T_CALLABLE, T_CASE, T_CATCH, T_CLASS, T_CLONE, T_CONST, T_CONTINUE,
    T_DECLARE, T_DEFAULT, T_DO, T_ECHO, T_ELSE, T_ELSEIF, T_EMPTY, T_ENDDECLARE, T_ENDFOR, T_ENDFOREACH,
    T_ENDIF, T_ENDSWITCH, T_ENDWHILE, T_EVAL, T_EXIT, T_EXTENDS, T_FINAL, T_FINALLY, T_FN, T_FOR, T_FOREACH,
    T_FUNCTION, T_GLOBAL, T_GOTO, T_HALT_COMPILER, T_IF, T_IMPLEMENTS, T_INCLUDE, T_INCLUDE_ONCE, T_INSTANCEOF,
    T_INSTEADOF, T_INTERFACE, T_ISSET, T_LIST, T_LOGICAL_AND, T_LOGICAL_OR, T_LOGICAL_XOR, T_NAMESPACE, T_NEW,
    T_PRINT, T_PRIVATE, T_PROTECTED, T_PUBLIC, T_REQUIRE, T_REQUIRE_ONCE, T_RETURN, T_STATIC, T_SWITCH,
    T_THROW, T_TRAIT, T_TRY, T_UNSET, T_USE, T_VAR, T_WHILE, T_YIELD, T_YIELD_FROM, CT_ARRAY_TYPEHINT,
    CT_CLASS_CONSTANT, CT_CONST_IMPORT, CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
    CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED, CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC, CT_FUNCTION_IMPORT,
    CT_NAMESPACE_OPERATOR, CT_USE_LAMBDA, CT_USE_TRAIT, T_ENUM, T_MATCH, T_PRIVATE_SET, T_PROTECTED_SET,
    T_PUBLIC_SET, T_READONLY,
];

const DEFAULT_WHITESPACES: &[u8] = b" \t\n\r\0\x0B";

impl Token {
    /// `new Token([$id, $content])`.
    pub fn new(id: Kind, content: impl Into<Vec<u8>>) -> Token {
        let content = content.into();
        debug_assert!(!content.is_empty(), "Cannot set empty content for id-based Token.");
        Token { id: Some(id), content }
    }

    /// `new Token($content)` (non-array token).
    pub fn char(content: impl Into<Vec<u8>>) -> Token {
        Token { id: None, content: content.into() }
    }

    pub fn is_array(&self) -> bool {
        self.id.is_some()
    }

    pub fn get_id(&self) -> Option<Kind> {
        self.id
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    pub fn get_name(&self) -> Option<&'static str> {
        self.id.map(Kind::name)
    }

    fn is_amp(&self) -> bool {
        self.content == b"&"
            && matches!(self.id, None | Some(T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG | T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG))
    }

    /// `Token::equals($other, $caseSensitive)`.
    pub fn equals<'a>(&self, other: impl Into<Proto<'a>>, case_sensitive: bool) -> bool {
        // A `&` token only matches the `&` prototype (via isAmp); upstream compares it to Token instances only.
        match other.into() {
            Proto::Char(b"&") => self.is_amp(),
            Proto::Char(c) => self.id.is_none() && self.content == c,
            Proto::Kind(k) => self.id == Some(k),
            Proto::KindContent(k, c) => {
                self.id == Some(k)
                    && if case_sensitive { self.content == c } else { self.content.eq_ignore_ascii_case(c) }
            }
        }
    }

    /// `equals` with a Token instance (`$other instanceof self` branch).
    pub fn equals_token(&self, other: &Token, case_sensitive: bool) -> bool {
        if self.id.is_none() && self.content == b"&" {
            return other.is_amp();
        }
        match (self.id, other.id) {
            (None, None) => self.content == other.content,
            (Some(a), Some(b)) if a == b => {
                if case_sensitive { self.content == other.content } else { self.content.eq_ignore_ascii_case(&other.content) }
            }
            _ => false,
        }
    }

    pub fn equals_any(&self, others: &[Proto<'_>], case_sensitive: bool) -> bool {
        others.iter().any(|&o| self.equals(o, case_sensitive))
    }

    /// `isGivenKind` with one kind.
    pub fn is_kind(&self, kind: Kind) -> bool {
        self.id == Some(kind)
    }

    /// `isGivenKind` with a list of kinds.
    pub fn is_given_kind(&self, kinds: &[Kind]) -> bool {
        self.id.is_some_and(|id| kinds.contains(&id))
    }

    pub fn is_cast(&self) -> bool {
        self.is_given_kind(CAST_KINDS)
    }
    pub fn is_classy(&self) -> bool {
        self.is_given_kind(CLASSY_KINDS)
    }
    pub fn is_comment(&self) -> bool {
        self.is_given_kind(&[T_COMMENT, T_DOC_COMMENT])
    }
    pub fn is_object_operator(&self) -> bool {
        self.is_given_kind(OBJECT_OPERATOR_KINDS)
    }
    pub fn is_keyword(&self) -> bool {
        self.is_given_kind(KEYWORDS)
    }
    pub fn is_magic_constant(&self) -> bool {
        self.is_given_kind(MAGIC_CONSTANTS)
    }
    pub fn is_native_constant(&self) -> bool {
        self.is_array()
            && [&b"true"[..], b"false", b"null"].iter().any(|c| self.content.eq_ignore_ascii_case(c))
    }

    /// `isWhitespace($whitespaces)`; `None` = PHP's default `" \t\n\r\0\x0B"`. Note that an
    /// empty (cleared) token counts as whitespace, like upstream.
    pub fn is_whitespace(&self, whitespaces: Option<&[u8]>) -> bool {
        if self.id.is_some() && self.id != Some(T_WHITESPACE) {
            return false;
        }
        let ws = whitespaces.unwrap_or(DEFAULT_WHITESPACES);
        self.content.iter().all(|c| ws.contains(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals_semantics() {
        let s = Token::new(T_STRING, "Foo");
        assert!(s.equals(T_STRING, true));
        assert!(s.equals((T_STRING, "foo"), false));
        assert!(!s.equals((T_STRING, "foo"), true));
        assert!(!s.equals("Foo", true));
        assert!(Token::char("(").equals("(", true));
        let amp = Token::new(T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG, "&");
        assert!(amp.equals("&", true));
        assert!(Token::char("&").equals("&", true));
        assert!(Token::char("").is_whitespace(None));
        assert!(!Token::new(T_COMMENT, " ").is_whitespace(None));
    }
}
