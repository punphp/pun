//! DateIntervalInstantiationRule (level 5, `featureToggles.checkDateIntervalConstructor`): `new DateInterval()` with
//! a constant string the constructor rejects. PHPStan runs the constructor; this ports what it runs, PHP 8.5's
//! `date_interval_initialize()` over timelib 2022.17's `timelib_strtointerval()` (parse_iso_intervals.re).

use mago_syntax::cst::*;

use super::Checker;

pub const IDENTIFIERS: &[&str] = &["new.dateInterval"];

pub(super) fn instantiation(c: &mut Checker, new: &Instantiation) {
    if !c.toggled.on.contains(crate::rules::driver::Toggles::DATE_INTERVAL) {
        return;
    }
    let Expression::Identifier(id) = new.class else {
        return;
    };
    let Some(argument) = new.argument_list.as_ref().and_then(|l| l.arguments.first()) else {
        return;
    };
    if !c.resolved(id).eq_ignore_ascii_case("dateinterval") {
        return;
    }
    let Some(t) = c.phpstan_type(argument.value()) else {
        return;
    };
    for duration in t.constant_strings() {
        if let Err(error) = construct(&duration) {
            let message = format!(
                "Instantiating DateInterval with {} produces an error: {error}",
                String::from_utf8_lossy(&duration)
            );
            c.report(new, "new.dateInterval", message, None);
        }
    }
}

/// `date_interval_initialize()`: the exception message of `new DateInterval($format)`, whose `%s` stops at a NUL.
fn construct(format: &[u8]) -> Result<(), String> {
    let printed = || String::from_utf8_lossy(format.split(|b| *b == 0).next().unwrap_or_default()).into_owned();
    let parsed = strtointerval(format);
    if parsed.error {
        Err(format!("Unknown or bad format ({})", printed()))
    } else if parsed.period || (parsed.begin && parsed.end) {
        Ok(())
    } else {
        Err(format!("Failed to parse interval ({})", printed()))
    }
}

/// What `timelib_strtointerval()` found: an error, a period, a begin and an end date.
#[derive(Default)]
struct Parsed {
    error: bool,
    period: bool,
    begin: bool,
    end: bool,
    date: bool,
}

/// C's `isspace()` in the C locale.
fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// `timelib_strtointerval()`: trims the whitespace (keeping one character of an all-whitespace string), then scans.
fn strtointerval(s: &[u8]) -> Parsed {
    let mut parsed = Parsed::default();
    if s.is_empty() {
        // "Empty string"
        parsed.error = true;
        return parsed;
    }
    let (mut start, mut end) = (0, s.len() - 1);
    while is_space(s[start]) && start < end {
        start += 1;
    }
    while is_space(s[end]) && end > start {
        end -= 1;
    }
    let input = &s[start..=end];
    let mut at = 0;
    while at < input.len() {
        at += token(input, at, &mut parsed);
    }
    parsed
}

fn digits(s: &[u8], at: usize) -> usize {
    s.get(at..).unwrap_or_default().iter().take_while(|b| b.is_ascii_digit()).count()
}

/// Whether `s[at..]` starts with the fixed-length `pattern`.
fn fixed(s: &[u8], at: usize, pattern: &[Unit]) -> bool {
    pattern.len() <= s.len().saturating_sub(at)
        && pattern.iter().enumerate().all(|(i, class)| class.matches(&s[at + i..]))
}

/// One position of a fixed-length scanner rule.
#[derive(Clone, Copy)]
enum Unit {
    Digit,
    Byte(u8),
    /// `hour24lz`, `minutelz`, `monthlz`, `monthlzz`, `daylz`, `daylzz` as a two-byte unit (the second byte is a
    /// [`Unit::Rest`]).
    Pair(fn(u8, u8) -> bool),
    Rest,
}

impl Unit {
    fn matches(self, s: &[u8]) -> bool {
        match self {
            Unit::Digit => s[0].is_ascii_digit(),
            Unit::Byte(b) => s[0] == b,
            Unit::Pair(f) => s.len() >= 2 && f(s[0], s[1]),
            Unit::Rest => true,
        }
    }
}

/// `[01][0-9] | "2"[0-4]`
fn hour24lz(a: u8, b: u8) -> bool {
    matches!((a, b), (b'0' | b'1', b'0'..=b'9') | (b'2', b'0'..=b'4'))
}

