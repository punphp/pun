//! `lint/laravel/prefer-validation-rule-array`, ported from rector-laravel 2.5.0's
//! `ValidationRuleArrayStringValueToArrayRector` (src/Rector/MethodCall).
//!
//! In the rules array of `$request->validate([...])` (an `Illuminate\Http\Request`), `Validator::make($data, [...])`
//! (the `Illuminate\Support\Facades\Validator` facade), `$factory->make($data, [...])` (an
//! `Illuminate\Validation\Factory`, with exactly two arguments) and the first returned array of a form request's
//! `rules()` method that has one, each keyed item whose value is a string becomes an array of the rules between its
//! `|`s: `'name' => 'required|string'` becomes `'name' => ['required', 'string']`. php-parser prints each new string
//! anew: a plain one single-quoted, an interpolated one double-quoted with each interpolation in `{}`. A heredoc or
//! nowdoc is read as php-parser reads it (`parseDocString`), and prints the same way.

use mago_allocator::{Arena, LocalArena};
use mago_syntax::cst::{
    ArrayElement, ClassLikeMember, CompositeString, DocumentIndentation, DocumentKind,
    DocumentString, Expression, Literal, LiteralStringKind, MethodBody, Node, NodeKind, Statement,
    StringPart,
};
use mago_syntax_core::utils::parse_literal_string_in;
use pun_problem::{Edit, Fix, Safety};

use super::object_type::is_object_type;
use super::{is_a, is_name, member_name};
use crate::rules::class_method::qualified;
use crate::rules::prefer_absolute_include::print_string;
use crate::rules::rector_code_quality::text;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferValidationRuleArray;

impl Rule for PreferValidationRuleArray {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-validation-rule-array"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::MethodCall,
            NodeKind::StaticMethodCall,
            NodeKind::Class,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let rules = match node {
            Node::MethodCall(call) => {
                let Some(name) = member_name(&call.method) else {
                    return;
                };
                let args = call.argument_list.arguments.as_slice();
                if is_name(name, "validate") {
                    if !is_object_type(ctx, ancestors, call.object, "Illuminate\\Http\\Request") {
                        return;
                    }
                    let Some(first) = args.first() else { return };
                    first.value()
                } else if is_name(name, "make") {
                    let [_, rules] = args else { return };
                    if !is_object_type(
                        ctx,
                        ancestors,
                        call.object,
                        "Illuminate\\Validation\\Factory",
                    ) {
                        return;
                    }
                    rules.value()
                } else {
                    return;
                }
            }
            Node::StaticMethodCall(call) => {
                if !member_name(&call.method).is_some_and(|n| is_name(n, "make")) {
                    return;
                }
                let [_, rules] = call.argument_list.arguments.as_slice() else {
                    return;
                };
                if !is_object_type(
                    ctx,
                    ancestors,
                    call.class,
                    "Illuminate\\Support\\Facades\\Validator",
                ) {
                    return;
                }
                rules.value()
            }
            _ => {
                let Node::Class(class) = node else { return };
                let name = qualified(ancestors, class.name.value);
                if !is_a(ctx, &name, &["Illuminate\\Foundation\\Http\\FormRequest"]) {
                    return;
                }
                let mut edits = Vec::new();
                for member in class.members.iter() {
                    let ClassLikeMember::Method(method) = member else {
                        continue;
                    };
                    if !is_name(method.name.value, "rules") {
                        continue;
                    }
                    let MethodBody::Concrete(body) = &method.body else {
                        continue;
                    };
                    edits.extend(first_return_edits(ctx, Node::Block(body)));
                }
                report(node, edits, out);
                return;
            }
        };
        report(node, rule_edits(ctx, unparenthesize(rules)), out);
    }
}

fn report(node: Node<'_, '_>, edits: Vec<Edit>, out: &mut Vec<Report>) {
    if edits.is_empty() {
        return;
    }
    out.push(Report {
        span: node_span(&node),
        message:
            "Write each validation rule as an item of an array instead of joining them with `|`."
                .to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Safe,
            edits,
        }),
    });
}

/// The edits of the first `return [...]` under `node` (in source order, parents first, closures included) whose
/// array has a rule to split: Rector's traversal stops at the node after it.
fn first_return_edits(ctx: &Context<'_>, node: Node<'_, '_>) -> Vec<Edit> {
    let (mut stack, mut children) = (vec![node], Vec::new());
    while let Some(node) = stack.pop() {
        if let Node::Statement(Statement::Return(r)) = node
            && let Some(value) = r.value
        {
            let edits = rule_edits(ctx, unparenthesize(value));
            if !edits.is_empty() {
                return edits;
            }
        }
        node.visit_children(|child| children.push(child));
        stack.extend(children.drain(..).rev());
    }
    Vec::new()
}

