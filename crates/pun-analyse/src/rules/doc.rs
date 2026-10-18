//! PHPStan's reading of PHPDocs: a doc comment's tags (phpdoc-parser's PhpDocParser, for the tag values PHPStan
//! resolves), the `NameScope` a PHPDoc is resolved in (FileTypeMapper) and the resolved tags (PhpDocNodeResolver).

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use mago_allocator::LocalArena;
use mago_database::file::{File, FileId};
use mago_names::kind::NameKind;
use mago_names::scope::NamespaceScope;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::parser::parse_file;
use pun_phpstan::phpdoc::{self, Node};
use pun_phpstan::resolve::{Scope, resolve_in};
use pun_phpstan::types::Type;

/// Each tag of a doc comment and its value: the rest of its line and the lines after it up to the next tag, joined
/// with a space. A tag starts a line (after the leading `*`).
pub fn tags(doc: &[u8]) -> Vec<(String, String)> {
    let body = doc.strip_prefix(b"/**").unwrap_or(doc);
    let body = body.strip_suffix(b"*/").unwrap_or(body);
    let mut tags: Vec<(String, String)> = Vec::new();
    for line in String::from_utf8_lossy(body).lines() {
        let line = line.trim_start();
        let line = line.strip_prefix('*').unwrap_or(line).trim();
        if let Some(rest) = line.strip_prefix('@') {
            let len = rest
                .find(|ch: char| {
                    !(ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '\\' | ':'))
                })
                .unwrap_or(rest.len());
            tags.push((format!("@{}", &rest[..len]), rest[len..].trim().to_string()));
        } else if let Some((_, text)) = tags.last_mut()
            && !line.is_empty()
        {
            text.push(' ');
            text.push_str(line);
        }
    }
    tags
}

/// A `@param` value: the type, `...`, and the parameter's name without `$` (`parseParamTagValue()`; a typeless one
/// is not a ParamTagValueNode).
fn param_value(value: &str) -> Option<(Node, bool, String)> {
    if value.starts_with(['$', '&']) || value.starts_with("...") {
        return None;
    }
    let (node, offset) = phpdoc::parse_prefix(value).ok()?;
    let mut rest = value[offset..].trim_start();
    rest = rest.strip_prefix('&').unwrap_or(rest).trim_start();
    let variadic = rest.starts_with("...");
    if variadic {
        rest = rest[3..].trim_start();
    }
    let name = rest.strip_prefix('$')?;
    let len = name
        .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_' || !ch.is_ascii()))
        .unwrap_or(name.len());
    if len == 0 || name.as_bytes()[0].is_ascii_digit() {
        return None;
    }
    Some((node, variadic, name[..len].to_string()))
}

/// A type-first tag value (`@return`, `@var`, `@throws`, `@extends`, ...): the type and the rest.
fn type_value(value: &str) -> Option<(Node, &str)> {
    let (node, offset) = phpdoc::parse_prefix(value).ok()?;
    Some((node, value[offset..].trim_start()))
}

/// `@template` (with its variants) as written.
pub struct TemplateTag {
    pub name: String,
    pub bound: Option<Node>,
    pub default: Option<Node>,
}

const PARAM_TAGS: [&str; 4] = ["@param", "@phan-param", "@psalm-param", "@phpstan-param"];
const RETURN_TAGS: [&str; 5] = [
    "@return",
    "@phan-return",
    "@phan-real-return",
    "@psalm-return",
    "@phpstan-return",
];
const OUT_TAGS: [&str; 3] = ["@param-out", "@psalm-param-out", "@phpstan-param-out"];
const CLOSURE_THIS_TAGS: [&str; 2] = ["@param-closure-this", "@phpstan-param-closure-this"];
const SELF_OUT_TAGS: [&str; 4] = [
    "@phpstan-this-out",
    "@phpstan-self-out",
    "@psalm-this-out",
    "@psalm-self-out",
];
const VAR_TAGS: [&str; 4] = ["@var", "@phan-var", "@psalm-var", "@phpstan-var"];
const TEMPLATE_TAGS: [&str; 4] = [
    "@template",
    "@phan-template",
    "@psalm-template",
    "@phpstan-template",
];

/// The tags of one doc comment that PHPStan resolves to types, in PhpDocNodeResolver's order (a later tag of the
/// same kind wins, except a `@psalm-` one whose type is unresolvable).
#[derive(Default)]
pub struct Doc {
    /// (tag, parameter name, type, variadic)
    pub params: Vec<(String, String, Node, bool)>,
    /// (tag, type)
    pub returns: Vec<(String, Node)>,
    /// (tag, variable name or empty, type)
    pub vars: Vec<(String, String, Node)>,
    /// `@param-out` and `@param-closure-this`: (tag, parameter name, type)
    pub outs: Vec<(String, String, Node)>,
    pub closure_this: Vec<(String, String, Node)>,
    /// `resolveSelfOutTypeTag()`: the first tag of the first kind that has one
    pub self_out: Option<Node>,
    pub templates: Vec<TemplateTag>,
}

