//! Port of `PhpCsFixer\Fixer\ClassNotation\OrderedClassElementsFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::ordered_class_elements::{Element, PHPUNIT, get_elements, sort_tokens};
use anyhow::Result;
use pun_format::Fixer;
use pun_tokens::Tokens;
use pun_tokens::php_compat::strcasecmp;
use pun_tokens::token::CLASSY_KINDS;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;

const TYPE_HIERARCHY: &[(&str, Option<&[&str]>)] = &[
    ("use_trait", None),
    ("public", None),
    ("protected", None),
    ("private", None),
    ("case", Some(&["public"])),
    ("constant", None),
    ("constant_public", Some(&["constant", "public"])),
    ("constant_protected", Some(&["constant", "protected"])),
    ("constant_private", Some(&["constant", "private"])),
    ("property", None),
    ("property_static", Some(&["property"])),
    ("property_public", Some(&["property", "public"])),
    ("property_protected", Some(&["property", "protected"])),
    ("property_private", Some(&["property", "private"])),
    (
        "property_public_abstract",
        Some(&["property_abstract", "property_public"]),
    ),
    (
        "property_public_readonly",
        Some(&["property_readonly", "property_public"]),
    ),
    (
        "property_protected_abstract",
        Some(&["property_abstract", "property_protected"]),
    ),
    (
        "property_protected_readonly",
        Some(&["property_readonly", "property_protected"]),
    ),
    (
        "property_private_readonly",
        Some(&["property_readonly", "property_private"]),
    ),
    (
        "property_public_static",
        Some(&["property_static", "property_public"]),
    ),
    (
        "property_protected_static",
        Some(&["property_static", "property_protected"]),
    ),
    (
        "property_private_static",
        Some(&["property_static", "property_private"]),
    ),
    ("method", None),
    ("method_abstract", Some(&["method"])),
    ("method_static", Some(&["method"])),
    ("method_public", Some(&["method", "public"])),
    ("method_protected", Some(&["method", "protected"])),
    ("method_private", Some(&["method", "private"])),
    (
        "method_public_abstract",
        Some(&["method_abstract", "method_public"]),
    ),
    (
        "method_protected_abstract",
        Some(&["method_abstract", "method_protected"]),
    ),
    (
        "method_private_abstract",
        Some(&["method_abstract", "method_private"]),
    ),
    (
        "method_public_abstract_static",
        Some(&["method_abstract", "method_static", "method_public"]),
    ),
    (
        "method_protected_abstract_static",
        Some(&["method_abstract", "method_static", "method_protected"]),
    ),
    (
        "method_private_abstract_static",
        Some(&["method_abstract", "method_static", "method_private"]),
    ),
    (
        "method_public_static",
        Some(&["method_static", "method_public"]),
    ),
    (
        "method_protected_static",
        Some(&["method_static", "method_protected"]),
    ),
    (
        "method_private_static",
        Some(&["method_static", "method_private"]),
    ),
];
const SPECIAL_TYPES: &[&str] = &["construct", "destruct", "magic", "phpunit"];
const DEFAULT_ORDER: &[&str] = &[
    "use_trait",
    "case",
    "constant_public",
    "constant_protected",
    "constant_private",
    "property_public",
    "property_protected",
    "property_private",
    "construct",
    "destruct",
    "magic",
    "phpunit",
    "method_public",
    "method_protected",
    "method_private",
];
struct OrderedClassElementsFixer {
    type_position: HashMap<String, i64>,
    sort_alpha: bool,
    case_sensitive: bool,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(OrderedClassElementsFixer {
        type_position: HashMap::new(),
        sort_alpha: false,
        case_sensitive: false,
    })
}

