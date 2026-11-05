//! What the rules read of a file's syntax beyond Mago's codex, taken once from the scan's parse: the namespaces and
//! imports, the comments and parsed doc comments, and the declaration offsets and written parent lists.

use std::collections::{HashMap, HashSet};

use mago_names::kind::NameKind;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::phpdoc;

use super::doc::{Doc, FileScopes};

/// One file's facts.
pub(crate) struct Facts {
    pub scopes: FileScopes,
    pub comments: Comments,
    pub declarations: Declarations,
    /// `declare(strict_types=1)` anywhere in the text, ignoring whitespace and case.
    pub strict_types: bool,
}

impl Facts {
    pub fn of(contents: &[u8], program: &Program) -> Facts {
        let scopes = FileScopes::of(program);
        let mut declarations = Declarations::default();
        DeclarationWalker { out: &mut declarations, scopes: &scopes }.walk_program(program, &mut ());
        let text: Vec<u8> = contents.iter().filter(|b| !b.is_ascii_whitespace()).map(u8::to_ascii_lowercase).collect();
        let strict_types = text.windows(23).any(|w| w == b"declare(strict_types=1)");
        Facts { comments: Comments::of(contents), declarations, scopes, strict_types }
    }
}

/// What the rules read of one doc comment.
#[derive(Clone)]
pub struct DocEntry {
    pub doc: Doc,
    /// [`super::doc::tags()`]
    pub tags: Vec<(String, Vec<u8>)>,
    /// [`super::declarations::doc_tags()`]
    pub names: Vec<String>,
    /// PhpDocParser's tags, none where it throws.
    pub phpdoc: Vec<phpdoc::Tag>,
}

impl DocEntry {
    pub fn parse(raw: &[u8]) -> DocEntry {
        DocEntry {
            doc: Doc::parse(raw),
            tags: super::doc::tags(raw),
            names: super::declarations::doc_tags(raw),
            phpdoc: phpdoc::tags(raw).unwrap_or_default(),
        }
    }
}

const NONE: u32 = u32::MAX;

/// Each `*/` of a file with the comment starts the backward searches of [`Comments::doc_before()`] find before it,
/// and the doc comments those close, parsed.
#[derive(Default)]
pub struct Comments {
    /// The offset after each `*/`, ascending.
    ends: Vec<u32>,
    /// The doc comment each `*/` closes (an index into `docs`), else NONE.
    doc: Vec<u32>,
    /// The start of the last `/*` before each `*/`, else NONE.
    plain: Vec<u32>,
    /// (start, end, entry)
    docs: Vec<(u32, u32, DocEntry)>,
}

impl Comments {
    pub fn of(contents: &[u8]) -> Comments {
        let mut out = Comments::default();
        // the last two `/**` and `/*` seen: the last one can overlap the `*/` (`/**/`, `/*/`), the one before cannot
        let (mut doc_opens, mut opens, mut last_close) = ([None; 2], [None; 2], None);
        for p in 0..contents.len().saturating_sub(1) {
            match &contents[p..p + 2] {
                b"/*" => {
                    if contents.get(p + 2) == Some(&b'*') {
                        doc_opens = [doc_opens[1], Some(p)];
                    }
                    opens = [opens[1], Some(p)];
                }
                b"*/" => {
                    let end = p + 2;
                    // the searches of the text before the `*/`: a `/**` or `/*` entirely before it
                    let doc_start = doc_opens[1].filter(|&s| s + 3 <= p).or(doc_opens[0]);
                    let plain_start = opens[1].filter(|&s| s + 2 <= p).or(opens[0]);
                    let doc = doc_start.filter(|&s| {
                        contents.get(s + 3).is_some_and(u8::is_ascii_whitespace)
                            && last_close.is_none_or(|q| q < s + 3)
                    });
                    out.ends.push(end as u32);
                    out.plain.push(plain_start.map_or(NONE, |s| s as u32));
                    out.doc.push(match doc {
                        Some(start) => {
                            let raw = &contents[start..end];
                            out.docs.push((start as u32, end as u32, DocEntry::parse(raw)));
                            (out.docs.len() - 1) as u32
                        }
                        None => NONE,
                    });
                    last_close = Some(p);
                }
                _ => {}
            }
        }
        out
    }

    /// The index of the `*/` that ends at `end`.
    fn closing(&self, end: usize) -> Option<usize> {
        self.ends.binary_search(&(end as u32)).ok()
    }

