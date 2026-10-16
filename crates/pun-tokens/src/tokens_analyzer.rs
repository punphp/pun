//! Port of `PhpCsFixer\Tokenizer\TokensAnalyzer` (subset; see docs/tokens.md).

use std::collections::BTreeMap;

use crate::kind::Kind::{self, *};
use crate::token::Proto;
use crate::tokens::Tokens;

fn c(s: &'static str) -> Proto<'static> {
    Proto::Char(s.as_bytes())
}
fn k(kind: Kind) -> Proto<'static> {
    Proto::Kind(kind)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassyElementType {
    Property,
    Method,
    PromotedProperty,
    Const,
    TraitImport,
    Case,
}

impl ClassyElementType {
    /// Upstream's `'type'` string.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Property => "property",
            Self::Method => "method",
            Self::PromotedProperty => "promoted_property",
            Self::Const => "const",
            Self::TraitImport => "trait_import",
            Self::Case => "case",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassyElement {
    pub class_index: usize,
    pub kind: ClassyElementType,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClassyModifiers {
    pub final_: Option<usize>,
    pub abstract_: Option<usize>,
    pub readonly: Option<usize>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MethodAttributes {
    /// T_PUBLIC / T_PROTECTED / T_PRIVATE
    pub visibility: Option<Kind>,
    pub static_: bool,
    pub abstract_: bool,
    pub final_: bool,
}

pub struct TokensAnalyzer<'a> {
    tokens: &'a Tokens,
}

fn idx(i: Option<usize>) -> usize {
    i.expect("Cannot access offset of type null on SplFixedArray")
}

impl<'a> TokensAnalyzer<'a> {
    pub fn new(tokens: &'a Tokens) -> Self {
        TokensAnalyzer { tokens }
    }

    fn prev(&self, i: usize) -> usize {
        idx(self.tokens.get_prev_meaningful_token(i))
    }

    fn next(&self, i: usize) -> usize {
        idx(self.tokens.get_next_meaningful_token(i))
    }

    /// index => element, ordered by index.
    pub fn get_classy_elements(&self) -> BTreeMap<usize, ClassyElement> {
        let mut elements = BTreeMap::new();
        let count = self.tokens.len() as isize - 2;
        let mut index = 1isize;
        while index < count {
            if self.tokens[index as usize].is_classy() {
                let (i, new) = self.find_classy_elements(index as usize, index as usize);
                index = i as isize;
                for (k, v) in new {
                    elements.entry(k).or_insert(v);
                }
            }
            index += 1;
        }
        elements
    }

    pub fn get_classy_modifiers(&self, mut index: usize) -> ClassyModifiers {
        assert!(self.tokens[index].is_classy(), "Not an \"classy\" at given index {index}.");
        let mut m = ClassyModifiers::default();
        loop {
            index = self.prev(index);
            match self.tokens[index].id {
                Some(T_FINAL) => m.final_ = Some(index),
                Some(T_ABSTRACT) => m.abstract_ = Some(index),
                Some(T_READONLY) => m.readonly = Some(index),
                _ => break,
            }
        }
        m
    }

    /// Upstream returns either one list or per-namespace lists; this returns per-namespace
    /// lists keyed by namespace ordinal (`per_namespace == false` puts everything under 0,
    /// and callers take `[0]`).
    pub fn get_import_use_indexes(&self, per_namespace: bool) -> BTreeMap<usize, Vec<usize>> {
        let t = self.tokens;
        let mut uses: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut ns = 0;
        let mut index = 0;
        while index < t.len() {
            if t[index].is_kind(T_NAMESPACE) {
                let n = idx(t.get_next_token_of_kind(index, &[c(";"), c("{")], true));
                if t[n].equals("{", true) {
                    index = n;
                }
                if per_namespace {
                    ns += 1;
                }
            } else if t[index].is_kind(T_USE) {
                uses.entry(ns).or_default().push(index);
            }
            index += 1;
        }
        uses
    }

    pub fn is_array(&self, index: usize) -> bool {
        self.tokens[index].is_given_kind(&[T_ARRAY, CT_ARRAY_BRACKET_OPEN])
    }

    pub fn is_array_multi_line(&self, mut index: usize) -> bool {
        assert!(self.is_array(index), "Not an array at given index {index}.");
        if self.tokens[index].is_kind(T_ARRAY) {
            index = self.next(index);
        }
        self.is_block_multiline(index)
    }

    pub fn is_block_multiline(&self, mut index: usize) -> bool {
        let t = self.tokens;
        let (bt, start) = Tokens::detect_block_type(&t[index]).unwrap_or((Tokens::BLOCK_TYPE_BRACE, false));
        assert!(start, "Not an block start at given index {index}.");
        let end = t.find_block_end(bt, index);
        index += 1;
        while index < end {
            if let Some((bt, true)) = Tokens::detect_block_type(&t[index]) {
                index = t.find_block_end(bt, index) + 1;
                continue;
            }
            let token = &t[index];
            if token.is_whitespace(None) && !t[index - 1].is_kind(T_END_HEREDOC) && token.content.contains(&b'\n') {
                return true;
            }
            index += 1;
        }
        false
    }

    pub fn get_method_attributes(&self, index: usize) -> MethodAttributes {
        assert!(self.tokens[index].is_kind(T_FUNCTION), "No T_FUNCTION at given index {index}");
        let mut a = MethodAttributes::default();
        let mut i = index;
        while let Some(p) = self.tokens.get_prev_meaningful_token(i) {
            i = p;
            match self.tokens[i].id {
                Some(T_STATIC) => a.static_ = true,
                Some(T_FINAL) => a.final_ = true,
                Some(T_ABSTRACT) => a.abstract_ = true,
                Some(v @ (T_PRIVATE | T_PROTECTED | T_PUBLIC)) => a.visibility = Some(v),
                _ => break,
            }
        }
        a
    }

    pub fn is_anonymous_class(&self, index: usize) -> bool {
        let t = self.tokens;
        assert!(t[index].is_classy(), "No classy token at given index {index}.");
        if !t[index].is_kind(T_CLASS) {
            return false;
        }
        let mut index = self.prev(index);
        if t[index].is_kind(T_READONLY) {
            index = self.prev(index);
        }
        while t[index].is_kind(CT_ATTRIBUTE_CLOSE) {
            index = t.find_block_start(Tokens::BLOCK_TYPE_ATTRIBUTE, index);
            index = self.prev(index);
        }
        t[index].is_kind(T_NEW)
    }

    pub fn is_lambda(&self, index: usize) -> bool {
        assert!(self.tokens[index].is_given_kind(&[T_FUNCTION, T_FN]), "No T_FUNCTION or T_FN at given index {index}");
        let mut start = self.next(index);
        if self.tokens[start].is_kind(CT_RETURN_REF) {
            start = self.next(start);
        }
        self.tokens[start].equals("(", true)
    }

    pub fn get_last_token_index_of_arrow_function(&self, index: usize) -> usize {
        let t = self.tokens;
        assert!(t[index].is_kind(T_FN), "Not an \"arrow function\" at given index {index}.");
        let stop = [c(")"), c("]"), c(","), c(";"), k(T_CLOSE_TAG)];
        let mut index = idx(t.get_next_token_of_kind(index, &[k(T_DOUBLE_ARROW)], true));
        loop {
            index = self.next(index);
            if t[index].equals_any(&stop, true) {
                break;
            }
            match Tokens::detect_block_type(&t[index]) {
                None => continue,
                Some((bt, true)) => {
                    index = t.find_block_end(bt, index);
                    continue;
                }
                Some(_) => break,
            }
        }
        self.prev(index)
    }

    pub fn is_unary_successor_operator(&self, index: usize) -> bool {
        if !self.tokens[index].is_given_kind(&[T_INC, T_DEC]) {
            return false;
        }
        self.tokens[self.prev(index)].equals_any(
            &[
                c("]"),
                k(T_STRING),
                k(T_VARIABLE),
                k(CT_ARRAY_INDEX_BRACE_CLOSE),
                k(CT_DYNAMIC_PROP_BRACE_CLOSE),
                k(CT_DYNAMIC_VAR_BRACE_CLOSE),
            ],
            true,
        )
    }

    pub fn is_unary_predecessor_operator(&self, index: usize) -> bool {
        let t = self.tokens;
        let token = &t[index];
        if token.is_given_kind(&[T_INC, T_DEC]) {
            return !self.is_unary_successor_operator(index);
        }
        if token.equals_any(&[c("!"), c("~"), c("@"), k(T_ELLIPSIS)], true) {
            return true;
        }
        if !token.equals_any(&[c("+"), c("-"), c("&"), k(CT_RETURN_REF)], true) {
            return false;
        }
        let prev = &t[self.prev(index)];
        if !prev.equals_any(
            &[
                c("]"),
                c("}"),
                c(")"),
                c("\""),
                c("`"),
                k(CT_ARRAY_BRACKET_CLOSE),
                k(CT_ARRAY_INDEX_BRACE_CLOSE),
                k(CT_DYNAMIC_PROP_BRACE_CLOSE),
                k(CT_DYNAMIC_VAR_BRACE_CLOSE),
                k(T_CLASS_C),
                k(T_CONSTANT_ENCAPSED_STRING),
                k(T_DEC),
                k(T_DIR),
                k(T_DNUMBER),
                k(T_FILE),
                k(T_FUNC_C),
                k(T_INC),
                k(T_LINE),
                k(T_LNUMBER),
                k(T_METHOD_C),
                k(T_NS_C),
                k(T_STRING),
                k(T_TRAIT_C),
                k(T_VARIABLE),
            ],
            true,
        ) {
            return true;
        }
        if !token.equals("&", true) || !prev.is_kind(T_STRING) {
            return false;
        }
        let p = idx(t.get_prev_token_of_kind(
            index,
            &[c(";"), c("{"), c("}"), k(T_DOUBLE_ARROW), k(T_FN), k(T_FUNCTION), k(T_OPEN_TAG), k(T_OPEN_TAG_WITH_ECHO)],
            true,
        ));
        t[p].is_given_kind(&[T_FN, T_FUNCTION])
    }

    pub fn is_binary_operator(&self, index: usize) -> bool {
        let token = &self.tokens[index];
        if token.is_given_kind(&[T_INLINE_HTML, T_ENCAPSED_AND_WHITESPACE, CT_TYPE_INTERSECTION]) {
            return false;
        }
        if matches!(&token.content[..], b"+" | b"-" | b"&") {
            return !self.is_unary_predecessor_operator(index);
        }
        if let Some(id) = token.id {
            return matches!(
                id,
                T_AND_EQUAL
                    | T_BOOLEAN_AND
                    | T_BOOLEAN_OR
                    | T_CONCAT_EQUAL
                    | T_DIV_EQUAL
                    | T_DOUBLE_ARROW
                    | T_IS_EQUAL
                    | T_IS_GREATER_OR_EQUAL
                    | T_IS_IDENTICAL
                    | T_IS_NOT_EQUAL
                    | T_IS_NOT_IDENTICAL
                    | T_IS_SMALLER_OR_EQUAL
                    | T_LOGICAL_AND
                    | T_LOGICAL_OR
                    | T_LOGICAL_XOR
                    | T_MINUS_EQUAL
                    | T_MOD_EQUAL
                    | T_MUL_EQUAL
                    | T_OR_EQUAL
                    | T_PLUS_EQUAL
                    | T_POW
                    | T_POW_EQUAL
                    | T_SL
                    | T_SL_EQUAL
                    | T_SR
                    | T_SR_EQUAL
                    | T_XOR_EQUAL
                    | T_SPACESHIP
                    | T_COALESCE
                    | T_COALESCE_EQUAL
            );
        }
        matches!(&token.content[..], b"=" | b"*" | b"/" | b"%" | b"<" | b">" | b"|" | b"^" | b".")
    }

    pub fn is_while_part_of_do_while(&self, index: usize) -> bool {
        let t = self.tokens;
        assert!(t[index].is_kind(T_WHILE), "No T_WHILE at given index {index}");
        let end = self.prev(index);
        if !t[end].equals("}", true) {
            return false;
        }
        let start = t.find_block_start(Tokens::BLOCK_TYPE_BRACE, end);
        t[self.prev(start)].is_kind(T_DO)
    }

    pub fn is_enum_case(&self, case_index: usize) -> bool {
        let t = self.tokens;
        assert!(t[case_index].is_kind(T_CASE), "No T_CASE given at index {case_index}");
        if !t.is_token_kind_found(T_ENUM) {
            return false;
        }
        let mut p = case_index;
        loop {
            p = idx(t.get_prev_token_of_kind(p, &[c("}"), k(T_ENUM), k(T_SWITCH)], true));
            if t[p].equals("}", true) {
                p = t.find_block_start(Tokens::BLOCK_TYPE_BRACE, p);
            } else {
                break;
            }
        }
        t[p].is_kind(T_ENUM)
    }

    pub fn is_super_global(&self, index: usize) -> bool {
        let token = &self.tokens[index];
        token.is_kind(T_VARIABLE)
            && matches!(
                &token.content.to_ascii_uppercase()[..],
                b"$_COOKIE" | b"$_ENV" | b"$_FILES" | b"$_GET" | b"$_POST" | b"$_REQUEST" | b"$_SERVER" | b"$_SESSION" | b"$GLOBALS"
            )
    }

    fn find_classy_elements(&self, class_index: usize, mut index: usize) -> (usize, BTreeMap<usize, ClassyElement>) {
        let t = self.tokens;
        let mut elements = BTreeMap::new();
        let merge = |elements: &mut BTreeMap<usize, ClassyElement>, new: BTreeMap<usize, ClassyElement>| {
            for (k, v) in new {
                elements.entry(k).or_insert(v);
            }
        };
        let (mut curly, mut braces) = (0i32, 0i32);
        let count = t.len();
        index += 1;
        while index < count {
            let token = &t[index];
            if token.is_kind(T_ENCAPSED_AND_WHITESPACE) {
                index += 1;
                continue;
            }
            if token.is_kind(T_CLASS) {
                let nested = index;
                index = self.next(index);
                if t[index].equals("(", true) {
                    index += 1;
                    let mut level = 1;
                    while index < count {
                        let tk = &t[index];
                        if tk.equals("(", true) {
                            level += 1;
                        } else if tk.equals(")", true) {
                            level -= 1;
                            if level == 0 {
                                let (i, new) = self.find_classy_elements(nested, index);
                                index = i;
                                merge(&mut elements, new);
                                break;
                            }
                        } else if tk.is_kind(T_CLASS) {
                            let (i, new) = self.find_classy_elements(index, index);
                            index = i;
                            merge(&mut elements, new);
                        }
                        index += 1;
                    }
                } else {
                    let (i, new) = self.find_classy_elements(nested, nested);
                    index = i;
                    merge(&mut elements, new);
                }
                index += 1;
                continue;
            }
            if token.equals("(", true) {
                braces += 1;
            } else if token.equals(")", true) {
                braces -= 1;
            } else if token.equals("{", true) {
                curly += 1;
            } else if token.equals("}", true) {
                curly -= 1;
                if curly == 0 {
                    break;
                }
            } else if curly == 1 && token.is_array() {
                let mut add = |i: usize, kind| {
                    elements.insert(i, ClassyElement { class_index, kind });
                };
                if braces == 0 && token.is_kind(T_VARIABLE) {
                    add(index, ClassyElementType::Property);
                } else if token.is_kind(CT_PROPERTY_HOOK_BRACE_OPEN) {
                    index = idx(t.get_next_token_of_kind(index, &[k(CT_PROPERTY_HOOK_BRACE_CLOSE)], true));
                } else if token.is_kind(T_ATTRIBUTE) {
                    index = t.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, index);
                } else if token.is_kind(T_FUNCTION) {
                    let name = self.next(index);
                    if !t[name].equals("(", true) {
                        add(index, ClassyElementType::Method);
                        if t[name].content == b"__construct" {
                            let open = self.next(name);
                            let close = t.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
                            let kinds = [
                                CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
                                CT_CONSTRUCTOR_PROPERTY_PROMOTION_PROTECTED,
                                CT_CONSTRUCTOR_PROPERTY_PROMOTION_PRIVATE,
                                T_PRIVATE_SET,
                                T_PROTECTED_SET,
                                T_PUBLIC_SET,
                                T_READONLY,
                                T_FINAL,
                            ];
                            // Upstream iterates findGivenKind's per-kind groups in this kind order.
                            let found = t.find_given_kinds(&kinds, open, Some(close));
                            for kd in kinds {
                                for &m in &found[&kd] {
                                    let var = idx(t.get_next_token_of_kind(m, &[k(T_VARIABLE)], true));
                                    add(var, ClassyElementType::PromotedProperty);
                                }
                            }
                        }
                    }
                } else if token.is_kind(T_CONST) {
                    add(index, ClassyElementType::Const);
                } else if token.is_kind(CT_USE_TRAIT) {
                    add(index, ClassyElementType::TraitImport);
                } else if token.is_kind(T_CASE) {
                    add(index, ClassyElementType::Case);
                }
            }
            index += 1;
        }
        (index, elements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classy_elements_and_operators() {
        let t = Tokens::from_code(
            b"<?php class A { use T; const X = 1; public int $p; public function __construct(private int $q) {} function f() { $x = -1; $i++; return new class { public $z; }; } }",
        );
        let a = TokensAnalyzer::new(&t);
        let kinds: Vec<&str> = a.get_classy_elements().values().map(|e| e.kind.as_str()).collect();
        assert_eq!(kinds, ["trait_import", "const", "property", "method", "promoted_property", "method", "property"]);
        let minus = t.iter().position(|x| x.content == b"-").unwrap();
        assert!(a.is_unary_predecessor_operator(minus) && !a.is_binary_operator(minus));
        let inc = t.iter().position(|x| x.is_kind(T_INC)).unwrap();
        assert!(a.is_unary_successor_operator(inc));
        let f = t.iter().position(|x| x.is_kind(T_FUNCTION)).unwrap();
        assert!(!a.is_lambda(f));
        assert_eq!(a.get_method_attributes(f).visibility, Some(T_PUBLIC));
    }
}
