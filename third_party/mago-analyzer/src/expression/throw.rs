use mago_allocator::Arena;

use mago_codex::ttype::TType;
use mago_codex::ttype::get_non_accepting_never;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_syntax::cst::Throw;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::artifacts::VariableThrowKind;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;

impl<'ast, 'arena> Analyzable<'ast, 'arena> for Throw<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        let was_inside_throw = block_context.flags.inside_throw();
        block_context.flags.set_inside_throw(true);
        self.exception.analyze(context, block_context, artifacts)?;
        block_context.flags.set_inside_throw(was_inside_throw);
        block_context.flags.set_has_returned(true);

        if let Some(exception_type) = artifacts.get_expression_type(self.exception).cloned() {
            for exception_atomic in exception_type.types.as_ref() {
                if exception_atomic.extends_or_implements(context.codebase, b"Throwable") {
                    for object_name in exception_atomic.get_all_object_names() {
                        block_context.possibly_thrown_exceptions.entry(object_name).or_default().insert(self.span());
                        artifacts.variable_throw_scopes.entry((self.span(), VariableThrowKind::Expression)).or_default()
                            .insert(object_name, block_context.variable_scope());
                    }
                } else {
                    let exception_atomic_str = exception_atomic.get_id();

                    context.collector.report_with_code(
                        IssueCode::InvalidThrow,
                        Issue::error(format!(
                            "Cannot throw type `{exception_atomic_str}` because it is not an instance of Throwable."
                        ))
                        .with_annotation(
                            Annotation::primary(self.span())
                                .with_message(format!("This has type `{exception_atomic_str}`, not `Throwable`"))
                        )
                        .with_note(
                            "Only objects that implement the `Throwable` interface (like `Exception` or `Error`) can be thrown."
                        )
                        .with_help(
                            "Ensure the value being thrown is an instance of `Exception`, `Error`, or a subclass thereof."
                        ),
                    );
                }
            }
        }

        artifacts.set_expression_type(self, get_non_accepting_never());

        Ok(())
    }
}
