//! Rector's post rectors over one file (`PostFileProcessor`), after a rule changed names or docs in it:
//! `ClassRenamingPostRector`, `NameImportingPostRector` (`NameImporter`, `ClassNameImportSkipper` and its voters),
//! `DocblockNameImportingPostRector` (`NameImportingPhpDocNodeVisitor`), `UseAddingPostRector` (`FileNode::addImports()`)
//! and `UnusedImportRemovingPostRector`, then the file printed as php-parser's format-preserving printer prints it.
//!
//! A rule makes a [`Session`] of the file, changes names ([`Session::rename`]) and docs ([`Session::doc_mut`]) as its
//! Rector rule changes the nodes, and [`Session::finish`] gives the file as Rector writes it.

use mago_syntax::cst::Program;
use pun_problem::Span;

use super::differ::{self, Comments, NewItem, OldItem, Piece};
use super::doc::Doc;
use super::resolve::{self, DocType, Scope, has_class};
use super::view::{FileView, Kind, StmtKind, UseItemView, UseView};
use crate::Context;
use crate::rules::phpdoc_writer::DocTarget;
use crate::rules::rector_conditions::indentation;

/// The import options of Rector's configuration (`RectorConfig::importNames()`, `importShortClasses()`,
/// `removeUnusedImports()`).
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Options {
    pub import_names: bool,
    pub import_short_classes: bool,
    pub remove_unused_imports: bool,
}

/// A `Name` node as the rules and post rectors left it.
#[derive(Clone, Debug)]
struct NameState {
    /// A `FullyQualified` node.
    fq: bool,
    /// What it stands for: a fully qualified node's name, else the name as printed.
    name: String,
    /// `ORIGINAL_NAME`, `None` for a node a rule made.
    original: Option<String>,
    /// The text printed, when it changed.
    text: Option<String>,
}

/// A doc comment of the file.
struct DocState {
    span: Span,
    /// Its index among the view's docs, `None` for a doc no node takes (a `Nop`'s).
    view: Option<usize>,
    namespace: Option<String>,
    class: Option<String>,
    doc: Option<Doc>,
    /// Whether `createFromNode()` of some node reads it (it is the node's last doc comment).
    last: bool,
}

/// An item of a statement list as the post rectors leave it.
#[derive(Clone, Debug)]
enum Item {
    /// The old statement at this index.
    Old(usize),
    /// A `use` statement `FileNode::addImports()` made; `comments` the old statement whose comments moved to it.
    New {
        kind: Kind,
        name: String,
        comments: Option<usize>,
    },
    /// A `Nop`, with the comments of the old statement at this index.
    Nop(Option<usize>),
}

/// A statement list's state.
#[derive(Clone)]
struct ListState {
    items: Vec<Item>,
    /// Per `use` of the root, per item: whether it is kept.
    kept: Vec<Vec<bool>>,
    /// Old statements printed anew (`ORIGINAL_NODE` set to null).
    reprinted: Vec<bool>,
    /// Old statements whose comments moved to a new `use`.
    cleared: Vec<bool>,
}

/// An object type of the import handling: `FullyQualifiedObjectType` or `AliasedObjectType`.
#[derive(Clone, Debug)]
enum ObjType {
    Fq(String),
    Aliased(String, String),
}

impl ObjType {
    /// `equals()` with a `FullyQualifiedObjectType` of `class`.
    fn equals(&self, class: &str) -> bool {
        match self {
            ObjType::Fq(name) | ObjType::Aliased(_, name) => name == class,
        }
    }

    /// `areShortNamesEqual()` with a `FullyQualifiedObjectType` of `class`.
    fn short_names_equal(&self, class: &str) -> bool {
        match self {
            ObjType::Fq(name) => last(name) == last(class),
            ObjType::Aliased(alias, _) => alias == last(class),
        }
    }
}

/// What [`Session::carry`] hands to the next round of a file.
pub(crate) struct Carry {
    pending: Pending,
    used: (Vec<ObjType>, Vec<String>, Vec<String>),
    short_names: Option<Vec<(String, String)>>,
    /// The docs as the round left them: the next round runs on the same nodes and their `PhpDocInfo`, never on the
    /// printed text parsed again.
    docs: Vec<Option<Doc>>,
}

/// `PendingImports`.
#[derive(Clone, Default)]
struct Pending {
    uses: Vec<String>,
    functions: Vec<String>,
    constants: Vec<String>,
}

impl Pending {
    fn is_short_imported(&self, class: &str) -> bool {
        let short = last(class);
        self.constants.iter().any(|c| last(c) == short)
            || self
                .uses
                .iter()
                .chain(&self.functions)
                .any(|c| last(c).eq_ignore_ascii_case(short))
    }

    fn is_import_shortable(&self, class: &str) -> bool {
        self.uses
            .iter()
            .chain(&self.constants)
            .chain(&self.functions)
            .any(|c| c == class)
    }

    fn is_empty(&self) -> bool {
        self.uses.is_empty() && self.functions.is_empty() && self.constants.is_empty()
    }
}

/// The last part of a name (`FullyQualifiedObjectType::getShortName()`).
fn last(name: &str) -> &str {
    name.rsplit('\\').next().unwrap_or_default()
}

/// What a voter sees of the node a name is imported for.
struct At<'a> {
    namespace: Option<&'a str>,
    /// For a `FullyQualified` name: whether its original name is one part (`OriginalNameImportSkipVoter`).
    original_unqualified: Option<bool>,
}

/// The file being changed.
pub(crate) struct Session<'s> {
    ctx: &'s Context<'s>,
    source: &'s str,
    pub view: FileView,
    names: Vec<NameState>,
    docs: Vec<DocState>,
    lists: Vec<ListState>,
    /// `RenamedNameCollector`.
    renamed: Vec<String>,
    /// The `implements` names a class drops (`array_unique()`).
    implements_removed: Vec<Span>,
    pending: Pending,
    /// `FileNode`'s `UsedImports`, read when the file was parsed.
    used: (Vec<ObjType>, Vec<String>, Vec<String>),
    short_names: Option<Vec<(String, String)>>,
    /// Pun does not follow Rector here: the file is left alone.
    unsupported: bool,
}

