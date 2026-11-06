//! `lint/types/add-return-phpdoc-json-array`, ported from Rector 2.6.1's `AddReturnDocblockForJsonArrayRector`
//! (rules/TypeDeclarationDocblocks/Rector/ClassMethod): a function or method whose one `return` returns
//! `json_decode($json, true)` (or Nette's `Json::decode($json, true)`) gets `@return array<string, mixed>`.

use mago_syntax::cst::{
    Argument, ArgumentList, Call, ClassLikeMemberSelector, Expression, Literal, Node, NodeKind,
};
use pun_phpstan::types::{Type, mixed};

use super::class_scope;
use super::docblock::{FunctionLike, doc_report};
use super::facts::facts;
use super::names::resolve_class_name;
use super::silent_void::Place;
use crate::rules::phpdoc_writer::type_mapper::is_useful_array_tag;
use crate::rules::phpdoc_writer::change_return_type;
use crate::{Context, Report, Rule};

pub struct AddReturnPhpdocJsonArray;

impl Rule for AddReturnPhpdocJsonArray {
    fn id(&self) -> &'static str {
        "lint/types/add-return-phpdoc-json-array"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = FunctionLike::of(node) else {
            return;
        };
        // definitely not an array return
        if f.has_non_array_return_type() {
            return;
        }
        let facts = facts(ctx);
        let scope = class_scope(ancestors);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: scope.as_ref(),
        };
        let Some(value) = f.only_return_with_expr(&place).and_then(|r| r.value) else {
            return;
        };
        if !is_json_decode_to_array(ancestors, value) {
            return;
        }
        let target = f.target(ctx);
        let mut info = target.info(ctx.source);
        if is_useful_array_tag(info.return_tag().map(|t| &t.ty)) {
            return;
        }
        change_return_type(&mut info, &Type::array(Type::String, mixed()));
        out.extend(doc_report(
            f.name,
            "Add the `@return array<string, mixed>` type of the decoded JSON.",
            ctx.source,
            &target,
            &info,
        ));
    }
}

/// `isJsonDecodeToArray()`.
fn is_json_decode_to_array(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    match expr {
        Expression::Call(Call::Function(call)) => {
            let Expression::Identifier(id) = call.function else {
                return false;
            };
            // `isName()` of a function name: the name as resolved or as written in the global namespace
            let written = String::from_utf8_lossy(id.value());
            let name = written.trim_start_matches('\\');
            let short = name.rsplit('\\').next().unwrap_or(name);
            if !short.eq_ignore_ascii_case("json_decode") || (name.contains('\\') && !name.eq_ignore_ascii_case("json_decode")) {
                return false;
            }
            is_true_arg(&call.argument_list, "associative", 1)
        }
        Expression::Call(Call::StaticMethod(call)) => {
            let Expression::Identifier(class) = call.class else {
                return false;
            };
            if !resolve_class_name(ancestors, class).eq_ignore_ascii_case("Nette\\Utils\\Json") {
                return false;
            }
            if !matches!(&call.method, ClassLikeMemberSelector::Identifier(m) if m.value.eq_ignore_ascii_case(b"decode")) {
                return false;
            }
            is_true_arg(&call.argument_list, "forceArrays", 1)
        }
        _ => false,
    }
}

/// `CallLike::getArg($name, $position)` is `true` (`ValueResolver::isTrue()`).
fn is_true_arg(list: &ArgumentList<'_>, name: &str, position: usize) -> bool {
    let arg = list.arguments.iter().enumerate().find_map(|(i, a)| match a {
        Argument::Named(n) if n.name.value == name.as_bytes() => Some(n.value),
        Argument::Positional(p) if i == position && p.ellipsis.is_none() => Some(p.value),
        _ => None,
    });
    matches!(arg, Some(Expression::Literal(Literal::True(_))))
}
