//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocTypesFixer`.
use crate::analyzers::abstract_phpdoc_types_fixer as base;
use crate::analyzers::docblock::TypeExpression;
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Tokens;
use pun_tokens::php_compat::strtolower;
use serde_json::Value;
use std::path::Path;

const POSSIBLE_TYPES: &[(&str, &[&str])] = &[
    ("alias", &["boolean", "double", "integer"]),
    (
        "meta",
        &[
            "$this", "false", "mixed", "parent", "resource", "scalar", "self", "static", "true",
            "void",
        ],
    ),
    (
        "simple",
        &[
            "array", "bool", "callable", "float", "int", "iterable", "null", "object", "string",
        ],
    ),
];

struct PhpdocTypesFixer {
    types_set_to_fix: Vec<&'static str>,
}

pub fn fixer() -> Box<dyn Fixer> {
    let mut f = PhpdocTypesFixer {
        types_set_to_fix: vec![],
    };
    f.configure(&Value::Null).unwrap();
    Box::new(f)
}

impl PhpdocTypesFixer {
    /// normalize() only lowercases a type whose lowercase form is in the set, so a docblock where every
    /// case-insensitive occurrence of those names is already lowercase comes out unchanged.
    fn may_change(&self, doc: &[u8]) -> bool {
        // Only an uppercase ASCII letter can start or sit inside such an occurrence.
        doc.iter()
            .enumerate()
            .filter(|(_, b)| b.is_ascii_uppercase())
            .any(|(i, _)| {
                self.types_set_to_fix.iter().any(|t| {
                    let t = t.as_bytes();
                    t.contains(&doc[i].to_ascii_lowercase())
                        && (i.saturating_sub(t.len() - 1)..=i).any(|s| {
                            doc.get(s..s + t.len())
                                .is_some_and(|w| w.eq_ignore_ascii_case(t))
                        })
                })
            })
    }
    fn normalize(&self, t: &[u8]) -> Vec<u8> {
        let e = TypeExpression::plain(t);
        e.map_types(&mut |t| {
            if t.is_union_type() {
                return t;
            }
            let lower = strtolower(t.to_string());
            if self.types_set_to_fix.iter().any(|x| x.as_bytes() == lower) {
                return TypeExpression::plain(&lower);
            }
            t
        })
        .to_string()
        .to_vec()
    }
}

impl Fixer for PhpdocTypesFixer {
    fn name(&self) -> &'static str {
        "phpdoc_types"
    }
    fn priority(&self) -> i32 {
        16
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["exclude", "groups"])?;
        let all: Vec<&str> = POSSIBLE_TYPES
            .iter()
            .flat_map(|(_, t)| t.iter().copied())
            .collect();
        let groups_all: Vec<&str> = POSSIBLE_TYPES.iter().map(|(g, _)| *g).collect();
        let exclude = r.strings("exclude", &[], &all)?;
        let groups = r.strings("groups", &groups_all, &groups_all)?;
        let mut set: Vec<&'static str> = vec![];
        for g in &groups {
            for t in POSSIBLE_TYPES.iter().find(|(n, _)| n == g).unwrap().1 {
                if !set.contains(t) {
                    set.push(t);
                }
            }
        }
        set.retain(|t| !exclude.iter().any(|e| e == t));
        self.types_set_to_fix = set;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        base::is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        base::apply_fix(
            tokens,
            &format!("phpdoc_types {:?}", self.types_set_to_fix),
            &|doc| self.may_change(doc),
            &|t| self.normalize(t),
        );
    }
}
