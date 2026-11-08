//! Port of `App\PrettierFormatters\EscapedDirectiveSpacing`: puts back the whitespace prettier's blade plugin
//! eats in front of an escaped "@@" directive in HTML text ("Text @@if" -> "Text@@if" would stop escaping it).
use crate::analyzers::preg::RawPreg;
use std::collections::HashSet;

const PATTERN: &str = "/(\\w+)@@(?=(\\w+))/";

/// `$glued`: the "word@@word" pairs the source already glued together.
#[derive(Default)]
pub(super) struct EscapedDirectiveSpacing {
    glued: HashSet<Vec<u8>>,
}

fn pair(m: &crate::analyzers::preg::Captures) -> Vec<u8> {
    [m.str(1), b"@@", m.str(2)].concat()
}

impl EscapedDirectiveSpacing {
    /// Records the pairs; nothing is masked (`preg_match_all` failing leaves none).
    pub(super) fn pre_format(&mut self, content: &[u8]) {
        self.glued = RawPreg::match_all(PATTERN, content).iter().map(pair).collect();
    }

    /// `Err` where Pint's `preg_replace_callback` returns `null` (cast to `""`, emptying the file); Pun reports the
    /// file instead, like `EmbeddedBladeMasker::pre_format`.
    pub(super) fn post_format(&self, content: &[u8]) -> Result<Vec<u8>, String> {
        RawPreg::replace_callback(
            PATTERN,
            |m| match self.glued.contains(&pair(m)) {
                true => m.str(0).to_vec(),
                false => [m.str(1), b" @@"].concat(),
            },
            content,
        )
        .map_err(|e| format!("preg_replace_callback(): {e}"))
    }
}
