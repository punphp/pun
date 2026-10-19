//! The names PHPStan prints for anonymous classes: BetterReflectionProvider::getAnonymousClassReflection()'s
//! `<parent>@anonymous/<relative path>:<line>`, with AnonymousClassVisitor's `:<index>` when several start on one
//! line.

use std::collections::HashMap;

use mago_codex::get_anonymous_class_name;
use mago_database::file::File;
use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::{AnonymousClass, Program};
use mago_syntax::walker::MutWalker;

use crate::relative_path::RelativePath;

struct Collect<'ast, 'arena> {
    classes: Vec<&'ast AnonymousClass<'arena>>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Collect<'ast, 'arena> {
    fn walk_in_anonymous_class(&mut self, a: &'ast AnonymousClass<'arena>, _: &mut ()) {
        self.classes.push(a);
    }
}

/// php-parser's start of the `Class_` node: its attributes, else its modifiers, else `class`.
fn start(a: &AnonymousClass) -> u32 {
    a.attribute_lists
        .first()
        .map(|l| l.span())
        .or_else(|| a.modifiers.first().map(|m| m.span()))
        .unwrap_or(a.class.span)
        .start
        .offset
}

/// The display name of each anonymous class of `file`, by Mago's codex name for it.
pub(crate) fn names(
    file: &File,
    program: &Program,
    names: &ResolvedNames,
    relative: &RelativePath,
) -> HashMap<Vec<u8>, String> {
    let mut collect = Collect {
        classes: Vec::new(),
    };
    collect.walk_program(program, &mut ());
    let path = relative.get(&String::from_utf8_lossy(&file.name));
    let lines: Vec<u32> = collect
        .classes
        .iter()
        .map(|a| file.line_number(start(a)) + 1)
        .collect();
    let mut out = HashMap::new();
    let mut seen: HashMap<u32, usize> = HashMap::new();
    for (a, &line) in collect.classes.iter().zip(&lines) {
        let parent = a
            .extends
            .as_ref()
            .and_then(|e| e.types.first())
            .or_else(|| a.implements.as_ref().and_then(|i| i.types.first()))
            .map_or_else(
                || "class".to_string(),
                |t| String::from_utf8_lossy(names.get(t)).into_owned(),
            );
        let mut display = format!("{parent}@anonymous/{path}:{line}");
        if lines.iter().filter(|&&l| l == line).count() > 1 {
            let index = seen.entry(line).or_default();
            *index += 1;
            display = format!("{display}:{index}");
        }
        out.insert(
            get_anonymous_class_name(file, a.span()).as_bytes().to_vec(),
            display,
        );
    }
    out
}

/// `ClassReflection::getName()` of an anonymous class declared in `file` (absolute), from its display name
/// `<parent>@anonymous/<path>:<line>[:<index>]`: AnonymousClassNameHelper's `AnonymousClass` followed by the md5 of
/// `<path>:<line>[:<index>]`, the path relative to the working directory (SimpleRelativePathHelper).
pub(crate) fn internal_name(display: &str, file: &str, relative: &RelativePath) -> String {
    let fuzzy = display
        .split_once("@anonymous/")
        .map_or(display, |(_, k)| k);
    // the line and index follow the last `:` pair of the fuzzy path
    let at = fuzzy
        .rsplit_once(':')
        .map(|(head, tail)| match head.rsplit_once(':') {
            Some((_, line)) if line.bytes().all(|b| b.is_ascii_digit()) && !line.is_empty() => {
                format!("{line}:{tail}")
            }
            _ => tail.to_string(),
        })
        .unwrap_or_default();
    let key = format!("{}:{at}", relative.simple(file));
    let digest = md5(key.as_bytes());
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    format!("AnonymousClass{hex}")
}

/// RFC 1321 MD5.
fn md5(input: &[u8]) -> [u8; 16] {
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    // K[i] = floor(abs(sin(i + 1)) * 2^32)
    let k: Vec<u32> = (0..64)
        .map(|i| (((i + 1) as f64).sin().abs() * 4_294_967_296.0) as u32)
        .collect();
    let mut message = input.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&((input.len() as u64).wrapping_mul(8)).to_le_bytes());
    let mut state: [u32; 4] = [0x6745_2301, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];
    for chunk in message.chunks(64) {
        let m: Vec<u32> = chunk
            .chunks(4)
            .map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
            .collect();
        let [mut a, mut b, mut c, mut d] = state;
        for i in 0..64 {
            let (f, g) = match i / 16 {
                0 => ((b & c) | (!b & d), i),
                1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                2 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let rotated = a
                .wrapping_add(f)
                .wrapping_add(k[i])
                .wrapping_add(m[g])
                .rotate_left(S[i]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(rotated);
        }
        state = [
            state[0].wrapping_add(a),
            state[1].wrapping_add(b),
            state[2].wrapping_add(c),
            state[3].wrapping_add(d),
        ];
    }
    let mut out = [0u8; 16];
    for (i, word) in state.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_le_bytes());
    }
    out
}
