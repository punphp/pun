//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocArrayTypeFixer`.
use crate::analyzers::abstract_phpdoc_types_fixer as base;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Tokens;
use pun_tokens::php_compat::substr_count;
use std::path::Path;

struct PhpdocArrayTypeFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocArrayTypeFixer)
}

fn normalize(type_: &[u8]) -> Vec<u8> {
    if Preg::r#match(r#"/^\??\s*['"]/"#, type_) {
        return type_.to_vec();
    }
    let (prefix, type_) = match type_.strip_prefix(b"?") {
        Some(rest) => (&b"?"[..], rest),
        None => (&b""[..], type_),
    };
    let mut out = prefix.to_vec();
    out.extend(Preg::replace_callback(
        r"/^(.+?)((?:\h*\[\h*\])+)$/",
        |m| {
            let mut type_ = m.str(1);
            let level = substr_count(m.str(2), b"[");
            if type_.starts_with(b"(") && type_.ends_with(b")") {
                type_ = &type_[1..type_.len() - 1];
            }
            [b"array<".repeat(level), type_.to_vec(), b">".repeat(level)].concat()
        },
        type_,
    ));
    out
}

impl Fixer for PhpdocArrayTypeFixer {
    fn name(&self) -> &'static str {
        "phpdoc_array_type"
    }
    fn priority(&self) -> i32 {
        2
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        base::is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        base::apply_fix(tokens, "phpdoc_array_type", &|_| true, &normalize);
    }
}
