//! Port of Mago's `no-self-assignment`, for variables: deletes `$a = $a;` statements.
//! Opt-in only (`Pun/no_self_assignment`). Property self-assignments (`$this->x = $this->x;`, which Mago also
//! deletes as "potentially unsafe") are left alone: `__set`, property hooks, Eloquent mutators and readonly
//! properties give them effects. The only effect dropped for variables is that `$a = $a;` on an undefined
//! `$a` warns and then defines it as `null`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoSelfAssignment;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoSelfAssignment)
}

impl Fixer for NoSelfAssignment {
    fn name(&self) -> &'static str {
        "Pun/no_self_assignment"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_VARIABLE)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (0..tokens.count()).rev() {
            if !tokens[i].is_kind(T_VARIABLE) || tokens[i].get_content() == b"$this" {
                continue;
            }
            let prev = tokens.get_prev_meaningful_token(i).expect("open tag");
            if !(tokens[prev].equals_any(&[";".into(), "{".into(), "}".into()], true)
                || tokens[prev].is_kind(T_OPEN_TAG))
            {
                continue;
            }
            let Some(eq) = tokens
                .get_next_meaningful_token(i)
                .filter(|&e| tokens[e].equals("=", true))
            else {
                continue;
            };
            let Some(rhs) = tokens.get_next_meaningful_token(eq).filter(|&r| {
                tokens[r].is_kind(T_VARIABLE) && tokens[r].get_content() == tokens[i].get_content()
            }) else {
                continue;
            };
            let Some(semi) = tokens
                .get_next_meaningful_token(rhs)
                .filter(|&s| tokens[s].equals(";", true))
            else {
                continue;
            };
            if (i..semi).any(|k| tokens[k].is_comment()) {
                continue;
            }
            tokens.clear_range(i, semi);
            // Drop the statement's line: merge the whitespace before and after it, keeping the later one.
            if i > 0
                && tokens[i - 1].is_whitespace(None)
                && semi + 1 < tokens.count()
                && tokens[semi + 1].is_whitespace(None)
            {
                let before = tokens[i - 1].get_content();
                let after = tokens[semi + 1].get_content();
                let keep = if after.contains(&b'\n') {
                    [
                        &before[..before.iter().rposition(|&c| c == b'\n').unwrap_or(0)],
                        after,
                    ]
                    .concat()
                } else {
                    before.to_vec()
                };
                tokens.clear_at(semi + 1);
                tokens.set(i - 1, Token::new(T_WHITESPACE, keep));
            }
        }
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
        // bad_example: only the variable case is fixed.
        assert_eq!(
            fix("<?php\n\n$a = $a;\n$this->x = $this->x;\n$foo->bar = $foo->bar;\n"),
            "<?php\n\n$this->x = $this->x;\n$foo->bar = $foo->bar;\n"
        );
        let same = "<?php\n\n$a = $b;\n$this->x = $other->x;\n$foo->bar = $baz->bar;\n";
        assert_eq!(fix(same), same);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php\nfunction f($a) {\n    $x = 1;\n    $a = $a;\n    return $a;\n}\n"),
            "<?php\nfunction f($a) {\n    $x = 1;\n    return $a;\n}\n"
        );
        assert_eq!(fix("<?php foo(); $a=$a; bar();"), "<?php foo(); bar();");
        assert_eq!(fix("<?php foo();\n$a = $a;"), "<?php foo();\n");
        for same in [
            "<?php if ($x) $a = $a;",
            "<?php $b = $a = $a;",
            "<?php $a = $a + 1;",
            "<?php $a = $a ?? null;",
            "<?php $a = $a[0];",
            "<?php $a = &$a;",
            "<?php $a = $A;",
            "<?php $a = /* keep */ $a;",
            "<?php for ($i = $i; $i < 3; $i++) {}",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
