//! Token boundaries follow PHP's INITIAL scanner (token_get_all), not executable-file shebang skipping.
//! Expected boundaries are derived from the literal prefixes/openers; source-fidelity checks reject masking.

use std::borrow::Cow;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_database::file::{File, FileId};
use mago_span::HasSpan;
use mago_syntax::cst::{InlineKind, Statement};
use mago_syntax::lexer::Lexer;
use mago_syntax::parser::{parse_file_borrowed_with_settings, parse_file_content_with_settings};
use mago_syntax::settings::{LexerSettings, ParserSettings};
use mago_syntax::token::TokenKind;
use mago_syntax_core::input::Input;
use pun_lint::{Linter, rules};

#[test]
fn shebang_lexer_stops_at_enabled_openers_and_retains_original_bytes_and_spans() {
    use TokenKind::{EchoTag, InlineShebang, InlineText, OpenTag, ShortOpenTag};
    // PHP token_get_all exposes these inline-prefix/opener boundaries. Mago keeps a genuine LF shebang as
    // InlineShebang and separates opening-tag whitespace; neither convention may swallow an opening tag.
    for (source, short_tags, prefix, inline, opener, tag) in [
        ("#!php<?php 1;", true, "#!php", InlineText, "<?php", OpenTag),
        (
            "#!php<?php 1;",
            false,
            "#!php",
            InlineText,
            "<?php",
            OpenTag,
        ),
        (
            "#!php<?PHP\t1;",
            false,
            "#!php",
            InlineText,
            "<?PHP",
            OpenTag,
        ),
        ("#!php<?= 1;", true, "#!php", InlineText, "<?=", EchoTag),
        ("#!php<?= 1;", false, "#!php", InlineText, "<?=", EchoTag),
        ("#!php<? 1;", true, "#!php", InlineText, "<?", ShortOpenTag),
        (
            "#!php<?ignored<?php 1;",
            false,
            "#!php<?ignored",
            InlineText,
            "<?php",
            OpenTag,
        ),
        (
            "#!php<?ignored<?= 1;",
            false,
            "#!php<?ignored",
            InlineText,
            "<?=",
            EchoTag,
        ),
        (
            "#!php<?ignored\n<?php 1;",
            false,
            "#!php<?ignored\n",
            InlineShebang,
            "<?php",
            OpenTag,
        ),
        (
            "#!php\n<?php 1;",
            true,
            "#!php\n",
            InlineShebang,
            "<?php",
            OpenTag,
        ),
        (
            "#!php\r\n<?php 1;",
            false,
            "#!php\r\n",
            InlineShebang,
            "<?php",
            OpenTag,
        ),
        (
            "#!php\r<?php 1;",
            false,
            "#!php\r",
            InlineText,
            "<?php",
            OpenTag,
        ),
        (
            " #!php<?php 1;",
            false,
            " #!php",
            InlineText,
            "<?php",
            OpenTag,
        ),
        (
            "/* lead */#!php<?php 1;",
            true,
            "/* lead */#!php",
            InlineText,
            "<?php",
            OpenTag,
        ),
        ("#!php<?php", false, "#!php", InlineText, "<?php", OpenTag),
    ] {
        let file = FileId::new(b"shebang.php");
        let mut lexer = Lexer::new(
            Input::new(file, source.as_bytes()),
            LexerSettings {
                enable_short_tags: short_tags,
            },
        );
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.advance())
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(
            (tokens[0].kind, tokens[0].value),
            (inline, prefix.as_bytes()),
            "{source:?}"
        );
        assert_eq!(
            (tokens[1].kind, tokens[1].value),
            (tag, opener.as_bytes()),
            "{source:?}"
        );
        assert_eq!(tokens[1].start.offset as usize, prefix.len());
        let mut offset = 0;
        for token in tokens {
            let span = token.span_for(file);
            assert_eq!(span.file_id, file);
            assert_eq!(span.start.offset as usize, offset);
            offset += token.value.len();
            assert_eq!(span.end.offset as usize, offset);
            assert_eq!(
                token.value,
                &source.as_bytes()[span.start.offset as usize..offset]
            );
        }
        assert_eq!(offset, source.len());
    }
}