impl Fixer for OrderedClassElementsFixer {
    fn name(&self) -> &'static str {
        "ordered_class_elements"
    }
    fn priority(&self) -> i32 {
        65
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(
            self.name(),
            options,
            &["order", "sort_algorithm", "case_sensitive"],
        )?;
        let order = r.strings("order", DEFAULT_ORDER, &[])?;
        let ok = order.iter().any(|v| {
            TYPE_HIERARCHY.iter().any(|(t, _)| t == v)
                || SPECIAL_TYPES.contains(&v.as_str())
                || v.starts_with("method:")
        });
        if !ok {
            return Err(r.invalid("order", r.get("order").unwrap(), &[]));
        }
        self.sort_alpha = r.string("sort_algorithm", "none", &["none", "alpha"])? == "alpha";
        self.case_sensitive = r.bool("case_sensitive", false)?;

        // upstream's array with nullable values (its order doesn't matter: only lookups follow)
        let mut pos: HashMap<String, Option<i64>> = HashMap::new();
        let isset = |pos: &HashMap<String, Option<i64>>, k: &str| pos.get(k).copied().flatten();
        for (i, t) in order.iter().enumerate() {
            pos.insert(t.clone(), Some(i as i64));
        }
        for (t, parents) in TYPE_HIERARCHY {
            if isset(&pos, t).is_some() {
                continue;
            }
            let v = parents.and_then(|ps| ps.iter().find_map(|p| isset(&pos, p)));
            pos.insert(t.to_string(), v);
        }
        let last = order.len() as i64;
        self.type_position = pos
            .into_iter()
            .map(|(k, v)| (k, v.unwrap_or(last) * 10))
            .collect();
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(CLASSY_KINDS)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let count = tokens.count();
        let mut i = 1;
        while i < count {
            if !tokens[i].is_classy() {
                i += 1;
                continue;
            }
            i = tokens
                .get_next_token_of_kind(i, &["{".into()], true)
                .unwrap();
            let elements = get_elements(tokens, i);
            if elements.is_empty() {
                i += 1;
                continue;
            }
            let end_index = elements.last().unwrap().end;
            let sorted = self.sort_elements(&elements);
            if sorted != elements {
                sort_tokens(tokens, i, end_index, &sorted);
            }
            i = end_index + 1;
        }
    }
}

impl OrderedClassElementsFixer {
    fn sort_elements(&self, elements: &[Element]) -> Vec<Element> {
        let mut v: Vec<(i64, &Element)> = elements
            .iter()
            .map(|e| (self.get_type_position(e), e))
            .collect();
        v.sort_by(|a, b| {
            if a.0 == b.0 {
                self.sort_group_elements(a.1, b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        v.into_iter().map(|(_, e)| e.clone()).collect()
    }

    fn get_type_position(&self, e: &Element) -> i64 {
        let mut ty = e.type_.to_string();
        if ["method", "magic", "phpunit"].contains(&e.type_)
            && let Some(&p) = self
                .type_position
                .get(&format!("method:{}", String::from_utf8_lossy(&e.name)))
        {
            return p;
        }
        if SPECIAL_TYPES.contains(&e.type_) {
            if let Some(&p) = self.type_position.get(e.type_) {
                let mut position = p;
                if e.type_ == "phpunit" {
                    position +=
                        PHPUNIT.iter().position(|n| n.as_bytes() == e.name).unwrap() as i64 + 1;
                }
                return position;
            }
            ty = "method".into();
        }
        if ["constant", "property", "method"].contains(&ty.as_str()) {
            ty = format!("{ty}_{}", e.visibility);
            if e.abstract_ {
                ty += "_abstract";
            }
            if e.static_ {
                ty += "_static";
            }
            if e.readonly {
                ty += "_readonly";
            }
        }
        *self
            .type_position
            .get(&ty)
            // A modifier mix PHP's compiler rejects (`public static readonly`): upstream's assert fails and the file
            // errors with its message.
            .unwrap_or_else(|| panic!("assert(isset($this->typePosition[$type]))"))
    }

    fn sort_group_elements(&self, a: &Element, b: &Element) -> Ordering {
        if self.sort_alpha {
            return if self.case_sensitive {
                a.name.cmp(&b.name)
            } else {
                strcasecmp(&a.name, &b.name).cmp(&0)
            };
        }
        a.start.cmp(&b.start)
    }
}

#[cfg(test)]
mod tests {
    // Pint 1.29.3 on a property PHP's compiler rejects (`public static readonly`): the file errors with this message.
    #[test]
    #[should_panic(expected = "assert(isset($this->typePosition[$type]))")]
    fn rejected_modifier_mix_errors_like_upstream() {
        let mut tokens = pun_tokens::Tokens::from_code(
            b"<?php\nclass A {\n    public function b() {}\n    public static readonly int $x;\n    public $a;\n}\n",
        );
        super::fixer().fix(std::path::Path::new("a.php"), &mut tokens);
    }
}
