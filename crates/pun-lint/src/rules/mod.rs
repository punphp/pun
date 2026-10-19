//! The ported rules. Each file is one Pun rule, ported function by function from its upstream source.

mod laravel;
mod mago_const_value;
mod mago_usage;
mod no_and_assignment;
mod no_and_true;
mod no_assign_in_condition;
mod no_assign_in_ternary_branch;
mod no_cast_in_concat;
mod no_concat_assign_new_item;
mod no_condition_exact_return;
mod no_dead_php_version_check;
mod no_double_negation;
mod no_double_self_assignment;
mod no_duplicate_array_key;
mod no_duplicate_case_body;
mod no_duplicate_match_arm;
mod no_else_after_return;
mod no_empty_block;
mod no_empty_catch;
mod no_empty_loop;
mod no_future_php_version_check;
mod no_identical_branches;
mod no_implode_ternary;
mod no_invalid_open_tag;
mod no_iterator_to_array_in_foreach;
mod no_mirror_assignment;
mod no_negated_and;
mod no_negated_boolean_literal;
mod no_negated_condition;
mod no_negated_group;
mod no_null_property_init;
mod no_overwritten_assignment;
mod no_redundant_condition;
mod no_redundant_final_const;
mod no_redundant_null_check;
mod no_redundant_promoted_assign;
mod no_redundant_property_default;
mod no_redundant_readonly_property;
mod no_redundant_write_visibility;
mod no_return_in_constructor;
mod no_single_argument_sprintf;
mod no_single_case_switch;
mod no_single_item_in_array;
mod no_tautology_ternary;
mod no_unreachable_code;
mod no_unsafe_finally;
mod no_unused_closure_variable;
mod no_unused_expression;
mod no_unused_foreach_key;
mod no_unused_global;
mod no_unused_label;
mod no_unused_private_method_param;
mod no_unused_private_property;
mod no_unused_public_method_param;
mod no_unused_static;
mod no_useless_array_values;
mod no_useless_block;
mod no_useless_catch;
mod no_useless_condition;
mod no_useless_continue;
mod php_parser;
mod prefer_absolute_include;
mod prefer_array_append;
mod prefer_array_literal;
mod prefer_boolean_and_or;
mod prefer_coalesce_chain;
mod prefer_coalesce_over_foreach;
mod prefer_compound_assignment;
mod prefer_early_return;
mod prefer_early_return_of_variable;
mod prefer_early_return_over_prepared_value;
mod prefer_empty_array_compare;
mod prefer_func_num_args;
mod prefer_in_array;
mod prefer_in_array_over_foreach;
mod prefer_in_array_over_search;
mod prefer_new_self;
mod prefer_not_in_array;
mod prefer_null_coalesce;
mod prefer_property_default;
mod prefer_property_over_local_getter;
mod prefer_simple_condition;
mod rector_batch5;
mod rector_code_quality;
mod rector_conditions;
mod rector_stmts;
mod side_effect;
mod used_variable;
mod no_sole_value_sprintf;
mod no_compact;
mod require_is_a_allow_string;

