//! `lint/dead-code/no-redundant-promoted-assign`, ported from Rector 2.8.4's
//! `RemoveUselessAssignFromPropertyPromotionRector` (rules/DeadCode/Rector/ClassMethod).
//!
//! A constructor whose body only assigns promoted parameters to their own properties (`$this->std = $std;`) does
//! nothing the promotion does not already do; the assignments are removed. A readonly promoted property is left out:
//! assigning it again is an error that the code may rely on. Any other statement in the body keeps it as it is.

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_stmts::{assign, block_list, expression, remove, variable_name};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoRedundantPromotedAssign;

impl Rule for NoRedundantPromotedAssign {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-redundant-promoted-assign"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else { return };
        if !method.name.value.eq_ignore_ascii_case(b"__construct") {
            return;
        }
        let MethodBody::Concrete(block) = &method.body else {
            return;
        };
        let list = block_list(ctx.source, block);
        if list.is_empty() {
            return;
        }
        // Promoted parameters, readonly ones left out: assigning them again is an error on the first place.
        let names: Vec<&[u8]> = method
            .parameter_list
            .parameters
            .iter()
            .filter(|p| !p.modifiers.is_empty() || p.hooks.is_some())
            .filter(|p| !p.modifiers.iter().any(Modifier::is_readonly))
            .map(|p| &p.variable.name[1..])
            .collect();
        if names.is_empty() || list.nop {
            return;
        }
        // Every statement must be `$this->name = $name;` for a promoted `name`, or nothing is removed.
        for statement in &list.stmts {
            let Some(assign) = expression(statement).and_then(assign) else {
                return;
            };
            let Expression::Access(Access::Property(fetch)) = unparenthesize(assign.lhs) else {
                return;
            };
            // php-parser's name of `$this->$a` and `$this->{$a}` is the variable `$a`, and Rector's `getName` of a
            // variable is its name.
            let property = match &fetch.property {
                ClassLikeMemberSelector::Identifier(identifier) => Some(identifier.value),
                ClassLikeMemberSelector::Variable(Variable::Direct(v)) => Some(&v.name[1..]),
                ClassLikeMemberSelector::Expression(e) => variable_name(e.expression),
                _ => None,
            };
            let Some(property) = property else {
                return;
            };
            let is_this =
                variable_name(fetch.object).is_some_and(|n| n.eq_ignore_ascii_case(b"this"));
            let promoted = names.iter().any(|n| n.eq_ignore_ascii_case(property));
            let same = variable_name(assign.rhs).is_some_and(|n| n.eq_ignore_ascii_case(property));
            if !(is_this && promoted && same) {
                return;
            }
        }
        let indices: Vec<usize> = (0..list.stmts.len()).collect();
        out.push(Report {
            span: node_span(block),
            message: "These assignments repeat what the promoted properties already do."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: remove(ctx, &list, &indices),
            }),
        });
    }
}
