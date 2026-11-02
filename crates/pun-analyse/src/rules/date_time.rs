//! DateTimeInstantiationRule (level 5): `new DateTime()` or `new DateTimeImmutable()` with a constant string whose
//! parse reports errors. PHPStan runs the constructor and reads `DateTime::getLastErrors()['errors']`; this ports
//! what that runs, PHP 8.5's `php_date_initialize()` over timelib 2022.17's `timelib_strtotime()` (parse_date.re).
//!
//! The re2c scanner is its rules as regular expressions, matched like re2c does: the longest match, the earliest rule
//! on a tie. The actions keep only the state an error depends on: the have-time/date/zone counters and the relative
//! offsets whose overflow is an error. Timezone identifiers are the system PHP's (`date_time_zones.txt`,
//! `DateTimeZone::listIdentifiers(DateTimeZone::ALL_WITH_BC)`, matched case-insensitively as its database does).

use std::sync::OnceLock;

use mago_syntax::cst::*;
use regex_automata::dfa::{Automaton, StartKind, dense};
use regex_automata::nfa::thompson;
use regex_automata::util::{start, syntax};
use regex_automata::{Anchored, MatchKind};

use super::Checker;

pub const IDENTIFIERS: &[&str] = &["new.dateTime", "new.dateTimeImmutable"];

pub(super) fn instantiation(c: &mut Checker, new: &Instantiation) {
    if c.level < 5 {
        return;
    }
    let Expression::Identifier(id) = new.class else {
        return;
    };
    let Some(argument) = new.argument_list.as_ref().and_then(|l| l.arguments.first()) else {
        return;
    };
    let class = c.resolved(id);
    let (class, identifier) = if class.eq_ignore_ascii_case("datetime") {
        ("DateTime", "new.dateTime")
    } else if class.eq_ignore_ascii_case("datetimeimmutable") {
        ("DateTimeImmutable", "new.dateTimeImmutable")
    } else {
        return;
    };
    let Some(t) = c.phpstan_type(argument.value()) else {
        return;
    };
    for date in t.constant_strings() {
        for error in construct(&date) {
            let message = format!(
                "Instantiating {class} with {} produces an error: {error}",
                String::from_utf8_lossy(&date)
            );
            c.report(new, identifier, message, None);
        }
    }
}

/// `DateTime::getLastErrors()['errors']` after `new DateTime($time)`: an empty string is "now".
fn construct(time: &[u8]) -> Vec<&'static str> {
    if time.is_empty() {
        return Vec::new();
    }
    // php_date.c: zval_from_error_container() keys the errors by position, so a later one replaces an earlier one
    let mut errors: Vec<(usize, &'static str)> = Vec::new();
    for (position, message) in strtotime(time) {
        match errors.iter_mut().find(|(p, _)| *p == position) {
            Some(e) => e.1 = message,
            None => errors.push((position, message)),
        }
    }
    errors.into_iter().map(|(_, m)| m).collect()
}

/// C's `isspace()` in the C locale.
fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// re2c's `YYMAXFILL`: the scanner's buffer is the string and this many NUL bytes.
const YYMAXFILL: usize = 36;
/// `TIMELIB_MAX_ERRORS`.
const MAX_ERRORS: usize = 1000;

/// `timelib_strtotime()`'s errors, with their positions.
fn strtotime(time: &[u8]) -> Vec<(usize, &'static str)> {
    let (mut start, mut end) = (0, time.len() - 1);
    while start <= end && is_space(time[start]) {
        start += 1;
    }
    if start > end {
        return vec![(0, "Empty string")];
    }
    while end > start && is_space(time[end]) {
        end -= 1;
    }
    let len = end + 1 - start;
    let mut buffer = time[start..=end].to_vec();
    buffer.resize(len + YYMAXFILL, 0);
    let mut s = Scanner::default();
    let mut cursor = 0;
    // YYFILL returns EOI once fewer than YYMAXFILL bytes are left at a token's start
    while cursor <= len {
        let (rule, end) = longest(&buffer, cursor);
        s.tok = cursor;
        s.action(rule, &buffer[cursor..end]);
        cursor = end;
    }
    s.errors
}

/// The scanner's state that its errors depend on.
#[derive(Default)]
struct Scanner {
    /// The start of the token being scanned (`s->tok`), where its errors are.
    tok: usize,
    errors: Vec<(usize, &'static str)>,
    have_time: i64,
    have_date: bool,
    have_zone: i64,
    /// `relative.y`, `m`, `d`, `h`, `i`, `s` and `us`: adding to one beyond 64 bits is an error.
    relative: [i64; 7],
}

const YEAR: usize = 0;
const MONTH: usize = 1;
const DAY: usize = 2;
const HOUR: usize = 3;
const MINUTE: usize = 4;
const SECOND: usize = 5;
const MICROSECOND: usize = 6;

/// A `const char *` into a token: reading at or past its end gives the terminating NUL.
struct Ptr<'a> {
    s: &'a [u8],
    i: usize,
}

