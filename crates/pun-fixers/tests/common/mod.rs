//! Shared by the fixture and oracle-case tests.
use pun_format::Fixer;
use std::path::Path;

/// AbstractFixerTestCase::doTest on a configured fixer: `input` (when there is one) must be a candidate, fix to
/// `expected` and mark the tokens changed; `expected` must stay as it is, unmarked. Returns the failed checks and the
/// code to show for them (the fixed input, or the changed expected code when there is no input).
pub(crate) fn do_test(
    fixer: &dyn Fixer,
    file: &Path,
    expected: &str,
    input: Option<&str>,
) -> (Vec<&'static str>, Vec<u8>) {
    let supported = fixer.supports(file);
    let mut problems = Vec::new();
    let mut got_text = Vec::new();
    if let Some(input) = input {
        let mut tokens = pun_tokens::Tokens::from_code(input.as_bytes());
        if supported {
            if !fixer.is_candidate(&tokens) {
                problems.push("not a candidate for input");
            }
            fixer.fix(file, &mut tokens);
        }
        let got = tokens.generate_code();
        if got != expected.as_bytes() {
            problems.push("input did not fix to expected");
        }
        if !tokens.is_changed() {
            problems.push("tokens not marked changed");
        }
        got_text = got;
    }
    let mut tokens = pun_tokens::Tokens::from_code(expected.as_bytes());
    // AbstractFixer::fix: applyFix only for non-empty, candidate, supported input.
    if supported && tokens.count() > 0 && fixer.is_candidate(&tokens) {
        fixer.fix(file, &mut tokens);
    }
    let got = tokens.generate_code();
    if got != expected.as_bytes() {
        problems.push("expected code changed");
        if input.is_none() {
            got_text = got;
        }
    } else if tokens.is_changed() {
        problems.push("expected tokens marked changed");
    }
    (problems, got_text)
}
