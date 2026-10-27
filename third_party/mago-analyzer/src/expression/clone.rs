use mago_allocator::Arena;
use std::rc::Rc;
use std::sync::Arc;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::has_method::TObjectHasMethod;
use mago_codex::ttype::combiner;
use mago_codex::ttype::get_never;
use mago_codex::ttype::get_object;
use mago_codex::ttype::intersect_union_types;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_syntax::cst::Clone;
use mago_word::word;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;

impl<'ast, 'arena> Analyzable<'ast, 'arena> for Clone<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        self.object.analyze(context, block_context, artifacts)?;

        let Some(object_type) = artifacts.get_rc_expression_type(&self.object).cloned() else {
            return Ok(());
        };

        let mut invalid_clone_atomics = vec![];
        let mut has_mixed_type = false;
        let mut has_cloneable_object = false;

        let mut atomic_types = object_type.types.iter().collect::<Vec<_>>();
        while let Some(atomic_type) = atomic_types.pop() {
            match atomic_type {
                TAtomic::Object(object) => match object {
                    TObject::Any
                    | TObject::WithProperties(_)
                    | TObject::HasMethod(_)
                    | TObject::HasProperty(_) => {
                        has_cloneable_object = true;
                    }
                    TObject::Enum(_) => {
                        invalid_clone_atomics.push(atomic_type);
                    }
                    TObject::Named(named_object) => {
                        if !context.codebase.trait_exists(named_object.name.as_bytes())
                            && !context
                                .codebase
                                .class_or_interface_exists(named_object.name.as_bytes())
                        {
                            invalid_clone_atomics.push(atomic_type);
                        } else {
                            has_cloneable_object = true;
                        }
                    }
                },
                TAtomic::GenericParameter(parameter) => {
                    atomic_types.extend(parameter.constraint.types.iter());
                }
                TAtomic::Mixed(_) => {
                    has_mixed_type = true;
                }
                TAtomic::Scalar(scalar)
                    if scalar.is_false() && object_type.ignore_falsable_issues() => {}
                TAtomic::Null | TAtomic::Void if object_type.ignore_nullable_issues() => {}
                TAtomic::Callable(callable)
                    if callable.get_signature().is_none_or(
                        mago_codex::ttype::atomic::callable::TCallableSignature::is_closure,
                    ) =>
                {
                    has_cloneable_object = true;
                }
                _ => {
                    invalid_clone_atomics.push(atomic_type);
                }
            }
        }

        if has_mixed_type {
            context.collector.report_with_code(
                IssueCode::MixedClone,
                Issue::warning("Cannot statically verify `clone` on a `mixed` type.")
                    .with_annotation(Annotation::primary(self.object.span()).with_message(format!(
                        "This expression has type `{}`, which could be a non-object at runtime.",
                        object_type.get_id()
                    )))
                    .with_note("Cloning requires the value to be an object. Using `clone` on a non-object will cause a fatal error.")
                    .with_help("Use type hints or `is_object()` checks to ensure the value is an object before cloning."),
            );
        }

        if !invalid_clone_atomics.is_empty() {
            let invalid_types_str = invalid_clone_atomics
                .iter()
                .map(|t| t.get_id().to_string())
                .collect::<Vec<_>>()
                .join("|");

            if has_cloneable_object || has_mixed_type {
                context.collector.report_with_code(
                    IssueCode::PossiblyInvalidClone,
                    Issue::warning(format!(
                        "Expression of type `{}` might not be a cloneable object.",
                        object_type.get_id()
                    ))
                    .with_annotation(Annotation::primary(self.object.span()).with_message(format!(
                        "This could be of type `{invalid_types_str}`, which cannot be cloned"
                    )))
                    .with_note("Attempting to `clone` a non-object or an enum will result in a fatal error.")
                    .with_help("Ensure the value is a cloneable object before this operation, for example by using an `instanceof` check."),
                );
            } else {
                let (primary_message, note) = if invalid_clone_atomics.iter().any(|t| t.is_enum()) {
                    (
                        "This expression is an enum, which is not cloneable",
                        "PHP enums are singleton-like objects and cannot be cloned. This will cause a fatal `Error`.",
                    )
                } else {
                    (
                        "This expression is not an object",
                        "The `clone` operator can only be used on objects. This will cause a fatal `Error`.",
                    )
                };

                context.collector.report_with_code(
                    IssueCode::InvalidClone,
                    Issue::error(format!(
                        "Invalid `clone` operation on non-cloneable type `{invalid_types_str}`."
                    ))
                    .with_annotation(Annotation::primary(self.object.span()).with_message(primary_message))
                    .with_note(note)
                    .with_help("Remove the `clone` operator or ensure the expression evaluates to a cloneable object instance."),
                );
            }
        }

        artifacts.set_rc_expression_type(
            self,
            Rc::new(resolve_clone_type(&object_type, context.codebase)),
        );

        Ok(())
    }
}