/// `processValidationRules`: an edit for each keyed item of the array literal `rules` whose value is a string.
fn rule_edits(ctx: &Context<'_>, rules: &Expression<'_>) -> Vec<Edit> {
    let elements = match rules {
        Expression::Array(a) => &a.elements,
        Expression::LegacyArray(a) => &a.elements,
        _ => return Vec::new(),
    };
    let mut edits = Vec::new();
    for element in elements.iter() {
        let ArrayElement::KeyValue(item) = element else {
            continue;
        };
        let value = item.value;
        let items = match value {
            Expression::Literal(Literal::String(s)) => s.value.and_then(string_rules),
            Expression::CompositeString(CompositeString::Interpolated(s)) => {
                let parts = (s.parts.iter())
                    .map(|p| match p {
                        StringPart::Literal(l) => l.value.map(|v| Part::Literal(v.to_vec())),
                        StringPart::Expression(e) => Some(Part::Expression(text(ctx, *e))),
                        StringPart::BracedExpression(b) => {
                            Some(Part::Expression(text(ctx, b.expression)))
                        }
                    })
                    .collect::<Option<Vec<_>>>();
                parts.and_then(|p| interpolated_rules(&p))
            }
            Expression::CompositeString(CompositeString::Document(d)) => {
                match (document_value(ctx, d), document_parts(ctx, d)) {
                    (Some(value), _) => string_rules(&value),
                    (None, parts) => parts.and_then(|p| interpolated_rules(&p)),
                }
            }
            _ => continue,
        };
        // A string that is not UTF-8 stays (php-parser escapes its bytes one by one).
        if let Some(items) = items {
            edits.push(Edit::new(
                node_span(value),
                format!("[{}]", items.join(", ")),
            ));
        }
    }
    edits
}

/// A part of an interpolated string as php-parser holds it: the decoded text of an `InterpolatedStringPart`, or an
/// expression, printed as `{...}`.
enum Part<'s> {
    Literal(Vec<u8>),
    Expression(&'s str),
}

/// `processStringRule`: the rules between the `|`s of a string, each printed as a new single-quoted string.
fn string_rules(value: &[u8]) -> Option<Vec<String>> {
    (value.split(|b| *b == b'|'))
        .map(|rule| print_string(LiteralStringKind::SingleQuoted, rule))
        .collect()
}

/// `processInterpolatedStringRule`: the parts split at each `|` of their literal text, each rule printed as a new
/// double-quoted string, an empty rule dropped.
fn interpolated_rules(parts: &[Part<'_>]) -> Option<Vec<String>> {
    let mut rules: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();
    for part in parts {
        match part {
            Part::Literal(value) => {
                for (index, segment) in value.split(|b| *b == b'|').enumerate() {
                    if index != 0 && !current.is_empty() {
                        rules.push(std::mem::take(&mut current));
                    }
                    if !segment.is_empty() {
                        let quoted = print_string(LiteralStringKind::DoubleQuoted, segment)?;
                        current.push(quoted[1..quoted.len() - 1].to_string());
                    }
                }
            }
            Part::Expression(code) => current.push(format!("{{{code}}}")),
        }
    }
    if !current.is_empty() {
        rules.push(current);
    }
    Some(
        rules
            .iter()
            .map(|r| format!("\"{}\"", r.concat()))
            .collect(),
    )
}

/// The value of a heredoc or nowdoc php-parser reads as a `String_` (no interpolated part), as [`document_parts`]
/// decodes it.
pub(crate) fn document_value(ctx: &Context<'_>, doc: &DocumentString<'_>) -> Option<Vec<u8>> {
    let mut value = Vec::new();
    for part in document_parts(ctx, doc)? {
        match part {
            Part::Literal(v) => value.extend(v),
            Part::Expression(_) => return None,
        }
    }
    Some(value)
}

/// php-parser's `parseDocString`: the parts of a heredoc or nowdoc, each literal part without the closing label's
/// indentation at the start of its lines (`stripIndentation`), then decoded (a heredoc's escapes); a literal part left
/// empty goes. The newline before the closing label is not in Mago's last part. Mixed tabs and spaces count as no
/// indentation, as php-parser recovers from that error.
fn document_parts<'s>(ctx: &Context<'s>, doc: &DocumentString<'_>) -> Option<Vec<Part<'s>>> {
    let indent = match doc.indentation {
        DocumentIndentation::Whitespace(n) | DocumentIndentation::Tab(n) => n,
        DocumentIndentation::None | DocumentIndentation::Mixed(..) => 0,
    };
    let mut out = Vec::new();
    for (i, part) in doc.parts.iter().enumerate() {
        let literal = match part {
            StringPart::Literal(l) => l,
            StringPart::Expression(e) => {
                out.push(Part::Expression(text(ctx, *e)));
                continue;
            }
            StringPart::BracedExpression(b) => {
                out.push(Part::Expression(text(ctx, b.expression)));
                continue;
            }
        };
        let raw = strip_indentation(literal.raw, indent, i == 0);
        let value = match doc.kind {
            DocumentKind::Nowdoc => raw,
            DocumentKind::Heredoc => {
                // Mago decodes a double-quoted string's escapes; a heredoc keeps `\"` as it is, so that
                // backslash is doubled first.
                let mut kept = Vec::with_capacity(raw.len());
                let mut backslashes = 0;
                for &b in &raw {
                    if b == b'"' && backslashes % 2 == 1 {
                        kept.push(b'\\');
                    }
                    backslashes = if b == b'\\' { backslashes + 1 } else { 0 };
                    kept.push(b);
                }
                let arena = LocalArena::new();
                let kept = arena.alloc_slice_copy(&kept);
                parse_literal_string_in(&arena, kept, Some(b'"'), false)?.to_vec()
            }
        };
        if !value.is_empty() {
            out.push(Part::Literal(value));
        }
    }
    Some(out)
}

/// `stripIndentation`: up to `indent` blanks taken from the start of each line (and of the text, when it starts a
/// line).
fn strip_indentation(raw: &[u8], indent: usize, at_line_start: bool) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len());
    let mut line_start = at_line_start;
    let mut i = 0;
    while i < raw.len() {
        if line_start {
            let blanks = raw[i..]
                .iter()
                .take_while(|b| matches!(b, b' ' | b'\t'))
                .count();
            i += blanks.min(indent);
            line_start = false;
            continue;
        }
        out.push(raw[i]);
        line_start = raw[i] == b'\n';
        i += 1;
    }
    out
}
