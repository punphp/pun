//! Port of Mago's `no-redundant-nullsafe`: `$user?->name ?? 'Guest'` → `$user->name ?? 'Guest'` and
//! `isset($user?->profile)` → `isset($user->profile)`: `??` and `isset` already read the property quietly.
//! Opt-in only (`Pun/no_redundant_nullsafe`). Like Mago, only the last `?->` of the operand, and only when the
//! property fetch is the whole left side of `??` (`'a' . $x?->b ?? 'c'` is `('a' . $x?->b) ?? 'c'`) or a whole
//! `isset()` argument.
use crate::analyzers::helpers::COMPOUND_ASSIGNMENTS;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoRedundantNullsafe;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoRedundantNullsafe)
}

impl Fixer for NoRedundantNullsafe {
    fn name(&self) -> &'static str {
        "Pun/no_redundant_nullsafe"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_NULLSAFE_OBJECT_OPERATOR)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (0..tokens.count()).rev() {
            if !tokens[i].is_kind(T_NULLSAFE_OBJECT_OPERATOR) {
                continue;
            }
            let Some(name) = tokens
                .get_next_meaningful_token(i)
                .filter(|&n| tokens[n].is_kind(T_STRING))
            else {
                continue;
            };
            // Parsed code never ends right after a property name, and a chain always follows the open tag.
            let after = tokens.get_next_meaningful_token(name).unwrap();
            let Some(start) = chain_start(tokens, i) else {
                continue;
            };
            let before = tokens.get_prev_meaningful_token(start).expect("open tag");
            let redundant = if tokens[after].is_kind(T_COALESCE) {
                is_coalesce_boundary(&tokens[before])
            } else if tokens[after].equals_any(&[",".into(), ")".into()], true)
                && tokens[before].equals_any(&["(".into(), ",".into()], true)
            {
                enclosing_paren(tokens, start)
                    .and_then(|o| tokens.get_prev_meaningful_token(o))
                    .is_some_and(|p| tokens[p].is_kind(T_ISSET))
            } else {
                false
            };
            if redundant {
                tokens.set(i, Token::new(T_OBJECT_OPERATOR, "->"));
            }
        }
    }
}

/// First token of the access chain ending just before `op` (`$a->b()['c']?->`, `Foo::bar()?->`, `(expr)?->`).
fn chain_start(tokens: &Tokens, op: usize) -> Option<usize> {
    let mut p = tokens.get_prev_meaningful_token(op)?;
    loop {
        let t = &tokens[p];
        let first = if t.equals(")", true) {
            let open = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, p);
            let callee = tokens.get_prev_meaningful_token(open)?;
            if tokens[callee].is_given_kind(&[T_STRING, T_VARIABLE])
                || tokens[callee].equals_any(&[")".into(), "]".into()], true)
            {
                p = callee;
                continue;
            }
            open
        } else if t.equals("]", true) {
            p = tokens
                .get_prev_meaningful_token(tokens.find_block_start(Tokens::BLOCK_TYPE_INDEX_BRACKET, p))
                .expect("open tag");
            continue;
        } else if t.is_given_kind(&[T_STRING, T_VARIABLE]) {
            p
        } else {
            return None;
        };
        match tokens.get_prev_meaningful_token(first) {
            Some(q)
                if tokens[q].is_given_kind(&[
                    T_OBJECT_OPERATOR,
                    T_NULLSAFE_OBJECT_OPERATOR,
                    T_DOUBLE_COLON,
                    T_NS_SEPARATOR,
                ]) =>
            {
                p = tokens.get_prev_meaningful_token(q)?;
                if tokens[q].is_kind(T_NS_SEPARATOR) && !tokens[p].is_kind(T_STRING) {
                    return Some(q); // leading `\`
                }
            }
            Some(q) if tokens[q].equals("$", true) || tokens[q].is_kind(T_NEW) => return None,
            _ => return Some(first),
        }
    }
}

