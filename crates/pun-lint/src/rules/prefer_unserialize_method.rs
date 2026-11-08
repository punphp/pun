//! `lint/classes/prefer-unserialize-method`, ported from Rector 2.8.4's `WakeupToUnserializeRector`
//! (rules/Php85/Rector/Class_).
//!
//! PHP 8.5 deprecates `__wakeup()`. A class without `__unserialize()` gets its `__wakeup()` renamed to
//! `__unserialize(array $data): void`, with a body that copies each entry of `$data` to the property of that name.
//! The old body goes. php-parser prints the new body in place of a single old statement; for any other count it
//! reprints the whole method. Pun leaves an abstract `__wakeup()` alone, where Rector would print a body for it.

use mago_syntax::cst::{MethodBody, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::lint_b_serialize::{append_parameter, class_method, rename, set_return_type};
use super::rector_batch5::modifiers_text;
use super::rector_stmts::{block_list, line_indent};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferUnserializeMethod;

impl Rule for PreferUnserializeMethod {
    fn id(&self) -> &'static str {
        "lint/classes/prefer-unserialize-method"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::DEPRECATED_METHOD_WAKEUP
        80500
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if class_method(node, "__unserialize").is_some() {
            return;
        }
        let Some(method) = class_method(node, "__wakeup") else {
            return;
        };
        let MethodBody::Concrete(block) = &method.body else {
            return;
        };
        let source = ctx.source;
        let list = block_list(source, block);
        let edits = if list.len() == 1 {
            // The one statement, or the `Nop` of a body that holds only comments.
            let old = list
                .nop_span(source)
                .filter(|_| list.stmts.is_empty())
                .unwrap_or_else(|| list.with_comments(ctx, 0));
            vec![
                rename(method, "__unserialize"),
                append_parameter(source, method, "array $data"),
                set_return_type(method, "void"),
                Edit::new(old, foreach(line_indent(source, old.start))),
            ]
        } else {
            let span = node_span(method);
            let indent = line_indent(source, span.start);
            let mut code = String::new();
            for list in &method.attribute_lists {
                let s = node_span(list);
                code.push_str(&source[s.start..s.end]);
                code.push('\n');
                code.push_str(indent);
            }
            code.push_str(&modifiers_text(&method.modifiers));
            code.push_str("function ");
            if method.ampersand.is_some() {
                code.push('&');
            }
            code.push_str("__unserialize(");
            for parameter in &method.parameter_list.parameters {
                let s = node_span(parameter);
                code.push_str(&source[s.start..s.end]);
                code.push_str(", ");
            }
            let body = foreach(&format!("{indent}    "));
            code.push_str(&format!(
                "array $data): void\n{indent}{{\n{indent}    {body}\n{indent}}}"
            ));
            vec![Edit::new(Span::new(span.start, span.end), code)]
        };
        out.push(Report {
            span: node_span(&method.name),
            message: "Replace the deprecated `__wakeup()` with `__unserialize()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}

/// The `foreach` Rector's `assignProperties()` builds, printed at `indent`.
fn foreach(indent: &str) -> String {
    format!(
        "foreach ($data as $property => $value) {{\n\
         {indent}    if (property_exists($this, $property)) {{\n\
         {indent}        $this->{{$property}} = $value;\n\
         {indent}    }}\n\
         {indent}}}"
    )
}
