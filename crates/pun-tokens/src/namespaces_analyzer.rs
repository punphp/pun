//! Ports of `PhpCsFixer\Tokenizer\Analyzer\NamespacesAnalyzer` and `Analysis\NamespaceAnalysis`.
//! `Tokens::namespace_declarations` caches the result per token-list version, like upstream's
//! `Tokens::getNamespaceDeclarations()`.

use crate::Tokens;
use crate::kind::Kind::*;
use crate::php_compat::trim;
use std::sync::Arc;

/// `NamespaceAnalysis`: public fields named after upstream's getters. Strings are bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamespaceAnalysis {
    pub full_name: Vec<u8>,
    pub short_name: Vec<u8>,
    pub start_index: usize,
    pub end_index: usize,
    pub scope_start_index: usize,
    pub scope_end_index: usize,
}

impl NamespaceAnalysis {
    pub fn is_global_namespace(&self) -> bool {
        self.full_name.is_empty()
    }
}

pub struct NamespacesAnalyzer;

impl NamespacesAnalyzer {
    pub fn get_declarations(&self, tokens: &Tokens) -> Vec<NamespaceAnalysis> {
        self.declarations(tokens).to_vec()
    }

    /// `Tokens::getNamespaceDeclarations()`: computed once per token-list version.
    pub fn declarations(&self, tokens: &Tokens) -> Arc<Vec<NamespaceAnalysis>> {
        tokens.namespace_declarations()
    }

    /// `getDeclarations`, uncached.
    pub(crate) fn compute(tokens: &Tokens) -> Vec<NamespaceAnalysis> {
        let mut namespaces = Vec::new();
        let count = tokens.count();
        let mut index = 1;
        while index < count {
            if !tokens[index].is_kind(T_NAMESPACE) {
                index += 1;
                continue;
            }
            let declaration_end_index = tokens
                .get_next_token_of_kind(index, &[";".into(), "{".into()], true)
                .unwrap();
            let namespace =
                trim(&tokens.generate_partial_code(index + 1, declaration_end_index - 1)).to_vec();
            let short_name = namespace
                .rsplit(|&c| c == b'\\')
                .next()
                .unwrap_or(&[])
                .to_vec();
            let scope_end_index = if tokens[declaration_end_index].equals("{", true) {
                tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, declaration_end_index)
            } else {
                tokens
                    .get_next_token_of_kind(declaration_end_index, &[T_NAMESPACE.into()], true)
                    .unwrap_or(count)
                    - 1
            };
            namespaces.push(NamespaceAnalysis {
                full_name: namespace,
                short_name,
                start_index: index,
                end_index: declaration_end_index,
                scope_start_index: index,
                scope_end_index,
            });
            // Continue the analysis after the end of this namespace to find the next one
            index = scope_end_index + 1;
        }
        if namespaces.is_empty() && tokens.is_token_kind_found(T_OPEN_TAG) {
            let open_tag_index = usize::from(tokens[0].is_kind(T_INLINE_HTML));
            namespaces.push(NamespaceAnalysis {
                full_name: Vec::new(),
                short_name: Vec::new(),
                start_index: open_tag_index,
                end_index: open_tag_index,
                scope_start_index: open_tag_index,
                scope_end_index: count - 1,
            });
        }
        namespaces
    }

    pub fn get_namespace_at(&self, tokens: &Tokens, index: usize) -> NamespaceAnalysis {
        assert!(index < tokens.len(), "Token index {index} does not exist.");
        self.declarations(tokens)
            .iter()
            .find(|ns| ns.scope_start_index <= index && ns.scope_end_index >= index)
            .cloned()
            .unwrap_or_else(|| panic!("Unable to get the namespace at index {index}."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Row = (String, String, usize, usize, usize, usize);
    /// (full name, short name, start, end, scope start, scope end).
    type Expected = (&'static str, &'static str, usize, usize, usize, usize);

    fn row(n: &NamespaceAnalysis) -> Row {
        (
            String::from_utf8(n.full_name.clone()).unwrap(),
            String::from_utf8(n.short_name.clone()).unwrap(),
            n.start_index,
            n.end_index,
            n.scope_start_index,
            n.scope_end_index,
        )
    }

    fn rows(expected: &[Expected]) -> Vec<Row> {
        expected
            .iter()
            .map(|&(f, s, a, b, c, d)| (f.into(), s.into(), a, b, c, d))
            .collect()
    }

    /// Expectations: PHP-CS-Fixer 3.95's `NamespacesAnalyzer::getDeclarations` on each input
    /// (upstream's NamespacesAnalyzerTest cases plus two `;` namespaces).
    #[test]
    fn declarations_match_php_cs_fixer() {
        let cases: &[(&str, &[Expected])] = &[
            ("<?php // no namespaces", &[("", "", 0, 0, 0, 1)]),
            (
                "<?php namespace Foo\\Bar;",
                &[("Foo\\Bar", "Bar", 1, 6, 1, 6)],
            ),
            (
                "<?php namespace Foo\\Bar{}; namespace Foo\\Baz {};",
                &[
                    ("Foo\\Bar", "Bar", 1, 6, 1, 7),
                    ("Foo\\Baz", "Baz", 10, 16, 10, 17),
                ],
            ),
            (
                "#!/usr/bin/php\n<?php\nreturn true;",
                &[("", "", 1, 1, 1, 5)],
            ),
            ("there is no namespace if there is no PHP code", &[]),
            (
                "<?php namespace A; f(); namespace B; g();",
                &[("A", "A", 1, 4, 1, 10), ("B", "B", 11, 14, 11, 19)],
            ),
        ];
        for (code, expected) in cases {
            let t = Tokens::from_code(code.as_bytes());
            let got: Vec<Row> = NamespacesAnalyzer
                .get_declarations(&t)
                .iter()
                .map(row)
                .collect();
            assert_eq!(got, rows(expected), "{code}");
        }
        let t = Tokens::from_code(b"<?php namespace Foo\\Bar;");
        assert!(!NamespacesAnalyzer.get_declarations(&t)[0].is_global_namespace());
        let t = Tokens::from_code(b"<?php // no namespaces");
        assert!(NamespacesAnalyzer.get_declarations(&t)[0].is_global_namespace());
    }

    #[test]
    fn namespace_at_matches_php_cs_fixer() {
        let t = Tokens::from_code(b"<?php namespace Foo\\Bar{}; namespace Foo\\Baz {};");
        let at = |i| row(&NamespacesAnalyzer.get_namespace_at(&t, i));
        assert_eq!(at(5), rows(&[("Foo\\Bar", "Bar", 1, 6, 1, 7)])[0]);
        assert_eq!(at(13), rows(&[("Foo\\Baz", "Baz", 10, 16, 10, 17)])[0]);
    }

    #[test]
    #[should_panic(expected = "Token index 666 does not exist.")]
    fn namespace_at_out_of_range() {
        NamespacesAnalyzer.get_namespace_at(&Tokens::default(), 666);
    }

    #[test]
    #[should_panic(expected = "Unable to get the namespace at index 0.")]
    fn namespace_at_without_php_code() {
        NamespacesAnalyzer.get_namespace_at(&Tokens::from_code(b"x"), 0);
    }
}
