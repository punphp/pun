//! Port of `PhpCsFixer\Preg`: preg_* with PHP's exact semantics (pattern syntax with delimiters and
//! modifiers, `/u` added first and dropped when the subject or pattern is not valid UTF-8, Perl-style
//! empty-match advancing, `$1`/`\1`/`${1}` replacement references, PREG_SPLIT_* flags).
//!
//! Patterns and subjects are bytes (`impl AsRef<[u8]>`, so `&str` works). Errors that make upstream
//! throw `PregException` panic, matching the crate's "panic where upstream throws" rule.
//!
//! ```ignore
//! use crate::analyzers::preg::{self, Preg};
//! if Preg::r#match("/^\\s+$/", content) { .. }
//! let m = Preg::match_groups("/(a)(b)?/", s, 0).unwrap(); m.str(1); m.exists(2);
//! let out = Preg::replace("/\\R/", "\n", s);
//! ```
#![allow(non_upper_case_globals)]

use pcre2_sys::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::rc::Rc;

pub const PREG_OFFSET_CAPTURE: u32 = 256;
pub const PREG_SPLIT_NO_EMPTY: u32 = 1;
pub const PREG_SPLIT_DELIM_CAPTURE: u32 = 2;

/// PHP's pcre.backtrack_limit / pcre.recursion_limit defaults.
const BACKTRACK_LIMIT: u32 = 1_000_000;
const RECURSION_LIMIT: u32 = 100_000;

/// A compiled pattern (PHP's pcre_cache_entry).
struct Regex {
    code: *mut pcre2_code_8,
    utf: bool,
    capture_count: usize,
    /// (name, group number) in PCRE2 name-table order (sorted by name), like PHP's subpat_names.
    names: Rc<[(Vec<u8>, usize)]>,
    /// Match data + context, allocated once per compiled pattern (matches never nest on one thread).
    matcher: Matcher,
}

impl Drop for Regex {
    fn drop(&mut self) {
        unsafe { pcre2_code_free_8(self.code) }
    }
}

struct Matcher {
    md: *mut pcre2_match_data_8,
    mctx: *mut pcre2_match_context_8,
}

impl Drop for Matcher {
    fn drop(&mut self) {
        unsafe {
            pcre2_match_data_free_8(self.md);
            pcre2_match_context_free_8(self.mctx);
        }
    }
}

/// Pattern-cache hasher: patterns run to kilobytes (TypeExpression's is ~6 KB) and are looked up on every call, so
/// only the length and three 8-byte words (start, middle, end) are hashed; key equality still compares every byte.
#[derive(Default)]
struct EndsHasher(u64);

impl Hasher for EndsHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, b: &[u8]) {
        let word = |at: usize| {
            let mut w = [0u8; 8];
            let end = b.len().min(at + 8);
            w[..end - at].copy_from_slice(&b[at..end]);
            u64::from_le_bytes(w)
        };
        let n = b.len();
        for x in [n as u64, word(0), word(n / 2), word(n.saturating_sub(8))] {
            self.0 = (self.0.rotate_left(5) ^ x).wrapping_mul(0x517c_c1b7_2722_0a95);
        }
    }
}

/// Byte offsets of each group (`None`: did not participate).
type Groups = Vec<Option<(usize, usize)>>;

type PatternMap<V> = HashMap<Vec<u8>, V, BuildHasherDefault<EndsHasher>>;

thread_local! {
    static CACHE: RefCell<PatternMap<Option<Rc<Regex>>>> = RefCell::default();
    /// with_fallback's two compiled variants (`u` added, `u` removed) per pattern as given.
    static VARIANTS: RefCell<PatternMap<[Option<Rc<Regex>>; 2]>> = RefCell::default();
    static JIT_STACK: *mut pcre2_jit_stack_8 = unsafe { pcre2_jit_stack_create_8(32 * 1024, 192 * 1024, std::ptr::null_mut()) };
}

/// php_pcre.c pcre_get_compiled_regex_cache: parse delimiters + modifiers and compile.
/// `None` = PHP would warn and return false (bad delimiter, unknown modifier, compile error).
fn compile(pattern: &[u8]) -> Option<Rc<Regex>> {
    if let Some(r) = CACHE.with(|c| c.borrow().get(pattern).cloned()) {
        return r;
    }
    let r = compile_uncached(pattern).map(Rc::new);
    CACHE.with(|c| c.borrow_mut().insert(pattern.to_vec(), r.clone()));
    r
}

