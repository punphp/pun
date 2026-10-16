//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocScalarFixer`.
use crate::analyzers::abstract_phpdoc_types_fixer as base;
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Tokens;
use serde_json::Value;
use std::path::Path;

const TYPES_MAP: &[(&str, &str)] = &[
    ("boolean", "bool"),
    ("callback", "callable"),
    ("double", "float"),
    ("integer", "int"),
    ("never-return", "never"),
    ("never-returns", "never"),
    ("no-return", "never"),
    ("real", "float"),
    ("str", "string"),
];
const DEFAULT_TYPES: &[&str] = &["boolean", "callback", "double", "integer", "real", "str"];

struct PhpdocScalarFixer {
    types: Vec<String>,
}

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocScalarFixer { types: DEFAULT_TYPES.iter().map(|s| s.to_string()).collect() })
}

impl PhpdocScalarFixer {
    fn normalize(&self, t: &[u8]) -> Vec<u8> {
        let mut t = t;
        let mut suffix = Vec::new();
        while t.ends_with(b"[]") {
            t = &t[..t.len() - 2];
            suffix.extend_from_slice(b"[]");
        }
        let mut out = t.to_vec();
        if self.types.iter().any(|x| x.as_bytes() == t) {
            out = TYPES_MAP.iter().find(|(k, _)| k.as_bytes() == t).unwrap().1.as_bytes().to_vec();
        }
        out.extend(suffix);
        out
    }
}

impl Fixer for PhpdocScalarFixer {
    fn name(&self) -> &'static str {
        "phpdoc_scalar"
    }
    fn priority(&self) -> i32 {
        15
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["types"])?;
        let allowed: Vec<&str> = TYPES_MAP.iter().map(|(k, _)| *k).collect();
        self.types = r.strings("types", DEFAULT_TYPES, &allowed)?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        base::is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        base::apply_fix(tokens, &format!("phpdoc_scalar {:?}", self.types), &|t| self.normalize(t));
    }
}
