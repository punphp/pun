//! Port of `PhpCsFixer\Fixer\ClassNotation\OrderedClassElementsFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::php_compat::{strcasecmp, strtolower};
use pun_tokens::token::CLASSY_KINDS;
use pun_tokens::{Kind, Proto, Tokens};
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
    ("property_public_abstract", Some(&["property_abstract", "property_public"])),
    ("property_public_readonly", Some(&["property_readonly", "property_public"])),
    ("property_protected_abstract", Some(&["property_abstract", "property_protected"])),
    ("property_protected_readonly", Some(&["property_readonly", "property_protected"])),
    ("property_private_readonly", Some(&["property_readonly", "property_private"])),
    ("property_public_static", Some(&["property_static", "property_public"])),
    ("property_protected_static", Some(&["property_static", "property_protected"])),
    ("property_private_static", Some(&["property_static", "property_private"])),
    ("method", None),
    ("method_abstract", Some(&["method"])),
    ("method_static", Some(&["method"])),
    ("method_public", Some(&["method", "public"])),
    ("method_protected", Some(&["method", "protected"])),
    ("method_private", Some(&["method", "private"])),
    ("method_public_abstract", Some(&["method_abstract", "method_public"])),
    ("method_protected_abstract", Some(&["method_abstract", "method_protected"])),
    ("method_private_abstract", Some(&["method_abstract", "method_private"])),
    ("method_public_abstract_static", Some(&["method_abstract", "method_static", "method_public"])),
    ("method_protected_abstract_static", Some(&["method_abstract", "method_static", "method_protected"])),
    ("method_private_abstract_static", Some(&["method_abstract", "method_static", "method_private"])),
    ("method_public_static", Some(&["method_static", "method_public"])),
    ("method_protected_static", Some(&["method_static", "method_protected"])),
    ("method_private_static", Some(&["method_static", "method_private"])),
];
const SPECIAL_TYPES: &[&str] = &["construct", "destruct", "magic", "phpunit"];
const DEFAULT_ORDER: &[&str] = &[
    "use_trait", "case", "constant_public", "constant_protected", "constant_private", "property_public", "property_protected",
    "property_private", "construct", "destruct", "magic", "phpunit", "method_public", "method_protected", "method_private",
];
const PHPUNIT: &[&str] = &[
    "setupbeforeclass", "dosetupbeforeclass", "teardownafterclass", "doteardownafterclass", "setup", "dosetup", "assertpreconditions",
    "assertpostconditions", "teardown", "doteardown",
];

struct OrderedClassElementsFixer {
    type_position: HashMap<String, i64>,
    sort_alpha: bool,
    case_sensitive: bool,
}

pub fn fixer() -> Box<dyn Fixer> {
    let mut f = OrderedClassElementsFixer { type_position: HashMap::new(), sort_alpha: false, case_sensitive: false };
    f.configure(&Value::Null).unwrap();
    Box::new(f)
}

#[derive(Clone, PartialEq, Debug)]
struct Element {
    start: usize,
    end: usize,
    visibility: &'static str,
    abstract_: bool,
    static_: bool,
    readonly: bool,
    type_: &'static str,
    name: Vec<u8>,
}

impl Fixer for OrderedClassElementsFixer {
    fn name(&self) -> &'static str {
        "ordered_class_elements"
    }
    fn priority(&self) -> i32 {
        65
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["order", "sort_algorithm", "case_sensitive"])?;
        let order = r.strings("order", DEFAULT_ORDER, &[])?;
        let ok = order.iter().any(|v| TYPE_HIERARCHY.iter().any(|(t, _)| t == v) || SPECIAL_TYPES.contains(&v.as_str()) || v.starts_with("method:"));
        if !ok {
            bail!("[ordered_class_elements] Invalid configuration: The option \"order\" with value array is invalid.");
        }
        self.sort_alpha = r.string("sort_algorithm", "none", &["none", "alpha"])? == "alpha";
        self.case_sensitive = r.bool("case_sensitive", false)?;

        // insertion-ordered map with nullable values, like upstream's array
        let mut pos: Vec<(String, Option<i64>)> = vec![];
        let set = |pos: &mut Vec<(String, Option<i64>)>, k: &str, v: Option<i64>| match pos.iter_mut().find(|(x, _)| x == k) {
            Some(e) => e.1 = v,
            None => pos.push((k.to_string(), v)),
        };
        let isset = |pos: &Vec<(String, Option<i64>)>, k: &str| pos.iter().find(|(x, _)| x == k).and_then(|e| e.1);
        for (i, t) in order.iter().enumerate() {
            set(&mut pos, t, Some(i as i64));
        }
        for (t, parents) in TYPE_HIERARCHY {
            if isset(&pos, t).is_some() {
                continue;
            }
            let v = parents.and_then(|ps| ps.iter().find_map(|p| isset(&pos, p)));
            set(&mut pos, t, v);
        }
        let last = order.len() as i64;
        self.type_position = pos.into_iter().map(|(k, v)| (k, v.unwrap_or(last) * 10)).collect();
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
            i = tokens.get_next_token_of_kind(i, &["{".into()], true).unwrap();
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

