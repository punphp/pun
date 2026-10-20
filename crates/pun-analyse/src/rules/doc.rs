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

/// The doc comment (`/** ...`) that ends right before `offset`, skipping other comments in between: the one PHP's
/// tokenizer reads (the start is the last `/**` before its `*/`, so `/*` inside the text is no start).
pub fn doc_before(contents: &[u8], offset: usize) -> Option<&[u8]> {
    let mut end = offset.min(contents.len());
    loop {
        end -= contents[..end]
            .iter()
            .rev()
            .take_while(|b| b.is_ascii_whitespace())
            .count();
        if !contents[..end].ends_with(b"*/") || end < 4 {
            return None;
        }
        let start = contents[..end - 2].windows(3).rposition(|w| w == b"/**");
        let plain = contents[..end - 2].windows(2).rposition(|w| w == b"/*");
        match start {
            Some(start)
                if contents.get(start + 3).is_some_and(u8::is_ascii_whitespace)
                    && !contents[start + 3..end - 2].windows(2).any(|w| w == b"*/") =>
            {
                return Some(&contents[start..end]);
            }
            _ => end = plain?,
        }
    }
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

/// A `@method` tag as written (`parseMethodTagValue()`).
pub struct MethodTag {
    pub name: String,
    pub ret: Option<Node>,
    /// (name without `$`, type, whether the default is `null`)
    pub params: Vec<(String, Option<Node>, bool)>,
    pub templates: Vec<TemplateTag>,
}

/// Splits `s` at the commas outside brackets and quotes.
fn split_top(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let (mut depth, mut quote, mut start) = (0i32, None, 0);
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        match quote {
            Some(_) if b == b'\\' => i += 1,
            Some(q) if b == q => quote = None,
            Some(_) => {}
            None => match b {
                b'\'' | b'"' => quote = Some(b),
                b'(' | b'<' | b'[' | b'{' => depth += 1,
                b')' | b'>' | b']' | b'}' => depth -= 1,
                b',' if depth == 0 => {
                    out.push(&s[start..i]);
                    start = i + 1;
                }
                _ => {}
            },
        }
        i += 1;
    }
    out.push(&s[start..]);
    out
}

/// The leading identifier of `s` (a method or class name).
fn identifier(s: &str) -> &str {
    let len = s
        .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '\\' || !ch.is_ascii()))
        .unwrap_or(s.len());
    &s[..len]
}

