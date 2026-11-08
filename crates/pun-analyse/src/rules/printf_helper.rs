//! PHPStan 2.3.5's PrintfFormatParser (php_formatted_print() of PHP 8.0+); scanf's count is PHP 8.5
//! ValidateFormat(format, 0).

use std::collections::HashSet;

const INT_MAX: u64 = 2147483647;

/// `PrintfFormatParser::parse()` then `getRequiredArgumentsCount()`: how many arguments the format consumes, `None`
/// when it throws ValueError. `h`/`H` are specifiers from PHP 8.0 (`supportsHhPrintfSpecifier()`).
pub(super) fn printf_count(format: &[u8], hh: bool) -> Option<usize> {
    Some(
        printf_uses(format, hh)?
            .iter()
            .map(|u| u.index + 1)
            .max()
            .unwrap_or(0),
    )
}

/// One argument use of `PrintfFormatParser::parse()`.
pub(super) struct Use {
    /// The zero-based argument after the format.
    pub(super) index: usize,
    /// `value`, `width` (`*`) or `precision` (`.*`).
    pub(super) kind: &'static str,
    pub(super) specifier: u8,
    /// The placeholder's text, from `%` to its specifier.
    pub(super) placeholder: Vec<u8>,
    /// The placeholder's 1-based position in the format.
    pub(super) number: usize,
}

/// `PrintfFormatParser::parse()`: the argument uses in the order the format consumes them, `None` when it throws
/// ValueError.
pub(super) fn printf_uses(format: &[u8], hh: bool) -> Option<Vec<Use>> {
    let length = format.len();
    let (mut i, mut next, mut number) = (0, 0, 0);
    let mut uses = Vec::new();
    let mut next_argument = || {
        next += 1;
        next - 1
    };
    while let Some(percent) = format[i..].iter().position(|b| *b == b'%').map(|p| p + i) {
        i = percent + 1;
        if format.get(i) == Some(&b'%') {
            i += 1;
            continue;
        }
        let mut pending = Vec::new();
        let mut argument = None;
        if !format.get(i).is_some_and(u8::is_ascii_alphabetic) {
            argument = argument_number(format, &mut i)?;
            while let Some(&char) = format.get(i) {
                if matches!(char, b' ' | b'0' | b'-' | b'+') {
                    i += 1;
                } else if char == b'\'' {
                    if length - i <= 1 {
                        return None;
                    }
                    i += 2;
                } else {
                    break;
                }
            }
            if format.get(i) == Some(&b'*') {
                i += 1;
                let width = argument_number(format, &mut i)?.unwrap_or_else(&mut next_argument);
                pending.push((width, "width"));
            } else if format.get(i).is_some_and(u8::is_ascii_digit)
                && self::number(format, &mut i) >= INT_MAX
            {
                return None;
            }
            if format.get(i) == Some(&b'.') {
                i += 1;
                if format.get(i) == Some(&b'*') {
                    i += 1;
                    let precision =
                        argument_number(format, &mut i)?.unwrap_or_else(&mut next_argument);
                    pending.push((precision, "precision"));
                } else if format.get(i).is_some_and(u8::is_ascii_digit)
                    && self::number(format, &mut i) >= INT_MAX
                {
                    return None;
                }
            }
        }
        if format.get(i) == Some(&b'l') {
            i += 1;
        }
        let argument = argument.unwrap_or_else(&mut next_argument);
        let specifier = *format.get(i)?;
        if !b"sdueEfFgGcoxXb%".contains(&specifier) && !(hh && b"hH".contains(&specifier)) {
            return None;
        }
        let placeholder = format[percent..=i].to_vec();
        number += 1;
        for (index, kind) in pending.into_iter().chain([(argument, "value")]) {
            uses.push(Use {
                index,
                kind,
                specifier,
                placeholder: placeholder.clone(),
                number,
            });
        }
        i += 1;
    }
    Some(uses)
}

