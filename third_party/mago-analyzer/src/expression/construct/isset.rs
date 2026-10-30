use mago_allocator::Arena;
use std::rc::Rc;
use mago_codex::ttype::get_bool;
use mago_codex::ttype::get_false;
use mago_codex::ttype::get_true;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_syntax::cst::Expression;
use mago_syntax::cst::IssetConstruct;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;

impl<'ast, 'arena> Analyzable<'ast, 'arena> for IssetConstruct<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        let mut all_definitely_set = true;
        let mut any_definitely_unset = false;
        for value in &self.values {
            if !is_valid_isset_expression(value) {
                context.collector.report_with_code(
                    IssueCode::InvalidIssetExpression,
                    Issue::error("Cannot use `isset()` on the result of an expression.")
                        .with_annotation(
                            Annotation::primary(value.span()).with_message("This is not a variable or property"),
                        )
                        .with_note("The `isset()` function is designed to check if a variable, property, or array element is set and not null.")
                        .with_help("Consider using `null !== expression` for this check instead."),
                );
            }

            let was_inside_isset = block_context.flags.inside_isset();
            block_context.flags.set_inside_isset(true);
            value.analyze(context, block_context, artifacts)?;
            block_context.flags.set_inside_isset(was_inside_isset);
            if let Some(value_type) = artifacts.get_expression_type(value) {
                if value_type.is_never() || value_type.is_null() {
                    any_definitely_unset = true;
                    continue;
                }

                if value_type.possibly_undefined()
                    || value_type.possibly_undefined_from_try()
                    || value_type.has_null()
                    || value_type.has_nullable_mixed()
                {
                    all_definitely_set = false;
                }
            } else {
                all_definitely_set = false;
            }
        }

        let result_type = if any_definitely_unset {
            get_false()
        } else if all_definitely_set {
            get_true()
        } else {
            get_bool()
        };

        artifacts.set_expression_type(self, result_type);
        if block_context.native_locals.is_some() {
            // IssetHandler folds captured reads on its post-revert scope. The
            // reads keep their operand positions; no expression is walked again.
            let mut verdict = Some(true);
            let mut available = true;
            for value in &self.values {
                match crate::native::isset(value, block_context, artifacts, context.codebase) {
                    Some(Some(false)) => verdict = Some(false),
                    Some(None) if verdict != Some(false) => verdict = None,
                    Some(_) => {},
                    None => available = false,
                }
            }
            if available {
                artifacts.set_rc_native_expression_type(self, Rc::new(match verdict {
                    Some(true) => get_true(),
                    Some(false) => get_false(),
                    None => get_bool(),
                }));
            }
        }

        Ok(())
    }
}

const fn is_valid_isset_expression(expression: &Expression) -> bool {
    match expression {
        Expression::Variable(_) | Expression::Access(_) | Expression::ArrayAccess(_) => true,
        Expression::Assignment(assignment) => assignment.operator.is_assign() && assignment.rhs.is_reference(),
        _ => false,
    }
}
