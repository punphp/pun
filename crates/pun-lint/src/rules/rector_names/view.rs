//! php-parser's view of the names in a file, as Rector sees them once PHPStan's parser (php-parser's `NameResolver`,
//! `replaceNodes` and `preserveOriginalNames` on) and Rector's decorating visitors (`NameNodeVisitor`,
//! `ClassConstFetchNodeVisitor`, `ContextNodeVisitor`) ran: every `Name` node with what it resolves to and the
//! attributes Rector reads; the `use` imports of each namespace (`FileNode::getUsesAndGroupUses()`); the class-likes;
//! and the doc comments with the nodes php-parser gives them to.

use std::collections::HashMap;

use mago_syntax::cst::*;
use pun_problem::Span;

use crate::rules::rector_stmts::tokens;
use crate::syntax::node_span;

/// What a name stands for: php-parser's `Use_::TYPE_*` as the resolver uses it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Kind {
    Class,
    Function,
    Constant,
}

/// One `Name` node.
#[derive(Clone, Debug)]
pub(crate) struct PhpName {
    pub span: Span,
    /// The name as written, a leading `\` included (the original name and its token's text).
    pub written: String,
    pub kind: Kind,
    /// The `FullyQualified` node the resolver put in its place, without the leading `\`; `None` where it stays a
    /// `Name` (`self`, `parent`, `static`, and an unqualified function or constant name inside a namespace).
    pub resolved: Option<String>,
    /// `IS_NEW_INSTANCE_NAME`.
    pub new: bool,
    /// `IS_STATICCALL_CLASS_NAME`.
    pub static_call: bool,
    /// `IS_CLASS_EXTENDS` (a class's parent, not an interface's).
    pub extends: bool,
    /// `IS_CLASS_IMPLEMENT`: a class's interfaces, and an enum's (where Rector sets nothing).
    pub implements: bool,
    /// `CLASS_CONST_FETCH_NAME`: the constant `Name::CONSTANT` fetches.
    pub class_const_fetch: Option<String>,
    /// The namespace the name is in (`Scope::getNamespace()`).
    pub namespace: Option<String>,
    /// The class-like the name is in (`Scope::getClassReflection()`), by name; `Some("")` in an anonymous class.
    pub class: Option<String>,
}

impl PhpName {
    /// `Name::toString()` of the original name: the written name without its leading `\`, or without the
    /// `namespace\` of a `Name\Relative` (whose parts leave it out).
    pub fn original(&self) -> &str {
        let written = self.written.trim_start_matches('\\');
        match written.get(..10) {
            Some(prefix) if written.len() > 10 && prefix.eq_ignore_ascii_case("namespace\\") => &written[10..],
            _ => written,
        }
    }
}

/// One item of a `use` statement.
#[derive(Clone, Debug)]
pub(crate) struct UseItemView {
    /// The item's name as written, without a leading `\` (`UseItem::$name`).
    pub name: String,
    pub name_span: Span,
    /// The alias, when written (`UseItem::$alias`).
    pub alias: Option<String>,
    /// The item's own type in a mixed group (`use A\{function f, B}`), else the statement's.
    pub kind: Kind,
    /// The item with its `as` alias, as it is removed from a list.
    pub span: Span,
}

impl UseItemView {
    /// `UseItem::getAlias()`: the alias, or the last part of the name.
    pub fn alias_or_last(&self) -> &str {
        match &self.alias {
            Some(alias) => alias,
            None => self.name.rsplit('\\').next().unwrap_or_default(),
        }
    }

    /// `$useUse->name->getLast()`.
    pub fn last(&self) -> &str {
        self.name.rsplit('\\').next().unwrap_or_default()
    }
}

/// A `use` statement: php-parser's `Use_`, or `GroupUse` (`prefix` set).
#[derive(Clone, Debug)]
pub(crate) struct UseView {
    pub span: Span,
    /// The statement's type (`TYPE_UNKNOWN` for a mixed group is `None`).
    pub kind: Option<Kind>,
    /// A group's prefix, without a leading `\`.
    pub prefix: Option<String>,
    pub items: Vec<UseItemView>,
}

