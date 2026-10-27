//! `lint/dead-code/no-redundant-default-argument`, ported from Rector 2.6.1's
//! `RemoveArgumentFromDefaultParentCallRector` (rules/DeadCode/Rector/ClassMethod).
//!
//! In a class that extends another, a statement `parent::name(...)` of a public or protected method's body loses its
//! trailing arguments that equal the default of their parameter, taken from the first ancestor with the method
//! (interfaces, then traits, then parent classes, as `getAncestors()` orders them). Only literal values count (a
//! scalar, a constant, a class constant, or an array of those); a named argument keeps them all. Pun reads a default
//! that is a scalar, `null` or `[]`, and an argument that is a scalar, a constant or `[]`; any other pair stops there.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::class_method::{class_info, default_value, element_parts, expr_value, is_private};
use super::rector_batch5::list_removal;
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoRedundantDefaultArgument;

impl Rule for NoRedundantDefaultArgument {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-redundant-default-argument"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let facts = facts(ctx);
        let Some(class) = class_info(node, ancestors, &facts) else {
            return;
        };
        let codebase = &facts.codebase;
        let Some(own) =
            (class.extends.as_ref()).and_then(|_| codebase.get_class_like(class.name.as_bytes()))
        else {
            return;
        };
        let ancestor_names: Vec<&[u8]> = own
            .all_parent_interfaces
            .iter()
            .chain(own.used_traits.iter())
            .chain(own.all_parent_classes.iter())
            .map(|w| w.as_bytes())
            .collect();
        for member in class.members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let MethodBody::Concrete(body) = &method.body else {
                continue;
            };
            if is_private(method) {
                continue;
            }
            for statement in body.statements.iter() {
                let Statement::Expression(statement) = statement else {
                    continue;
                };
                let Expression::Call(Call::StaticMethod(call)) =
                    unparenthesize(statement.expression)
                else {
                    continue;
                };
                let (Expression::Parent(_), ClassLikeMemberSelector::Identifier(name)) =
                    (call.class, &call.method)
                else {
                    continue;
                };
                let args: Option<Vec<&PositionalArgument<'_>>> =
                    (call.argument_list.arguments.iter())
                        .map(|a| match a {
                            Argument::Positional(a) => Some(a),
                            Argument::Named(_) => None,
                        })
                        .collect();
                let Some(args) = args.filter(|a| !a.is_empty()) else {
                    continue;
                };
                let Some(parent_method) = ancestor_names
                    .iter()
                    .find_map(|a| codebase.get_declaring_method(a, name.value))
                else {
                    continue;
                };
                let params = &parent_method.parameters;
                let mut kept = args.len();
                for index in (0..params.len()).rev() {
                    let param = &params[index];
                    if !param.flags.has_default() || param.flags.is_variadic() || index >= kept {
                        break;
                    }
                    let arg = args[index];
                    if index + 1 < kept
                        || arg.ellipsis.is_some()
                        || is_dynamic_expr(arg.value)
                        || expr_value(arg.value)
                            .is_none_or(|v| Some(v) != default_value(&facts, param))
                    {
                        break;
                    }
                    kept = index;
                }
                if kept == args.len() {
                    continue;
                }
                let spans: Vec<Span> = args.iter().map(|a| node_span(*a)).collect();
                let removed: Vec<bool> = (0..args.len()).map(|i| i >= kept).collect();
                let edits = list_removal(&spans, &removed).unwrap_or_else(|| {
                    let list = &call.argument_list;
                    vec![Edit::new(
                        Span::new(
                            list.left_parenthesis.end.offset as usize,
                            list.right_parenthesis.start.offset as usize,
                        ),
                        "",
                    )]
                });
                out.push(Report {
                    span: node_span(call),
                    message: format!(
                        "Remove the arguments of `parent::{}()` that equal the parameters' defaults.",
                        String::from_utf8_lossy(name.value)
                    ),
                    help: None,
                    fix: Some(Fix {
                        safety: Safety::Safe,
                        edits,
                    }),
                });
            }
        }
    }
}

/// `ExprAnalyzer::isDynamicExpr`: anything but a scalar that is not an interpolated string, a constant, a class
/// constant of a named class (not `static::`), or an array of those with literal keys; a leading `+` or `-` is looked
/// through.
fn is_dynamic_expr(expr: &Expression<'_>) -> bool {
    let mut expr = unparenthesize(expr);
    if let Expression::UnaryPrefix(u) = expr
        && matches!(
            u.operator,
            UnaryPrefixOperator::Plus(_) | UnaryPrefixOperator::Negation(_)
        )
    {
        expr = unparenthesize(u.operand);
    }
    match expr {
        Expression::Array(a) => is_dynamic_array(&a.elements),
        Expression::LegacyArray(a) => is_dynamic_array(&a.elements),
        Expression::Literal(_) | Expression::MagicConstant(_) => false,
        Expression::ConstantAccess(_) => false,
        Expression::Access(Access::ClassConstant(c)) => {
            !matches!(
                c.class,
                Expression::Identifier(_) | Expression::Self_(_) | Expression::Parent(_)
            ) || !matches!(c.constant, ClassLikeConstantSelector::Identifier(_))
        }
        _ => true,
    }
}

fn is_dynamic_array(elements: &TokenSeparatedSequence<'_, ArrayElement<'_>>) -> bool {
    elements
        .iter()
        .filter_map(element_parts)
        .any(|(key, value)| {
            key.is_some_and(|k| {
                !matches!(
                    k,
                    Expression::Literal(Literal::String(_) | Literal::Integer(_))
                )
            }) || is_dynamic_expr(value)
        })
}
