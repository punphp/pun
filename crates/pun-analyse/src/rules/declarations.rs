//! Class-member declaration rules that read no inferred types: DuplicateDeclarationRule (with
//! DuplicateDeclarationHelper), AbstractPrivateMethodRule, FinalPrivateMethodRule, FinalPrivateConstantRule and
//! InvalidPromotedPropertiesRule, plus what they read of the class being declared (`InClassNode`'s ClassReflection)
//! and of PHPDoc tags.

use std::collections::HashSet;

use mago_codex::symbol::SymbolKind;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::Checker;
use super::reflection::display;

/// The class-like a member is declared in, as its ClassReflection answers (in a trait walked in a class's context,
/// that class).
pub struct Decl {
    /// `getDisplayName()`, `None` for an anonymous class (its `class@anonymous/file:line` name is not ported, so the
    /// rules that print it skip anonymous classes).
    pub display: Option<String>,
    pub kind: SymbolKind,
    pub is_abstract: bool,
    pub is_readonly: bool,
    /// `isImmutable()` from the declaration's own PHPDoc (or, in a trait's context, the class's whole answer).
    pub immutable_doc: bool,
    /// The parent class, for `isImmutable()` and a property's `isVirtual()`.
    pub parent: Option<String>,
}

/// The tag names of a PHPDoc (`/** ... */`): phpdoc-parser's tags start a line, after the optional leading `*`.
pub fn doc_tags(doc: &[u8]) -> Vec<String> {
    let body = doc.strip_prefix(b"/**").unwrap_or(doc);
    let body = body.strip_suffix(b"*/").unwrap_or(body);
    let mut tags = Vec::new();
    for line in body.split(|b| *b == b'\n') {
        let line = line.trim_ascii_start();
        let line = match line.strip_prefix(b"*") {
            Some(rest) if !rest.starts_with(b"/") => rest.trim_ascii_start(),
            _ => line,
        };
        let Some(rest) = line.strip_prefix(b"@") else {
            continue;
        };
        let len = rest
            .iter()
            .take_while(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'\\' | b':'))
            .count();
        if rest.first().is_some_and(u8::is_ascii_alphabetic) {
            tags.push(format!("@{}", String::from_utf8_lossy(&rest[..len])));
        }
    }
    tags
}

/// php-parser's `getDocComment()`: the last `/** */` comment before `offset` with only whitespace and other block
/// comments between.
pub fn doc_before(contents: &[u8], offset: usize) -> Option<&[u8]> {
    let mut end = offset.min(contents.len());
    loop {
        end -= contents[..end]
            .iter()
            .rev()
            .take_while(|b| b.is_ascii_whitespace())
            .count();
        if !contents[..end].ends_with(b"*/") {
            return None;
        }
        let start = contents[..end - 2].windows(2).rposition(|w| w == b"/*")?;
        let comment = &contents[start..end];
        // PHP's T_DOC_COMMENT: `/**` and whitespace
        if comment.starts_with(b"/**") && comment.get(3).is_some_and(u8::is_ascii_whitespace) {
            return Some(comment);
        }
        end = start;
    }
}

/// Whether a PHPDoc has one of `names`.
pub fn has_tag(tags: &[String], names: &[&str]) -> bool {
    tags.iter().any(|t| names.contains(&t.as_str()))
}

pub const READONLY_TAGS: &[&str] = &[
    "@readonly",
    "@phan-read-only",
    "@psalm-readonly",
    "@phpstan-readonly",
    "@phpstan-readonly-allow-private-mutation",
    "@psalm-readonly-allow-private-mutation",
];
const IMMUTABLE_TAGS: &[&str] = &[
    "@immutable",
    "@phan-immutable",
    "@psalm-immutable",
    "@phpstan-immutable",
];

/// ClassReflection::isImmutable() from a class's own PHPDoc: `@immutable` or `@readonly` tags.
pub fn immutable_tags(tags: &[String]) -> bool {
    has_tag(tags, IMMUTABLE_TAGS) || has_tag(tags, READONLY_TAGS)
}

fn has(modifiers: &[Modifier], f: impl Fn(&Modifier) -> bool) -> bool {
    modifiers.iter().any(f)
}

pub fn is_abstract(m: &Modifier) -> bool {
    matches!(m, Modifier::Abstract(_))
}
pub fn is_final(m: &Modifier) -> bool {
    matches!(m, Modifier::Final(_))
}
pub fn is_private(m: &Modifier) -> bool {
    matches!(m, Modifier::Private(_))
}

