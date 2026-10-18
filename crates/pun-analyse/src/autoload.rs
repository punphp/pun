//! Composer's psr-4 and psr-0 files, found on demand: a file is scanned once an analysed or scanned file names the
//! class its path holds, as PHPStan's PSR locators look a class up by name instead of reading every file.
//!
//! The names a file holds are over-approximated from its bytes, so that a file any rule or type could reach is
//! scanned: every name-like token in code, comments and strings (`\\` as in a string literal included), as written,
//! relative to each namespace the file declares, and through each import (`use`, `as`, group use, and any qualified
//! name's last segment). Case is ignored, as PHP's class lookup ignores it.

use std::collections::{HashMap, HashSet};

use foldhash::fast::FixedState;

use crate::project::Autoloaded;

type Map<K, V> = HashMap<K, V, FixedState>;

/// The autoloaded files by the class their path holds: index into the build's file list.
pub(crate) struct Index {
    psr4: Map<Box<[u8]>, Vec<usize>>,
    psr0: Map<Box<[u8]>, Vec<usize>>,
}

/// A token's bytes in the file: `\` or `\\` between segments, no leading `\`.
type Range = (usize, usize);

/// A name: a token, below a group use's prefix if it is in one.
type Full = (Option<Range>, Range);

impl Index {
    pub(crate) fn new<'a>(files: impl Iterator<Item = (usize, &'a Autoloaded)>) -> Index {
        let mut index = Index {
            psr4: Map::default(),
            psr0: Map::default(),
        };
        for (i, file) in files {
            let map = if file.psr0 {
                &mut index.psr0
            } else {
                &mut index.psr4
            };
            map.entry(file.class.clone().into_boxed_slice())
                .or_default()
                .push(i);
        }
        index
    }

    /// Adds to `out` the files that may hold a class `code` names.
    pub(crate) fn wanted(&self, code: &[u8], out: &mut Vec<usize>) {
        if self.psr4.is_empty() && self.psr0.is_empty() {
            return;
        }
        let names = Names::of(code);
        // by the alias lowercased, as PHP resolves an alias whatever its case
        let mut aliases: Map<Vec<u8>, Vec<Full>> = Map::default();
        for &(alias, full) in &names.aliases {
            aliases
                .entry(code[alias.0..alias.1].to_ascii_lowercase())
                .or_default()
                .push(full);
        }
        let mut first_segment = Vec::new();
        let mut seen: HashSet<(bool, &[u8]), FixedState> =
            HashSet::with_capacity_and_hasher(names.tokens.len(), FixedState::default());
        let mut name = Vec::new();
        for &(prefix, token) in &names.grouped {
            name.clear();
            push_full(&mut name, code, (Some(prefix), token));
            self.look(&mut name, out);
        }
        for &(range, fq) in &names.tokens {
            let token = &code[range.0..range.1];
            if !seen.insert((fq, token)) {
                continue;
            }
            name.clear();
            push_name(&mut name, token);
            self.look(&mut name, out);
            if fq {
                continue;
            }
            for &ns in &names.namespaces {
                name.clear();
                push_name(&mut name, &code[ns.0..ns.1]);
                name.push(b'\\');
                push_name(&mut name, token);
                self.look(&mut name, out);
            }
            let first = token
                .iter()
                .position(|&b| b == b'\\')
                .unwrap_or(token.len());
            first_segment.clear();
            first_segment.extend(token[..first].iter().map(u8::to_ascii_lowercase));
            if let Some(fulls) = aliases.get(&first_segment) {
                for &full in fulls {
                    name.clear();
                    push_full(&mut name, code, full);
                    push_name(&mut name, &token[first..]);
                    self.look(&mut name, out);
                }
            }
        }
    }

    /// The files that may hold class `name` (lowercased); `name` is used as scratch.
    fn look(&self, name: &mut [u8], out: &mut Vec<usize>) {
        if let Some(files) = self.psr4.get(&*name) {
            out.extend(files);
        }
        // psr-0: `_` in the class's own name is a directory separator
        let last = name.iter().rposition(|&b| b == b'\\').map_or(0, |i| i + 1);
        for b in &mut name[last..] {
            if *b == b'_' {
                *b = b'\\';
            }
        }
        if let Some(files) = self.psr0.get(&*name) {
            out.extend(files);
        }
    }
}

