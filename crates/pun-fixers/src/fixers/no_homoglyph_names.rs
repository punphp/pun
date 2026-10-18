//! Port of `PhpCsFixer\Fixer\Naming\NoHomoglyphNamesFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct NoHomoglyphNamesFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoHomoglyphNamesFixer)
}

/// `REPLACEMENTS`, in upstream order (dumped from the pinned class).
const REPLACEMENTS: &[(&str, u8)] = &[
    ("O", b'0'),
    ("０", b'0'),
    ("I", b'1'),
    ("１", b'1'),
    ("２", b'2'),
    ("３", b'3'),
    ("４", b'4'),
    ("５", b'5'),
    ("６", b'6'),
    ("７", b'7'),
    ("８", b'8'),
    ("９", b'9'),
    ("Α", b'A'),
    ("А", b'A'),
    ("Ａ", b'A'),
    ("ʙ", b'B'),
    ("Β", b'B'),
    ("В", b'B'),
    ("Ｂ", b'B'),
    ("Ϲ", b'C'),
    ("С", b'C'),
    ("Ⅽ", b'C'),
    ("Ｃ", b'C'),
    ("Ⅾ", b'D'),
    ("Ｄ", b'D'),
    ("Ε", b'E'),
    ("Е", b'E'),
    ("Ｅ", b'E'),
    ("Ϝ", b'F'),
    ("Ｆ", b'F'),
    ("ɢ", b'G'),
    ("Ԍ", b'G'),
    ("Ｇ", b'G'),
    ("ʜ", b'H'),
    ("Η", b'H'),
    ("Н", b'H'),
    ("Ｈ", b'H'),
    ("l", b'I'),
    ("Ι", b'I'),
    ("І", b'I'),
    ("Ⅰ", b'I'),
    ("Ｉ", b'I'),
    ("Ј", b'J'),
    ("Ｊ", b'J'),
    ("Κ", b'K'),
    ("К", b'K'),
    ("K", b'K'),
    ("Ｋ", b'K'),
    ("ʟ", b'L'),
    ("Ⅼ", b'L'),
    ("Ｌ", b'L'),
    ("Μ", b'M'),
    ("М", b'M'),
    ("Ⅿ", b'M'),
    ("Ｍ", b'M'),
    ("ɴ", b'N'),
    ("Ν", b'N'),
    ("Ｎ", b'N'),
    ("Ο", b'O'),
    ("О", b'O'),
    ("Ｏ", b'O'),
    ("Ρ", b'P'),
    ("Р", b'P'),
    ("Ｐ", b'P'),
    ("Ｑ", b'Q'),
    ("ʀ", b'R'),
    ("Ｒ", b'R'),
    ("Ѕ", b'S'),
    ("Ｓ", b'S'),
    ("Τ", b'T'),
    ("Т", b'T'),
    ("Ｔ", b'T'),
    ("Ｕ", b'U'),
    ("Ѵ", b'V'),
    ("Ⅴ", b'V'),
    ("Ｖ", b'V'),
    ("Ｗ", b'W'),
    ("Χ", b'X'),
    ("Х", b'X'),
    ("Ⅹ", b'X'),
    ("Ｘ", b'X'),
    ("ʏ", b'Y'),
    ("Υ", b'Y'),
    ("Ү", b'Y'),
    ("Ｙ", b'Y'),
    ("Ζ", b'Z'),
    ("Ｚ", b'Z'),
    ("＿", b'_'),
    ("ɑ", b'a'),
    ("а", b'a'),
    ("ａ", b'a'),
    ("Ь", b'b'),
    ("ｂ", b'b'),
    ("ϲ", b'c'),
    ("с", b'c'),
    ("ⅽ", b'c'),
    ("ｃ", b'c'),
    ("ԁ", b'd'),
    ("ⅾ", b'd'),
    ("ｄ", b'd'),
    ("е", b'e'),
    ("ｅ", b'e'),
    ("ｆ", b'f'),
    ("ɡ", b'g'),
    ("ｇ", b'g'),
    ("һ", b'h'),
    ("ｈ", b'h'),
    ("ɩ", b'i'),
    ("і", b'i'),
    ("ⅰ", b'i'),
    ("ｉ", b'i'),
    ("ј", b'j'),
    ("ｊ", b'j'),
    ("ｋ", b'k'),
    ("ⅼ", b'l'),
    ("ｌ", b'l'),
    ("ⅿ", b'm'),
    ("ｍ", b'm'),
    ("ｎ", b'n'),
    ("ο", b'o'),
    ("о", b'o'),
    ("ｏ", b'o'),
    ("р", b'p'),
    ("ｐ", b'p'),
    ("ｑ", b'q'),
    ("ｒ", b'r'),
    ("ѕ", b's'),
    ("ｓ", b's'),
    ("ｔ", b't'),
    ("ｕ", b'u'),
    ("ν", b'v'),
    ("ѵ", b'v'),
    ("ⅴ", b'v'),
    ("ｖ", b'v'),
    ("ѡ", b'w'),
    ("ｗ", b'w'),
    ("х", b'x'),
    ("ⅹ", b'x'),
    ("ｘ", b'x'),
    ("у", b'y'),
    ("ｙ", b'y'),
    ("ｚ", b'z'),
];

impl Fixer for NoHomoglyphNamesFixer {
    fn name(&self) -> &'static str {
        "no_homoglyph_names"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_VARIABLE, T_STRING])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            let token = &tokens[index];
            if !token.is_given_kind(&[T_VARIABLE, T_STRING]) {
                continue;
            }
            let (replaced, count) = Preg::replace_callback_limit(
                "/[^[:ascii:]]/u",
                |m| {
                    let s = m.str(0);
                    REPLACEMENTS
                        .iter()
                        .find(|(k, _)| k.as_bytes() == s)
                        .map_or_else(|| s.to_vec(), |&(_, v)| vec![v])
                },
                token.get_content(),
                -1,
            );
            if count > 0 {
                let id = token.get_id().unwrap();
                tokens.set(index, Token::new(id, replaced));
            }
        }
    }
}
