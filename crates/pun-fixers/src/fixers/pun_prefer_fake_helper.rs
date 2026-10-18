//! Port of Mago's `prefer-fake-helper` (Laravel): `$this->faker` → `fake()` inside `*Factory` classes.
//! Opt-in only (`Pun/prefer_fake_helper`). Like Mago, occurrences inside string interpolation are left alone;
//! unlike Mago, write contexts (`$this->faker = ...`, `isset($this->faker)`) are skipped too, since
//! `fake()` there would not compile.
use crate::analyzers::helpers::COMPOUND_ASSIGNMENTS;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PreferFakeHelper;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PreferFakeHelper)
}

impl Fixer for PreferFakeHelper {
    fn name(&self) -> &'static str {
        "Pun/prefer_fake_helper"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_CLASS, T_VARIABLE, T_OBJECT_OPERATOR])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut i = 0;
        while i < tokens.count() {
            if let Some(open) = class_body(tokens, i) {
                let named_factory = tokens.get_next_meaningful_token(i).is_some_and(|n| {
                    tokens[n].is_kind(T_STRING) && tokens[n].get_content().ends_with(b"Factory")
                });
                let end = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, open);
                if named_factory {
                    fix_body(tokens, open, end);
                }
                i = end;
            }
            i += 1;
        }
    }
}

fn fix_body(tokens: &mut Tokens, open: usize, end: usize) {
    let mut in_string = false;
    let mut i = open + 1;
    while i < end {
        let t = &tokens[i];
        if t.equals("\"", true) || t.equals("`", true) {
            in_string = !in_string;
        } else if t.is_kind(T_START_HEREDOC) {
            in_string = true;
        } else if t.is_kind(T_END_HEREDOC) {
            in_string = false;
        } else if let Some(o) = class_body(tokens, i) {
            // Nested anonymous class: `$this` is not the factory there.
            i = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, o);
        } else if !in_string && t.is_kind(T_VARIABLE) && t.get_content() == b"$this" {
            try_fix(tokens, i);
        }
        i += 1;
    }
}

fn try_fix(tokens: &mut Tokens, this: usize) {
    let Some(arrow) = tokens
        .get_next_meaningful_token(this)
        .filter(|&a| tokens[a].is_kind(T_OBJECT_OPERATOR))
    else {
        return;
    };
    let Some(prop) = tokens
        .get_next_meaningful_token(arrow)
        .filter(|&p| tokens[p].is_kind(T_STRING) && tokens[p].get_content() == b"faker")
    else {
        return;
    };
    let next = tokens.get_next_meaningful_token(prop);
    if next.is_some_and(|n| {
        let t = &tokens[n];
        t.equals_any(&["(".into(), "[".into(), "=".into()], true)
            || t.is_given_kind(&[T_INC, T_DEC])
            || t.is_given_kind(COMPOUND_ASSIGNMENTS)
    }) {
        return;
    }
    let p = tokens
        .get_prev_meaningful_token(this)
        .expect("inside a class body");
    if tokens[p].equals("&", true)
        || tokens[p].is_given_kind(&[T_INC, T_DEC])
        || inside_isset_or_unset(tokens, this)
    {
        return;
    }
    if (this..prop).any(|k| tokens[k].is_comment()) {
        return;
    }
    tokens.override_range(
        this,
        prop,
        vec![
            Token::new(T_STRING, "fake"),
            Token::char("("),
            Token::char(")"),
        ],
    );
}

/// The `{` of a class declaration or anonymous class at `i`; `None` for anything else, including `Foo::class`.
fn class_body(tokens: &Tokens, i: usize) -> Option<usize> {
    if !tokens[i].is_kind(T_CLASS)
        || tokens
            .get_prev_meaningful_token(i)
            .is_some_and(|p| tokens[p].is_kind(T_DOUBLE_COLON))
    {
        return None;
    }
    tokens.get_next_token_of_kind(i, &["{".into()], true)
}

/// Walks back from `index` (inside a class body, so a `{` is always found) to its statement start.
fn inside_isset_or_unset(tokens: &Tokens, index: usize) -> bool {
    let mut depth = 0;
    let mut i = index;
    loop {
        let p = tokens
            .get_prev_meaningful_token(i)
            .expect("inside a class body");
        let t = &tokens[p];
        if t.equals(")", true) {
            depth += 1;
        } else if t.equals("(", true) {
            if depth == 0 {
                return tokens
                    .get_prev_meaningful_token(p)
                    .is_some_and(|k| tokens[k].is_given_kind(&[T_ISSET, T_UNSET]));
            }
            depth -= 1;
        } else if depth == 0 && t.equals_any(&[";".into(), "{".into(), "}".into()], true) {
            return false;
        }
        i = p;
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

    fn factory(body: &str) -> String {
        format!(
            "<?php\n\nclass UserFactory\n{{\n    public function definition(): array\n    {{\n        {body}\n    }}\n}}\n"
        )
    }

    #[test]
    fn mago_cases() {
        // fix_faker_property
        assert_eq!(
            fix(&factory(
                "return ['name' => $this->faker->unique()->word()];"
            )),
            factory("return ['name' => fake()->unique()->word()];")
        );
        // fake_helper_used, faker_property_outside_factory, interpolation is reported but not fixed
        for same in [
            factory("return ['name' => fake()->word()];"),
            "<?php\n\nclass ReportService\n{\n    public function build(): string\n    {\n        return $this->faker->word();\n    }\n}\n".into(),
            factory(r#"return ['url' => "https://{$this->faker->domainName}"];"#),
            factory(r#"return ['url' => "host: $this->faker"];"#),
        ] {
            assert_eq!(fix(&same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix(&factory(
                "return ['a' => fn () => $this->faker->name(), 'b' => \"x\", 'c' => $this->faker->email];"
            )),
            factory("return ['a' => fn () => fake()->name(), 'b' => \"x\", 'c' => fake()->email];")
        );
        for same in [
            factory("$this->faker = null;"),
            factory("isset($this->faker);"),
            factory("$this->faker();"),
            factory("return new class { function f() { return $this->faker->name(); } };"),
            "<?php trait UserFactory { function f() { return $this->faker->name(); } }".into(),
            factory("return <<<EOT\n    {$this->faker->name}\n    EOT;"),
            factory("return $this-> /* keep */ faker->name();"),
            factory("return [$this, $this->other];"),
        ] {
            assert_eq!(fix(&same), same);
        }
        // A class after the factory is not touched.
        let two = "<?php class AFactory { function f() { $this->faker->a(); } } class B { function f() { $this->faker->b(); } }";
        assert_eq!(fix(two), two.replacen("$this->faker->a", "fake()->a", 1));
        // `::class` is not a class declaration (it used to swallow everything up to the next `{`).
        let refs = "<?php $f = AFactory::class; class AFactory { function f() { $m = B::class; if ($m) { $this->faker->a(); } } }";
        assert_eq!(fix(refs), refs.replacen("$this->faker->a", "fake()->a", 1));
    }
}