impl Ptr<'_> {
    fn at(&self, offset: usize) -> u8 {
        self.s.get(self.i + offset).copied().unwrap_or(0)
    }

    fn c(&self) -> u8 {
        self.at(0)
    }
}

/// What `timelib_set_relative()` does with a unit.
#[derive(Clone, Copy)]
enum Unit {
    Relative(usize, i64),
    Weekday,
    Special,
}

/// `timelib_relunit_lookup`.
const RELUNITS: &[(&[u8], Unit)] = &[
    (b"ms", Unit::Relative(MICROSECOND, 1000)),
    (b"msec", Unit::Relative(MICROSECOND, 1000)),
    (b"msecs", Unit::Relative(MICROSECOND, 1000)),
    (b"millisecond", Unit::Relative(MICROSECOND, 1000)),
    (b"milliseconds", Unit::Relative(MICROSECOND, 1000)),
    ("µs".as_bytes(), Unit::Relative(MICROSECOND, 1)),
    (b"usec", Unit::Relative(MICROSECOND, 1)),
    (b"usecs", Unit::Relative(MICROSECOND, 1)),
    ("µsec".as_bytes(), Unit::Relative(MICROSECOND, 1)),
    ("µsecs".as_bytes(), Unit::Relative(MICROSECOND, 1)),
    (b"microsecond", Unit::Relative(MICROSECOND, 1)),
    (b"microseconds", Unit::Relative(MICROSECOND, 1)),
    (b"sec", Unit::Relative(SECOND, 1)),
    (b"secs", Unit::Relative(SECOND, 1)),
    (b"second", Unit::Relative(SECOND, 1)),
    (b"seconds", Unit::Relative(SECOND, 1)),
    (b"min", Unit::Relative(MINUTE, 1)),
    (b"mins", Unit::Relative(MINUTE, 1)),
    (b"minute", Unit::Relative(MINUTE, 1)),
    (b"minutes", Unit::Relative(MINUTE, 1)),
    (b"hour", Unit::Relative(HOUR, 1)),
    (b"hours", Unit::Relative(HOUR, 1)),
    (b"day", Unit::Relative(DAY, 1)),
    (b"days", Unit::Relative(DAY, 1)),
    (b"week", Unit::Relative(DAY, 7)),
    (b"weeks", Unit::Relative(DAY, 7)),
    (b"fortnight", Unit::Relative(DAY, 14)),
    (b"fortnights", Unit::Relative(DAY, 14)),
    (b"forthnight", Unit::Relative(DAY, 14)),
    (b"forthnights", Unit::Relative(DAY, 14)),
    (b"month", Unit::Relative(MONTH, 1)),
    (b"months", Unit::Relative(MONTH, 1)),
    (b"year", Unit::Relative(YEAR, 1)),
    (b"years", Unit::Relative(YEAR, 1)),
    (b"mondays", Unit::Weekday),
    (b"monday", Unit::Weekday),
    (b"mon", Unit::Weekday),
    (b"tuesdays", Unit::Weekday),
    (b"tuesday", Unit::Weekday),
    (b"tue", Unit::Weekday),
    (b"wednesdays", Unit::Weekday),
    (b"wednesday", Unit::Weekday),
    (b"wed", Unit::Weekday),
    (b"thursdays", Unit::Weekday),
    (b"thursday", Unit::Weekday),
    (b"thu", Unit::Weekday),
    (b"fridays", Unit::Weekday),
    (b"friday", Unit::Weekday),
    (b"fri", Unit::Weekday),
    (b"saturdays", Unit::Weekday),
    (b"saturday", Unit::Weekday),
    (b"sat", Unit::Weekday),
    (b"sundays", Unit::Weekday),
    (b"sunday", Unit::Weekday),
    (b"sun", Unit::Weekday),
    (b"weekday", Unit::Special),
    (b"weekdays", Unit::Special),
];

/// `timelib_reltext_lookup`: a word's amount (0 when it is none of them).
const RELTEXT: &[(&[u8], i64)] = &[
    (b"first", 1),
    (b"next", 1),
    (b"second", 2),
    (b"third", 3),
    (b"fourth", 4),
    (b"fifth", 5),
    (b"sixth", 6),
    (b"seventh", 7),
    (b"eight", 8),
    (b"eighth", 8),
    (b"ninth", 9),
    (b"tenth", 10),
    (b"eleventh", 11),
    (b"twelfth", 12),
    (b"last", -1),
    (b"previous", -1),
    (b"this", 0),
];

