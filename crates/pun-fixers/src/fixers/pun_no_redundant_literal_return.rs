//! Port of Mago's `no-redundant-literal-return`: `if ($x === null) { return null; } return $x;` → `return $x;`
//! (also `elseif ($x === <lit>) { return <lit>; }` arms and an `else { return $x; }`).
//! Opt-in only (`Pun/no_redundant_literal_return`). Deviation from Mago: float literals are not treated as
//! literals, because `-0.0 === 0.0` holds while returning `$x` would return `-0.0`. Alternative syntax
//! (`if (...):`) and guards containing comments are skipped.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoRedundantLiteralReturn;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoRedundantLiteralReturn)
}

impl Fixer for NoRedundantLiteralReturn {
    fn name(&self) -> &'static str {
        "Pun/no_redundant_literal_return"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_IF, T_RETURN, T_IS_IDENTICAL])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (0..tokens.count()).rev() {
            if tokens[i].is_kind(T_IF) {
                fix_if(tokens, i);
            }
        }
    }
}

fn fix_if(tokens: &mut Tokens, i: usize) -> Option<()> {
    let prev = tokens.get_prev_meaningful_token(i)?;
    if !(tokens[prev].equals_any(&[";".into(), "{".into(), "}".into(), ":".into()], true)
        || tokens[prev].is_kind(T_OPEN_TAG))
    {
        return None;
    }
    let mut k = i;
    let mut var: Option<Vec<u8>> = None;
    // `if` and every `elseif`: `(<var> === <lit>)` then a body that only returns `<lit>`.
    loop {
        let open = tokens
            .get_next_meaningful_token(k)
            .filter(|&o| tokens[o].equals("(", true))?;
        let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
        let (v, lit) = condition(tokens, open, close)?;
        if var.as_ref().is_some_and(|w| *w != v) {
            return None;
        }
        var = Some(v);
        let (ret, end) = return_body(tokens, close)?;
        if literal(tokens, &ret)? != lit {
            return None;
        }
        k = tokens.get_next_meaningful_token(end)?;
        if !tokens[k].is_kind(T_ELSEIF) {
            break;
        }
    }
    let var = var?;
    let is_var = |r: &[usize], tokens: &Tokens| {
        r.len() == 1
            && tokens[r[0]].is_kind(T_VARIABLE)
            && tokens[r[0]].get_content() == var.as_slice()
    };
    if tokens[k].is_kind(T_ELSE) {
        let (ret, end) = return_body(tokens, k)?;
        if !is_var(&ret, tokens) || (i..=end).any(|j| tokens[j].is_comment()) {
            return None;
        }
        tokens.override_range(
            i,
            end,
            vec![
                Token::new(T_RETURN, "return"),
                Token::new(T_WHITESPACE, " "),
                Token::new(T_VARIABLE, var),
                Token::char(";"),
            ],
        );
    } else {
        // The next statement must be `return <var>;`.
        if !tokens[k].is_kind(T_RETURN) || (i..k).any(|j| tokens[j].is_comment()) {
            return None;
        }
        let v = tokens.get_next_meaningful_token(k)?;
        tokens
            .get_next_meaningful_token(v)
            .filter(|&s| tokens[s].equals(";", true))?;
        if !is_var(&[v], tokens) {
            return None;
        }
        tokens.clear_range(i, k - 1);
    }
    Some(())
}

/// `(<var> === <lit>)` or `(<lit> === <var>)`: the variable name and the normalized literal.
fn condition(tokens: &Tokens, open: usize, close: usize) -> Option<(Vec<u8>, Vec<Vec<u8>>)> {
    let inner: Vec<usize> = (open + 1..close)
        .filter(|&k| !tokens[k].is_whitespace(None))
        .collect();
    let op = inner
        .iter()
        .position(|&k| tokens[k].is_kind(T_IS_IDENTICAL))?;
    let (lhs, rhs) = (&inner[..op], &inner[op + 1..]);
    let is_var = |s: &[usize]| s.len() == 1 && tokens[s[0]].is_kind(T_VARIABLE);
    let (v, lit) = if is_var(lhs) {
        (lhs, rhs)
    } else if is_var(rhs) {
        (rhs, lhs)
    } else {
        return None;
    };
    Some((tokens[v[0]].get_content().to_vec(), literal(tokens, lit)?))
}