fn compile_uncached(pattern: &[u8]) -> Option<Regex> {
    let mut p = 0;
    while p < pattern.len()
        && matches!(pattern[p], b' ' | b'\t' | b'\n' | b'\r' | b'\x0B' | b'\x0C')
    {
        p += 1;
    }
    if p >= pattern.len() {
        return None;
    }
    let delimiter = pattern[p];
    if delimiter.is_ascii_alphanumeric() || delimiter == b'\\' || delimiter == 0 {
        return None;
    }
    p += 1;
    let start = p;
    let end_delimiter = match delimiter {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        d => d,
    };
    if end_delimiter == delimiter {
        while p < pattern.len() {
            if pattern[p] == b'\\' && p + 1 < pattern.len() {
                p += 1;
            } else if pattern[p] == delimiter {
                break;
            }
            p += 1;
        }
    } else {
        // Bracket-style: track nesting.
        let mut brackets = 1;
        while p < pattern.len() {
            if pattern[p] == b'\\' && p + 1 < pattern.len() {
                p += 1;
            } else if pattern[p] == end_delimiter {
                brackets -= 1;
                if brackets == 0 {
                    break;
                }
            } else if pattern[p] == delimiter {
                brackets += 1;
            }
            p += 1;
        }
    }
    if p >= pattern.len() {
        return None; // no ending delimiter
    }
    let body = &pattern[start..p];
    let mut options = 0u32;
    let mut utf = false;
    for &m in &pattern[p + 1..] {
        match m {
            b'i' => options |= PCRE2_CASELESS,
            b'm' => options |= PCRE2_MULTILINE,
            b'n' => options |= PCRE2_NO_AUTO_CAPTURE,
            b's' => options |= PCRE2_DOTALL,
            b'x' => options |= PCRE2_EXTENDED,
            b'A' => options |= PCRE2_ANCHORED,
            b'D' => options |= PCRE2_DOLLAR_ENDONLY,
            b'S' | b'X' => {}
            b'U' => options |= PCRE2_UNGREEDY,
            b'u' => {
                options |= PCRE2_UTF | PCRE2_UCP;
                utf = true;
            }
            b'J' => options |= PCRE2_DUPNAMES,
            b' ' | b'\n' | b'\r' => {}
            _ => return None, // 'e' and 'r' are not used by upstream; unknown modifier
        }
    }
    unsafe {
        let mut err: libc::c_int = 0;
        let mut erroff: usize = 0;
        let code = pcre2_compile_8(
            body.as_ptr(),
            body.len(),
            options,
            &mut err,
            &mut erroff,
            std::ptr::null_mut(),
        );
        if code.is_null() {
            return None;
        }
        pcre2_jit_compile_8(code, PCRE2_JIT_COMPLETE);
        let mut capture_count: u32 = 0;
        pcre2_pattern_info_8(
            code,
            PCRE2_INFO_CAPTURECOUNT,
            &mut capture_count as *mut u32 as *mut libc::c_void,
        );
        let mut name_count: u32 = 0;
        let mut entry_size: u32 = 0;
        let mut table: *const u8 = std::ptr::null();
        pcre2_pattern_info_8(
            code,
            PCRE2_INFO_NAMECOUNT,
            &mut name_count as *mut u32 as *mut libc::c_void,
        );
        pcre2_pattern_info_8(
            code,
            PCRE2_INFO_NAMEENTRYSIZE,
            &mut entry_size as *mut u32 as *mut libc::c_void,
        );
        pcre2_pattern_info_8(
            code,
            PCRE2_INFO_NAMETABLE,
            &mut table as *mut *const u8 as *mut libc::c_void,
        );
        let mut names = Vec::new();
        for i in 0..name_count as usize {
            let entry =
                std::slice::from_raw_parts(table.add(i * entry_size as usize), entry_size as usize);
            let group = ((entry[0] as usize) << 8) | entry[1] as usize;
            let name = &entry[2..];
            let name = &name[..name.iter().position(|&b| b == 0).unwrap_or(name.len())];
            names.push((name.to_vec(), group));
        }
        let matcher = Matcher::new(code);
        debug_assert!(names.is_sorted_by(|a, b| a.0 <= b.0));
        Some(Regex {
            code,
            utf,
            capture_count: capture_count as usize,
            names: names.into(),
            matcher,
        })
    }
}

impl Matcher {
    fn new(code: *const pcre2_code_8) -> Matcher {
        unsafe {
            let md = pcre2_match_data_create_from_pattern_8(code, std::ptr::null_mut());
            let mctx = pcre2_match_context_create_8(std::ptr::null_mut());
            pcre2_set_match_limit_8(mctx, BACKTRACK_LIMIT);
            pcre2_set_depth_limit_8(mctx, RECURSION_LIMIT);
            JIT_STACK.with(|s| pcre2_jit_stack_assign_8(mctx, None, *s as *mut libc::c_void));
            Matcher { md, mctx }
        }
    }
}

impl Regex {
    fn matcher(&self) -> &Matcher {
        &self.matcher
    }