/// The abbreviations of timezonemap.h, with abbr_search()'s "utc" and "gmt".
const ABBREVIATIONS: &[&str] = &[
    "a", "acdt", "acst", "addt", "adt", "aedt", "aest", "ahdt", "ahst", "akdt", "akst", "amt",
    "apt", "ast", "awdt", "awst", "awt", "b", "bdst", "bdt", "bmt", "bst", "c", "cast", "cat",
    "cddt", "cdt", "cemt", "cest", "cet", "chst", "cmt", "cpt", "cst", "cwt", "d", "dmt", "e",
    "eat", "eddt", "edt", "eest", "eet", "emt", "ept", "est", "ewt", "f", "ffmt", "fmt", "g",
    "gdt", "gmt", "gst", "h", "hdt", "hkst", "hkt", "hmt", "hpt", "hst", "hwt", "i", "iddt", "idt",
    "imt", "ist", "jdt", "jmt", "jst", "k", "kdt", "kmt", "kst", "l", "lst", "m", "mddt", "mdst",
    "mdt", "mest", "met", "mmt", "mpt", "msd", "msk", "mst", "mwt", "n", "nddt", "ndt", "npt",
    "nst", "nwt", "nzdt", "nzmt", "nzst", "o", "p", "pddt", "pdt", "pkst", "pkt", "plmt", "pmt",
    "ppmt", "ppt", "pst", "pwt", "q", "qmt", "r", "rmt", "s", "sast", "sdmt", "sjmt", "smt", "sst",
    "t", "tbmt", "tmt", "u", "uct", "utc", "v", "w", "wast", "wat", "wemt", "west", "wet", "wib",
    "wit", "wita", "wmt", "x", "y", "yddt", "ydt", "ypt", "yst", "ywt", "z",
];

impl Scanner {
    fn add_error(&mut self, message: &'static str) {
        if self.errors.len() < MAX_ERRORS {
            self.errors.push((self.tok, message));
        }
    }

    /// `TIMELIB_HAVE_TIME()`: false (and an error) when the time is already set.
    fn have_time(&mut self) -> bool {
        if self.have_time != 0 {
            self.add_error("Double time specification");
            return false;
        }
        self.have_time = 1;
        true
    }

    /// `TIMELIB_HAVE_DATE()`.
    fn have_date(&mut self) -> bool {
        if self.have_date {
            self.add_error("Double date specification");
            return false;
        }
        self.have_date = true;
        true
    }

    /// `TIMELIB_HAVE_TZ()`: a second zone is a warning, a third one an error.
    fn have_tz(&mut self) -> bool {
        self.have_zone += 1;
        if self.have_zone > 2 {
            self.add_error("Double timezone specification");
        }
        self.have_zone == 1
    }

    /// `timelib_get_signed_nr()`.
    fn get_signed_nr(&mut self, p: &mut Ptr, max_length: usize) -> i64 {
        while !p.c().is_ascii_digit() && p.c() != b'+' && p.c() != b'-' {
            if p.c() == 0 {
                self.add_error("Found unexpected data");
                return 0;
            }
            p.i += 1;
        }
        let mut negative = false;
        while p.c() == b'+' || p.c() == b'-' {
            negative ^= p.c() == b'-';
            p.i += 1;
        }
        while !p.c().is_ascii_digit() {
            if p.c() == 0 {
                self.add_error("Found unexpected data");
                return 0;
            }
            p.i += 1;
        }
        let mut number: i128 = 0;
        let mut length = 0;
        while p.c().is_ascii_digit() && length < max_length {
            number = number * 10 + i128::from(p.c() - b'0');
            p.i += 1;
            length += 1;
        }
        // strtoll()'s ERANGE
        match i64::try_from(if negative { -number } else { number }) {
            Ok(n) => n,
            Err(_) => {
                self.add_error("Number out of range");
                0
            }
        }
    }

    /// `add_with_overflow()`.
    fn add_relative(&mut self, field: usize, amount: i64, multiplier: i64) {
        let (sum, overflow) = self.relative[field].overflowing_add(amount.wrapping_mul(multiplier));
        self.relative[field] = sum;
        if overflow {
            self.add_error("Number out of range");
        }
    }

