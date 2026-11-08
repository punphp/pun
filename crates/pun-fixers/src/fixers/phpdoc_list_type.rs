//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocListTypeFixer`.
use crate::analyzers::abstract_phpdoc_types_fixer as base;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Tokens;
use std::path::Path;

struct PhpdocListTypeFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocListTypeFixer)
}

fn normalize(type_: &[u8]) -> Vec<u8> {
    Preg::replace(r"/\barray(?=<(?:[^,<]|<[^>]+>)+(>|{|\())/i", "list", type_)
}

impl Fixer for PhpdocListTypeFixer {
    fn name(&self) -> &'static str {
        "phpdoc_list_type"
    }
    fn priority(&self) -> i32 {
        1
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        base::is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        base::apply_fix(tokens, "phpdoc_list_type", &|_| true, &normalize);
    }
}