    /// One pcre2_match call. Ok(None) = no match, Err = PHP's preg_last_error() != PREG_NO_ERROR.
    fn exec(
        &self,
        m: &Matcher,
        subject: &[u8],
        offset: usize,
        options: u32,
    ) -> Result<Option<Groups>, i32> {
        let rc = unsafe {
            pcre2_match_8(
                self.code,
                subject.as_ptr(),
                subject.len(),
                offset,
                options | PCRE2_NO_UTF_CHECK,
                m.md,
                m.mctx,
            )
        };
        if rc == PCRE2_ERROR_NOMATCH {
            return Ok(None);
        }
        if rc < 0 {
            return Err(rc);
        }
        let ov = unsafe {
            std::slice::from_raw_parts(
                pcre2_get_ovector_pointer_8(m.md),
                2 * (self.capture_count + 1),
            )
        };
        // rc = highest set group + 1; groups at or above rc are unset (like PHP's `count`).
        let groups = (0..=self.capture_count)
            .map(|i| {
                if i < rc as usize && ov[2 * i] != usize::MAX {
                    Some((ov[2 * i], ov[2 * i + 1]))
                } else {
                    None
                }
            })
            .collect();
        Ok(Some(groups))
    }

    fn unit_len(&self, subject: &[u8], at: usize) -> usize {
        if !self.utf {
            return 1;
        }
        // `at` is on a character boundary (with_fallback drops `u` otherwise), so never a continuation byte
        match subject[at] {
            0xF0.. => 4,
            0xE0.. => 3,
            0xC0.. => 2,
            _ => 1,
        }
    }

    /// All matches of a /g loop (php_pcre_match_impl global, preg_replace, preg_split share this logic),
    /// stopping after `limit` matches when Some.
    fn exec_all(
        &self,
        subject: &[u8],
        start: usize,
        limit: Option<usize>,
    ) -> Result<Vec<Groups>, i32> {
        let m = self.matcher();
        let mut out = Vec::new();
        let mut offset = start;
        let mut options = 0;
        loop {
            if limit.is_some_and(|l| out.len() >= l) {
                break;
            }
            match self.exec(m, subject, offset, options)? {
                Some(g) => {
                    let (s, e) = g[0].unwrap();
                    out.push(g);
                    offset = e;
                    options = if s == e {
                        PCRE2_NOTEMPTY_ATSTART | PCRE2_ANCHORED
                    } else {
                        0
                    };
                }
                None => {
                    if options == 0 || offset >= subject.len() {
                        break;
                    }
                    offset += self.unit_len(subject, offset);
                    options = 0;
                }
            }
        }
        Ok(out)
    }
}

/// `$matches` of one match.
#[derive(Clone, Debug)]
pub struct Captures<'s> {
    subject: &'s [u8],
    groups: Groups,
    /// Number of numeric entries PHP puts in `$matches` (`count` from pcre2_match: trailing unmatched groups omitted).
    len: usize,
    names: Rc<[(Vec<u8>, usize)]>,
}

impl<'s> Captures<'s> {
    fn new(subject: &'s [u8], groups: Groups, re: &Regex) -> Self {
        let len = groups
            .iter()
            .rposition(Option::is_some)
            .map_or(0, |i| i + 1);
        Captures {
            subject,
            groups,
            len,
            names: re.names.clone(),
        }
    }

    /// Group text, `None` when the group did not participate (PHP: "" or null).
    pub fn get(&self, i: usize) -> Option<&'s [u8]> {
        self.groups
            .get(i)
            .copied()
            .flatten()
            .map(|(s, e)| &self.subject[s..e])
    }

    /// Group text as PHP gives it ("" for unmatched).
    pub fn str(&self, i: usize) -> &'s [u8] {
        self.get(i).unwrap_or(b"")
    }

    /// Byte offset of the group, -1 when unmatched (PREG_OFFSET_CAPTURE).
    pub fn offset(&self, i: usize) -> isize {
        self.groups
            .get(i)
            .copied()
            .flatten()
            .map_or(-1, |(s, _)| s as isize)
    }

    /// Number of numeric entries in `$matches` (never 0: group 0 always matched).
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// With duplicate names (`J`) PHP's `$matches['name']` is the last group of that name present in `$matches`.
    fn group_by_name(&self, name: &[u8]) -> Option<usize> {
        // pcre2api: the name table is sorted by name (byte order, a prefix first), so equal names are one run.
        let lo = self.names.partition_point(|(n, _)| n.as_slice() < name);
        self.names[lo..]
            .iter()
            .take_while(|(n, _)| n == name)
            .filter(|&&(_, g)| g < self.len)
            .map(|&(_, g)| g)
            .max()
    }

    /// Named group text (`$matches['name']`), `None` when unmatched or absent.
    pub fn name(&self, name: &str) -> Option<&'s [u8]> {
        self.get(self.group_by_name(name.as_bytes())?)
    }

    /// `$matches['name'] ?? ''`.
    pub fn name_str(&self, name: &str) -> &'s [u8] {
        self.name(name).unwrap_or(b"")
    }

    /// `array_key_exists('name', $matches)`.
    pub fn name_exists(&self, name: &str) -> bool {
        self.group_by_name(name.as_bytes()).is_some()
    }

    pub fn name_offset(&self, name: &str) -> isize {
        self.group_by_name(name.as_bytes())
            .map_or(-1, |g| self.offset(g))
    }
}

/// One split piece (with PREG_SPLIT_OFFSET_CAPTURE the offset is meaningful).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub text: Vec<u8>,
    pub offset: usize,
}