/// After `after` (a condition's `)` or `else`): `{ return <expr>; }` or `return <expr>;`.
/// Returns the expression's non-whitespace tokens and the body's last token.
fn return_body(tokens: &Tokens, after: usize) -> Option<(Vec<usize>, usize)> {
    let mut k = tokens.get_next_meaningful_token(after)?;
    let braced = tokens[k].equals("{", true);
    if braced {
        k = tokens.get_next_meaningful_token(k)?;
    }
    if !tokens[k].is_kind(T_RETURN) {
        return None;
    }
    let mut expr = vec![];
    loop {
        k = tokens.get_next_meaningful_token(k)?;
        if tokens[k].equals(";", true) {
            break;
        }
        if let Some((bt, true)) = Tokens::detect_block_type(&tokens[k]) {
            let end = tokens.find_block_end(bt, k);
            expr.extend((k..=end).filter(|&j| !tokens[j].is_whitespace(None)));
            k = end;
        } else {
            expr.push(k);
        }
    }
    if braced {
        k = tokens
            .get_next_meaningful_token(k)
            .filter(|&b| tokens[b].equals("}", true))?;
    }
    (!expr.is_empty()).then_some((expr, k))
}

/// Normalized literal (null/true/false, string, integer, arrays of those); `None` for anything else.
fn literal(tokens: &Tokens, ks: &[usize]) -> Option<Vec<Vec<u8>>> {
    let mut out: Vec<Vec<u8>> = vec![];
    let mut parens = 0usize;
    let mut after_array = false;
    for &k in ks {
        let t = &tokens[k];
        let c = t.get_content();
        let norm = if t.is_kind(T_CONSTANT_ENCAPSED_STRING) {
            let inner = &c[1..c.len() - 1];
            if (c[0] == b'\'' && !inner.contains(&b'\\'))
                || (c[0] == b'"' && !inner.iter().any(|&b| b == b'\\' || b == b'$'))
            {
                [b"s:", inner].concat()
            } else {
                [b"r:", c].concat()
            }
        } else if t.is_kind(T_LNUMBER) {
            [b"i:", c].concat()
        } else if t.is_kind(T_STRING)
            && [&b"null"[..], b"true", b"false"]
                .iter()
                .any(|w| c.eq_ignore_ascii_case(w))
        {
            c.to_ascii_lowercase()
        } else if t.is_kind(T_ARRAY) {
            after_array = true;
            continue;
        } else if t.equals("(", true) && after_array {
            parens += 1;
            b"[".to_vec()
        } else if t.equals(")", true) && parens > 0 {
            parens -= 1;
            b"]".to_vec()
        } else if t.is_kind(CT_ARRAY_BRACKET_OPEN) {
            b"[".to_vec()
        } else if t.is_kind(CT_ARRAY_BRACKET_CLOSE) {
            b"]".to_vec()
        } else if t.equals(",", true) || t.is_kind(T_DOUBLE_ARROW) {
            c.to_vec()
        } else {
            return None;
        };
        after_array = false;
        if norm == b"]" && out.last().is_some_and(|l| l == b",") {
            out.pop();
        }
        out.push(norm);
    }
    (!out.is_empty() && !after_array).then_some(out)
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

    fn func(body: &str) -> String {
        format!("<?php\n\nfunction getValue($x) {{\n{body}\n}}\n")
    }

    #[test]
    fn mago_cases() {
        let ret = func("    return $x;");
        for (cond, lit) in [
            ("$x === null", "null"),
            ("null === $x", "null"),
            ("$x === true", "true"),
            ("$x === false", "false"),
            ("$x === 0", "0"),
            ("$x === 'default'", "'default'"),
            ("$x === []", "[]"),
            ("$x === array()", "array()"),
            ("$x === [1, 2, 3]", "[1, 2, 3]"),
        ] {
            assert_eq!(
                fix(&func(&format!(
                    "    if ({cond}) {{\n        return {lit};\n    }}\n    return $x;"
                ))),
                ret
            );
        }
        // with_else_clause, with_elseif_and_return, with_elseif_and_else, multiple_elseif_clauses
        assert_eq!(
            fix(&func(
                "    if ($x === null) {\n        return null;\n    } else {\n        return $x;\n    }"
            )),
            ret
        );
        assert_eq!(
            fix(&func(
                "    if ($x === null) {\n        return null;\n    } elseif ($x === '') {\n        return '';\n    }\n    return $x;"
            )),
            ret
        );
        assert_eq!(
            fix(&func(
                "    if ($x === null) {\n        return null;\n    } elseif ($x === '') {\n        return '';\n    } else {\n        return $x;\n    }"
            )),
            ret
        );
        assert_eq!(
            fix(&func(
                "    if ($x === null) {\n        return null;\n    } elseif ($x === '') {\n        return '';\n    } elseif ($x === 0) {\n        return 0;\n    }\n    return $x;"
            )),
            ret
        );
        for same in [
            func("    if ($x === null) {\n        return 'default';\n    }\n    return $x;"),
            func("    if ($x === null) {\n        return null;\n    } else {\n        return 'something else';\n    }"),
            "<?php\n\nfunction getValue($x, $y) {\n    if ($x === null) {\n        return null;\n    } elseif ($y === '') {\n        return '';\n    }\n    return $x;\n}\n".to_string(),
            func("    if ($x == null) {\n        return null;\n    }\n    return $x;"),
            func("    if ($x === null) {\n        log('returning null');\n        return null;\n    }\n    return $x;"),
            "<?php\n\nfunction getValue($x, $y) {\n    if ($x === null) {\n        return null;\n    }\n    return $y;\n}\n".to_string(),
            func("    if ($x === null) {\n        return null;\n    }\n    echo $x;"),
            func("    if ($x === null) {\n        return null;\n    } elseif ($x === '') {\n        return 'empty';\n    }\n    return $x;"),
        ] {
            assert_eq!(fix(&same), same);
        }
    }

    #[test]
    fn edge_cases() {
        let ret = func("    return $x;");
        assert_eq!(
            fix(&func("    if ($x === NULL) return null;\n    return $x;")),
            ret
        );
        assert_eq!(
            fix(&func(
                "    if ($x === ['a' => \"b\",]) {\n        return ['a' => 'b'];\n    }\n    return $x;"
            )),
            ret
        );
        assert_eq!(
            fix(&func(
                "    if ($x === [1]) {\n        return array(1);\n    }\n    return $x;"
            )),
            ret
        );
        assert_eq!(
            fix(&func(
                "    if ($x === \"a\\n\") {\n        return \"a\\n\";\n    }\n    return $x;"
            )),
            ret
        );
        for same in [
            func("    if ($x === 0.0) {\n        return 0.0;\n    }\n    return $x;"),
            func("    if ($x === -1) {\n        return -1;\n    }\n    return $x;"),
            func("    if ($x === FOO) {\n        return FOO;\n    }\n    return $x;"),
            func("    if ($x === null):\n        return null;\n    endif;\n    return $x;"),
            func("    if ($x === null) {\n        return null; // why\n    }\n    return $x;"),
            func(
                "    if ($x === null) {\n        return null;\n    } else if ($x === '') {\n        return '';\n    }\n    return $x;",
            ),
            func("    if ($x === null) {\n        return null;\n    }\n    return $x + 1;"),
            func("    if ($x->y === null) {\n        return null;\n    }\n    return $x;"),
            // Escaped strings are compared by their source text only, so equal values spelled differently are kept.
            func("    if ($x === 'a\\'b') {\n        return \"a'b\";\n    }\n    return $x;"),
        ] {
            assert_eq!(fix(&same), same);
        }
    }
}