fn get_elements(tokens: &Tokens, start: usize) -> Vec<Element> {
    let mut start_index = start + 1;
    let mut elements = vec![];
    loop {
        let mut e = Element { start: start_index, end: 0, visibility: "public", abstract_: false, static_: false, readonly: false, type_: "", name: vec![] };
        let mut i = start_index;
        loop {
            let token = &tokens[i];
            if token.is_kind(Kind::T_ATTRIBUTE) {
                i = tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, i) + 1;
                continue;
            }
            if token.equals("}", true) {
                return elements;
            }
            if token.is_kind(Kind::T_ABSTRACT) {
                e.abstract_ = true;
                i += 1;
                continue;
            }
            if token.is_kind(Kind::T_STATIC) {
                e.static_ = true;
                i += 1;
                continue;
            }
            if token.is_kind(Kind::T_READONLY) {
                e.readonly = true;
            }
            if token.is_given_kind(&[Kind::T_PROTECTED, Kind::T_PRIVATE]) {
                e.visibility = if strtolower(&token.content) == b"protected" { "protected" } else { "private" };
                i += 1;
                continue;
            }
            if !token.is_given_kind(&[Kind::CT_USE_TRAIT, Kind::T_CASE, Kind::T_CONST, Kind::T_VARIABLE, Kind::T_FUNCTION]) {
                i += 1;
                continue;
            }
            let (ty, name) = detect_element_type(tokens, i);
            e.type_ = ty;
            if let Some(n) = name {
                e.name = n;
            }
            if ty == "property" {
                e.name = tokens[i].content.clone();
            } else if ty == "constant" {
                let eq = tokens.get_next_token_of_kind(i, &["=".into()], true).unwrap();
                e.name = tokens[tokens.get_prev_meaningful_token(eq).unwrap()].content.clone();
            } else if ["use_trait", "case", "method", "magic", "construct", "destruct"].contains(&ty) {
                e.name = tokens[tokens.get_next_meaningful_token(i).unwrap()].content.clone();
            }
            e.end = find_element_end(tokens, i);
            break;
        }
        start_index = e.end + 1;
        elements.push(e);
    }
}

fn detect_element_type(tokens: &Tokens, index: usize) -> (&'static str, Option<Vec<u8>>) {
    let token = &tokens[index];
    if token.is_kind(Kind::CT_USE_TRAIT) {
        return ("use_trait", None);
    }
    if token.is_kind(Kind::T_CASE) {
        return ("case", None);
    }
    if token.is_kind(Kind::T_CONST) {
        return ("constant", None);
    }
    if token.is_kind(Kind::T_VARIABLE) {
        return ("property", None);
    }
    let name = &tokens[tokens.get_next_meaningful_token(index).unwrap()];
    if name.equals((Kind::T_STRING, "__construct"), false) {
        return ("construct", None);
    }
    if name.equals((Kind::T_STRING, "__destruct"), false) {
        return ("destruct", None);
    }
    let phpunit: Vec<Proto> = [
        "setUpBeforeClass", "doSetUpBeforeClass", "tearDownAfterClass", "doTearDownAfterClass", "setUp", "doSetUp", "assertPreConditions",
        "assertPostConditions", "tearDown", "doTearDown",
    ]
    .iter()
    .map(|n| (Kind::T_STRING, *n).into())
    .collect();
    if name.equals_any(&phpunit, false) {
        return ("phpunit", Some(strtolower(&name.content)));
    }
    (if name.content.starts_with(b"__") { "magic" } else { "method" }, None)
}

fn find_element_end(tokens: &Tokens, index: usize) -> usize {
    let mut index = tokens.get_next_token_of_kind(index, &["(".into(), "{".into(), ";".into(), Kind::CT_PROPERTY_HOOK_BRACE_OPEN.into()], true).unwrap();
    if tokens[index].equals("(", true) {
        index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
        index = tokens.get_next_token_of_kind(index, &["{".into(), ";".into()], true).unwrap();
    }
    if tokens[index].equals("{", true) {
        index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index);
    }
    if tokens[index].is_kind(Kind::CT_PROPERTY_HOOK_BRACE_OPEN) {
        index = tokens.find_block_end(Tokens::BLOCK_TYPE_PROPERTY_HOOK, index);
    }
    index += 1;
    while tokens[index].is_whitespace(Some(b" \t")) || tokens[index].is_comment() {
        index += 1;
    }
    index -= 1;
    if tokens[index].is_whitespace(None) { index - 1 } else { index }
}

impl OrderedClassElementsFixer {
    fn sort_elements(&self, elements: &[Element]) -> Vec<Element> {
        let mut v: Vec<(i64, &Element)> = elements.iter().map(|e| (self.get_type_position(e), e)).collect();
        v.sort_by(|a, b| if a.0 == b.0 { self.sort_group_elements(a.1, b.1) } else { a.0.cmp(&b.0) });
        v.into_iter().map(|(_, e)| e.clone()).collect()
    }

    fn get_type_position(&self, e: &Element) -> i64 {
        let mut ty = e.type_.to_string();
        if ["method", "magic", "phpunit"].contains(&e.type_) {
            if let Some(&p) = self.type_position.get(&format!("method:{}", String::from_utf8_lossy(&e.name))) {
                return p;
            }
        }
        if SPECIAL_TYPES.contains(&e.type_) {
            if let Some(&p) = self.type_position.get(e.type_) {
                let mut position = p;
                if e.type_ == "phpunit" {
                    position += PHPUNIT.iter().position(|n| n.as_bytes() == e.name).unwrap() as i64 + 1;
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
        *self.type_position.get(&ty).unwrap_or_else(|| panic!("ordered_class_elements: no position for {ty}"))
    }

    fn sort_group_elements(&self, a: &Element, b: &Element) -> Ordering {
        if self.sort_alpha {
            return if self.case_sensitive { a.name.cmp(&b.name) } else { strcasecmp(&a.name, &b.name).cmp(&0) };
        }
        a.start.cmp(&b.start)
    }
}

fn sort_tokens(tokens: &mut Tokens, start_index: usize, end_index: usize, elements: &[Element]) {
    let mut replace = vec![];
    for e in elements {
        replace.extend_from_slice(&tokens.as_slice()[e.start..=e.end]);
    }
    tokens.override_range(start_index + 1, end_index, replace);
}
