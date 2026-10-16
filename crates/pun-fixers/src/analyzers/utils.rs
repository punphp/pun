//! Port of `PhpCsFixer\Utils` (the parts fixers use; `sortFixers` lives in pun_format::Pipeline).
use super::preg::Preg;
use pun_tokens::Token;

pub struct Utils;

impl Utils {
    /// Converts a camel cased string to a snake cased string.
    pub fn camel_case_to_underscore(string: &str) -> String {
        let replaced = Preg::replace("/(?<!^)(?<!_)((?=[\\p{Lu}][^\\p{Lu}])|(?<![\\p{Lu}])(?=[\\p{Lu}]))/", "_", string);
        String::from_utf8_lossy(&replaced).to_lowercase() // mb_strtolower
    }

    /// Everything after the final newline of a whitespace token.
    pub fn calculate_trailing_whitespace_indent(token: &Token) -> Vec<u8> {
        assert!(token.is_whitespace(None), "The given token must be whitespace, got \"{:?}\".", token.get_name());
        let content = token.get_content();
        let mut s = Vec::with_capacity(content.len());
        let mut i = 0;
        while i < content.len() {
            if content[i] == b'\r' {
                s.push(b'\n');
                if content.get(i + 1) == Some(&b'\n') {
                    i += 1;
                }
            } else {
                s.push(content[i]);
            }
            i += 1;
        }
        match s.iter().rposition(|&b| b == b'\n') {
            None => Vec::new(),
            Some(p) => s[p..].iter().skip_while(|&&b| b == b'\n').copied().collect(),
        }
    }

    /// Stable sort by a derived value (Schwartzian transform, ties keep input order).
    pub fn stable_sort<T, R>(elements: Vec<T>, get_compared_value: impl Fn(&T) -> R, compare_values: impl Fn(&R, &R) -> std::cmp::Ordering) -> Vec<T> {
        let mut items: Vec<(T, R)> = elements.into_iter().map(|e| {
            let v = get_compared_value(&e);
            (e, v)
        }).collect();
        items.sort_by(|a, b| compare_values(&a.1, &b.1)); // Vec::sort_by is stable
        items.into_iter().map(|(e, _)| e).collect()
    }
}
