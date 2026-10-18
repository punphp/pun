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