pub struct Preg;

/// `Preg::match('/\R/', $s)`: CR, LF, VT or FF; with the `/u` attempt (valid UTF-8) also U+0085, U+2028 and U+2029,
/// otherwise (the non-`u` fallback) the byte 0x85.
fn has_line_break(s: &[u8]) -> bool {
    if s.iter().any(|b| matches!(b, b'\n' | b'\r' | 0x0b | 0x0c)) {
        return true;
    }
    if s.is_ascii() {
        return false;
    }
    match std::str::from_utf8(s) {
        Ok(t) => t.contains(['\u{85}', '\u{2028}', '\u{2029}']),
        Err(_) => s.contains(&0x85),
    }
}

fn add_utf8_modifier(pattern: &[u8]) -> Vec<u8> {
    let mut p = pattern.to_vec();
    p.push(b'u');
    p
}

fn remove_utf8_modifier(pattern: &[u8]) -> Vec<u8> {
    if pattern.is_empty() {
        return Vec::new();
    }
    let delimiter = pattern[0];
    let end = pattern.iter().rposition(|&b| b == delimiter).unwrap();
    let mut out = pattern[..end].to_vec();
    out.extend(pattern[end..].iter().filter(|&&b| b != b'u'));
    out
}

/// Runs `f` with the `u` variant of the pattern, then the non-`u` variant, like every Preg method.
/// PHP fails the `u` attempt when the pattern does not compile (e.g. invalid UTF-8 in it), the
/// subject is not valid UTF-8, or the offset is not on a character boundary.
fn with_fallback<T>(
    pattern: &[u8],
    subject: &[u8],
    offset: usize,
    method: &str,
    f: impl Fn(&Regex) -> Result<T, i32>,
) -> T {
    let variants = VARIANTS
        .with(|v| v.borrow().get(pattern).cloned())
        .unwrap_or_else(|| {
            let v = [
                compile(&add_utf8_modifier(pattern)),
                compile(&remove_utf8_modifier(pattern)),
            ];
            VARIANTS.with(|m| m.borrow_mut().insert(pattern.to_vec(), v.clone()));
            v
        });
    for re in variants {
        let Some(re) = re else { continue };
        if re.utf
            && (std::str::from_utf8(subject).is_err()
                || (offset < subject.len() && (subject[offset] & 0xC0) == 0x80))
        {
            continue;
        }
        if let Ok(r) = f(&re) {
            return r;
        }
    }
    panic!(
        "PregException: error occurred when calling {method} with pattern {:?}",
        String::from_utf8_lossy(pattern)
    );
}

