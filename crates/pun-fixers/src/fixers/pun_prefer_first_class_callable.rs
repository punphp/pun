//! Port of Mago's `prefer-first-class-callable` (default settings, so methods only):
//! `fn ($x) => $this->format($x)` → `$this->format(...)`, `function ($x) { return Str::upper($x); }` → `Str::upper(...)`.
//! Opt-in only (`Pun/prefer_first_class_callable`). Risky (Mago marks the fix unsafe too): the callable now
//! receives every argument its caller passes, e.g. Laravel collections pass the key as a second argument,
//! which reaches an optional second parameter of the method. Deviations from Mago, which change behaviour:
//! - closures/arrow functions with a return type, by-reference return, by-reference or defaulted parameters,
//!   or parameter attributes are skipped (the coercion/defaults would be lost);
//! - a variadic parameter must be forwarded as `...$args` (Mago also accepts `fn (...$a) => f($a)`);
//! - a closure calling a method on a variable it doesn't `use` by value is skipped (the closure would fail,
//!   the first-class callable would not).
use crate::analyzers::helpers::split_list;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PreferFirstClassCallable;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PreferFirstClassCallable)
}

impl Fixer for PreferFirstClassCallable {
    fn name(&self) -> &'static str {
        "Pun/prefer_first_class_callable"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_FN, T_FUNCTION])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (0..tokens.count()).rev() {
            if tokens[i].is_given_kind(&[T_FN, T_FUNCTION]) {
                fix_one(tokens, i);
            }
        }
    }
}