    /// The doc comment (`/**` ... `*/`) that ends right before `offset`, skipping other comments in between: the one
    /// PHP's tokenizer reads (the start is the last `/**` before its `*/`, so `/*` inside the text is no start).
    pub fn doc_before<'c>(&'c self, contents: &'c [u8], offset: usize) -> Option<(&'c [u8], &'c DocEntry)> {
        let mut end = offset.min(contents.len());
        loop {
            end -= contents[..end].iter().rev().take_while(|b| b.is_ascii_whitespace()).count();
            if !contents[..end].ends_with(b"*/") || end < 4 {
                return None;
            }
            let i = self.closing(end)?;
            match self.doc[i] {
                NONE => match self.plain[i] {
                    NONE => return None,
                    start => end = start as usize,
                },
                d => {
                    let (start, end, entry) = &self.docs[d as usize];
                    return Some((&contents[*start as usize..*end as usize], entry));
                }
            }
        }
    }

    /// The parsed doc comment at `start..end`, when it is one [`Comments::doc_before()`] finds.
    pub fn entry(&self, start: u32, end: u32) -> Option<&DocEntry> {
        let i = self.docs.binary_search_by_key(&end, |(_, end, _)| *end).ok()?;
        let (found, _, entry) = &self.docs[i];
        (*found == start).then_some(entry)
    }

    /// php-parser's `getDocComment()`: the last `/** */` comment before `offset` with only whitespace and other block
    /// comments between.
    pub fn php_doc_comment<'c>(&self, contents: &'c [u8], offset: usize) -> Option<&'c [u8]> {
        let mut end = offset.min(contents.len());
        loop {
            end -= contents[..end].iter().rev().take_while(|b| b.is_ascii_whitespace()).count();
            if !contents[..end].ends_with(b"*/") {
                return None;
            }
            let start = match self.plain[self.closing(end)?] {
                NONE => return None,
                start => start as usize,
            };
            let comment = &contents[start..end];
            // PHP's T_DOC_COMMENT: `/**` and whitespace
            if comment.starts_with(b"/**") && comment.get(3).is_some_and(u8::is_ascii_whitespace) {
                return Some(comment);
            }
            end = start;
        }
    }
}

/// Declaration offsets the codex does not keep.
#[derive(Default)]
pub struct Declarations {
    /// Where each property or constant declaration starts, by the offset of each item's name.
    pub starts: HashMap<u32, u32>,
    /// The offsets of native `readonly` modifiers' properties, promoted parameters and classes.
    pub readonly: HashSet<u32>,
    /// By class-like start: whether it is an interface, and its written `implements` (an interface's `extends`)
    /// names, resolved, in order.
    pub parents: HashMap<u32, (bool, Vec<Vec<u8>>)>,
}

struct DeclarationWalker<'d> {
    out: &'d mut Declarations,
    scopes: &'d FileScopes,
}

impl DeclarationWalker<'_> {
    fn parents<'n>(&mut self, at: Span, interface: bool, names: impl Iterator<Item = &'n Identifier<'n>>) {
        let scope = self.scopes.at(at.start.offset);
        let resolved = names
            .map(|id| scope.resolve(NameKind::Default, id.value()).0.to_vec())
            .collect();
        self.out.parents.insert(at.start.offset, (interface, resolved));
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for DeclarationWalker<'_> {
    fn walk_in_property(&mut self, p: &'ast Property<'arena>, _: &mut ()) {
        for v in p.variables() {
            self.out.starts.insert(v.span.start.offset, p.span().start.offset);
            if p.modifiers().iter().any(Modifier::is_readonly) {
                self.out.readonly.insert(v.span.start.offset);
            }
        }
    }
    fn walk_in_function_like_parameter(&mut self, p: &'ast FunctionLikeParameter<'arena>, _: &mut ()) {
        if p.modifiers.iter().any(Modifier::is_readonly) {
            self.out.readonly.insert(p.variable.span.start.offset);
        }
    }
    fn walk_in_class(&mut self, c: &'ast Class<'arena>, _: &mut ()) {
        if c.modifiers.iter().any(Modifier::is_readonly) {
            self.out.readonly.insert(c.span().start.offset);
        }
        self.parents(c.span(), false, c.implements.iter().flat_map(|i| i.types.iter()));
    }
    fn walk_in_anonymous_class(&mut self, c: &'ast AnonymousClass<'arena>, _: &mut ()) {
        if c.modifiers.iter().any(Modifier::is_readonly) {
            self.out.readonly.insert(c.span().start.offset);
        }
        self.parents(c.span(), false, c.implements.iter().flat_map(|i| i.types.iter()));
    }
    fn walk_in_enum(&mut self, e: &'ast Enum<'arena>, _: &mut ()) {
        self.parents(e.span(), false, e.implements.iter().flat_map(|i| i.types.iter()));
    }
    fn walk_in_interface(&mut self, i: &'ast Interface<'arena>, _: &mut ()) {
        self.parents(i.span(), true, i.extends.iter().flat_map(|e| e.types.iter()));
    }
    fn walk_in_class_like_constant(&mut self, k: &'ast ClassLikeConstant<'arena>, _: &mut ()) {
        for item in k.items.iter() {
            self.out.starts.insert(item.span().start.offset, k.span().start.offset);
        }
    }
    fn walk_in_constant(&mut self, k: &'ast Constant<'arena>, _: &mut ()) {
        for item in k.items.iter() {
            self.out.starts.insert(item.span().start.offset, k.span().start.offset);
        }
    }
}
