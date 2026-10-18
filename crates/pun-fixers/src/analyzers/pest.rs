//! Token helpers shared by the Pest `Pun/*` rules (Pun's own; Mago's `rule/utils/pest.rs` works on its AST).
use pun_tokens::Kind::*;
use pun_tokens::Tokens;

/// Whether the name at `name` is called as a global function: not a method or static call, a declaration or
/// `new`, and not qualified (`Foo\name`, `namespace\name`). `\name` counts.
pub(crate) fn is_global_function_call(tokens: &Tokens, name: usize) -> bool {
    tokens.get_prev_meaningful_token(name).is_none_or(|b| {
        !tokens[b].is_given_kind(&[
            T_OBJECT_OPERATOR,
            T_NULLSAFE_OBJECT_OPERATOR,
            T_DOUBLE_COLON,
            T_FUNCTION,
            T_NEW,
        ]) && !(tokens[b].is_kind(T_NS_SEPARATOR)
            && tokens
                .get_prev_meaningful_token(b)
                .is_some_and(|q| tokens[q].is_given_kind(&[T_STRING, T_NAMESPACE])))
    })
}

/// Walks `root(...)->a()->b->` back from the `->` at `arrow` to its root, and returns the root as (name, `(`, `)`)
/// when it is a global function call.
pub(crate) fn chain_root_call(tokens: &Tokens, arrow: usize) -> Option<(usize, usize, usize)> {
    let is_arrow =
        |k: usize| tokens[k].is_given_kind(&[T_OBJECT_OPERATOR, T_NULLSAFE_OBJECT_OPERATOR]);
    let mut p = tokens.get_prev_meaningful_token(arrow).expect("open tag");
    loop {
        let (name, open) = if tokens[p].equals(")", true) {
            let open = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, p);
            let name = tokens
                .get_prev_meaningful_token(open)
                .filter(|&n| tokens[n].is_kind(T_STRING))?;
            (name, Some(open))
        } else if tokens[p].is_kind(T_STRING) {
            (p, None)
        } else {
            return None;
        };
        match tokens.get_prev_meaningful_token(name) {
            // An arrow always has at least the open tag before it.
            Some(b) if is_arrow(b) => p = tokens.get_prev_meaningful_token(b).expect("open tag"),
            _ => {
                return open
                    .filter(|_| is_global_function_call(tokens, name))
                    .map(|o| (name, o, p));
            }
        }
    }
}

pub(crate) struct ExpectMatcher {
    pub expect_open: usize,
    pub expect_close: usize,
    pub has_not: bool,
    pub close: usize,
}

/// Matches `expect(<arg>)[->not]-><method>()` with `method` at `i`: the matcher must apply directly to `expect()`
/// (no `->and()` in between, where Mago would rewrite the wrong expectation).
pub(crate) fn match_expect_matcher(tokens: &Tokens, i: usize) -> Option<ExpectMatcher> {
    let arrow = tokens
        .get_prev_meaningful_token(i)
        .filter(|&a| tokens[a].is_kind(T_OBJECT_OPERATOR))?;
    let open = tokens
        .get_next_meaningful_token(i)
        .filter(|&o| tokens[o].equals("(", true))?;
    let close = tokens
        .get_next_meaningful_token(open)
        .filter(|&c| tokens[c].equals(")", true))?;
    let mut p = tokens.get_prev_meaningful_token(arrow)?;
    let mut has_not = false;
    if tokens[p].is_kind(T_STRING) && tokens[p].get_content() == b"not" {
        let a = tokens
            .get_prev_meaningful_token(p)
            .filter(|&a| tokens[a].is_kind(T_OBJECT_OPERATOR))?;
        p = tokens.get_prev_meaningful_token(a)?;
        has_not = true;
    }
    if !tokens[p].equals(")", true) {
        return None;
    }
    let expect_open = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, p);
    let name = tokens
        .get_prev_meaningful_token(expect_open)
        .filter(|&n| tokens[n].is_kind(T_STRING))?;
    (tokens[name].get_content().eq_ignore_ascii_case(b"expect")
        && is_global_function_call(tokens, name))
    .then_some(ExpectMatcher {
        expect_open,
        expect_close: p,
        has_not,
        close,
    })
}
