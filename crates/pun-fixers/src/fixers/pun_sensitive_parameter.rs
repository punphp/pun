//! Port of Mago's `sensitive-parameter`: adds `#[\SensitiveParameter]` to parameters whose name ends in
//! `password`, `token`, `secret`, `apikey` or `api_key` (after an optional `get`/`get_` prefix), unless the
//! parameter is boolean-like (`bool $canChangePassword`) or already has the attribute.
//! Opt-in only (`Pun/sensitive_parameter`). The attribute needs PHP >= 8.0 syntax and only takes effect on 8.2+.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct SensitiveParameter;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SensitiveParameter)
}

impl Fixer for SensitiveParameter {
    fn name(&self) -> &'static str {
        "Pun/sensitive_parameter"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_FUNCTION, T_FN])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (0..tokens.count()).rev() {
            if !tokens[i].is_given_kind(&[T_FUNCTION, T_FN]) {
                continue;
            }
            let mut k = tokens.get_next_meaningful_token(i);
            while let Some(n) = k.filter(|&n| tokens[n].is_given_kind(&[CT_RETURN_REF, T_STRING]) || tokens[n].equals("&", true)) {
                k = tokens.get_next_meaningful_token(n);
            }
            let Some(open) = k.filter(|&o| tokens[o].equals("(", true)) else { continue };
            let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
            for (s, e) in split_list(tokens, open, close).into_iter().rev() {
                if needs_attribute(tokens, s, e) {
                    tokens.insert_at(s, vec![
                        Token::new(T_ATTRIBUTE, "#["),
                        Token::new(T_NS_SEPARATOR, "\\"),
                        Token::new(T_STRING, "SensitiveParameter"),
                        Token::new(CT_ATTRIBUTE_CLOSE, "]"),
                        Token::new(T_WHITESPACE, " "),
                    ]);
                }
            }
        }
    }
}

fn needs_attribute(tokens: &Tokens, s: usize, e: usize) -> bool {
    let mut type_names = vec![];
    let mut k = s;
    while k <= e {
        let t = &tokens[k];
        if t.is_kind(T_ATTRIBUTE) {
            let end = tokens.find_block_end(Tokens::BLOCK_TYPE_ATTRIBUTE, k);
            if (k..end).any(|a| tokens[a].is_kind(T_STRING) && tokens[a].get_content().eq_ignore_ascii_case(b"SensitiveParameter")) {
                return false;
            }
            k = end;
        } else if t.is_kind(T_VARIABLE) {
            let lower = t.get_content()[1..].to_ascii_lowercase();
            let name = lower.strip_prefix(b"get").map(|n| n.strip_prefix(b"_").unwrap_or(n)).unwrap_or(&lower);
            let sensitive = [&b"password"[..], b"token", b"secret", b"apikey", b"api_key"].iter().any(|w| name.ends_with(w));
            let boolean = !type_names.is_empty() && type_names.iter().all(|n: &Vec<u8>| [&b"bool"[..], b"true", b"false", b"null"].contains(&n.as_slice()));
            return sensitive && !boolean;
        } else if t.is_given_kind(&[T_STRING, T_ARRAY, T_CALLABLE, T_STATIC, CT_ARRAY_TYPEHINT]) {
            type_names.push(t.get_content().to_ascii_lowercase());
        }
        k += 1;
    }
    false
}

/// (first, last) meaningful token of each comma-separated item between `open` and `close`.
fn split_list(tokens: &Tokens, open: usize, close: usize) -> Vec<(usize, usize)> {
    let mut items = vec![];
    let mut k = tokens.get_next_meaningful_token(open).unwrap();
    while k < close {
        let start = k;
        let mut last = k;
        while k < close && !tokens[k].equals(",", true) {
            if let Some((t, true)) = Tokens::detect_block_type(&tokens[k]) {
                k = tokens.find_block_end(t, k);
            }
            last = k;
            k = tokens.get_next_meaningful_token(k).unwrap();
        }
        items.push((start, last));
        if k < close {
            k = tokens.get_next_meaningful_token(k).unwrap();
        }
    }
    items
}

#[cfg(test)]
mod tests {
    fn fix(code: &str) -> String {
        let mut tokens = pun_tokens::Tokens::from_code(code.as_bytes());
        super::fixer().fix(std::path::Path::new("t.php"), &mut tokens);
        String::from_utf8(tokens.generate_code()).unwrap()
    }

    #[test]
    fn mago_cases() {
        let f = |params: &str| format!("<?php\n\nfunction login({params}): void {{\n    // ...\n}}\n");
        for (from, to) in [
            ("string $username, string $password", "string $username, #[\\SensitiveParameter] string $password"),
            ("string $username, $password", "string $username, #[\\SensitiveParameter] $password"),
            ("string $username, bool|string $password", "string $username, #[\\SensitiveParameter] bool|string $password"),
            ("string $oldPassword, string $newPassword", "#[\\SensitiveParameter] string $oldPassword, #[\\SensitiveParameter] string $newPassword"),
        ] {
            assert_eq!(fix(&f(from)), f(to));
        }
        for same in [
            "string $username, #[SensitiveParameter] string $password",
            "string $username, #[\\SensitiveParameter] string $password",
            "string $username, string $email",
            "bool $canChangePassword",
            "?bool $canChangePassword",
            "null|true|false $canChangePassword",
        ] {
            assert_eq!(fix(&f(same)), f(same));
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php class A {\n    public function __construct(\n        private readonly string $apiKey,\n        #[Foo] ?string $get_token = null,\n    ) {}\n}"),
            "<?php class A {\n    public function __construct(\n        #[\\SensitiveParameter] private readonly string $apiKey,\n        #[\\SensitiveParameter] #[Foo] ?string $get_token = null,\n    ) {}\n}"
        );
        assert_eq!(fix("<?php $f = fn (&$secret) => 1;"), "<?php $f = fn (#[\\SensitiveParameter] &$secret) => 1;");
        assert_eq!(fix("<?php $f = function (string ...$tokens) {};"), "<?php $f = function (string ...$tokens) {};");
        assert_eq!(fix("<?php function &f(Secret $client_secret) {}"), "<?php function &f(#[\\SensitiveParameter] Secret $client_secret) {}");
        for same in [
            "<?php function f($passwordConfirmed = PASSWORD) {}",
            "<?php $f = function () use ($password) {};",
            "<?php function f(#[\\SensitiveParameter, Foo] $password) {}",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