/// `parseArgumentNumber()`: `Some(None)` without an `n$`, the zero-based index after one, `None` out of range.
fn argument_number(format: &[u8], i: &mut usize) -> Option<Option<usize>> {
    let end = *i
        + format[*i..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count();
    if format.get(end) != Some(&b'$') {
        return Some(None);
    }
    let value = number(format, i);
    *i += 1;
    (value > 0 && value < INT_MAX).then(|| Some(value as usize - 1))
}

/// `parseNumber()`: the digits at `i`, INT_MAX past ten significant digits.
fn number(format: &[u8], i: &mut usize) -> u64 {
    let start = *i;
    while format.get(*i).is_some_and(u8::is_ascii_digit) {
        *i += 1;
    }
    let digits = &format[start..*i];
    let digits = &digits[digits.iter().take_while(|b| **b == b'0').count()..];
    if digits.len() > 10 {
        return INT_MAX;
    }
    digits.iter().fold(0, |n, d| n * 10 + u64::from(d - b'0'))
}

/// `count(sscanf('', '%*n' . $format))`: `%*n` prevents EOF from replacing the preallocated array with null.
/// Port of PHP-8.5/ext/standard/scanf.c ValidateFormat with numVars=0; no input conversion is needed.
pub(super) fn scanf_count(mut format: &[u8]) -> Option<usize> {
    const SCAN_MAX_ARGS: usize = 255;
    format = format.split(|b| *b == 0).next().unwrap();
    let mut offset = 0;
    let mut sequential = false;
    let mut positional = false;
    let mut index = 0;
    let mut maximum = 0;
    let mut assigned = HashSet::new();
    while offset < format.len() {
        let ch = format[offset];
        offset += 1;
        if ch != b'%' {
            continue;
        }
        let mut ch = *format.get(offset)?;
        offset += 1;
        if ch == b'%' {
            continue;
        }
        let suppress = ch == b'*';
        if suppress {
            ch = *format.get(offset)?;
            offset += 1;
        } else {
            let mut end = offset - 1;
            let number = decimal(format, &mut end);
            if end >= offset && format.get(end) == Some(&b'$') {
                positional = true;
                // ZEND_STRTOUL's unsigned long narrows to C int before the index bounds check.
                let value = number as i32;
                if sequential || value < 1 || value as usize > SCAN_MAX_ARGS {
                    return None;
                }
                index = value as usize - 1;
                maximum = maximum.max(value as usize);
                offset = end + 1;
                ch = *format.get(offset)?;
                offset += 1;
            } else {
                sequential = true;
                if positional {
                    return None;
                }
            }
        }
        if ch.is_ascii_digit() {
            offset -= 1;
            decimal(format, &mut offset);
            ch = *format.get(offset)?;
            offset += 1;
        }
        if matches!(ch, b'l' | b'L' | b'h') {
            ch = *format.get(offset)?;
            offset += 1;
        }
        if ch == b'[' {
            ch = *format.get(offset)?;
            offset += 1;
            if ch == b'^' {
                ch = *format.get(offset)?;
                offset += 1;
            }
            if ch == b']' {
                ch = *format.get(offset)?;
                offset += 1;
            }
            while ch != b']' {
                ch = *format.get(offset)?;
                offset += 1;
            }
        } else if !b"ndDioxXufeEgsc".contains(&ch) {
            return None;
        }
        if !suppress {
            if !assigned.insert(index) {
                return None;
            }
            index += 1;
        }
    }
    Some(if positional { maximum } else { index })
}

fn decimal(bytes: &[u8], offset: &mut usize) -> u64 {
    let mut value = 0u64;
    while let Some(&byte) = bytes.get(*offset).filter(|b| b.is_ascii_digit()) {
        value = value
            .saturating_mul(10)
            .saturating_add(u64::from(byte - b'0'));
        *offset += 1;
    }
    value
}