impl<'arena> Checker<'_, 'arena> {
    /// The tags of the PHPDoc right before `node` in this file.
    pub(super) fn tags_before(&self, node: impl HasSpan) -> Vec<String> {
        doc_before(&self.file.contents, node.span().start.offset as usize)
            .map(doc_tags)
            .unwrap_or_default()
    }

    /// The class-like declared here, read from its declaration.
    pub(super) fn decl_of(
        &self,
        at: Span,
        name: Option<Span>,
        kind: SymbolKind,
        modifiers: &[Modifier],
        extends: Option<&Extends<'arena>>,
    ) -> Decl {
        Decl {
            display: name.map(|n| self.resolved_span(n)),
            kind,
            is_abstract: has(modifiers, is_abstract),
            is_readonly: has(modifiers, |m| matches!(m, Modifier::Readonly(_))),
            immutable_doc: immutable_tags(&self.tags_before(at)),
            parent: extends
                .filter(|_| kind == SymbolKind::Class)
                .and_then(|e| e.types.first())
                .map(|p| self.resolved(p)),
        }
    }

    /// The class `class` whose context a trait is walked in.
    pub(super) fn decl_of_class(&mut self, class: &str) -> Option<Decl> {
        let meta = self.codebase.get_class_like(class.as_bytes())?;
        Some(Decl {
            display: Some(display(meta)),
            kind: meta.kind,
            is_abstract: meta.flags.is_abstract(),
            is_readonly: meta.flags.is_readonly(),
            immutable_doc: self.class_immutable(class, 0),
            parent: meta
                .direct_parent_class
                .map(|p| String::from_utf8_lossy(p.as_bytes()).into_owned()),
        })
    }

    /// ClassReflection::isImmutable() of a class in the codebase: its PHPDoc or a parent's.
    pub(super) fn class_immutable(&mut self, class: &str, depth: usize) -> bool {
        let key = class.to_ascii_lowercase();
        if let Some(known) = self.immutable.get(&key) {
            return *known;
        }
        let Some(meta) = self.codebase.get_class_like(class.as_bytes()) else {
            return false;
        };
        let own = self
            .file_contents(meta.span.file_id)
            .is_some_and(|contents| {
                doc_before(&contents, meta.span.start.offset as usize)
                    .is_some_and(|d| immutable_tags(&doc_tags(d)))
            });
        // a cyclic hierarchy is another rule's error
        let parent = meta
            .direct_parent_class
            .filter(|_| depth < 64)
            .map(|p| String::from_utf8_lossy(p.as_bytes()).into_owned());
        let result = own || parent.is_some_and(|p| self.class_immutable(&p, depth + 1));
        self.immutable.insert(key, result);
        result
    }

    /// `$scope->getPhpVersion()->...()->no()` for a feature of PHP `version` and later: false only when every
    /// version the narrowed `PHP_VERSION_ID` allows is older.
    pub(super) fn may_be_at_least(&self, version: i64) -> bool {
        // ConstantResolver::PHP_MIN_ANALYZABLE_VERSION_ID: a range no narrower is the configured version
        const MIN: i64 = 50207;
        match super::scope::version_range(&self.guards) {
            Some((lo, hi)) if lo.is_some_and(|l| l > MIN) || hi.is_some() => {
                let lo = lo.unwrap_or(MIN).max(MIN);
                // an empty range is never, which every version range contains
                hi.is_none_or(|h| h >= version || h < lo)
            }
            _ => i64::from(self.php_version) >= version,
        }
    }
}

/// DuplicateDeclarationRule: the duplicate constants, enum cases, properties (promoted ones too) and methods of a
/// class, interface or enum.
pub fn duplicates<'arena>(c: &mut Checker<'_, 'arena>, members: &[ClassLikeMember<'arena>]) {
    // called for named class-likes only, right after their Decl
    let decl = c.decls.last().expect("declared");
    let display = decl.display.clone().expect("named");
    let kind = match decl.kind {
        SymbolKind::Interface => "interface",
        SymbolKind::Enum => "enum",
        _ => "class",
    };
    let mut constants = HashSet::new();
    for m in members {
        match m {
            ClassLikeMember::EnumCase(case) => {
                let name = String::from_utf8_lossy(case.item.name().value).into_owned();
                if !constants.insert(name.clone()) {
                    c.report_non_ignorable(
                        case,
                        &format!("{kind}.duplicateEnumCase"),
                        format!("Cannot redeclare enum case {display}::{name}."),
                        None,
                    );
                }
            }
            ClassLikeMember::Constant(constant) => {
                for item in constant.items.iter() {
                    let name = String::from_utf8_lossy(item.name.value).into_owned();
                    if !constants.insert(name.clone()) {
                        c.report_non_ignorable(
                            item,
                            &format!("{kind}.duplicateConstant"),
                            format!("Cannot redeclare constant {display}::{name}."),
                            None,
                        );
                    }
                }
            }
            _ => {}
        }
    }
    let mut properties = HashSet::new();
    let mut duplicate_property = |c: &mut Checker<'_, 'arena>, at: Span, name: &[u8]| {
        let name = String::from_utf8_lossy(&name[1..]).into_owned();
        if !properties.insert(name.clone()) {
            c.report_non_ignorable(
                at,
                &format!("{kind}.duplicateProperty"),
                format!("Cannot redeclare property {display}::${name}."),
                None,
            );
        }
    };
    for m in members {
        if let ClassLikeMember::Property(p) = m {
            for v in p.variables() {
                duplicate_property(c, v.span, v.name);
            }
        }
    }
    let mut methods = HashSet::new();
    for m in members {
        let ClassLikeMember::Method(method) = m else {
            continue;
        };
        if method.name.value.eq_ignore_ascii_case(b"__construct") {
            for p in method.parameter_list.parameters.iter() {
                if !p.modifiers.is_empty() {
                    duplicate_property(c, p.span(), p.variable.name);
                }
            }
        }
        let name = String::from_utf8_lossy(method.name.value).into_owned();
        if !methods.insert(name.to_ascii_lowercase()) {
            c.report_non_ignorable(
                method,
                &format!("{kind}.duplicateMethod"),
                format!("Cannot redeclare method {display}::{name}()."),
                None,
            );
        }
    }
}

