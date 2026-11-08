//! PHPParser/token_get_all captures distinguish permissive analysis tokenization from native lint admission.

use std::borrow::Cow;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_database::file::{File, FileId};
use mago_span::HasSpan;
use mago_syntax::cst::{LiteralFloat, LiteralInteger};
use mago_syntax::lexer::Lexer;
use mago_syntax::parser::{parse_file_borrowed_with_settings, parse_file_with_settings};
use mago_syntax::settings::{LexerSettings, ParserSettings};
use mago_syntax::token::TokenKind;
use mago_syntax::walker::MutWalker;
use mago_syntax_core::input::Input;
use serde_json::{Value, json};

fn inputs() -> Value {
    serde_json::from_str(include_str!(
        "../../pun-analyse/tests/fixtures/parser/numeric-admission-inputs.json"
    ))
    .unwrap()
}

fn expected() -> Value {
    serde_json::from_str(include_str!(
        "../../pun-analyse/tests/fixtures/parser/numeric-admission-expected.json"
    ))
    .unwrap()
}

#[test]
fn retained_numeric_tokens_match_native_classes_without_changing_bytes_or_spans() {
    let inputs = inputs();
    let expected = expected();
    for name in [
        "legacy",
        "multiple",
        "overflow-prefix",
        "overflow-wide",
        "overflow-edge",
        "separated-prefix",
        "offset-bare",
        "offset-braced",
    ] {
        let source = inputs[name].as_str().unwrap().as_bytes();
        let file = FileId::zero();
        let mut lexer = Lexer::new(
            Input::new(file, source),
            LexerSettings {
                retain_invalid_legacy_octal: true,
                ..Default::default()
            },
        );
        let mut numeric = Vec::new();
        let mut offset = 0;
        while let Some(token) = lexer.advance() {
            let token = token.unwrap();
            let span = token.span_for(file);
            assert_eq!(span.start.offset as usize, offset, "{name}");
            offset += token.value.len();
            assert_eq!(span.end.offset as usize, offset);
            assert_eq!(token.value, &source[span.start.offset as usize..offset]);
            let kind = match token.kind {
                TokenKind::LiteralInteger => "T_LNUMBER",
                TokenKind::LiteralFloat => "T_DNUMBER",
                TokenKind::OffsetNumber => "T_NUM_STRING",
                _ => continue,
            };
            let line = 1 + source[..span.start.offset as usize]
                .iter()
                .filter(|b| **b == b'\n')
                .count();
            numeric.push(json!([
                kind,
                std::str::from_utf8(token.value).unwrap(),
                line
            ]));
        }
        assert_eq!(offset, source.len());
        assert_eq!(json!(numeric), expected["tokens"][name], "{name}");
    }
}