    /// `timelib_set_relative()`; `keep_time` is `TIMELIB_TIME_PART_KEEP`.
    fn set_relative(&mut self, p: &mut Ptr, amount: i64, keep_time: bool) {
        let begin = p.i;
        while !matches!(
            p.c(),
            0 | b' ' | b',' | b'\t' | b';' | b':' | b'/' | b'.' | b'-' | b'(' | b')'
        ) {
            p.i += 1;
        }
        let word = &p.s[begin..p.i];
        let Some((_, unit)) = RELUNITS
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(word))
        else {
            return;
        };
        match *unit {
            Unit::Relative(field, multiplier) => self.add_relative(field, amount, multiplier),
            Unit::Weekday => {
                if !keep_time {
                    self.have_time = 0;
                }
                let weeks = if amount > 0 { amount - 1 } else { amount };
                self.relative[DAY] = self.relative[DAY].wrapping_add(weeks.wrapping_mul(7));
            }
            Unit::Special => {
                if !keep_time {
                    self.have_time = 0;
                }
            }
        }
    }

    /// The action of the rule matched on `token`.
    fn action(&mut self, rule: Rule, token: &[u8]) {
        let mut p = Ptr { s: token, i: 0 };
        match rule {
            Rule::Yesterday | Rule::Tomorrow => {
                self.have_time = 0;
                self.relative[DAY] = if rule == Rule::Yesterday { -1 } else { 1 };
            }
            Rule::Now | Rule::FirstLastDayOf | Rule::Year4 | Rule::Skip => {}
            Rule::Noon | Rule::BackFrontOf => self.have_time = 1,
            Rule::MidnightToday => self.have_time = 0,
            Rule::Timestamp | Rule::TimestampMs => {
                self.have_date = false;
                self.have_time = 0;
                if !self.have_tz() {
                    return;
                }
                let seconds = self.get_signed_nr(&mut p, 24);
                self.relative[SECOND] = self.relative[SECOND].wrapping_add(seconds);
                if rule == Rule::TimestampMs {
                    let negative = token.get(1) == Some(&b'-');
                    let before = p.i;
                    let us = self.get_signed_nr(&mut p, 6);
                    // `us * pow(10, 7 - (ptr - ptr_before))`, into an unsigned 64-bit integer
                    let scaled = (us as f64 * 10f64.powi(7 - (p.i - before) as i32)) as i64;
                    self.relative[MICROSECOND] = if negative {
                        scaled.wrapping_neg()
                    } else {
                        scaled
                    };
                }
            }
            Rule::WeekdayOf => {
                let amount = relative_text(&mut p);
                eat_spaces(&mut p);
                self.set_relative(&mut p, amount, false);
            }
            // the time tokens hold no zone, so their zone parse never runs
            Rule::Time => {
                self.have_time();
            }
            Rule::GnuNoColon => {
                if self.have_time > 1 {
                    self.add_error("Double time specification");
                    return;
                }
                self.have_time += 1;
            }
            Rule::Date | Rule::MonthText => {
                self.have_date();
            }
            Rule::IsoDateSigned | Rule::IsoDateX => {
                if self.have_date() {
                    self.get_signed_nr(&mut p, if rule == Rule::IsoDateX { 19 } else { 4 });
                }
            }
            // a zone in these tokens is an offset, which is always found
            Rule::DateTime => {
                if self.have_time() {
                    self.have_date();
                }
            }
            Rule::IsoWeekDay | Rule::IsoWeek => {
                if !self.have_date() {
                    return;
                }
                let year = get_nr(&mut p, 4);
                let week = get_nr(&mut p, 2);
                let day = if rule == Rule::IsoWeekDay {
                    get_nr(&mut p, 1)
                } else {
                    1
                };
                self.relative[DAY] = daynr_from_weeknr(year, week, day);
            }
            Rule::Ago => {
                for field in [YEAR, MONTH, DAY, HOUR, MINUTE, SECOND] {
                    self.relative[field] = self.relative[field].wrapping_neg();
                }
            }
            Rule::DayText => self.have_time = 0,
            Rule::RelativeText | Rule::RelativeTextWeek => {
                while p.c() != 0 {
                    let amount = relative_text(&mut p);
                    eat_spaces(&mut p);
                    self.set_relative(&mut p, amount, false);
                }
            }
            Rule::Zone => {
                if !self.have_tz() {
                    return;
                }
                eat_spaces(&mut p);
                if zone_not_found(&mut p) {
                    self.add_error("The timezone could not be found in the database");
                }
            }
            Rule::DateShortTime12 => {
                if self.have_date() {
                    self.have_time();
                }
            }
            Rule::DateShortTime24 => {
                if !self.have_date() || !self.have_time() {
                    return;
                }
                get_month(&mut p);
                for _ in 0..3 {
                    get_nr(&mut p, 2);
                }
                if p.c() == b':' {
                    get_nr(&mut p, 2);
                    if p.c() == b'.' {
                        get_frac_nr(&mut p);
                    }
                }
                if p.c() != 0 && zone_not_found(&mut p) {
                    self.add_error("The timezone could not be found in the database");
                }
            }
            Rule::Relative => {
                while p.c() != 0 {
                    let amount = self.get_signed_nr(&mut p, 24);
                    eat_spaces(&mut p);
                    self.set_relative(&mut p, amount, true);
                }
            }
            Rule::Any => self.add_error("Unexpected character"),
        }
    }
}

