//! Small token helpers shared by several fixers (Pun's own; upstream inlines these).
use crate::analyzers::preg::Preg;
use pun_tokens::Tokens;

/// (first, last) meaningful token of each comma-separated item between `open` and `close`.
pub fn split_list(tokens: &Tokens, open: usize, close: usize) -> Vec<(usize, usize)> {
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

/// `Preg::match('/\R/', $s)`.
pub fn has_newline(s: &[u8]) -> bool {
    Preg::r#match("/\\R/", s)
}