impl Doc {
    pub fn parse(doc: &[u8]) -> Doc {
        let mut out = Doc::default();
        let tags = tags(doc);
        let order = |list: &[&str], tag: &str| list.iter().position(|t| *t == tag);
        let mut params = Vec::new();
        let mut returns = Vec::new();
        let mut vars = Vec::new();
        let mut outs = Vec::new();
        let mut closure_this = Vec::new();
        let mut self_out: Option<(usize, Node)> = None;
        for (tag, value) in &tags {
            if let Some(rank) = order(&PARAM_TAGS, tag) {
                if let Some((node, variadic, name)) = param_value(value) {
                    params.push((rank, (tag.clone(), name, node, variadic)));
                }
            } else if let Some(rank) = order(&OUT_TAGS, tag) {
                if let Some((node, _, name)) = param_value(value) {
                    outs.push((rank, (tag.clone(), name, node)));
                }
            } else if let Some(rank) = order(&SELF_OUT_TAGS, tag) {
                if let Some((node, _)) = type_value(value)
                    && self_out.as_ref().is_none_or(|(r, _)| rank < *r)
                {
                    self_out = Some((rank, node));
                }
            } else if let Some(rank) = order(&CLOSURE_THIS_TAGS, tag) {
                if let Some((node, _, name)) = param_value(value) {
                    closure_this.push((rank, (tag.clone(), name, node)));
                }
            } else if let Some(rank) = order(&RETURN_TAGS, tag) {
                if let Some((node, _)) = type_value(value) {
                    returns.push((rank, (tag.clone(), node)));
                }
            } else if let Some(rank) = order(&VAR_TAGS, tag) {
                if let Some((node, rest)) = type_value(value) {
                    let name = rest
                        .strip_prefix('$')
                        .map(|n| {
                            n.split(|ch: char| {
                                !(ch.is_ascii_alphanumeric() || ch == '_' || !ch.is_ascii())
                            })
                            .next()
                            .unwrap_or("")
                            .to_string()
                        })
                        .unwrap_or_default();
                    vars.push((rank, (tag.clone(), name, node)));
                }
            } else {
                let base = tag
                    .strip_suffix("-covariant")
                    .or_else(|| tag.strip_suffix("-contravariant"))
                    .unwrap_or(tag);
                if TEMPLATE_TAGS.contains(&base)
                    && let Some(t) = template_value(value)
                {
                    out.templates.push(t);
                }
            }
        }
        // stable: the tag order within one prefix stays
        params.sort_by_key(|(rank, _)| *rank);
        returns.sort_by_key(|(rank, _)| *rank);
        vars.sort_by_key(|(rank, _)| *rank);
        outs.sort_by_key(|(rank, _)| *rank);
        closure_this.sort_by_key(|(rank, _)| *rank);
        out.self_out = self_out.map(|(_, n)| n);
        out.outs = outs.into_iter().map(|(_, p)| p).collect();
        out.closure_this = closure_this.into_iter().map(|(_, p)| p).collect();
        out.params = params.into_iter().map(|(_, p)| p).collect();
        out.returns = returns.into_iter().map(|(_, r)| r).collect();
        out.vars = vars.into_iter().map(|(_, v)| v).collect();
        out
    }

    /// `resolveParamTags()`: each parameter's type, by name.
    pub fn param_types(&self, scope: &NameScope) -> HashMap<String, (Type, bool)> {
        let mut out = HashMap::new();
        for (tag, name, node, variadic) in &self.params {
            let ty = resolve_in(node, scope);
            if skip(tag, &ty) {
                continue;
            }
            out.insert(name.clone(), (ty, *variadic));
        }
        out
    }

    /// `resolveParamOutTags()` or `resolveParamClosureThisTags()` of `tags` ([`Doc::outs`], [`Doc::closure_this`]).
    pub fn named_types(
        tags: &[(String, String, Node)],
        scope: &NameScope,
    ) -> HashMap<String, Type> {
        let mut out = HashMap::new();
        for (tag, name, node) in tags {
            let ty = resolve_in(node, scope);
            if skip(tag, &ty) {
                continue;
            }
            out.insert(name.clone(), ty);
        }
        out
    }

    /// `resolveReturnTag()`.
    pub fn return_type(&self, scope: &NameScope) -> Option<Type> {
        let mut out = None;
        for (tag, node) in &self.returns {
            let ty = resolve_in(node, scope);
            if skip(tag, &ty) {
                continue;
            }
            out = Some(ty);
        }
        out
    }

    /// `resolveVarTags()`: the named ones by name, else the unnamed ones of the last tag kind that has some (under
    /// the empty name).
    pub fn var_types(&self, scope: &NameScope) -> Vec<(String, Type)> {
        let mut named: Vec<(String, Type)> = Vec::new();
        let mut unnamed: Vec<(usize, Type)> = Vec::new();
        for (tag, name, node) in &self.vars {
            let ty = resolve_in(node, scope);
            if skip(tag, &ty) {
                continue;
            }
            let rank = VAR_TAGS.iter().position(|t| t == tag).unwrap_or(0);
            if name.is_empty() {
                unnamed.push((rank, ty));
            } else if let Some(entry) = named.iter_mut().find(|(n, _)| n == name) {
                entry.1 = ty;
            } else {
                named.push((name.clone(), ty));
            }
        }
        if let Some(last) = unnamed.iter().map(|(rank, _)| *rank).max() {
            return unnamed
                .into_iter()
                .filter(|(rank, _)| *rank == last)
                .map(|(_, ty)| (String::new(), ty))
                .collect();
        }
        named
    }
}