/// `timelib_parse_zone()`: whether the zone is not found.
fn zone_not_found(p: &mut Ptr) -> bool {
    while matches!(p.c(), b' ' | b'\t' | b'(') {
        p.i += 1;
    }
    // timelib_parse_tz_cor() accepts every offset the scanner's tzcorrection matches
    if matches!(p.c(), b'+' | b'-')
        || p.s[p.i..].starts_with(b"GMT") && matches!(p.at(3), b'+' | b'-')
    {
        return false;
    }
    let begin = p.i;
    while p.c().is_ascii_alphanumeric() || matches!(p.c(), b'/' | b'_' | b'-' | b'+') {
        p.i += 1;
    }
    let word = &p.s[begin..p.i];
    // MAX_ABBR_LEN is glibc's _POSIX_TZNAME_MAX, 6
    let abbreviation = word.len() < 6
        && ABBREVIATIONS
            .iter()
            .any(|a| a.as_bytes().eq_ignore_ascii_case(word));
    // an abbreviation is found whatever the database has (it is looked up too for "UTC")
    !abbreviation && !is_zone_identifier(word)
}

/// Whether the timezone database has `name`, compared case-insensitively.
fn is_zone_identifier(name: &[u8]) -> bool {
    include_str!("date_time_zones.txt")
        .lines()
        .any(|z| z.as_bytes().eq_ignore_ascii_case(name))
}

/// `timelib_eat_spaces()`: spaces, tabs, NBSPs and NNBSPs.
fn eat_spaces(p: &mut Ptr) {
    loop {
        match (p.c(), p.at(1), p.at(2)) {
            (b' ' | b'\t', _, _) => p.i += 1,
            (0xe2, 0x80, 0xaf) => p.i += 3,
            (0xc2, 0xa0, _) => p.i += 2,
            _ => return,
        }
    }
}

/// `timelib_get_nr()`.
fn get_nr(p: &mut Ptr, max_length: usize) -> i64 {
    // the tokens always hold the digits it looks for
    while p.c() != 0 && !p.c().is_ascii_digit() {
        p.i += 1;
    }
    let mut number = 0;
    let mut length = 0;
    while p.c().is_ascii_digit() && length < max_length {
        number = number * 10 + i64::from(p.c() - b'0');
        p.i += 1;
        length += 1;
    }
    number
}

/// `timelib_get_frac_nr()`, for where it leaves the pointer.
fn get_frac_nr(p: &mut Ptr) {
    while p.c() != 0 && !matches!(p.c(), b'.' | b':' | b'0'..=b'9') {
        p.i += 1;
    }
    while matches!(p.c(), b'.' | b':' | b'0'..=b'9') {
        p.i += 1;
    }
}

/// `timelib_get_month()`, for where it leaves the pointer.
fn get_month(p: &mut Ptr) {
    while matches!(p.c(), b' ' | b'\t' | b'-' | b'.' | b'/') {
        p.i += 1;
    }
    while p.c().is_ascii_alphabetic() {
        p.i += 1;
    }
}

/// `timelib_get_relative_text()`: the amount of a relative word.
fn relative_text(p: &mut Ptr) -> i64 {
    while matches!(p.c(), b' ' | b'\t' | b'-' | b'/') {
        p.i += 1;
    }
    let begin = p.i;
    while p.c().is_ascii_alphabetic() {
        p.i += 1;
    }
    let word = &p.s[begin..p.i];
    RELTEXT
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(word))
        .map_or(0, |(_, amount)| *amount)
}

/// timelib's dow.c: `timelib_daynr_from_weeknr()`.
fn daynr_from_weeknr(year: i64, week: i64, day: i64) -> i64 {
    const COMMON: [i64; 13] = [-1, 0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
    const LEAP: [i64; 13] = [-1, 6, 2, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5];
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let century = 6 - (year.rem_euclid(400) / 100).rem_euclid(4) * 2;
    let y = year.rem_euclid(100);
    let month = if leap { LEAP[1] } else { COMMON[1] };
    let dow = (century + y + month + y / 4 + 1).rem_euclid(7);
    let first = -(if dow > 4 { dow - 7 } else { dow });
    first + (week - 1) * 7 + day
}

/// The scanner's rules, in parse_date.re's order (re2c prefers the earliest of the longest matches).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rule {
    Yesterday,
    Now,
    Noon,
    MidnightToday,
    Tomorrow,
    Timestamp,
    TimestampMs,
    FirstLastDayOf,
    BackFrontOf,
    WeekdayOf,
    /// timetiny12 | timeshort12 | timelong12, mssqltime, timetiny24 | timeshort24 | timelong24 | iso8601long and
    /// iso8601nocolon: they only set the time.
    Time,
    GnuNoColon,
    /// americanshort | american, iso8601date2, gnudateshorter, gnudateshort, datefull, pointeddate4, pointeddate2,
    /// datenoday, datenodayrev, datetextual | datenoyear, datenoyearrev, datenocolon, pgydotd, pgtextshort and
    /// pgtextreverse: they only set the date.
    Date,
    IsoDateSigned,
    IsoDateX,
    /// xmlrpc | xmlrpcnocolon | soap | wddx | exif, and clf.
    DateTime,
    IsoWeekDay,
    IsoWeek,
    Year4,
    Ago,
    DayText,
    RelativeTextWeek,
    RelativeText,
    MonthText,
    Zone,
    DateShortTime12,
    DateShortTime24,
    Relative,
    /// `[.,]`, space, `"\000"|"\n"`.
    Skip,
    Any,
}