#[test]
fn shebang_without_an_enabled_opener_keeps_inline_text_and_lf_classification() {
    for (source, short_tags, kind) in [
        ("#!php", true, InlineKind::Text),
        ("#!php\n", true, InlineKind::Shebang),
        ("#!php\r\n", false, InlineKind::Shebang),
        ("#!php\r", false, InlineKind::Text),
        ("#!php<?ignored", false, InlineKind::Text),
        ("#!php<?ignored\n", false, InlineKind::Shebang),
        (" #!php\n", false, InlineKind::Text),
    ] {
        let arena = LocalArena::new();
        let program = parse_file_content_with_settings(
            &arena,
            FileId::zero(),
            source.as_bytes(),
            ParserSettings {
                lexer: LexerSettings {
                    enable_short_tags: short_tags,
                },
                ..Default::default()
            },
        );
        assert!(!program.has_errors(), "{source:?}: {:?}", program.errors);
        let [Statement::Inline(inline)] = program.statements.as_slice() else {
            panic!("{source:?}: {:?}", program.statements);
        };
        assert_eq!(inline.kind, kind, "{source:?}");
        assert_eq!(inline.value, source.as_bytes());
        assert_eq!(
            (inline.span.start.offset, inline.span.end.offset),
            (0, source.len() as u32)
        );
    }
}

#[test]
fn shebang_owned_and_borrowed_csts_restore_declarations_without_rewriting_source() {
    for prefix in [
        "#!php<?php ",
        "#!php<?PHP\t",
        "#!php\r<?php ",
        "#!php\n<?php ",
        "#!php\r\n<?php ",
        "#!php<?php\n",
    ] {
        for short_tags in [false, true] {
            let source = format!("{prefix}class Restored {{}} function restored() {{}}");
            let file = File::ephemeral(
                Cow::Borrowed(b"shebang.php"),
                Cow::Owned(source.as_bytes().to_vec()),
            );
            let arena = LocalArena::new();
            let settings = ParserSettings {
                lexer: LexerSettings {
                    enable_short_tags: short_tags,
                },
                ..Default::default()
            };
            let owned = parse_file_content_with_settings(&arena, file.id, &file.contents, settings);
            let borrowed = parse_file_borrowed_with_settings(&arena, &file, settings);
            for program in [owned, borrowed] {
                assert!(!program.has_errors(), "{source:?}: {:?}", program.errors);
                assert_eq!(program.source_text, source.as_bytes());
                let [
                    Statement::Inline(inline),
                    Statement::OpeningTag(_),
                    Statement::Class(class),
                    Statement::Function(function),
                ] = program.statements.as_slice()
                else {
                    panic!("{source:?}: {:?}", program.statements);
                };
                assert_eq!(inline.value, &source.as_bytes()[..prefix.len() - 6]);
                assert_eq!(class.name.value, b"Restored");
                assert_eq!(class.span().start.offset as usize, prefix.len());
                assert_eq!(
                    &source.as_bytes()
                        [class.span().start.offset as usize..class.span().end.offset as usize],
                    b"class Restored {}"
                );
                assert_eq!(function.name.value, b"restored");
                assert_eq!(
                    &source.as_bytes()[function.span().start.offset as usize
                        ..function.span().end.offset as usize],
                    b"function restored() {}"
                );
            }
        }
    }
}

#[test]
fn shebang_lint_fixes_keep_inline_bytes_before_the_restored_php() {
    let linter = Linter::new(vec![Box::new(rules::PreferCompoundAssignment)], 80400);
    for prefix in [
        "#!php<?php ",
        "#!php<?php\n",
        "#!php\r<?php ",
        "#!php\n<?php ",
        "#!php\r\n<?php ",
    ] {
        // The arithmetic rewrite is derived by hand. Only the PHP expression changes, never its inline prefix.
        let source = format!("{prefix}$a = $a + 1;");
        let fixed = linter.fix(Path::new("shebang.php"), &source).unwrap();
        assert_eq!(fixed.source, format!("{prefix}$a += 1;"));
        assert_eq!(fixed.applied.len(), 1);
        assert!(fixed.remaining.is_empty());
    }
}

#[test]
fn shebang_lint_native_facts_reparse_the_same_class_and_variable_return() {
    let linter = Linter::new(vec![Box::new(rules::AddReturnTypeFromNew)], 80400);
    for prefix in [
        "#!php<?php ",
        "#!php\r<?php ",
        "#!php\n<?php ",
        "#!php\r\n<?php ",
    ] {
        // ReturnTypeFromReturnNewRector's variable-return contract: this variable holds the declared class.
        let source = format!(
            "{prefix}class Restored {{}} function create() {{ $value = new Restored; return $value; }}"
        );
        let fixed = linter
            .fix(Path::new("shebang-native.php"), &source)
            .unwrap();
        assert_eq!(
            fixed.source,
            format!(
                "{prefix}class Restored {{}} function create(): \\Restored {{ $value = new Restored; return $value; }}"
            )
        );
        assert_eq!(fixed.applied.len(), 1);
        assert!(fixed.remaining.is_empty());
    }
}
