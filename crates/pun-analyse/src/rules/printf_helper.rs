//! PHPStan 2.2.7 PrintfHelper; scanf's count is PHP 8.5 ValidateFormat(format, 0), not a printf regex.

use std::collections::HashSet;
use std::sync::OnceLock;

/// PrintfHelper::parsePlaceholders / getPlaceholdersCount, using the same PCRE pattern and byte lengths.
pub(super) fn printf_count(format: &[u8], php_version: u32) -> Option<usize> {
    static PHP8: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    static PHP7: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let (cache, extra) = if php_version >= 80000 {
        (&PHP8, "hH")
    } else {
        (&PHP7, "")
    };
    let regex = cache.get_or_init(|| pun_phpstan::php::preg(&format!(
        r"~(?<before>%*)%(?:(?<position>\d+)\$)?[-+]?(?:[ 0]|(?:'[^%]))?(?<width>\*)?-?\d*(?:\.(?:\d+|(?<precision>\*))?)?(?<specifier>[bs{extra}s]|l?[cdeEgfFGouxX])?~"
    )).unwrap());
    let mut parameter = 0usize;
    let mut maximum = 0usize;
    for captures in regex.captures_iter(format) {
        let captures = captures.ok()?;
        if captures.name("before").unwrap().as_bytes().len() % 2 != 0 {
            continue;
        }
        for name in ["width", "precision"] {
            if captures
                .name(name)
                .is_some_and(|m| !m.as_bytes().is_empty())
            {
                parameter += 1;
                maximum = maximum.max(parameter);
            }
        }
        captures
            .name("specifier")
            .filter(|m| !m.as_bytes().is_empty())?;
        let position = match captures
            .name("position")
            .filter(|m| !m.as_bytes().is_empty())
        {
            Some(m) => {
                pun_tokens::php_compat::intval(&String::from_utf8_lossy(m.as_bytes())) as usize
            }
            None => {
                parameter += 1;
                parameter
            }
        };
        maximum = maximum.max(position);
    }
    Some(maximum)
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