/// `[0-5][0-9]`
fn minutelz(a: u8, b: u8) -> bool {
    matches!(a, b'0'..=b'5') && b.is_ascii_digit()
}

/// `"0" [1-9] | "1" [0-2]`
fn monthlz(a: u8, b: u8) -> bool {
    matches!((a, b), (b'0', b'1'..=b'9') | (b'1', b'0'..=b'2'))
}

/// `"0" [0-9] | "1" [0-2]`
fn monthlzz(a: u8, b: u8) -> bool {
    matches!((a, b), (b'0', b'0'..=b'9') | (b'1', b'0'..=b'2'))
}

/// `"0" [1-9] | [1-2][0-9] | "3" [01]`
fn daylz(a: u8, b: u8) -> bool {
    matches!((a, b), (b'0', b'1'..=b'9') | (b'1' | b'2', b'0'..=b'9') | (b'3', b'0' | b'1'))
}

/// `"0" [0-9] | [1-2][0-9] | "3" [01]`
fn daylzz(a: u8, b: u8) -> bool {
    matches!((a, b), (b'0' | b'1' | b'2', b'0'..=b'9') | (b'3', b'0' | b'1'))
}

use Unit::{Byte, Digit, Pair, Rest};

/// `datetimebasic = year4 monthlz daylz "T" hour24lz minutelz secondlz "Z"`
const DATETIME_BASIC: [Unit; 16] = [
    Digit, Digit, Digit, Digit, Pair(monthlz), Rest, Pair(daylz), Rest, Byte(b'T'), Pair(hour24lz), Rest,
    Pair(minutelz), Rest, Pair(minutelz), Rest, Byte(b'Z'),
];

/// `datetimeextended = year4 "-" monthlz "-" daylz "T" hour24lz ':' minutelz ':' secondlz "Z"`
const DATETIME_EXTENDED: [Unit; 20] = [
    Digit, Digit, Digit, Digit, Byte(b'-'), Pair(monthlz), Rest, Byte(b'-'), Pair(daylz), Rest,
    Byte(b'T'), Pair(hour24lz), Rest, Byte(b':'), Pair(minutelz), Rest, Byte(b':'), Pair(minutelz), Rest, Byte(b'Z'),
];

/// `combinedrep = "P" year4 "-" monthlzz "-" daylzz "T" hour24lz ':' minutelz ':' secondlz`
const COMBINED: [Unit; 20] = [
    Byte(b'P'), Digit, Digit, Digit, Digit, Byte(b'-'), Pair(monthlzz), Rest, Byte(b'-'), Pair(daylzz),
    Rest, Byte(b'T'), Pair(hour24lz), Rest, Byte(b':'), Pair(minutelz), Rest, Byte(b':'), Pair(minutelz), Rest,
];

/// The length of `period = "P" (number "Y")? (number "M")? (number "W")? (number "D")? ("T" (number "H")? (number
/// "M")? (number "S")?)?` at `at`. A digit run is followed by one letter, so taking each group that matches gives
/// the longest match.
fn period(s: &[u8], at: usize) -> Option<usize> {
    if s.get(at) != Some(&b'P') {
        return None;
    }
    let mut end = at + 1;
    let group = |end: &mut usize, letter: u8| {
        let n = digits(s, *end);
        if n > 0 && s.get(*end + n) == Some(&letter) {
            *end += n + 1;
        }
    };
    for letter in *b"YMWD" {
        group(&mut end, letter);
    }
    if s.get(end) == Some(&b'T') {
        end += 1;
        for letter in *b"HMS" {
            group(&mut end, letter);
        }
    }
    Some(end - at)
}