impl Preg {
    /// `Preg::match($pattern, $subject)`.
    pub fn r#match(pattern: impl AsRef<[u8]>, subject: impl AsRef<[u8]>) -> bool {
        // `/\R/` (a line break anywhere) is the hottest pattern, run on nearly every whitespace token.
        if pattern.as_ref() == br"/\R/" {
            return has_line_break(subject.as_ref());
        }
        Self::match_groups(pattern, subject.as_ref(), 0).is_some()
    }

    /// `Preg::match($pattern, $subject, $matches, $flags)`; `None` when no match. `flags` is only
    /// documentation at call sites: offsets (PREG_OFFSET_CAPTURE) are always available on Captures.
    pub fn match_groups<'s>(
        pattern: impl AsRef<[u8]>,
        subject: &'s [u8],
        flags: u32,
    ) -> Option<Captures<'s>> {
        Self::match_at(pattern, subject, flags, 0)
    }

    /// `Preg::match($pattern, $subject, $matches, $flags, $offset)`.
    pub fn match_at<'s>(
        pattern: impl AsRef<[u8]>,
        subject: &'s [u8],
        _flags: u32,
        offset: usize,
    ) -> Option<Captures<'s>> {
        let pattern = pattern.as_ref();
        with_fallback(pattern, subject, offset, "PhpCsFixer\\Preg::match", |re| {
            let m = re.matcher();
            Ok(re
                .exec(m, subject, offset, 0)?
                .map(|g| Captures::new(subject, g, re)))
        })
    }

    /// `Preg::matchAll($pattern, $subject, $matches, PREG_SET_ORDER)`: every match in order (its
    /// length is matchAll's return value). PREG_PATTERN_ORDER callers read `m.str(i)` of every match.
    pub fn match_all<'s>(
        pattern: impl AsRef<[u8]>,
        subject: &'s [u8],
        _flags: u32,
    ) -> Vec<Captures<'s>> {
        let pattern = pattern.as_ref();
        with_fallback(pattern, subject, 0, "PhpCsFixer\\Preg::matchAll", |re| {
            Ok(re
                .exec_all(subject, 0, None)?
                .into_iter()
                .map(|g| Captures::new(subject, g, re))
                .collect())
        })
    }

    /// `Preg::replace($pattern, $replacement, $subject)`.
    pub fn replace(
        pattern: impl AsRef<[u8]>,
        replacement: impl AsRef<[u8]>,
        subject: impl AsRef<[u8]>,
    ) -> Vec<u8> {
        Self::replace_limit(pattern, replacement, subject, -1).0
    }

    /// `Preg::replace($pattern, $replacement, $subject, $limit, $count)` -> (result, count).
    pub fn replace_limit(
        pattern: impl AsRef<[u8]>,
        replacement: impl AsRef<[u8]>,
        subject: impl AsRef<[u8]>,
        limit: isize,
    ) -> (Vec<u8>, usize) {
        let replacement = replacement.as_ref();
        Self::replace_impl(
            pattern.as_ref(),
            subject.as_ref(),
            limit,
            "PhpCsFixer\\Preg::replace",
            &|c: &Captures| expand_replacement(replacement, c),
        )
    }

    /// `Preg::replaceCallback($pattern, $callback, $subject)`.
    pub fn replace_callback(
        pattern: impl AsRef<[u8]>,
        callback: impl Fn(&Captures) -> Vec<u8>,
        subject: impl AsRef<[u8]>,
    ) -> Vec<u8> {
        Self::replace_callback_limit(pattern, callback, subject, -1).0
    }

    pub fn replace_callback_limit(
        pattern: impl AsRef<[u8]>,
        callback: impl Fn(&Captures) -> Vec<u8>,
        subject: impl AsRef<[u8]>,
        limit: isize,
    ) -> (Vec<u8>, usize) {
        Self::replace_impl(
            pattern.as_ref(),
            subject.as_ref(),
            limit,
            "PhpCsFixer\\Preg::replaceCallback",
            &callback,
        )
    }

    fn replace_impl(
        pattern: &[u8],
        subject: &[u8],
        limit: isize,
        method: &str,
        f: &dyn Fn(&Captures) -> Vec<u8>,
    ) -> (Vec<u8>, usize) {
        with_fallback(pattern, subject, 0, method, |re| {
            let limit = (limit >= 0).then_some(limit as usize);
            let all = re.exec_all(subject, 0, limit)?;
            let mut out = Vec::with_capacity(subject.len());
            let mut last = 0;
            let count = all.len();
            for g in all {
                let (s, e) = g[0].unwrap();
                out.extend_from_slice(&subject[last..s]);
                out.extend(f(&Captures::new(subject, g, re)));
                last = e;
            }
            out.extend_from_slice(&subject[last..]);
            Ok((out, count))
        })
    }

    /// `Preg::split($pattern, $subject, $limit, $flags)`.
    pub fn split(
        pattern: impl AsRef<[u8]>,
        subject: impl AsRef<[u8]>,
        limit: isize,
        flags: u32,
    ) -> Vec<Piece> {
        let (pattern, subject) = (pattern.as_ref(), subject.as_ref());
        with_fallback(pattern, subject, 0, "PhpCsFixer\\Preg::split", |re| {
            split_impl(re, subject, limit, flags)
        })
    }

    /// `Preg::split` returning just the strings.
    pub fn split_str(pattern: impl AsRef<[u8]>, subject: impl AsRef<[u8]>) -> Vec<Vec<u8>> {
        Self::split(pattern, subject, -1, 0)
            .into_iter()
            .map(|p| p.text)
            .collect()
    }
}

/// `preg_quote($str, $delimiter)`.
pub fn preg_quote(s: &[u8], delimiter: Option<u8>) -> Vec<u8> {
    let mut out = Vec::with_capacity(s.len());
    for &b in s {
        match b {
            b'.' | b'\\' | b'+' | b'*' | b'?' | b'[' | b'^' | b']' | b'$' | b'(' | b')' | b'{'
            | b'}' | b'=' | b'!' | b'>' | b'<' | b'|' | b':' | b'-' | b'#' => {
                out.push(b'\\');
                out.push(b);
            }
            0 => out.extend_from_slice(b"\\000"),
            _ if Some(b) == delimiter => {
                out.push(b'\\');
                out.push(b);
            }
            _ => out.push(b),
        }
    }
    out
}

