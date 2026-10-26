//! `lint/phpunit/prefer-data-provider-attribute`, ported from rector-phpunit's
//! `DataProviderAnnotationToAttributeRector` (rules/AnnotationsToAttributes/Rector/ClassMethod).
//!
//! A test method's `@dataProvider` tags become `#[\PHPUnit\Framework\Attributes\DataProvider('method')]` groups, or
//! `#[\PHPUnit\Framework\Attributes\DataProviderExternal(\Class::class, 'method')]` for a `Class::method` value,
//! after its attributes, when the attribute class exists. A group the method has already (or got from an earlier
//! tag) is not added again; the tag goes either way. Rector runs it only for PHPUnit 10 and later; Pun leaves that to
//! the rule's group.

use mago_syntax::cst::{
    Access, AttributeList, ClassLikeConstantSelector, Expression, Identifier, Literal,
    LiteralStringKind, Node, NodeKind, PartialArgument,
};
use pun_problem::Safety;

use super::annotations::{Annotated, TagValue, named_tags};
use super::{is_in_test_class, report};
use crate::rules::prefer_absolute_include::print_string;
use crate::rules::rector_batch5::resolve_class_name;
use crate::rules::types::facts::facts;
use crate::{Context, Report, Rule};

const DATA_PROVIDER: &str = "PHPUnit\\Framework\\Attributes\\DataProvider";
const DATA_PROVIDER_EXTERNAL: &str = "PHPUnit\\Framework\\Attributes\\DataProviderExternal";

pub struct PreferDataProviderAttribute;

impl Rule for PreferDataProviderAttribute {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-data-provider-attribute"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else {
            return;
        };
        let Some(annotated) = Annotated::of(ctx.source, node, ancestors) else {
            return;
        };
        let Some(doc) = annotated.doc.as_ref() else {
            return;
        };
        if !doc.text_all().contains("@dataProvider")
            || !is_in_test_class(ctx, ancestors, Some(method))
            || facts(ctx)
                .codebase
                .get_class_like(DATA_PROVIDER.as_bytes())
                .is_none()
        {
            return;
        }
        let Some(tags) = named_tags(ctx, ancestors, doc, "dataProvider") else {
            return;
        };
        // The groups the method has, as php-parser prints them with their names resolved (`NodeComparator`).
        let mut existing: Vec<Option<String>> = method
            .attribute_lists
            .iter()
            .map(|list| print_group(ancestors, list))
            .collect();
        let mut removed = vec![false; doc.len()];
        let mut groups = Vec::new();
        for (i, value) in tags {
            let value = match value {
                TagValue::Generic(value) | TagValue::Annotation(value) => value,
                TagValue::Parenthesized(rest) => {
                    let Some(content) = doctrine_content(&rest) else {
                        return;
                    };
                    content
                }
            };
            let Some(token) = value
                .split([' ', '\t', '\n', '\r', '\0', '\x0B'])
                .find(|t| !t.is_empty())
            else {
                continue;
            };
            let group = create_attribute_group(token);
            if !existing.contains(&Some(group.clone())) {
                existing.push(Some(group.clone()));
                groups.push(group);
            }
            // cleanup
            removed[i] = true;
        }
        if !removed.contains(&true) {
            return;
        }
        let new_doc = doc.print(&removed);
        out.push(report(
            doc.span,
            "Use the `#[DataProvider]` attribute instead of the `@dataProvider` annotation.",
            Safety::Safe,
            annotated.edits(ctx.source, Some(&new_doc), &groups),
        ));
    }
}

/// The start of the original content `DoctrineAnnotationDecorator::processDoctrine()` gives a Doctrine tag whose
/// text after the name is `rest`: `(argument)` and the description, as far as its first line goes. `None` for what
/// Pun does not rebuild: brackets that do not close on the first line, more than one argument, or an argument
/// phpdoc-parser's printer may write otherwise.
fn doctrine_content(rest: &str) -> Option<String> {
    let inner = rest.trim_start_matches([' ', '\t']).strip_prefix('(')?;
    let close = inner.find(')')?;
    let argument = inner[..close].trim_matches([' ', '\t']);
    let simple = !argument.contains(|c: char| {
        c.is_whitespace() || matches!(c, ',' | '(' | '[' | ']' | '{' | '}' | '\'' | '@')
    }) && (!argument.contains('"')
        || (argument.matches('"').count() == 2 && argument.ends_with('"')));
    if !simple {
        return None;
    }
    // processDoctrine(): no arguments give an empty content
    if argument.is_empty() {
        return Some(String::new());
    }
    let line = inner[close + 1..].split('\n').next().unwrap_or_default();
    if line.contains('@') {
        return None;
    }
    let description = line.split_whitespace().next().unwrap_or_default();
    Some(format!("({argument}){description}"))
}

