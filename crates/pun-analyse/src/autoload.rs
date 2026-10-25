//! Composer's psr-4 and psr-0 files, found on demand: a file is scanned once an analysed or scanned file names the
//! class its path holds, as PHPStan's PSR locators look a class up by name instead of reading every file.
//!
//! The names a file holds are over-approximated from its bytes, so that a file any rule or type could reach is
//! scanned: every name-like token in code, comments and strings (`\\` as in a string literal included), as written,
//! relative to each namespace the file declares, and through each import (`use`, `as`, group use, and any qualified
//! name's last segment). Case is ignored for class files. Laravel's cold AliasLoader lookup is exact-case; a
//! reachable exact alias activates its codex declaration, after which loaded alias identity is case-insensitive.

use std::collections::{HashMap, HashSet};

use foldhash::fast::FixedState;
use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::{Program, Statement};

use crate::project::Autoloaded;

type Map<K, V> = HashMap<K, V, FixedState>;

/// The autoloaded files by the class their path holds: index into the build's file list.
pub(crate) struct Index {
    psr4: Map<Box<[u8]>, Vec<usize>>,
    psr0: Map<Box<[u8]>, Vec<usize>>,
    aliases: Map<Box<[u8]>, Alias>,
}

struct Alias {
    declarations: Vec<usize>,
    files: Vec<usize>,
}

/// A token's bytes in the file: `\` or `\\` between segments, no leading `\`.
type Range = (usize, usize);

/// A name: a token, below a group use's prefix if it is in one.
type Full = (Option<Range>, Range);

impl Index {
    pub(crate) fn new<'a>(
        files: impl Iterator<Item = (usize, &'a Autoloaded)>,
        aliases: &[(String, String)],
    ) -> Index {
        let mut index = Index {
            psr4: Map::default(),
            psr0: Map::default(),
            aliases: Map::default(),
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
        let by_name: Map<&str, usize> = aliases
            .iter()
            .enumerate()
            .map(|(i, (a, _))| (a.as_str(), i))
            .collect();
        for (i, (name, target)) in aliases.iter().enumerate() {
            let mut declarations = vec![i];
            let mut files = Vec::new();
            let mut target = target.as_str();
            loop {
                index.look_files(
                    &mut target
                        .trim_start_matches('\\')
                        .as_bytes()
                        .to_ascii_lowercase(),
                    &mut files,
                );
                if !files.is_empty() {
                    break;
                }
                let Some(&next) = by_name.get(target.trim_start_matches('\\')) else {
                    break;
                };
                if declarations.contains(&next) {
                    break;
                }
                declarations.push(next);
                target = &aliases[next].1;
            }
            index.aliases.insert(
                name.as_bytes().into(),
                Alias {
                    declarations,
                    files,
                },
            );
        }
        index
    }

    /// Adds the files and alias declarations that a class `code` names can reach.
    pub(crate) fn wanted(
        &self,
        code: &[u8],
        program: &Program,
        resolved: &ResolvedNames,
        out: &mut Vec<usize>,
        declarations: &mut Vec<usize>,
    ) {
        if self.psr4.is_empty() && self.psr0.is_empty() && self.aliases.is_empty() {
            return;
        }
        // Resolved spellings, not arbitrary strings/comments or an unqualified name in another namespace.
        // In particular, scanning Facade::defaultAliases itself must not activate every alias in its array.
        let namespaces: Vec<_> = if self.aliases.is_empty() {
            Vec::new()
        } else {
            program
                .statements
                .iter()
                .filter_map(|s| match s {
                    Statement::Namespace(ns) => Some(ns),
                    _ => None,
                })
                .collect()
        };
        if !self.aliases.is_empty() {
            for (start, _, name, _) in resolved.iter() {
                if !namespaces.iter().any(|ns| {
                    ns.name
                        .as_ref()
                        .is_some_and(|n| n.span().start.offset == start)
                }) {
                    self.look_alias(name, out, declarations);
                }
            }
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
        // PHPDoc has no ResolvedNames entries. Use its actual namespace and the file's import expansions, as
        // the byte index does for target files; an unqualified local type must not become a global alias.
        for doc in program
            .trivia
            .iter()
            .filter(|t| !self.aliases.is_empty() && t.kind.is_docblock())
        {
            let namespace = namespaces
                .iter()
                .find(|ns| {
                    ns.span().start.offset <= doc.span.start.offset
                        && doc.span.end.offset <= ns.span().end.offset
                })
                .and_then(|ns| ns.name.as_ref());
            for (range, fq) in Names::of(doc.value).tokens {
                let token = &doc.value[range.0..range.1];
                let mut name = Vec::new();
                if !fq && let Some(ns) = namespace {
                    push_name(&mut name, ns.value());
                    name.push(b'\\');
                }
                push_name(&mut name, token);
                self.look_alias(&name, out, declarations);
                let first = token
                    .iter()
                    .position(|&b| b == b'\\')
                    .unwrap_or(token.len());
                if !fq && let Some(fulls) = aliases.get(&token[..first].to_ascii_lowercase()) {
                    for &full in fulls {
                        name.clear();
                        push_full(&mut name, code, full);
                        push_name(&mut name, &token[first..]);
                        self.look_alias(&name, out, declarations);
                    }
                }
            }
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

    fn look_alias(&self, name: &[u8], out: &mut Vec<usize>, declarations: &mut Vec<usize>) {
        if let Some(alias) = self.aliases.get(name) {
            out.extend(&alias.files);
            declarations.extend(&alias.declarations);
        }
    }

    fn look(&self, name: &mut [u8], out: &mut Vec<usize>) {
        name.make_ascii_lowercase();
        self.look_files(name, out);
    }

    /// The files that may hold class `name` (lowercased); `name` is used as scratch.
    fn look_files(&self, name: &mut [u8], out: &mut Vec<usize>) {
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

/// Appends `token`, collapsing doubled namespace separators in string literals.
fn push_name(name: &mut Vec<u8>, token: &[u8]) {
    let mut previous = 0;
    for &b in token {
        if !(b == b'\\' && previous == b'\\') {
            name.push(b);
        }
        previous = b;
    }
}

/// Appends `full`.
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