#[derive(Default)]
struct Numbers(Vec<Value>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Numbers {
    fn walk_out_literal_integer(&mut self, node: &'ast LiteralInteger<'arena>, (): &mut ()) {
        self.0.push(json!({"kind":"Scalar_Int", "raw":std::str::from_utf8(node.raw).unwrap(),
            "value":node.value.unwrap(), "start":node.span.start.offset, "end":node.span.end.offset}));
    }
    fn walk_out_literal_float(&mut self, node: &'ast LiteralFloat<'arena>, (): &mut ()) {
        self.0.push(
            json!({"kind":"Scalar_Float", "raw":std::str::from_utf8(node.raw).unwrap(),
            "value":*node.value, "start":node.span.start.offset, "end":node.span.end.offset}),
        );
    }
}

#[test]
fn owned_and_borrowed_csts_keep_php_prefix_values_and_complete_raw_spans() {
    let inputs = inputs();
    let expected = expected();
    for name in [
        "legacy",
        "valid",
        "overflow-prefix",
        "overflow-wide",
        "overflow-edge",
        "separated-prefix",
        "offset-bare",
    ] {
        let source = inputs[name].as_str().unwrap().as_bytes();
        let file = File::ephemeral(Cow::Borrowed(b"numeric.php"), Cow::Owned(source.to_vec()));
        let arena = LocalArena::new();
        let settings = ParserSettings {
            lexer: LexerSettings {
                retain_invalid_legacy_octal: true,
                ..Default::default()
            },
            ..Default::default()
        };
        for program in [
            parse_file_with_settings(&arena, &file, settings),
            parse_file_borrowed_with_settings(&arena, &file, settings),
        ] {
            assert!(!program.has_errors(), "{name}: {:?}", program.errors);
            assert_eq!(program.source_text, source);
            let mut numbers = Numbers::default();
            numbers.walk_program(program, &mut ());
            // PHP 5's permitted invalid octals expose the prefix values, not parseLNumber's PHP 7 dummy.
            let mut captured = expected["literals"]["50600"][name].clone();
            for number in captured.as_array_mut().unwrap() {
                // json_encode serializes integral PHP floats without a fractional part.
                if number["kind"] == "Scalar_Float" {
                    number["value"] = json!(number["value"].as_f64().unwrap());
                }
            }
            assert_eq!(json!(numbers.0), captured, "{name}");
            for number in numbers.0 {
                let start = number["start"].as_u64().unwrap() as usize;
                let end = number["end"].as_u64().unwrap() as usize;
                assert_eq!(
                    number["raw"].as_str().unwrap().as_bytes(),
                    &source[start..end]
                );
            }
        }
    }
}

#[test]
fn strict_lexer_linter_and_formatter_still_reject_native_invalid_octals() {
    let linter = pun_lint::Linter::new(Vec::new(), 80500);
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/numeric-admission.json")).unwrap();
    for literal in ["08", "078", "0_8", "07_9", "010000000000000000000008"] {
        let source = format!("<?php $a={literal};");
        let mut lexer = Lexer::new(
            Input::new(FileId::zero(), source.as_bytes()),
            LexerSettings::default(),
        );
        let failure = std::iter::from_fn(|| lexer.advance())
            .find_map(Result::err)
            .unwrap();
        let bad = literal.find(['8', '9']).unwrap();
        assert_eq!(failure.span().start.offset as usize, 9 + bad);
        assert_eq!(
            failure.to_string(),
            format!(
                "Unexpected token `{}` (0x{:02X})",
                literal.as_bytes()[bad] as char,
                literal.as_bytes()[bad]
            )
        );
        assert!(matches!(
            linter.check(Path::new("numeric.php"), &source),
            Err(pun_lint::LintError::Syntax { .. })
        ));
        assert_eq!(
            pun_format::lint(source.as_bytes()).unwrap_err().message,
            expected[literal].as_str().unwrap().as_bytes()
        );
    }
    for literal in [
        "077",
        "07_7",
        "0o77",
        "08.0",
        "08e2",
        "01000000000000000000000",
    ] {
        let source = format!("<?php $a={literal};");
        assert_eq!(
            linter.check(Path::new("numeric.php"), &source).unwrap(),
            [] as [pun_problem::Problem; 0]
        );
        assert!(pun_format::lint(source.as_bytes()).is_ok());
    }
}

#[test]
fn numeric_retention_does_not_enable_disabled_short_tags_or_recover_other_lexical_errors() {
    for short_tags in [false, true] {
        let mut lexer = Lexer::new(
            Input::new(FileId::zero(), b"<? $a=08;"),
            LexerSettings {
                enable_short_tags: short_tags,
                retain_invalid_legacy_octal: true,
            },
        );
        let kinds: Vec<_> = std::iter::from_fn(|| lexer.advance())
            .map(|t| t.unwrap().kind)
            .collect();
        if short_tags {
            assert!(kinds.contains(&TokenKind::LiteralInteger));
        } else {
            assert_eq!(kinds, [TokenKind::InlineText]);
        }
    }
    let mut lexer = Lexer::new(
        Input::new(FileId::zero(), b"<?php 08;\0"),
        LexerSettings {
            retain_invalid_legacy_octal: true,
            ..Default::default()
        },
    );
    let failure = std::iter::from_fn(|| lexer.advance())
        .find_map(Result::err)
        .unwrap();
    assert_eq!(failure.span().start.offset, 9);
}