pub use laravel::*;
pub use no_and_assignment::NoAndAssignment;
pub use no_and_true::NoAndTrue;
pub use no_assign_in_condition::NoAssignInCondition;
pub use no_assign_in_ternary_branch::NoAssignInTernaryBranch;
pub use no_cast_in_concat::NoCastInConcat;
pub use no_concat_assign_new_item::NoConcatAssignNewItem;
pub use no_condition_exact_return::NoConditionExactReturn;
pub use no_dead_php_version_check::NoDeadPhpVersionCheck;
pub use no_double_negation::NoDoubleNegation;
pub use no_double_self_assignment::NoDoubleSelfAssignment;
pub use no_duplicate_array_key::NoDuplicateArrayKey;
pub use no_duplicate_case_body::NoDuplicateCaseBody;
pub use no_duplicate_match_arm::NoDuplicateMatchArm;
pub use no_else_after_return::NoElseAfterReturn;
pub use no_empty_block::NoEmptyBlock;
pub use no_empty_catch::NoEmptyCatch;
pub use no_empty_loop::NoEmptyLoop;
pub use no_future_php_version_check::NoFuturePhpVersionCheck;
pub use no_identical_branches::NoIdenticalBranches;
pub use no_implode_ternary::NoImplodeTernary;
pub use no_invalid_open_tag::NoInvalidOpenTag;
pub use no_iterator_to_array_in_foreach::NoIteratorToArrayInForeach;
pub use no_mirror_assignment::NoMirrorAssignment;
pub use no_negated_and::NoNegatedAnd;
pub use no_negated_boolean_literal::NoNegatedBooleanLiteral;
pub use no_negated_condition::NoNegatedCondition;
pub use no_negated_group::NoNegatedGroup;
pub use no_null_property_init::NoNullPropertyInit;
pub use no_overwritten_assignment::NoOverwrittenAssignment;
pub use no_redundant_condition::NoRedundantCondition;
pub use no_redundant_final_const::NoRedundantFinalConst;
pub use no_redundant_null_check::NoRedundantNullCheck;
pub use no_redundant_promoted_assign::NoRedundantPromotedAssign;
pub use no_redundant_property_default::NoRedundantPropertyDefault;
pub use no_redundant_readonly_property::NoRedundantReadonlyProperty;
pub use no_redundant_write_visibility::NoRedundantWriteVisibility;
pub use no_return_in_constructor::NoReturnInConstructor;
pub use no_single_argument_sprintf::NoSingleArgumentSprintf;
pub use no_single_case_switch::NoSingleCaseSwitch;
pub use no_single_item_in_array::NoSingleItemInArray;
pub use no_tautology_ternary::NoTautologyTernary;
pub use no_unreachable_code::NoUnreachableCode;
pub use no_unsafe_finally::NoUnsafeFinally;
pub use no_unused_closure_variable::NoUnusedClosureVariable;
pub use no_unused_expression::NoUnusedExpression;
pub use no_unused_foreach_key::NoUnusedForeachKey;
pub use no_unused_global::NoUnusedGlobal;
pub use no_unused_label::NoUnusedLabel;
pub use no_unused_private_method_param::NoUnusedPrivateMethodParam;
pub use no_unused_private_property::NoUnusedPrivateProperty;
pub use no_unused_public_method_param::NoUnusedPublicMethodParam;
pub use no_unused_static::NoUnusedStatic;
pub use no_useless_array_values::NoUselessArrayValues;
pub use no_useless_block::NoUselessBlock;
pub use no_useless_catch::NoUselessCatch;
pub use no_useless_condition::NoUselessCondition;
pub use no_useless_continue::NoUselessContinue;
pub use prefer_absolute_include::PreferAbsoluteInclude;
pub use prefer_array_append::PreferArrayAppend;
pub use prefer_array_literal::PreferArrayLiteral;
pub use prefer_boolean_and_or::PreferBooleanAndOr;
pub use prefer_coalesce_chain::PreferCoalesceChain;
pub use prefer_coalesce_over_foreach::PreferCoalesceOverForeach;
pub use prefer_compound_assignment::PreferCompoundAssignment;
pub use prefer_early_return::PreferEarlyReturn;
pub use prefer_early_return_of_variable::PreferEarlyReturnOfVariable;
pub use prefer_early_return_over_prepared_value::PreferEarlyReturnOverPreparedValue;
pub use prefer_empty_array_compare::PreferEmptyArrayCompare;
pub use prefer_func_num_args::PreferFuncNumArgs;
pub use prefer_in_array::PreferInArray;
pub use prefer_in_array_over_foreach::PreferInArrayOverForeach;
pub use prefer_in_array_over_search::PreferInArrayOverSearch;
pub use prefer_new_self::PreferNewSelf;
pub use prefer_not_in_array::PreferNotInArray;
pub use prefer_null_coalesce::PreferNullCoalesce;
pub use prefer_property_default::PreferPropertyDefault;
pub use prefer_property_over_local_getter::PreferPropertyOverLocalGetter;
pub use prefer_simple_condition::PreferSimpleCondition;
pub use no_sole_value_sprintf::NoSoleValueSprintf;
pub use no_compact::NoCompact;
pub use require_is_a_allow_string::RequireIsAAllowString;