/// Appends `token` lowercased, with `\\` as `\`.
fn push_name(name: &mut Vec<u8>, token: &[u8]) {
    let mut previous = 0;
    for &b in token {
        if !(b == b'\\' && previous == b'\\') {
            name.push(b.to_ascii_lowercase());
        }
        previous = b;
    }
}

/// Appends `full`, lowercased.
fn push_full(name: &mut Vec<u8>, code: &[u8], (prefix, token): Full) {
    if let Some(p) = prefix {
        push_name(name, &code[p.0..p.1]);
        name.push(b'\\');
    }
    push_name(name, &code[token.0..token.1]);
}

/// A file's name-like tokens.
struct Names {
    /// Each token, and whether it is fully qualified.
    tokens: Vec<(Range, bool)>,
    namespaces: Vec<Range>,
    /// An alias (a qualified name's last segment, or the name after `as`) and the name it may stand for.
    aliases: Vec<(Range, Full)>,
    /// The names in a group use, below its prefix.
    grouped: Vec<(Range, Range)>,
}

/// Bytes that start a name: letters, `_` and non-ASCII.
const START: [bool; 256] = {
    let mut t = [false; 256];
    let mut b = 0;
    while b < 256 {
        t[b] = (b as u8).is_ascii_alphabetic() || b == b'_' as usize || b >= 0x80;
        b += 1;
    }
    t
};

fn is_start(b: u8) -> bool {
    START[b as usize]
}

fn is_part(b: u8) -> bool {
    START[b as usize] || b.is_ascii_digit()
}

impl Names {
    fn of(code: &[u8]) -> Names {
        let mut names = Names {
            tokens: Vec::new(),
            namespaces: Vec::new(),
            aliases: Vec::new(),
            grouped: Vec::new(),
        };
        // the group use the scan is in: its prefix and where it closes
        let mut group: Option<(Range, usize)> = None;
        // the two names before this one
        let mut previous: [Option<Full>; 2] = [None, None];
        let mut i = 0;
        while i < code.len() {
            match code[i..].iter().position(|&b| is_start(b)) {
                Some(k) => i += k,
                None => break,
            }
            let at = |k: usize| i.checked_sub(k).map(|j| code[j]);
            let member = at(1) == Some(b'$') || (at(1) == Some(b'>') && at(2) == Some(b'-'));
            let fq = at(1) == Some(b'\\');
            let start = i;
            let mut end;
            let mut trailing = false;
            loop {
                i += code[i..]
                    .iter()
                    .position(|&b| !is_part(b))
                    .unwrap_or(code.len() - i);
                end = i;
                let sep = code[i..]
                    .iter()
                    .take(2)
                    .take_while(|&&b| b == b'\\')
                    .count();
                if sep == 0 {
                    break;
                }
                i += sep;
                if !code.get(i).is_some_and(|&b| is_start(b)) {
                    trailing = true;
                    break;
                }
            }
            if member {
                continue;
            }
            let range = (start, end);
            if group.is_some_and(|(_, close)| i > close) {
                group = None;
            }
            if trailing {
                // `use A\{B, C as D}`: the names up to `}` are below `A\`
                let open = code[i..]
                    .iter()
                    .position(|b| !b.is_ascii_whitespace())
                    .map(|k| i + k)
                    .filter(|&k| code[k] == b'{');
                if let Some(open) = open
                    && let Some(close) = code[open..].iter().position(|&b| b == b'}')
                {
                    group = Some((range, open + close));
                }
                continue;
            }
            let text = |r: Option<Full>| r.map(|(_, r)| &code[r.0..r.1]);
            let prefix = group.map(|(prefix, _)| prefix);
            let full = (prefix, range);
            if text(previous[1]).is_some_and(|t| t.eq_ignore_ascii_case(b"namespace")) {
                names.namespaces.push(range);
            }
            if text(previous[1]).is_some_and(|t| t.eq_ignore_ascii_case(b"as"))
                && let Some(aliased) = previous[0]
            {
                names.aliases.push((range, aliased));
            }
            let last = code[start..end]
                .iter()
                .rposition(|&b| b == b'\\')
                .map_or(start, |k| start + k + 1);
            if last > start || prefix.is_some() {
                names.aliases.push(((last, end), full));
            }
            if let Some(prefix) = prefix {
                names.grouped.push((prefix, range));
            }
            names.tokens.push((range, fq));
            previous = [previous[1], Some(full)];
        }
        names
    }
}