impl<'s> Session<'s> {
    pub fn new(ctx: &'s Context<'s>, program: &Program<'_>) -> Session<'s> {
        let source = ctx.source;
        let view = FileView::of(source, program);
        let names = view
            .names
            .iter()
            .map(|n| NameState {
                fq: n.resolved.is_some(),
                name: n
                    .resolved
                    .clone()
                    .unwrap_or_else(|| n.original().to_string()),
                original: n.resolved.as_ref().map(|_| n.original().to_string()),
                text: None,
            })
            .collect();
        let lists = view
            .roots
            .iter()
            .map(|r| ListState {
                items: (0..r.stmts.len())
                    .map(|i| match r.stmts[i].kind {
                        StmtKind::Nop => Item::Nop(Some(i)),
                        _ => Item::Old(i),
                    })
                    .collect(),
                kept: r.uses.iter().map(|u| vec![true; u.items.len()]).collect(),
                reprinted: vec![false; r.stmts.len()],
                cleared: vec![false; r.stmts.len()],
            })
            .collect();
        let mut session = Session {
            ctx,
            source,
            names,
            docs: Vec::new(),
            lists,
            renamed: Vec::new(),
            implements_removed: Vec::new(),
            pending: Pending::default(),
            used: (Vec::new(), Vec::new(), Vec::new()),
            short_names: None,
            unsupported: false,
            view,
        };
        session.used = session.used_imports();
        session.read_docs();
        session
    }

    /// Every doc comment of the file, read with the imports it was parsed with.
    fn read_docs(&mut self) {
        let uses = self.view.uses_root().uses.clone();
        let tokens = crate::rules::php_parser::tokens(self.source);
        for token in tokens.iter().filter(|t| t.kind.is_comment()) {
            let text = &self.source[token.start..token.end];
            if !(text.starts_with("/**")
                && text
                    .as_bytes()
                    .get(3)
                    .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n')))
            {
                continue;
            }
            let span = Span::new(token.start, token.end);
            let view = self.view.docs.iter().position(|d| d.span == span);
            let (namespace, class, last) = match view {
                Some(i) => {
                    let d = &self.view.docs[i];
                    (d.namespace.clone(), d.class.clone(), d.is_last)
                }
                // a `Nop`'s: `getDocComment()` reads its last doc
                None => (
                    self.namespace_at(span.start),
                    None,
                    nop_run(&tokens, span.end).0,
                ),
            };
            let scope = Scope {
                namespace: namespace.as_deref(),
                class: class.as_deref(),
                uses: &uses,
            };
            let ctx = self.ctx;
            let doc = Doc::parse(
                text,
                &|tag| resolve::resolve_tag(ctx, scope, tag),
                &|name| {
                    resolve::narrow(ctx, scope, name)
                        .class_name()
                        .map(str::to_string)
                },
            );
            self.docs.push(DocState {
                span,
                view,
                namespace,
                class,
                doc,
                last,
            });
        }
    }

    /// The namespace the offset `at` is in.
    fn namespace_at(&self, at: usize) -> Option<String> {
        self.view
            .roots
            .iter()
            .filter(|r| r.is_namespace)
            .find(|r| {
                r.header
                    .as_ref()
                    .is_some_and(|h| h.span.start <= at && at <= h.span.end.max(r.inner.end))
            })
            .and_then(|r| r.name.clone())
    }

    /// The number of `Name` nodes ([`FileView::names`]).
    pub fn name_count(&self) -> usize {
        self.names.len()
    }

    /// What the name `i` stands for now (`FullyQualified::toString()` of a fully qualified one), `None` for a name
    /// that is not fully qualified.
    pub fn fully_qualified(&self, i: usize) -> Option<&str> {
        self.names[i].fq.then_some(self.names[i].name.as_str())
    }

    /// `Name::toString()` of the name `i` now.
    pub fn name_string(&self, i: usize) -> String {
        self.names[i].name.clone()
    }

    /// `new FullyQualified($new)` in place of the name `i`.
    pub fn rename(&mut self, i: usize, new: &str) {
        let state = &mut self.names[i];
        state.fq = true;
        state.name = new.to_string();
        state.original = None;
        state.text = Some(format!("\\{new}"));
    }

    /// What a round of the file leaves for the next (`FileProcessor::processFile()` runs the post rectors again on
    /// a changed file): the `FileNode`'s `PendingImports` and `UsedImports`, `ShortNameResolver`'s cache by path, and
    /// the docs.
    pub fn carry(&self) -> Carry {
        Carry {
            pending: self.pending.clone(),
            used: self.used.clone(),
            short_names: self.short_names.clone(),
            docs: self.docs.iter().map(|d| d.doc.clone()).collect(),
        }
    }

    /// Start as the next round after the one that left `carry`.
    pub fn resume(&mut self, carry: Carry) {
        self.pending = carry.pending;
        self.used = carry.used;
        self.short_names = carry.short_names;
        for (state, doc) in self.docs.iter_mut().zip(carry.docs) {
            state.doc = doc;
        }
    }

    /// `RenamedNameCollector::add()`.
    pub fn collect_renamed(&mut self, old: &str) {
        if !self.renamed.iter().any(|r| r == old) {
            self.renamed.push(old.to_string());
        }
    }

    /// Drop an `implements` name of a class (`array_unique()` on `$class->implements`).
    pub fn remove_implements(&mut self, span: Span) {
        self.implements_removed.push(span);
    }

    /// The doc comments the rules read: their index, the view's doc, and the doc.
    pub fn doc_indexes(&self) -> Vec<usize> {
        (0..self.docs.len()).collect()
    }

    /// The view's doc of the doc `i` (`None` for one no node takes).
    pub fn doc_view(&self, i: usize) -> Option<&super::view::DocView> {
        self.docs[i].view.map(|v| &self.view.docs[v])
    }

    /// Whether `createFromNode()` reads the doc `i`.
    pub fn doc_is_last(&self, i: usize) -> bool {
        self.docs[i].last
    }

    /// The doc `i`, `None` where Pun does not read it as Rector does.
    pub fn doc(&self, i: usize) -> Option<&Doc> {
        self.docs[i].doc.as_ref()
    }

    /// The doc `i` to change.
    pub fn doc_mut(&mut self, i: usize) -> Option<&mut Doc> {
        self.docs[i].doc.as_mut()
    }

    /// The scope a doc was read in: its namespace, class, and the file's imports then.
    pub fn doc_scope(&self, i: usize) -> Scope<'_> {
        Scope {
            namespace: self.docs[i].namespace.as_deref(),
            class: self.docs[i].class.as_deref(),
            uses: &self.view.uses_root().uses,
        }
    }

    /// Give up on the file: Pun does not follow Rector here.
    pub fn unsupported(&mut self) {
        self.unsupported = true;
    }

    /// The root `FileNode::getUsesAndGroupUses()` reads.
    fn uses_root(&self) -> usize {
        match self.view.namespaces {
            1 => self.view.roots.iter().position(|r| r.is_namespace).unwrap(),
            _ => 0,
        }
    }

    /// The `use` statements of the root `r` as they stand: old ones with their kept items, then new ones in place.
    fn current_uses(&self, r: usize) -> Vec<UseView> {
        let root = &self.view.roots[r];
        let list = &self.lists[r];
        let mut uses = Vec::new();
        for item in &list.items {
            match item {
                Item::Old(i) => {
                    if let StmtKind::Use(k) = root.stmts[*i].kind {
                        let mut u = root.uses[k].clone();
                        u.items = u
                            .items
                            .into_iter()
                            .zip(&list.kept[k])
                            .filter(|(_, k)| **k)
                            .map(|(i, _)| i)
                            .collect();
                        uses.push(u);
                    }
                }
                // the names are read before `addImports()` makes any new one
                Item::New { .. } | Item::Nop(_) => {}
            }
        }
        uses
    }

    /// `UsedImportsResolver::resolveForStmts()` of the file as parsed: its first class, then every import.
    fn used_imports(&self) -> (Vec<ObjType>, Vec<String>, Vec<String>) {
        let mut classes = Vec::new();
        if let Some(class) = self.view.class_likes.iter().find(|c| c.is_class) {
            classes.push(ObjType::Fq(class.name.clone()));
        }
        let (mut functions, mut constants) = (Vec::new(), Vec::new());
        for root in &self.view.roots {
            for u in &root.uses {
                // a group with a type of its own is skipped (`processGroupUse()`)
                if u.prefix.is_some() && u.kind.is_some() {
                    continue;
                }
                for (i, item) in u.items.iter().enumerate() {
                    let name = match &u.prefix {
                        Some(p) => format!("{p}\\{}", item.name),
                        None => u.full_name(i),
                    };
                    match item.kind {
                        Kind::Class => classes.push(match &item.alias {
                            Some(alias) => ObjType::Aliased(alias.clone(), name),
                            None => ObjType::Fq(name),
                        }),
                        Kind::Function => functions.push(name),
                        Kind::Constant => constants.push(name),
                    }
                }
            }
        }
        (classes, functions, constants)
    }

    /// `FileNode::resolveUsedImportTypes()`: the pending imports, then those of the imports root.
    fn used_import_types(&self) -> Vec<ObjType> {
        let mut types: Vec<ObjType> = self.pending.uses.iter().cloned().map(ObjType::Fq).collect();
        for u in self.current_uses(self.uses_root()) {
            for (i, item) in u.items.iter().enumerate() {
                let name = u.full_name(i);
                types.push(match &item.alias {
                    Some(alias) => ObjType::Aliased(alias.clone(), name),
                    None => ObjType::Fq(name),
                });
            }
        }
        types
    }

    /// `FileNode::hasImport()`.
    fn has_import(&self, class: &str) -> bool {
        self.used_import_types().iter().any(|t| t.equals(class))
    }

    /// Run the post rectors and print the file: the new source (the old one where nothing changed), `None` where Pun
    /// does not follow Rector. `has_renames` is whether `RenamedClassesDataCollector` holds renames.
    pub fn finish(&mut self, options: Options, has_renames: bool) -> Option<String> {
        if self.unsupported {
            return None;
        }
        // `AddUseStatementGuard`
        let guard = self.view.namespaces < 2 && !self.view.inline_html;
        if options.import_names && has_renames && guard {
            self.class_renaming();
        }
        if options.import_names && guard {
            self.import_names(options);
            self.import_doc_names(options);
        }
        self.add_uses();
        if options.remove_unused_imports {
            self.remove_unused_imports();
        }
        if self.unsupported {
            return None;
        }
        self.print()
    }

    /// `ClassRenamingPostRector`: `FileNode::removeImports()` of the renamed classes.
    fn class_renaming(&mut self) {
        let r = self.view.placement_root();
        let root = &self.view.roots[r];
        let list = &mut self.lists[r];
        let mut removed_stmts = Vec::new();
        for item in &list.items {
            let Item::Old(i) = item else {
                continue;
            };
            let StmtKind::Use(k) = root.stmts[*i].kind else {
                continue;
            };
            if root.uses[k].prefix.is_some() {
                continue;
            }
            for (j, use_item) in root.uses[k].items.iter().enumerate() {
                if list.kept[k][j] && self.renamed.contains(&use_item.name) {
                    list.kept[k][j] = false;
                }
            }
            if list.kept[k].iter().all(|k| !k) {
                removed_stmts.push(*i);
            }
        }
        list.items
            .retain(|item| !matches!(item, Item::Old(i) if removed_stmts.contains(i)));
        self.renamed.clear();
    }

    /// `NameImportingPostRector`: every fully qualified name through `NameImporter::importName()`.
    fn import_names(&mut self, options: Options) {
        let uses = self.current_uses(self.uses_root());
        for i in 0..self.names.len() {
            if !self.names[i].fq {
                continue;
            }
            let name = self.names[i].name.clone();
            let kind = self.view.names[i].kind;
            if should_skip_name(&name, kind, &uses) {
                continue;
            }
            // `FullyQualifiedNodeMapper`: an aliased name is no `FullyQualifiedObjectType`
            let original = self.names[i].original.clone().unwrap_or_default();
            if !original.is_empty() && original != name && !name.ends_with(&format!("\\{original}"))
            {
                continue;
            }
            // `resolveNameInUse()`
            if let Some(alias) = alias_of(&uses, &name) {
                self.set_text(i, alias);
                continue;
            }
            // (its import under another alias: `shouldSkipName()` skipped a name whose short name an import ends with,
            // unless the import is the name itself, whose alias the line above took)
            let namespace = self.view.names[i].namespace.clone();
            let original_unqualified = self.names[i].original.as_ref().map(|o| !o.contains('\\'));
            let at = At {
                namespace: namespace.as_deref(),
                original_unqualified,
            };
            if self.should_skip_class(&name, &at, options) {
                continue;
            }
            let short = last(&name).to_string();
            if self.pending.is_short_imported(&name) {
                if self.pending.is_import_shortable(&name) {
                    self.set_text(i, short);
                }
                continue;
            }
            if !self.has_import(&name) {
                match kind {
                    Kind::Function => self.pending.functions.push(name.clone()),
                    Kind::Constant => self.pending.constants.push(name.clone()),
                    Kind::Class => self.pending.uses.push(name.clone()),
                }
            }
            // the name as written at the node's first token
            let written = &self.view.names[i].written;
            let unchanged =
                self.names[i].text.is_none() && !written.contains('\\') && *written == short;
            if !unchanged {
                self.set_text(i, short);
            }
        }
    }

    /// The name `i` becomes the `Name` `text`.
    fn set_text(&mut self, i: usize, text: String) {
        let state = &mut self.names[i];
        state.fq = false;
        state.original = None;
        state.name = text.clone();
        state.text = Some(text);
    }

    /// `ClassNameImportSkipper::shouldSkipNameForFullyQualifiedObjectType()`: the voters in their order.
    fn should_skip_class(&mut self, class: &str, at: &At<'_>, options: Options) -> bool {
        let short_lower = last(class).to_ascii_lowercase();
        // `AliasClassNameImportSkipVoter`
        let long_lower = class.to_ascii_lowercase();
        for aliased in self.aliased_uses() {
            let aliased = aliased.to_ascii_lowercase();
            if aliased.ends_with(&format!("\\{short_lower}"))
                || (aliased == short_lower && long_lower == short_lower)
            {
                return true;
            }
        }
        // `ClassLikeNameClassNameImportSkipVoter`
        let namespace = at.namespace.unwrap_or_default().to_ascii_lowercase();
        let sub = match class.rfind('\\') {
            Some(p) => class[..p].to_ascii_lowercase(),
            None => String::new(),
        };
        for short in self.short_class_like_names() {
            if short.to_ascii_lowercase() != short_lower {
                continue;
            }
            if namespace.is_empty() || namespace != sub {
                return true;
            }
        }
        // `FullyQualifiedNameClassNameImportSkipVoter`
        let fq_short = last(class).to_string();
        // (no short name starts with `\\`: the voter's `ltrim()` of one never applies)
        for (short, full) in self.short_names().clone() {
            if fq_short != short {
                continue;
            }
            if class != full.trim_start_matches('\\') {
                return true;
            }
            break;
        }
        // `UsesClassNameImportSkipVoter`
        for t in self.used_import_types() {
            if !t.equals(class) && t.short_names_equal(class) {
                return true;
            }
            if t.equals(class) {
                break;
            }
        }
        // `ReservedClassNameImportSkipVoter`
        const RESERVED: &[&str] = &[
            "bool", "false", "float", "int", "null", "parent", "self", "static", "string", "true",
            "void", "never", "iterable", "object", "mixed", "array", "callable",
        ];
        if RESERVED.contains(&short_lower.as_str()) {
            return true;
        }
        // `ShortClassImportSkipVoter`
        if !class.trim_start_matches('\\').contains('\\') && !options.import_short_classes {
            return true;
        }
        // `OriginalNameImportSkipVoter`
        class.contains('\\') && at.original_unqualified == Some(true)
    }

    /// `AliasUsesResolver::resolveFromNode()`: the classes every aliased class import of the file names.
    fn aliased_uses(&self) -> Vec<String> {
        let mut names = Vec::new();
        for r in 0..self.view.roots.len() {
            for u in self.current_uses(r) {
                let group = u.prefix.is_some();
                if group && u.kind.is_some() {
                    continue;
                }
                if !group && u.kind != Some(Kind::Class) {
                    continue;
                }
                for (i, item) in u.items.iter().enumerate() {
                    if item.kind == Kind::Class && item.alias.is_some() {
                        names.push(match &u.prefix {
                            Some(p) => format!("{p}\\{}", item.name),
                            None => u.full_name(i),
                        });
                    }
                }
            }
        }
        names
    }

    /// `ShortNameResolver::resolveShortClassLikeNames()`: the short names of the class-likes of the imports root.
    fn short_class_like_names(&self) -> Vec<String> {
        // (the voters run on a file with one namespace at most, `AddUseStatementGuard`)
        let root = (self.view.namespaces == 1).then(|| self.uses_root());
        let mut names: Vec<String> = Vec::new();
        for c in &self.view.class_likes {
            if root.is_none_or(|r| c.root == r) && !names.contains(&c.short) {
                names.push(c.short.clone());
            }
        }
        names
    }

    /// `ShortNameResolver::resolveFromFile()`, read once per file.
    fn short_names(&mut self) -> &Vec<(String, String)> {
        if self.short_names.is_none() {
            let mut entries: Vec<(usize, String, String)> = Vec::new();
            for c in &self.view.class_likes {
                entries.push((c.start, c.short.clone(), c.name.clone()));
            }
            for (i, state) in self.names.iter().enumerate() {
                if let Some(original) = &state.original
                    && !original.contains('\\')
                {
                    entries.push((
                        self.view.names[i].span.start,
                        original.clone(),
                        state.name.clone(),
                    ));
                }
            }
            entries.sort_by_key(|e| e.0);
            let mut map: Vec<(String, String)> = Vec::new();
            let insert = |map: &mut Vec<(String, String)>, k: String, v: String| match map
                .iter_mut()
                .find(|e| e.0 == k)
            {
                Some(e) => e.1 = v,
                None => map.push((k, v)),
            };
            for (_, k, v) in entries {
                insert(&mut map, k, v);
            }
            // the `use` statements still there as parsed, `GroupUse` aside
            let mut stmt_uses = Vec::new();
            for r in 0..self.view.roots.len() {
                for (k, u) in self.view.roots[r].uses.iter().enumerate() {
                    let still = self.lists[r].items.iter().any(|item| match item {
                        Item::Old(i) => self.view.roots[r].stmts[*i].kind == StmtKind::Use(k),
                        _ => false,
                    });
                    if u.prefix.is_none() && still {
                        let mut u = u.clone();
                        u.items = u
                            .items
                            .into_iter()
                            .zip(&self.lists[r].kept[k])
                            .filter(|(_, k)| **k)
                            .map(|(i, _)| i)
                            .collect();
                        stmt_uses.push(u);
                    }
                }
            }
            let mut doc_names = Vec::new();
            for d in self.docs.iter().filter(|d| d.last) {
                match &d.doc {
                    Some(doc) => doc_names.extend(doc.short_names()),
                    None => self.unsupported = true,
                }
            }
            let mut doc_map: Vec<(String, String)> = Vec::new();
            for short in doc_names {
                if let Some(full) = resolve::match_name_with_uses(&stmt_uses, &short) {
                    insert(&mut doc_map, short, full);
                }
            }
            for (k, v) in doc_map {
                insert(&mut map, k, v);
            }
            self.short_names = Some(map);
        }
        self.short_names.as_ref().unwrap()
    }

    /// `DocblockNameImportingPostRector`: the names of the docs of statements and parameters.
    fn import_doc_names(&mut self, options: Options) {
        for d in 0..self.docs.len() {
            // a doc no node takes is a `Nop`'s, a statement's
            let owned = self.docs[d]
                .view
                .is_none_or(|v| self.view.docs[v].owners.stmt || self.view.docs[v].owners.param);
            if !owned || !self.docs[d].last {
                continue;
            }
            let Some(doc) = self.docs[d].doc.clone() else {
                self.unsupported = true;
                return;
            };
            if doc.children.is_empty() {
                continue;
            }
            let namespace = self.docs[d].namespace.clone();
            let class = self.docs[d].class.clone();
            let mut doc = doc;
            for ident in doc.idents() {
                let annotation = matches!(
                    doc.children[ident.child].value,
                    super::doc::Value::Annotation(_)
                );
                match (annotation, ident.index) {
                    // the annotation's own name is read twice: as the tag (`@` and `\` trimmed,
                    // `enterSpacelessPhpDocTagNode()`) and as its identifier (`processDoctrineAnnotationTagValueNode()`)
                    (true, 0) => {
                        let tag = doc.children[ident.child].tag.clone().unwrap_or_default();
                        if let Some(short) = self.import_doc_name(
                            tag.trim_start_matches(['@', '\\']),
                            false,
                            namespace.as_deref(),
                            class.as_deref(),
                            options,
                        ) {
                            doc.set_tag(ident.child, &format!("@{short}"));
                        }
                        if let Some(short) = self.import_doc_name(
                            &ident.name,
                            false,
                            namespace.as_deref(),
                            class.as_deref(),
                            options,
                        ) {
                            doc.set_ident(&ident, &short);
                        }
                    }
                    // a nested annotation's identifier (`processDoctrineAnnotationTagValueNode()`, which reads it
                    // without a `\`)
                    (true, _) => {
                        if let Some(short) = self.import_doc_name(
                            &ident.name,
                            false,
                            namespace.as_deref(),
                            class.as_deref(),
                            options,
                        ) {
                            doc.set_ident(&ident, &short);
                        }
                    }
                    (false, _) if ident.name.contains('\\') => {
                        if let Some(short) = self.import_doc_name(
                            &ident.name,
                            ident.template_parent,
                            namespace.as_deref(),
                            class.as_deref(),
                            options,
                        ) {
                            doc.set_ident(&ident, &short);
                        }
                    }
                    _ => {}
                }
            }
            if doc.is_changed() {
                self.docs[d].doc = Some(doc);
            }
        }
    }

    /// `NameImportingPhpDocNodeVisitor` on the identifier `name`: the short name it becomes, after the import is queued.
    fn import_doc_name(
        &mut self,
        name: &str,
        template_parent: bool,
        namespace: Option<&str>,
        class: Option<&str>,
        options: Options,
    ) -> Option<String> {
        let uses = self.current_uses(self.uses_root());
        let scope = Scope {
            namespace,
            class,
            uses: &uses,
        };
        let ty = resolve::map_identifier(self.ctx, scope, name);
        let fq = ty.fully_qualified()?.to_string();
        if template_parent {
            return None;
        }
        let fq = fq.trim_start_matches('@').to_string();
        let at = At {
            namespace,
            original_unqualified: None,
        };
        if self.should_skip_class(&fq, &at, options) {
            return None;
        }
        let short = last(&fq).to_string();
        if self.pending.is_short_imported(&fq) && !self.pending.is_import_shortable(&fq) {
            return None;
        }
        if !self.should_import(&short, name, &fq) {
            return None;
        }
        self.pending.uses.push(fq);
        Some(short)
    }

    /// `NameImportingPhpDocNodeVisitor::shouldImport()`.
    fn should_import(&self, short: &str, name: &str, class: &str) -> bool {
        if short == name {
            return false;
        }
        if let Some(rest) = name.strip_prefix('\\') {
            if short != class {
                return short != rest;
            }
            return true;
        }
        if !has_class(self.ctx, class) {
            return false;
        }
        // (`name` has no leading `\\` here)
        match name.find(&format!("\\{short}")) {
            None => !self.has_import(class),
            Some(p) => name[..p].trim_start_matches('\\').split('\\').count() > 1,
        }
    }

    /// `UseAddingPostRector`: `FileNode::addImports()` of the pending imports.
    fn add_uses(&mut self) {
        if self.pending.is_empty() {
            return;
        }
        let mut classes: Vec<String> = Vec::new();
        for c in &self.pending.uses {
            if !classes.contains(c) {
                classes.push(c.clone());
            }
        }
        let r = self.view.placement_root();
        let namespace = self.view.roots[r]
            .is_namespace
            .then(|| self.view.roots[r].name.clone())
            .flatten();
        let is_namespace = self.view.roots[r].is_namespace;
        if !is_namespace {
            classes.retain(|c| c.contains('\\'));
        }
        classes.retain(|c| {
            !self
                .used
                .0
                .iter()
                .any(|t| matches!(t, ObjType::Fq(n) if n == c))
        });
        let constants: Vec<String> = self
            .pending
            .constants
            .iter()
            .filter(|c| !self.used.2.contains(c))
            .cloned()
            .collect();
        let functions: Vec<String> = self
            .pending
            .functions
            .iter()
            .filter(|c| !self.used.1.contains(c))
            .cloned()
            .collect();
        let mut new_uses = Vec::new();
        for (kind, list) in [
            (Kind::Class, &classes),
            (Kind::Constant, &constants),
            (Kind::Function, &functions),
        ] {
            for name in list {
                let current = namespace.as_ref().is_some_and(|ns| {
                    name.starts_with(&format!("{ns}\\")) && format!("{ns}\\{}", last(name)) == *name
                });
                if current || (namespace.is_none() && !name.trim_start_matches('\\').contains('\\'))
                {
                    continue;
                }
                new_uses.push(Item::New {
                    kind,
                    name: name.clone(),
                    comments: None,
                });
            }
        }
        // (`appendUsedImports()`)
        self.used.0.extend(classes.into_iter().map(ObjType::Fq));
        self.used.1.extend(functions);
        self.used.2.extend(constants);
        if new_uses.is_empty() {
            return;
        }
        let root = &self.view.roots[r];
        let list = &mut self.lists[r];
        let is_old = |item: &Item, kind: StmtKind| matches!(item, Item::Old(i) if root.stmts[*i].kind == kind);
        // (no new import is in the list yet)
        let is_use = |item: &Item| matches!(item, Item::Old(i) if matches!(root.stmts[*i].kind, StmtKind::Use(_)));
        // a `Use_`, not a `GroupUse`, with comments: they move to the first new import
        let mirror = |list: &mut ListState, new_uses: &mut Vec<Item>, at: usize| {
            if let Some(Item::Old(i)) = list.items.get(at)
                && let StmtKind::Use(k) = root.stmts[*i].kind
                && root.uses[k].prefix.is_none()
                && root.stmts[*i].comment_start < root.stmts[*i].span.start
                && !list.cleared[*i]
            {
                if let Some(Item::New { comments, .. }) = new_uses.first_mut() {
                    *comments = Some(*i);
                }
                list.cleared[*i] = true;
            }
        };
        let mut new_uses = new_uses;
        for key in 0..list.items.len() {
            if matches!(list.items[key], Item::Nop(_)) {
                continue;
            }
            if !is_old(&list.items[key], StmtKind::Declare) {
                break;
            }
            mirror(list, &mut new_uses, key + 1);
            if let Some(Item::Old(next)) = list.items.get(key + 1)
                && matches!(root.stmts[*next].kind, StmtKind::Use(_))
            {
                list.reprinted[*next] = true;
            }
            let mut added = vec![Item::Nop(None)];
            added.extend(new_uses);
            list.items.splice(key + 1..key + 1, added);
            return;
        }
        mirror(list, &mut new_uses, 0);
        let nop = list.items.first().is_some_and(|first| !is_use(first));
        if nop {
            new_uses.push(Item::Nop(None));
        }
        list.items.splice(0..0, new_uses);
    }

    /// `UnusedImportRemovingPostRector`: on the file, then on each namespace.
    fn remove_unused_imports(&mut self) {
        let order: Vec<usize> = (0..self.view.roots.len()).collect();
        for r in order {
            let names = self.used_names(r);
            let root = &self.view.roots[r];
            let namespace = root.name.clone();
            let list = &mut self.lists[r];
            let first_key = usize::from(
                matches!(list.items.first(), Some(Item::Old(i)) if root.stmts[*i].kind == StmtKind::Declare),
            );
            let lower: Vec<String> = names.iter().map(|n| n.to_ascii_lowercase()).collect();
            let mut new_items = Vec::new();
            for (key, item) in list.items.iter().enumerate() {
                let (kind, items, k): (Kind, Vec<UseItemView>, Option<usize>) = match item {
                    Item::Old(i) => match root.stmts[*i].kind {
                        StmtKind::Use(k) if root.uses[k].prefix.is_none() => (
                            root.uses[k].kind.unwrap(),
                            root.uses[k]
                                .items
                                .iter()
                                .zip(&list.kept[k])
                                .filter(|(_, k)| **k)
                                .map(|(i, _)| i.clone())
                                .collect(),
                            Some(k),
                        ),
                        _ => {
                            new_items.push(item.clone());
                            continue;
                        }
                    },
                    Item::New { kind, name, .. } => (*kind, new_use_view(*kind, name).items, None),
                    Item::Nop(_) => {
                        new_items.push(item.clone());
                        continue;
                    }
                };
                if names.is_empty() {
                    continue;
                }
                let case_sensitive = kind == Kind::Constant;
                let list_names = if case_sensitive { &names } else { &lower };
                let namespace = namespace.as_ref().map(|n| {
                    if case_sensitive {
                        n.clone()
                    } else {
                        n.to_ascii_lowercase()
                    }
                });
                let used: Vec<bool> = items
                    .iter()
                    .map(|i| is_import_used(i, case_sensitive, list_names, namespace.as_deref()))
                    .collect();
                if let Some(k) = k {
                    let mut at = 0;
                    for flag in list.kept[k].iter_mut().filter(|f| **f) {
                        *flag = used[at];
                        at += 1;
                    }
                }
                if used.iter().any(|u| *u) {
                    new_items.push(item.clone());
                    continue;
                }
                let comments = match item {
                    Item::Old(i) => (root.stmts[*i].comment_start < root.stmts[*i].span.start
                        && !list.cleared[*i])
                        .then_some(*i),
                    Item::New { comments: c, .. } | Item::Nop(c) => *c,
                };
                if key == first_key
                    && let Some(c) = comments
                {
                    new_items.push(Item::Nop(Some(c)));
                }
            }
            list.items = new_items;
        }
    }

    /// `resolveUsedPhpAndDocNames()` of the root `r`: the names of its code (`use` statements aside) and its docs.
    fn used_names(&mut self, r: usize) -> Vec<String> {
        let root = &self.view.roots[r];
        // (a namespace's statements include the `Nop` of the comments after its last one)
        let last = root.stmts.last().map_or(0, |s| s.span.end);
        let region = match &root.header {
            Some(h) => Span::new(root.inner.start, h.span.end.max(root.inner.end).max(last)),
            None => Span::new(0, self.source.len()),
        };
        let inside = |s: Span| region.start <= s.start && s.end <= region.end;
        let mut names = Vec::new();
        if !root.is_namespace {
            // the names of the `namespace` statements
            names.extend(self.view.roots.iter().filter_map(|r| r.name.clone()));
        }
        let mut code: Vec<(usize, String)> = Vec::new();
        for (i, n) in self.view.names.iter().enumerate() {
            if inside(n.span) {
                let state = &self.names[i];
                let name = match (&state.original, state.fq) {
                    (Some(original), true) => original.clone(),
                    _ => state.name.clone(),
                };
                code.push((n.span.start, name));
            }
        }
        // a `GroupUse` is no `Use_`: its prefix and its items are names
        for root in &self.view.roots {
            for u in root
                .uses
                .iter()
                .filter(|u| u.prefix.is_some() && inside(u.span))
            {
                code.push((u.span.start, u.prefix.clone().unwrap()));
                for item in &u.items {
                    code.push((item.name_span.start, item.name.clone()));
                }
            }
        }
        code.sort_by_key(|c| c.0);
        names.extend(code.into_iter().map(|c| c.1));
        let uses = self.view.uses_root().uses.clone();
        for d in &self.docs {
            if !inside(d.span) {
                continue;
            }
            let Some(doc) = &d.doc else {
                self.unsupported = true;
                continue;
            };
            let scope = Scope {
                namespace: d.namespace.as_deref(),
                class: d.class.as_deref(),
                uses: &uses,
            };
            names.extend(doc.used_names(&|name| resolve::resolve_string_name(scope, name)));
        }
        let mut unique = Vec::new();
        for n in names {
            if !unique.contains(&n) {
                unique.push(n);
            }
        }
        unique
    }

    /// The file printed: each changed statement list through `pArray()` (or its fallback), the names and docs that
    /// changed in place.
    fn print(&mut self) -> Option<String> {
        let source = self.source;
        let mut edits: Vec<(Span, String)> = Vec::new();
        for (i, state) in self.names.iter().enumerate() {
            if let Some(text) = &state.text
                && *text != self.view.names[i].written
            {
                edits.push((self.view.names[i].span, text.clone()));
            }
        }
        for d in &self.docs {
            let Some(doc) = d.doc.as_ref().filter(|doc| doc.is_changed()) else {
                continue;
            };
            if !doc.reprintable() {
                return None;
            }
            match d.view {
                Some(v) => {
                    let edit = DocTarget::of(source, self.view.docs[v].node_start)
                        .edit(source, &doc.print());
                    edits.push((edit.span, edit.text));
                }
                // a `Nop` printed anew: its comments, then the empty statement on the next line (its indentation
                // stripped, `EXTRA_SPACE_BEFORE_NOP_REGEX`); a later round finds it printed already
                None if doc.print() != source[d.span.start..d.span.end] => {
                    let end = nop_run(&crate::rules::php_parser::tokens(source), d.span.end).1;
                    let text = format!("{}{}\n", doc.print(), &source[d.span.end..end]);
                    edits.push((Span::new(d.span.start, end), text));
                }
                None => {}
            }
        }
        // a class's `implements` without the names it dropped
        for class in &self.view.class_likes {
            let spans: Vec<Span> = class.implements.iter().map(|i| i.0).collect();
            let kept: Vec<bool> = spans
                .iter()
                .map(|s| !self.implements_removed.contains(s))
                .collect();
            if kept.iter().all(|k| *k) {
                continue;
            }
            let (pieces, end) = differ::removed_items(&spans, &kept, spans[0].start)?;
            let text = render_pieces(source, &pieces, &edits);
            edits.push((Span::new(spans[0].start, end), text));
        }
        // `use` statements without the items that went
        for r in 0..self.view.roots.len() {
            let root = &self.view.roots[r];
            for item in &self.lists[r].items {
                let Item::Old(i) = item else {
                    continue;
                };
                let StmtKind::Use(k) = root.stmts[*i].kind else {
                    continue;
                };
                let kept = &self.lists[r].kept[k];
                if kept.iter().all(|k| *k) || self.lists[r].reprinted[*i] {
                    continue;
                }
                let u = &root.uses[k];
                let spans: Vec<Span> = u.items.iter().map(|i| i.span).collect();
                // (an item is kept: a `use` without any is gone)
                let (pieces, end) = differ::removed_items(&spans, kept, u.span.start)?;
                edits.push((
                    Span::new(u.span.start, end),
                    render_pieces(source, &pieces, &edits),
                ));
            }
        }
        // the statement lists
        let mut file_count_changed = false;
        let mut segments: Vec<(Span, String)> = Vec::new();
        let mut fell_back = Vec::new();
        for r in 0..self.view.roots.len() {
            let root = &self.view.roots[r];
            let list = &self.lists[r];
            let unchanged = list.items.len() == root.stmts.len()
                && list.items.iter().enumerate().all(|(n, item)| match item {
                    Item::Old(i) => *i == n && !list.reprinted[*i] && !list.cleared[*i],
                    Item::Nop(Some(i)) => *i == n && root.stmts[*i].kind == StmtKind::Nop,
                    _ => false,
                });
            if unchanged {
                continue;
            }
            if r == 0 {
                file_count_changed = list.items.len() != root.stmts.len();
            }
            let old: Vec<OldItem> = root
                .stmts
                .iter()
                .map(|s| OldItem {
                    comment_start: s.comment_start,
                    start: s.span.start,
                    end: s.span.end,
                })
                .collect();
            let new: Vec<NewItem> = list
                .items
                .iter()
                .map(|item| self.new_item(r, item))
                .collect();
            let has_nop = list.items.iter().any(|i| matches!(i, Item::Nop(_)));
            let pos = root.header.as_ref().map_or(0, |h| h.list_pos);
            // php-parser checks the braces of a single statement's list
            let braces = match &root.header {
                Some(h) => h.braced,
                None => false,
            };
            match differ::statements(source, &old, &new, pos, braces) {
                Some((pieces, end)) => {
                    let mut text = render_pieces(source, &pieces, &edits);
                    if has_nop {
                        text = differ::strip_blank_lines(&text, pos > 0);
                    }
                    segments.push((Span::new(pos, end), text));
                }
                None => {
                    // a statement that ends with `?>` before others: the printer writes `?>` again after it, so what
                    // follows becomes inline HTML (the page shows it, or PHP code turns into text); Rector writes that,
                    // Pun leaves the file alone
                    let stmts = &root.stmts;
                    if stmts[..stmts.len() - 1]
                        .iter()
                        .any(|s| self.source[..s.span.end].ends_with("?>"))
                    {
                        return None;
                    }
                    segments.push(self.fallback(r, &edits));
                    fell_back.push(r);
                }
            }
        }
        // a braced namespace printed `namespace X;` beside one still braced: Rector writes that mix, which PHP rejects
        let braced = |r: &usize| {
            self.view.roots[*r]
                .header
                .as_ref()
                .is_some_and(|h| h.braced)
        };
        if self.semicolon_namespaces()
            && fell_back.iter().any(braced)
            && (0..self.view.roots.len()).any(|r| braced(&r) && !fell_back.contains(&r))
        {
            return None;
        }
        // the segments take the place of the edits inside them
        let mut all: Vec<(Span, String)> = segments.clone();
        all.extend(edits.into_iter().filter(|(s, _)| {
            !segments
                .iter()
                .any(|(g, _)| g.start <= s.start && s.end <= g.end)
        }));
        let mut text = render(source, &all, 0, source.len());
        // `BetterStandardPrinter::printFormatPreserving()`: a new line in case of added statements
        if file_count_changed && !text.ends_with('\n') {
            text.push('\n');
        }
        Some(text)
    }

    /// The old `use` statement `k` of the root `r` with its kept items.
    fn current_uses_of(&self, r: usize, k: usize) -> UseView {
        let mut u = self.view.roots[r].uses[k].clone();
        u.items = u
            .items
            .into_iter()
            .zip(&self.lists[r].kept[k])
            .filter(|(_, k)| **k)
            .map(|(i, _)| i)
            .collect();
        u
    }

    /// The differ's item for `item` of the root `r`.
    fn new_item(&self, r: usize, item: &Item) -> NewItem {
        let root = &self.view.roots[r];
        let list = &self.lists[r];
        match item {
            // a `use` printed anew (the only statement that is), or the old statement kept
            Item::Old(i) => {
                let reprinted = match (list.reprinted[*i], root.stmts[*i].kind) {
                    (true, StmtKind::Use(k)) => Some(k),
                    _ => None,
                };
                NewItem {
                    key: reprinted.is_none().then_some(*i),
                    text: reprinted
                        .map_or_else(String::new, |k| print_use(&self.current_uses_of(r, k))),
                    comments: match list.cleared[*i] {
                        true => Comments::None,
                        false => Comments::Of(*i),
                    },
                }
            }
            Item::New {
                kind,
                name,
                comments,
            } => NewItem {
                key: None,
                text: print_use(&new_use_view(*kind, name)),
                comments: match comments {
                    Some(i) => Comments::Printed(self.comment_texts(r, *i)),
                    None => Comments::None,
                },
            },
            Item::Nop(Some(i)) if root.stmts[*i].kind == StmtKind::Nop => NewItem::kept(*i),
            Item::Nop(comments) => NewItem {
                key: None,
                text: String::new(),
                comments: match comments {
                    Some(i) => Comments::Of(*i),
                    None => Comments::None,
                },
            },
        }
    }

    /// The comments of the old statement `i` of the root `r`, a changed doc as it is now.
    fn comment_texts(&self, r: usize, i: usize) -> Vec<String> {
        let stmt = &self.view.roots[r].stmts[i];
        let end = match stmt.kind {
            StmtKind::Nop => stmt.span.end + 1,
            _ => stmt.span.start,
        };
        crate::rules::php_parser::tokens(self.source)
            .into_iter()
            .filter(|t| t.kind.is_comment() && t.start >= stmt.comment_start && t.end <= end)
            .map(|t| {
                let span = Span::new(t.start, t.end);
                match self
                    .docs
                    .iter()
                    .find(|d| d.span == span)
                    .and_then(|d| d.doc.as_ref().filter(|doc| doc.is_changed()))
                {
                    Some(doc) => doc.print(),
                    None => self.source[t.start..t.end]
                        .trim_end_matches(['\n', '\r'])
                        .to_string(),
                }
            })
            .collect()
    }

    /// `canUseSemicolonNamespaces`: no namespace of the file is without a name.
    fn semicolon_namespaces(&self) -> bool {
        !self
            .view
            .roots
            .iter()
            .any(|r| r.is_namespace && r.name.is_none())
    }

    /// `pStmt_Namespace()`, or the file's `"<?php\n" . pStmts()`, where `pArray()` gives up on the list of the root `r`.
    fn fallback(&self, r: usize, edits: &[(Span, String)]) -> (Span, String) {
        let root = &self.view.roots[r];
        let list = &self.lists[r];
        // `namespace X;` and the statements, or `namespace X {` and the statements indented
        let (mut text, level) = match (&root.name, root.is_namespace) {
            (Some(name), _) if self.semicolon_namespaces() => (format!("namespace {name};\n"), 0),
            (name, true) => (
                format!(
                    "namespace{} {{",
                    name.as_ref().map_or(String::new(), |n| format!(" {n}"))
                ),
                4,
            ),
            (_, false) => ("<?php\n".to_string(), 0),
        };
        let nl = format!("\n{}", " ".repeat(level));
        for item in &list.items {
            let comments = match item {
                Item::Old(i) if !list.cleared[*i] => self.comment_texts(r, *i),
                Item::Nop(Some(i)) => self.comment_texts(r, *i),
                Item::New {
                    comments: Some(i), ..
                } => self.comment_texts(r, *i),
                _ => Vec::new(),
            };
            if !comments.is_empty() {
                let printed: Vec<String> = comments
                    .iter()
                    .map(|c| crate::rules::phpunit::annotations::reformatted(c).replace('\n', &nl))
                    .collect();
                text.push_str(&nl);
                text.push_str(&printed.join(&nl));
                if matches!(item, Item::Nop(_)) {
                    continue;
                }
            }
            text.push_str(&nl);
            match item {
                Item::Old(i) => {
                    let span = root.stmts[*i].span;
                    match (list.reprinted[*i], root.stmts[*i].kind) {
                        (true, StmtKind::Use(k)) => {
                            text.push_str(&print_use(&self.current_uses_of(r, k)))
                        }
                        // `pStmt_InlineHTML()`: `?>`, the line break the closing tag took (`hasLeadingNewline`, also
                        // set on HTML that starts the file), the HTML, `<?php `
                        (_, StmtKind::Inline) => {
                            let html = &self.source[span.start..span.end];
                            let rest = html
                                .strip_prefix("\r\n")
                                .or_else(|| html.strip_prefix('\n'));
                            let (newline, value) = match rest {
                                _ if span.start == 0 => ("\n", html),
                                Some(rest) => ("\n", rest),
                                None => ("", html),
                            };
                            text.push_str(&format!("?>{newline}{value}<?php "));
                        }
                        // the printer copies the statement, its indentation made the level's
                        _ => {
                            let printed = render(self.source, edits, span.start, span.end);
                            let adjustment =
                                level as isize - indentation(self.source, span.start) as isize;
                            text.push_str(&differ::token_code(&printed, adjustment));
                        }
                    }
                }
                Item::New { kind, name, .. } => {
                    text.push_str(&print_use(&new_use_view(*kind, name)))
                }
                Item::Nop(_) => {}
            }
        }
        if level > 0 {
            text.push_str("\n}");
        }
        let span = match &root.header {
            Some(h) if h.braced => h.span,
            // `Parser::fixupNamespaceAttributes()`: a namespace without braces ends with its last statement, before a
            // closing tag
            Some(h) => Span::new(
                h.span.start,
                root.stmts.last().map_or(root.inner.start, |s| s.span.end),
            ),
            None => Span::new(0, self.source.len()),
        };
        (span, text)
    }
}

/// The comments that follow the one ending at `end` up to the next code, as one `Nop` holds them: whether none of
/// them is a doc comment, and where the last of them ends.
fn nop_run(tokens: &[crate::rules::php_parser::Token], end: usize) -> (bool, usize) {
    let (mut last_doc, mut run_end) = (true, end);
    for t in tokens.iter().filter(|t| t.start >= end) {
        match t.kind {
            mago_syntax::token::TokenKind::Whitespace => {}
            mago_syntax::token::TokenKind::DocBlockComment => (last_doc, run_end) = (false, t.end),
            kind if kind.is_comment() => run_end = t.end,
            _ => break,
        }
    }
    (last_doc, run_end)
}

/// `ClassNameImportSkipper::shouldSkipName()`.
fn should_skip_name(name: &str, kind: Kind, uses: &[UseView]) -> bool {
    if !name.contains('\\') {
        return kind != Kind::Class;
    }
    let short = last(name);
    for u in uses {
        let prefix = u.prefix();
        for item in &u.items {
            if !item.last().eq_ignore_ascii_case(short) {
                continue;
            }
            // (an alias, one part, is never the qualified `name`)
            return format!("{prefix}{}", item.name) != name;
        }
    }
    false
}

/// `AliasNameResolver::resolveByName()`: the alias of the import of `name`.
fn alias_of(uses: &[UseView], name: &str) -> Option<String> {
    for u in uses {
        for (i, item) in u.items.iter().enumerate() {
            if let Some(alias) = &item.alias
                && u.full_name(i) == name
            {
                return Some(alias.clone());
            }
        }
    }
    None
}

/// `UnusedImportRemovingPostRector::isUseImportUsed()`.
fn is_import_used(
    item: &UseItemView,
    case_sensitive: bool,
    names: &[String],
    namespace: Option<&str>,
) -> bool {
    let mut compared = item.alias.clone().unwrap_or_else(|| item.name.clone());
    if !case_sensitive {
        compared = compared.to_ascii_lowercase();
    }
    if names.contains(&compared) {
        return true;
    }
    let last_name = last(&compared).to_string();
    // (an import's name never ends with `\\`: its last part is never empty)
    let prefix = format!("{last_name}\\");
    for name in names {
        if name.starts_with('\\') {
            continue;
        }
        // `isSubNamespace()`
        if !name.contains('\\') && compared.ends_with(&format!("\\{name}")) {
            return true;
        }
        if let Some(sub) = name.strip_prefix(&prefix)
            && !sub.contains('\\')
        {
            return true;
        }
        if !name.starts_with(&format!("{last_name}\\")) {
            if name.starts_with(&format!("{compared}\\")) {
                return true;
            }
            continue;
        }
        let Some(namespace) = namespace else {
            return true;
        };
        if !name.starts_with(&format!("{namespace}\\")) {
            return true;
        }
    }
    false
}

/// A `use` statement `FileNode::createUses()` makes, as a view.
fn new_use_view(kind: Kind, name: &str) -> UseView {
    UseView {
        span: Span::new(0, 0),
        kind: Some(kind),
        prefix: None,
        items: vec![UseItemView {
            name: name.to_string(),
            name_span: Span::new(0, 0),
            alias: None,
            kind,
            span: Span::new(0, 0),
        }],
    }
}

/// `pStmt_Use()`.
fn print_use(u: &UseView) -> String {
    let kind = match u.kind {
        Some(Kind::Function) => "function ",
        Some(Kind::Constant) => "const ",
        _ => "",
    };
    let items: Vec<String> = u
        .items
        .iter()
        .map(|i| match &i.alias {
            Some(alias) => format!("{} as {alias}", i.name),
            None => i.name.clone(),
        })
        .collect();
    format!("use {kind}{};", items.join(", "))
}

/// The pieces, the file's bytes with the edits inside them.
fn render_pieces(source: &str, pieces: &[Piece], edits: &[(Span, String)]) -> String {
    pieces
        .iter()
        .map(|p| match p {
            Piece::Source(s) => render(source, edits, s.start, s.end),
            Piece::Text(t) => t.clone(),
        })
        .collect()
}

/// `source[from..to]` with the edits that lie inside it; of two that overlap, the one that starts first, the longer
/// at one start (an edit that holds others was rendered with them).
fn render(source: &str, edits: &[(Span, String)], from: usize, to: usize) -> String {
    let mut inside: Vec<&(Span, String)> = edits
        .iter()
        .filter(|(s, _)| from <= s.start && s.end <= to)
        .collect();
    inside.sort_by_key(|(s, _)| (s.start, std::cmp::Reverse(s.end)));
    let mut out = String::new();
    let mut at = from;
    for (span, text) in inside {
        if span.start < at {
            continue;
        }
        out.push_str(&source[at..span.start]);
        out.push_str(text);
        at = span.end;
    }
    out.push_str(&source[at..to]);
    out
}

/// The doc identifier's type, for the rules (`IdentifierPhpDocTypeMapper`).
pub(crate) fn doc_type(ctx: &Context<'_>, scope: Scope<'_>, name: &str) -> DocType {
    resolve::map_identifier(ctx, scope, name)
}
