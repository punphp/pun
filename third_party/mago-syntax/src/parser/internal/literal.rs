use mago_allocator::prelude::*;
use mago_database::file::HasFileId;
use ordered_float::OrderedFloat;

use mago_syntax_core::utils::parse_literal_float;
use mago_syntax_core::utils::parse_literal_integer;
use mago_syntax_core::utils::parse_literal_string_in;

use crate::T;
use crate::cst::cst::Keyword;
use crate::cst::cst::Literal;
use crate::cst::cst::LiteralFloat;
use crate::cst::cst::LiteralInteger;
use crate::cst::cst::LiteralString;
use crate::cst::cst::LiteralStringKind;
use crate::error::ParseError;
use crate::lexer::{OctalPrefix, octal_prefix};
use crate::parser::Parser;

impl<'arena, A> Parser<'_, 'arena, A>
where
    A: Arena,
{
    pub(crate) fn parse_literal(&mut self) -> Result<Literal<'arena>, ParseError> {
        let token = self.stream.consume()?;

        // Only the opt-in lexer emits these complete malformed legacy-octal tokens. Keep the PHP 5
        // prefix integer, or PHPParser Float_::parse's overflowing prefix, without rewriting raw bytes.
        if token.value.starts_with(b"0")
            && token.value.iter().any(|byte| matches!(byte, b'8' | b'9'))
            && token.value.iter().all(|byte| byte.is_ascii_digit() || *byte == b'_')
        {
            return Ok(match octal_prefix(token.value) {
                OctalPrefix::Integer(value) => Literal::Integer(LiteralInteger {
                    span: token.span_for(self.stream.file_id()),
                    raw: token.value,
                    value: Some(value),
                }),
                OctalPrefix::Float(value) => Literal::Float(LiteralFloat {
                    span: token.span_for(self.stream.file_id()),
                    raw: token.value,
                    value: OrderedFloat(value),
                }),
            });
        }

        Ok(match &token.kind {
            T![LiteralFloat] => Literal::Float(LiteralFloat {
                span: token.span_for(self.stream.file_id()),
                raw: token.value,
                // `parse_literal_float` only fails on lexer output the lexer would already have
                // rejected; substitute `0.0` defensively rather than panicking if that invariant
                // ever drifts.
                value: OrderedFloat(parse_literal_float(token.value).unwrap_or(0.0)),
            }),
            T![LiteralInteger] => {
                let value = parse_literal_integer(token.value);
                if value.is_some_and(|value| value > i64::MAX as u64) {
                    Literal::Float(LiteralFloat {
                        span: token.span_for(self.stream.file_id()),
                        raw: token.value,
                        value: OrderedFloat(overflowing_integer(token.value)),
                    })
                } else {
                    Literal::Integer(LiteralInteger {
                        span: token.span_for(self.stream.file_id()),
                        raw: token.value,
                        value,
                    })
                }
            }
            T!["true"] => Literal::True(Keyword { span: token.span_for(self.stream.file_id()), value: token.value }),
            T!["false"] => Literal::False(Keyword { span: token.span_for(self.stream.file_id()), value: token.value }),
            T!["null"] => Literal::Null(Keyword { span: token.span_for(self.stream.file_id()), value: token.value }),
            T![LiteralString] => Literal::String(LiteralString {
                kind: if token.value.starts_with(b"\"")
                    || token.value.starts_with(b"b\"")
                    || token.value.starts_with(b"B\"")
                {
                    LiteralStringKind::DoubleQuoted
                } else {
                    LiteralStringKind::SingleQuoted
                },
                span: token.span_for(self.stream.file_id()),
                raw: token.value,
                value: parse_literal_string_in(self.arena, token.value, None, true),
            }),
            T![PartialLiteralString] => {
                let kind = if token.value.starts_with(b"\"")
                    || token.value.starts_with(b"b\"")
                    || token.value.starts_with(b"B\"")
                {
                    LiteralStringKind::DoubleQuoted
                } else {
                    LiteralStringKind::SingleQuoted
                };

                return Err(ParseError::UnclosedLiteralString(kind, token.span_for(self.stream.file_id())));
            }
            _ => {
                return Err(self.stream.unexpected(
                    Some(token),
                    T!["true", "false", "null", LiteralFloat, LiteralInteger, LiteralString, PartialLiteralString],
                ));
            }
        })
    }
}

/// PHPParser Float_::parse: decimal conversion, or PHP's _php_math_basetozval for radix literals.
/// Called only for lexer-validated integers above INT_MAX; keep the source magnitude, not a clamped u64.
fn overflowing_integer(bytes: &[u8]) -> f64 {
    let (base, start) = match bytes {
        [b'0', b'x' | b'X', ..] => (16_u64, 2),
        [b'0', b'b' | b'B', ..] => (2, 2),
        [b'0', b'o' | b'O', ..] => (8, 2),
        [b'0', ..] => (8, 1),
        _ => return parse_literal_float(bytes).expect("valid decimal integer token"),
    };
    let mut integer = 0_u64;
    let mut float = 0.0;
    let mut floating = false;
    for &byte in &bytes[start..] {
        if byte == b'_' {
            continue;
        }
        let digit = u64::from(match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => byte - b'A' + 10,
        });
        if !floating {
            if integer <= (i64::MAX as u64 - digit) / base {
                integer = integer * base + digit;
                continue;
            }
            float = integer as f64;
            floating = true;
        }
        float = float * base as f64 + digit as f64;
    }
    float
}