use crate::Rule;

/// Every rule, in a fixed order.
pub fn all() -> Vec<Box<dyn Rule>> {
    let mut rules = laravel::all();
    rules.extend(ungrouped());
    rules
}

/// The rules outside the group modules.
fn ungrouped() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(PreferCompoundAssignment),
        Box::new(NoAndAssignment),
        Box::new(PreferNullCoalesce),
        Box::new(PreferArrayAppend),
        Box::new(NoNegatedAnd),
        Box::new(NoConcatAssignNewItem),
        Box::new(NoNegatedBooleanLiteral),
        Box::new(NoDoubleNegation),
        Box::new(NoNegatedGroup),
        Box::new(PreferEmptyArrayCompare),
        Box::new(PreferFuncNumArgs),
        Box::new(NoUselessArrayValues),
        Box::new(NoTautologyTernary),
        Box::new(NoSingleItemInArray),
        Box::new(NoSingleArgumentSprintf),
        Box::new(NoAndTrue),
        Box::new(NoCastInConcat),
        Box::new(NoUselessContinue),
        Box::new(NoEmptyBlock),
        Box::new(NoUselessCatch),
        Box::new(NoDoubleSelfAssignment),
        Box::new(NoDuplicateArrayKey),
        Box::new(NoRedundantFinalConst),
        Box::new(NoNullPropertyInit),
        Box::new(NoUnusedClosureVariable),
        Box::new(NoUnusedForeachKey),
        Box::new(NoReturnInConstructor),
        Box::new(NoUselessBlock),
        Box::new(NoUnreachableCode),
        Box::new(NoMirrorAssignment),
        Box::new(NoRedundantPromotedAssign),
        Box::new(NoDuplicateCaseBody),
        Box::new(NoIdenticalBranches),
        Box::new(NoElseAfterReturn),
        Box::new(PreferEarlyReturn),
        Box::new(PreferEarlyReturnOverPreparedValue),
        Box::new(PreferEarlyReturnOfVariable),
        Box::new(PreferInArray),
        Box::new(PreferNotInArray),
        Box::new(NoRedundantNullCheck),
        Box::new(PreferSimpleCondition),
        Box::new(PreferInArrayOverSearch),
        Box::new(NoAssignInTernaryBranch),
        Box::new(NoSingleCaseSwitch),
        Box::new(NoImplodeTernary),
        Box::new(PreferNewSelf),
        Box::new(PreferArrayLiteral),
        Box::new(NoRedundantReadonlyProperty),
        Box::new(PreferAbsoluteInclude),
        Box::new(PreferBooleanAndOr),
        Box::new(NoFuturePhpVersionCheck),
        Box::new(NoDeadPhpVersionCheck),
        Box::new(NoUnusedPublicMethodParam),
        Box::new(NoUnusedPrivateMethodParam),
        Box::new(NoUnusedPrivateProperty),
        Box::new(NoRedundantPropertyDefault),
        Box::new(PreferPropertyDefault),
        Box::new(PreferInArrayOverForeach),
        Box::new(PreferCoalesceChain),
        Box::new(PreferCoalesceOverForeach),
        Box::new(PreferPropertyOverLocalGetter),
        Box::new(NoEmptyCatch),
        Box::new(NoUnsafeFinally),
        Box::new(NoAssignInCondition),
        Box::new(NoUnusedLabel),
        Box::new(NoRedundantWriteVisibility),
        Box::new(NoIteratorToArrayInForeach),
        Box::new(NoInvalidOpenTag),
        Box::new(NoNegatedCondition),
        Box::new(NoUnusedGlobal),
        Box::new(NoUnusedStatic),
        Box::new(NoDuplicateMatchArm),
        Box::new(NoUselessCondition),
        Box::new(NoConditionExactReturn),
        Box::new(NoOverwrittenAssignment),
        Box::new(NoRedundantCondition),
        Box::new(NoEmptyLoop),
        Box::new(NoUnusedExpression),
        Box::new(NoSoleValueSprintf),
        Box::new(NoCompact),
        Box::new(RequireIsAAllowString),
    ]
}
