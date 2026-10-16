//! Port of `PhpCsFixer\Tokenizer\Analyzer\FullyQualifiedNameAnalyzer`.
use crate::analyzers::analysis::{NamespaceAnalysis, NamespaceUseAnalysis, NamespaceUseType};
use crate::analyzers::namespace_uses_analyzer::NamespaceUsesAnalyzer;
use crate::analyzers::namespaces_analyzer::NamespacesAnalyzer;
use pun_tokens::Tokens;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct FullyQualifiedNameAnalyzer<'a> {
    tokens: &'a Tokens,
    namespace_analyses: RefCell<Vec<NamespaceAnalysis>>,
    namespace_use_analyses: RefCell<HashMap<usize, Vec<NamespaceUseAnalysis>>>,
}

impl<'a> FullyQualifiedNameAnalyzer<'a> {
    pub fn new(tokens: &'a Tokens) -> Self {
        FullyQualifiedNameAnalyzer { tokens, namespace_analyses: RefCell::default(), namespace_use_analyses: RefCell::default() }
    }

    pub fn get_fully_qualified_name(&self, name: &[u8], index_in_namespace: usize, import_type: NamespaceUseType) -> Vec<u8> {
        let n = self.get_fully_qualified_name_with_possibly_leading_slash(name, index_in_namespace, import_type);
        n[n.iter().take_while(|&&c| c == b'\\').count()..].to_vec()
    }

    fn get_fully_qualified_name_with_possibly_leading_slash(&self, name: &[u8], index: usize, import_type: NamespaceUseType) -> Vec<u8> {
        if name[0] == b'\\' {
            return name.to_vec();
        }
        let namespace_analysis = self.get_namespace_analysis(index);
        let mut cache = self.namespace_use_analyses.borrow_mut();
        let uses = cache
            .entry(namespace_analysis.start_index)
            .or_insert_with(|| NamespaceUsesAnalyzer.get_declarations_in_namespace(self.tokens, &namespace_analysis, false));
        // $declarations[strtolower(shortName)] = fullName: later duplicates overwrite in place.
        let mut declarations: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        for u in uses.iter() {
            if u.type_ != import_type {
                continue;
            }
            let key = u.short_name.to_ascii_lowercase();
            match declarations.iter_mut().find(|(k, _)| *k == key) {
                Some(slot) => slot.1 = u.full_name.clone(),
                None => declarations.push((key, u.full_name.clone())),
            }
        }
        let lowercase_name = name.to_ascii_lowercase();
        for (lowercase_short_name, full_name) in &declarations {
            if lowercase_name == *lowercase_short_name {
                return full_name.clone();
            }
            if !lowercase_name.starts_with(&[&lowercase_short_name[..], b"\\"].concat()) {
                continue;
            }
            return [&full_name[..], &name[lowercase_short_name.len()..]].concat();
        }
        [&namespace_analysis.full_name[..], b"\\", name].concat()
    }

    fn get_namespace_analysis(&self, index: usize) -> NamespaceAnalysis {
        if let Some(ns) = self.namespace_analyses.borrow().iter().find(|ns| ns.scope_start_index <= index && ns.scope_end_index >= index) {
            return ns.clone();
        }
        let ns = NamespacesAnalyzer.get_namespace_at(self.tokens, index);
        self.namespace_analyses.borrow_mut().push(ns.clone());
        ns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected values: PhpCsFixer's FullyQualifiedNameAnalyzer::getFullyQualifiedName on the same code and index
    /// (function imports ignored for classes, a repeated short name overwrites, `y\Q` resolves through `use C\Y`).
    #[test]
    fn matches_oracle() {
        let tokens = Tokens::from_code(b"<?php namespace N; use function F\\x; use A\\X; use B\\X; use C\\Y; class Z {}");
        let a = FullyQualifiedNameAnalyzer::new(&tokens);
        let last = tokens.count() - 1;
        let got = |n: &str, t| String::from_utf8(a.get_fully_qualified_name(n.as_bytes(), last, t)).unwrap();
        assert_eq!(got("x", NamespaceUseType::Class), "B\\X");
        assert_eq!(got("X", NamespaceUseType::Class), "B\\X");
        assert_eq!(got("y\\Q", NamespaceUseType::Class), "C\\Y\\Q");
        assert_eq!(got("Q", NamespaceUseType::Class), "N\\Q");
        assert_eq!(got("x", NamespaceUseType::Function), "F\\x");
    }
}