/// `createAttributeGroup()`: the group for the tag's first word, printed.
fn create_attribute_group(value: &str) -> String {
    let method_name = value.trim_matches(['(', ')']);
    let (class_name, method_name) = match method_name.split_once("::") {
        Some((class, method)) => (class, method),
        None => ("", method_name),
    };
    if class_name.is_empty() {
        return format!("#[\\{DATA_PROVIDER}({})]", map_item(method_name));
    }
    let class_name = if class_name.starts_with('\\') {
        class_name.to_string()
    } else {
        format!("\\{class_name}")
    };
    format!(
        "#[\\{DATA_PROVIDER_EXTERNAL}({}, {})]",
        map_item(&format!("{class_name}::class")),
        map_item(method_name)
    )
}

/// `AnnotationToAttributeMapper::map()` for a string item, printed: `Class::CONSTANT`
/// (`ClassConstFetchAnnotationToAttributeMapper`), else a string (`StringAnnotationToAttributeMapper`, its outer
/// `"` gone), double-quoted when it has a `'` and no line break (`NamedArgsFactory::normalizeStringDoubleQuote()`).
fn map_item(value: &str) -> String {
    if value.contains("::") && !value.starts_with('"') {
        let parts: Vec<&str> = value.split("::").collect();
        if let [class, constant] = parts[..]
            && !class.is_empty()
            && is_class_name(class.trim_start_matches('\\'))
            && is_constant_name(constant)
        {
            return format!("{class}::{constant}");
        }
        return print_str(value);
    }
    let value = if value.starts_with('"') && value.ends_with('"') {
        value.trim_matches('"')
    } else {
        value
    };
    print_str(value)
}

/// A `String_` as php-parser prints it, its kind as the mappers set it.
fn print_str(value: &str) -> String {
    let kind = if value.contains('\'') && !value.contains('\n') {
        LiteralStringKind::DoubleQuoted
    } else {
        LiteralStringKind::SingleQuoted
    };
    print_string(kind, value.as_bytes()).unwrap_or_default()
}

/// `RectorAssert::className()`: `^[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*(\\[a-zA-Z_\x7f-\xff][a-zA-Z0-9_\x7f-\xff]*)*$`.
fn is_class_name(name: &str) -> bool {
    name.split('\\').all(|part| is_name(part, 0x7f))
}

/// `RectorAssert::constantName()`: `^[a-zA-Z_\x80-\xff][a-zA-Z0-9_\x80-\xff]*$`.
fn is_constant_name(name: &str) -> bool {
    is_name(name, 0x80)
}

/// A name of letters, `_`, digits (not first) and bytes from `high` up.
fn is_name(name: &str, high: u8) -> bool {
    let b = name.as_bytes();
    let letter = |c: u8| c.is_ascii_alphabetic() || c == b'_' || c >= high;
    !b.is_empty() && letter(b[0]) && b[1..].iter().all(|&c| letter(c) || c.is_ascii_digit())
}

/// An attribute group as php-parser's printer gives it once php-parser's `NameResolver` has replaced its names, when
/// it has one attribute whose arguments are positional strings or class constants; `None` for any other group,
/// which no new group equals.
fn print_group(ancestors: &[Node<'_, '_>], list: &AttributeList<'_>) -> Option<String> {
    let [attribute] = list.attributes.as_slice() else {
        return None;
    };
    let name = format!("\\{}", resolve_class_name(ancestors, &attribute.name));
    let Some(arguments) = &attribute.argument_list else {
        return Some(format!("#[{name}]"));
    };
    let mut printed = Vec::new();
    for argument in arguments.arguments.iter() {
        let PartialArgument::Positional(argument) = argument else {
            return None;
        };
        if argument.ellipsis.is_some() {
            return None;
        }
        printed.push(print_expression(ancestors, argument.value)?);
    }
    Some(format!("#[{name}({})]", printed.join(", ")))
}

/// A string literal or a `Class::CONSTANT` as php-parser prints it, its class name resolved.
fn print_expression(ancestors: &[Node<'_, '_>], expression: &Expression<'_>) -> Option<String> {
    match expression {
        Expression::Literal(Literal::String(string)) => print_string(string.kind, string.value?),
        Expression::Access(Access::ClassConstant(access)) => {
            let (Expression::Identifier(class), ClassLikeConstantSelector::Identifier(constant)) =
                (access.class, &access.constant)
            else {
                return None;
            };
            let constant = String::from_utf8_lossy(constant.value);
            Some(format!("{}::{constant}", print_class(ancestors, class)))
        }
        _ => None,
    }
}

/// A class name as php-parser prints it after `NameResolver`: `self`, `parent` and `static` as written, any other
/// name fully qualified.
fn print_class(ancestors: &[Node<'_, '_>], class: &Identifier<'_>) -> String {
    let written = String::from_utf8_lossy(class.value());
    if ["self", "parent", "static"]
        .iter()
        .any(|special| written.eq_ignore_ascii_case(special))
    {
        return written.into_owned();
    }
    format!("\\{}", resolve_class_name(ancestors, class))
}
