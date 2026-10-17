//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocNoAccessFixer` (proxy of general_phpdoc_annotation_remove).
use crate::analyzers::general_phpdoc_annotation_remove::GeneralPhpdocAnnotationRemove;
use pun_format::Fixer;
use pun_tokens::Tokens;
use std::path::Path;

struct PhpdocNoAccessFixer(GeneralPhpdocAnnotationRemove);

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocNoAccessFixer(GeneralPhpdocAnnotationRemove {
        annotations: &["access"],
    }))
}

impl Fixer for PhpdocNoAccessFixer {
    fn name(&self) -> &'static str {
        "phpdoc_no_access"
    }
    fn priority(&self) -> i32 {
        GeneralPhpdocAnnotationRemove::PRIORITY
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        self.0.is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        self.0.fix(tokens);
    }
}