impl UseView {
    /// `UseImportsResolver::resolvePrefix()`.
    pub fn prefix(&self) -> String {
        match &self.prefix {
            Some(prefix) => format!("{prefix}\\"),
            None => String::new(),
        }
    }

    /// The fully qualified name of item `i` (`$prefix . $useUse->name`).
    pub fn full_name(&self, i: usize) -> String {
        format!("{}{}", self.prefix(), self.items[i].name)
    }
}

/// A namespace statement, or the file itself where it has none: where imports are placed, read and removed.
#[derive(Clone, Debug)]
pub(crate) struct Root {
    /// The namespace's name, `None` for the file and for `namespace { }`.
    pub name: Option<String>,
    /// Whether this is a `namespace` statement (`Namespace_`), not the file.
    pub is_namespace: bool,
    /// Its `use` statements, in order (`Use_` and `GroupUse`).
    pub uses: Vec<UseView>,
    /// Where the namespace's statements start and end (after its `;` or `{`, before its `}` or the next namespace).
    pub inner: Span,
    /// Its statements as php-parser keeps them (`stmts`).
    pub stmts: Vec<StmtView>,
    /// The `namespace` statement, `None` for the file.
    pub header: Option<Header>,
}

/// A `namespace` statement's own parts.
#[derive(Clone, Debug)]
pub(crate) struct Header {
    /// The statement, from `namespace` to its last statement or `}`.
    pub span: Span,
    /// Where php-parser's printer starts the statement list: after the name (after `namespace` without one).
    pub list_pos: usize,
    pub braced: bool,
}

/// What a statement of a list is, as far as the name handling tells them apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StmtKind {
    /// A `use` statement: its index in the root's `uses`.
    Use(usize),
    Declare,
    Namespace,
    /// The `Nop` php-parser adds for comments after the last statement.
    Nop,
    /// `InlineHTML`: the text after `?>`.
    Inline,
    Other,
}

/// A statement of a list.
#[derive(Clone, Copy, Debug)]
pub(crate) struct StmtView {
    pub span: Span,
    /// Where its comments start (its start without any).
    pub comment_start: usize,
    pub kind: StmtKind,
}

/// A class, interface, trait or enum declared in the file.
#[derive(Clone, Debug)]
pub(crate) struct ClassLikeView {
    /// The fully qualified name, without a leading `\`.
    pub name: String,
    /// The short name as declared.
    pub short: String,
    /// Whether it is a `class` (`Class_`).
    pub is_class: bool,
    /// The interfaces of a class, as written (`implements`), each with its span.
    pub implements: Vec<(Span, String)>,
    /// The span of the `implements` clause's types, from the first to the last.
    pub implements_span: Option<Span>,
    /// The root it is declared in.
    pub root: usize,
    /// Where it starts.
    pub start: usize,
}

/// The node kinds a doc comment is given to (php-parser's `comments` attribute of every node that starts at the token
/// after the comment), as far as Rector's visitors tell them apart.
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Owners {
    /// A `Stmt` (statements and class members).
    pub stmt: bool,
    /// A `Param`.
    pub param: bool,
    /// One of the node types `RenameClassRector` reads docs of: `Property`, `FunctionLike`, `Expression`,
    /// `ClassLike`, `If_`.
    pub renamed: bool,
}

/// A doc comment (`/** ... */`) and the nodes it belongs to.
#[derive(Clone, Debug)]
pub(crate) struct DocView {
    pub span: Span,
    pub owners: Owners,
    /// Where the node it belongs to starts.
    pub node_start: usize,
    /// Whether it is the node's last doc comment (`getDocComment()`).
    pub is_last: bool,
    /// The namespace it is in.
    pub namespace: Option<String>,
    /// The class-like whose member (or itself) it documents, by name.
    pub class: Option<String>,
}