/// php_pcre_split_impl.
fn split_impl(re: &Regex, subject: &[u8], limit: isize, flags: u32) -> Result<Vec<Piece>, i32> {
    let no_empty = flags & PREG_SPLIT_NO_EMPTY != 0;
    let delim_capture = flags & PREG_SPLIT_DELIM_CAPTURE != 0;
    let mut limit_val = if limit == 0 { -1 } else { limit };
    let mut out = Vec::new();
    let push = |out: &mut Vec<Piece>, s: usize, e: usize| {
        out.push(Piece {
            text: subject[s..e].to_vec(),
            offset: s,
        })
    };
    let m = re.matcher();
    let mut offset = 0;
    let mut last_match_offset = 0;
    let mut options = 0;
    if limit_val == -1 || limit_val > 1 {
        loop {
            match re.exec(m, subject, offset, options)? {
                Some(g) => {
                    let (s, e) = g[0].unwrap();
                    if !no_empty || s != last_match_offset {
                        push(&mut out, last_match_offset, s);
                        if limit_val != -1 {
                            limit_val -= 1;
                        }
                    }
                    if delim_capture {
                        let count = g.iter().rposition(Option::is_some).map_or(0, |i| i + 1);
                        for grp in g.iter().take(count).skip(1) {
                            let (gs, ge) = grp.unwrap_or((0, 0));
                            if !no_empty || gs != ge {
                                push(&mut out, gs, ge);
                            }
                        }
                    }
                    offset = e;
                    last_match_offset = e;
                    if s == e {
                        if limit_val != -1 && limit_val <= 1 {
                            break;
                        }
                        options = PCRE2_NOTEMPTY_ATSTART | PCRE2_ANCHORED;
                    } else {
                        options = 0;
                    }
                }
                None => {
                    if options != 0 && offset < subject.len() {
                        offset += re.unit_len(subject, offset);
                        options = 0;
                    } else {
                        break;
                    }
                }
            }
            if !(limit_val == -1 || limit_val > 1) {
                break;
            }
        }
    }
    let start = last_match_offset;
    if !no_empty || start < subject.len() {
        push(&mut out, start, subject.len());
    }
    Ok(out)
}

/// preg_replace's replacement expansion (`\n`, `$n`, `${n}`, n in 0..=99).
fn expand_replacement(replacement: &[u8], c: &Captures) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    // PHP's `walk_last`: the last literally copied byte; `\\` / `\$` after a copied `\\` replaces it.
    let mut walk_last = 0u8;
    while i < replacement.len() {
        let ch = replacement[i];
        if (ch == b'\\' || ch == b'$') && walk_last == b'\\' {
            *out.last_mut().unwrap() = ch;
            walk_last = 0;
            i += 1;
            continue;
        }
        if (ch == b'\\' || ch == b'$') && i + 1 < replacement.len() {
            let mut w = i;
            let in_brace = ch == b'$' && replacement[w + 1] == b'{';
            if in_brace {
                w += 1;
            }
            w += 1;
            if w < replacement.len() && replacement[w].is_ascii_digit() {
                let mut backref = (replacement[w] - b'0') as usize;
                w += 1;
                if w < replacement.len() && replacement[w].is_ascii_digit() {
                    backref = backref * 10 + (replacement[w] - b'0') as usize;
                    w += 1;
                }
                let ok = if in_brace {
                    if w < replacement.len() && replacement[w] == b'}' {
                        w += 1;
                        true
                    } else {
                        false
                    }
                } else {
                    true
                };
                if ok {
                    out.extend_from_slice(c.str(backref));
                    i = w;
                    walk_last = 0;
                    continue;
                }
            }
        }
        out.push(ch);
        walk_last = ch;
        i += 1;
    }
    out
}

/// Plain PHP `preg_*` calls (no `/u` added, no fallback), for ports of code that calls the PHP
/// functions directly instead of `PhpCsFixer\Preg` (e.g. Pint's Blade formatters). A pattern PHP
/// cannot compile, or a match error, behaves like PHP's `false`/`null` return: no match / `None`.
pub struct RawPreg;

impl RawPreg {
    fn run<T>(pattern: &[u8], subject: &[u8], f: impl Fn(&Regex) -> Result<T, i32>) -> Option<T> {
        let re = compile(pattern)?;
        if re.utf && std::str::from_utf8(subject).is_err() {
            return None;
        }
        f(&re).ok()
    }