/// PHPStan's CloneHandler::resolveCloneType: intersect with object, then map $this to static.
fn resolve_clone_type(operand: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
    // Object intersection and CloneTypeTraverser retain the operand's bottom identity.
    // Rebuilding its atomic would lose explicit accepting/nonaccepting never flags.
    if operand.is_never() {
        return operand.clone();
    }

    let object = get_object();
    let mut types = Vec::new();
    for atomic in operand.types.iter() {
        let mut result = match atomic {
            TAtomic::GenericParameter(parameter) => {
                let bound = intersect_union_types(&parameter.constraint, &object, codebase)
                    .unwrap_or_else(get_never);
                if bound.is_never() {
                    get_never()
                } else {
                    let mut parameter = parameter.clone();
                    if parameter.constraint.is_mixed() {
                        parameter.add_intersection_type(TAtomic::Object(TObject::Any));
                    } else {
                        parameter.constraint = Arc::new(bound);
                    }
                    TUnion::from_atomic(TAtomic::GenericParameter(parameter))
                }
            }
            TAtomic::Callable(TCallable::Signature(signature)) if !signature.is_closure() => {
                let mut callable_object = TObjectHasMethod::new(word("__invoke"));
                callable_object.intersection_types = Some(vec![atomic.clone()]);
                TUnion::from_atomic(TAtomic::Object(TObject::HasMethod(callable_object)))
            }
            _ => intersect_union_types(&TUnion::from_atomic(atomic.clone()), &object, codebase)
                .unwrap_or_else(get_never),
        };
        for atomic in result.types.to_mut() {
            clone_this_to_static(atomic);
        }
        types.extend(result.types.into_owned());
    }
    TUnion::from_vec(combiner::combine(types, codebase, Default::default()))
}

/// CloneTypeTraverser visits union/intersection members, not template bounds or object type arguments.
fn clone_this_to_static(atomic: &mut TAtomic) {
    if let TAtomic::Object(TObject::Named(object)) = atomic {
        object.is_this = false;
    }
    if let Some(intersections) = atomic.get_intersection_types_mut() {
        for intersection in intersections {
            clone_this_to_static(intersection);
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::code::IssueCode;
    use crate::test_analysis;

    test_analysis! {
        name = clone_valid,
        code = indoc! {"
            <?php

            class Example {}

            function get_clone(Example $example): Example {
                return clone $example;
            }
        "}
    }

    test_analysis! {
        name = clone_maybe_invalid_valid,
        code = indoc! {"
            <?php

            class Example {}

            function get_clone(Example|string $example): Example {
                return clone $example;
            }
        "},
        issues = [
            IssueCode::PossiblyInvalidClone,
        ]
    }

    test_analysis! {
        name = clone_enum,
        code = indoc! {"
            <?php

            enum Color { case Red; }

            function get_clone(Color $color): Color {
                return clone $color;
            }
        "},
        issues = [
            IssueCode::InvalidClone,
        ]
    }
}