/// The names, imports, class-likes and doc comments of a file.
#[derive(Clone, Debug, Default)]
pub(crate) struct FileView {
    pub names: Vec<PhpName>,
    pub roots: Vec<Root>,
    pub class_likes: Vec<ClassLikeView>,
    pub docs: Vec<DocView>,
    /// The number of `namespace` statements.
    pub namespaces: usize,
    /// Whether the file has inline HTML (`InlineHTML`), outside the `<?php` tag's newline.
    pub inline_html: bool,
}

impl FileView {
    /// The root whose imports Rector reads (`FileNode::getUsesAndGroupUses()`): the namespace when the file has exactly
    /// one, else the file.
    pub fn uses_root(&self) -> &Root {
        match self.namespaces {
            1 => self.roots.iter().find(|r| r.is_namespace).unwrap(),
            _ => &self.roots[0],
        }
    }

    /// The root new imports go to (`FileNode::resolvePlacementNamespace()`): the first namespace, else the file.
    pub fn placement_root(&self) -> usize {
        self.roots.iter().position(|r| r.is_namespace).unwrap_or(0)
    }

    /// Read `program` (of `source`).
    pub fn of(source: &str, program: &Program<'_>) -> FileView {
        let mut walk = Walk {
            view: FileView::default(),
            namespace: None,
            aliases: Aliases::default(),
            classes: Vec::new(),
            comments: comment_runs(source),
            root: 0,
        };
        walk.view.roots.push(Root {
            name: None,
            is_namespace: false,
            uses: Vec::new(),
            inner: Span::new(0, source.len()),
            stmts: Vec::new(),
            header: None,
        });
        walk.view.namespaces = program
            .statements
            .iter()
            .filter(|s| matches!(s, Statement::Namespace(_)))
            .count();
        walk.node(Node::Program(program));
        walk.view.names.sort_by_key(|n| n.span.start);
        let mut view = walk.view;
        let all = crate::rules::php_parser::tokens(source);
        view.roots[0].stmts = stmt_list(source, &all, program.statements.iter(), view.roots[0].inner);
        let mut root = 0;
        for statement in program.statements.iter() {
            let Statement::Namespace(namespace) = statement else {
                continue;
            };
            root += 1;
            let body: Vec<&Statement<'_>> = match &namespace.body {
                NamespaceBody::Implicit(body) => body.statements.iter().collect(),
                NamespaceBody::BraceDelimited(block) => block.statements.iter().collect(),
            };
            let inner = view.roots[root].inner;
            let inner = match &namespace.body {
                // up to the next token: the comments after the last statement belong to the list
                NamespaceBody::Implicit(_) => Span::new(
                    inner.start,
                    all.iter()
                        .find(|t| t.start >= inner.end && !t.kind.is_trivia())
                        .map_or(source.len(), |t| t.start),
                ),
                NamespaceBody::BraceDelimited(_) => inner,
            };
            view.roots[root].stmts = stmt_list(source, &all, body.into_iter(), inner);
        }
        view
    }
}

/// The alias tables of php-parser's `NameContext`: class aliases lower-cased, function aliases lower-cased, constant
/// aliases as written.
#[derive(Default)]
struct Aliases {
    class: HashMap<String, String>,
    function: HashMap<String, String>,
    constant: HashMap<String, String>,
}

struct Walk {
    view: FileView,
    namespace: Option<String>,
    aliases: Aliases,
    /// The enclosing class-likes, innermost last (`""` for an anonymous class).
    classes: Vec<String>,
    /// The doc comments before each token start, with where the token before them ends.
    comments: HashMap<usize, (usize, Vec<Span>)>,
    root: usize,
}

/// For each token that comments come before, its start: where the token before the comments ends, and the doc
/// comments among them.
fn comment_runs(source: &str) -> HashMap<usize, (usize, Vec<Span>)> {
    let mut runs = HashMap::new();
    let mut docs: Vec<Span> = Vec::new();
    let mut last_end = 0;
    let mut has_comment = false;
    for token in tokens(source, 0, source.len()) {
        if token.kind.is_comment() {
            has_comment = true;
            let text = &source[token.start..token.end];
            if text.starts_with("/**")
                && text
                    .as_bytes()
                    .get(3)
                    .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n'))
            {
                docs.push(Span::new(token.start, token.end));
            }
            continue;
        }
        if token.kind.is_trivia() {
            continue;
        }
        if has_comment {
            runs.insert(token.start, (last_end, std::mem::take(&mut docs)));
        }
        has_comment = false;
        docs.clear();
        last_end = token.end;
    }
    runs
}