    /// `preg_match($pattern, $subject, $matches) === 1`.
    pub fn match_groups<'s>(pattern: impl AsRef<[u8]>, subject: &'s [u8]) -> Option<Captures<'s>> {
        Self::run(pattern.as_ref(), subject, |re| {
            let m = re.matcher();
            Ok(re
                .exec(m, subject, 0, 0)?
                .map(|g| Captures::new(subject, g, re)))
        })
        .flatten()
    }

    pub fn r#match(pattern: impl AsRef<[u8]>, subject: impl AsRef<[u8]>) -> bool {
        Self::match_groups(pattern, subject.as_ref()).is_some()
    }

    /// `preg_match_all($pattern, $subject, $m, PREG_SET_ORDER | PREG_OFFSET_CAPTURE)`: every match;
    /// use `Captures::offset` for offsets.
    pub fn match_all<'s>(pattern: impl AsRef<[u8]>, subject: &'s [u8]) -> Vec<Captures<'s>> {
        Self::run(pattern.as_ref(), subject, |re| {
            Ok(re
                .exec_all(subject, 0, None)?
                .into_iter()
                .map(|g| Captures::new(subject, g, re))
                .collect())
        })
        .unwrap_or_default()
    }

    /// `preg_replace_callback`; `None` = PHP's `null` (error).
    pub fn replace_callback(
        pattern: impl AsRef<[u8]>,
        callback: impl Fn(&Captures) -> Vec<u8>,
        subject: impl AsRef<[u8]>,
    ) -> Option<Vec<u8>> {
        let subject = subject.as_ref();
        Self::run(pattern.as_ref(), subject, |re| {
            let all = re.exec_all(subject, 0, None)?;
            let mut out = Vec::with_capacity(subject.len());
            let mut last = 0;
            for g in all {
                let (s, e) = g[0].unwrap();
                out.extend_from_slice(&subject[last..s]);
                out.extend(callback(&Captures::new(subject, g, re)));
                last = e;
            }
            out.extend_from_slice(&subject[last..]);
            Ok(out)
        })
    }

    /// `preg_replace`; `None` = PHP's `null` (error).
    pub fn replace(
        pattern: impl AsRef<[u8]>,
        replacement: impl AsRef<[u8]>,
        subject: impl AsRef<[u8]>,
    ) -> Option<Vec<u8>> {
        let replacement = replacement.as_ref();
        Self::replace_callback(pattern, |c| expand_replacement(replacement, c), subject)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn line_break_fast_path() {
        // PhpCsFixer\Preg::match('/\R/', $s) on PHP 8.5.
        let cases: &[(&[u8], bool)] = &[
            (b"", false),
            (b"  ", false),
            (b"a\nb", true),
            (b"\r", true),
            (b"\x0b", true),
            (b"\x0c", true),
            ("é".as_bytes(), false),
            (b"\xc2\x85", true),
            (b"\x85", true),
            (b"a\xe2\x80\xa8", true),
            (b"\xe2\x80\xa9", true),
            (b"\xff\x85", true),
            (b"\xff", false),
        ];
        for &(s, want) in cases {
            assert_eq!(super::Preg::r#match(r"/\R/", s), want, "{s:?}");
        }
    }

    use super::*;

    // Expected values produced by PHP 8.5 / PCRE2 10.46.
    #[test]
    fn php_semantics() {
        let s = |v: Vec<Piece>| {
            v.into_iter()
                .map(|p| String::from_utf8(p.text).unwrap())
                .collect::<Vec<_>>()
        };
        assert_eq!(s(Preg::split("//", "abc", -1, 0)), ["", "a", "b", "c", ""]);
        assert_eq!(
            s(Preg::split("/(,)/", "a,b,,c", -1, PREG_SPLIT_DELIM_CAPTURE)),
            ["a", ",", "b", ",", "", ",", "c"]
        );
        assert_eq!(
            s(Preg::split("/,/", "a,b,,c", -1, PREG_SPLIT_NO_EMPTY)),
            ["a", "b", "c"]
        );
        assert_eq!(s(Preg::split("/,/", "a,b,c", 2, 0)), ["a", "b,c"]);
        assert_eq!(s(Preg::split("/x*/", "axb", -1, 0)), ["", "a", "", "b", ""]);
        assert_eq!(Preg::replace("/x*/", "-", "abc"), b"-a-b-c-");
        assert_eq!(Preg::replace("/x*/", "-", "axxb"), b"-a--b-");
        assert_eq!(
            Preg::replace("/(a)(b)?/", "[$2|\\1|${1}0]", "ac"),
            b"[|a|a0]c"
        );
        assert_eq!(Preg::replace("/./", "x", "é"), b"x"); // /u added
        assert_eq!(Preg::replace("/./", "x", b"\xe9"), b"x"); // invalid UTF-8: falls back
        assert_eq!(Preg::replace("/./", "x", b"\xff\xfe"), b"xx");
        assert_eq!(
            Preg::replace_limit("/a/", "b", "aaa", 2),
            (b"bba".to_vec(), 2)
        );
        let m = Preg::match_groups("/(a)(b)?(c)?/", b"a", 0).unwrap();
        assert_eq!(m.len(), 2);
        let m = Preg::match_groups("/(a)(x)?(c)/", b"ac", 0).unwrap();
        assert_eq!((m.len(), m.str(2), m.offset(2)), (4, &b""[..], -1));
        let m = Preg::match_groups("/(?<n>b)/", b"ab", 0).unwrap();
        assert_eq!((m.name("n"), m.name_offset("n")), (Some(&b"b"[..]), 1));
        assert!(Preg::r#match(
            "{[/\\\\](stub|fixture)s?[/\\\\]}i",
            "a/Fixtures/b"
        ));
        assert!(
            Preg::r#match("/a$/D", "a")
                && !Preg::r#match("/a$/D", "a\n")
                && Preg::r#match("/a$/", "a\n")
        );
        assert_eq!(Preg::match_all("/\\d/", b"a1b2", 0).len(), 2);
        assert_eq!(Preg::replace("/^/m", ">", "a\nb"), b">a\n>b");
        assert_eq!(Preg::replace("/\\R/", "|", "a\r\nb\nc"), b"a|b|c");
        assert_eq!(preg_quote(b"a.b/c\0", Some(b'/')), b"a\\.b\\/c\\000");
    }

    // Expected values from PhpCsFixer\Preg on PHP 8.5 (delimiters, modifiers, PregException cases, split/replace edges).
    #[test]
    fn php_semantics_delimiters_modifiers_errors() {
        for (p, s, m0) in [
            ("(a)", "a", "a"),
            ("[A]i", "xa", "a"),
            ("{a{2}}", "aa", "aa"),
            ("{a\\}b}", "a}b", "a}b"),
            ("<a>", "a", "a"),
            ("/a\\/b/", "a/b", "a/b"),
            ("/a b # c/x", "ab", "ab"),
            ("/a/S", "a", "a"),
            ("/a/X", "a", "a"),
            ("/a+?/U", "aa", "aa"),
            ("/a/ \n", "a", "a"),
        ] {
            assert_eq!(
                Preg::match_groups(p, s.as_bytes(), 0).map(|m| m.str(0).to_vec()),
                Some(m0.as_bytes().to_vec()),
                "{p}"
            );
        }
        assert!(!Preg::r#match("/b/A", "ab"));
        let m = Preg::match_groups("/(a)(?<n>b)?/n", b"ab", 0).unwrap();
        assert_eq!(
            (m.len(), m.str(1), m.name("n")),
            (2, &b"b"[..], Some(&b"b"[..]))
        );
        let m = Preg::match_groups("/(?<x>a)|(?<x>b)/J", b"b", 0).unwrap();
        assert_eq!(
            (m.str(1), m.str(2), m.name_str("x")),
            (&b""[..], &b"b"[..], &b"b"[..])
        );
        let m = Preg::match_groups("/(?<x>a)|(?<x>b)/J", b"a", 0).unwrap(); // group 2 not in $matches
        assert_eq!(
            (
                m.len(),
                m.name_str("x"),
                m.name_exists("x"),
                m.name_exists("y")
            ),
            (2, &b"a"[..], true, false)
        );
        for p in ["", "  ", "abc", "\\a\\", "/abc", "{abc", "/a/e", "/(/"] {
            assert!(
                std::panic::catch_unwind(|| Preg::r#match(p, "a")).is_err(),
                "{p:?} must throw PregException"
            );
        }
        assert!(
            std::panic::catch_unwind(|| Preg::match_at("/a/", b"abc", 0, 5)).is_err(),
            "offset past the end throws"
        );
        // "Backtrack limit exhausted" in PHP
        assert!(
            std::panic::catch_unwind(|| Preg::r#match(
                "/(a*)*[bc]/",
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa!"
            ))
            .is_err()
        );
        let s = |v: Vec<Piece>| {
            v.into_iter()
                .map(|p| String::from_utf8(p.text).unwrap())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            s(Preg::split("//", "é日😀", -1, PREG_SPLIT_NO_EMPTY)),
            ["é", "日", "😀"]
        );
        assert_eq!(Preg::replace("/x*/", "-", b"\xff\xfe"), b"-\xff-\xfe-"); // invalid UTF-8: non-u, byte steps
        assert_eq!(s(Preg::split("/x*/", "abc", 2, 0)), ["", "abc"]);
        assert_eq!(s(Preg::split("/x*/", "abc", 1, 0)), ["abc"]);
        let pieces = Preg::split("/(,)/", "a,b", -1, PREG_SPLIT_DELIM_CAPTURE);
        assert_eq!(
            pieces
                .iter()
                .map(|p| (p.text.as_slice(), p.offset))
                .collect::<Vec<_>>(),
            [(&b"a"[..], 0), (b",", 1), (b"b", 2)]
        );
        assert_eq!(
            Preg::split_str("/,/", "a,b"),
            [b"a".to_vec(), b"b".to_vec()]
        );
        assert_eq!(
            Preg::replace(
                "/(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)/",
                r"[$11|\11|${11}|${1|$1x|\\|$|\",
                "abcdefghijk"
            ),
            br"[k|k|k|${1|ax|\|$|\"
        );
        assert_eq!(Preg::replace("/(a)/", r"${1}2${99}$99\99", "a"), b"a2");
        assert_eq!(
            Preg::replace("/(a)/", r"\\1|\$1|\\\1|\a|$$1", "a"),
            br"\1|$1|\a|\a|$a"
        ); // `\\` and `\$` escapes
    }

    // Expected values from plain preg_* on PHP 8.5 (no `u` added, errors are false/null).
    #[test]
    fn raw_preg_php_values() {
        let m = RawPreg::match_groups("/(a)(b)?/", b"xa").unwrap();
        assert_eq!((m.len(), m.str(1)), (2, &b"a"[..]));
        let all = RawPreg::match_all("/a(\\d)/", b"a1 a2");
        assert_eq!(
            (all.len(), all[1].str(1), all[1].offset(1)),
            (2, &b"2"[..], 4)
        );
        assert!(!RawPreg::r#match("/a/u", b"\xff"));
        assert!(RawPreg::r#match("/a/", "ba"));
        assert_eq!(RawPreg::replace("/(/", "", "a"), None);
        assert_eq!(
            RawPreg::replace("/(a)/", "[$1]", "aba"),
            Some(b"[a]b[a]".to_vec())
        );
        assert_eq!(
            RawPreg::replace_callback("/b/", |c| c.str(0).to_ascii_uppercase(), "abcb"),
            Some(b"aBcB".to_vec())
        );
    }
}
