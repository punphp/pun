//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitAssertNewNamesFixer`.
use crate::analyzers::abstract_php_unit_fixer::{PreviousAssertCalls, apply_fix, is_candidate};
use pun_format::Fixer;
use pun_tokens::Kind::T_STRING;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PhpUnitAssertNewNamesFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpUnitAssertNewNamesFixer)
}

const REPLACEMENTS: &[(&[u8], &str)] = &[
    (b"assertnotisreadable", "assertIsNotReadable"),
    (b"assertnotiswritable", "assertIsNotWritable"),
    (b"assertdirectorynotexists", "assertDirectoryDoesNotExist"),
    (b"assertfilenotexists", "assertFileDoesNotExist"),
    (
        b"assertdirectorynotisreadable",
        "assertDirectoryIsNotReadable",
    ),
    (
        b"assertdirectorynotiswritable",
        "assertDirectoryIsNotWriteable",
    ),
    (b"assertfilenotisreadable", "assertFileIsNotReadable"),
    (b"assertfilenotiswritable", "assertFileIsNotWriteable"),
    (b"assertregexp", "assertMatchesRegularExpression"),
    (b"assertnotregexp", "assertDoesNotMatchRegularExpression"),
];

impl Fixer for PhpUnitAssertNewNamesFixer {
    fn name(&self) -> &'static str {
        "php_unit_assert_new_names"
    }
    fn priority(&self) -> i32 {
        -10
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        apply_fix(tokens, |tokens, start, end| {
            let mut calls = PreviousAssertCalls::new(start, end);
            while let Some(call) = calls.next(tokens) {
                if let Some((_, replacement)) = REPLACEMENTS
                    .iter()
                    .find(|(name, _)| *name == call.lowered_name.as_slice())
                {
                    tokens.set(
                        call.index,
                        Token::new(T_STRING, replacement.as_bytes().to_vec()),
                    );
                }
            }
        });
    }
}