/// AbstractPrivateMethodRule and FinalPrivateMethodRule on a method (`name` is its name, a trait method's alias).
pub fn method<'arena>(c: &mut Checker<'_, 'arena>, m: &Method<'arena>, name: &str) {
    let decl = c.decls.last().expect("in a class-like");
    let (display, class_abstract, kind) = (decl.display.clone(), decl.is_abstract, decl.kind);
    let private = has(m.modifiers.as_slice(), is_private);
    if let Some(display) = &display
        && private
        && has(m.modifiers.as_slice(), is_abstract)
        && c.trait_context.is_none()
        && (class_abstract || kind == SymbolKind::Interface)
    {
        c.report_non_ignorable(
            m,
            "method.abstractPrivate",
            format!("Private method {display}::{name}() cannot be abstract."),
            None,
        );
    }
    if let Some(display) = &display
        && name != "__construct"
        && c.may_be_at_least(80000)
        && private
        && (has(m.modifiers.as_slice(), is_final) || has_tag(&c.tags_before(m), &["@final"]))
    {
        c.report(
            m,
            "method.finalPrivate",
            format!(
                "Private method {display}::{name}() cannot be final as it is never overridden by other classes."
            ),
            None,
        );
    }
}

/// FinalPrivateConstantRule.
pub fn constant<'arena>(c: &mut Checker<'_, 'arena>, constant: &ClassLikeConstant<'arena>) {
    let modifiers = constant.modifiers.as_slice();
    let Some(display) = c
        .decls
        .last()
        .and_then(|d| d.display.clone())
        .filter(|_| has(modifiers, is_final) && has(modifiers, is_private))
    else {
        return;
    };
    for item in constant.items.iter() {
        c.report_non_ignorable(
            constant,
            "classConstant.finalPrivate",
            format!(
                "Private constant {display}::{}() cannot be final as it is never overridden by other classes.",
                String::from_utf8_lossy(item.name.value)
            ),
            None,
        );
    }
}

/// A function-like for InvalidPromotedPropertiesRule: a method's name (a trait method's alias and original name) and
/// whether it has a body.
pub enum FunctionLike<'n> {
    Method {
        name: &'n str,
        original: &'n [u8],
        body: bool,
    },
    Other,
}

/// InvalidPromotedPropertiesRule.
pub fn promoted<'arena>(
    c: &mut Checker<'_, 'arena>,
    at: Span,
    kind: FunctionLike,
    parameters: &FunctionLikeParameterList<'arena>,
) {
    let promotes = |p: &FunctionLikeParameter| {
        !p.modifiers.is_empty() || p.hooks.as_ref().is_some_and(|h| !h.hooks.is_empty())
    };
    if !parameters.parameters.iter().any(promotes) {
        return;
    }
    if i64::from(c.php_version) < 80000 {
        c.report_non_ignorable(
            at,
            "property.promotedNotSupported",
            "Promoted properties are supported only on PHP 8.0 and later.".to_string(),
            None,
        );
        return;
    }
    let FunctionLike::Method {
        name,
        original,
        body,
    } = kind
    else {
        return c.report_non_ignorable(
            at,
            "property.invalidPromoted",
            "Promoted properties can be in constructor only.".to_string(),
            None,
        );
    };
    if !name.eq_ignore_ascii_case("__construct") && !original.eq_ignore_ascii_case(b"__construct") {
        return c.report_non_ignorable(
            at,
            "property.invalidPromoted",
            "Promoted properties can be in constructor only.".to_string(),
            None,
        );
    }
    if !body {
        return c.report_non_ignorable(
            at,
            "property.invalidPromoted",
            "Promoted properties are not allowed in abstract constructors.".to_string(),
            None,
        );
    }
    for p in parameters.parameters.iter() {
        if p.modifiers.is_empty() {
            continue;
        }
        if has(p.modifiers.as_slice(), is_final) && i64::from(c.php_version) < 80500 {
            c.report_non_ignorable(
                p,
                "property.invalidPromoted",
                "Final promoted properties are supported only on PHP 8.5 and later.".to_string(),
                None,
            );
        }
        if p.ellipsis.is_some() {
            c.report_non_ignorable(
                p,
                "property.invalidPromoted",
                format!(
                    "Promoted property parameter ${} can not be variadic.",
                    String::from_utf8_lossy(&p.variable.name[1..])
                ),
                None,
            );
        }
    }
}