/// `shouldSkipType()`: a `@psalm-` tag whose type PHPStan cannot resolve.
fn skip(tag: &str, ty: &Type) -> bool {
    tag.starts_with("@psalm-") && contains_error(ty)
}

fn contains_error(ty: &Type) -> bool {
    let found = std::cell::Cell::new(false);
    fn walk(t: &Type, found: &std::cell::Cell<bool>) {
        if matches!(t, Type::Error) {
            found.set(true);
        }
        t.traverse(&|inner| {
            walk(inner, found);
            inner.clone()
        });
    }
    walk(ty, &found);
    found.get()
}

/// `parseTemplateTagValue()`: `T`, `T of Bound`, `T as Bound`, `T = Default`.
fn template_value(value: &str) -> Option<TemplateTag> {
    let len = value
        .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '\\' || !ch.is_ascii()))
        .unwrap_or(value.len());
    if len == 0 {
        return None;
    }
    let name = value[..len].to_string();
    let mut rest = value[len..].trim_start();
    let mut bound = None;
    if let Some(b) = rest
        .strip_prefix("of ")
        .or_else(|| rest.strip_prefix("as "))
    {
        let (node, after) = type_value(b.trim_start())?;
        bound = Some(node);
        rest = after;
    }
    let default = match rest.strip_prefix('=') {
        Some(d) => Some(type_value(d.trim_start())?.0),
        None => None,
    };
    Some(TemplateTag {
        name,
        bound,
        default,
    })
}

/// PHPStan's `NameScope`: the namespace and `use` imports of the place a PHPDoc is, the class `self` means, and the
/// template types in scope.
#[derive(Clone, Default)]
pub struct NameScope {
    pub names: NamespaceScope,
    pub class: Option<String>,
    pub parent: Option<String>,
    pub templates: HashMap<String, Type>,
}

impl Scope for NameScope {
    fn class(&self) -> Option<&str> {
        self.class.as_deref()
    }
    fn parent(&self) -> Option<String> {
        self.parent.clone()
    }
    fn resolve_name(&self, name: &str) -> String {
        let (resolved, _) = self.names.resolve(NameKind::Default, name.as_bytes());
        String::from_utf8_lossy(&resolved).into_owned()
    }
    fn template(&self, name: &str) -> Option<Type> {
        self.templates.get(name).cloned()
    }
}

/// The namespaces of a file with their imports (FileTypeMapper's walk), by byte range.
pub struct FileScopes {
    namespaces: Vec<(u32, u32, NamespaceScope)>,
}

impl FileScopes {
    pub fn of(program: &Program) -> FileScopes {
        let mut namespaces = Vec::new();
        let mut global = NamespaceScope::global();
        let mut any = false;
        for s in program.statements.iter() {
            match s {
                Statement::Namespace(n) => {
                    any = true;
                    let name = n.name.as_ref().map(|id| id.value().to_vec());
                    let mut scope = NamespaceScope::new(name);
                    for inner in n.statements().iter() {
                        if let Statement::Use(u) = inner {
                            scope.populate_from_use(u);
                        }
                    }
                    let span = n.span();
                    namespaces.push((span.start.offset, span.end.offset, scope));
                }
                Statement::Use(u) => global.populate_from_use(u),
                _ => {}
            }
        }
        if !any {
            namespaces.push((0, u32::MAX, global));
        }
        FileScopes { namespaces }
    }

    /// The namespace scope at `offset`.
    pub fn at(&self, offset: u32) -> NamespaceScope {
        self.namespaces
            .iter()
            .find(|(start, end, _)| *start <= offset && offset <= *end)
            .map(|(_, _, s)| s.clone())
            .unwrap_or_default()
    }
}

/// [`FileScopes`] of files other than the analysed one, parsed once per content.
pub fn scopes_of(id: FileId, contents: &[u8]) -> Arc<FileScopes> {
    static CACHE: OnceLock<Mutex<HashMap<(FileId, u64), Arc<FileScopes>>>> = OnceLock::new();
    let key = (id, fingerprint(contents));
    let cache = CACHE.get_or_init(Default::default);
    if let Some(found) = cache.lock().expect("scopes cache").get(&key) {
        return found.clone();
    }
    let arena = LocalArena::new();
    let file = File::ephemeral(
        std::borrow::Cow::Borrowed(b"scopes.php".as_slice()),
        std::borrow::Cow::Owned(contents.to_vec()),
    );
    let program = parse_file(&arena, &file);
    let scopes = Arc::new(FileScopes::of(program));
    cache
        .lock()
        .expect("scopes cache")
        .insert(key, scopes.clone());
    scopes
}

fn fingerprint(contents: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    contents.hash(&mut h);
    h.finish()
}