/// The rules' patterns: parse_date.re's definitions, `'…'` case-insensitive and `"…"` case-sensitive.
fn rules() -> Vec<(Rule, String)> {
    let i = |words: &str| format!("(?i:{words})");
    let space = r"(?:[ \t]+|(?:\xC2\xA0)+|(?:\xE2\x80\xAF)+)".to_string();
    let frac = r"\.[0-9]+";
    let hour24 = "(?:[01]?[0-9]|2[0-4])";
    let hour24lz = "(?:[01][0-9]|2[0-4])";
    let hour12 = "(?:0?[1-9]|1[0-2])";
    let minute = "(?:[0-5]?[0-9])";
    let minutelz = "(?:[0-5][0-9])";
    let second = format!("(?:{minute}|60)");
    let secondlz = format!("(?:{minutelz}|60)");
    let meridian = r"(?:[AaPp]\.?[Mm]\.?)[\x00\t ]";
    let tz = r"(?:\(?[A-Za-z]{1,6}\)?|[A-Z][a-z]+(?:[_/\-][A-Za-z]+)+)";
    let tzcorrection = format!(
        "(?:GMT)?[+-](?:{hour24}(?::?{minute})?|{hour24lz}{minutelz}{secondlz}|{hour24lz}:{minutelz}:{secondlz})"
    );
    let daysuf = "(?:st|nd|rd|th)";
    let month = "(?:0?[0-9]|1[0-2])";
    let day = format!("(?:(?:[0-2]?[0-9]|3[01]){daysuf}?)");
    let year = "[0-9]{1,4}";
    let year2 = "[0-9]{2}";
    let year4 = "[0-9]{4}";
    let year4withsign = "[+-]?[0-9]{4}";
    let yearx = "[+-][0-9]{5,19}";
    let dayofyear = "(?:00[1-9]|0[1-9][0-9]|[1-2][0-9][0-9]|3[0-5][0-9]|36[0-6])";
    let weekofyear = "(?:0[1-9]|[1-4][0-9]|5[0-3])";
    let monthlz = "(?:0[0-9]|1[0-2])";
    let daylz = "(?:0[0-9]|[1-2][0-9]|3[01])";
    let dayfulls = i("sundays|mondays|tuesdays|wednesdays|thursdays|fridays|saturdays");
    let dayfull = i("sunday|monday|tuesday|wednesday|thursday|friday|saturday");
    let dayabbr = i("sun|mon|tue|wed|thu|fri|sat|sun");
    let dayspecial = i("weekday|weekdays");
    let daytext = format!("(?:{dayfulls}|{dayfull}|{dayabbr}|{dayspecial})");
    let monthfull =
        i("january|february|march|april|may|june|july|august|september|october|november|december");
    let monthabbr = i("jan|feb|mar|apr|may|jun|jul|aug|sep|sept|oct|nov|dec");
    let monthroman = "(?:I|II|III|IV|V|VI|VII|VIII|IX|X|XI|XII)";
    let monthtext = format!("(?:{monthfull}|{monthabbr}|{monthroman})");
    let t = i("t");
    let timetiny12 = format!("{hour12}{space}?{meridian}");
    let timeshort12 = format!("{hour12}[:.]{minutelz}{space}?{meridian}");
    let timelong12 = format!("{hour12}[:.]{minute}[:.]{secondlz}{space}?{meridian}");
    let timetiny24 = format!("{t}{hour24}");
    let timeshort24 = format!("{t}?{hour24}[:.]{minute}");
    let timelong24 = format!("{t}?{hour24}[:.]{minute}[:.]{second}");
    let iso8601long = format!("{t}?{hour24}[:.]{minute}[:.]{second}{frac}");
    let iso8601normtz =
        format!("{t}?{hour24}[:.]{minute}[:.]{secondlz}{space}?(?:{tzcorrection}|{tz})");
    let gnunocolon = format!("{t}?{hour24lz}{minutelz}");
    let iso8601nocolon = format!("{t}?{hour24lz}{minutelz}{secondlz}");
    let americanshort = format!("{month}/{day}");
    let american = format!("{month}/{day}/{year}");
    let iso8601dateslash = format!("{year4}/{monthlz}/{daylz}/?");
    let dateslash = format!("{year4}/{month}/{day}");
    let iso8601date4 = format!("{year4withsign}-{monthlz}-{daylz}");
    let iso8601date2 = format!("{year2}-{monthlz}-{daylz}");
    let iso8601datex = format!("{yearx}-{monthlz}-{daylz}");
    let gnudateshorter = format!("{year4}-{month}");
    let gnudateshort = format!("{year}-{month}-{day}");
    let pointeddate4 = format!(r"{day}[.\t\-]{month}[.\-]{year4}");
    let pointeddate2 = format!(r"{day}[.\t]{month}\.{year2}");
    let datefull = format!(r"{day}[ \t.\-]*{monthtext}[ \t.\-]*{year}");
    let datenoday = format!(r"{monthtext}[ .\t\-]*{year4}");
    let datenodayrev = format!(r"{year4}[ .\t\-]*{monthtext}");
    let datetextual = format!(r"{monthtext}[ .\t\-]*{day}[,.stndrh\t ]+{year}");
    let datenoyear = format!(r"{monthtext}[ .\t\-]*{day}(?:[,.stndrh\t ]+|\x00)");
    let datenoyearrev = format!(r"{day}[ .\t\-]*{monthtext}");
    let datenocolon = format!("{year4}{monthlz}{daylz}");
    let soap = format!(
        "{year4}-{monthlz}-{daylz}T{hour24lz}:{minutelz}:{secondlz}{frac}(?:{tzcorrection})?"
    );
    let xmlrpc = format!("{year4}{monthlz}{daylz}T{hour24}:{minutelz}:{secondlz}");
    let xmlrpcnocolon = format!("{year4}{monthlz}{daylz}{t}{hour24}{minutelz}{secondlz}");
    let wddx = format!("{year4}-{month}-{day}T{hour24}:{minute}:{second}");
    let pgydotd = format!(r"{year4}[.\-]?{dayofyear}");
    let pgtextshort = format!("{monthabbr}-{daylz}-{year}");
    let pgtextreverse = format!("{year}-{monthabbr}-{daylz}");
    let mssqltime = format!("{hour12}:{minutelz}:{secondlz}[:.][0-9]+{meridian}");
    let isoweekday = format!("{year4}-?W{weekofyear}-?[0-7]");
    let isoweek = format!("{year4}-?W{weekofyear}");
    let exif = format!("{year4}:{monthlz}:{daylz} {hour24lz}:{minutelz}:{secondlz}");
    let firstdayof = i("first day of");
    let lastdayof = i("last day of");
    let backof = format!("{}{hour24}(?:{space}?{meridian})?", i("back of "));
    let frontof = format!("{}{hour24}(?:{space}?{meridian})?", i("front of "));
    let clf =
        format!("{day}/{monthabbr}/{year4}:{hour24lz}:{minutelz}:{secondlz}{space}{tzcorrection}");
    let timestamp = "@-?[0-9]+";
    let timestampms = r"@-?[0-9]+\.[0-9]{0,6}";
    let reltextnumber = i(
        "first|second|third|fourth|fifth|sixth|seventh|eight|eighth|ninth|tenth|eleventh|twelfth",
    );
    let reltexttext = i("next|last|previous|this");
    let reltextunit = format!(
        "(?:{}|{}|(?:{}{}?)|{}|{daytext})",
        i("ms"),
        i(r"\xC2\xB5s"),
        i(
            r"msec|millisecond|\xC2\xB5sec|microsecond|usec|sec|second|min|minute|hour|day|fortnight|forthnight|month|year"
        ),
        i("s"),
        i("weeks"),
    );
    let relnumber = r"(?:[+-]*[ \t]*[0-9]{1,13})";
    let relative = format!("{relnumber}{space}?(?:{reltextunit}|{})", i("week"));
    let relativetext = format!("(?:{reltextnumber}|{reltexttext}){space}{reltextunit}");
    let relativetextweek = format!("{reltexttext}{space}{}", i("week"));
    let weekdayof = format!(
        "(?:{reltextnumber}|{reltexttext}){space}(?:{dayfulls}|{dayfull}|{dayabbr}){space}{}",
        i("of")
    );
    let any = |parts: &[&str]| parts.join("|");
    vec![
        (Rule::Yesterday, i("yesterday")),
        (Rule::Now, i("now")),
        (Rule::Noon, i("noon")),
        (Rule::MidnightToday, i("midnight|today")),
        (Rule::Tomorrow, i("tomorrow")),
        (Rule::Timestamp, timestamp.to_string()),
        (Rule::TimestampMs, timestampms.to_string()),
        (Rule::FirstLastDayOf, any(&[&firstdayof, &lastdayof])),
        (Rule::BackFrontOf, any(&[&backof, &frontof])),
        (Rule::WeekdayOf, weekdayof),
        (Rule::Time, any(&[&timetiny12, &timeshort12, &timelong12])),
        (Rule::Time, mssqltime),
        (
            Rule::Time,
            any(&[&timetiny24, &timeshort24, &timelong24, &iso8601long]),
        ),
        (Rule::GnuNoColon, gnunocolon),
        (Rule::Time, iso8601nocolon),
        (Rule::Date, any(&[&americanshort, &american])),
        (
            Rule::IsoDateSigned,
            any(&[&iso8601date4, &iso8601dateslash, &dateslash]),
        ),
        (Rule::Date, iso8601date2),
        (Rule::IsoDateX, iso8601datex),
        (Rule::Date, gnudateshorter),
        (Rule::Date, gnudateshort),
        (Rule::Date, datefull),
        (Rule::Date, pointeddate4),
        (Rule::Date, pointeddate2),
        (Rule::Date, datenoday),
        (Rule::Date, datenodayrev),
        (Rule::Date, any(&[&datetextual, &datenoyear])),
        (Rule::Date, datenoyearrev),
        (Rule::Date, datenocolon),
        (
            Rule::DateTime,
            any(&[&xmlrpc, &xmlrpcnocolon, &soap, &wddx, &exif]),
        ),
        (Rule::Date, pgydotd),
        (Rule::IsoWeekDay, isoweekday),
        (Rule::IsoWeek, isoweek),
        (Rule::Date, pgtextshort),
        (Rule::Date, pgtextreverse),
        (Rule::DateTime, clf),
        (Rule::Year4, year4.to_string()),
        (Rule::Ago, i("ago")),
        (Rule::DayText, daytext),
        (Rule::RelativeTextWeek, relativetextweek),
        (Rule::RelativeText, relativetext),
        (Rule::MonthText, any(&[&monthfull, &monthabbr])),
        (Rule::Zone, any(&[&tzcorrection, tz])),
        (
            Rule::DateShortTime12,
            any(&[
                &format!("{datenoyear}{timeshort12}"),
                &format!("{datenoyear}{timelong12}"),
            ]),
        ),
        (
            Rule::DateShortTime24,
            any(&[
                &format!("{datenoyear}{timeshort24}"),
                &format!("{datenoyear}{timelong24}"),
                &format!("{datenoyear}{iso8601normtz}"),
            ]),
        ),
        (Rule::Relative, relative),
        (Rule::Skip, r"[.,]".to_string()),
        (Rule::Skip, space),
        (Rule::Skip, r"\x00|\n".to_string()),
        (Rule::Any, r"[\x00-\xFF]".to_string()),
    ]
}

