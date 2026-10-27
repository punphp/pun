//! The typed tags (`@var`, `@return`, `@param`) of a doc comment as phpdoc-parser reads them, as far as Pun follows:
//! a type that is a name (`IdentifierTypeNode`), or a union or an intersection of names, each maybe with `[]`s, then
//! the variable and the description. A changed tag is printed as phpdoc-parser prints a tag value node anew:
//! `@tag type $variable description`, single-spaced.

use crate::rules::lint_b_doc::Doc;

/// The type of a typed tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TagType<'s> {
    /// A name (`IdentifierTypeNode`).
    Name(&'s str),
    /// A union of two or more types, as written (`UnionTypeNode`).
    Union(Vec<&'s str>),
    /// An intersection of two or more types, as written (`IntersectionTypeNode`).
    Intersection(Vec<&'s str>),
    /// Any other type, e.g. `Foo[]`, as written.
    Other(&'s str),
}

/// A typed tag of a doc comment: its index among the doc's children, its name (`@var`), type, variable and
/// description.
#[derive(Debug, Clone)]
pub(crate) struct TypedTag<'s> {
    pub index: usize,
    pub name: &'s str,
    pub ty: TagType<'s>,
    pub variable: &'s str,
    pub description: &'s str,
}

impl TypedTag<'_> {
    /// The tag with the type `ty`, as phpdoc-parser prints a changed tag.
    pub(crate) fn print_with(&self, ty: &str) -> String {
        let rest = format!("{} {}", self.variable, self.description);
        let value = format!("{ty} {}", rest.trim());
        format!("{} {}", self.name, value.trim())
    }
}

/// The first `name` tag (`@var`, `@return`) of `doc`, or, with `variable`, the first that names it (`@param`).
/// `Err(())` where Pun does not read the tag as phpdoc-parser does: a value over several lines, or a type it does not
/// parse.
pub(crate) fn typed_tag<'s>(
    doc: &Doc<'s>,
    name: &str,
    variable: Option<&str>,
) -> Result<Option<TypedTag<'s>>, ()> {
    for index in 0..doc.len() {
        let text = doc.text(index);
        let Some(rest) = text.strip_prefix(name) else {
            continue;
        };
        if !rest.is_empty() && !rest.starts_with([' ', '\t']) {
            continue;
        }
        if text.contains('\n') {
            return Err(());
        }
        let tag = parse(index, &text[..name.len()], rest).ok_or(())?;
        if variable.is_none_or(|v| tag.variable == v) {
            return Ok(Some(tag));
        }
    }
    Ok(None)
}

/// The tag value `rest` (after the tag name) parsed: a type, then a variable and a description.
fn parse<'s>(index: usize, name: &'s str, rest: &'s str) -> Option<TypedTag<'s>> {
    let value = rest.trim_start_matches([' ', '\t']);
    let mut members = Vec::new();
    let mut separator = None;
    let mut at = 0;
    loop {
        let len = member_len(&value[at..])?;
        members.push(&value[at..at + len]);
        at += len;
        let after = value[at..].trim_start_matches([' ', '\t']);
        match after.chars().next() {
            Some(c @ ('|' | '&')) if separator.is_none_or(|s| s == c) => {
                separator = Some(c);
                at = value.len() - after.len() + 1;
                at += value[at..].len() - value[at..].trim_start_matches([' ', '\t']).len();
            }
            Some('|' | '&') => return None,
            _ => break,
        }
    }
    let tail = &value[at..];
    if !tail.is_empty() && !tail.starts_with([' ', '\t']) {
        return None;
    }
    let tail = tail.trim_matches([' ', '\t']);
    let (variable, description) = match tail.strip_prefix('$') {
        Some(v) => {
            let len = v
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || !c.is_ascii()))
                .unwrap_or(v.len());
            (
                &tail[..len + 1],
                tail[len + 1..].trim_start_matches([' ', '\t']),
            )
        }
        None => ("", tail),
    };
    let ty = match (separator, members.as_slice()) {
        (None, [one]) if is_name(one) => TagType::Name(one),
        (None, [one]) => TagType::Other(one),
        (Some('|'), _) => TagType::Union(members),
        _ => TagType::Intersection(members),
    };
    Some(TypedTag {
        index,
        name,
        ty,
        variable,
        description,
    })
}

/// The length of a name with any `[]`s at the start of `s`; `None` for anything else.
fn member_len(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    let mut i = 0;
    loop {
        if b.get(i) == Some(&b'\\') {
            i += 1;
        }
        let start = i;
        while i < b.len()
            && (b[i].is_ascii_alphanumeric()
                || b[i] == b'_'
                || b[i] >= 0x80
                || (i > start && b[i] == b'-'))
        {
            i += 1;
        }
        if i == start || b[start].is_ascii_digit() {
            return None;
        }
        if b.get(i) != Some(&b'\\') {
            break;
        }
    }
    while b.get(i) == Some(&b'[') && b.get(i + 1) == Some(&b']') {
        i += 2;
    }
    // generics, shapes, callables and constants are not followed
    if matches!(b.get(i), Some(b'<' | b'{' | b'(' | b'[' | b':')) {
        return None;
    }
    Some(i)
}

/// Whether a member is a bare name.
fn is_name(member: &str) -> bool {
    !member.ends_with(']')
}

/// The short name of a type name (after its last `\`).
pub(crate) fn short_name(name: &str) -> &str {
    name.rsplit('\\').next().unwrap_or(name)
}