/// `scan()`: the longest rule match at `at` (the first rule on a tie) and its action; the length consumed.
fn token(s: &[u8], at: usize, parsed: &mut Parsed) -> usize {
    let recurrences = (s[at] == b'R' && digits(s, at + 1) > 0).then(|| 1 + digits(s, at + 1));
    let datetime = if fixed(s, at, &DATETIME_EXTENDED) {
        Some(DATETIME_EXTENDED.len())
    } else {
        fixed(s, at, &DATETIME_BASIC).then_some(DATETIME_BASIC.len())
    };
    let period = period(s, at);
    let combined = fixed(s, at, &COMBINED).then_some(COMBINED.len());
    let rules = [recurrences, datetime, period, combined];
    let longest = rules.iter().flatten().max().copied().unwrap_or(0);
    match rules.iter().position(|r| longest > 0 && *r == Some(longest)) {
        // recurrences
        Some(0) => {}
        Some(1) => {
            if parsed.date || parsed.period {
                parsed.end = true;
            } else {
                parsed.begin = true;
            }
            parsed.date = true;
        }
        Some(2) => {
            period_action(&s[at..at + longest], parsed);
            parsed.period = true;
        }
        Some(_) => parsed.period = true,
        // `[ .,\t/]` and `"\000"|"\n"` are skipped, `any` is "Unexpected character"
        None => {
            if !matches!(s[at], b' ' | b'.' | b',' | b'\t' | b'/' | 0 | b'\n') {
                parsed.error = true;
            }
            return 1;
        }
    }
    longest
}

/// The `period` rule's action over its token, whose C string ends with a NUL.
fn period_action(token: &[u8], parsed: &mut Parsed) {
    let at = |i: usize| token.get(i).copied().unwrap_or(0);
    let mut ptr = 1;
    loop {
        if at(ptr) == b'T' {
            ptr += 1;
        }
        if at(ptr) == 0 {
            // "Missing expected time part"
            parsed.error = true;
            break;
        }
        // timelib_get_unsigned_nr(&ptr, 12): the token has no sign, and its numbers start at `ptr`
        ptr += digits(token, ptr).min(12);
        if !matches!(at(ptr), b'Y' | b'W' | b'D' | b'H' | b'S' | b'M') {
            // "Undefined period specifier"
            parsed.error = true;
        }
        ptr += 1;
        if parsed.error || at(ptr) == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::construct;

    /// `new DateInterval($s)` on PHP 8.5.12 (timelib 2022.17), via `php -r`.
    #[test]
    fn matches_php() {
        let ok = |s: &[u8]| assert_eq!(construct(s), Ok(()), "{}", String::from_utf8_lossy(s));
        let bad = |s: &[u8], m: &str| assert_eq!(construct(s), Err(m.to_string()), "{}", String::from_utf8_lossy(s));
        ok(b"P1D");
        ok(b"P1W2D");
        ok(b"P1Y2M3DT4H5M6S");
        ok(b"P0001-02-03T04:05:06");
        ok(b"P0000-00-00T24:59:59");
        ok(b"R5/2008-03-01T13:00:00Z/P1Y2M10DT2H30M");
        ok(b"2008-03-01T13:00:00Z/2009-05-11T15:30:00Z");
        ok(b"20080301T130000Z/20090511T153000Z");
        ok(b" P1D ");
        ok(b"P1D\n");
        ok(b"PT36H");
        ok(b"P1D,.P2D");
        ok(b"P123456789012D");
        ok(b"P1M1D");
        bad(b"1M", "Unknown or bad format (1M)");
        bad(b"", "Unknown or bad format ()");
        bad(b"asdfasdf", "Unknown or bad format (asdfasdf)");
        bad(b"PT1.5S", "Unknown or bad format (PT1.5S)");
        bad(b"p1d", "Unknown or bad format (p1d)");
        bad(b"P1DT", "Unknown or bad format (P1DT)");
        bad(b"PT", "Unknown or bad format (PT)");
        bad(b"P", "Unknown or bad format (P)");
        bad(b"P-1D", "Unknown or bad format (P-1D)");
        bad(b"P1D\0x", "Unknown or bad format (P1D)");
        bad(b"P1D1Y", "Unknown or bad format (P1D1Y)");
        bad(b"P1234567890123D", "Unknown or bad format (P1234567890123D)");
        bad(b"P0001-13-03T04:05:06", "Unknown or bad format (P0001-13-03T04:05:06)");
        bad(b"PT1HT1M", "Unknown or bad format (PT1HT1M)");
        bad(b"R", "Unknown or bad format (R)");
        bad(b"\r", "Unknown or bad format (\r)");
        bad(b"R5", "Failed to parse interval (R5)");
        bad(b"   ", "Failed to parse interval (   )");
        bad(b"2008-03-01T13:00:00Z", "Failed to parse interval (2008-03-01T13:00:00Z)");
        bad(b"2008-03-01T25:00:00Z/2009-05-11T15:30:00Z", "Unknown or bad format (2008-03-01T25:00:00Z/2009-05-11T15:30:00Z)");
    }
}