fn text(id: &Identifier<'_>) -> String {
    String::from_utf8_lossy(id.value()).into_owned()
}

fn is_special_class_name(name: &str) -> bool {
    matches!(name.to_ascii_lowercase().as_str(), "self" | "parent" | "static")
}

impl Walk {
    /// php-parser's `NameContext::getResolvedName()` (`$type` the kind) of the name written `written`.
    fn resolve(&self, written: &str, kind: Kind) -> Option<String> {
        // don't resolve special class names
        if kind == Kind::Class && !written.contains('\\') && is_special_class_name(written) {
            return None;
        }
        // fully qualified names are already resolved
        if let Some(fq) = written.strip_prefix('\\') {
            return Some(fq.to_string());
        }
        let concat = |ns: &Option<String>, rest: &str| match ns {
            Some(ns) => format!("{ns}\\{rest}"),
            None => rest.to_string(),
        };
        // `namespace\Foo`: relative to the current namespace
        if written.len() > 10 && written[..10].eq_ignore_ascii_case("namespace\\") {
            return Some(concat(&self.namespace, &written[10..]));
        }
        // Try to resolve aliases
        match written.split_once('\\') {
            Some((first, rest)) => {
                // resolve aliases for qualified names, always against class alias table
                if let Some(alias) = self.aliases.class.get(&first.to_ascii_lowercase()) {
                    return Some(format!("{alias}\\{rest}"));
                }
            }
            None => {
                let found = match kind {
                    Kind::Class => self.aliases.class.get(&written.to_ascii_lowercase()),
                    Kind::Function => self.aliases.function.get(&written.to_ascii_lowercase()),
                    Kind::Constant => self.aliases.constant.get(written),
                };
                if let Some(alias) = found {
                    return Some(alias.clone());
                }
                if kind != Kind::Class {
                    // outside of a namespace unaliased unqualified is same as fully qualified
                    return match self.namespace {
                        None => Some(written.to_string()),
                        // Cannot resolve statically
                        Some(_) => None,
                    };
                }
            }
        }
        // if no alias exists prepend current namespace
        Some(concat(&self.namespace, written))
    }

    fn name(&mut self, id: &Identifier<'_>, kind: Kind) -> &mut PhpName {
        let written = text(id);
        let resolved = self.resolve(&written, kind);
        self.view.names.push(PhpName {
            span: node_span(id),
            written,
            kind,
            resolved,
            new: false,
            static_call: false,
            extends: false,
            implements: false,
            class_const_fetch: None,
            namespace: self.namespace.clone(),
            class: self.classes.last().cloned(),
        });
        self.view.names.last_mut().unwrap()
    }

    /// A class name in an expression's place (`new X`, `X::y()`), if the expression is one.
    fn class_expression(&mut self, expr: &Expression<'_>) -> Option<&mut PhpName> {
        match expr {
            Expression::Identifier(id) => Some(self.name(id, Kind::Class)),
            _ => None,
        }
    }

