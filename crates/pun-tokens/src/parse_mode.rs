//! `token_get_all($code, TOKEN_PARSE)` differs from plain lexing only in that the parser
//! retags semi-reserved keywords it reduces as `identifier` (and `readonly` used as a
//! function name) to T_STRING (zend_lex_tstring feedback). PHP-CS-Fixer tokenizes with
//! TOKEN_PARSE, so `Tokens::from_code` applies this.
//!
//! ponytail: context heuristic over neighbouring tokens instead of a parser; covers every
//! grammar rule that calls zend_lex_tstring, verified by the `parse` parity mode. Code that
//! PHP would reject with a ParseError is not detected.

use crate::kind::Kind::{self, *};
use crate::lexer::RawToken;

fn is_member_modifier(k: Kind) -> bool {
    matches!(
        k,
        T_PUBLIC | T_PROTECTED | T_PRIVATE | T_STATIC | T_ABSTRACT | T_FINAL | T_READONLY
    )
}

pub fn is_semi_reserved(k: Kind) -> bool {
    is_member_modifier(k)
        || matches!(
            k,
            T_INCLUDE
                | T_INCLUDE_ONCE
                | T_EVAL
                | T_REQUIRE
                | T_REQUIRE_ONCE
                | T_LOGICAL_OR
                | T_LOGICAL_XOR
                | T_LOGICAL_AND
                | T_INSTANCEOF
                | T_NEW
                | T_CLONE
                | T_EXIT
                | T_IF
                | T_ELSEIF
                | T_ELSE
                | T_ENDIF
                | T_ECHO
                | T_DO
                | T_WHILE
                | T_ENDWHILE
                | T_FOR
                | T_ENDFOR
                | T_FOREACH
                | T_ENDFOREACH
                | T_DECLARE
                | T_ENDDECLARE
                | T_AS
                | T_TRY
                | T_CATCH
                | T_FINALLY
                | T_THROW
                | T_USE
                | T_INSTEADOF
                | T_GLOBAL
                | T_VAR
                | T_UNSET
                | T_ISSET
                | T_EMPTY
                | T_CONTINUE
                | T_GOTO
                | T_FUNCTION
                | T_CONST
                | T_RETURN
                | T_PRINT
                | T_YIELD
                | T_LIST
                | T_SWITCH
                | T_ENDSWITCH
                | T_CASE
                | T_DEFAULT
                | T_BREAK
                | T_ARRAY
                | T_CALLABLE
                | T_EXTENDS
                | T_IMPLEMENTS
                | T_NAMESPACE
                | T_TRAIT
                | T_INTERFACE
                | T_CLASS
                | T_CLASS_C
                | T_TRAIT_C
                | T_FUNC_C
                | T_METHOD_C
                | T_LINE
                | T_FILE
                | T_DIR
                | T_NS_C
                | T_FN
                | T_MATCH
                | T_ENUM
                | T_PROPERTY_C
        )
}

pub fn apply_parse_mode(tokens: &mut [RawToken<'_>]) {
    let meaningful: Vec<usize> = (0..tokens.len())
        .filter(|&i| {
            !matches!(
                tokens[i].kind,
                Some(T_WHITESPACE | T_COMMENT | T_DOC_COMMENT)
            )
        })
        .collect();
    let kind = |m: Option<&usize>| m.and_then(|&i| tokens[i].kind);
    let is_char = |m: Option<&usize>, c: &[u8]| {
        m.is_some_and(|&i| tokens[i].kind.is_none() && tokens[i].text == c)
    };
    let is_amp = |m: Option<&usize>| {
        matches!(
            kind(m),
            Some(T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG | T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG)
        )
    };
    let mut demote = Vec::new();
    for (j, &i) in meaningful.iter().enumerate() {
        let Some(k) = tokens[i].kind else { continue };
        if !is_semi_reserved(k) {
            continue;
        }
        let prev = j.checked_sub(1).and_then(|p| meaningful.get(p));
        let prev2 = j.checked_sub(2).and_then(|p| meaningful.get(p));
        let next = meaningful.get(j + 1);
        let pk = kind(prev);
        let yes = pk == Some(T_DOUBLE_COLON)
            || pk == Some(T_FUNCTION)
            || (is_amp(prev) && kind(prev2) == Some(T_FUNCTION))
            || pk == Some(T_NAMESPACE)
            // class constant / enum case name: `const LIST = 1`, `case Default = 1`
            || is_char(next, b"=")
            || (pk == Some(T_CASE) && is_char(next, b";"))
            // named argument: `foo(default: 1)`
            || (is_char(next, b":") && (is_char(prev, b"(") || is_char(prev, b",")))
            // trait alias: `foo as list;`, `foo as public match;`
            || (pk == Some(T_AS)
                && !is_member_modifier(k)
                && !(matches!(k, T_LIST | T_ARRAY) && is_char(next, b"(")))
            || (pk.is_some_and(is_member_modifier) && kind(prev2) == Some(T_AS))
            // trait method reference: `use T { list as foo; }`
            || (kind(next) == Some(T_AS) && (is_char(prev, b"{") || is_char(prev, b";")))
            // `readonly(...)` function call
            || (k == T_READONLY
                && is_char(next, b"(")
                && !pk.is_some_and(|p| is_member_modifier(p) || matches!(p, T_VAR | T_PUBLIC_SET | T_PROTECTED_SET | T_PRIVATE_SET))
                && !(is_char(prev, b"{") || is_char(prev, b";") || is_char(prev, b"}") || is_char(prev, b"(") || is_char(prev, b",")));
        if yes {
            demote.push(i);
        }
    }
    for i in demote {
        tokens[i].kind = Some(T_STRING);
    }
}
