#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NumberKind {
    Integer,
    Float,
    OctalOrFloat,
    IntegerOrFloat,
}

pub(crate) enum OctalPrefix {
    Integer(u64),
    Float(f64),
}

/// PHP's tokenizer classifies a malformed legacy octal by the prefix before its first 8/9.
/// Float_::parse converts that same prefix with octdec: accumulate integers through INT_MAX,
/// then widen and continue in floating point (_php_math_basetozval, base 8).
pub(crate) fn octal_prefix(bytes: &[u8]) -> OctalPrefix {
    let mut integer = 0_u64;
    let mut float = None;
    for &byte in bytes.iter().take_while(|byte| !matches!(byte, b'8' | b'9')) {
        if byte == b'_' {
            continue;
        }
        let digit = u64::from(byte - b'0');
        if let Some(value) = &mut float {
            *value = *value * 8.0 + digit as f64;
        } else if integer > (i64::MAX as u64 - digit) / 8 {
            float = Some(integer as f64 * 8.0 + digit as f64);
        } else {
            integer = integer * 8 + digit;
        }
    }
    match float {
        Some(value) => OctalPrefix::Float(value),
        None => OctalPrefix::Integer(integer),
    }
}
