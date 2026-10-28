//! The type tags of a doc comment as phpstan/phpdoc-parser reads them for Rector (`VarTagValueNode`,
//! `ParamTagValueNode`, `ReturnTagValueNode`): the type node (pun-phpstan's phpdoc parser), the variable and whether a
//! description follows, read from one child of a [`Doc`]. Also the checks Rector makes on the type node itself
//! (`PhpDocTypeChanger::isAllowed()`, the removal guards and the union analyzers).

use pun_phpstan::phpdoc::{ConstExpr, Node, parse_prefix};

use crate::rules::lint_b_doc::Doc;

/// Which value node a tag holds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Kind {
    Var,
    Param,
    Return,
}

/// A parsed type tag.
#[derive(Clone, Debug)]
pub(crate) struct TypeTag {
    /// The tag's name with its `@` (`@var`, `@phpstan-param`).
    pub name: String,
    pub kind: Kind,
    pub ty: Node,
    /// The variable, with its `$`; `""` for none.
    pub variable: String,
    /// Whether a description follows the type and variable.
    pub has_description: bool,
}

/// The value node phpdoc-parser builds for the tag `name`.
fn kind_of(name: &str) -> Option<Kind> {
    match name {
        "@var" | "@phpstan-var" | "@psalm-var" | "@phan-var" => Some(Kind::Var),
        "@param" | "@phpstan-param" | "@psalm-param" | "@phan-param" => Some(Kind::Param),
        "@return" | "@phpstan-return" | "@psalm-return" | "@phan-return" | "@phan-real-return" => {
            Some(Kind::Return)
        }
        _ => None,
    }
}

/// The type tag of child `i` of `doc`; `None` for text, another tag, or a value phpdoc-parser rejects
/// (`InvalidTagValueNode`, a `TypelessParamTagValueNode`).
pub(crate) fn type_tag(doc: &Doc<'_>, i: usize) -> Option<TypeTag> {
    parse(doc.text(i))
}

fn parse(text: &str) -> Option<TypeTag> {
    // the continuation lines without their `*`
    let text: String = text
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            if i == 0 {
                line.to_string()
            } else {
                let line = line.trim_start();
                line.strip_prefix('*').unwrap_or(line).to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let end = text.find(char::is_whitespace).unwrap_or(text.len());
    let (name, rest) = text.split_at(end);
    let kind = kind_of(name)?;
    let value = rest.trim_start();
    if value.is_empty() {
        return None;
    }
    let (ty, offset) = parse_prefix(value).ok()?;
    let mut rest = &value[offset..];
    let mut variable = String::new();
    let after_type = rest.trim_start_matches([' ', '\t']);
    let variable_at = |s: &str| -> Option<usize> {
        let name = s.strip_prefix('$')?;
        let len = name
            .bytes()
            .take_while(|b| b.is_ascii_alphanumeric() || *b == b'_' || *b >= 0x80)
            .count();
        (len > 0).then_some(len + 1)
    };
    match kind {
        Kind::Var => {
            if let Some(len) = variable_at(after_type) {
                variable = after_type[..len].to_string();
                rest = &after_type[len..];
            }
        }
        Kind::Param => {
            let mut s = after_type
                .strip_prefix('&')
                .unwrap_or(after_type)
                .trim_start();
            s = s.strip_prefix("...").unwrap_or(s).trim_start();
            let len = variable_at(s)?;
            variable = s[..len].to_string();
            rest = &s[len..];
        }
        Kind::Return => {}
    }
    Some(TypeTag {
        name: name.to_string(),
        kind,
        ty,
        variable,
        has_description: !rest.trim().is_empty(),
    })
}

/// `getVarTagValueNode()`, `getReturnTagValue()`: the value of the first tag named `name` (`@var`, `@return`),
/// when that tag's value is valid (`getVarTagValues()[0]`).
pub(crate) fn first_named(doc: &Doc<'_>, name: &str) -> Option<(usize, TypeTag)> {
    let i = (0..doc.len()).find(|&i| tag_name(doc.text(i)) == name)?;
    type_tag(doc, i).map(|t| (i, t))
}

/// The name of the tag a child's text starts with (`@var`), `""` for text.
pub(crate) fn tag_name(text: &str) -> &str {
    if !text.starts_with('@') {
        return "";
    }
    &text[..text.find(char::is_whitespace).unwrap_or(text.len())]
}

/// `PhpDocInfo::removeByType()` of `kind`'s value node: the children holding one, except the `@psalm-` and
/// `@phpstan-` tags; a `VarTagValueNode` with another variable than `variable` (when it is not `""`) stays.
pub(crate) fn removed_by_type(doc: &Doc<'_>, kind: Kind, variable: &str) -> Vec<bool> {
    (0..doc.len())
        .map(|i| {
            type_tag(doc, i).is_some_and(|t| {
                t.kind == kind
                    && !t.name.starts_with("@psalm-")
                    && !t.name.starts_with("@phpstan-")
                    && (variable.is_empty() || kind != Kind::Var || t.variable == variable)
            })
        })
        .collect()
}

/// `PhpDocTypeChanger::isAllowed()`: a type the native one cannot say (a generic, an array, a callable, a shape, a
/// constant fetch, `class-string`), or a union or intersection with one.
pub(crate) fn is_allowed(node: &Node) -> bool {
    match node {
        Node::Union(types) | Node::Inter(types) => types.iter().any(is_allowed),
        Node::Const(ConstExpr::Fetch(..))
        | Node::Generic { .. }
        | Node::Array(_)
        | Node::Callable { .. }
        | Node::ArrayShape { .. } => true,
        Node::Ident(name) => name == "class-string",
        _ => false,
    }
}

/// `MixedArrayTypeNodeAnalyzer::hasMixedArrayType()`: a `mixed[]` member.
pub(crate) fn has_mixed_array(members: &[Node]) -> bool {
    members.iter().any(
        |m| matches!(m, Node::Array(item) if matches!(&**item, Node::Ident(n) if n == "mixed")),
    )
}

/// `GenericTypeNodeAnalyzer::hasGenericType()`.
pub(crate) fn has_generic(members: &[Node]) -> bool {
    members.iter().any(|m| matches!(m, Node::Generic { .. }))
}

/// `StandaloneTypeRemovalGuard::isLegal()`: `true` or `false` documented over a native `bool` says more.
pub(crate) fn is_standalone_legal(node: &Node, native_is_bool_identifier: bool) -> bool {
    match node {
        Node::Ident(name) if native_is_bool_identifier => name != "false" && name != "true",
        _ => true,
    }
}