/// The rules and their DFA, which reports every pattern that matches (re2c's longest-match semantics).
fn scanner() -> &'static (Vec<Rule>, dense::DFA<Vec<u32>>) {
    static SCANNER: OnceLock<(Vec<Rule>, dense::DFA<Vec<u32>>)> = OnceLock::new();
    SCANNER.get_or_init(|| {
        let (kinds, patterns): (Vec<Rule>, Vec<String>) = rules().into_iter().unzip();
        let dfa = dense::Builder::new()
            .configure(
                dense::Config::new()
                    .match_kind(MatchKind::All)
                    .start_kind(StartKind::Anchored)
                    .dfa_size_limit(None)
                    .determinize_size_limit(None),
            )
            .syntax(syntax::Config::new().unicode(false).utf8(false))
            .thompson(thompson::Config::new().utf8(false))
            .build_many(&patterns)
            .expect("timelib's scanner rules compile");
        (kinds, dfa)
    })
}

/// The rule re2c matches at `at`, and the end of its token: the longest match, the earliest rule among them.
fn longest(buffer: &[u8], at: usize) -> (Rule, usize) {
    let (kinds, dfa) = scanner();
    let mut state = dfa
        .start_state(&start::Config::new().anchored(Anchored::Yes))
        .expect("an anchored start state");
    let mut best = (Rule::Any, at + 1);
    let mut record = |state, end| {
        let rule = (0..dfa.match_len(state))
            .map(|k| dfa.match_pattern(state, k).as_usize())
            .min()
            .expect("a match state has a pattern");
        best = (kinds[rule], end);
    };
    for (offset, &byte) in buffer[at..].iter().enumerate() {
        state = dfa.next_state(state, byte);
        if dfa.is_match_state(state) {
            record(state, at + offset);
        } else if dfa.is_dead_state(state) {
            break;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::construct;

    /// `DateTime::getLastErrors()['errors']` after `new DateTime($s)` on PHP 8.5.12 (timelib 2022.17), exported
    /// by scripts/export-date-time-errors.php.
    #[test]
    fn matches_php() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/date-time/errors.json"
        );
        let cases: Vec<(String, Vec<String>)> =
            serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let mut wrong = Vec::new();
        for (input, want) in &cases {
            let bytes = input.chars().map(|c| c as u32 as u8).collect::<Vec<_>>();
            let got = construct(&bytes);
            if got != *want {
                wrong.push(format!("{input:?}: want {want:?}, got {got:?}"));
            }
        }
        assert!(
            wrong.is_empty(),
            "{} of {} differ:\n{}",
            wrong.len(),
            cases.len(),
            wrong.join("\n")
        );
    }
}
