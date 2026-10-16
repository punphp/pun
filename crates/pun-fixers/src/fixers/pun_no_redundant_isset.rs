//! Port of Mago's `no-redundant-isset`: `isset($a, $a['b'])` → `isset($a['b'])`. An argument is redundant when
//! another argument extends its access path (`isset($a['b'])` already implies `isset($a)`), or repeats it.
//! Opt-in only (`Pun/no_redundant_isset`). Like Mago, only paths of plain variables, `->name` / `?->name`
//! and literal string or integer `[...]` indexes are compared.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::Tokens;
use std::path::Path;

struct NoRedundantIsset;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoRedundantIsset)
}

impl Fixer for NoRedundantIsset {
    fn name(&self) -> &'static str {
        "Pun/no_redundant_isset"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_ISSET)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for i in (0..tokens.count()).rev() {
            if !tokens[i].is_kind(T_ISSET) {
                continue;
            }
            let Some(open) = tokens.get_next_meaningful_token(i).filter(|&o| tokens[o].equals("(", true)) else { continue };
            let close = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
            let args = split_list(tokens, open, close);
            if args.len() < 2 || (open..=close).any(|k| tokens[k].is_comment()) {
                continue;
            }
            let paths: Vec<Option<Vec<Vec<u8>>>> = args.iter().map(|&(s, e)| access_path(tokens, s, e)).collect();
            let redundant: Vec<bool> = (0..args.len())
                .map(|a| {
                    let Some(pa) = &paths[a] else { return false };
                    paths.iter().enumerate().any(|(b, pb)| {
                        pb.as_ref().is_some_and(|pb| (b != a && pa.len() < pb.len() && pb.starts_with(pa)) || (b > a && pa == pb))
                    })
                })
                .collect();
            let n = args.len();
            let mut a = n;
            while a > 0 {
                a -= 1;
                if !redundant[a] {
                    continue;
                }
                let ge = a + 1;
                while a > 0 && redundant[a - 1] {
                    a -= 1;
                }
                if ge < n {
                    tokens.clear_range(args[a].0, args[ge].0 - 1);
                } else {
                    tokens.clear_range(args[a - 1].1 + 1, args[ge - 1].1);
                }
            }
        }
    }
}

/// `$foo->bar['baz']` → `["$foo", "->bar", "['baz']"]`; `None` for anything else (dynamic names or indexes).
fn access_path(tokens: &Tokens, s: usize, e: usize) -> Option<Vec<Vec<u8>>> {
    if !tokens[s].is_kind(T_VARIABLE) {
        return None;
    }
    let mut path = vec![tokens[s].get_content().to_vec()];
    let mut k = s;
    while k != e {
        let op = tokens.get_next_meaningful_token(k)?;
        let next = tokens.get_next_meaningful_token(op)?;
        if tokens[op].is_given_kind(&[T_OBJECT_OPERATOR, T_NULLSAFE_OBJECT_OPERATOR]) && tokens[next].is_kind(T_STRING) {
            path.push([tokens[op].get_content(), tokens[next].get_content()].concat());
            k = next;
        } else if tokens[op].equals("[", true) && tokens[next].is_given_kind(&[T_CONSTANT_ENCAPSED_STRING, T_LNUMBER]) {
            let close = tokens.get_next_meaningful_token(next).filter(|&c| tokens[c].equals("]", true))?;
            path.push([b"[", tokens[next].get_content(), b"]"].concat());
            k = close;
        } else {
            return None;
        }
        if k > e {
            return None;
        }
    }
    Some(path)
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
        for (from, to) in [
            ("<?php isset($a[0], $a[0][1]);", "<?php isset($a[0][1]);"),
            ("<?php isset($d, $d['first']);", "<?php isset($d['first']);"),
            ("<?php isset($d, $d['first'], $d['first']['second']);", "<?php isset($d['first']['second']);"),
            ("<?php isset($d['first']['second'], $d['first'], $d);", "<?php isset($d['first']['second']);"),
            ("<?php isset($o, $o->foo);", "<?php isset($o->foo);"),
            ("<?php isset($d, $d['first']['second']);", "<?php isset($d['first']['second']);"),
            ("<?php isset($d, $d);", "<?php isset($d);"),
            ("<?php isset($foo, $foo->bar, $foo->bar->baz, $foo->bar->baz['x'], $foo->bar->baz['x']['y']);", "<?php isset($foo->bar->baz['x']['y']);"),
        ] {
            assert_eq!(fix(from), to);
        }
        for same in [
            "<?php isset($a);",
            "<?php isset($a, $b);",
            "<?php isset($d['a'], $d['b']);",
            "<?php isset($a['x'], $b['x']);",
            "<?php isset($a->x, $a->y);",
            "<?php isset($a->{$x}, $a->{$x}->y);",
            "<?php isset($a[$x], $a[$x]['y']);",
        ] {
            assert_eq!(fix(same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(fix("<?php isset(\n    $a,\n    $b,\n    $a['x'],\n);"), "<?php isset(\n    $b,\n    $a['x'],\n);");
        assert_eq!(fix("<?php isset($a['x'], $b, $a);"), "<?php isset($a['x'], $b);");
        assert_eq!(fix("<?php isset($a, $b, $a->c, $b->d);"), "<?php isset($a->c, $b->d);");
        assert_eq!(fix("<?php isset($a, $a?->b->c, $a::$d);"), "<?php isset($a?->b->c, $a::$d);");
        for same in [
            "<?php isset($a['x'], $a[\"x\"]);",
            "<?php isset($a, /* c */ $a['b']);",
            "<?php isset($a, $a->b());",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