/// Tokens after which an operand is the whole left side of `??` (everything of lower precedence).
fn is_coalesce_boundary(t: &Token) -> bool {
    t.equals_any(
        &[
            "(".into(),
            ",".into(),
            "[".into(),
            "{".into(),
            "}".into(),
            ";".into(),
            "=".into(),
            "?".into(),
            ":".into(),
        ],
        true,
    ) || t.is_given_kind(COMPOUND_ASSIGNMENTS)
        || t.is_given_kind(&[
        T_COALESCE,
        T_DOUBLE_ARROW,
        T_RETURN,
        T_ECHO,
        T_PRINT,
        T_YIELD,
        T_THROW,
        T_BOOLEAN_AND,
        T_BOOLEAN_OR,
        T_LOGICAL_AND,
        T_LOGICAL_OR,
        T_LOGICAL_XOR,
        T_OPEN_TAG,
        T_OPEN_TAG_WITH_ECHO,
        CT_ARRAY_BRACKET_OPEN,
        CT_NAMED_ARGUMENT_COLON,
    ])
}

/// The unmatched `(` around `k`, if no other bracket is in between.
fn enclosing_paren(tokens: &Tokens, k: usize) -> Option<usize> {
    let mut p = tokens.get_prev_meaningful_token(k)?;
    loop {
        match Tokens::detect_block_type(&tokens[p]) {
            Some((bt, false)) => p = tokens.find_block_start(bt, p),
            Some(_) => return tokens[p].equals("(", true).then_some(p),
            None => {}
        }
        p = tokens.get_prev_meaningful_token(p)?;
    }
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

    #[test]
    fn mago_cases() {
        // bad_example / good_example
        assert_eq!(
            fix(
                "<?php\n\n$name = $user?->name ?? 'Guest';\n\nif (isset($user?->profile)) {\n    // Do something with $user->profile\n}\n"
            ),
            "<?php\n\n$name = $user->name ?? 'Guest';\n\nif (isset($user->profile)) {\n    // Do something with $user->profile\n}\n"
        );
        let same = "<?php\n\n$name = $user->name ?? 'Guest';\n\nif (isset($user->profile)) {\n}\n";
        assert_eq!(fix(same), same);
    }

    #[test]
    fn edge_cases() {
        for (from, to) in [
            (
                "<?php return $a?->b?->c ?? null;",
                "<?php return $a?->b->c ?? null;",
            ),
            (
                "<?php foo($this->repo()['x']?->name ?? '', 1);",
                "<?php foo($this->repo()['x']->name ?? '', 1);",
            ),
            (
                "<?php $x = \\App\\user()?->id ?? 0;",
                "<?php $x = \\App\\user()->id ?? 0;",
            ),
            (
                "<?php $x = ['k' => $u?->id ?? 0];",
                "<?php $x = ['k' => $u->id ?? 0];",
            ),
            (
                "<?php $x = $c ? $u?->id ?? 0 : 1;",
                "<?php $x = $c ? $u->id ?? 0 : 1;",
            ),
            ("<?php isset($a, $u?->id);", "<?php isset($a, $u->id);"),
            (
                "<?php $x = ($a ?: $b)?->id ?? 0;",
                "<?php $x = ($a ?: $b)->id ?? 0;",
            ),
            (
                "<?php isset($a[0], $u?->id);",
                "<?php isset($a[0], $u->id);",
            ),
        ] {
            assert_eq!(fix(from), to);
        }
        for same in [
            "<?php $x = 'a' . $u?->id ?? 'c';",
            "<?php $x = !$u?->ok ?? false;",
            "<?php $x = $u?->id;",
            "<?php $x = $u?->name() ?? '';",
            "<?php $x = $u?->a->b ?? '';",
            "<?php $x = $u?->a['b'] ?? '';",
            "<?php isset($u?->a->b);",
            "<?php foo($u?->id);",
            "<?php isset($x[$u?->id]);",
            "<?php $x = $a == $u?->id ?? 0;",
            // Dynamic property names end the chain walk: skipped.
            "<?php $x = $a->{'b'}?->c ?? 0;",
            "<?php $x = $u?->$p ?? 0;",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