fn fix_one(tokens: &mut Tokens, kw: usize) -> Option<()> {
    let is_arrow = tokens[kw].is_kind(T_FN);
    let mut start = kw;
    if let Some(p) = tokens
        .get_prev_meaningful_token(kw)
        .filter(|&p| tokens[p].is_kind(T_STATIC))
    {
        start = p;
    }
    if tokens
        .get_prev_meaningful_token(start)
        .is_some_and(|p| tokens[p].is_kind(CT_ATTRIBUTE_CLOSE))
    {
        return None;
    }
    let popen = tokens
        .get_next_meaningful_token(kw)
        .filter(|&o| tokens[o].equals("(", true))?;
    let pclose = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, popen);
    let params = split_list(tokens, popen, pclose)
        .into_iter()
        .map(|(s, e)| {
            if tokens[e].is_kind(T_VARIABLE)
                && !(s..e).any(|k| {
                    tokens[k].is_given_kind(&[
                        T_ATTRIBUTE,
                        T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG,
                        CT_CONSTRUCTOR_PROPERTY_PROMOTION_PUBLIC,
                    ]) || tokens[k].equals("&", true)
                })
            {
                let variadic = tokens
                    .get_prev_meaningful_token(e)
                    .is_some_and(|p| tokens[p].is_kind(T_ELLIPSIS));
                Some((tokens[e].get_content().to_vec(), variadic))
            } else {
                None
            }
        })
        .collect::<Option<Vec<_>>>()?;
    let mut k = tokens.get_next_meaningful_token(pclose)?;
    let mut use_clause = None;
    if tokens[k].is_kind(CT_USE_LAMBDA) {
        let uopen = tokens.get_next_meaningful_token(k)?;
        let uclose = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, uopen);
        use_clause = Some((uopen, uclose));
        k = tokens.get_next_meaningful_token(uclose)?;
    }
    let body_start = if is_arrow {
        tokens[k].is_kind(T_DOUBLE_ARROW).then_some(k)?
    } else {
        tokens[k].equals("{", true).then_some(k)?
    };
    let mut c = tokens.get_next_meaningful_token(body_start)?;
    if !is_arrow {
        c = tokens[c]
            .is_kind(T_RETURN)
            .then(|| tokens.get_next_meaningful_token(c))??;
    }
    let call_start = c;
    // `$var->method(` or `Name::method(` / `static::method(`
    let callee = if tokens[c].is_kind(T_VARIABLE) {
        let arrow = tokens
            .get_next_meaningful_token(c)
            .filter(|&a| tokens[a].is_kind(T_OBJECT_OPERATOR))?;
        c = arrow;
        Some(tokens[call_start].get_content().to_vec())
    } else {
        if tokens[c].is_kind(T_NS_SEPARATOR) {
            c = tokens.get_next_meaningful_token(c)?;
        }
        if !tokens[c].is_given_kind(&[T_STRING, T_STATIC]) {
            return None;
        }
        while let Some(n) = tokens
            .get_next_meaningful_token(c)
            .filter(|&n| tokens[n].is_kind(T_NS_SEPARATOR))
        {
            c = tokens
                .get_next_meaningful_token(n)
                .filter(|&s| tokens[s].is_kind(T_STRING))?;
        }
        c = tokens
            .get_next_meaningful_token(c)
            .filter(|&d| tokens[d].is_kind(T_DOUBLE_COLON))?;
        None
    };
    let method = tokens
        .get_next_meaningful_token(c)
        .filter(|&m| tokens[m].is_kind(T_STRING))?;
    let aopen = tokens
        .get_next_meaningful_token(method)
        .filter(|&o| tokens[o].equals("(", true))?;
    let aclose = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, aopen);
    let args = split_list(tokens, aopen, aclose);
    if args.len() != params.len() {
        return None;
    }
    for (&(s, e), (name, variadic)) in args.iter().zip(&params) {
        let spread = tokens[s].is_kind(T_ELLIPSIS);
        if !tokens[e].is_kind(T_VARIABLE)
            || tokens[e].get_content() != name.as_slice()
            || spread != *variadic
            || (spread && tokens.get_next_meaningful_token(s) != Some(e))
            || (!spread && s != e)
        {
            return None;
        }
    }
    let after = tokens.get_next_meaningful_token(aclose)?;
    let end = if is_arrow {
        let t = &tokens[after];
        if !(t.equals_any(&[",".into(), ")".into(), ";".into(), "]".into()], true)
            || t.is_given_kind(&[CT_ARRAY_BRACKET_CLOSE, T_CLOSE_TAG]))
        {
            return None;
        }
        aclose
    } else {
        if !tokens[after].equals(";", true) {
            return None;
        }
        let brace = tokens
            .get_next_meaningful_token(after)
            .filter(|&b| tokens[b].equals("}", true))?;
        if let Some(var) = callee.as_ref().filter(|v| v.as_slice() != b"$this") {
            let (uopen, uclose) = use_clause?;
            let captured = (uopen..uclose).any(|u| {
                tokens[u].is_kind(T_VARIABLE)
                    && tokens[u].get_content() == var.as_slice()
                    && !tokens.get_prev_meaningful_token(u).is_some_and(|p| {
                        tokens[p].equals("&", true)
                            || tokens[p].is_kind(T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG)
                    })
            });
            if !captured {
                return None;
            }
        }
        brace
    };
    if (start..=end).any(|k| tokens[k].is_comment()) {
        return None;
    }
    if end != aclose {
        tokens.clear_range(aclose + 1, end);
    }
    tokens.override_range(
        aopen,
        aclose,
        vec![
            Token::char("("),
            Token::new(CT_FIRST_CLASS_CALLABLE, "..."),
            Token::char(")"),
        ],
    );
    tokens.clear_range(start, call_start - 1);
    Some(())
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
        // simple_method_call, simple_static_method_call, closure_method_call_with_non_callee_reference_capture
        assert_eq!(
            fix("<?php\n\nrun(fn($x) => $this->method($x));\n"),
            "<?php\n\nrun($this->method(...));\n"
        );
        assert_eq!(
            fix("<?php\n\nrun(fn($x) => SomeClass::method($x));\n"),
            "<?php\n\nrun(SomeClass::method(...));\n"
        );
        assert_eq!(
            fix(
                "<?php\n\n$unused = null;\n$obj = new SomeClass();\nrun(function($x) use (&$unused, $obj) { return $obj->method($x); });\n"
            ),
            "<?php\n\n$unused = null;\n$obj = new SomeClass();\nrun($obj->method(...));\n"
        );
        for same in [
            "<?php\n\nfunction getSomeClass() { return new SomeClass(); }\nrun(fn() => getSomeClass()->method());\n",
            "<?php\n\nfunction getSomeClass() { return new SomeClass(); }\nrun(fn($x) => getSomeClass()->method($x));\n",
            "<?php\n\n$someClass = new SomeClass();\nrun(fn($x) => $someClass?->method($x));\n",
            "<?php\n\n$someClass = new SomeClass();\n$method = \"method\";\nrun(fn($x) => $someClass->$method($x));\n",
            "<?php\n\nfunction getSomeClass() { return new SomeClass(); }\nrun(function($x) { return getSomeClass()->method($x); });\n",
            "<?php\n\n$callable = \"strlen\";\nrun(function($x) use (&$callable) { return $callable($x); });\n",
            "<?php\n\n$obj = new SomeClass();\nrun(function($x) use (&$obj) { return $obj->method($x); });\n",
            // check_functions is off by default
            "<?php\n\nrun(fn($x) => strlen($x));\n",
            "<?php\n\n$unused = null;\nrun(function($x) use (&$unused) { return strlen($x); });\n",
            "<?php\n\narray_map(static fn(array $nums): int => Calculator::sum(...$nums), $groups);\n",
            "<?php\n\narray_map(function(array $nums) { return Calculator::sum(...$nums); }, $groups);\n",
        ] {
            assert_eq!(fix(same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php $users->map(fn (User $u) => $this->present($u));"),
            "<?php $users->map($this->present(...));"
        );
        assert_eq!(
            fix("<?php $f = static fn (...$a) => \\App\\Support\\Str::join(...$a);"),
            "<?php $f = \\App\\Support\\Str::join(...);"
        );
        assert_eq!(
            fix("<?php $f = [fn ($a, $b) => static::cmp($a, $b,)];"),
            "<?php $f = [static::cmp(...)];"
        );
        assert_eq!(
            fix("<?php $f = function ($x) {\n    return $this->handle($x);\n};"),
            "<?php $f = $this->handle(...);"
        );
        for same in [
            "<?php $f = fn ($x): string => $this->m($x);",
            "<?php $f = fn ($x = 1) => $this->m($x);",
            "<?php $f = fn (&$x) => $this->m($x);",
            "<?php $f = fn &($x) => $this->m($x);",
            "<?php $f = fn ($x, $y) => $this->m($y, $x);",
            "<?php $f = fn ($x) => $this->m($x, 1);",
            "<?php $f = fn ($x) => $this->m($x) ?? 1;",
            "<?php $f = fn ($x) => $this->m($x)->n();",
            "<?php $f = fn (...$a) => $this->m($a);",
            "<?php $f = function ($x) { return $obj->m($x); };",
            "<?php $f = function ($x) { $y = 1; return $this->m($x); };",
            "<?php $f = function ($x) { return $this->m(/* c */ $x); };",
            "<?php $f = #[Pure] fn ($x) => $this->m($x);",
            "<?php class A { public function m($x) { return $this->n($x); } }",
            "<?php $f = fn ($x) => (new Foo)->m($x);",
            "<?php $f = fn ($x) => $this->m(f($x));",
            "<?php $f = function ($x) { return $this->m($x) + 1; };",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