fn method_value(value: &str) -> Option<MethodTag> {
    let mut rest = value.trim_start();
    let mut is_static = false;
    if let Some(after) = rest.strip_prefix("static")
        && after.starts_with(char::is_whitespace)
    {
        is_static = true;
        rest = after.trim_start();
    }
    let first = identifier(rest);
    let after_first = rest[first.len()..].trim_start();
    let (ret, name, after) =
        if !first.is_empty() && (after_first.starts_with('(') || after_first.starts_with('<')) {
            // no return type: `static` alone was the return type
            let ret = if is_static {
                Some(phpdoc::parse_prefix("static").ok()?.0)
            } else {
                None
            };
            (ret, first, after_first)
        } else {
            let (node, offset) = phpdoc::parse_prefix(rest).ok()?;
            let after = rest[offset..].trim_start();
            let name = identifier(after);
            if name.is_empty() {
                return None;
            }
            (Some(node), name, after[name.len()..].trim_start())
        };
    let mut after = after;
    let mut templates = Vec::new();
    if let Some(inner) = after.strip_prefix('<') {
        let close = inner.find('>')?;
        for t in split_top(&inner[..close]) {
            templates.push(template_value(t.trim())?);
        }
        after = inner[close + 1..].trim_start();
    }
    let inner = after.strip_prefix('(')?;
    let mut depth = 1;
    let close = inner.char_indices().find_map(|(i, ch)| {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        (depth == 0).then_some(i)
    })?;
    let mut params = Vec::new();
    for param in split_top(&inner[..close]) {
        let param = param.trim();
        if param.is_empty() {
            continue;
        }
        let (decl, default) = match param.split_once('=') {
            Some((d, v)) => (d.trim_end(), Some(v.trim())),
            None => (param, None),
        };
        let at = decl.rfind('$')?;
        let pname = identifier(&decl[at + 1..]).to_string();
        let mut ty = decl[..at].trim_end();
        ty = ty.strip_suffix("...").unwrap_or(ty).trim_end();
        ty = ty.strip_suffix('&').unwrap_or(ty).trim_end();
        let node = if ty.is_empty() {
            None
        } else {
            Some(phpdoc::parse(ty).ok()?)
        };
        let null_default = default.is_some_and(|d| d.eq_ignore_ascii_case("null"));
        params.push((pname, node, null_default));
    }
    Some(MethodTag {
        name: name.to_string(),
        ret,
        params,
        templates,
    })
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
const METHOD_TAGS: [&str; 4] = [
    "@method",
    "@phan-method",
    "@psalm-method",
    "@phpstan-method",
];
const PROPERTY_TAGS: [&str; 2] = ["@property", "@phpstan-property"];
const PROPERTY_READ_TAGS: [&str; 2] = ["@property-read", "@phpstan-property-read"];
const PROPERTY_WRITE_TAGS: [&str; 2] = ["@property-write", "@phpstan-property-write"];

/// `resolveExtendsTags()` and `resolveImplementsTags()`
const ANCESTOR_TAGS: [&str; 8] = [
    "@extends",
    "@phan-extends",
    "@phan-inherits",
    "@template-extends",
    "@phpstan-extends",
    "@implements",
    "@template-implements",
    "@phpstan-implements",
];
const USE_TAGS: [&str; 3] = ["@use", "@template-use", "@phpstan-use"];
const TYPE_ALIAS_TAGS: [&str; 3] = ["@phan-type", "@psalm-type", "@phpstan-type"];
const TYPE_ALIAS_IMPORT_TAGS: [&str; 2] = ["@psalm-import-type", "@phpstan-import-type"];
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
    /// `@extends` and `@implements` types, in tag order (a later one for the same class wins)
    pub ancestors: Vec<Node>,
    /// `resolveMethodTags()`: by name in first-seen order, a later tag replacing an earlier one
    pub methods: Vec<MethodTag>,
    /// `resolvePropertyTags()`: (name, readable type, writable type)
    pub properties: Vec<(String, Option<Node>, Option<Node>)>,
    /// `@mixin` types
    pub mixins: Vec<Node>,
    /// `resolveUsesTags()`: `@use` types, in tag-kind order (a later one for the same trait wins)
    pub uses: Vec<Node>,
    /// `resolveExtendsTags()` and `resolveImplementsTags()` apart, in tag-kind order
    pub extends: Vec<Node>,
    pub implements: Vec<Node>,
    pub templates: Vec<TemplateTag>,
    /// each template's variance (`invariant`, `covariant` or `contravariant`, from the tag name)
    pub template_variances: Vec<&'static str>,
    /// `resolveTypeAliasTags()`: the names of `@phpstan-type` & co.
    pub type_aliases: Vec<String>,
    /// `resolveTypeAliasImportTags()`: (imported alias, class as written, `as` name)
    pub alias_imports: Vec<(String, String, Option<String>)>,
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
        let mut ancestors = Vec::new();
        let mut uses = Vec::new();
        let mut methods: Vec<(usize, MethodTag)> = Vec::new();
        let mut properties: Vec<(usize, String, Node)> = Vec::new();
        for (tag, value) in &tags {
            if let Some(rank) = order(&PARAM_TAGS, tag) {
                if let Some((node, variadic, name)) = param_value(value) {
                    params.push((rank, (tag.clone(), name, node, variadic)));
                }
            } else if let Some(rank) = order(&OUT_TAGS, tag) {
                if let Some((node, _, name)) = param_value(value) {
                    outs.push((rank, (tag.clone(), name, node)));
                }
            } else if let Some(rank) = order(&METHOD_TAGS, tag) {
                if let Some(m) = method_value(value) {
                    methods.push((rank, m));
                }
            } else if let Some(rank) = order(&PROPERTY_TAGS, tag)
                .or_else(|| order(&PROPERTY_READ_TAGS, tag).map(|r| 10 + r))
                .or_else(|| order(&PROPERTY_WRITE_TAGS, tag).map(|r| 20 + r))
            {
                if let Some((node, rest)) = type_value(value)
                    && let Some(name) = rest.strip_prefix('$')
                {
                    let name = identifier(name);
                    if !name.is_empty() {
                        properties.push((rank, name.to_string(), node));
                    }
                }
            } else if tag == "@mixin" {
                if let Some((node, _)) = type_value(value) {
                    out.mixins.push(node);
                }
            } else if let Some(rank) = order(&ANCESTOR_TAGS, tag) {
                if let Some((node, _)) = type_value(value) {
                    ancestors.push((rank, node));
                }
            } else if TYPE_ALIAS_TAGS.contains(&tag.as_str()) {
                let name = identifier(value);
                if !name.is_empty() {
                    out.type_aliases.push(name.to_string());
                }
            } else if TYPE_ALIAS_IMPORT_TAGS.contains(&tag.as_str()) {
                if let Some(import) = alias_import(value) {
                    out.alias_imports.push(import);
                }
            } else if let Some(rank) = order(&USE_TAGS, tag) {
                if let Some((node, _)) = type_value(value) {
                    uses.push((rank, node));
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
                // PhpDocParser expects whitespace or the end after the type: `@return Foo(` is invalid
                // ponytail: for `@return` only; the other tags still take the type prefix
                if let Ok((node, offset)) = phpdoc::parse_prefix(value)
                    && !value[offset..].starts_with(|ch: char| !ch.is_whitespace())
                {
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
                    out.template_variances.push(match &tag[base.len()..] {
                        "-covariant" => "covariant",
                        "-contravariant" => "contravariant",
                        _ => "invariant",
                    });
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
        ancestors.sort_by_key(|(rank, _)| *rank);
        for (rank, node) in &ancestors {
            match rank {
                0..5 => out.extends.push(node.clone()),
                _ => out.implements.push(node.clone()),
            }
        }
        methods.sort_by_key(|(rank, _)| *rank);
        for (_, m) in methods {
            match out.methods.iter_mut().find(|x| x.name == m.name) {
                Some(slot) => *slot = m,
                None => out.methods.push(m),
            }
        }
        properties.sort_by_key(|(rank, _, _)| *rank);
        for (rank, name, node) in properties {
            let slot = match out.properties.iter().position(|(n, _, _)| *n == name) {
                Some(i) => i,
                None => {
                    out.properties.push((name, None, None));
                    out.properties.len() - 1
                }
            };
            let entry = &mut out.properties[slot];
            match rank / 10 {
                0 => {
                    entry.1 = Some(node.clone());
                    entry.2 = Some(node);
                }
                1 => entry.1 = Some(node),
                _ => entry.2 = Some(node),
            }
        }
        out.ancestors = ancestors.into_iter().map(|(_, n)| n).collect();
        uses.sort_by_key(|(rank, _)| *rank);
        out.uses = uses.into_iter().map(|(_, n)| n).collect();
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

/// `parseTypeAliasImportTagValue()`: `Alias from Class` with an optional `as Name`.
fn alias_import(value: &str) -> Option<(String, String, Option<String>)> {
    let alias = identifier(value);
    let rest = value[alias.len()..].trim_start().strip_prefix("from")?;
    if alias.is_empty() || !rest.starts_with(char::is_whitespace) {
        return None;
    }
    let rest = rest.trim_start();
    let len = rest
        .find(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '\\' || !ch.is_ascii()))
        .unwrap_or(rest.len());
    if len == 0 {
        return None;
    }
    let from = rest[..len].to_string();
    let rest = rest[len..].trim_start();
    let alias_as = rest
        .strip_prefix("as")
        .filter(|r| r.starts_with(char::is_whitespace))
        .map(|r| identifier(r.trim_start()).to_string())
        .filter(|n| !n.is_empty());
    Some((alias.to_string(), from, alias_as))
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
    fn constant_names(&self, name: &str) -> Vec<String> {
        if let Some(alias) = self
            .names
            .resolve_alias(NameKind::Constant, name.as_bytes())
        {
            return vec![String::from_utf8_lossy(&alias).into_owned()];
        }
        match self.names.namespace_name() {
            Some(namespace) => vec![
                format!("{}\\{name}", String::from_utf8_lossy(namespace)),
                name.to_string(),
            ],
            None => vec![name.to_string()],
        }
    }
    fn template(&self, name: &str) -> Option<Type> {
        self.templates.get(name).cloned()
    }
    fn has_use_alias(&self, name: &str) -> bool {
        self.names
            .resolve_alias(NameKind::Default, name.as_bytes())
            .is_some()
    }
    fn in_namespace(&self) -> bool {
        self.names.namespace_name().is_some()
    }
}

/// The namespaces of a file with their imports (FileTypeMapper's walk), by byte range.
pub struct FileScopes {
    namespaces: Vec<(u32, u32, NamespaceScope)>,
    /// each `use Trait, ...;` in a class body: its offset and the trait names as written
    trait_uses: Vec<(u32, Vec<String>)>,
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
        let mut trait_uses = Vec::new();
        fn walk(node: mago_syntax::cst::Node<'_, '_>, out: &mut Vec<(u32, Vec<String>)>) {
            if let mago_syntax::cst::Node::TraitUse(t) = node {
                let names = t
                    .trait_names
                    .iter()
                    .map(|n| String::from_utf8_lossy(n.value()).into_owned())
                    .collect();
                out.push((t.span().start.offset, names));
            }
            for child in node.children() {
                walk(child, out);
            }
        }
        walk(mago_syntax::cst::Node::Program(program), &mut trait_uses);
        FileScopes {
            namespaces,
            trait_uses,
        }
    }

    /// The `use` statements of traits between `start` and `end`: offset and written names.
    pub fn trait_uses(&self, start: u32, end: u32) -> impl Iterator<Item = &(u32, Vec<String>)> {
        self.trait_uses
            .iter()
            .filter(move |(at, _)| start <= *at && *at <= end)
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