    fn uses(&mut self, r#use: &Use<'_>) {
        let item = |name: &Identifier<'_>, alias: &Option<UseItemAlias<'_>>, kind: Kind, span| UseItemView {
            name: text(name).trim_start_matches('\\').to_string(),
            name_span: node_span(name),
            alias: alias
                .as_ref()
                .map(|a| String::from_utf8_lossy(a.identifier.value).into_owned()),
            kind,
            span,
        };
        let kind_of = |t: &UseType<'_>| match t {
            UseType::Function(_) => Kind::Function,
            UseType::Const(_) => Kind::Constant,
        };
        let view = match &r#use.items {
            UseItems::Sequence(s) => UseView {
                span: node_span(r#use),
                kind: Some(Kind::Class),
                prefix: None,
                items: s
                    .items
                    .iter()
                    .map(|i| item(&i.name, &i.alias, Kind::Class, node_span(i)))
                    .collect(),
            },
            UseItems::TypedSequence(s) => UseView {
                span: node_span(r#use),
                kind: Some(kind_of(&s.r#type)),
                prefix: None,
                items: s
                    .items
                    .iter()
                    .map(|i| item(&i.name, &i.alias, kind_of(&s.r#type), node_span(i)))
                    .collect(),
            },
            UseItems::TypedList(l) => UseView {
                span: node_span(r#use),
                kind: Some(kind_of(&l.r#type)),
                prefix: Some(text(&l.namespace).trim_start_matches('\\').to_string()),
                items: l
                    .items
                    .iter()
                    .map(|i| item(&i.name, &i.alias, kind_of(&l.r#type), node_span(i)))
                    .collect(),
            },
            UseItems::MixedList(l) => UseView {
                span: node_span(r#use),
                kind: None,
                prefix: Some(text(&l.namespace).trim_start_matches('\\').to_string()),
                items: l
                    .items
                    .iter()
                    .map(|i| {
                        let kind = i.r#type.as_ref().map_or(Kind::Class, kind_of);
                        item(&i.item.name, &i.item.alias, kind, node_span(i))
                    })
                    .collect(),
            },
        };
        // `NameResolver::addAlias()`
        for i in 0..view.items.len() {
            let name = view.full_name(i);
            let item = &view.items[i];
            let alias = item.alias_or_last().to_string();
            match item.kind {
                Kind::Class => self.aliases.class.insert(alias.to_ascii_lowercase(), name),
                Kind::Function => self.aliases.function.insert(alias.to_ascii_lowercase(), name),
                Kind::Constant => self.aliases.constant.insert(alias, name),
            };
        }
        self.view.roots[self.root].uses.push(view);
    }

    fn start_namespace(&mut self, namespace: &Namespace<'_>) {
        self.namespace = namespace.name.as_ref().map(|n| text(n));
        self.aliases = Aliases::default();
        let inner = match &namespace.body {
            NamespaceBody::Implicit(body) => Span::new(
                node_span(&body.terminator).end,
                body.statements
                    .last()
                    .map_or(node_span(&body.terminator).end, |s| node_span(s).end),
            ),
            NamespaceBody::BraceDelimited(block) => Span::new(
                block.left_brace.end.offset as usize,
                block.right_brace.start.offset as usize,
            ),
        };
        let start = node_span(namespace).start;
        let list_pos = match &namespace.name {
            Some(name) => node_span(name).end,
            None => start + "namespace".len(),
        };
        self.view.roots.push(Root {
            name: self.namespace.clone(),
            is_namespace: true,
            uses: Vec::new(),
            inner,
            stmts: Vec::new(),
            header: Some(Header {
                span: node_span(namespace),
                list_pos,
                braced: matches!(namespace.body, NamespaceBody::BraceDelimited(_)),
            }),
        });
        self.root = self.view.roots.len() - 1;
    }

    fn class_like(&mut self, start: usize, short: &LocalIdentifier<'_>, is_class: bool) -> String {
        let short = String::from_utf8_lossy(short.value).into_owned();
        let name = match &self.namespace {
            Some(ns) => format!("{ns}\\{short}"),
            None => short.clone(),
        };
        self.view.class_likes.push(ClassLikeView {
            name: name.clone(),
            short,
            is_class,
            implements: Vec::new(),
            implements_span: None,
            root: self.root,
            start,
        });
        name
    }

    /// The names of an `implements` clause, kept as the class's list when `of_class` (an enum's too are interfaces
    /// only, which Rector does not check: Pun keeps the code valid).
    fn implements(&mut self, implements: &Implements<'_>, of_class: bool) {
        let mut list = Vec::new();
        for id in implements.types.iter() {
            list.push((node_span(id), text(id)));
            self.name(id, Kind::Class).implements = true;
        }
        if of_class {
            let class = self.view.class_likes.last_mut().unwrap();
            class.implements_span = Some(Span::new(list[0].0.start, list.last().unwrap().0.end));
            class.implements = list;
        }
    }

    fn doc(&mut self, node: &Node<'_, '_>) {
        let start = node_span(node).start;
        let Some((_, docs)) = self.comments.get(&start).cloned() else {
            return;
        };
        let stmt = matches!(node, Node::Statement(_) | Node::ClassLikeMember(_));
        let param = matches!(node, Node::FunctionLikeParameter(_));
        let renamed = matches!(
            node,
            Node::Statement(Statement::Expression(_) | Statement::If(_))
                | Node::Property(_)
                | Node::Function(_)
                | Node::Method(_)
                | Node::Closure(_)
                | Node::ArrowFunction(_)
                | Node::Class(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
        );
        let count = docs.len();
        for (i, span) in docs.into_iter().enumerate() {
            match self.view.docs.iter_mut().find(|d| d.span == span) {
                Some(doc) => {
                    doc.owners.stmt |= stmt;
                    doc.owners.param |= param;
                    doc.owners.renamed |= renamed;
                }
                None => self.view.docs.push(DocView {
                    span,
                    owners: Owners {
                        stmt,
                        param,
                        renamed,
                    },
                    node_start: start,
                    is_last: i + 1 == count,
                    namespace: self.namespace.clone(),
                    class: self.classes.last().cloned(),
                }),
            }
        }
    }

    fn node(&mut self, node: Node<'_, '_>) {
        let mut pushed_class = false;
        match node {
            Node::Namespace(namespace) => self.start_namespace(namespace),
            Node::Use(r#use) => {
                self.uses(r#use);
                // the imported names are no `Name` nodes Rector reads
                return;
            }
            Node::Class(class) => {
                let name = self.class_like(node_span(class).start, &class.name, true);
                self.classes.push(name);
                pushed_class = true;
                if let Some(extends) = &class.extends {
                    for id in extends.types.iter() {
                        self.name(id, Kind::Class).extends = true;
                    }
                }
                if let Some(implements) = &class.implements {
                    self.implements(implements, true);
                }
            }
            Node::AnonymousClass(class) => {
                self.classes.push(String::new());
                pushed_class = true;
                if let Some(extends) = &class.extends {
                    for id in extends.types.iter() {
                        self.name(id, Kind::Class).extends = true;
                    }
                }
                if let Some(implements) = &class.implements {
                    for id in implements.types.iter() {
                        self.name(id, Kind::Class).implements = true;
                    }
                }
            }
            Node::Interface(interface) => {
                let name = self.class_like(node_span(interface).start, &interface.name, false);
                self.classes.push(name);
                pushed_class = true;
                if let Some(extends) = &interface.extends {
                    for id in extends.types.iter() {
                        self.name(id, Kind::Class);
                    }
                }
            }
            Node::Trait(r#trait) => {
                let name = self.class_like(node_span(r#trait).start, &r#trait.name, false);
                self.classes.push(name);
                pushed_class = true;
            }
            Node::Enum(r#enum) => {
                let name = self.class_like(node_span(r#enum).start, &r#enum.name, false);
                self.classes.push(name);
                pushed_class = true;
                if let Some(implements) = &r#enum.implements {
                    self.implements(implements, false);
                }
            }
            Node::Inline(inline) if is_inline_html(inline) => self.view.inline_html = true,
            Node::Attribute(attribute) => {
                self.name(&attribute.name, Kind::Class);
            }
            Node::Hint(Hint::Identifier(id)) => {
                self.name(id, Kind::Class);
            }
            Node::TraitUse(r#use) => {
                for id in r#use.trait_names.iter() {
                    self.name(id, Kind::Class);
                }
            }
            Node::TraitUseAbsoluteMethodReference(reference) => {
                self.name(&reference.trait_name, Kind::Class);
            }
            // (Mago gives the names of a `TraitUseAdaptation`'s precedence, never the precedence node itself)
            Node::TraitUseAdaptation(mago_syntax::cst::TraitUseAdaptation::Precedence(adaptation)) => {
                for id in adaptation.trait_names.iter() {
                    self.name(id, Kind::Class);
                }
            }
            Node::Instantiation(new) => {
                if let Some(name) = self.class_expression(new.class) {
                    name.new = true;
                }
            }
            Node::StaticMethodCall(call) => {
                if let Some(name) = self.class_expression(call.class) {
                    name.static_call = true;
                }
            }
            Node::StaticMethodPartialApplication(call) => {
                if let Some(name) = self.class_expression(call.class) {
                    name.static_call = true;
                }
            }
            Node::StaticPropertyAccess(access) => {
                self.class_expression(access.class);
            }
            Node::ClassConstantAccess(access) => {
                let constant = match &access.constant {
                    ClassLikeConstantSelector::Identifier(id) => {
                        Some(String::from_utf8_lossy(id.value).into_owned())
                    }
                    _ => None,
                };
                if let Some(name) = self.class_expression(access.class) {
                    name.class_const_fetch = constant;
                }
            }
            Node::Binary(binary) if binary.operator.is_instanceof() => {
                self.class_expression(binary.rhs);
            }
            Node::FunctionCall(call) => {
                if let Expression::Identifier(id) = call.function {
                    self.name(id, Kind::Function);
                }
            }
            Node::FunctionPartialApplication(call) => {
                if let Expression::Identifier(id) = call.function {
                    self.name(id, Kind::Function);
                }
            }
            Node::ConstantAccess(access) => {
                self.name(&access.name, Kind::Constant);
            }
            _ => {}
        }
        self.doc(&node);
        for child in node.children() {
            self.node(child);
        }
        if pushed_class {
            self.classes.pop();
        }
    }
}

/// The statements of a list as php-parser keeps them: without the open and close tags, `;` and the line break Mago
/// reads as inline HTML after `?>`; with a `Nop` for comments after the last one (up to `inner.end`).
fn stmt_list<'a, 'b: 'a>(
    source: &str,
    all: &[crate::rules::php_parser::Token],
    statements: impl Iterator<Item = &'a Statement<'b>>,
    inner: Span,
) -> Vec<StmtView> {
    let mut list = Vec::new();
    let mut uses = 0;
    for statement in statements {
        let kind = match statement {
            Statement::OpeningTag(_) | Statement::ClosingTag(_) | Statement::Noop(_) => continue,
            Statement::Inline(inline) if !is_inline_html(inline) => continue,
            Statement::Use(_) => {
                uses += 1;
                StmtKind::Use(uses - 1)
            }
            Statement::Declare(_) => StmtKind::Declare,
            Statement::Namespace(_) => StmtKind::Namespace,
            Statement::Inline(_) => StmtKind::Inline,
            _ => StmtKind::Other,
        };
        let span = node_span(statement);
        // `InlineHTML` takes no comments (and may come before any open tag)
        let comment_start = match statement {
            Statement::Inline(_) => span.start,
            _ => {
                let before = crate::rules::php_parser::previous_token_end(source, all, span.start);
                all.iter()
                    .find(|t| t.start >= before && t.end <= span.start && t.kind.is_comment())
                    .map_or(span.start, |t| t.start)
            }
        };
        list.push(StmtView {
            span,
            comment_start,
            kind,
        });
    }
    let after = list.last().map_or(inner.start, |s| s.span.end);
    if crate::rules::php_parser::trailing_comment(all, after, inner.end) {
        let comments: Vec<_> = all
            .iter()
            .filter(|t| t.start >= after && t.end <= inner.end && t.kind.is_comment())
            .collect();
        let end = comments.last().unwrap().end;
        list.push(StmtView {
            span: Span::new(end, end),
            comment_start: comments[0].start,
            kind: StmtKind::Nop,
        });
    }
    list
}

/// Whether Mago's inline statement is php-parser's `InlineHTML`: not the line break after `?>`, which php-parser's
/// close tag takes.
fn is_inline_html(inline: &Inline<'_>) -> bool {
    !(matches!(inline.value, b"\n" | b"\r\n") && inline.span.start.offset > 0)
}
