//! Port of Mago's `prefer-array-validation-rules` (Laravel): inside a `rules()` method,
//! `'name' => 'required|string|max:255'` → `'name' => ['required', 'string', 'max:255']`.
//! Opt-in only (`Pun/prefer_array_validation_rules`). Same bail-outs as Mago (`regex:`, empty segments,
//! quotes/backslashes); additionally only string-keyed elements whose value is the whole string literal.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::tokens_analyzer::ClassyElementType;
use pun_tokens::{Token, Tokens, TokensAnalyzer};
use std::path::Path;

struct PreferArrayValidationRules;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PreferArrayValidationRules)
}

impl Fixer for PreferArrayValidationRules {
    fn name(&self) -> &'static str {
        "Pun/prefer_array_validation_rules"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_FUNCTION, T_DOUBLE_ARROW, T_CONSTANT_ENCAPSED_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let methods: Vec<usize> = TokensAnalyzer::new(tokens)
            .get_classy_elements()
            .into_iter()
            .filter(|(i, e)| {
                e.kind == ClassyElementType::Method
                    && tokens
                        .get_next_meaningful_token(*i)
                        .is_some_and(|n| tokens[n].get_content().eq_ignore_ascii_case(b"rules"))
            })
            .map(|(i, _)| i)
            .collect();
        for &method in methods.iter().rev() {
            let Some(open) = tokens.get_next_token_of_kind(method, &["{".into(), ";".into()], true)
            else {
                continue;
            };
            if !tokens[open].equals("{", true) {
                continue;
            }
            let end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, open);
            let mut found = Vec::new();
            let mut i = open + 1;
            while i < end {
                if tokens[i].is_kind(T_FUNCTION) {
                    // Nested closure: its own scope, as in Mago.
                    if let Some(o) = tokens.get_next_token_of_kind(i, &["{".into()], true) {
                        i = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, o);
                    }
                } else if tokens[i].is_kind(T_FN) {
                    i = TokensAnalyzer::new(tokens).get_last_token_index_of_arrow_function(i);
                } else if let Some(r) = replacement(tokens, i) {
                    found.push((i, r));
                }
                i += 1;
            }
            for (i, r) in found.into_iter().rev() {
                tokens.override_range(i, i, r);
            }
        }
    }
}

fn replacement(tokens: &Tokens, i: usize) -> Option<Vec<Token>> {
    if !tokens[i].is_kind(T_CONSTANT_ENCAPSED_STRING) {
        return None;
    }
    let arrow = tokens
        .get_prev_meaningful_token(i)
        .filter(|&a| tokens[a].is_kind(T_DOUBLE_ARROW))?;
    tokens
        .get_prev_meaningful_token(arrow)
        .filter(|&k| tokens[k].is_kind(T_CONSTANT_ENCAPSED_STRING))?;
    tokens.get_next_meaningful_token(i).filter(|&n| {
        tokens[n].equals_any(
            &[",".into(), ")".into(), CT_ARRAY_BRACKET_CLOSE.into()],
            true,
        )
    })?;
    let raw = tokens[i].get_content();
    let quote = raw[0];
    if !(quote == b'\'' || quote == b'"') {
        return None;
    }
    let content = &raw[1..raw.len() - 1];
    if !content.contains(&b'|') || content.windows(6).any(|w| w == b"regex:") {
        return None;
    }
    let segments: Vec<&[u8]> = content.split(|&b| b == b'|').collect();
    if segments
        .iter()
        .any(|s| s.is_empty() || s.iter().any(|b| matches!(b, b'\'' | b'\\' | b'$')))
    {
        return None;
    }
    let mut out = vec![Token::new(CT_ARRAY_BRACKET_OPEN, "[")];
    for (n, s) in segments.iter().enumerate() {
        if n > 0 {
            out.push(Token::char(","));
            out.push(Token::new(T_WHITESPACE, " "));
        }
        out.push(Token::new(
            T_CONSTANT_ENCAPSED_STRING,
            [b"'", *s, b"'"].concat(),
        ));
    }
    out.push(Token::new(CT_ARRAY_BRACKET_CLOSE, "]"));
    Some(out)
}

#[cfg(test)]
mod tests {
    fn fix(code: &str) -> String {
        let mut tokens = pun_tokens::Tokens::from_code(code.as_bytes());
        // Gate on is_candidate like Pipeline::run does, so a too-strict candidate check fails the cases.
        let f = super::fixer();
        if f.is_candidate(&tokens) {
            f.fix(std::path::Path::new("t.php"), &mut tokens);
        }
        String::from_utf8(tokens.generate_code()).unwrap()
    }

    fn class(name: &str, method: &str, items: &str) -> String {
        format!(
            "<?php\n\nclass {name}\n{{\n    public function {method}(): array\n    {{\n        return [\n{items}        ];\n    }}\n}}\n"
        )
    }

    #[test]
    fn mago_cases() {
        // fix_pipe_string_to_array
        assert_eq!(
            fix(&class(
                "StoreUserRequest",
                "rules",
                "            'name' => 'required|string|max:255',\n"
            )),
            class(
                "StoreUserRequest",
                "rules",
                "            'name' => ['required', 'string', 'max:255'],\n"
            )
        );
        // pipe_delimited_rule_string (count = 2)
        assert_eq!(
            fix(&class(
                "StoreUserRequest",
                "rules",
                "            'name' => 'required|string|max:255',\n            'email' => 'required|email',\n"
            )),
            class(
                "StoreUserRequest",
                "rules",
                "            'name' => ['required', 'string', 'max:255'],\n            'email' => ['required', 'email'],\n"
            )
        );
        // array_rules, single_rule_string, regex_rule_not_split, pipe_string_outside_rules_method
        for same in [
            class(
                "StoreUserRequest",
                "rules",
                "            'name' => ['required', 'string'],\n",
            ),
            class(
                "StoreUserRequest",
                "rules",
                "            'name' => 'required',\n",
            ),
            class(
                "StoreUserRequest",
                "rules",
                "            'slug' => 'required|regex:/^[a-z]+|[0-9]+$/',\n",
            ),
            class("Report", "build", "            'label' => 'left|right',\n"),
        ] {
            assert_eq!(fix(&same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix(&class(
                "R",
                "RULES",
                "            \"a\" => \"nullable|int\"\n"
            )),
            class("R", "RULES", "            \"a\" => ['nullable', 'int']\n")
        );
        for same in [
            class("R", "rules", "            'a' => 'x||y',\n"),
            class("R", "rules", "            'a' => 'x|' . $y,\n"),
            class("R", "rules", "            'a' => \"x|$y\",\n"),
            class("R", "rules", "            'a' => fn () => ['b' => 'x|y'],\n"),
            class("R", "rules", "            'a' => function () { return ['b' => 'x|y']; },\n"),
            "<?php function rules() { return ['a' => 'x|y']; }".into(),
            "<?php abstract class R { abstract public function rules(); public function x() { return ['a' => 'x|y']; } }".into(),
            class("R", "rules", "            'a' => b'x|y',\n"),
        ] {
            assert_eq!(fix(&same), same);
        }
    }
}
